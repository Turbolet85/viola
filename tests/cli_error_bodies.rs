//! The CLI's error documents (security-plan §Error Handling; design-system §Surface: cli, Streams):
//! each failure a verb reaches under `--json`, with the home's path, an upstream text and a planted
//! stripped value in play, prints one document with its literal exit, stderr empty and no ESC byte,
//! and the error-body check reads it clean. The check's own self-check stands here too.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::Running;
use support::events::{assert_error_body_clean, error_body_hit};
use support::fake::empty_bin;
use support::home::{StampedHome, TestHome, UNWRAPPED, VIOLA, stamped_home, unwiden, widen};

const STRIPPED_NAME: &str = "CLAUDE_VIOLA_TEST_NEW_X";
const STRIPPED_VALUE: &str = "canary-newname-value-e2f8";
const UPSTREAM: &str = "canary-chain-value-5c1e upstream text";

const UNREACHABLE: &str = "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n";

/// One failure: a label, the verb's words, its stdin, its `VIOLA_NAME`, the literal exit and the
/// literal document.
struct Row {
    label: &'static str,
    args: &'static [&'static str],
    stdin: Option<String>,
    from: Option<&'static str>,
    exit: i32,
    doc: &'static str,
}

fn row(label: &'static str, args: &'static [&'static str], exit: i32, doc: &'static str) -> Row {
    Row {
        label,
        args,
        stdin: None,
        from: None,
        exit,
        doc,
    }
}

/// `row` with `text` on its stdin.
fn fed(
    label: &'static str,
    args: &'static [&'static str],
    text: String,
    exit: i32,
    doc: &'static str,
) -> Row {
    Row {
        stdin: Some(text),
        ..row(label, args, exit, doc)
    }
}

/// The verb under `--json` with the planted value on its child and `PATH` an empty bin dir, read
/// whole. The check comes first: an assertion that fails prints what it read.
fn assert_row(home: &Path, bin: &Path, row: &Row) {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home)
        .args(row.args)
        .env(STRIPPED_NAME, STRIPPED_VALUE)
        .env("PATH", bin)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(if row.stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    if let Some(from) = row.from {
        command.env("VIOLA_NAME", from);
    }
    let mut child = command.spawn().expect("viola");
    if let Some(text) = &row.stdin {
        let mut pipe = child.stdin.take().expect("stdin");
        match pipe.write_all(text.as_bytes()) {
            Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => panic!("stdin: {e}"),
            _ => {}
        }
    }
    let ran = Running::over(child).finish();
    let label = row.label;
    assert!(ran.stderr.is_empty(), "{label}: stderr holds bytes");
    assert_eq!(ran.stdout.lines().count(), 1, "{label}: one document");
    assert!(!ran.stdout.contains('\u{1b}'), "{label}: an ESC byte");
    let body: Value = serde_json::from_str(&ran.stdout).expect("a JSON document");
    assert_error_body_clean(label, &body, home, &[STRIPPED_VALUE, UPSTREAM]);
    assert_eq!(ran.code, Some(row.exit), "{label}");
    assert_eq!(ran.stdout, row.doc, "{label}");
}

/// A text holding the home's path and the upstream text.
fn loaded(home: &Path, tail: &str) -> String {
    format!("{} {UPSTREAM}{tail}", home.display())
}

/// A deny answer whose message holds the same.
fn loaded_answer(home: &Path) -> String {
    json!({"behavior": "deny", "message": loaded(home, "")}).to_string()
}

#[rstest]
fn cli_error_documents_hold_no_path_or_upstream_text(stamped_home: StampedHome) {
    // Over a home no wrapper lives in.
    let tmp = TestHome::new();
    let bin = empty_bin(tmp.scratch());
    let absent = tmp.path();
    let dead = [
        fed(
            "send, no wrapper",
            &["send", "builder", "--json"],
            loaded(absent, ""),
            21,
            UNREACHABLE,
        ),
        row(
            "wait, no wrapper",
            &["wait", "builder", "--json", "--timeout-ms", "100"],
            21,
            UNREACHABLE,
        ),
        row(
            "last, no wrapper",
            &["last", "builder", "--json"],
            21,
            UNREACHABLE,
        ),
        fed(
            "answer, no wrapper",
            &["answer", "builder", "1", "--json"],
            loaded_answer(absent),
            21,
            UNREACHABLE,
        ),
        row(
            "pause, no wrapper",
            &["pause", "builder", "--json"],
            21,
            UNREACHABLE,
        ),
        row(
            "release, no wrapper",
            &["release", "builder", "--json"],
            21,
            UNREACHABLE,
        ),
        row(
            "link, no wrapper",
            &["link", "overseer", "builder", "--json"],
            21,
            UNREACHABLE,
        ),
        row(
            "unlink, no wrapper",
            &["unlink", "overseer", "builder", "--json"],
            21,
            UNREACHABLE,
        ),
        row(
            "revive, no session",
            &["revive", "builder", "--json"],
            1,
            "{\"v\":1,\"error\":\"no-session\",\"detail\":null}\n",
        ),
    ];
    for row in &dead {
        assert_row(absent, &bin, row);
    }

    // A home another user could write, started unseeded.
    let widened = TestHome::new();
    viola_state::fs::create_private_dir(widened.path()).expect("the home");
    widen(widened.path());
    let refused = [
        row("list, a widened home", &["list", "--json"], 21, UNREACHABLE),
        row(
            "plugin install, a widened home",
            &["plugin", "install", "--json"],
            21,
            UNREACHABLE,
        ),
    ];
    for row in &refused {
        assert_row(widened.path(), &bin, row);
    }
    unwiden(widened.path());

    // Against a live wrapper.
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let home = wrapper.home().to_path_buf();
    let live = [
        fed(
            "send of a control character",
            &["send", "builder", "--json"],
            loaded(&home, "\u{1b}[201~"),
            13,
            "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"control-character\"}\n",
        ),
        fed(
            "send of an empty text",
            &["send", "builder", "--json"],
            String::new(),
            13,
            "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"empty-text\"}\n",
        ),
        fed(
            "answer of an unknown id",
            &["answer", "builder", "1", "--json"],
            loaded_answer(&home),
            13,
            "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n",
        ),
        row(
            "pause, the step before the refused send",
            &["pause", "builder", "--json"],
            0,
            "{\"v\":1,\"ok\":{\"wheel\":\"human\"}}\n",
        ),
        fed(
            "send after pause",
            &["send", "builder", "--json"],
            loaded(&home, ""),
            10,
            "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n",
        ),
        Row {
            from: Some("overseer"),
            ..row(
                "release under VIOLA_NAME",
                &["release", "builder", "--json"],
                20,
                "{\"v\":1,\"error\":\"wrapper-fault\",\"detail\":{\"code\":-32602,\
                 \"message\":\"invalid params\",\"data\":{\"reason\":\"release-from-driver\"}}}\n",
            )
        },
        row(
            "link of a self-pair",
            &["link", "builder", "builder", "--json"],
            20,
            "{\"v\":1,\"error\":\"wrapper-fault\",\"detail\":{\"code\":-32602,\
             \"message\":\"invalid params\",\"data\":null}}\n",
        ),
    ];
    for row in &live {
        assert_row(&home, &bin, row);
    }
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// One body per class reads its hit, and a closed body reads none.
#[rstest]
#[case::unix_home(json!({"message": "at /home/x/y"}), Some("absolute-path"))]
#[case::mac_home(json!({"data": {"p": "/Users/x"}}), Some("absolute-path"))]
#[case::windows_users(json!(["C:\\Users\\x"]), Some("absolute-path"))]
#[case::drive_letter_inside(json!({"message": "at D:/x"}), Some("absolute-path"))]
#[case::the_home_given(json!({"message": "at /srv/vhome/x"}), Some("absolute-path"))]
#[case::a_key(json!({"/home/x/y": 1}), Some("absolute-path"))]
#[case::caused_by(json!({"message": "Caused by: x"}), Some("caused-by"))]
#[case::serde_path(json!({"message": "expected a value at line 1 column 2"}), Some("serde-path"))]
#[case::planted(json!({"detail": ["a canary-planted-value b"]}), Some("planted"))]
#[case::a_closed_fault(json!({"code": -32603, "message": "internal error", "data": null}), None)]
#[case::a_closed_refusal(json!({"refusal": "not-delivered", "detail": "control-character"}), None)]
#[case::a_scheme_is_no_drive(json!({"message": "viola://x"}), None)]
#[case::a_line_without_a_column(json!({"message": "at line 1"}), None)]
fn error_body_check_reads_each_class_as_a_hit(#[case] body: Value, #[case] hit: Option<&str>) {
    let home = Path::new("/srv/vhome");
    assert_eq!(
        error_body_hit(&body, home, &["canary-planted-value"]),
        hit,
        "{body}"
    );
}

/// A hit names its place and its class, and never the bytes it read.
#[test]
fn error_body_check_reports_no_byte_of_what_it_read() {
    let body = json!({"message": "at /home/canary-planted-value"});
    let panicked = std::panic::catch_unwind(|| {
        assert_error_body_clean("the place", &body, Path::new("/srv/vhome"), &[]);
    })
    .expect_err("a hit");
    let report = panicked.downcast_ref::<String>().expect("the report");
    assert_eq!(report, "the place: an error body holds absolute-path");
}
