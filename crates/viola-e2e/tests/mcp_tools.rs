//! The stdio MCP server end to end (test-plan §6 Paths 2–7, Cross-surface parity; architecture
//! §Established Decisions [MCP]): an rmcp client spawns `<bin dir>/viola mcp --home <home>` with no
//! `VIOLA_*` variable over a booted harness session on the fake agent, and each tool's result is
//! read beside its CLI verb's `--json` document over the same instance. Every step is keyed on a
//! cursor, a tool's own return or an event line, never a sleep.

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use rmcp::model::{CallToolRequestParams, CallToolResult};
use rmcp::service::{RoleClient, RunningService, ServiceExt as _};
use serde_json::{Value, json};
use viola_e2e::harness::boot::session_path;
use viola_e2e::harness::session::{HarnessSession, SessionOptions, harness_session};

const CANARY: &str = "canary-chain-value-5c1e";
const FIVE_HOUR_RESET: &str = "2099-01-01T00:00:00Z";
const SEVEN_DAY_RESET: &str = "2099-01-06T00:00:00Z";
const UNWRAPPED_ID: &str = "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f";
const UNWRAPPED_ROW: &str = r#"[{"cwd":"~/b","kind":"interactive","name":"b title","sessionId":"0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f","startedAt":1760000000001,"status":"idle"}]"#;

/// `builder` on a fake-agent script, its gated steps released through a control file the test owns:
/// the `builder:<args>` spec and the control file's path. `script` is a committed one under the
/// workspace, or with `steps` the test's own, written beside the control file.
fn scripted_builder(script: &str, steps: Option<&Value>) -> (tempfile::TempDir, PathBuf, String) {
    let ws = viola_e2e::harness::Workspace::from_build();
    let control_dir = tempfile::tempdir().expect("control dir");
    let control = control_dir.path().join("builder.control");
    std::fs::write(&control, b"").expect("control");
    let script = match steps {
        Some(steps) => {
            let path = control_dir.path().join(script);
            std::fs::write(&path, json!({"v": 1, "steps": steps}).to_string()).expect("script");
            path
        }
        None => ws.root.join(script),
    };
    let spec = format!(
        "builder:--script {} --control {}",
        script.display(),
        control.display()
    );
    (control_dir, control, spec)
}

/// Releases the next `n` gated steps of a scripted fake agent.
fn release(control: &Path, n: usize) {
    std::fs::OpenOptions::new()
        .append(true)
        .open(control)
        .and_then(|mut f| f.write_all("go\n".repeat(n).as_bytes()))
        .expect("release");
}

/// One rmcp client over one `viola mcp` process, on its own runtime. The test owns the child: the
/// client talks over its stdin and stdout, and closing the client closes its stdin, after which the
/// server's own exit is awaited, never a kill (a killed instrumented server leaves a truncated
/// coverage profile, which `llvm-profdata` refuses: rmcp's child transport kills 3 s after a close).
struct Mcp {
    runtime: tokio::runtime::Runtime,
    client: Option<RunningService<RoleClient, ()>>,
    child: tokio::process::Child,
}

fn viola_exe(s: &HarnessSession) -> PathBuf {
    s.bin_dir()
        .join(format!("viola{}", std::env::consts::EXE_SUFFIX))
}

fn session_bin(s: &HarnessSession) -> PathBuf {
    s.workspace().session_dir(s.session()).join("bin")
}

impl Mcp {
    /// `viola mcp --home <home>` on the session's `PATH` (the fake agent answers `claude agents`),
    /// with this process's `VIOLA_*` names removed: the server has no name of its own.
    fn spawn(s: &HarnessSession) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let home = s.home().to_path_buf();
        let path = session_path(&session_bin(s));
        let (child, client) = runtime.block_on(async {
            let mut child = tokio::process::Command::new(viola_exe(s))
                .arg("--home")
                .arg(&home)
                .arg("mcp")
                .env("PATH", &path)
                .env_remove("VIOLA_NAME")
                .env_remove("VIOLA_DIR")
                .env_remove("VIOLA_BIN")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("viola mcp");
            let stdout = child.stdout.take().expect("stdout");
            let stdin = child.stdin.take().expect("stdin");
            let client = ().serve((stdout, stdin)).await.expect("initialized");
            (child, client)
        });
        Self {
            runtime,
            client: Some(client),
            child,
        }
    }

    fn client(&self) -> &RunningService<RoleClient, ()> {
        self.client.as_ref().expect("open")
    }

    fn tools(&self) -> rmcp::model::ListToolsResult {
        self.runtime
            .block_on(self.client().list_tools(None))
            .expect("tools/list")
    }

    fn call(&self, name: &'static str, args: Value) -> CallToolResult {
        let args = args.as_object().expect("an object").clone();
        self.runtime
            .block_on(
                self.client()
                    .call_tool(CallToolRequestParams::new(name).with_arguments(args)),
            )
            .unwrap_or_else(|e| panic!("{name}: {e}"))
    }
}

impl Drop for Mcp {
    fn drop(&mut self) {
        if let Some(client) = self.client.take() {
            let _ = self.runtime.block_on(client.cancel());
        }
        let _ = self.runtime.block_on(self.child.wait());
    }
}

struct Ran {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

/// `<bin dir>/viola --home <home> <args…>` on the session's `PATH`, with this process's `VIOLA_*`
/// names removed and `env` set, `stdin` written and closed.
fn viola(s: &HarnessSession, args: &[&str], stdin: Option<&[u8]>, env: &[(&str, OsString)]) -> Ran {
    let mut command = Command::new(viola_exe(s));
    command
        .arg("--home")
        .arg(s.home())
        .args(args)
        .env("PATH", session_path(&session_bin(s)))
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in env {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("viola");
    if let Some(bytes) = stdin {
        let mut pipe = child.stdin.take().expect("stdin");
        match pipe.write_all(bytes) {
            Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => panic!("stdin: {e}"),
            _ => {}
        }
    }
    let out = child.wait_with_output().expect("output");
    Ran {
        code: out.status.code(),
        stdout: String::from_utf8(out.stdout).expect("utf-8 stdout"),
        stderr: String::from_utf8(out.stderr).expect("utf-8 stderr"),
    }
}

/// A CLI verb's one `--json` document.
fn cli_json(s: &HarnessSession, args: &[&str], stdin: Option<&[u8]>) -> Value {
    let ran = viola(s, args, stdin, &[]);
    assert!(
        ran.code == Some(0) || ran.code == Some(11) || ran.code == Some(13),
        "{args:?}: exit {:?}, stderr {}",
        ran.code,
        ran.stderr
    );
    serde_json::from_str(&ran.stdout).expect("one JSON document")
}

/// A CLI verb's `--json` document less `v`, whatever its exit: the parity oracle of a tool result.
fn less_v(stdout: &str) -> Value {
    let mut doc: Value = serde_json::from_str(stdout).expect("one JSON document");
    doc.as_object_mut().expect("an object").remove("v");
    doc
}

fn body(result: &CallToolResult) -> Value {
    result
        .structured_content
        .clone()
        .expect("structured content")
}

fn text(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|block| block.as_text().map(|t| t.text.clone()))
        .collect()
}

fn keys(value: &Value) -> Vec<String> {
    let mut keys: Vec<String> = value
        .as_object()
        .expect("an object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    keys
}

/// A refusal result: `isError`, the body and the `refused: <reason>` text, and no `release`
/// anywhere in either.
fn assert_refused(result: &CallToolResult, reason: &str, detail: Option<&str>) {
    assert_eq!(result.is_error, Some(true), "{result:?}");
    assert_eq!(body(result), json!({"refusal": reason, "detail": detail}));
    assert_eq!(text(result), format!("refused: {reason}"));
    let shown = serde_json::to_string(result).expect("json");
    assert!(!shown.contains("release"), "{shown}");
}

/// No `isError` body or text holds the home's absolute path or an error chain.
fn assert_sanitized(results: &[&CallToolResult], home: &Path) {
    let home = home.to_string_lossy().replace('\\', "\\\\");
    for result in results {
        assert_eq!(result.is_error, Some(true));
        let shown = serde_json::to_string(result).expect("json");
        assert!(!shown.contains(home.as_str()), "{shown}");
        assert!(!shown.contains("Caused by"), "{shown}");
    }
}

fn events(instance_dir: &Path) -> Vec<Value> {
    let text = std::fs::read_to_string(instance_dir.join("events.ndjson")).expect("events");
    text.split_inclusive('\n')
        .filter(|l| l.ends_with('\n'))
        .map(|l| serde_json::from_str(l).expect("an event line"))
        .collect()
}

/// Until a line of `kind` is logged, bounded at 7 s: a file-state probe, never a sleep.
fn wait_for_kind(instance_dir: &Path, kind: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(7);
    while count(instance_dir, kind) == 0 {
        assert!(
            std::time::Instant::now() < deadline,
            "no {kind} line within 7 s"
        );
        std::thread::yield_now();
    }
}

fn count(instance_dir: &Path, kind: &str) -> usize {
    events(instance_dir)
        .iter()
        .filter(|l| l["kind"] == kind)
        .count()
}

/// The tool list, `list` beside CLI `list --json`, and the four schema-violating inputs.
#[test]
fn mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli() {
    let s = harness_session("mcp_tools", "list", SessionOptions::default());
    let mcp = Mcp::spawn(&s);

    let listed_tools = mcp.tools();
    // A `2026-07-28` client drops every tool of a list result without these two (claude 2.1.287,
    // evidence/live-mcp.md).
    assert_eq!(listed_tools.ttl_ms, Some(0));
    assert_eq!(
        listed_tools.cache_scope,
        Some(rmcp::model::CacheScope::Private)
    );
    let tools = listed_tools.tools;
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert_eq!(names, ["send", "wait", "last", "answer", "list"]);
    let shown = serde_json::to_string(&tools).expect("json");
    for word in ["release", "pause", "link", "unlink"] {
        assert!(!shown.contains(word), "a tool names {word}");
    }

    // One session viola does not wrap, planted beside the fake agent its `agents --json` reads.
    std::fs::write(session_bin(&s).join("agents-rows.json"), UNWRAPPED_ROW).expect("agents rows");
    let listed = mcp.call("list", json!({}));
    assert_eq!(listed.is_error, Some(false));
    let cli = cli_json(&s, &["list", "--json"], None);
    assert_eq!(body(&listed), cli["ok"]);
    let items = cli["ok"]["items"].as_array().expect("items");
    let unwrapped: Vec<&Value> = items.iter().filter(|i| i["wrapped"] == false).collect();
    assert_eq!(unwrapped.len(), 1, "{cli}");
    assert_eq!(unwrapped[0]["name"], UNWRAPPED_ID);

    let builder = s.instance_dir("builder");
    let issued = count(&builder, "send-issued");
    let violations = [
        mcp.call("send", json!({"target": "Builder!", "text": CANARY})),
        mcp.call(
            "answer",
            json!({"target": "builder", "dialog_id": "1", "response": {"behavior": "allow"}}),
        ),
        mcp.call(
            "answer",
            json!({"target": "builder", "dialog_id": 1, "response": {"behavior": "sometimes"}}),
        ),
        mcp.call("send", json!({"target": "builder", "text": "x\u{1b}[2J"})),
        mcp.call("send", json!({"target": "builder", "text": "\n"})),
    ];
    let bodies: Vec<Value> = violations.iter().map(body).collect();
    assert_eq!(
        bodies,
        [
            json!({"error": "invalid-params", "detail": "target"}),
            json!({"error": "invalid-params", "detail": "dialog_id"}),
            json!({"error": "invalid-params", "detail": "response"}),
            json!({"refusal": "not-delivered", "detail": "control-character"}),
            json!({"refusal": "not-delivered", "detail": "empty-text"}),
        ]
    );
    assert_eq!(text(&violations[0]), "refused: invalid-params");
    // The two text refusals are the CLI's own documents.
    for (text_in, at) in [("x\u{1b}[2J", 3), ("\n", 4)] {
        let cli = viola(
            &s,
            &["send", "builder", "--json"],
            Some(text_in.as_bytes()),
            &[],
        );
        assert_eq!(cli.code, Some(13), "stderr: {}", cli.stderr);
        assert_eq!(body(&violations[at]), less_v(&cli.stdout));
    }
    assert_eq!(
        count(&builder, "send-issued"),
        issued,
        "a refused input sent nothing"
    );
    assert_sanitized(&violations.iter().collect::<Vec<_>>(), s.home());

    // The plugin `run` wrote registers the server by the pinned binary's absolute path.
    let snapshot: Value =
        serde_json::from_slice(&std::fs::read(builder.join("snapshot.json")).expect("snapshot"))
            .expect("json");
    let pinned = snapshot["data"]["pinned_bin"].as_str().expect("pinned_bin");
    let plugins: Vec<PathBuf> = std::fs::read_dir(s.home().join("plugin"))
        .expect("plugin/")
        .map(|entry| entry.expect("entry").path().join(".mcp.json"))
        .collect();
    assert_eq!(plugins.len(), 1, "{plugins:?}");
    let registered: Value =
        serde_json::from_slice(&std::fs::read(&plugins[0]).expect(".mcp.json")).expect("json");
    assert_eq!(
        registered,
        json!({"mcpServers": {"viola": {"command": pinned, "args": ["mcp"]}}})
    );
    assert!(Path::new(pinned).is_absolute(), "{pinned}");

    drop(mcp);
    // The server's own lines: its start, as process `mcp`, with no instance of its own.
    let lines = diagnostics(s.home(), "mcp.ndjson");
    let started = lines
        .iter()
        .find(|l| l["event"] == "process-start" && l["subject"] == "self")
        .expect("the server's process-start");
    assert_eq!(started["process"], "mcp");
    assert!(started.get("instance").is_none(), "{started}");
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
}

fn diagnostics(home: &Path, file: &str) -> Vec<Value> {
    let text = std::fs::read_to_string(home.join("diagnostics").join(file)).expect("role file");
    text.split_inclusive('\n')
        .filter(|l| l.ends_with('\n'))
        .map(|l| serde_json::from_str(l).expect("a diagnostics line"))
        .collect()
}

/// A home another user could write refuses the server before anything under it is opened: exit 1,
/// nothing on stdout or stderr, no role file.
#[cfg(unix)]
#[test]
fn mcp_tools_a_widened_home_exits_1_and_prints_nothing() {
    use std::os::unix::fs::PermissionsExt as _;
    let ws = viola_e2e::harness::Workspace::from_build();
    ws.ensure_e2e_home().expect("e2e home base");
    let base = tempfile::tempdir_in(ws.root.join("target/e2e-home")).expect("tempdir");
    let home = base.path().join("vhome");
    std::fs::create_dir(&home).expect("home");
    std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o770)).expect("chmod");
    let bin_dir = viola_e2e::harness::bin_dir_from_exe(&std::env::current_exe().expect("exe"));
    let out = Command::new(bin_dir.join("viola"))
        .arg("--home")
        .arg(&home)
        .arg("mcp")
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::null())
        .output()
        .expect("viola mcp");
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert!(!home.join("diagnostics").exists(), "a role file opened");
    let mode = std::fs::metadata(&home).expect("home").permissions().mode();
    assert_eq!(mode & 0o777, 0o770, "the home's mode left as found");
}

/// Every file under `dir`, recursively.
fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).expect("read_dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else {
            files.push(path);
        }
    }
    files
}

/// A fresh home under the e2e base, not yet made: the holder, and the home.
fn fresh_home() -> (tempfile::TempDir, PathBuf) {
    let ws = viola_e2e::harness::Workspace::from_build();
    ws.ensure_e2e_home().expect("e2e home base");
    let base = tempfile::tempdir_in(ws.root.join("target/e2e-home")).expect("tempdir");
    let home = base.path().join("vhome");
    (base, home)
}

/// `viola --home <home> mcp` the test owns, all three streams piped and no `VIOLA_*` name.
fn own_mcp(home: &Path) -> std::process::Child {
    let bin_dir = viola_e2e::harness::bin_dir_from_exe(&std::env::current_exe().expect("exe"));
    Command::new(bin_dir.join(format!("viola{}", std::env::consts::EXE_SUFFIX)))
        .arg("--home")
        .arg(home)
        .arg("mcp")
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola mcp")
}

/// The child's exit within 8 s; past it the child is killed and reaped, and the bound reads `None`.
fn exit_within_bound(child: &mut std::process::Child) -> Option<std::process::ExitStatus> {
    use std::time::{Duration, Instant};

    let until = Instant::now() + Duration::from_secs(8);
    loop {
        if let Some(status) = child.try_wait().expect("try_wait") {
            return Some(status);
        }
        if Instant::now() >= until {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// The one codes-only `parse-rejected{mcp-stdin, oversize}` line of `mcp.ndjson`, valid against the
/// diag-line schema, and no byte of the over-long line in any file under the home.
fn assert_one_oversize_line(home: &Path) {
    let ws = viola_e2e::harness::Workspace::from_build();
    let lines = diagnostics(home, "mcp.ndjson");
    let rejected: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "parse-rejected")
        .collect();
    assert_eq!(rejected.len(), 1, "one parse-rejected line");
    assert_eq!(rejected[0]["parser"], "mcp-stdin");
    assert_eq!(rejected[0]["detail"], "oversize");
    assert_eq!(rejected[0]["count"], 1);
    let schema: Value = serde_json::from_slice(
        &std::fs::read(ws.root.join("schemas/diag-line.v1.json")).expect("schema"),
    )
    .expect("schema json");
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    let errors: Vec<String> = validator
        .iter_errors(rejected[0])
        .map(|e| format!("{} at {}", e.schema_path(), e.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{errors:?}");

    for file in files_under(home) {
        let bytes = std::fs::read(&file).expect("home file");
        assert!(
            !bytes.windows(CANARY.len()).any(|w| w == CANARY.as_bytes()),
            "the line's canary in {}",
            file.strip_prefix(home).expect("under home").display()
        );
    }
}

/// One stdin line past `MAX_FRAME` as the first and only line, stdin left open: the session never
/// initializes, so `viola mcp` exits 1 with both streams empty and the one line.
#[test]
fn mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line() {
    use std::io::Read as _;

    let (_base, home) = fresh_home();
    let mut child = own_mcp(&home);
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");
    let mut stderr = child.stderr.take().expect("stderr");
    let out = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stdout.read_to_end(&mut bytes);
        bytes
    });
    let err = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stderr.read_to_end(&mut bytes);
        bytes
    });
    let max_frame = 16 * 1024 * 1024;
    let writer = std::thread::spawn(move || {
        let mut over = CANARY.as_bytes().to_vec();
        over.resize(max_frame + 1, b'a');
        // Once the server stops taking stdin it may exit mid-write: a broken pipe (EPIPE, ENOTCONN,
        // or ERROR_NO_DATA on Windows) is then the expected write error, so only the exit counts.
        let _ = stdin.write_all(&over).and_then(|()| stdin.flush());
        stdin
    });
    let status = exit_within_bound(&mut child);
    drop(writer.join().expect("the writer"));
    let stdout = out.join().expect("stdout reader");
    let stderr = err.join().expect("stderr reader");
    assert_eq!(
        status.and_then(|s| s.code()),
        Some(1),
        "exit within the bound"
    );
    assert!(stdout.is_empty(), "stdout: {} bytes", stdout.len());
    assert!(stderr.is_empty(), "stderr: {} bytes", stderr.len());
    assert_one_oversize_line(&home);
}

/// A `tools/call` whose `params` are wrong-shaped, the canary and a path inside them: stdout holds
/// neither, the session stays up, a `tools/list` after it is answered, and a closed stdin exits 0.
#[test]
fn mcp_tools_a_wrong_shaped_call_answers_no_caller_text() {
    use std::io::{BufRead as _, BufReader};
    use std::time::Duration;

    let (_base, home) = fresh_home();
    let mut child = own_mcp(&home);
    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    drop(child.stderr.take());
    let (frames, read) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if frames.send(line).is_err() {
                break;
            }
        }
    });
    let path = "/home/caller/secret";
    let opening = concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"#,
        r#""2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#,
        "\n",
    );
    stdin.write_all(opening.as_bytes()).expect("initialize");
    let bound = Duration::from_secs(8);
    let initialized = read.recv_timeout(bound).expect("the initialize response");
    let call = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {"name": 7, "arguments": format!("{CANARY} {path}"), "note": path},
    });
    let list = json!({"jsonrpc": "2.0", "id": 3, "method": "tools/list", "params": {}});
    let rest = format!(
        "{}\n{call}\n{list}\n",
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#
    );
    stdin.write_all(rest.as_bytes()).expect("the frames");
    stdin.flush().expect("flush");
    let mut lines = vec![initialized];
    let listed = loop {
        match read.recv_timeout(bound) {
            Ok(line) => {
                let frame: Value = serde_json::from_str(&line).expect("an MCP frame");
                lines.push(line);
                if frame["id"] == 3 {
                    break Some(frame);
                }
            }
            Err(_) => break None,
        }
    };
    drop(stdin);
    let status = exit_within_bound(&mut child);
    drop(read);
    reader.join().expect("the reader");
    for line in &lines {
        assert!(!line.contains(CANARY), "the call's canary on stdout");
        assert!(!line.contains(path), "the call's path on stdout");
    }
    assert!(
        listed.is_some_and(|frame| frame["result"]["tools"].is_array()),
        "tools/list answered after the call"
    );
    assert_eq!(
        status.and_then(|s| s.code()),
        Some(0),
        "a closed stdin exits 0"
    );
}

/// No key or string of `value` holds the home, an absolute path, an error chain or the upstream
/// text; a hit names `label` and never the bytes.
fn assert_no_path_or_upstream_text(label: &str, value: &Value, home: &Path) {
    fn strings<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
        match value {
            Value::String(text) => out.push(text),
            Value::Array(items) => items.iter().for_each(|item| strings(item, out)),
            Value::Object(members) => members.iter().for_each(|(key, item)| {
                out.push(key);
                strings(item, out);
            }),
            _ => {}
        }
    }
    let spelled = home.display().to_string();
    let mut all = Vec::new();
    strings(value, &mut all);
    for text in all {
        let drive = text.as_bytes().windows(3).any(|window| {
            window[0].is_ascii_alphabetic()
                && window[1] == b':'
                && matches!(window[2], b'\\' | b'/')
        });
        let hit = drive
            || text.contains(spelled.as_str())
            || text.contains("/home/")
            || text.contains("/Users/")
            || text.contains("Caused by")
            || text.contains(CANARY);
        assert!(!hit, "{label}: an error body holds a path or upstream text");
    }
}

/// Each `isError` result's body and text, driven with the session home's path and an upstream text
/// in play, and an unknown tool named by the path: the JSON-RPC `-32602`.
#[test]
fn mcp_tools_error_results_hold_no_path_or_upstream_text() {
    let s = harness_session(
        "mcp_tools",
        "errors",
        SessionOptions {
            unstamped: true,
            ..SessionOptions::default()
        },
    );
    let home = s.home().to_path_buf();
    let path = home.display().to_string();
    let upstream = format!("{path} {CANARY}");
    let mcp = Mcp::spawn(&s);
    let rows: [(&str, &'static str, Value, Value); 5] = [
        (
            "send with target the path",
            "send",
            json!({"target": path, "text": upstream}),
            json!({"error": "invalid-params", "detail": "target"}),
        ),
        (
            "wait with a string after",
            "wait",
            json!({"target": "builder", "after": upstream}),
            json!({"error": "invalid-params", "detail": "after"}),
        ),
        (
            "answer with a response holding a path",
            "answer",
            json!({"target": "builder", "dialog_id": 1, "response": {"behavior": upstream}}),
            json!({"error": "invalid-params", "detail": "response"}),
        ),
        (
            "send of a control character",
            "send",
            json!({"target": "builder", "text": format!("{upstream}\u{1b}[2J")}),
            json!({"refusal": "not-delivered", "detail": "control-character"}),
        ),
        (
            "a dead instance",
            "last",
            json!({"target": "ghost"}),
            json!({"error": "instance-unreachable", "detail": null}),
        ),
    ];
    for (label, tool, args, want) in rows {
        let result = mcp.call(tool, args);
        let shown = serde_json::to_value(&result).expect("json");
        assert_no_path_or_upstream_text(label, &shown, &home);
        assert_eq!(result.is_error, Some(true), "{label}");
        assert_eq!(body(&result), want, "{label}");
        let code = want
            .get("refusal")
            .or_else(|| want.get("error"))
            .and_then(Value::as_str)
            .expect("a code");
        assert_eq!(text(&result), format!("refused: {code}"), "{label}");
    }
    let args = json!({"note": upstream});
    let unknown = mcp.runtime.block_on(
        mcp.client().call_tool(
            CallToolRequestParams::new(path.clone())
                .with_arguments(args.as_object().expect("an object").clone()),
        ),
    );
    let Err(rmcp::ServiceError::McpError(fault)) = unknown else {
        panic!("an unknown tool is a JSON-RPC error");
    };
    let shown = serde_json::to_value(&fault).expect("json");
    assert_no_path_or_upstream_text("an unknown tool", &shown, &home);
    assert_eq!(fault.code.0, -32602);
    assert_eq!(fault.message, "unknown tool");
    // rmcp's client reads the wire's `"data":null` as no `data`; any held value would read as one.
    assert_eq!(fault.data, None);

    drop(mcp);
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
}

/// An initialized session, then one stdin line past `MAX_FRAME` (16 MiB) with stdin left open: the
/// server answers the `initialize` and nothing more, ends the session and exits 0, its stderr
/// empty, and `mcp.ndjson` holds one codes-only `parse-rejected{mcp-stdin, oversize}` line that
/// validates against the diag-line schema and holds no byte of the line.
#[test]
fn mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line() {
    use std::io::Read as _;

    let (_base, home) = fresh_home();
    let mut child = own_mcp(&home);
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");
    let mut stderr = child.stderr.take().expect("stderr");
    let out = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stdout.read_to_end(&mut bytes);
        bytes
    });
    let err = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stderr.read_to_end(&mut bytes);
        bytes
    });
    let max_frame = 16 * 1024 * 1024;
    let writer = std::thread::spawn(move || {
        let opening = concat!(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"#,
            r#""2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#,
            "\n",
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            "\n",
        );
        let mut over = CANARY.as_bytes().to_vec();
        over.resize(max_frame + 1, b'a');
        // Once the server stops taking stdin it may exit mid-write: a broken pipe (EPIPE, or
        // ERROR_NO_DATA on Windows) is then the expected write error, so only the exit counts.
        let _ = stdin
            .write_all(opening.as_bytes())
            .and_then(|()| stdin.write_all(&over))
            .and_then(|()| stdin.flush());
        stdin
    });
    let status = exit_within_bound(&mut child);
    let stdin = writer.join().expect("the writer");
    drop(stdin);
    let stdout = out.join().expect("stdout reader");
    let stderr = err.join().expect("stderr reader");
    assert_eq!(
        status.and_then(|s| s.code()),
        Some(0),
        "exit within the bound"
    );
    assert!(stderr.is_empty(), "stderr: {} bytes", stderr.len());

    let frames: Vec<Value> = String::from_utf8(stdout)
        .expect("utf-8 stdout")
        .lines()
        .map(|l| serde_json::from_str(l).expect("an MCP frame"))
        .collect();
    assert_eq!(frames.len(), 1, "only the initialize response");
    assert_eq!(frames[0]["id"], 1);
    assert!(frames[0]["result"].is_object(), "{}", frames[0]);
    assert_one_oversize_line(&home);
}

/// Path 2 and Path 3 through the tools: `send` reads back, `wait` wakes on the turn's end at the
/// send's cursor, `last` and a timed-out `wait` match the CLI, and a stopped instance is
/// unreachable.
#[test]
fn mcp_tools_path2_path3_send_wait_last_match_the_cli() {
    // A turn of one tool's activity, then its end, both recorded 2.1.287 payloads.
    let steps = json!([
        {"event": "PostToolUse", "variant": "parallel-1", "gate": true},
        {"event": "Stop", "variant": "default", "gate": true},
    ]);
    let (_control_dir, control, spec) = scripted_builder("turn.json", Some(&steps));
    let s = harness_session(
        "mcp_tools",
        "traffic",
        SessionOptions {
            instances: vec!["overseer".to_owned(), spec],
            ..SessionOptions::default()
        },
    );
    let home = s.home().to_path_buf();
    let mcp = Mcp::spawn(&s);

    let sent = mcp.call(
        "send",
        json!({"target": "builder", "text": format!("{CANARY} through the tool")}),
    );
    assert_eq!(sent.is_error, Some(false), "{sent:?}");
    let ok = body(&sent);
    assert_eq!(keys(&ok), ["cursor", "submitted_at"]);
    let cursor = ok["cursor"].as_u64().expect("a cursor");

    // The turn's tool activity lands first, then its end: the wait from the send's cursor passes
    // the activity line and wakes on the end.
    release(&control, 1);
    wait_for_kind(&s.instance_dir("builder"), "activity");
    release(&control, 1);
    let woken = mcp.call("wait", json!({"target": "builder", "after": cursor}));
    assert_eq!(woken.is_error, Some(false));
    let woke = body(&woken);
    assert_eq!(woke["event"]["kind"], "turn-ended", "{woke}");
    let after = cursor.to_string();
    let cli_woke = cli_json(&s, &["wait", "builder", "--after", &after, "--json"], None);
    assert_eq!(woke, cli_woke["ok"]);

    let last = mcp.call("last", json!({"target": "builder"}));
    assert_eq!(last.is_error, Some(false));
    assert_eq!(
        body(&last),
        cli_json(&s, &["last", "builder", "--json"], None)["ok"]
    );

    let end = woke["cursor"].as_u64().expect("the wake's cursor");
    let timed_out = mcp.call(
        "wait",
        json!({"target": "builder", "after": end, "timeout_ms": 500}),
    );
    assert_eq!(body(&timed_out), json!({"timed_out": true}));

    let cli_sent = cli_json(&s, &["send", "builder", "--json"], Some(CANARY.as_bytes()));
    assert_eq!(keys(&cli_sent["ok"]), keys(&ok));

    // The send's frame on the server's side, its connection the server's own.
    let request = diagnostics(&home, "mcp.ndjson")
        .into_iter()
        .find(|l| l["event"] == "channel-request" && l["method"] == "send")
        .expect("the send's channel-request");
    assert_eq!(request["process"], "mcp");
    assert!(
        request["conn"]
            .as_str()
            .is_some_and(|c| c.starts_with("mcp-")),
        "{request}"
    );
    let shown =
        std::fs::read_to_string(home.join("diagnostics").join("mcp.ndjson")).expect("mcp.ndjson");
    assert!(!shown.contains(CANARY), "the send's text reached a line");

    let exe = viola_exe(&s);
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
    let gone = mcp.call("wait", json!({"target": "builder", "timeout_ms": 500}));
    let cli_gone = Command::new(&exe)
        .arg("--home")
        .arg(&home)
        .args(["wait", "builder", "--timeout-ms", "500", "--json"])
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::null())
        .output()
        .expect("viola wait");
    assert_eq!(cli_gone.status.code(), Some(21));
    assert_eq!(
        body(&gone),
        less_v(&String::from_utf8(cli_gone.stdout).expect("utf-8"))
    );
    assert_eq!(gone.is_error, Some(true));
    assert_eq!(
        body(&gone),
        json!({"error": "instance-unreachable", "detail": null})
    );
    assert_eq!(text(&gone), "refused: instance-unreachable");
    assert_sanitized(&[&gone], &home);
}

/// Path 4: a raised question answered by its id through the tool, and the same id again unknown.
/// Path 5: after the human's `pause`, `send` is refused `human-typing` with no word of `release`.
#[test]
fn mcp_tools_path4_path5_answer_by_id_and_a_paused_send() {
    let ws = viola_e2e::harness::Workspace::from_build();
    let (_control_dir, control, spec) = scripted_builder("fixtures/fake-scripts/path4.json", None);
    let s = harness_session(
        "mcp_tools",
        "dialogs",
        SessionOptions {
            instances: vec!["overseer".to_owned(), spec],
            ..SessionOptions::default()
        },
    );
    let mcp = Mcp::spawn(&s);

    release(&control, 1);
    let raised = mcp.call("wait", json!({"target": "builder", "after": 0}));
    let raised = body(&raised);
    assert_eq!(raised["event"]["kind"], "question", "{raised}");
    let dialog_id = raised["event"]["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog id");

    let fixture = ws
        .root
        .join("fixtures/claude/2.1.287/PreToolUse.ask-user-question.json");
    let fixture: Value =
        serde_json::from_slice(&std::fs::read(fixture).expect("fixture")).expect("json");
    let asked = fixture["tool_input"]["questions"][0]["question"]
        .as_str()
        .expect("a question")
        .to_owned();
    let response = json!({"answers": {asked: format!("{CANARY} red")}});
    let answered = mcp.call(
        "answer",
        json!({"target": "builder", "dialog_id": dialog_id, "response": response}),
    );
    assert_eq!(answered.is_error, Some(false), "{answered:?}");
    let again = mcp.call(
        "answer",
        json!({"target": "builder", "dialog_id": dialog_id, "response": response}),
    );
    assert_refused(&again, "not-delivered", Some("unknown-dialog"));
    let id = dialog_id.to_string();
    let cli_again = viola(
        &s,
        &["answer", "builder", &id, "--json"],
        Some(response.to_string().as_bytes()),
        &[],
    );
    assert_eq!(cli_again.code, Some(13), "stderr: {}", cli_again.stderr);
    assert_eq!(body(&again), less_v(&cli_again.stdout));

    let paused = viola(&s, &["pause", "builder", "--json"], None, &[]);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    let refused = mcp.call(
        "send",
        json!({"target": "builder", "text": format!("{CANARY} while paused")}),
    );
    assert_eq!(refused.is_error, Some(true));
    assert_eq!(text(&refused), "refused: human-typing");
    let cli_refused = viola(
        &s,
        &["send", "builder", "--json"],
        Some(CANARY.as_bytes()),
        &[],
    );
    assert_eq!(cli_refused.code, Some(10), "stderr: {}", cli_refused.stderr);
    assert_eq!(body(&refused), less_v(&cli_refused.stdout));
    assert!(
        !serde_json::to_string(&refused)
            .expect("json")
            .contains("release")
    );
    assert_sanitized(&[&again, &refused], s.home());

    drop(mcp);
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
}

/// Path 6: a five-hour reading over the threshold, fed only through `viola hook statusline`, refuses
/// the tool's `send` as the CLI refuses it: no cursor, no `release`.
#[test]
fn mcp_tools_path6_budget_paused_send() {
    let s = harness_session(
        "mcp_tools",
        "budget",
        SessionOptions {
            statusline_echo: true,
            ..SessionOptions::default()
        },
    );
    let builder = s.instance_dir("builder");
    let usage = json!({
        "session_id": CANARY,
        "model": {"display_name": "synthetic"},
        "rate_limits": {
            "five_hour": {"used_percentage": 91, "resets_at": FIVE_HOUR_RESET},
            "seven_day": {"used_percentage": 10, "resets_at": SEVEN_DAY_RESET},
        },
    })
    .to_string();
    let feed_env = [
        ("VIOLA_NAME", OsString::from("builder")),
        ("VIOLA_DIR", builder.as_os_str().to_owned()),
    ];
    let fed = viola(
        &s,
        &["hook", "statusline"],
        Some(usage.as_bytes()),
        &feed_env,
    );
    assert_eq!(fed.code, Some(0));
    assert!(s.home().join("budget.json").is_file(), "no reading filed");

    let mcp = Mcp::spawn(&s);
    let refused = mcp.call(
        "send",
        json!({"target": "builder", "text": format!("{CANARY} over the budget")}),
    );
    assert_refused(&refused, "budget-paused", Some("five-hour"));
    assert!(body(&refused).get("cursor").is_none());
    let cli = cli_json(&s, &["send", "builder", "--json"], Some(CANARY.as_bytes()));
    assert_eq!(
        cli,
        json!({"v": 1, "refusal": "budget-paused", "detail": "five-hour"})
    );

    drop(mcp);
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
}

/// Path 7: on an unstamped home an answer is refused `unverified-cli`, as the CLI refuses it.
#[test]
fn mcp_tools_path7_unverified_cli_answer() {
    let s = harness_session(
        "mcp_tools",
        "unstamped",
        SessionOptions {
            unstamped: true,
            ..SessionOptions::default()
        },
    );
    let mcp = Mcp::spawn(&s);
    let refused = mcp.call(
        "answer",
        json!({"target": "builder", "dialog_id": 1, "response": {"behavior": "allow"}}),
    );
    assert_refused(&refused, "unverified-cli", None);
    let cli = viola(
        &s,
        &["answer", "builder", "1", "--json"],
        Some(br#"{"behavior": "allow"}"#),
        &[],
    );
    assert_eq!(cli.code, Some(12), "stderr: {}", cli.stderr);
    let doc: Value = serde_json::from_str(&cli.stdout).expect("one JSON document");
    assert_eq!(
        doc,
        json!({"v": 1, "refusal": "unverified-cli", "detail": null})
    );
    assert_sanitized(&[&refused], s.home());

    drop(mcp);
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
}
