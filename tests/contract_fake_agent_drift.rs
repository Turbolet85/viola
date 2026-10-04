//! The fake agent's drift contract (test-plan §6 Contract suite, §7 Fake agent): for every recorded
//! `fixtures/claude/<version>/` set, a print-mode turn fires the spine hooks in the recorded order
//! and hands each hook exactly its recorded fixture's bytes. The recorded sets are walked at run
//! time, not by `#[files]`, and an empty walk fails.
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
