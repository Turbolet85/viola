//! Critical Path 3, its cli + channel half (test-plan §6 Path 3; architecture [Message Broker /
//! IPC]): `viola wait` parks on the wrapper until the next driver-relevant line at or after its
//! cursor, returns an already-logged one at once, and times out typed; `viola last` reads the
//! newest turn, rebuilt across a wrapper restart. Plus the human lines, exit 21, the `cli` role's
//! one `error: internal error` line, and the wrapper's `-32602` for bad params. Fixtures are
//! synthetic, written per test.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::{Map, Value, json};
use support::cli::{Ran, spawn, viola};
use support::events::wait_events;
use support::fake;
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, snapshot_data, workspace_path};
use support::hygiene::load_schema;
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::watch::{WITHIN, Watch};
use viola_channel::Client;

const CANARY: &str = "canary-chain-value-5c1e";
const PATH3: &str = "fixtures/fake-scripts/path3.json";

/// `<scratch>/fixtures/<recorded version>/` with the synthetic SessionStart, UserPromptSubmit,
/// PostToolUse and a Stop carrying `message`.
fn fixtures(scratch: &Path, message: &str) -> PathBuf {
    let dir = scratch.join("fixtures");
    let bodies = [
        (
            "SessionStart",
            json!({"hook_event_name": "SessionStart", "session_id": "s-1", "source": "startup"}),
        ),
        (
            "UserPromptSubmit",
            json!({"hook_event_name": "UserPromptSubmit", "session_id": "s-1", "prompt": CANARY}),
        ),
        (
            "PostToolUse",
            json!({"hook_event_name": "PostToolUse", "session_id": "s-1", "tool_name": "Bash"}),
        ),
        (
            "Stop",
            json!({"hook_event_name": "Stop", "session_id": "s-1", "last_assistant_message": message}),
        ),
    ];
    for (event, body) in bodies {
        fake::write_fixture(&dir, fake::RECORDED_CLI_VERSION, event, "default", &body);
    }
    dir
}

/// A wrapper over the Path 3 script, its `n`th SessionStart record landed.
fn boot_path3(stamped: StampedHome, message: &str, starts: usize) -> Wrapper {
    let fx = fixtures(stamped.home.scratch(), message);
    let fx = fx.to_str().expect("utf-8 path").to_owned();
    let wrapper = Wrapper::boot(stamped, "builder", Some(PATH3), &["--fixtures", &fx]);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().filter(|e| e["kind"] == "session-start").count() >= starts
    });
    wrapper
}

fn fresh(message: &str) -> Wrapper {
    boot_path3(StampedHome::unstamped(TestHome::new()), message, 1)
}

/// Each complete `events.ndjson` line with the offset after its `\n`.
fn ends(instance_dir: &Path) -> Vec<(u64, Value)> {
    let bytes = std::fs::read(instance_dir.join("events.ndjson")).unwrap_or_default();
    let mut end = 0u64;
    let mut lines = Vec::new();
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        end += u64::try_from(line.len()).expect("len");
        if line.ends_with(b"\n") {
            lines.push((end, serde_json::from_slice(line).expect("one JSON object")));
        }
    }
    lines
}

fn end_offset(instance_dir: &Path) -> u64 {
    std::fs::metadata(instance_dir.join("events.ndjson")).map_or(0, |m| m.len())
}

/// The one line of `kind`, with its end offset.
fn only(instance_dir: &Path, kind: &str) -> (u64, Value) {
    let mut found: Vec<(u64, Value)> = ends(instance_dir)
        .into_iter()
        .filter(|(_, l)| l["kind"] == kind)
        .collect();
    assert_eq!(found.len(), 1, "one {kind} line");
    found.remove(0)
}

/// `--json`: exit 0, one document on stdout, nothing on stderr.
fn json_ok(ran: &Ran) -> Value {
    assert_eq!(ran.code, Some(0), "stderr: {}", ran.stderr);
    assert!(ran.stderr.is_empty(), "--json writes nothing on stderr");
    assert_eq!(ran.stdout.lines().count(), 1, "one JSON document");
    serde_json::from_str(&ran.stdout).expect("one JSON document")
}

fn keys(v: &Value) -> Vec<&str> {
    let mut keys: Vec<&str> = v
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    keys
}

fn diagnostics(home: &Path, file: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(file))
}

/// No home-level diagnostics line carries the canary, and every `run` and `cli` line passes the
/// diag-line schema (obs-plan §9 G4).
fn assert_logs_clean(home: &Path) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("role file");
        assert!(!text.contains(CANARY), "{path:?} holds content");
    }
    for file in ["run-builder.ndjson", "cli-builder.ndjson"] {
        for (n, line) in diagnostics(home, file).iter().enumerate() {
            assert!(validator.is_valid(line), "{file} line {n} fails the schema");
        }
    }
}

/// Waits until the wrapper logged a `channel-request` for `method`: the call is in, so a `wait`
/// is parked once it has scanned.
fn wait_request_logged(home: &Path, method: &str, count: usize) {
    let watch = Watch::start("request");
    let deadline = Instant::now() + WITHIN;
    loop {
        let n = diagnostics(home, "run-builder.ndjson")
            .iter()
            .filter(|l| l["event"] == "channel-request" && l["method"] == method)
            .count();
        if n >= count {
            return;
        }
        watch.note(&format!("{method} requests {n}"));
        watch.deadline_check(
            deadline,
            &format!("no {method} request reached the wrapper"),
        );
        std::thread::yield_now();
    }
}

/// verification-matrix v1-30: a `wait` parked across an `activity` line returns the `turn-ended`
/// with its line end as `cursor`, the same cursor again returns it at once, a timeout is typed,
/// and `last` reads `null` before the turn and the message after it.
#[test]
fn path3_wait_parks_until_turn_ended_then_last_reads_it() {
    let message = format!("{CANARY} 41 passed, 0 failed");
    let wrapper = fresh(&message);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let c = end_offset(&dir);
    let c_text = c.to_string();

    let before = json_ok(&viola(&home, &["last", "builder", "--json"], None, None));
    assert_eq!(
        before,
        json!({"v": 1, "ok": {"last_assistant_message": null, "ts": null}})
    );

    let mut parked = spawn(
        &home,
        &["wait", "builder", "--after", &c_text, "--json"],
        None,
        None,
    );
    wait_request_logged(&home, "wait", 1);
    wrapper.release();
    wait_events(&dir, "the activity line", |l| {
        l.iter().any(|e| e["kind"] == "activity")
    });
    assert!(parked.still_running(), "an activity line ended the wait");

    wrapper.release();
    let woken = json_ok(&parked.finish());
    let (turn_end, turn) = only(&dir, "turn-ended");
    assert_eq!(keys(&woken), ["ok", "v"]);
    assert_eq!(keys(&woken["ok"]), ["cursor", "event"]);
    assert_eq!(woken["v"], 1);
    assert_eq!(woken["ok"]["cursor"], turn_end);
    assert_eq!(woken["ok"]["event"], turn);
    assert_eq!(turn["data"]["last_assistant_message"], message.as_str());

    let again = json_ok(&viola(
        &home,
        &["wait", "builder", "--after", &c_text, "--json"],
        None,
        None,
    ));
    assert_eq!(again, woken, "a logged turn returns at once");

    let end = end_offset(&dir).to_string();
    let args = [
        "wait",
        "builder",
        "--after",
        &end,
        "--timeout-ms",
        "500",
        "--json",
    ];
    let timed_out = viola(&home, &args, None, None);
    assert_eq!(
        json_ok(&timed_out),
        json!({"v": 1, "ok": {"timed_out": true}})
    );
    assert_eq!(timed_out.stdout, "{\"v\":1,\"ok\":{\"timed_out\":true}}\n");

    let after = json_ok(&viola(&home, &["last", "builder", "--json"], None, None));
    assert_eq!(
        after,
        json!({"v": 1, "ok": {"last_assistant_message": message, "ts": turn["ts"]}})
    );

    // The woken call on both sides: its bounds on the request, the kind on the response.
    for file in ["cli-builder.ndjson", "run-builder.ndjson"] {
        let lines = diagnostics(&home, file);
        let request = lines
            .iter()
            .find(|l| l["event"] == "channel-request" && l["method"] == "wait")
            .unwrap_or_else(|| panic!("{file}: no wait request"));
        assert_eq!(request["after"], c, "{file}");
        let response = lines
            .iter()
            .find(|l| {
                l["event"] == "channel-response"
                    && l["conn"] == request["conn"]
                    && l["corr"] == request["corr"]
            })
            .unwrap_or_else(|| panic!("{file}: no joined wait response"));
        assert_eq!(response["outcome"], "turn-ended", "{file}");
    }
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A `send` cursor is a valid `after`: the turn the sent prompt starts is the one returned.
#[test]
fn wait_after_a_send_cursor_returns_the_turn() {
    let wrapper = fresh(CANARY);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let sent = spawn(&home, &["send", "builder", "--json"], Some(CANARY), None).finish();
    let cursor = json_ok(&sent)["ok"]["cursor"]
        .as_u64()
        .expect("cursor")
        .to_string();
    let parked = spawn(
        &home,
        &["wait", "builder", "--after", &cursor, "--json"],
        None,
        None,
    );
    wrapper.release();
    wrapper.release();
    let woken = json_ok(&parked.finish());
    let (turn_end, _) = only(&dir, "turn-ended");
    assert_eq!(woken["ok"]["event"]["kind"], "turn-ended");
    assert_eq!(woken["ok"]["cursor"], turn_end);
    wrapper.stop();
}

/// The newest turn is rebuilt from the log when the same name starts again.
#[test]
fn last_survives_a_wrapper_restart() {
    let message = format!("{CANARY} kept");
    let wrapper = fresh(&message);
    let dir = wrapper.instance_dir();
    wrapper.release();
    wrapper.release();
    wait_events(&dir, "the turn-ended line", |l| {
        l.iter().any(|e| e["kind"] == "turn-ended")
    });
    let before = json_ok(&viola(
        wrapper.home(),
        &["last", "builder", "--json"],
        None,
        None,
    ));
    assert_eq!(before["ok"]["last_assistant_message"], message.as_str());

    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    // Without the script: the control file still holds the two releases, and a scripted agent
    // would replay the turn.
    let fx = stamped.home.scratch().join("fixtures");
    let fx = fx.to_str().expect("utf-8 path").to_owned();
    let again = Wrapper::boot(stamped, "builder", None, &["--fixtures", &fx]);
    wait_events(
        &again.instance_dir(),
        "the second session-start record",
        |l| l.iter().filter(|e| e["kind"] == "session-start").count() >= 2,
    );
    assert_eq!(
        only(&again.instance_dir(), "turn-ended").1["ts"],
        before["ok"]["ts"]
    );
    let after = json_ok(&viola(
        again.home(),
        &["last", "builder", "--json"],
        None,
        None,
    ));
    assert_eq!(after, before);
    again.stop();
}

/// Human `last` shows each control as `\xHH` text and keeps `\n` and `\t`; `--json` stays
/// serde-escaped.
#[test]
fn last_human_escapes_controls() {
    let message = format!("{CANARY}\u{1b}[2J\u{7}\r\u{7f}\u{9b}31m\nnext\tcol");
    let wrapper = fresh(&message);
    wrapper.release();
    wrapper.release();
    wait_events(&wrapper.instance_dir(), "the turn-ended line", |l| {
        l.iter().any(|e| e["kind"] == "turn-ended")
    });
    let human = viola(wrapper.home(), &["last", "builder"], None, None);
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert!(
        human.stderr.is_empty(),
        "no context line when stderr is piped"
    );
    assert_eq!(
        human.stdout,
        format!("{CANARY}\\x1B[2J\\x07\\x0D\\x7F\\x9B31m\nnext\tcol\n")
    );
    let raw_control = human
        .stdout
        .chars()
        .any(|c| c.is_control() && c != '\n' && c != '\t');
    assert!(!raw_control, "a raw control character reached stdout");

    let json = viola(wrapper.home(), &["last", "builder", "--json"], None, None);
    assert_eq!(
        json_ok(&json)["ok"]["last_assistant_message"],
        message.as_str()
    );
    assert!(
        json.stdout.contains("\\u001b[2J\\u0007\\r"),
        "{}",
        json.stdout
    );
    assert!(!json.stdout.contains('\u{1b}'));
    wrapper.stop();
}

/// Under a terminal `waiting:` is printed once, then the result line; piped, the result alone and
/// no escape byte.
#[test]
fn wait_human_lines_are_linear() {
    let wrapper = fresh(CANARY);
    let home = wrapper.home().to_path_buf();
    let args: Vec<OsString> = vec![
        "--home".into(),
        home.clone().into(),
        "wait".into(),
        "builder".into(),
        "--timeout-ms".into(),
        "200".into(),
    ];
    let mut pty = OuterPty::spawn(Path::new(VIOLA), &args, &[]);
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let screen = String::from_utf8_lossy(&pty.finish()).into_owned();
    assert_eq!(screen.matches("waiting: builder").count(), 1, "{screen:?}");
    assert!(screen.contains("timed out  builder  200 ms"), "{screen:?}");

    let piped = viola(
        &home,
        &["wait", "builder", "--timeout-ms", "200"],
        None,
        None,
    );
    assert_eq!(piped.code, Some(0), "stderr: {}", piped.stderr);
    assert_eq!(piped.stdout, "timed out  builder  200 ms\n");
    assert!(piped.stderr.is_empty(), "{}", piped.stderr);
    assert!(!piped.stdout.contains('\u{1b}'));
    wrapper.stop();
}

#[test]
fn wait_last_unreachable_exit_21() {
    let home = TestHome::new();
    for verb in ["wait", "last"] {
        let human = viola(home.path(), &[verb, "builder"], None, None);
        assert_eq!(human.code, Some(21), "{verb}");
        assert!(human.stdout.is_empty(), "{verb}");
        assert_eq!(
            human.stderr,
            "unable  builder  instance-unreachable\n\
             hint: builder is not running; viola list shows the live instances\n",
            "{verb}"
        );
        let json = viola(home.path(), &[verb, "builder", "--json"], None, None);
        assert_eq!(json.code, Some(21), "{verb}");
        assert!(json.stderr.is_empty(), "{verb}");
        assert_eq!(
            json.stdout,
            "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n"
        );
    }
    let exits: Vec<Value> = diagnostics(home.path(), "cli-builder.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), 4);
    for exit in &exits {
        assert_eq!(exit["exit_code"], 21);
        assert_eq!(exit["detail"], "instance-dead");
        assert_eq!(exit["during"], "connect");
        assert_eq!(exit["level"], "WARN");
    }
}

/// A `cli` verb that fails prints exactly one `error: internal error` and no hint: a second
/// printer reads two lines, an unfiled verb none.
#[test]
fn cli_verbs_print_internal_error_once() {
    let home = TestHome::new();
    let scratch = home.scratch().join("home-is-a-file");
    std::fs::write(&scratch, CANARY).expect("a regular file");
    for (args, stdin) in [
        (&["send", "builder"][..], Some("hi")),
        (&["wait", "builder"][..], None),
        (&["last", "builder"][..], None),
    ] {
        let ran = spawn(&scratch, args, stdin, None).finish();
        assert_eq!(ran.code, Some(1), "{args:?}");
        assert!(ran.stdout.is_empty(), "{args:?}");
        assert_eq!(ran.stderr, "error: internal error\n", "{args:?}");
        assert_eq!(ran.stderr.lines().count(), 1, "{args:?}");
    }
}

/// A `cli` verb that fails inside a home that is a directory still prints the one line, and its
/// chain lands in the instance detail file alone: `diagnostics` is a regular file here, so the role
/// file cannot open (obs-plan §7 Per-role behaviour).
#[test]
fn wait_in_a_home_whose_diagnostics_is_a_file_keeps_the_chain_in_the_detail_file() {
    let home = TestHome::new();
    std::fs::create_dir(home.path()).expect("the home");
    std::fs::write(home.path().join("diagnostics"), b"x").expect("a file where the dir should be");
    let ran = viola(home.path(), &["wait", "builder"], None, None);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(ran.stderr, "error: internal error\n");
    let detail = home
        .path()
        .join("instances")
        .join("builder")
        .join("diagnostics")
        .join("detail-cli.ndjson");
    let detail = support::ndjson::read_lines(&detail);
    assert_eq!(detail.len(), 1);
    assert_eq!(detail[0]["event"], "process-exit");
    assert!(detail[0]["chain"].as_array().is_some_and(|c| !c.is_empty()));
}

/// The wrapper re-runs the bounds itself: each bad `after`, `timeout_ms` or `from` is `-32602` with
/// `data: null`, and an extra field is no fault.
#[test]
fn channel_wait_last_invalid_params() {
    let wrapper = fresh(CANARY);
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    let endpoint = snapshot["endpoint"].as_str().expect("endpoint").to_owned();
    let mut client = Client::connect(&endpoint, "cli").expect("connect");
    let bad = [
        ("wait", json!({"after": -1})),
        ("wait", json!({"after": "x"})),
        ("wait", json!({"timeout_ms": "x"})),
        ("wait", json!({"from": "Bad Name"})),
        ("last", json!({"from": 7})),
    ];
    for (method, params) in bad {
        let fields: Map<String, Value> = params.as_object().cloned().expect("object");
        let reply = client.request(method, fields).expect("reply");
        assert_eq!(reply["error"]["code"], -32602, "{method} {params}");
        assert_eq!(reply["error"]["data"], Value::Null, "{method} {params}");
    }
    let mut extra = Map::new();
    extra.insert("later_field".to_owned(), true.into());
    let reply = client.request("last", extra).expect("reply");
    assert_eq!(
        reply["result"]["ok"],
        json!({"last_assistant_message": null, "ts": null})
    );
    drop(client);
    wrapper.stop();
}
