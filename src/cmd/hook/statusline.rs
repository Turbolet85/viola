//! `viola hook statusline` (hidden): the command the per-session settings override puts in the
//! status line's place. It records the usage reading the payload carries to `budget.json`, runs the
//! user's own statusline command with the same stdin and prints that command's stdout unchanged.
//! It exits 0 on every path, writes nothing to stderr, and outside a wrapped session writes nothing
//! anywhere (architecture §Standard Contracts → Hook contract; obs-plan §4 Scenario: Budget
//! governor).

use std::io::{Read, Write};
use std::process::{Child, Command, ExitCode, ExitStatus, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use chrono::Utc;
use tracing::field::Empty;
use tracing::instrument;
use viola_agent_claude::statusline;
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{MAX_FRAME, obs_event};
use viola_state::budget::write_budget;
use viola_state::snapshot::read_snapshot;
use viola_state::strict::check_instance;

use super::{Instance, STATUSLINE_DEADLINE, instance_of, reject_stdin};
use crate::cmd::Failure;
use crate::obs::{self, DetailSink};

/// The word after `hook` that names this arm. It is no hook event: `HookEvent` does not hold it.
pub(super) const WORD: &str = "statusline";

const SUBJECT: &str = "statusline-shell";

/// How often the exit of a command that has closed its stdout is looked for.
const EXIT_POLL: Duration = Duration::from_millis(1);

/// The wrapped session by its canonical directory: a `VIOLA_DIR` that resolves to anything but
/// `<home>/instances/<name>` is not one.
fn canonical(instance: &Instance) -> Option<Instance> {
    let dir = std::fs::canonicalize(&instance.dir).ok()?;
    let name: &str = instance.name.as_ref();
    instance_of(Some(name.into()), Some(dir.into_os_string()))
}

pub(super) fn statusline(started: Instant) -> Result<ExitCode, Failure> {
    let instance = instance_of(
        std::env::var_os("VIOLA_NAME"),
        std::env::var_os("VIOLA_DIR"),
    );
    let Some(instance) = instance.as_ref().and_then(canonical) else {
        return Ok(ExitCode::SUCCESS);
    };
    // Read before the log opens: opening it narrows the home to its owner, which would erase the
    // very state this check refuses.
    let trusted = check_instance(&instance.home, &instance.dir).is_ok();
    let (level, _) = obs::read_diagnostics_level(&instance.home);
    let _ = obs::viola_obs_init(
        &instance.home,
        ObsProcess::Hook,
        Some(instance.name.clone()),
        level,
    );
    #[cfg(feature = "fake-agent")]
    super::seam::panic_if_asked();
    let stdin = &mut std::io::stdin().lock();
    let out = &mut std::io::stdout().lock();
    let io = Io {
        stdin,
        out,
        bound: STATUSLINE_DEADLINE,
    };
    match handle(&instance, trusted, io, started) {
        Ok(()) => Ok(ExitCode::SUCCESS),
        Err(error) => Err(Failure {
            error,
            sink: Some(DetailSink {
                home: instance.home,
                instance: instance.name,
                process: ObsProcess::Hook,
            }),
        }),
    }
}

/// The payload's source, where the user's output goes, and how long the user's command may take.
struct Io<'a> {
    stdin: &'a mut dyn Read,
    out: &'a mut dyn Write,
    bound: Duration,
}

/// What the arm did, for its decision line.
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    budget_written: bool,
    detail: Option<&'static str>,
}

impl Outcome {
    /// A fail-open path: nothing was written, run or printed.
    fn failed(detail: &'static str) -> Self {
        Self {
            budget_written: false,
            detail: Some(detail),
        }
    }
}

/// `hook-invoked`, the arm's body when the instance passed its check, then `hook-decision`. With a
/// refused instance nothing is read, run or written.
#[instrument(skip_all, name = "hook.statusline")]
fn handle(instance: &Instance, trusted: bool, io: Io<'_>, started: Instant) -> anyhow::Result<()> {
    let invoked_at = obs::timestamp(Utc::now());
    obs_event!(
        INFO,
        ObsEvent::HookInvoked,
        hook_event = WORD,
        invoked_at = invoked_at.as_str(),
    );
    let outcome = if trusted {
        serve(instance, io)?
    } else {
        Outcome::failed("strict-modes-failed")
    };
    decided(&outcome, started);
    Ok(())
}

/// The payload read, its reading written, then the user's command run and its stdout printed.
fn serve(instance: &Instance, io: Io<'_>) -> anyhow::Result<Outcome> {
    let mut bytes = Vec::new();
    io.stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {
        reject_stdin("oversize");
        return Ok(Outcome::failed("oversize-stdin"));
    }
    let Ok(reading) = statusline::reading(&bytes) else {
        reject_stdin("malformed");
        return Ok(Outcome::failed("malformed-json"));
    };
    // A payload with no reading writes nothing, so the last reading stands.
    let budget_written =
        reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
    let command = read_snapshot(&instance.dir).and_then(|snapshot| snapshot.statusline_command);
    let shell = command
        .as_deref()
        .and_then(statusline::shell_argv)
        .map(|argv| shell_out(argv, &bytes, io.bound));
    let detail = match shell {
        Some(Shell::Exited {
            code: Some(0),
            stdout,
        }) => {
            // The user's bytes as they are, in one write; a reader that has gone is no fault.
            let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
            None
        }
        Some(Shell::Deadline) => Some("deadline"),
        Some(Shell::Exited { .. } | Shell::NotStarted) | None => None,
    };
    Ok(Outcome {
        budget_written,
        detail,
    })
}

fn decided(outcome: &Outcome, started: Instant) {
    let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    match outcome.detail {
        Some(detail) => obs_event!(
            WARN,
            ObsEvent::HookDecision,
            hook_event = WORD,
            budget_written = outcome.budget_written,
            deadline_hit = (detail == "deadline").then_some(true),
            duration_ms = duration_ms,
            detail = detail,
        ),
        None => obs_event!(
            INFO,
            ObsEvent::HookDecision,
            hook_event = WORD,
            budget_written = outcome.budget_written,
            duration_ms = duration_ms,
        ),
    }
}

/// What the user's command did within the bound.
#[derive(Debug, PartialEq, Eq)]
enum Shell {
    /// It ended by itself: its exit code (none when a signal ended it) and its stdout.
    Exited {
        code: Option<i32>,
        stdout: Vec<u8>,
    },
    /// It outlived the bound and was ended.
    Deadline,
    NotStarted,
}

/// The user's command through the shell, as ONE argument, with the payload on its stdin and this
/// process's environment and working directory unchanged. The command is user content: neither
/// line nor the span carries it.
#[instrument(
    skip_all,
    name = "statusline.shell_out",
    fields(shell_exit_status = Empty, duration_ms = Empty)
)]
fn shell_out([shell, flag, command]: [&str; 3], stdin: &[u8], bound: Duration) -> Shell {
    let started = Instant::now();
    obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
    let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
    let code = match &ran {
        Shell::Exited { code, .. } => *code,
        Shell::Deadline | Shell::NotStarted => None,
    };
    let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = SUBJECT,
        shell_exit_status = code,
        duration_ms = duration_ms,
    );
    let span = tracing::Span::current();
    if let Some(code) = code {
        span.record("shell_exit_status", code);
    }
    span.record("duration_ms", duration_ms);
    ran
}

/// `command` spawned directly with `stdin` on its stdin, its stdout read through the frame cap and
/// its stderr discarded; a child that outlives `bound` is ended.
fn run_bounded(command: &mut Command, stdin: &[u8], bound: Duration) -> Shell {
    let spawned = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    let Ok(mut child) = spawned else {
        return Shell::NotStarted;
    };
    feed(child.stdin.take(), stdin.to_vec());
    let stdout = drain(child.stdout.take());
    match wait_bounded(&mut child, &stdout, bound) {
        Some((status, stdout)) => Shell::Exited {
            code: status.code(),
            stdout,
        },
        None => {
            let _ = child.kill();
            let _ = child.wait();
            Shell::Deadline
        }
    }
}

/// The payload into the child's stdin on its own thread, then the pipe closed: a command that
/// never reads its stdin must not hold the arm.
fn feed(pipe: Option<impl Write + Send + 'static>, bytes: Vec<u8>) {
    if let Some(mut pipe) = pipe {
        std::thread::spawn(move || {
            let _ = pipe.write_all(&bytes);
        });
    }
}

/// A pipe being read to its end, through the frame cap, on its own thread.
fn drain(pipe: Option<impl Read + Send + 'static>) -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    if let Some(pipe) = pipe {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
            let _ = tx.send(bytes);
        });
    }
    rx
}

/// The child's exit and its whole stdout, when both arrive within `bound` of this call.
fn wait_bounded(
    child: &mut Child,
    stdout: &mpsc::Receiver<Vec<u8>>,
    bound: Duration,
) -> Option<(ExitStatus, Vec<u8>)> {
    let deadline = Instant::now() + bound;
    let bytes = stdout.recv_timeout(bound).ok()?;
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            return Some((status, bytes));
        }
        if Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(EXIT_POLL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::fs;
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
    use viola_channel::test_support::JsonFields as Fields;
    use viola_core::ViolaName;
    use viola_state::snapshot::{InstanceSnapshot, Wheel, write_snapshot};

    /// Synthetic statusline stdin: a reading beside the tests' canary.
    const PAYLOAD: &str = r#"{"session_id":"canary-chain-value-5c1e","rate_limits":{"five_hour":{"used_percentage":91,"resets_at":1738425600}}}"#;

    /// A wrapped session's directories under `root`, and no snapshot.
    fn instance_in(root: &Path) -> Instance {
        fs::create_dir_all(root).expect("root");
        let home = fs::canonicalize(root)
            .expect("canonical root")
            .join("vhome");
        let dir = home.join("instances").join("builder");
        fs::create_dir_all(&dir).expect("instance dir");
        Instance {
            name: ViolaName::try_new("builder".to_owned()).expect("valid"),
            dir,
            home,
        }
    }

    /// The same, with a snapshot recording `command` as the user's statusline command.
    fn instance_running(root: &Path, command: Option<&str>) -> Instance {
        let instance = instance_in(root);
        let snapshot = InstanceSnapshot {
            endpoint: None,
            pid: 1,
            started_at: "s".to_owned(),
            pinned_bin: "b".to_owned(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            statusline_command: command.map(str::to_owned),
        };
        write_snapshot(&instance.dir, &snapshot).expect("snapshot");
        instance
    }

    /// Every line the calling thread emits, its fields as JSON.
    #[derive(Clone, Default)]
    struct Lines(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> Layer<S> for Lines {
        fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            self.0.lock().expect("lines").push(Value::Object(fields.0));
        }
    }

    /// What `handle` printed for `stdin`, and the lines it wrote.
    fn handled(
        instance: &Instance,
        trusted: bool,
        mut stdin: impl Read,
        bound: Duration,
    ) -> (Vec<u8>, Vec<Value>) {
        let lines = Lines::default();
        let subscriber = tracing_subscriber::registry().with(lines.clone());
        let mut out = Vec::new();
        tracing::subscriber::with_default(subscriber, || {
            let io = Io {
                stdin: &mut stdin,
                out: &mut out,
                bound,
            };
            handle(instance, trusted, io, Instant::now()).expect("handled");
        });
        let got = lines.0.lock().expect("lines").clone();
        (out, got)
    }

    fn events(lines: &[Value]) -> Vec<&str> {
        lines.iter().filter_map(|l| l["event"].as_str()).collect()
    }

    fn budget(instance: &Instance) -> Option<Value> {
        let bytes = fs::read(instance.home.join("budget.json")).ok()?;
        Some(serde_json::from_slice(&bytes).expect("budget.json is JSON"))
    }

    /// A reader that fails the test when anything reads it.
    struct NeverRead;

    impl Read for NeverRead {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            panic!("stdin was read");
        }
    }

    const LONG: Duration = Duration::from_secs(8);

    #[test]
    fn statusline_canonical_instance_keeps_a_plain_directory() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        assert_eq!(canonical(&instance), Some(instance));
    }

    #[test]
    fn statusline_canonical_instance_of_a_missing_directory_is_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut instance = instance_in(tmp.path());
        fs::remove_dir(&instance.dir).expect("removed");
        assert_eq!(canonical(&instance), None);
        instance.dir = instance.home.join("instances").join("other");
        assert_eq!(canonical(&instance), None);
    }

    /// A `VIOLA_DIR` that is a link to a directory elsewhere has the right shape as written and the
    /// wrong one once resolved.
    #[cfg(unix)]
    #[test]
    fn statusline_canonical_instance_of_a_link_to_another_place_is_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        let elsewhere = instance.home.join("elsewhere");
        fs::create_dir(&elsewhere).expect("elsewhere");
        fs::remove_dir(&instance.dir).expect("removed");
        std::os::unix::fs::symlink(&elsewhere, &instance.dir).expect("link");
        assert_eq!(canonical(&instance), None);
    }

    /// A link that resolves to another instance directory of the right shape is that instance's
    /// canonical directory, never the link.
    #[cfg(unix)]
    #[test]
    fn statusline_canonical_instance_resolves_a_linked_home() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        let link = instance.home.with_file_name("linked");
        std::os::unix::fs::symlink(&instance.home, &link).expect("link");
        let through = Instance {
            name: instance.name.clone(),
            dir: link.join("instances").join("builder"),
            home: link,
        };
        assert_eq!(canonical(&through), Some(instance));
    }

    #[test]
    fn statusline_without_a_recorded_command_prints_nothing_and_records_the_reading() {
        let tmp = tempfile::tempdir().expect("tempdir");
        for instance in [
            instance_in(&tmp.path().join("a")),
            instance_running(&tmp.path().join("b"), None),
        ] {
            let (out, lines) = handled(&instance, true, PAYLOAD.as_bytes(), LONG);
            assert!(out.is_empty());
            let budget = budget(&instance).expect("a reading");
            assert_eq!(budget["v"], 1);
            assert_eq!(budget["five_hour"]["used_percentage"].as_f64(), Some(91.0));
            assert_eq!(budget["five_hour"]["resets_at"], "2025-02-01T16:00:00.000Z");
            assert_eq!(budget["seven_day"], "unknown");
            assert_eq!(events(&lines), ["hook-invoked", "hook-decision"]);
            assert_eq!(lines[0]["hook_event"], "statusline");
            assert!(lines[0]["invoked_at"].is_string());
            assert!(lines[0].get("corr").is_none());
            let mut decision = lines[1].clone();
            assert!(decision["duration_ms"].is_u64());
            for absent in ["process", "instance", "duration_ms"] {
                decision.as_object_mut().expect("object").remove(absent);
            }
            assert_eq!(
                decision,
                json!({"event": "hook-decision", "message": "hook-decision",
                    "hook_event": "statusline", "budget_written": true})
            );
        }
    }

    #[test]
    fn statusline_without_rate_limits_writes_no_reading_and_keeps_the_last() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        let (_, lines) = handled(&instance, true, br#"{"model":{}}"#.as_slice(), LONG);
        assert_eq!(budget(&instance), None);
        assert_eq!(lines[1]["budget_written"], false);
        assert!(lines[1].get("detail").is_none());
        handled(&instance, true, PAYLOAD.as_bytes(), LONG);
        let first = budget(&instance).expect("a reading");
        let (_, lines) = handled(&instance, true, br#"{"model":{}}"#.as_slice(), LONG);
        assert_eq!(budget(&instance), Some(first));
        assert_eq!(lines[1]["budget_written"], false);
    }

    /// A reading that cannot be written (a directory stands in the file's place) does not stop the
    /// arm.
    #[test]
    fn statusline_a_reading_that_cannot_be_written_is_budget_written_false() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        fs::create_dir(instance.home.join("budget.json.lock")).expect("a dir in the lock's place");
        let (out, lines) = handled(&instance, true, PAYLOAD.as_bytes(), LONG);
        assert!(out.is_empty());
        assert_eq!(events(&lines), ["hook-invoked", "hook-decision"]);
        assert_eq!(lines[1]["budget_written"], false);
        assert!(lines[1].get("detail").is_none());
    }

    #[test]
    fn statusline_a_refused_instance_reads_runs_and_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_running(tmp.path(), Some("cat"));
        let (out, lines) = handled(&instance, false, NeverRead, LONG);
        assert!(out.is_empty());
        assert_eq!(budget(&instance), None);
        assert_eq!(events(&lines), ["hook-invoked", "hook-decision"]);
        assert_eq!(lines[1]["detail"], "strict-modes-failed");
        assert_eq!(lines[1]["budget_written"], false);
        assert!(lines[1].get("deadline_hit").is_none());
    }

    #[test]
    fn statusline_oversize_stdin_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_running(tmp.path(), Some("cat"));
        let over = std::io::repeat(b' ').take(MAX_FRAME + 1);
        let (out, lines) = handled(&instance, true, over, LONG);
        assert!(out.is_empty());
        assert_eq!(budget(&instance), None);
        assert_eq!(
            events(&lines),
            ["hook-invoked", "parse-rejected", "hook-decision"]
        );
        assert_eq!(lines[1]["parser"], "hook-stdin");
        assert_eq!(lines[1]["detail"], "oversize");
        assert_eq!(lines[2]["detail"], "oversize-stdin");
        assert_eq!(lines[2]["budget_written"], false);
    }

    /// Exactly the frame cap is read whole: white space ahead of a payload is still that payload.
    #[test]
    fn statusline_stdin_of_exactly_the_frame_cap_is_taken() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        let cap = usize::try_from(MAX_FRAME).expect("the cap");
        let padded = " ".repeat(cap - PAYLOAD.len()) + PAYLOAD;
        let (_, lines) = handled(&instance, true, padded.as_bytes(), LONG);
        assert_eq!(events(&lines), ["hook-invoked", "hook-decision"]);
        assert_eq!(lines[1]["budget_written"], true);
    }

    #[test]
    fn statusline_malformed_stdin_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_running(tmp.path(), Some("cat"));
        let (out, lines) = handled(&instance, true, b"[1]".as_slice(), LONG);
        assert!(out.is_empty());
        assert_eq!(budget(&instance), None);
        assert_eq!(
            events(&lines),
            ["hook-invoked", "parse-rejected", "hook-decision"]
        );
        assert_eq!(lines[1]["detail"], "malformed");
        assert_eq!(lines[2]["detail"], "malformed-json");
    }

    #[test]
    fn statusline_shell_out_of_a_shell_that_cannot_start_is_not_started() {
        let missing = std::env::temp_dir().join("viola-no-such-shell");
        assert_eq!(
            run_bounded(&mut Command::new(missing), b"x", LONG),
            Shell::NotStarted
        );
    }

    #[cfg(unix)]
    mod shell {
        use super::*;

        fn sh(command: &str) -> [&str; 3] {
            statusline::shell_argv(command).expect("a shell on unix")
        }

        /// The command's stdout reaches `out` byte for byte, ESC bytes and a missing final newline
        /// included, and the command read exactly the payload.
        #[test]
        fn statusline_shell_out_passes_the_output_through_byte_for_byte() {
            let tmp = tempfile::tempdir().expect("tempdir");
            let seen = tmp.path().join("seen");
            let command = format!(
                "cat > '{}'; printf '\\033[31mred\\033[0m \\001 no newline'",
                seen.display()
            );
            let instance = instance_running(tmp.path(), Some(&command));
            let (out, lines) = handled(&instance, true, PAYLOAD.as_bytes(), LONG);
            assert_eq!(out, b"\x1b[31mred\x1b[0m \x01 no newline");
            assert_eq!(
                fs::read(&seen).expect("the command's stdin"),
                PAYLOAD.as_bytes()
            );
            assert_eq!(
                events(&lines),
                [
                    "hook-invoked",
                    "process-start",
                    "process-exit",
                    "hook-decision"
                ]
            );
            assert_eq!(lines[1]["subject"], "statusline-shell");
            assert_eq!(lines[2]["subject"], "statusline-shell");
            assert_eq!(lines[2]["shell_exit_status"], 0);
            assert!(lines[2]["duration_ms"].is_u64());
            assert_eq!(lines[3]["budget_written"], true);
            assert!(lines[3].get("detail").is_none());
            let text = serde_json::to_string(&lines).expect("lines");
            assert!(!text.contains("printf") && !text.contains("canary-chain-value-5c1e"));
        }

        #[test]
        fn statusline_shell_out_a_non_zero_exit_prints_nothing() {
            let tmp = tempfile::tempdir().expect("tempdir");
            let instance = instance_running(tmp.path(), Some("cat; printf shown; exit 3"));
            let (out, lines) = handled(&instance, true, PAYLOAD.as_bytes(), LONG);
            assert!(out.is_empty());
            assert_eq!(lines[2]["event"], "process-exit");
            assert_eq!(lines[2]["shell_exit_status"], 3);
            assert_eq!(lines[3]["budget_written"], true);
            assert!(lines[3].get("detail").is_none());
            assert_eq!(
                shell_out(sh("printf shown; exit 3"), b"", LONG),
                Shell::Exited {
                    code: Some(3),
                    stdout: b"shown".to_vec(),
                }
            );
        }

        /// The command gets the payload as its stdin and the shell gets the command as one
        /// argument: a quote or a semicolon in it is the shell's to read, never viola's.
        #[test]
        fn statusline_shell_out_hands_the_command_to_the_shell_as_one_argument() {
            assert_eq!(
                shell_out(sh("printf '%s|' \"a b\" 'c;d'; cat"), b"stdin-bytes", LONG),
                Shell::Exited {
                    code: Some(0),
                    stdout: b"a b|c;d|stdin-bytes".to_vec(),
                }
            );
        }

        /// A command that never reads a payload larger than a pipe holds is not waited on for it.
        #[test]
        fn statusline_shell_out_a_command_that_never_reads_its_stdin_still_ends() {
            let big = vec![b'x'; 1 << 20];
            assert_eq!(
                shell_out(sh("printf done"), &big, LONG),
                Shell::Exited {
                    code: Some(0),
                    stdout: b"done".to_vec(),
                }
            );
        }

        /// A child the test starts itself through `/bin/sh` and that outlives the bound is ended:
        /// the call returns long before the child's own 5 s.
        #[test]
        fn statusline_shell_out_a_child_that_outlives_the_bound_is_ended_and_reads_as_the_deadline()
        {
            let tmp = tempfile::tempdir().expect("tempdir");
            let instance = instance_running(tmp.path(), Some("printf early; exec sleep 5"));
            let started = Instant::now();
            let (out, lines) = handled(
                &instance,
                true,
                PAYLOAD.as_bytes(),
                Duration::from_millis(50),
            );
            let elapsed = started.elapsed();
            assert!(elapsed >= Duration::from_millis(50), "{elapsed:?}");
            assert!(elapsed < Duration::from_secs(3), "{elapsed:?}");
            assert!(out.is_empty());
            assert_eq!(lines[2]["event"], "process-exit");
            assert!(lines[2].get("shell_exit_status").is_none());
            assert_eq!(lines[3]["detail"], "deadline");
            assert_eq!(lines[3]["deadline_hit"], true);
            assert_eq!(lines[3]["budget_written"], true);
        }

        /// A command that closes its stdout and then outlives the bound is ended at the bound, not
        /// before it.
        #[test]
        fn statusline_shell_out_a_child_that_closes_stdout_and_lingers_is_ended_at_the_bound() {
            let bound = Duration::from_millis(200);
            let started = Instant::now();
            let ran = shell_out(sh("exec 1>&-; exec sleep 5"), b"", bound);
            let elapsed = started.elapsed();
            assert_eq!(ran, Shell::Deadline);
            assert!(elapsed >= bound, "{elapsed:?}");
            assert!(elapsed < Duration::from_secs(3), "{elapsed:?}");
        }

        /// A command that closes its stdout a moment before it exits is still waited for.
        #[test]
        fn statusline_shell_out_a_child_that_closes_stdout_before_it_exits_is_waited_for() {
            assert_eq!(
                shell_out(sh("printf out; exec 1>&-; sleep 0.2; exit 4"), b"", LONG),
                Shell::Exited {
                    code: Some(4),
                    stdout: b"out".to_vec(),
                }
            );
        }
    }
}
