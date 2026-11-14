//! The one bounded `hyprctl` read: the program the caller resolved, spawned directly (never through
//! a shell) with a closed argv and stdin null, stdout read through one byte past the frame cap on
//! its own thread, stderr drained and dropped, killed and reaped at the deadline. It writes one
//! `hyprctl-probe` spawn pair and, on a reply that does not parse, one `parse-rejected` line: codes
//! only, never a reply byte, address, pid or path.

use std::ffi::OsString;
use std::io::Read as _;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use tracing::instrument;
use viola_core::obs::ObsEvent;
use viola_core::{MAX_FRAME, obs_event};

use crate::reply::{self, Reply};
use crate::{HyprlandError, Request};

/// The bound on one reply: the version probe's 5 s, PROVISIONAL until a verb measures its own need.
pub const HYPRCTL_DEADLINE: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(10);
/// How long the output may take to arrive after the child ended: a grandchild still holding the
/// pipe is not waited for.
const DRAIN_GRACE: Duration = Duration::from_millis(500);

/// One read: the parsed reply or why there is none, the child's exit code (none when it never
/// started, was killed at the deadline or ended by a signal) and the raw stdout for the recorder.
#[derive(Debug)]
pub struct Read {
    pub reply: Result<Reply, HyprlandError>,
    pub exit: Option<i32>,
    pub stdout: Vec<u8>,
}

impl Read {
    fn failed(error: HyprlandError, exit: Option<i32>, stdout: Vec<u8>) -> Self {
        Self {
            reply: Err(error),
            exit,
            stdout,
        }
    }
}

/// `program request.args()`, with each name of `remove` taken out of the child's environment.
/// `None` is no program found: `Unsupported`, with no spawn and no line.
#[instrument(skip_all, name = "hyprland.read")]
pub fn read(program: Option<&Path>, request: &Request, remove: &[OsString]) -> Read {
    read_within(program, request, remove, HYPRCTL_DEADLINE)
}

fn read_within(
    program: Option<&Path>,
    request: &Request,
    remove: &[OsString],
    deadline: Duration,
) -> Read {
    let Some(program) = program else {
        return Read::failed(HyprlandError::Unsupported, None, Vec::new());
    };
    let started = Instant::now();
    obs_event!(INFO, ObsEvent::ProcessStart, subject = "hyprctl-probe");
    let ran = run(program, &request.args(), remove, deadline);
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = "hyprctl-probe",
        child_exit_status = ran.as_ref().ok().and_then(|r| r.exit),
        duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    );
    let Ok(ran) = ran else {
        return Read::failed(HyprlandError::Unreachable, None, Vec::new());
    };
    // A child whose reply ran past the cap is cut off by the read, so its exit says nothing.
    let reply = if ran.stdout.len() as u64 > MAX_FRAME {
        Err(HyprlandError::Oversize)
    } else if ran.exit != Some(0) {
        return Read::failed(HyprlandError::Unreachable, ran.exit, ran.stdout);
    } else {
        reply::parse(request, &ran.stdout)
    };
    if let Err(error @ (HyprlandError::Oversize | HyprlandError::Malformed)) = reply {
        obs_event!(
            WARN,
            ObsEvent::ParseRejected,
            parser = "hyprctl-json",
            detail = error.code(),
            count = 1u64,
        );
    }
    Read {
        reply,
        exit: ran.exit,
        stdout: ran.stdout,
    }
}

struct Ran {
    exit: Option<i32>,
    stdout: Vec<u8>,
}

fn run(
    program: &Path,
    args: &[OsString],
    remove: &[OsString],
    deadline: Duration,
) -> std::io::Result<Ran> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in remove {
        command.env_remove(name);
    }
    let mut child = command.spawn()?;
    let stdout = drain(child.stdout.take(), MAX_FRAME + 1);
    let _stderr = drain(child.stderr.take(), MAX_FRAME);
    let until = Instant::now() + deadline;
    let exit = loop {
        if let Some(status) = child.try_wait()? {
            break status.code();
        }
        if Instant::now() >= until {
            end(&mut child);
            break None;
        }
        std::thread::sleep(POLL);
    };
    Ok(Ran {
        exit,
        stdout: stdout
            .and_then(|rx| rx.recv_timeout(DRAIN_GRACE).ok())
            .unwrap_or_default(),
    })
}

fn end(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

/// A pipe read to its end (at most `cap` bytes) on its own thread.
fn drain(
    pipe: Option<impl std::io::Read + Send + 'static>,
    cap: u64,
) -> Option<mpsc::Receiver<Vec<u8>>> {
    pipe.map(|pipe| {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.take(cap).read_to_end(&mut bytes);
            let _ = tx.send(bytes);
        });
        rx
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reply::Window;
    use crate::test_capture::capture;
    use serde_json::{Value, json};
    use std::io::Write as _;
    use std::os::unix::fs::PermissionsExt as _;
    use std::path::PathBuf;

    const CANARY: &str = "CANARY-7f3a-not-a-log-word";

    /// A shell script planted executable in `dir`, every handle closed before any spawn.
    fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = std::fs::File::create(&path).expect("create");
        file.write_all(format!("#!/bin/sh\n{body}\n").as_bytes())
            .expect("write");
        file.sync_all().expect("sync");
        drop(file);
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
        path
    }

    fn events(lines: &[Value]) -> Vec<&str> {
        lines
            .iter()
            .map(|l| l["event"].as_str().expect("an event"))
            .collect()
    }

    #[test]
    fn read_without_a_program_is_unsupported_and_writes_no_line() {
        let (got, lines) = capture(|| read(None, &Request::Clients, &[]));
        assert_eq!(got.reply, Err(HyprlandError::Unsupported));
        assert_eq!(got.exit, None);
        assert!(got.stdout.is_empty());
        assert!(lines.is_empty(), "{lines:?}");
    }

    #[test]
    fn read_of_a_program_that_cannot_start_is_unreachable_with_the_spawn_pair_only() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let absent = tmp.path().join("absent");
        let (got, lines) = capture(|| read(Some(&absent), &Request::Clients, &[]));
        assert_eq!(got.reply, Err(HyprlandError::Unreachable));
        assert_eq!(got.exit, None);
        assert_eq!(events(&lines), ["process-start", "process-exit"]);
        assert_eq!(lines[1]["subject"], json!("hyprctl-probe"));
        assert_eq!(lines[1].get("child_exit_status"), None);
    }

    /// `echo` prints its arguments and exits 0: a text that is not JSON reads malformed, with one
    /// `parse-rejected` line holding no reply byte.
    #[test]
    fn read_of_a_program_whose_reply_is_no_json_is_malformed() {
        let (got, lines) = capture(|| read(Some(Path::new("/bin/echo")), &Request::Clients, &[]));
        assert_eq!(got.reply, Err(HyprlandError::Malformed));
        assert_eq!(got.exit, Some(0));
        assert_eq!(got.stdout, b"-j clients\n");
        assert_eq!(
            events(&lines),
            ["process-start", "process-exit", "parse-rejected"]
        );
        assert_eq!(
            lines[0],
            json!({"event": "process-start", "subject": "hyprctl-probe", "level": "INFO",
                   "message": "process-start"})
        );
        let exit = &lines[1];
        assert_eq!(exit["subject"], json!("hyprctl-probe"));
        assert_eq!(exit["child_exit_status"], json!(0));
        assert!(exit["duration_ms"].is_u64());
        assert_eq!(
            lines[2],
            json!({"event": "parse-rejected", "parser": "hyprctl-json", "detail": "malformed",
                   "count": 1, "level": "WARN", "message": "parse-rejected"})
        );
    }

    /// `sh` given `-j` refuses the option and exits non-zero: a failed read, never a parse.
    #[test]
    fn read_of_a_failing_program_is_unreachable_and_keeps_its_exit() {
        let (got, lines) = capture(|| read(Some(Path::new("/bin/sh")), &Request::Clients, &[]));
        assert_eq!(got.reply, Err(HyprlandError::Unreachable));
        assert!(got.exit.is_some_and(|code| code != 0), "{:?}", got.exit);
        assert_eq!(events(&lines), ["process-start", "process-exit"]);
    }

    #[test]
    fn read_parses_a_reply_and_no_reply_text_reaches_a_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let reply = format!(r#"[{{"address":"0x1a","pid":7,"class":"{CANARY}"}}]"#);
        let program = script(tmp.path(), "hyprctl", &format!("printf '%s' '{reply}'"));
        let (got, lines) = capture(|| read(Some(&program), &Request::Clients, &[]));
        let Ok(Reply::Clients(windows)) = got.reply else {
            panic!("windows: {:?}", got.reply);
        };
        assert_eq!(windows.len(), 1);
        let Window { address, pid, .. } = &windows[0];
        assert_eq!((address.as_ref(), *pid), ("0x1a", Some(7)));
        assert_eq!(got.stdout, reply.as_bytes());
        assert_eq!(events(&lines), ["process-start", "process-exit"]);
        let written = serde_json::to_string(&lines).expect("lines");
        assert!(!written.contains(CANARY));
        assert!(!written.contains("0x1a"));
        assert!(!written.contains(&program.display().to_string()));
    }

    #[test]
    fn read_takes_each_named_variable_out_of_the_childs_environment() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let program = script(
            tmp.path(),
            "hyprctl",
            "if [ -n \"${CARGO_MANIFEST_DIR+set}\" ]; then printf ok; else printf gone; fi",
        );
        let request = Request::Workspace(crate::WorkspaceId::new(1));
        let kept = read(Some(&program), &request, &[]);
        assert_eq!(kept.reply, Ok(Reply::Dispatched));
        let removed = read(
            Some(&program),
            &request,
            &[OsString::from("CARGO_MANIFEST_DIR")],
        );
        assert_eq!(removed.stdout, b"gone");
        assert_eq!(removed.reply, Err(HyprlandError::Refused));
    }

    /// A reply one byte past the frame cap is refused whole, never parsed truncated.
    #[test]
    fn read_of_a_reply_past_the_cap_is_oversize() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let over = MAX_FRAME + 1;
        let program = script(
            tmp.path(),
            "hyprctl",
            &format!(
                "printf '['; head -c {} /dev/zero | tr '\\0' ' '; printf ']'",
                over
            ),
        );
        let (got, lines) = capture(|| read(Some(&program), &Request::Clients, &[]));
        assert_eq!(got.reply, Err(HyprlandError::Oversize));
        assert_eq!(got.stdout.len() as u64, MAX_FRAME + 1);
        assert_eq!(lines[2]["detail"], json!("oversize"));
    }

    #[test]
    fn read_past_the_deadline_kills_and_reaps_its_child() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pid_file = tmp.path().join("pid");
        let program = script(
            tmp.path(),
            "hyprctl",
            &format!("echo $$ > '{}'; exec /bin/sleep 30", pid_file.display()),
        );
        let started = Instant::now();
        let got = read_within(
            Some(&program),
            &Request::Clients,
            &[],
            Duration::from_millis(200),
        );
        let took = started.elapsed();
        assert_eq!(got.reply, Err(HyprlandError::Unreachable));
        assert_eq!(got.exit, None);
        let pid = std::fs::read_to_string(&pid_file).expect("the child wrote its pid");
        let proc = PathBuf::from(format!("/proc/{}", pid.trim()));
        assert!(!proc.exists(), "the child {} outlived the read", pid.trim());
        assert!(took < Duration::from_secs(7), "took {took:?}");
    }
}
