//! Security control negatives, Linux and Windows (test-plan §6 Security control negatives;
//! security-plan §Authentication & Authorization, IPC client-side server verification): each CLI
//! frame verb checks, before its first frame, that the process serving the endpoint is the wrapper
//! `snapshot.json` records by pid and start time. The impostor is a real second process: the
//! snapshot, written by the writer, records the fake agent held as a child while this test process
//! serves the endpoint. Every verb then exits 21 `instance-unreachable` and writes no frame; the
//! control, a snapshot recording the serving process itself, reaches it. macOS's peer credentials
//! carry no pid, the recorded gap.
#![cfg(not(target_os = "macos"))]

#[allow(dead_code)]
mod support;

use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::viola;
use support::fake::FAKE;
use support::home::TestHome;
use viola_channel::{Dispatch, ProtocolError, Server, Serving};
use viola_state::snapshot::{InstanceSnapshot, Wheel, write_snapshot};

const UNREACHABLE: &str = "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n";
const HINT: &str =
    "hint: the process on builder's endpoint is not its recorded wrapper; it was not contacted";

/// The fake agent's own holder mode: it sleeps, then exits, writing nothing.
const HOLD: &str = "--hold-stdout-internal";

/// The fake agent held alive: a live process that is not this one, killed and reaped on drop.
struct Held(Child);

impl Held {
    fn spawn() -> Self {
        let child = Command::new(FAKE)
            .arg(HOLD)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("the held fake agent");
        Self(child)
    }
}

impl Drop for Held {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Every frame the test server was sent, answered with each verb's `ok`.
#[derive(Default)]
struct Frames(Mutex<Vec<String>>);

impl Dispatch for Frames {
    fn dispatch(&self, method: &str, _: &Value) -> Result<Value, ProtocolError> {
        self.0.lock().expect("frames").push(method.to_owned());
        Ok(match method {
            "send" => json!({"ok": {"submitted_at": "2026-10-26T00:00:00.000Z", "cursor": 1}}),
            "wait" => json!({"ok": {"timed_out": true}}),
            "last" => {
                json!({"ok": {"last_assistant_message": "done", "ts": "2026-10-26T00:00:00.000Z"}})
            }
            "pause" => json!({"ok": {"wheel": "human"}}),
            "release" => json!({"ok": {"wheel": "driver", "budget_paused": false}}),
            "link" => {
                json!({"ok": {"since": "2026-10-26T00:00:00.000Z", "already_linked": false}})
            }
            "unlink" => json!({"ok": {}}),
            _ => json!({"ok": {}}),
        })
    }
}

/// A home whose instance `builder` names an endpoint this process serves, its snapshot recording
/// `recorded` (the pid, and that process's own start time) as the wrapper, its heartbeat fresh.
struct Served {
    home: TestHome,
    endpoint: String,
    frames: Arc<Frames>,
    _socket_dir: tempfile::TempDir,
    _serving: Serving,
}

fn served(label: &str, recorded: u32) -> Served {
    let home = TestHome::new();
    let instance = home.path().join("instances").join("builder");
    viola_state::fs::create_private_dir(&instance).expect("the instance dir");
    let socket_dir = tempfile::tempdir().expect("the socket dir");
    let base = format!("viola-test-chan-{}-{label}", std::process::id());
    let endpoint = if cfg!(windows) {
        format!(r"\\.\pipe\{base}")
    } else {
        socket_dir
            .path()
            .join(format!("{base}.sock"))
            .to_string_lossy()
            .into_owned()
    };
    let frames = Arc::new(Frames::default());
    let serving = Server::bind(&endpoint)
        .expect("bound")
        .serve(Arc::clone(&frames) as Arc<dyn Dispatch>);
    let snapshot = InstanceSnapshot {
        endpoint: Some(endpoint.clone()),
        pid: recorded,
        started_at: viola_state::liveness::own_start(recorded).expect("a live recorded process"),
        pinned_bin: String::new(),
        cli_verified: true,
        cli_version: None,
        wheel: Wheel::Driver,
        budget_paused: false,
        budget_override_until: None,
        links: Vec::new(),
        child_pid: None,
        pending_dialog: None,
        cwd: None,
        statusline_command: None,
        agent_session_id: None,
    };
    write_snapshot(&instance, &snapshot).expect("the snapshot");
    viola_state::heartbeat::touch_heartbeat(&instance).expect("the heartbeat");
    Served {
        home,
        endpoint,
        frames,
        _socket_dir: socket_dir,
        _serving: serving,
    }
}

impl Served {
    fn frames(&self) -> Vec<String> {
        self.frames.0.lock().expect("frames").clone()
    }

    fn cli_lines(&self) -> Vec<Value> {
        support::ndjson::read_lines(
            &self
                .home
                .path()
                .join("diagnostics")
                .join("cli-builder.ndjson"),
        )
    }
}

/// The verb's arguments after `--home <home>`, and its stdin.
fn verb(name: &str, json: bool) -> (Vec<&'static str>, Option<&'static str>) {
    let (mut args, stdin): (Vec<&'static str>, _) = match name {
        "send" => (vec!["send", "builder"], Some("hello\n")),
        "wait" => (vec!["wait", "builder", "--timeout-ms", "500"], None),
        "last" => (vec!["last", "builder"], None),
        "answer" => (
            vec!["answer", "builder", "1"],
            Some("{\"behavior\": \"allow\"}"),
        ),
        "pause" => (vec!["pause", "builder"], None),
        "release" => (vec!["release", "builder"], None),
        "link" => (vec!["link", "overseer", "builder"], None),
        "unlink" => (vec!["unlink", "overseer", "builder"], None),
        other => panic!("no verb {other}"),
    };
    if json {
        args.push("--json");
    }
    (args, stdin)
}

/// The digit runs of `text`: a pid shows as one of them, never inside a longer number.
fn digit_runs(text: &str) -> Vec<&str> {
    text.split(|c: char| !c.is_ascii_digit())
        .filter(|run| !run.is_empty())
        .collect()
}

/// With an impostor serving the endpoint each verb exits 21 and writes no frame: under `--json`
/// exactly the `instance-unreachable` document and stderr empty; in human mode stdout empty, the
/// server-verification hint last on stderr and no SGR byte. Its `cli` log holds one
/// `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN and no
/// `channel-request`; no output holds the recorded pid, the home's path or the endpoint.
#[rstest]
#[case::send_json("send", true)]
#[case::send_human("send", false)]
#[case::wait_json("wait", true)]
#[case::wait_human("wait", false)]
#[case::last_json("last", true)]
#[case::last_human("last", false)]
#[case::answer_json("answer", true)]
#[case::answer_human("answer", false)]
#[case::pause_json("pause", true)]
#[case::pause_human("pause", false)]
#[case::release_json("release", true)]
#[case::release_human("release", false)]
#[case::link_json("link", true)]
#[case::link_human("link", false)]
#[case::unlink_json("unlink", true)]
#[case::unlink_human("unlink", false)]
fn security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper(
    #[case] name: &str,
    #[case] json: bool,
) {
    let held = Held::spawn();
    let pid = held.0.id();
    let served = served(&format!("{name}-{json}"), pid);
    let (args, stdin) = verb(name, json);
    let ran = viola(served.home.path(), &args, stdin, None);

    assert_eq!(ran.code, Some(21), "{name}: {}", ran.stderr);
    if json {
        assert_eq!(ran.stdout, UNREACHABLE, "{name}");
        assert_eq!(ran.stderr, "", "{name}");
    } else {
        assert_eq!(ran.stdout, "", "{name}");
        assert_eq!(
            ran.stderr.lines().last(),
            Some(HINT),
            "{name}: {}",
            ran.stderr
        );
        assert!(ran.stderr.contains("instance-unreachable"), "{name}");
        assert!(!ran.stderr.contains('\u{1b}'), "{name}: an SGR byte");
    }
    assert!(served.frames().is_empty(), "{name}: {:?}", served.frames());

    let lines = served.cli_lines();
    let exits: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), 1, "{name}: {lines:?}");
    let exit = exits[0];
    assert_eq!(exit["level"], "WARN");
    assert_eq!(exit["subject"], "self");
    assert_eq!(exit["exit_code"], 21);
    assert_eq!(exit["detail"], "server-verify-failed");
    assert_eq!(exit["during"], "connect");
    assert!(
        !lines.iter().any(|l| l["event"] == "channel-request"),
        "{name}: a channel-request"
    );

    let home = served.home.path().to_string_lossy().into_owned();
    let log = std::fs::read_to_string(
        served
            .home
            .path()
            .join("diagnostics")
            .join("cli-builder.ndjson"),
    )
    .expect("the cli log");
    let pid = pid.to_string();
    for (what, text) in [
        ("stdout", ran.stdout.as_str()),
        ("stderr", ran.stderr.as_str()),
        ("log", log.as_str()),
    ] {
        assert!(
            !digit_runs(text).contains(&pid.as_str()),
            "{name}: {what} holds the pid"
        );
        assert!(!text.contains(&home), "{name}: {what} holds the home");
        assert!(
            !text.contains(&served.endpoint),
            "{name}: {what} holds the endpoint"
        );
    }
    drop(held);
}

/// The control: the snapshot records the serving process itself, so each verb reaches it with one
/// frame and exits 0.
#[rstest]
#[case::send("send")]
#[case::wait("wait")]
#[case::last("last")]
#[case::answer("answer")]
#[case::pause("pause")]
#[case::release("release")]
#[case::link("link")]
#[case::unlink("unlink")]
fn security_negative_cli_verb_reaches_the_recorded_wrapper(#[case] name: &str) {
    let served = served(&format!("{name}-control"), std::process::id());
    for json in [true, false] {
        let (args, stdin) = verb(name, json);
        let ran = viola(served.home.path(), &args, stdin, None);
        assert_eq!(ran.code, Some(0), "{name} json={json}: {}", ran.stderr);
    }
    assert_eq!(served.frames(), [name, name], "{name}");
    assert!(
        served
            .cli_lines()
            .iter()
            .all(|l| l["detail"] != "server-verify-failed"),
        "{name}"
    );
}

/// The cases' own oracle: a pid is found as a whole digit run, never inside a longer number.
#[test]
fn digit_runs_reads_whole_numbers_only() {
    assert_eq!(digit_runs("pid 412, t=14120.5"), ["412", "14120", "5"]);
    assert!(!digit_runs("14120").contains(&"412"));
}
