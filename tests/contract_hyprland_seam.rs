//! The Hyprland seam held to the recorded set (test-plan · decision/fixture-drift-byte-compare): the
//! fake agent planted as `hyprctl` with a recorded reply in its set prints that reply's `stdout` byte
//! for byte and exits its `exit`, and `viola_hyprland::read` over it parses each reply into its type.
//! The fake's other modes read the closed failures with codes-only lines, and no reply text reaches a
//! line or an error. Every read runs under its own 5 s deadline; the fake is a test-scoped copy,
//! never the host's `hyprctl`.

#[allow(dead_code)]
mod support;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};
use support::fake::hyprctl_bin;
use support::home::workspace_path;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
use viola_channel::test_support::JsonFields;
use viola_hyprland::{
    Address, HyprlandError, Reply, Request, WorkspaceId, read, reply::HyprlandVersion,
};

/// A text no recorded reply holds, planted where a window's text would stand.
const CANARY: &str = "viola-canary-hypr-9d41";

/// Every line a closure wrote, its fields as JSON, on the calling thread.
#[derive(Clone, Default)]
struct Lines(Arc<Mutex<Vec<Value>>>);

impl<S: Subscriber> Layer<S> for Lines {
    fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
        let mut fields = JsonFields::default();
        event.record(&mut fields);
        self.0.lock().expect("lines").push(Value::Object(fields.0));
    }
}

fn captured<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
    let lines = Lines::default();
    let result =
        tracing::subscriber::with_default(tracing_subscriber::registry().with(lines.clone()), f);
    let got = lines.0.lock().expect("lines").clone();
    (result, got)
}

/// `fixtures/hyprland/*/*.json`, sorted: the recorded set.
fn recorded() -> Vec<PathBuf> {
    let root = workspace_path("fixtures/hyprland");
    let mut found: Vec<PathBuf> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|version| version.path().is_dir())
        .flat_map(|version| {
            std::fs::read_dir(version.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "json"))
        .collect();
    found.sort();
    found
}

fn doc(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("fixture")).expect("fixture JSON")
}

fn is_absent(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.ends_with(".absent-1.json"))
}

/// The request a recorded file names, a dispatch built from its own recorded argv.
fn request_of(doc: &Value) -> Request {
    let args: Vec<&str> = doc["args"]
        .as_array()
        .expect("args")
        .iter()
        .map(|a| a.as_str().expect("an arg"))
        .collect();
    match doc["request"].as_str().expect("a request word") {
        "clients" => Request::Clients,
        "active-window" => Request::ActiveWindow,
        "workspaces" => Request::Workspaces,
        "version" => Request::Version,
        "focus-window" => {
            let address = args[1]
                .strip_prefix(r#"hl.dsp.focus({ window = "address:"#)
                .and_then(|rest| rest.strip_suffix(r#"" })"#))
                .expect("a window focus dispatch");
            Request::FocusWindow(Address::try_new(address.to_owned()).expect("an address"))
        }
        "workspace" => {
            let id = args[1]
                .strip_prefix("hl.dsp.focus({ workspace = ")
                .and_then(|rest| rest.strip_suffix(" })"))
                .expect("a workspace focus dispatch");
            Request::Workspace(WorkspaceId::new(id.parse().expect("an id")))
        }
        other => panic!("no request word {other}"),
    }
}

/// The fake planted as `hyprctl` in a scratch dir with `set`, and `mode` beside it.
fn fake(scratch: &Path, mode: Option<&str>, set: &[PathBuf]) -> PathBuf {
    hyprctl_bin(scratch, mode, set)
}

#[test]
fn contract_hyprland_fake_replays_every_recorded_reply_byte_for_byte() {
    let set = recorded();
    assert!(!set.is_empty(), "no recorded hyprctl reply");
    for path in &set {
        let doc = doc(path);
        let tmp = tempfile::tempdir().expect("tempdir");
        let mode = is_absent(path).then_some("absent");
        let program = fake(tmp.path(), mode, std::slice::from_ref(path));
        let request = request_of(&doc);
        assert_eq!(
            request
                .args()
                .iter()
                .map(|a| a.to_str().expect("utf-8"))
                .collect::<Vec<_>>(),
            doc["args"]
                .as_array()
                .expect("args")
                .iter()
                .map(|a| a.as_str().expect("an arg"))
                .collect::<Vec<_>>(),
            "{}: the seam's argv is the recorded argv",
            path.display()
        );
        let got = read(Some(&program), &request, &[]);
        assert_eq!(
            got.stdout,
            doc["stdout"].as_str().expect("stdout").as_bytes(),
            "{}",
            path.display()
        );
        let exit = doc["exit"].as_i64().and_then(|e| i32::try_from(e).ok());
        assert_eq!(got.exit, exit, "{}", path.display());
    }
}

#[test]
fn contract_hyprland_read_parses_every_recorded_reply_into_its_type() {
    let set: Vec<PathBuf> = recorded().into_iter().filter(|p| !is_absent(p)).collect();
    assert!(!set.is_empty(), "no recorded hyprctl reply");
    for path in &set {
        let doc = doc(path);
        let tmp = tempfile::tempdir().expect("tempdir");
        let program = fake(tmp.path(), None, std::slice::from_ref(path));
        let request = request_of(&doc);
        let reply = read(Some(&program), &request, &[]).reply;
        let parsed = match (&request, &reply) {
            (Request::Clients, Ok(Reply::Clients(windows))) => !windows.is_empty(),
            (Request::ActiveWindow, Ok(Reply::ActiveWindow(_))) => true,
            (Request::Workspaces, Ok(Reply::Workspaces(spaces))) => !spaces.is_empty(),
            (Request::Version, Ok(Reply::Version(version))) => {
                Some(*version)
                    == HyprlandVersion::from_tag(&format!(
                        "v{}",
                        doc["hyprland"].as_str().expect("version")
                    ))
            }
            (Request::FocusWindow(_) | Request::Workspace(_), Ok(Reply::Dispatched)) => true,
            _ => false,
        };
        assert!(parsed, "{}: {reply:?}", path.display());
    }
}

#[test]
fn contract_hyprland_read_of_the_recorded_no_instance_reply_is_unreachable() {
    let set: Vec<PathBuf> = recorded().into_iter().filter(|p| is_absent(p)).collect();
    assert!(!set.is_empty(), "no recorded no-instance reply");
    for path in &set {
        let tmp = tempfile::tempdir().expect("tempdir");
        let program = fake(tmp.path(), Some("absent"), std::slice::from_ref(path));
        let got = read(Some(&program), &request_of(&doc(path)), &[]);
        assert_eq!(
            got.reply,
            Err(HyprlandError::Unreachable),
            "{}",
            path.display()
        );
        assert!(got.exit.is_some_and(|e| e != 0), "{}", path.display());
    }
}

#[test]
fn contract_hyprland_oversize_and_malformed_modes_read_their_failure_with_one_codes_only_line() {
    for (mode, error, detail) in [
        ("oversize", HyprlandError::Oversize, "oversize"),
        ("malformed", HyprlandError::Malformed, "malformed"),
    ] {
        let tmp = tempfile::tempdir().expect("tempdir");
        let program = fake(tmp.path(), Some(mode), &[]);
        let (got, lines) = captured(|| read(Some(&program), &Request::Clients, &[]));
        assert_eq!(got.reply, Err(error), "{mode}");
        let rejected: Vec<&Value> = lines
            .iter()
            .filter(|l| l["event"] == "parse-rejected")
            .collect();
        let want = json!({"event": "parse-rejected", "parser": "hyprctl-json", "detail": detail,
                          "count": 1, "message": "parse-rejected"});
        assert_eq!(rejected, [&want], "{mode}");
        let subjects: Vec<&Value> = lines.iter().map(|l| &l["subject"]).collect();
        assert_eq!(
            subjects,
            [
                &json!("hyprctl-probe"),
                &json!("hyprctl-probe"),
                &Value::Null
            ],
            "{mode}"
        );
        let written = serde_json::to_string(&lines).expect("lines");
        assert!(!written.contains("viola fixture text"), "{mode}");
    }
}

#[test]
fn contract_hyprland_canary_reaches_no_line_and_no_error() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let reply = json!([{"address": "0x1a2b", "title": CANARY, "class": CANARY,
                        "workspace": {"id": 1, "name": CANARY}, "pid": 7}]);
    let planted = tmp.path().join("Clients.recorded-1.json");
    let fixture = json!({"v": 1, "hyprland": "0.56.2", "request": "clients",
                         "args": ["-j", "clients"], "exit": 0, "stdout": reply.to_string()});
    std::fs::write(&planted, fixture.to_string()).expect("planted");
    let program = fake(
        &tmp.path().join("fake"),
        None,
        std::slice::from_ref(&planted),
    );
    let (got, lines) = captured(|| read(Some(&program), &Request::Clients, &[]));
    let Ok(Reply::Clients(windows)) = &got.reply else {
        panic!("windows: {:?}", got.reply);
    };
    assert_eq!(windows.len(), 1);
    assert!(!format!("{windows:?}").contains(CANARY));
    let written = serde_json::to_string(&lines).expect("lines");
    assert!(!written.contains(CANARY));
    assert!(!written.contains("0x1a2b"));
    for error in HyprlandError::ALL {
        assert!(!error.to_string().contains(CANARY));
    }
    let malformed = fake(&tmp.path().join("malformed"), Some("malformed"), &[]);
    let (failed, _) = captured(|| read(Some(&malformed), &Request::Clients, &[]));
    let error = failed.reply.expect_err("malformed");
    assert!(!error.to_string().contains("viola fixture text"));
}
