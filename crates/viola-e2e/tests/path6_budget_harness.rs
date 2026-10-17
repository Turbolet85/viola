//! Critical Path 6 in harness-session form (test-plan §6 Path 6; architecture [Budget Governor]):
//! a session booted with `--statusline-echo`, a reading fed through `viola hook statusline`, and
//! step 7's two-instance leg — the human's `release --budget` on `builder` lifts that instance's
//! pause alone while `overseer` stays paused. Every verb runs from the session's bin dir with no
//! `VIOLA_*` variable of this process; the statusline feed sets its own.

use std::ffi::OsString;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::{Value, json};
use viola_e2e::harness::boot::session_path;
use viola_e2e::harness::read_json;
use viola_e2e::harness::session::{HarnessSession, SessionOptions, harness_session};
use viola_e2e::harness::status::status;

/// What the fake agent's `statusline-echo` mode prints: a test literal.
#[cfg(unix)]
const STATUSLINE_ECHO_OUTPUT: &str = "viola-fake-statusline\n";
const FIVE_HOUR_RESET: &str = "2099-01-01T00:00:00Z";
const FIVE_HOUR_RESET_STORED: &str = "2099-01-01T00:00:00.000Z";
const SEVEN_DAY_RESET: &str = "2099-01-06T00:00:00Z";
const REFUSED_FIVE_HOUR: &str =
    "{\"v\":1,\"refusal\":\"budget-paused\",\"detail\":\"five-hour\"}\n";
const CANARY: &str = "canary-chain-value-5c1e";

struct Ran {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

/// `<bin dir>/viola --home <home> <args…>` on the session's `PATH`, with this process's `VIOLA_*`
/// names removed and `env` set, `stdin` written and closed.
fn viola(s: &HarnessSession, args: &[&str], stdin: Option<&[u8]>, env: &[(&str, OsString)]) -> Ran {
    let session_bin = s.workspace().session_dir(s.session()).join("bin");
    let mut command = Command::new(
        s.bin_dir()
            .join(format!("viola{}", std::env::consts::EXE_SUFFIX)),
    );
    command
        .arg("--home")
        .arg(s.home())
        .args(args)
        .env("PATH", session_path(&session_bin))
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

/// A reading of both windows, each reset far ahead of any run, with the tests' canary.
fn usage(five_hour: u64, seven_day: u64) -> Vec<u8> {
    json!({
        "session_id": CANARY,
        "model": {"display_name": "synthetic"},
        "rate_limits": {
            "five_hour": {"used_percentage": five_hour, "resets_at": FIVE_HOUR_RESET},
            "seven_day": {"used_percentage": seven_day, "resets_at": SEVEN_DAY_RESET},
        },
    })
    .to_string()
    .into_bytes()
}

fn send(s: &HarnessSession, name: &str) -> Ran {
    let text = format!("{CANARY} over the budget");
    viola(s, &["send", name, "--json"], Some(text.as_bytes()), &[])
}

fn json_doc(ran: &Ran) -> Value {
    assert!(!ran.stdout.contains('\x1b'), "an ESC byte in a --json read");
    serde_json::from_str(&ran.stdout).expect("one JSON document")
}

fn events(instance_dir: &Path) -> Vec<Value> {
    let text = std::fs::read_to_string(instance_dir.join("events.ndjson")).expect("events");
    text.split_inclusive('\n')
        .filter(|l| l.ends_with('\n'))
        .map(|l| serde_json::from_str(l).expect("an event line"))
        .collect()
}

/// The `budget-gate` records that carry an override.
fn overrides(instance_dir: &Path) -> Vec<Value> {
    events(instance_dir)
        .into_iter()
        .filter(|l| l["kind"] == "budget-gate" && l["data"].get("override_until").is_some())
        .collect()
}

fn item<'a>(board: &'a Value, name: &str) -> &'a Value {
    board["ok"]["items"]
        .as_array()
        .and_then(|items| items.iter().find(|i| i["name"] == name))
        .unwrap_or_else(|| panic!("item {name} in {board}"))
}

/// An RFC 3339 UTC instant with milliseconds, as viola writes one: its text sorts as its time.
fn stored_instant(value: &Value) -> &str {
    let text = value.as_str().expect("an instant");
    assert!(text.len() == 24 && text.ends_with('Z'), "{text}");
    text
}

#[test]
fn path6_two_instance_release_over_a_harness_session() {
    let s = harness_session(
        "path6_budget_harness",
        "release",
        SessionOptions {
            statusline_echo: true,
            ..SessionOptions::default()
        },
    );
    let home = s.home().to_path_buf();
    let builder = s.instance_dir("builder");
    let overseer = s.instance_dir("overseer");

    // 1. The boot flag planted the user's statusline command before the wrappers started.
    let source = home.join("statusline-source.json");
    let doc: Value = read_json(&source).expect("statusline source");
    let command = doc["statusLine"]["command"].as_str().expect("a command");
    assert_eq!(doc["statusLine"]["type"], "command");
    assert!(
        command.contains("viola-fake-agent") && command.contains(" statusline-echo "),
        "{command}"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = std::fs::metadata(&source)
            .expect("source")
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    // 2. A five-hour reading of 91 through the arm, as `builder` runs it.
    let feed_env = [
        ("VIOLA_NAME", OsString::from("builder")),
        ("VIOLA_DIR", builder.as_os_str().to_owned()),
    ];
    let fed = viola(&s, &["hook", "statusline"], Some(&usage(91, 10)), &feed_env);
    assert_eq!(fed.code, Some(0));
    assert!(fed.stderr.is_empty(), "stderr: {} bytes", fed.stderr.len());
    let marker = home.join("fake").join("statusline.marker");
    #[cfg(unix)]
    {
        assert_eq!(fed.stdout, STATUSLINE_ECHO_OUTPUT);
        let filed = std::fs::metadata(&marker).expect("the marker").len();
        assert!(filed > 0, "the user's command filed nothing");
    }
    // Windows wraps no status line: the arm runs no command and prints nothing.
    #[cfg(not(unix))]
    {
        assert!(fed.stdout.is_empty());
        assert!(!marker.exists());
    }
    let budget: Value = read_json(&home.join("budget.json")).expect("budget.json");
    assert_eq!(
        budget["five_hour"]["used_percentage"].as_f64(),
        Some(91.0),
        "{budget}"
    );

    // 3. Both instances refuse an automated send.
    for name in ["builder", "overseer"] {
        let refused = send(&s, name);
        assert_eq!(refused.code, Some(11), "{name}: {}", refused.stderr);
        assert_eq!(refused.stdout, REFUSED_FIVE_HOUR, "{name}");
    }

    // 4. The human lifts builder's pause; no `VIOLA_NAME` rides the release.
    let lifted = viola(
        &s,
        &["release", "builder", "--budget", "--json"],
        Some(b""),
        &[],
    );
    assert_eq!(lifted.code, Some(0), "stderr: {}", lifted.stderr);
    let lift = json_doc(&lifted);
    assert_eq!(lift["ok"]["budget_paused"], false);
    assert_eq!(lift["ok"]["override_until"], FIVE_HOUR_RESET_STORED);

    // 5. The board: the reading still crosses, builder's row lifted, overseer's still paused.
    let listed = viola(&s, &["list", "--json"], None, &[]);
    assert_eq!(listed.code, Some(0), "stderr: {}", listed.stderr);
    let board = json_doc(&listed);
    assert_eq!(board["ok"]["budget"]["paused"], true, "{board}");
    assert_eq!(item(&board, "builder")["budget_paused"], false);
    assert_eq!(item(&board, "overseer")["budget_paused"], true);
    let snapshot: Value = read_json(&builder.join("snapshot.json")).expect("snapshot");
    assert_eq!(
        snapshot["data"]["budget_override_until"],
        lift["ok"]["override_until"]
    );
    let records = overrides(&builder);
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(
        records[0]["data"],
        json!({"paused": false, "override_until": FIVE_HOUR_RESET_STORED})
    );
    assert!(
        stored_instant(&records[0]["data"]["override_until"]) > stored_instant(&records[0]["ts"])
    );
    assert!(overrides(&overseer).is_empty(), "overseer took no override");

    // 6. The human board's WHEEL cell names the pause on overseer's row alone.
    let human = viola(&s, &["list"], None, &[]);
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert!(!human.stdout.contains('\x1b'));
    let row = |name: &str| {
        human
            .stdout
            .lines()
            .find(|l| l.starts_with(&format!("{name} ")))
            .unwrap_or_else(|| panic!("row {name} in {}", human.stdout))
            .to_owned()
    };
    assert!(
        row("overseer").contains("driver - budget-paused"),
        "{}",
        row("overseer")
    );
    assert!(
        !row("builder").contains("budget-paused"),
        "{}",
        row("builder")
    );
    assert!(row("builder").contains("driver"), "{}", row("builder"));

    // 7. builder takes a send again; overseer still refuses it.
    let sent = send(&s, "builder");
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let still = send(&s, "overseer");
    assert_eq!(still.code, Some(11), "stderr: {}", still.stderr);
    assert_eq!(still.stdout, REFUSED_FIVE_HOUR);

    // 8. The harness status reads both instances live on the board.
    let st = status(s.workspace(), s.session());
    assert_eq!(st.code, 0, "{}", st.doc);
    assert_eq!(st.doc["state"], "ready");
    for name in ["builder", "overseer"] {
        let listed = item(&st.doc["list"], name);
        assert_eq!(listed["wrapped"], true, "{name}");
        assert_eq!(listed["liveness"], "live", "{name}");
    }

    // 9. The teardown proves every process and endpoint gone.
    let report = s.cleanup();
    assert_eq!(report.code, 0, "{}", report.doc);
    let fields = report.doc.as_object().expect("an object");
    assert!(
        fields.values().all(|v| *v != Value::Bool(false)),
        "{}",
        report.doc
    );
}
