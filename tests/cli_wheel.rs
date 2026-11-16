//! The wheel over the CLI and the wrapper channel (architecture [Human Takeover / Wheel]; test-plan
//! §6 Path 4 step 6 and Path 5's CLI half): `viola pause` takes the wheel for the human, so `send`
//! and `answer` are refused `human-typing` / `manual-pause` and a pending dialog is handed back to
//! the human at once; `viola release` returns it, and a `release` that carries `from` is a driver's,
//! refused `release-from-driver`. Every wrapper `send-refused` line carries the wheel.

#[allow(dead_code)]
mod support;

use std::path::Path;
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::viola;
use support::events::{events, wait_events};
use support::fake::{self, of_kind};
use support::home::{StampedHome, Wrapper, snapshot_data, stamped_home, workspace_path};
use support::hygiene::load_schema;
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";
const PATH4: &str = "fixtures/fake-scripts/path4.json";
const HINT: &str = "hint: the human has the wheel; send again after the human hands it back";

fn fixtures_arg() -> String {
    workspace_path("fixtures/claude")
        .to_str()
        .expect("utf-8 path")
        .to_owned()
}

/// A stamped wrapper over the committed hook fixtures (and `script`), its SessionStart landed.
fn boot(stamped: StampedHome, script: Option<&str>) -> Wrapper {
    support::events::boot(stamped, script, &[])
}

fn wheel_records(instance_dir: &Path) -> Vec<Value> {
    events(instance_dir)
        .into_iter()
        .filter(|l| l["kind"] == "wheel")
        .map(|l| {
            assert_eq!(l["source"], "wrapper");
            l["data"].clone()
        })
        .collect()
}

fn diagnostics(home: &Path, file: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(file))
}

/// No home-level line carries the canary, and every role line passes the diag-line schema.
fn assert_logs_clean(home: &Path) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("role file");
        assert!(!text.contains(CANARY), "{path:?} holds content");
        let file = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .into_owned();
        for (n, line) in diagnostics(home, &file).iter().enumerate() {
            assert!(validator.is_valid(line), "{file} line {n} fails the schema");
        }
    }
}

const REFUSED_PAUSED: &str = "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n";

/// test-plan §6 Path 5 steps 4–7 over the CLI: a pause refuses `send` and `answer`, `release`
/// returns the wheel, and a driver's `release` is refused.
#[rstest]
fn path5_pause_refuses_send_and_answer_and_release_returns_the_wheel(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let text = format!("{CANARY} pause me");

    let paused = viola(&home, &["pause", "builder", "--json"], Some(""), None);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    assert_eq!(paused.stdout, "{\"v\":1,\"ok\":{\"wheel\":\"human\"}}\n");
    assert!(paused.stderr.is_empty());

    let refused = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(refused.code, Some(10));
    assert_eq!(refused.stdout, REFUSED_PAUSED);
    assert!(refused.stderr.is_empty(), "--json carries no hint");
    let mirrored = viola(&home, &["send", "builder"], Some(&text), None);
    assert_eq!(mirrored.code, Some(10));
    assert!(mirrored.stdout.is_empty());
    assert_eq!(
        mirrored.stderr,
        format!("[/ ] unable         builder  human-typing  manual-pause\n{HINT}\n")
    );
    assert!(!mirrored.stderr.contains('\x1b'));

    let response = json!({"behavior": "deny", "message": CANARY}).to_string();
    let answered = viola(
        &home,
        &["answer", "builder", "1", "--json"],
        Some(&response),
        None,
    );
    assert_eq!(answered.code, Some(10));
    assert_eq!(answered.stdout, REFUSED_PAUSED);
    let human = viola(&home, &["answer", "builder", "1"], Some(&response), None);
    assert_eq!(human.code, Some(10));
    assert_eq!(
        human.stderr,
        format!("unable  builder  human-typing  manual-pause\n{HINT}\n")
    );

    let driver = viola(
        &home,
        &["release", "builder", "--json"],
        Some(""),
        Some("overseer"),
    );
    assert_eq!(driver.code, Some(20));
    assert_eq!(
        driver.stdout,
        "{\"v\":1,\"error\":\"wrapper-fault\",\"detail\":{\"code\":-32602,\
         \"message\":\"invalid params\",\"data\":{\"reason\":\"release-from-driver\"}}}\n"
    );
    let still = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(still.code, Some(10), "a driver's release moves nothing");

    let released = viola(&home, &["release", "builder", "--json"], Some(""), None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    assert_eq!(released.stdout, "{\"v\":1,\"ok\":{\"wheel\":\"driver\"}}\n");
    let sent = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);

    let paused = viola(&home, &["pause", "builder"], Some(""), None);
    assert_eq!(paused.code, Some(0));
    assert_eq!(
        paused.stdout,
        "builder  wheel human  manual-pause  I have control\n"
    );
    let released = viola(&home, &["release", "builder"], Some(""), None);
    assert_eq!(released.code, Some(0));
    assert_eq!(released.stdout, "builder  wheel driver  you have control\n");

    assert_eq!(
        wheel_records(&dir),
        [
            json!({"holder": "driver", "cause": "start"}),
            json!({"holder": "human", "cause": "manual-pause"}),
            json!({"holder": "driver", "cause": "release"}),
            json!({"holder": "human", "cause": "manual-pause"}),
            json!({"holder": "driver", "cause": "release"}),
        ]
    );
    assert_eq!(snapshot_data(&dir).expect("snapshot")["wheel"], "driver");

    let run = diagnostics(&home, "run-builder.ndjson");
    let from_driver: Vec<&Value> = run
        .iter()
        .filter(|l| l["event"] == "release-from-driver")
        .collect();
    assert_eq!(from_driver.len(), 1);
    assert_eq!(from_driver[0]["from"], "overseer");
    assert_eq!(from_driver[0]["from_trust"], "self-reported");
    assert!(from_driver[0]["corr"].is_u64() && from_driver[0]["conn"].is_string());
    let refused: Vec<&Value> = run
        .iter()
        .filter(|l| l["event"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 3, "{refused:?}");
    for line in refused {
        assert_eq!(line["side"], "wrapper");
        assert_eq!(line["wheel"], "human");
        assert_eq!(line["refusal"], "human-typing");
        assert_eq!(line["detail"], "manual-pause");
        assert!(line["corr"].is_u64() && line["rpc_id"].is_u64(), "{line}");
        assert!(line["conn"].is_string(), "{line}");
    }
    assert_logs_clean(&home);
    wrapper.stop();
}

const REFUSED_TURN: &str = "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"turn-running\"}\n";

/// A turn left running under a driver-held wheel (a harness turn with no Stop) refuses `send`
/// `turn-running`. A bare `release` changes nothing; `pause` then `release` returns the wheel and
/// clears the turn, so the next `send` is accepted.
#[rstest]
fn path5_a_turn_left_running_is_cleared_by_pause_then_release(stamped_home: StampedHome) {
    let fx = fixtures_arg();
    let wrapper = Wrapper::boot(
        stamped_home,
        "builder",
        None,
        &["--fixtures", &fx, "--inject-harness-turn"],
    );
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let text = format!("{CANARY} into a running turn");
    wrapper.release();
    wait_events(&dir, "the harness prompt-submitted", |l| {
        l.iter().any(|e| e["kind"] == "prompt-submitted")
    });

    let refused = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(refused.code, Some(13), "stderr: {}", refused.stderr);
    assert_eq!(refused.stdout, REFUSED_TURN);
    assert!(refused.stderr.is_empty(), "--json carries no hint");

    let released = viola(&home, &["release", "builder", "--json"], Some(""), None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    assert_eq!(released.stdout, "{\"v\":1,\"ok\":{\"wheel\":\"driver\"}}\n");
    let still = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(
        still.code,
        Some(13),
        "a release the wheel ignores clears nothing"
    );
    assert_eq!(still.stdout, REFUSED_TURN);

    let paused = viola(&home, &["pause", "builder"], Some(""), None);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    let released = viola(&home, &["release", "builder"], Some(""), None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    let sent = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);

    assert_eq!(
        wheel_records(&dir),
        [
            json!({"holder": "driver", "cause": "start"}),
            json!({"holder": "human", "cause": "manual-pause"}),
            json!({"holder": "driver", "cause": "release"}),
        ]
    );
    let refused: Vec<Value> = diagnostics(&home, "run-builder.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 2, "{refused:?}");
    for line in &refused {
        assert_eq!(line["side"], "wrapper");
        assert_eq!(line["wheel"], "driver");
        assert_eq!(line["refusal"], "not-delivered");
        assert_eq!(line["detail"], "turn-running");
        assert!(line["corr"].is_u64() && line["rpc_id"].is_u64(), "{line}");
        assert!(line["conn"].is_string(), "{line}");
    }
    assert_logs_clean(&home);
    wrapper.stop();
}

/// Waits until the wrapper logged `count` `channel-request` lines for `method`: the call is in.
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

/// A turn the CLI ends on an API error (a StopFailure, no Stop) ends the running turn: the stuck
/// `turn-running` state clears with no `pause` and no `release`, a parked `wait` returns the
/// `turn-ended` carrying only the closed cause, and the next `send` is accepted.
#[rstest]
fn path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release(stamped_home: StampedHome) {
    let scratch = stamped_home.home.scratch().to_path_buf();
    // The recorded set (its screens draw the input box a `send` waits for), plus a synthetic
    // StopFailure: no recorded set holds one.
    let fixtures = scratch.join("fixtures");
    let set = fixtures.join(fake::RECORDED_CLI_VERSION);
    std::fs::create_dir_all(&set).expect("fixture dir");
    let recorded = workspace_path("fixtures/claude").join(fake::RECORDED_CLI_VERSION);
    for entry in std::fs::read_dir(&recorded).expect("the recorded set") {
        let path = entry.expect("entry").path();
        std::fs::copy(&path, set.join(path.file_name().expect("name"))).expect("copy");
    }
    fake::write_fixture(
        &fixtures,
        fake::RECORDED_CLI_VERSION,
        "StopFailure",
        "default",
        &json!({"hook_event_name": "StopFailure", "session_id": "s-1", "error": "server_error",
            "last_assistant_message": "API Error: Connection lost mid-response"}),
    );
    let script = scratch.join("api-error.script.json");
    std::fs::write(
        &script,
        json!({"v": 1, "steps": [{"event": "StopFailure", "variant": "default", "gate": true}]})
            .to_string(),
    )
    .expect("script");
    let wrapper = Wrapper::boot(
        stamped_home,
        "builder",
        None,
        &[
            "--fixtures",
            fixtures.to_str().expect("utf-8 path"),
            "--inject-harness-turn",
            "--script",
            script.to_str().expect("utf-8 path"),
        ],
    );
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let text = format!("{CANARY} after an API error");
    wrapper.release();
    wait_events(&dir, "the harness prompt-submitted", |l| {
        l.iter().any(|e| e["kind"] == "prompt-submitted")
    });

    let refused = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(refused.code, Some(13), "stderr: {}", refused.stderr);
    assert_eq!(refused.stdout, REFUSED_TURN);

    let after = std::fs::metadata(dir.join("events.ndjson"))
        .expect("events")
        .len()
        .to_string();
    let mut parked = support::cli::spawn(
        &home,
        &["wait", "builder", "--after", &after, "--json"],
        None,
        None,
    );
    wait_request_logged(&home, "wait", 1);
    assert!(parked.still_running(), "the wait returned before the end");
    wrapper.release();
    let woken = parked.finish();
    assert_eq!(woken.code, Some(0), "stderr: {}", woken.stderr);
    let woken: Value = serde_json::from_str(&woken.stdout).expect("one document");
    let event = &woken["ok"]["event"];
    assert_eq!(event["kind"], "turn-ended");
    assert_eq!(event["source"], "hook");
    assert_eq!(
        event["data"],
        json!({"last_assistant_message": null, "background_tasks": null, "scheduled_wakes": null, "cause": "server-error"})
    );

    let sent = viola(&home, &["send", "builder", "--json"], Some(&text), None);
    assert_eq!(sent.code, Some(0), "{} {}", sent.stdout, sent.stderr);

    assert_eq!(
        wheel_records(&dir),
        [json!({"holder": "driver", "cause": "start"})]
    );
    let refused: Vec<Value> = diagnostics(&home, "run-builder.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 1, "{refused:?}");
    assert_eq!(refused[0]["detail"], "turn-running");
    assert_logs_clean(&home);
    wrapper.stop();
}

/// test-plan §6 Path 4 step 6: a dialog pending when the human takes the wheel is handed back at
/// once (its hook prints nothing and exits 0, well before the deadline), `pending_dialog` leaves
/// the snapshot, and an `answer` to it is refused `human-typing`.
#[rstest]
fn path4_a_dialog_pending_at_a_pause_is_handed_back_to_the_human(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH4));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();
    wrapper.release();
    let lines = wait_events(&dir, "the question", |l| {
        l.iter().any(|e| e["kind"] == "question")
    });
    let id = lines
        .iter()
        .find(|e| e["kind"] == "question")
        .and_then(|e| e["data"]["dialog_id"].as_u64())
        .expect("a dialog_id");
    let watch = Watch::start("pending");
    let deadline = Instant::now() + WITHIN;
    while snapshot_data(&dir).is_none_or(|d| d["pending_dialog"]["dialog_id"] != id) {
        watch.note("not pending yet");
        watch.deadline_check(deadline, "the question never went pending");
        std::thread::yield_now();
    }
    let paused_at = Instant::now();
    let paused = viola(&home, &["pause", "builder", "--json"], Some(""), None);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    let hooks = fake::wait_for(&receipt, "the question's hook", |l| {
        of_kind(l, "hook").len() >= 2
    });
    assert!(paused_at.elapsed() < WITHIN);
    let hook = of_kind(&hooks, "hook")[1].clone();
    assert_eq!(hook["event"], "PreToolUse");
    assert_eq!(hook["ran"], true, "{hook}");
    assert_eq!(hook["exit_code"], 0, "{hook}");
    assert_eq!(hook["stderr_len"], 0, "{hook}");
    assert_eq!(hook["stdout_hex"], "", "no decision body: {hook}");
    let data = snapshot_data(&dir).expect("snapshot");
    assert!(data.get("pending_dialog").is_none(), "{data}");
    assert_eq!(data["wheel"], "human");
    let response = json!({"answers": {"q": CANARY}}).to_string();
    let answer = viola(
        &home,
        &["answer", "builder", &id.to_string(), "--json"],
        Some(&response),
        None,
    );
    assert_eq!(answer.code, Some(10));
    assert_eq!(answer.stdout, REFUSED_PAUSED);
    let lines = events(&dir);
    assert_eq!(
        lines.iter().filter(|e| e["kind"] == "question").count(),
        1,
        "the dialog is logged once"
    );
    assert_logs_clean(&home);
    wrapper.stop();
}
