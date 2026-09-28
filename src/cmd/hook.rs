//! `viola hook <event>` (hidden): the hook payload on stdin becomes one `hook.event` for the
//! wrapper that owns the session. It fails open on every path: exit 0, nothing on stdout or stderr,
//! and outside a wrapped session nothing written anywhere (architecture [Hook Contract]; obs-plan
//! §4 E1). With `--capture <dir>` (the `viola verify` probe plugin) it only files the raw payload.

use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use chrono::Utc;
use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_agent_claude::hook::{HookEvent, Normalised};
use viola_agent_claude::ledger::{capture_file_name, parse_capture_file_name};
use viola_channel::Client;
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{MAX_FRAME, ViolaName, obs_event};
use viola_state::events::{EventLine, Source, try_append_event};
use viola_state::fs::{FILE_MODE, replace_private};
use viola_state::snapshot::read_snapshot;

use super::Failure;
use crate::obs::{self, DetailSink};

mod seam;

#[derive(clap::Args)]
pub(crate) struct HookArgs {
    /// The hook event, kebab-case
    event: String,
    /// File the raw payload into this directory and do nothing else
    #[arg(long, hide = true, value_name = "DIR")]
    capture: Option<PathBuf>,
}

/// The hook's whole budget from its start. Provisional: below test-plan §10's 1.0 s spine gate
/// until the product constant is named.
const SPINE_DEADLINE: Duration = Duration::from_millis(750);

fn spine_deadline(started: Instant) -> Instant {
    started + SPINE_DEADLINE
}

/// The wrapped session this process belongs to.
#[derive(Debug, PartialEq, Eq)]
struct Instance {
    name: ViolaName,
    dir: PathBuf,
    home: PathBuf,
}

/// `VIOLA_NAME` and `VIOLA_DIR` as `viola run` sets them in its child: a valid name, and an
/// absolute dir that is `<home>/instances/<name>`. Anything else is not a wrapped session.
fn instance_of(name: Option<OsString>, dir: Option<OsString>) -> Option<Instance> {
    let name = ViolaName::try_new(name?.into_string().ok()?).ok()?;
    let dir = PathBuf::from(dir?);
    if !dir.is_absolute() {
        return None;
    }
    let own: &str = name.as_ref();
    let instances = dir
        .parent()
        .filter(|_| dir.file_name() == Some(OsStr::new(own)))?;
    let home = instances
        .parent()
        .filter(|_| instances.file_name() == Some(OsStr::new("instances")))?
        .to_path_buf();
    Some(Instance { name, dir, home })
}

pub(crate) fn hook(args: &HookArgs) -> Result<ExitCode, Failure> {
    let started = Instant::now();
    let Some(event) = HookEvent::from_arg(&args.event) else {
        return Ok(ExitCode::SUCCESS);
    };
    if let Some(dir) = &args.capture {
        capture(event, dir, &mut std::io::stdin().lock());
        return Ok(ExitCode::SUCCESS);
    }
    let instance = instance_of(
        std::env::var_os("VIOLA_NAME"),
        std::env::var_os("VIOLA_DIR"),
    );
    let Some(instance) = instance else {
        return Ok(ExitCode::SUCCESS);
    };
    let (level, _) = obs::read_diagnostics_level(&instance.home);
    // A role file that will not open leaves no subscriber, so every line goes nowhere and the hook
    // carries on (obs-plan §3 step 6, D-29).
    let _ = obs::viola_obs_init(
        &instance.home,
        ObsProcess::Hook,
        Some(instance.name.clone()),
        level,
    );
    #[cfg(feature = "fake-agent")]
    seam::panic_if_asked();
    let stdin = &mut std::io::stdin().lock();
    match handle(event, &instance, spine_deadline(started), stdin) {
        Ok(detail) => {
            decided(event, detail, started);
            Ok(ExitCode::SUCCESS)
        }
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

/// The hook's body; `Some(detail)` names the fail-open path it took.
#[instrument(skip_all, name = "hook.handle", fields(hook_event = event.as_str()))]
fn handle(
    event: HookEvent,
    instance: &Instance,
    deadline: Instant,
    stdin: &mut dyn Read,
) -> anyhow::Result<Option<&'static str>> {
    let invoked_at = obs::timestamp(Utc::now());
    obs_event!(
        INFO,
        ObsEvent::HookInvoked,
        hook_event = event.as_str(),
        invoked_at = invoked_at.as_str(),
    );
    let mut bytes = Vec::new();
    stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {
        reject_stdin("oversize");
        return Ok(Some("oversize-stdin"));
    }
    let Ok(normalised) = viola_agent_claude::hook::normalise(event, &bytes) else {
        reject_stdin("malformed");
        return Ok(Some("malformed-json"));
    };
    if !normalised.drift.is_empty() {
        write_drift(instance, &normalised.drift);
    }
    if deliver(instance, &normalised, deadline) {
        return Ok(None);
    }
    if event == HookEvent::SessionEnd {
        append_session_end(instance, normalised);
    }
    Ok(Some("channel-unreachable"))
}

/// The raw payload into the first free `<dir>/<Event>.<k>.json`, and nothing else: no `VIOLA_*`
/// read, no log, no channel. `dir` must be an absolute, existing directory; every failure writes
/// nothing.
fn capture(event: HookEvent, dir: &Path, stdin: &mut dyn Read) -> Option<PathBuf> {
    if !dir.is_absolute() || !dir.is_dir() {
        return None;
    }
    let mut bytes = Vec::new();
    stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
    let path = dir.join(capture_file_name(event, free_k(dir)?));
    replace_private(&path, &bytes, FILE_MODE).ok()?;
    Some(path)
}

/// The first `k` no capture of any event holds, so the names sort in arrival order. `n` captures
/// leave one of `1..=n + 1` free, so the search is bounded.
fn free_k(dir: &Path) -> Option<u32> {
    let taken: Vec<u32> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|e| parse_capture_file_name(e.file_name().to_str()?).map(|(_, k)| k))
        .collect();
    let n = u32::try_from(taken.len()).ok()?;
    (1..=n + 1).find(|k| !taken.contains(k))
}

fn reject_stdin(detail: &'static str) {
    obs_event!(
        WARN,
        ObsEvent::ParseRejected,
        parser = "hook-stdin",
        detail = detail,
        count = 1u64,
    );
}

/// The drift report goes only to the instance detail file, never through the subscriber
/// (obs-plan §8).
fn write_drift(instance: &Instance, drift: &[Value]) {
    let mut fields = Map::new();
    fields.insert("drift_report".to_owned(), Value::Array(drift.to_vec()));
    let line = obs::detail_line(
        &obs::timestamp(Utc::now()),
        "WARN",
        "viola::cmd::hook",
        ObsEvent::ParseRejected,
        ObsProcess::Hook,
        &instance.name,
        fields,
    );
    obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
}

/// The event to the endpoint `snapshot.json` records, by `deadline`. Until the server check lands
/// (Epoch 6), the recorded endpoint is trusted as it stands.
fn deliver(instance: &Instance, normalised: &Normalised, deadline: Instant) -> bool {
    let Some(endpoint) = read_snapshot(&instance.dir).and_then(|s| s.endpoint) else {
        return false;
    };
    let Ok(mut client) = Client::connect_by(&endpoint, "hook", deadline) else {
        return false;
    };
    let mut params = Map::new();
    params.insert("ts".to_owned(), obs::timestamp(Utc::now()).into());
    params.insert(
        "event".to_owned(),
        json!({"kind": normalised.kind.as_str(), "data": normalised.data}),
    );
    client.notify("hook.event", params).is_ok()
}

/// A SessionEnd the channel did not take is appended directly, unless another writer holds the
/// log (architecture [Hook Transport]).
fn append_session_end(instance: &Instance, normalised: Normalised) {
    let line = EventLine::new(
        &instance.name,
        normalised.kind,
        Source::Hook,
        normalised.data,
        Utc::now(),
    );
    let _ = try_append_event(&instance.dir, &line);
}

fn decided(event: HookEvent, detail: Option<&'static str>, started: Instant) {
    let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    match detail {
        Some(detail) => obs_event!(
            WARN,
            ObsEvent::HookDecision,
            hook_event = event.as_str(),
            decision_emitted = false,
            duration_ms = duration_ms,
            detail = detail,
        ),
        None => obs_event!(
            INFO,
            ObsEvent::HookDecision,
            hook_event = event.as_str(),
            decision_emitted = false,
            duration_ms = duration_ms,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;
    use std::path::Path;

    fn os(s: &str) -> Option<OsString> {
        Some(OsString::from(s))
    }

    fn instance_in(root: &Path) -> Instance {
        let home = root.join("home");
        let dir = home.join("instances").join("builder");
        fs::create_dir_all(&dir).expect("instance dir");
        Instance {
            name: ViolaName::try_new("builder".to_owned()).expect("valid"),
            dir,
            home,
        }
    }

    #[test]
    fn spine_deadline_is_750_ms_after_the_start() {
        let start = Instant::now();
        assert_eq!(
            spine_deadline(start).saturating_duration_since(start),
            Duration::from_millis(750)
        );
    }

    #[test]
    fn instance_of_takes_the_home_two_levels_above_the_dir() {
        let home = std::env::temp_dir().join("h");
        let dir = home.join("instances").join("builder");
        let got = instance_of(os("builder"), Some(dir.clone().into_os_string()));
        assert_eq!(
            got,
            Some(Instance {
                name: ViolaName::try_new("builder".to_owned()).expect("valid"),
                dir,
                home,
            })
        );
    }

    /// Each dir is taken under an absolute base, except `relative`, which is taken as given.
    #[rstest]
    #[case::no_name(None, Some("h/instances/builder"))]
    #[case::no_dir(os("builder"), None)]
    #[case::invalid_name(os("Builder"), Some("h/instances/Builder"))]
    #[case::other_name(os("builder"), Some("h/instances/overseer"))]
    #[case::not_under_instances(os("builder"), Some("h/sessions/builder"))]
    #[case::the_instances_dir(os("builder"), Some("h/instances"))]
    #[case::relative(os("builder"), Some("relative"))]
    fn instance_of_refuses_anything_but_a_wrapped_dir(
        #[case] name: Option<OsString>,
        #[case] dir: Option<&str>,
    ) {
        let dir = dir.map(|d| match d {
            "relative" => Path::new("h").join("instances").join("builder"),
            d => std::env::temp_dir().join(d),
        });
        assert_eq!(instance_of(name, dir.map(PathBuf::into_os_string)), None);
    }

    fn handled(event: HookEvent, instance: &Instance, stdin: &[u8]) -> Option<&'static str> {
        let deadline = spine_deadline(Instant::now());
        handle(event, instance, deadline, &mut &stdin[..]).expect("handled")
    }

    #[test]
    fn handle_refuses_oversize_stdin_and_reads_a_frame_at_the_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let mut body = b"{}".to_vec();
        body.resize(cap + 1, b' ');
        assert_eq!(
            handled(HookEvent::Stop, &instance, &body),
            Some("oversize-stdin")
        );
        body.truncate(cap);
        assert_eq!(
            handled(HookEvent::Stop, &instance, &body),
            Some("channel-unreachable")
        );
    }

    #[test]
    fn handle_refuses_a_payload_that_is_not_an_object() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        assert_eq!(
            handled(HookEvent::Stop, &instance, b"[1]"),
            Some("malformed-json")
        );
        assert!(!instance.dir.join("diagnostics").exists());
    }

    /// A wrong-typed known field leaves one `drift_report` line in `detail-hook.ndjson`: its path
    /// and expected type, never the value.
    #[test]
    fn handle_writes_a_drift_report_only_to_the_detail_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        let body = br#"{"session_id": ["canary-chain-value-5c1e"], "source": "startup"}"#;
        assert_eq!(
            handled(HookEvent::SessionStart, &instance, body),
            Some("channel-unreachable")
        );
        let path = obs::detail_path(&instance.home, &instance.name, ObsProcess::Hook);
        let text = fs::read_to_string(path).expect("detail file");
        assert_eq!(text.lines().count(), 1);
        let line: Value = serde_json::from_str(text.trim_end()).expect("json");
        assert_eq!(line["event"], "parse-rejected");
        assert_eq!(line["process"], "hook");
        assert_eq!(
            line["drift_report"],
            json!([{"path": "session_id", "expected": "string"}])
        );
        assert!(!text.contains("canary-chain-value-5c1e"));
        assert!(crate::test_support::diag_detail_validator().is_valid(&line));
        assert!(!instance.home.join("diagnostics").exists());
    }

    #[test]
    fn handle_without_drift_writes_no_detail_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        assert_eq!(
            handled(
                HookEvent::Stop,
                &instance,
                br#"{"last_assistant_message": "x"}"#
            ),
            Some("channel-unreachable")
        );
        assert!(!instance.dir.join("diagnostics").exists());
        assert!(!instance.dir.join("events.ndjson").exists());
    }

    #[test]
    fn capture_files_each_payload_under_the_next_free_k() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let first = capture(HookEvent::SessionStart, tmp.path(), &mut &b"{\"a\":1}"[..]);
        assert_eq!(first, Some(tmp.path().join("SessionStart.1.json")));
        assert_eq!(
            fs::read(tmp.path().join("SessionStart.1.json")).expect("capture"),
            b"{\"a\":1}"
        );
        fs::write(tmp.path().join("notes.txt"), b"x").expect("unrelated file");
        fs::write(tmp.path().join("Stop.3.json"), b"x").expect("a later k");
        let second = capture(HookEvent::Stop, tmp.path(), &mut &b"not json"[..]);
        assert_eq!(second, Some(tmp.path().join("Stop.2.json")));
        assert_eq!(
            fs::read(tmp.path().join("Stop.2.json")).expect("raw"),
            b"not json"
        );
        let third = capture(HookEvent::Stop, tmp.path(), &mut &b""[..]);
        assert_eq!(third, Some(tmp.path().join("Stop.4.json")));
    }

    #[test]
    fn capture_keeps_one_byte_past_the_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let body = vec![b' '; cap + 5];
        let path = capture(HookEvent::Stop, tmp.path(), &mut &body[..]).expect("written");
        assert_eq!(fs::metadata(path).expect("meta").len(), MAX_FRAME + 1);
    }

    struct Failing;

    impl Read for Failing {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("closed"))
        }
    }

    #[test]
    fn capture_refuses_a_bad_dir_or_stdin_and_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let relative = Path::new("viola-capture-relative-5c1e");
        assert_eq!(capture(HookEvent::Stop, relative, &mut &b"{}"[..]), None);
        assert!(!relative.exists());
        // A relative dir that exists is refused too: only an absolute path is taken.
        assert_eq!(
            capture(HookEvent::Stop, Path::new("."), &mut &b"{}"[..]),
            None
        );
        assert!(!Path::new("Stop.1.json").exists());
        let missing = tmp.path().join("missing");
        assert_eq!(capture(HookEvent::Stop, &missing, &mut &b"{}"[..]), None);
        assert!(!missing.exists());
        let file = tmp.path().join("file");
        fs::write(&file, b"x").expect("a file where the dir should be");
        assert_eq!(capture(HookEvent::Stop, &file, &mut &b"{}"[..]), None);
        let dir = tmp.path().join("dir");
        fs::create_dir(&dir).expect("dir");
        assert_eq!(capture(HookEvent::Stop, &dir, &mut Failing), None);
        assert_eq!(fs::read_dir(&dir).expect("dir").count(), 0);
    }

    /// Only SessionEnd falls back to a direct append when the channel is out of reach.
    #[test]
    fn handle_appends_only_a_session_end_the_channel_did_not_take() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance = instance_in(tmp.path());
        assert_eq!(
            handled(HookEvent::SessionEnd, &instance, br#"{"reason": "exit"}"#),
            Some("channel-unreachable")
        );
        let text = fs::read_to_string(instance.dir.join("events.ndjson")).expect("events");
        let line: Value = serde_json::from_str(text.trim_end()).expect("json");
        assert_eq!(text.lines().count(), 1);
        assert_eq!(line["kind"], "session-end");
        assert_eq!(line["source"], "hook");
        assert_eq!(line["instance"], "builder");
        assert_eq!(line["data"], json!({}));
    }
}
