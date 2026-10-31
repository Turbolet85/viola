//! The tool handlers against a test wrapper, with no stdio: this test process stands in for the
//! wrapper, its snapshot written through `viola_state`'s own writer and its channel a real
//! `viola_channel::Server`, so each frame tool's request and the reply it maps are read on both
//! sides. The test wrapper counts every frame it is sent. An impostor's snapshot records another
//! live process, a child of this test binary, while this process serves the endpoint.

use std::path::Path;
use std::sync::{Arc, Mutex};

use rmcp::model::{CallToolResult, JsonObject};
use serde_json::{Value, json};
use viola_channel::{Dispatch, ProtocolError, Server, Serving};
use viola_core::ViolaName;
use viola_mcp::Tools;
use viola_state::snapshot::{InstanceSnapshot, Wheel, write_snapshot};

/// Every frame the test wrapper was sent, and its canned reply per method.
#[derive(Default)]
struct TestWrapper {
    frames: Mutex<Vec<(String, Value)>>,
    replies: Mutex<Vec<(String, Result<Value, ProtocolError>)>>,
}

impl Dispatch for TestWrapper {
    fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {
        self.frames
            .lock()
            .expect("frames")
            .push((method.to_owned(), params.clone()));
        self.replies
            .lock()
            .expect("replies")
            .iter()
            .find(|(m, _)| m == method)
            .map_or(Err(ProtocolError::MethodNotFound), |(_, reply)| {
                reply.clone()
            })
    }
}

/// A home holding the live instance `builder` served by a `TestWrapper`.
struct Booted {
    _dir: tempfile::TempDir,
    home: std::path::PathBuf,
    wrapper: Arc<TestWrapper>,
    _serving: Serving,
}

fn endpoint(dir: &Path, label: &str) -> String {
    let base = format!("viola-test-mcp-{}-{label}", std::process::id());
    if cfg!(windows) {
        format!(r"\\.\pipe\{base}")
    } else {
        dir.join(format!("{base}.sock"))
            .to_string_lossy()
            .into_owned()
    }
}

fn booted(label: &str, replies: Vec<(&str, Result<Value, ProtocolError>)>) -> Booted {
    booted_recording(label, replies, std::process::id())
}

/// A snapshot naming `endpoint` and recording `pid` (and that process's own start time) as the
/// wrapper, with a fresh heartbeat.
fn record_live(instance: &Path, endpoint: String, pid: u32) {
    let snapshot = InstanceSnapshot {
        endpoint: Some(endpoint),
        pid,
        started_at: viola_state::liveness::own_start(pid).expect("own start"),
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
    write_snapshot(instance, &snapshot).expect("snapshot");
    viola_state::heartbeat::touch_heartbeat(instance).expect("heartbeat");
}

/// `booted`, its snapshot recording `pid` (and that process's own start time) as the wrapper.
fn booted_recording(
    label: &str,
    replies: Vec<(&str, Result<Value, ProtocolError>)>,
    pid: u32,
) -> Booted {
    let dir = tempfile::tempdir().expect("tempdir");
    let home = dir.path().join("home");
    let instance = home.join("instances").join("builder");
    viola_state::fs::create_private_dir(&instance).expect("instance dir");
    let endpoint = endpoint(dir.path(), label);
    let wrapper = Arc::new(TestWrapper::default());
    *wrapper.replies.lock().expect("replies") = replies
        .into_iter()
        .map(|(method, reply)| (method.to_owned(), reply))
        .collect();
    let serving = Server::bind(&endpoint)
        .expect("bound")
        .serve(wrapper.clone());
    record_live(&instance, endpoint, pid);
    Booted {
        _dir: dir,
        home,
        wrapper,
        _serving: serving,
    }
}

impl Booted {
    fn tools(&self, from: Option<&str>) -> Tools {
        let from = from.map(|name| ViolaName::try_new(name.to_owned()).expect("a name"));
        Tools::new(
            self.home.clone(),
            from,
            Arc::new(|| json!({"v": 1, "ok": {}})),
        )
    }

    fn frames(&self) -> Vec<(String, Value)> {
        self.wrapper.frames.lock().expect("frames").clone()
    }
}

/// The held child's marker, set on the child alone (`Command::env`).
const HELD: &str = "MCP_HANDLERS_TEST_HELD";

/// The child half: a no-op in an ordinary run; spawned by the impostor case, it lives until its
/// stdin closes.
#[test]
fn handlers_held_child_entry() {
    if std::env::var_os(HELD).is_none() {
        return;
    }
    let _ = std::io::Read::read_to_end(&mut std::io::stdin(), &mut Vec::new());
}

/// A live process that is not this one, killed and reaped on drop.
#[cfg(not(target_os = "macos"))]
struct Held(std::process::Child);

#[cfg(not(target_os = "macos"))]
impl Held {
    fn spawn() -> Self {
        let child = std::process::Command::new(std::env::current_exe().expect("the test binary"))
            .args(["--exact", "handlers_held_child_entry"])
            .env(HELD, "1")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("the held child");
        Self(child)
    }
}

#[cfg(not(target_os = "macos"))]
impl Drop for Held {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(not(target_os = "macos"))]
fn text(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|block| block.as_text().map(|t| t.text.clone()))
        .collect()
}

fn call(tools: &Tools, name: &str, args: Value) -> Result<CallToolResult, rmcp::model::ErrorData> {
    let args: JsonObject = args.as_object().expect("an object").clone();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(tools.call(name, args))
}

/// The params a frame carried, without the envelope every frame carries.
fn own_params(params: &Value) -> Value {
    let mut params = params.as_object().expect("params").clone();
    for key in ["v", "sender", "conn"] {
        params.remove(key);
    }
    Value::Object(params)
}

#[test]
fn handlers_send_maps_the_readback_and_carries_from() {
    let wrapper = booted(
        "send",
        vec![(
            "send",
            Ok(json!({"ok": {"submitted_at": "2026-10-17T00:00:00.000Z", "cursor": 812}})),
        )],
    );
    let result = call(
        &wrapper.tools(Some("driver")),
        "send",
        json!({"target": "builder", "text": "hello\r\n"}),
    )
    .expect("a result");
    assert_eq!(result.is_error, Some(false));
    assert_eq!(
        result.structured_content,
        Some(json!({"submitted_at": "2026-10-17T00:00:00.000Z", "cursor": 812}))
    );
    let frames = wrapper.frames();
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].0, "send");
    assert_eq!(
        own_params(&frames[0].1),
        json!({"text": "hello\r\n", "from": "driver"})
    );
}

#[test]
fn handlers_send_without_a_name_sends_no_from_and_maps_a_refusal() {
    let wrapper = booted(
        "refusal",
        vec![(
            "send",
            Ok(json!({"refusal": "budget-paused", "detail": "five-hour"})),
        )],
    );
    let result = call(
        &wrapper.tools(None),
        "send",
        json!({"target": "builder", "text": "x"}),
    )
    .expect("a result");
    assert_eq!(result.is_error, Some(true));
    assert_eq!(
        result.structured_content,
        Some(json!({"refusal": "budget-paused", "detail": "five-hour"}))
    );
    assert_eq!(own_params(&wrapper.frames()[0].1), json!({"text": "x"}));
}

/// A peer's refusal holding free text reaches neither the body nor the `refused:` text.
#[test]
fn handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail() {
    let wrapper = booted(
        "freetext",
        vec![(
            "send",
            Ok(json!({"refusal": "/home/x/y refused", "detail": "/home/x/y"})),
        )],
    );
    let result = call(
        &wrapper.tools(None),
        "send",
        json!({"target": "builder", "text": "x"}),
    )
    .expect("a result");
    assert_eq!(result.is_error, Some(true));
    assert_eq!(
        result.structured_content,
        Some(json!({"refusal": "unknown", "detail": null}))
    );
    assert_eq!(
        serde_json::to_value(&result.content).expect("json")[0]["text"],
        "refused: unknown"
    );
}

/// `wait` sends the clamped bound, the cap when none was given, and `after` as given.
#[test]
fn handlers_wait_sends_the_clamped_bound_and_maps_a_timeout() {
    let wrapper = booted(
        "wait",
        vec![("wait", Ok(json!({"ok": {"timed_out": true}})))],
    );
    let tools = wrapper.tools(None);
    for (args, sent) in [
        (json!({"target": "builder"}), json!({"timeout_ms": 50_000})),
        (
            json!({"target": "builder", "after": 7, "timeout_ms": 900}),
            json!({"after": 7, "timeout_ms": 900}),
        ),
        (
            json!({"target": "builder", "timeout_ms": 900_000}),
            json!({"timeout_ms": 50_000}),
        ),
    ] {
        let result = call(&tools, "wait", args).expect("a result");
        assert_eq!(result.structured_content, Some(json!({"timed_out": true})));
        let frames = wrapper.frames();
        assert_eq!(own_params(&frames.last().expect("a frame").1), sent);
    }
}

#[test]
fn handlers_last_and_answer_map_their_replies() {
    let wrapper = booted(
        "last-answer",
        vec![
            (
                "last",
                Ok(json!({"ok": {"last_assistant_message": "done", "ts": "t"}})),
            ),
            (
                "answer",
                Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"})),
            ),
        ],
    );
    let tools = wrapper.tools(Some("driver"));
    let last = call(&tools, "last", json!({"target": "builder"})).expect("last");
    assert_eq!(
        last.structured_content,
        Some(json!({"last_assistant_message": "done", "ts": "t"}))
    );
    let answer = call(
        &tools,
        "answer",
        json!({"target": "builder", "dialog_id": 4, "response": {"behavior": "deny", "message": "no"}}),
    )
    .expect("answer");
    assert_eq!(
        answer.structured_content,
        Some(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
    );
    let frames = wrapper.frames();
    assert_eq!(own_params(&frames[0].1), json!({"from": "driver"}));
    assert_eq!(frames[1].0, "answer");
    assert_eq!(own_params(&frames[1].1)["dialog_id"], 4);
    assert_eq!(own_params(&frames[1].1)["from"], "driver");
}

#[test]
fn handlers_map_a_wrapper_fault_and_a_wait_refusal_as_the_two_faults() {
    let wrapper = booted(
        "faults",
        vec![
            ("send", Err(ProtocolError::Internal)),
            ("wait", Ok(json!({"refusal": "unknown", "detail": null}))),
        ],
    );
    let tools = wrapper.tools(None);
    let fault = call(&tools, "send", json!({"target": "builder", "text": "x"})).expect("send");
    assert_eq!(
        fault.structured_content,
        Some(
            json!({"error": "wrapper-fault", "detail": {"code": -32603, "message": "internal error", "data": null}})
        )
    );
    let wait = call(&tools, "wait", json!({"target": "builder"})).expect_err("an MCP fault");
    assert_eq!(
        serde_json::to_value(&wait).expect("json"),
        json!({"code": -32603, "message": "internal error", "data": null})
    );
}

/// A call its schema refuses writes no frame, whichever field failed.
#[test]
fn handlers_schema_violations_write_no_frame() {
    let wrapper = booted("schema", Vec::new());
    let tools = wrapper.tools(None);
    for (name, args) in [
        ("send", json!({"target": "Builder!", "text": "x"})),
        ("send", json!({"target": "builder", "text": "\u{1b}[2J"})),
        ("send", json!({"target": "builder", "text": "\r\n"})),
        ("wait", json!({"target": "builder", "after": "0"})),
        (
            "answer",
            json!({"target": "builder", "dialog_id": "1", "response": {"behavior": "allow"}}),
        ),
        (
            "answer",
            json!({"target": "builder", "dialog_id": 1, "response": {"behavior": "sure"}}),
        ),
    ] {
        let result = call(&tools, name, args).expect("a result");
        assert_eq!(result.is_error, Some(true), "{name}");
    }
    let unknown = call(&tools, "release", json!({"target": "builder"})).expect_err("no tool");
    assert_eq!(unknown.code.0, -32602);
    assert!(wrapper.frames().is_empty(), "{:?}", wrapper.frames());
}

/// The snapshot records a live process that is not the one serving the endpoint: every frame tool
/// reads `instance-unreachable` and the server is sent no frame. The cases above, whose snapshot
/// records the serving process, are the control.
#[cfg(not(target_os = "macos"))]
#[test]
fn handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper() {
    let held = Held::spawn();
    let wrapper = booted_recording(
        "impostor",
        vec![
            (
                "send",
                Ok(json!({"ok": {"submitted_at": "t", "cursor": 1}})),
            ),
            ("wait", Ok(json!({"ok": {"timed_out": true}}))),
            (
                "last",
                Ok(json!({"ok": {"last_assistant_message": null, "ts": null}})),
            ),
            ("answer", Ok(json!({"ok": {}}))),
        ],
        held.0.id(),
    );
    let tools = wrapper.tools(Some("driver"));
    for (name, args) in [
        ("send", json!({"target": "builder", "text": "x"})),
        ("wait", json!({"target": "builder", "timeout_ms": 900})),
        ("last", json!({"target": "builder"})),
        (
            "answer",
            json!({"target": "builder", "dialog_id": 1, "response": {"behavior": "allow"}}),
        ),
    ] {
        let result = call(&tools, name, args).expect("a result");
        assert_eq!(result.is_error, Some(true), "{name}");
        assert_eq!(
            result.structured_content,
            Some(json!({"error": "instance-unreachable", "detail": "server-verify-failed"})),
            "{name}"
        );
        assert_eq!(text(&result), "refused: instance-unreachable", "{name}");
    }
    assert!(wrapper.frames().is_empty(), "{:?}", wrapper.frames());
    drop(held);
}

/// Grants another user write on the instance directory: Unix mode 0770, Windows a Users write
/// grant through `icacls`, its exit asserted. `grant == false` takes it back.
fn widened(dir: &Path, grant: bool) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = if grant { 0o770 } else { 0o700 };
        std::fs::set_permissions(dir, std::fs::Permissions::from_mode(mode)).expect("chmod");
    }
    #[cfg(windows)]
    {
        let root = std::env::var_os("SystemRoot").expect("SystemRoot");
        let args: &[&str] = if grant {
            &["/grant", "*S-1-5-32-545:(OI)(CI)(W)"]
        } else {
            &["/remove:g", "*S-1-5-32-545"]
        };
        let status = std::process::Command::new(
            std::path::PathBuf::from(root)
                .join("System32")
                .join("icacls.exe"),
        )
        .arg(dir)
        .args(args)
        .stdout(std::process::Stdio::null())
        .status()
        .expect("icacls");
        assert!(status.success(), "icacls {args:?}");
    }
}

/// An instance directory another user may write, its recorded wrapper serving: every frame tool
/// reads `instance-unreachable` with `detail` `strict-modes-failed` before any snapshot read or
/// connection, and the
/// wrapper is sent no frame; the widening is left as found. Narrowed again, the same wrapper is
/// reached: the control.
#[test]
fn handlers_frame_tools_refuse_a_widened_instance_before_any_frame() {
    let wrapper = booted(
        "widened",
        vec![(
            "last",
            Ok(json!({"ok": {"last_assistant_message": null, "ts": null}})),
        )],
    );
    let dir = wrapper.home.join("instances").join("builder");
    widened(&dir, true);
    let tools = wrapper.tools(Some("driver"));
    for (name, args) in [
        ("send", json!({"target": "builder", "text": "x"})),
        ("wait", json!({"target": "builder", "timeout_ms": 900})),
        ("last", json!({"target": "builder"})),
        (
            "answer",
            json!({"target": "builder", "dialog_id": 1, "response": {"behavior": "allow"}}),
        ),
    ] {
        let result = call(&tools, name, args).expect("a result");
        assert_eq!(result.is_error, Some(true), "{name}");
        assert_eq!(
            result.structured_content,
            Some(json!({"error": "instance-unreachable", "detail": "strict-modes-failed"})),
            "{name}"
        );
    }
    assert!(wrapper.frames().is_empty(), "{:?}", wrapper.frames());
    assert!(
        viola_state::strict::check_instance(&wrapper.home, &dir).is_err(),
        "the widening was undone"
    );
    widened(&dir, false);
    let control = call(&tools, "last", json!({"target": "builder"})).expect("a result");
    assert_eq!(control.is_error, Some(false), "{control:?}");
    assert_eq!(wrapper.frames().len(), 1);
}

/// Each frame tool over the causes with no live endpoint and the one a failed connection reads,
/// each body the CLI's document less `v`: a name with no instance directory is `unwrapped-name`,
/// one whose directory stands with no wrapper `instance-dead`, and a recorded endpoint no one
/// serves `instance-dead`; no frame and the text `refused: instance-unreachable`.
#[test]
fn handlers_frame_tools_name_the_cause_of_an_unreachable_instance() {
    let wrapper = booted("causes", Vec::new());
    let stopped = wrapper.home.join("instances").join("stopped");
    viola_state::fs::create_private_dir(&stopped).expect("a stopped instance");
    let tools = wrapper.tools(Some("driver"));
    let target_args = |target: &str, name: &str| match name {
        "send" => json!({"target": target, "text": "x"}),
        "wait" => json!({"target": target, "timeout_ms": 900}),
        "last" => json!({"target": target}),
        _ => json!({"target": target, "dialog_id": 1, "response": {"behavior": "allow"}}),
    };
    let assert_cause = |tools: &Tools, target: &str, cause: &str| {
        for name in ["send", "wait", "last", "answer"] {
            let result = call(tools, name, target_args(target, name)).expect("a result");
            assert_eq!(result.is_error, Some(true), "{name} {target}");
            assert_eq!(
                result.structured_content,
                Some(json!({"error": "instance-unreachable", "detail": cause})),
                "{name} {target}"
            );
            assert_eq!(text(&result), "refused: instance-unreachable", "{name}");
        }
    };
    assert_cause(&tools, "ghost", "unwrapped-name");
    assert_cause(&tools, "stopped", "instance-dead");
    let unserved = wrapper.home.join("instances").join("unserved");
    viola_state::fs::create_private_dir(&unserved).expect("the instance");
    record_live(
        &unserved,
        endpoint(wrapper._dir.path(), "unserved"),
        std::process::id(),
    );
    assert_cause(&tools, "unserved", "instance-dead");
    assert!(wrapper.frames().is_empty(), "{:?}", wrapper.frames());
}
