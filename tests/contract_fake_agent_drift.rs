//! The fake agent's drift contract (test-plan §6 Contract suite, §7 Fake agent): for every recorded
//! `fixtures/claude/<version>/` set, a print-mode turn fires the spine hooks in the recorded order
//! and hands each hook exactly its recorded fixture's bytes; and every set recorded with its dialog
//! tier replays that tier, through `viola verify` against `--dialogs`, byte for byte and in the
//! recorded order. The recorded sets are walked at run time, not by `#[files]`, and an empty walk
//! fails.
//! andromeda:walks-tree — it reads every set under `fixtures/claude/`, named or not.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};
use support::fake::{self, FAKE, of_kind, unhex};
use support::home::{TestHome, workspace_path};

/// The spine order `viola verify`'s `spine-hooks` row measured, as a test literal.
const SPINE: [&str; 4] = ["SessionStart", "UserPromptSubmit", "Stop", "SessionEnd"];

/// The recorded CLI versions under `root`, sorted: one dir per version.
fn recorded_versions(root: &Path) -> Vec<String> {
    let mut found: Vec<String> = fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_owned))
        .collect();
    found.sort();
    found
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// What drifted between a print-mode receipt and the recorded set: `order` when the `hook` events
/// are not the spine, then each event whose stdin bytes are not its recorded fixture's.
fn drift(fixtures: &Path, version: &str, receipt: &[Value]) -> Vec<String> {
    let hooks = of_kind(receipt, "hook");
    let events: Vec<&str> = hooks.iter().filter_map(|h| h["event"].as_str()).collect();
    let mut drifted = Vec::new();
    if events != SPINE {
        drifted.push("order".to_owned());
    }
    for hook in hooks {
        let event = hook["event"].as_str().unwrap_or_default();
        let recorded = fs::read(fixtures.join(version).join(format!("{event}.default.json"))).ok();
        let sent = hook["stdin_hex"].as_str().map(unhex);
        if recorded.is_none() || sent != recorded {
            drifted.push(event.to_owned());
        }
    }
    drifted
}

/// `<dir>/hooks/hooks.json` registering `FAKE --version` (it exits 0) for each spine event.
fn write_spine_plugin(dir: &Path) {
    let hooks = dir.join("hooks");
    fs::create_dir_all(&hooks).expect("hooks dir");
    let mut registered = serde_json::Map::new();
    for event in SPINE {
        registered.insert(
            event.to_owned(),
            json!([{"hooks": [{"type": "command", "command": FAKE, "args": ["--version"]}]}]),
        );
    }
    fs::write(
        hooks.join("hooks.json"),
        json!({"hooks": registered}).to_string(),
    )
    .expect("hooks.json");
}

/// A receipt as the recorded set implies it: one `hook` line per event, in `order`, each carrying
/// that event's recorded fixture bytes.
fn receipt_of(fixtures: &Path, version: &str, order: &[&str]) -> Vec<Value> {
    order
        .iter()
        .map(|event| {
            let bytes = fs::read(fixtures.join(version).join(format!("{event}.default.json")))
                .expect("recorded fixture");
            json!({"v": 1, "kind": "hook", "event": event, "command_absolute": true, "ran": true,
                   "exit_code": 0, "stderr_len": 0, "stdout_hex": "", "stdin_hex": hex(&bytes)})
        })
        .collect()
}

fn first_recorded_version(root: &Path) -> String {
    recorded_versions(root)
        .into_iter()
        .next()
        .expect("no recorded fixture set")
}

#[test]
fn fake_agent_print_turn_matches_every_recorded_set() {
    let root = workspace_path("fixtures/claude");
    let versions = recorded_versions(&root);
    assert!(!versions.is_empty(), "no recorded fixture set to compare");
    for version in &versions {
        let home = TestHome::new();
        let plugin = home.scratch().join("plugin");
        write_spine_plugin(&plugin);
        let recorded: Value = serde_json::from_slice(
            &fs::read(root.join(version).join("UserPromptSubmit.default.json"))
                .expect("recorded UserPromptSubmit"),
        )
        .expect("recorded UserPromptSubmit JSON");
        let prompt = recorded["prompt"].as_str().expect("recorded prompt");
        let receipt = fake::receipt_path(home.path(), "drift");
        let out = Command::new(FAKE)
            .arg("-p")
            .arg(prompt)
            .arg("--cli-version")
            .arg(version)
            .arg("--fixtures")
            .arg(&root)
            .arg("--plugin-dir")
            .arg(&plugin)
            .arg("--receipt")
            .arg(&receipt)
            .output()
            .expect("fake agent");
        assert_eq!(out.status.code(), Some(0), "{version}: fake agent exit");
        let lines = fake::receipt(&receipt);
        let hooks = of_kind(&lines, "hook");
        let events: Vec<&Value> = hooks.iter().map(|h| &h["event"]).collect();
        assert_eq!(
            events,
            [
                &json!("SessionStart"),
                &json!("UserPromptSubmit"),
                &json!("Stop"),
                &json!("SessionEnd")
            ],
            "{version}: hook order"
        );
        for hook in &hooks {
            assert_eq!(hook["ran"], true, "{version}: {} ran", hook["event"]);
            assert_eq!(hook["exit_code"], 0, "{version}: {} exit", hook["event"]);
        }
        assert_eq!(
            drift(&root, version, &lines),
            Vec::<String>::new(),
            "{version}: drift"
        );
    }
}

/// The dialog-tier hook events, as test literals.
const DIALOG_EVENTS: [&str; 3] = ["PreToolUse", "PermissionRequest", "PostToolUse"];
/// The dialog prompts' variant stems in paste order, as test literals.
const DIALOG_STEMS: [&str; 4] = ["questions", "parallel", "permission", "plan"];

/// A set's recorded dialog variants in replay order: per stem, per call `n`, its PreToolUse,
/// PermissionRequest and PostToolUse fixtures as recorded; a call with neither of the first two
/// ends its stem.
fn dialog_replay(set: &Path) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    for stem in DIALOG_STEMS {
        for n in 1.. {
            let read = |event: &str| fs::read(set.join(format!("{event}.{stem}-{n}.json"))).ok();
            if read("PreToolUse").is_none() && read("PermissionRequest").is_none() {
                break;
            }
            for event in DIALOG_EVENTS {
                if let Some(bytes) = read(event) {
                    out.push((format!("{event}.{stem}-{n}"), bytes));
                }
            }
        }
    }
    out
}

/// What drifted between a receipt's dialog-tier `hook` lines and the set's recorded replay: each
/// position whose event or stdin bytes differ, then `count` when the lengths differ.
fn dialog_drift(set: &Path, receipt: &[Value]) -> Vec<String> {
    let expected = dialog_replay(set);
    let sent: Vec<(String, Vec<u8>)> = of_kind(receipt, "hook")
        .iter()
        .filter(|h| DIALOG_EVENTS.iter().any(|e| h["event"] == *e))
        .map(|h| {
            let event = h["event"].as_str().unwrap_or_default().to_owned();
            (
                event,
                h["stdin_hex"].as_str().map(unhex).unwrap_or_default(),
            )
        })
        .collect();
    let mut drifted: Vec<String> = expected
        .iter()
        .zip(&sent)
        .filter(|((name, bytes), (event, got))| {
            !name.starts_with(&format!("{event}.")) || got != bytes
        })
        .map(|((name, _), _)| name.clone())
        .collect();
    if expected.len() != sent.len() {
        drifted.push("count".to_owned());
    }
    drifted
}

/// Every set recorded with its dialog tier replays it byte for byte: `viola verify` against the
/// fake agent's `--dialogs` hands each dialog hook exactly its recorded variant's bytes, in the
/// recorded order. A set without dialog variants (the drift-only spine set) is not compared.
#[test]
fn fake_agent_dialog_replay_matches_every_recorded_dialog_set() {
    let root = workspace_path("fixtures/claude");
    let mut compared = 0;
    for version in recorded_versions(&root) {
        if dialog_replay(&root.join(&version)).is_empty() {
            continue;
        }
        let home = TestHome::new();
        let receipt = home.scratch().join("receipt.ndjson");
        let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
        let ran = support::verify::verify(
            home.path(),
            &root,
            &version,
            &[],
            &["--receipt", &receipt_arg],
            &[],
        );
        assert_eq!(
            ran.code,
            Some(0),
            "{version}: verify exit\n{}",
            ran.stdout_text()
        );
        let lines = fake::receipt(&receipt);
        assert_eq!(
            dialog_drift(&root.join(&version), &lines),
            Vec::<String>::new(),
            "{version}: dialog drift"
        );
        compared += 1;
    }
    assert!(compared >= 2, "dialog sets compared: {compared}");
}

/// The synthetic dialog set in a scratch fixture dir.
fn synthetic_dialog_set(home: &TestHome) -> std::path::PathBuf {
    let fixtures = home.scratch().join("fixtures");
    support::verify::write_dialog_set(&fixtures, "9.9.9");
    fixtures.join("9.9.9")
}

fn receipt_of_replay(set: &Path) -> Vec<Value> {
    dialog_replay(set)
        .iter()
        .map(|(name, bytes)| {
            let event = name.split('.').next().unwrap_or_default();
            json!({"v": 1, "kind": "hook", "event": event, "stdin_hex": hex(bytes)})
        })
        .collect()
}

#[test]
fn dialog_replay_reads_each_call_s_events_in_order() {
    let home = TestHome::new();
    let set = synthetic_dialog_set(&home);
    let names: Vec<String> = dialog_replay(&set).into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        names,
        [
            "PreToolUse.questions-1",
            "PostToolUse.questions-1",
            "PreToolUse.parallel-1",
            "PostToolUse.parallel-1",
            "PreToolUse.parallel-2",
            "PostToolUse.parallel-2",
            "PermissionRequest.permission-1",
            "PostToolUse.permission-1",
            "PreToolUse.plan-1",
            "PermissionRequest.plan-1",
            "PreToolUse.plan-2",
            "PostToolUse.plan-2",
        ]
    );
    assert_eq!(
        dialog_drift(&set, &receipt_of_replay(&set)),
        Vec::<String>::new()
    );
}

#[test]
fn dialog_drift_names_a_changed_byte_a_wrong_event_and_a_short_replay() {
    let home = TestHome::new();
    let set = synthetic_dialog_set(&home);
    let mut receipt = receipt_of_replay(&set);
    let mut bytes = unhex(receipt[3]["stdin_hex"].as_str().expect("hex"));
    let last = bytes.len() - 1;
    bytes[last] ^= 0x01;
    receipt[3]["stdin_hex"] = json!(hex(&bytes));
    receipt[6]["event"] = json!("PreToolUse");
    assert_eq!(
        dialog_drift(&set, &receipt),
        ["PostToolUse.parallel-1", "PermissionRequest.permission-1"]
    );
    receipt.truncate(5);
    assert!(dialog_drift(&set, &receipt).contains(&"count".to_owned()));
}

#[test]
fn drift_is_empty_for_the_recorded_set_in_spine_order() {
    let root = workspace_path("fixtures/claude");
    let version = first_recorded_version(&root);
    let receipt = receipt_of(&root, &version, &SPINE);
    assert_eq!(drift(&root, &version, &receipt), Vec::<String>::new());
}

#[test]
fn drift_names_a_swapped_order() {
    let root = workspace_path("fixtures/claude");
    let version = first_recorded_version(&root);
    let receipt = receipt_of(
        &root,
        &version,
        &["UserPromptSubmit", "SessionStart", "Stop", "SessionEnd"],
    );
    assert_eq!(drift(&root, &version, &receipt), ["order"]);
}

#[test]
fn drift_names_the_event_whose_fixture_differs_by_one_byte() {
    let root = workspace_path("fixtures/claude");
    let version = first_recorded_version(&root);
    let receipt = receipt_of(&root, &version, &SPINE);
    let home = TestHome::new();
    let planted = home.scratch().join("fixtures");
    fs::create_dir_all(planted.join(&version)).expect("planted dir");
    for event in SPINE {
        let name = format!("{event}.default.json");
        let mut bytes = fs::read(root.join(&version).join(&name)).expect("recorded fixture");
        if event == "Stop" {
            let last = bytes.len() - 1;
            bytes[last] ^= 0x01;
        }
        fs::write(planted.join(&version).join(&name), bytes).expect("planted fixture");
    }
    assert_eq!(drift(&planted, &version, &receipt), ["Stop"]);
}
