//! The two diagnostics line schemas (obs-plan §8): the `event` enum against the test-plan §3
//! literals, null-as-absence, and conformance of every line a real `viola run` writes.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::FAKE;
use support::home::{TestHome, VIOLA, home, workspace_path};
use support::hygiene::load_schema;
use support::watch::{WITHIN, Watch};

/// test-plan §3 Log format + obs-plan D-01…D-05; `a11y-violation` is a harness-only row.
const LOG_FORMAT_EVENTS: [&str; 19] = [
    "channel-request",
    "channel-response",
    "dialog-raised",
    "dialog-answered",
    "hook-invoked",
    "hook-decision",
    "send-issued",
    "send-confirmed",
    "send-refused",
    "release-from-driver",
    "process-start",
    "process-exit",
    "http-request",
    "panic",
    "liveness-changed",
    "state-recovered",
    "sse-opened",
    "sse-closed",
    "parse-rejected",
];

fn line_schema() -> Value {
    load_schema(&workspace_path("schemas/diag-line.v1.json"))
}

fn detail_schema() -> Value {
    load_schema(&workspace_path("schemas/diag-detail.v1.json"))
}

/// Codes only in a failure message: the line number and the failing schema and instance
/// locations, never the line's content.
fn violations(schema: &Value, file: &str, lines: &[Value]) -> Vec<String> {
    let validator = jsonschema::validator_for(schema).expect("valid schema");
    lines
        .iter()
        .enumerate()
        .flat_map(|(n, line)| {
            validator
                .iter_errors(line)
                .map(move |e| {
                    format!(
                        "{file}:{}: {} at {}",
                        n + 1,
                        e.schema_path(),
                        e.instance_path()
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn enum_strings(v: &Value) -> Vec<&str> {
    v.as_array()
        .expect("enum array")
        .iter()
        .map(|e| e.as_str().expect("string"))
        .collect()
}

#[test]
fn diag_line_schema_event_enum_equals_the_log_format_literals() {
    let line = line_schema();
    assert_eq!(
        enum_strings(&line["$defs"]["event"]["enum"]),
        LOG_FORMAT_EVENTS
    );
    let detail = detail_schema();
    assert_eq!(
        enum_strings(&detail["properties"]["event"]["enum"]),
        LOG_FORMAT_EVENTS
    );
}

fn channel_request() -> Value {
    json!({
        "timestamp": "2026-09-24T03:12:07.412Z", "level": "INFO", "target": "viola_channel::server",
        "message": "channel-request", "event": "channel-request", "process": "run",
        "instance": "builder", "corr": 7, "method": "send", "conn": "cli-4812-1790219525118-1"
    })
}

#[rstest]
#[case::null_corr("corr", Value::Null)]
#[case::null_instance("instance", Value::Null)]
#[case::message_not_the_event("message", json!("channel-response"))]
#[case::uncatalogued_key("prompt", json!("text"))]
#[case::bad_timestamp("timestamp", json!("2026-09-24T03:12:07Z"))]
fn diag_line_schema_rejects_literal_null_corr_and_instance(
    #[case] key: &str,
    #[case] value: Value,
) {
    let schema = line_schema();
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    let valid = channel_request();
    assert!(validator.is_valid(&valid));
    let mut without = valid.clone();
    let obj = without.as_object_mut().expect("object");
    obj.remove("instance");
    assert!(validator.is_valid(&without));
    let mut notification = without.clone();
    let obj = notification.as_object_mut().expect("object");
    obj.remove("corr");
    obj.insert("method".to_owned(), json!("hook.event"));
    assert!(validator.is_valid(&notification));
    let mut bad = valid;
    bad[key] = value;
    assert!(!validator.is_valid(&bad));
}

/// A codes-only line of `event` with `extra` fields and no `corr`.
fn line_without_corr(event: &str, extra: Value) -> Value {
    let mut line = json!({
        "timestamp": "2026-09-27T03:12:07.412Z", "level": "INFO", "target": "viola::test",
        "message": event, "event": event, "process": "run", "instance": "builder"
    });
    let obj = line.as_object_mut().expect("object");
    for (key, value) in extra.as_object().expect("extra fields") {
        obj.insert(key.clone(), value.clone());
    }
    line
}

/// Each line whose `corr` obs-plan §3 always defines is refused without it, and valid with it.
#[rstest]
#[case::dialog_raised("dialog-raised", json!({"dialog_kind": "question"}))]
#[case::dialog_answered("dialog-answered", json!({"dialog_kind": "permission"}))]
#[case::release_from_driver("release-from-driver", json!({"conn": "mcp-1-2-3"}))]
#[case::channel_request("channel-request", json!({"method": "send", "conn": "cli-1-2-3"}))]
#[case::channel_request_unlisted_method("channel-request", json!({"srv_conn": "srv-1"}))]
#[case::channel_response_fault("channel-response", json!({"result_class": "error", "error_code": -32601}))]
#[case::channel_response_ok("channel-response", json!({"result_class": "ok"}))]
#[case::send_wrapper("send-issued", json!({"side": "wrapper", "rpc_id": 3}))]
#[case::send_refused_wrapper("send-refused", json!({"side": "wrapper", "refusal": "human-typing"}))]
#[case::hook_decision_dialog("hook-decision", json!({"hook_event": "permission-request", "decision_emitted": true}))]
#[case::hook_decision_question("hook-decision", json!({"hook_event": "pre-tool-use"}))]
fn diag_line_schema_requires_corr_where_it_is_always_defined(
    #[case] event: &str,
    #[case] extra: Value,
) {
    let validator = jsonschema::validator_for(&line_schema()).expect("valid schema");
    let without = line_without_corr(event, extra);
    assert!(!validator.is_valid(&without), "{without}");
    let mut with = without;
    with["corr"] = json!(12);
    assert!(validator.is_valid(&with), "{with}");
}

/// Each null `corr` obs-plan documents stays valid without it.
#[rstest]
#[case::notification("channel-request", json!({"method": "hook.event", "srv_conn": "srv-2"}))]
#[case::parse_error_reply("channel-response", json!({"result_class": "error", "error_code": -32700}))]
#[case::oversize_reply("channel-response", json!({"result_class": "error", "error_code": -32600}))]
#[case::client_send_refused("send-refused", json!({"side": "client", "refusal": "not-delivered", "detail": "control-character"}))]
#[case::non_dialog_hook_invoked("hook-invoked", json!({"hook_event": "statusline"}))]
#[case::fail_open_hook_invoked("hook-invoked", json!({"hook_event": "pre-tool-use"}))]
#[case::non_dialog_hook_decision("hook-decision", json!({"hook_event": "statusline", "budget_written": true}))]
#[case::fail_open_hook_decision("hook-decision", json!({"hook_event": "permission-request", "detail": "channel-unreachable"}))]
fn diag_line_schema_accepts_each_documented_null_corr(#[case] event: &str, #[case] extra: Value) {
    let validator = jsonschema::validator_for(&line_schema()).expect("valid schema");
    let line = line_without_corr(event, extra);
    assert!(validator.is_valid(&line), "{line}");
}

#[test]
fn diag_line_schema_confines_corr_to_its_events() {
    let schema = line_schema();
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    let start = json!({
        "timestamp": "2026-09-24T03:12:07.412Z", "level": "INFO", "target": "viola::run",
        "message": "process-start", "event": "process-start", "process": "run",
        "instance": "builder", "subject": "self"
    });
    assert!(validator.is_valid(&start));
    let mut with_corr = start;
    with_corr["corr"] = json!(1);
    assert!(!validator.is_valid(&with_corr));
}

#[test]
fn diag_detail_schema_allows_content_fields_only_there() {
    let detail = json!({
        "timestamp": "2026-09-24T03:12:07.412Z", "level": "ERROR", "target": "viola::panic",
        "message": "panic", "event": "panic", "process": "run", "instance": "builder",
        "panic_location": "src/x.rs:3", "thread": "main", "panic_payload": "p", "backtrace": ["f"]
    });
    let detail_validator = jsonschema::validator_for(&detail_schema()).expect("valid schema");
    assert!(detail_validator.is_valid(&detail));
    let line_validator = jsonschema::validator_for(&line_schema()).expect("valid schema");
    assert!(!line_validator.is_valid(&detail));
    let mut no_instance = detail.clone();
    no_instance
        .as_object_mut()
        .expect("object")
        .remove("instance");
    assert!(!detail_validator.is_valid(&no_instance));
    let mut located_chain = detail;
    located_chain["event"] = json!("process-exit");
    located_chain["message"] = json!("process-exit");
    assert!(!detail_validator.is_valid(&located_chain));
}

fn run_lines(home: &Path, program: &str, config: Option<&str>) -> Vec<Value> {
    if let Some(config) = config {
        std::fs::create_dir_all(home).expect("home");
        std::fs::write(home.join("config.json"), config).expect("config");
    }
    // The receipt sits outside the home, which stays viola's to create.
    let receipts = tempfile::tempdir().expect("receipt dir");
    let receipt = receipts.path().join("r.ndjson");
    let mut cmd = Command::new(VIOLA);
    cmd.arg("--home")
        .arg(home)
        .args(["run", "builder", "--", program]);
    if program == FAKE {
        cmd.arg("--receipt").arg(&receipt);
    }
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("viola runs");
    // Ctrl-C only once the child's terminal is raw (its `start` receipt): sooner, it is swallowed.
    let watch = Watch::start("raw");
    let deadline = Instant::now() + WITHIN;
    loop {
        let exited = child.try_wait().expect("try_wait").is_some();
        if exited {
            break;
        }
        let started = std::fs::read_to_string(&receipt)
            .unwrap_or_default()
            .contains("\"kind\":\"start\"");
        if started {
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(b"\x03");
            }
            break;
        }
        watch.note(&format!("start receipt {started} exited {exited}"));
        watch.deadline_check(deadline, "the wrapped program never started");
        std::thread::yield_now();
    }
    let watch = Watch::start("exit");
    let deadline = Instant::now() + WITHIN;
    loop {
        let exited = child.try_wait().expect("try_wait").is_some();
        if exited {
            break;
        }
        watch.note(&format!("exited {exited}"));
        watch.deadline_check(deadline, "viola never exited");
        std::thread::yield_now();
    }
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

#[rstest]
fn diag_lines_from_real_runs_validate(
    #[from(home)] fake: TestHome,
    #[from(home)] missing: TestHome,
    #[from(home)] config: TestHome,
) {
    let schema = line_schema();
    let absent = missing.scratch().join("no-such-program");
    let runs = [
        ("fake-agent", run_lines(fake.path(), FAKE, None)),
        (
            "missing-program",
            run_lines(missing.path(), absent.to_str().expect("utf-8"), None),
        ),
        (
            "malformed-config",
            run_lines(
                config.path(),
                FAKE,
                Some(r#"{"v":1,"diagnostics_level":"loud","x":1}"#),
            ),
        ),
    ];
    let mut seen = 0;
    for (label, lines) in &runs {
        assert!(!lines.is_empty(), "{label}: no lines");
        seen += lines.len();
        assert_eq!(violations(&schema, label, lines), Vec::<String>::new());
    }
    // Each run's four, two and five lines plus its version-probe start and exit.
    assert_eq!(seen, 6 + 4 + 7);
}
