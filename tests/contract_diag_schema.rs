//! The two diagnostics line schemas (obs-plan §8): the `event` enum against the test-plan §3
//! literals, null-as-absence, and conformance of every line a real `viola run` writes.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::FAKE;
use support::home::{TestHome, VIOLA, home, workspace_path};
use support::hygiene::load_schema;

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
    obj.remove("corr");
    obj.remove("instance");
    assert!(validator.is_valid(&without));
    let mut bad = valid;
    bad[key] = value;
    assert!(!validator.is_valid(&bad));
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
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .args(["run", "builder", "--", program])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("viola runs");
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(b"\x03");
    }
    child.wait().expect("viola exits");
    std::fs::read_to_string(home.join("diagnostics").join("run-builder.ndjson"))
        .expect("role file")
        .lines()
        .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
        .collect()
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
    assert_eq!(seen, 4 + 2 + 5);
}
