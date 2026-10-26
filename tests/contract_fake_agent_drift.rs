//! The fake agent's drift contract (test-plan §6 Contract suite, §7 Fake agent): for every recorded
//! `fixtures/claude/<version>/` set, a print-mode turn fires the spine hooks in the recorded order
//! and hands each hook exactly its recorded fixture's bytes; and every set recorded with its dialog
//! tier replays that tier, through `viola verify` against `--dialogs`, byte for byte and in the
//! recorded order, and so does every set recorded with its framing variants, against `--framing`,
//! and, where the status line is wrapped, its two status line variants; every set recorded with its
//! resume variant replays it under `--resume`, and every set recorded with its compact variant replays
//! it under `--framing` once resumed into the variant's own session; every set recorded with its agents read is answered
//! byte for byte by the fake agent's `agents --json` over that read, and the fake agent's own
//! registry row carries the read's key set and types. The recorded sets are walked at run time, not
//! by `#[files]`, and an empty walk fails.
//! andromeda:walks-tree — it reads every set under `fixtures/claude/`, named or not.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

/// Runs `each` over every set in `versions` at once, each on a scoped thread of its own, and
/// returns how many sets it compared (`each` returns false for a set it skips). Every set whose run
/// panicked is named by its version, with its panic message, in one panic.
fn each_set_at_once(versions: &[String], each: impl Fn(&str) -> bool + Sync) -> usize {
    let each = &each;
    let outcomes: Vec<(&str, std::thread::Result<bool>)> = std::thread::scope(|scope| {
        let running: Vec<_> = versions
            .iter()
            .map(|version| (version.as_str(), scope.spawn(move || each(version))))
            .collect();
        running
            .into_iter()
            .map(|(version, run)| (version, run.join()))
            .collect()
    });
    let failed: Vec<String> = outcomes
        .iter()
        .filter_map(|(version, outcome)| {
            let payload = outcome.as_ref().err()?;
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("a panic with no text");
            Some(format!("{version}: {message}"))
        })
        .collect();
    assert!(failed.is_empty(), "failing sets:\n{}", failed.join("\n"));
    outcomes
        .iter()
        .filter(|(_, outcome)| matches!(outcome, Ok(true)))
        .count()
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
    each_set_at_once(&versions, |version| {
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
        true
    });
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
    let compared = each_set_at_once(&recorded_versions(&root), |version| {
        if dialog_replay(&root.join(version)).is_empty() {
            return false;
        }
        // A set recorded before the framing variants replays its dialogs with the framing pastes
        // echoed: its local-command row fails, so verify exits 1 with the dialog tier still replayed.
        let framed = !framing_replay(&root.join(version)).is_empty();
        let (lines, ran) = verify_receipt(&root, version, framed);
        assert_eq!(
            ran.code,
            Some(if framed { 0 } else { 1 }),
            "{version}: verify exit\n{}",
            ran.stdout_text()
        );
        assert_eq!(
            dialog_drift(&root.join(version), &lines),
            Vec::<String>::new(),
            "{version}: dialog drift"
        );
        true
    });
    assert!(compared >= 2, "dialog sets compared: {compared}");
}

/// `viola verify` over the recorded `version` against the fake agent, with the framing replay or
/// without it: the fake agent's receipt and the run.
fn verify_receipt(root: &Path, version: &str, framed: bool) -> (Vec<Value>, support::verify::Ran) {
    let home = TestHome::new();
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = if framed {
        support::verify::verify(home.path(), root, version, &[], &after, &[])
    } else {
        support::verify::verify_without_framing(home.path(), root, version, &after)
    };
    (fake::receipt(&receipt), ran)
}

/// The framing variants in replay order, as test literals: the long and the tag-like prompts, then
/// the two hooks the local command fired.
const FRAMING_VARIANTS: [(&str, &str); 4] = [
    ("UserPromptSubmit", "paste-1"),
    ("UserPromptSubmit", "paste-2"),
    ("SessionEnd", "clear-1"),
    ("SessionStart", "clear-1"),
];

/// A set's recorded framing variants in replay order, each with its bytes; empty for a set
/// recorded before them.
fn framing_replay(set: &Path) -> Vec<(String, Vec<u8>)> {
    FRAMING_VARIANTS
        .iter()
        .filter_map(|(event, variant)| {
            let bytes = fs::read(set.join(format!("{event}.{variant}.json"))).ok()?;
            Some((format!("{event}.{variant}"), bytes))
        })
        .collect()
}

/// What drifted between a receipt's `hook` lines and the set's recorded framing replay: each
/// variant no hook of its event was offered byte for byte exactly once, then `order` when the
/// variants were not offered in the recorded order.
fn framing_drift(set: &Path, receipt: &[Value]) -> Vec<String> {
    let hooks = of_kind(receipt, "hook");
    let mut drifted = Vec::new();
    let mut at = Vec::new();
    for (name, bytes) in framing_replay(set) {
        let event = name.split('.').next().unwrap_or_default();
        let found: Vec<usize> = hooks
            .iter()
            .enumerate()
            .filter(|(_, h)| h["event"] == event)
            .filter(|(_, h)| h["stdin_hex"].as_str().map(unhex).as_ref() == Some(&bytes))
            .map(|(i, _)| i)
            .collect();
        match found.as_slice() {
            [one] => at.push(*one),
            _ => drifted.push(name),
        }
    }
    if !at.is_sorted() {
        drifted.push("order".to_owned());
    }
    drifted
}

/// Every set recorded with its framing variants replays them byte for byte: `viola verify` against
/// the fake agent's `--framing` hands the trusted run's hooks exactly the recorded bytes, in the
/// recorded order. A set without them is not compared.
#[test]
fn fake_agent_framing_replay_matches_every_recorded_framing_set() {
    let root = workspace_path("fixtures/claude");
    let compared = each_set_at_once(&recorded_versions(&root), |version| {
        let set = root.join(version);
        let recorded = framing_replay(&set);
        if recorded.is_empty() {
            return false;
        }
        assert_eq!(
            recorded.len(),
            FRAMING_VARIANTS.len(),
            "{version}: a partial framing set"
        );
        let (lines, ran) = verify_receipt(&root, version, true);
        assert_eq!(
            ran.code,
            Some(0),
            "{version}: verify exit\n{}",
            ran.stdout_text()
        );
        assert_eq!(
            framing_drift(&set, &lines),
            Vec::<String>::new(),
            "{version}: framing drift"
        );
        true
    });
    assert!(compared >= 1, "framing sets compared: {compared}");
}

/// A receipt as the synthetic framing set implies it, one `hook` line per variant in replay order.
fn receipt_of_framing(set: &Path) -> Vec<Value> {
    framing_replay(set)
        .iter()
        .map(|(name, bytes)| {
            let event = name.split('.').next().unwrap_or_default();
            json!({"v": 1, "kind": "hook", "event": event, "stdin_hex": hex(bytes)})
        })
        .collect()
}

#[test]
fn framing_drift_names_a_changed_byte_a_doubled_variant_and_a_swapped_order() {
    let home = TestHome::new();
    let fixtures = home.scratch().join("fixtures");
    support::verify::write_framing_set(&fixtures, "9.9.9");
    let set = fixtures.join("9.9.9");
    let clean = receipt_of_framing(&set);
    assert_eq!(framing_drift(&set, &clean), Vec::<String>::new());

    let mut changed = clean.clone();
    let mut bytes = unhex(changed[1]["stdin_hex"].as_str().expect("hex"));
    let last = bytes.len() - 1;
    bytes[last] ^= 0x01;
    changed[1]["stdin_hex"] = json!(hex(&bytes));
    assert_eq!(framing_drift(&set, &changed), ["UserPromptSubmit.paste-2"]);

    let mut doubled = clean.clone();
    doubled.push(clean[3].clone());
    assert_eq!(framing_drift(&set, &doubled), ["SessionStart.clear-1"]);

    let mut swapped = clean.clone();
    swapped.swap(2, 3);
    assert_eq!(framing_drift(&set, &swapped), ["order"]);

    let mut wrong_event = clean;
    wrong_event[0]["event"] = json!("Stop");
    assert_eq!(
        framing_drift(&set, &wrong_event),
        ["UserPromptSubmit.paste-1"]
    );
    assert!(framing_replay(&home.scratch().join("missing")).is_empty());
}

/// The bytes the fake agent, started with `--resume <id>` over `version`'s set and given an EOF
/// once its first hook is receipted, hands its first hook, and that hook's event.
fn resume_launch(root: &Path, version: &str, id: &str) -> (String, Vec<u8>) {
    let home = TestHome::new();
    let plugin = home.scratch().join("plugin");
    write_spine_plugin(&plugin);
    let receipt = fake::receipt_path(home.path(), "resume");
    let mut child = Command::new(FAKE)
        .args(["--cli-version", version, "--resume", id, "--fixtures"])
        .arg(root)
        .arg("--plugin-dir")
        .arg(&plugin)
        .arg("--receipt")
        .arg(&receipt)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("fake agent");
    let lines = fake::wait_for(&receipt, "the launch hook", |l| {
        !of_kind(l, "hook").is_empty()
    });
    drop(child.stdin.take());
    assert_eq!(child.wait().expect("exit").code(), Some(0), "{version}");
    let first = &of_kind(&lines, "hook")[0];
    (
        first["event"].as_str().unwrap_or_default().to_owned(),
        first["stdin_hex"].as_str().map(unhex).unwrap_or_default(),
    )
}

/// Every set recorded with its resume variant replays it byte for byte: the fake agent started with
/// `--resume` and the variant's own `session_id` hands its launch SessionStart hook exactly the
/// recorded bytes.
#[test]
fn fake_agent_resume_replay_matches_every_recorded_resume_set() {
    let root = workspace_path("fixtures/claude");
    let compared = each_set_at_once(&recorded_versions(&root), |version| {
        let Ok(recorded) = fs::read(root.join(version).join("SessionStart.resume-1.json")) else {
            return false;
        };
        let doc: Value = serde_json::from_slice(&recorded).expect("the resume variant's JSON");
        let id = doc["session_id"].as_str().expect("a recorded session id");
        let (event, offered) = resume_launch(&root, version, id);
        assert_eq!(event, "SessionStart", "{version}");
        assert!(offered == recorded, "{version}: resume drift");
        true
    });
    assert!(compared >= 1, "resume sets compared: {compared}");
}

/// The bytes the fake agent, started with `--framing --resume <id>` over `version`'s set and handed
/// one bracketed `/compact` paste once its launch hook is receipted, hands the hook it fires for
/// the paste, and that hook's event.
fn compact_replay(root: &Path, version: &str, id: &str) -> (String, Vec<u8>) {
    use std::io::Write as _;
    let home = TestHome::new();
    let plugin = home.scratch().join("plugin");
    write_spine_plugin(&plugin);
    let receipt = fake::receipt_path(home.path(), "compact");
    let mut child = Command::new(FAKE)
        .args(["--cli-version", version, "--framing", "--resume", id])
        .arg("--fixtures")
        .arg(root)
        .arg("--plugin-dir")
        .arg(&plugin)
        .arg("--receipt")
        .arg(&receipt)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("fake agent");
    fake::wait_for(&receipt, "the launch hook", |l| {
        !of_kind(l, "hook").is_empty()
    });
    let mut stdin = child.stdin.take().expect("stdin");
    stdin
        .write_all(b"\x1b[200~/compact\x1b[201~\r")
        .expect("the paste");
    let lines = fake::wait_for(&receipt, "the compact hook", |l| {
        of_kind(l, "hook").len() >= 2
    });
    drop(stdin);
    assert_eq!(child.wait().expect("exit").code(), Some(0), "{version}");
    let hooks = of_kind(&lines, "hook");
    assert_eq!(hooks.len(), 2, "{version}: one hook for the paste");
    (
        hooks[1]["event"].as_str().unwrap_or_default().to_owned(),
        hooks[1]["stdin_hex"]
            .as_str()
            .map(unhex)
            .unwrap_or_default(),
    )
}

/// Every set recorded with its compact variant replays it byte for byte: the fake agent resumed
/// into the variant's own `session_id` hands the SessionStart hook `/compact` fires exactly the
/// recorded bytes.
#[test]
fn fake_agent_compact_replay_matches_every_recorded_compact_set() {
    let root = workspace_path("fixtures/claude");
    let compared = each_set_at_once(&recorded_versions(&root), |version| {
        let Ok(recorded) = fs::read(root.join(version).join("SessionStart.compact-1.json")) else {
            return false;
        };
        let doc: Value = serde_json::from_slice(&recorded).expect("the compact variant's JSON");
        assert_eq!(doc["source"], "compact", "{version}");
        let id = doc["session_id"].as_str().expect("a recorded session id");
        let (event, offered) = compact_replay(&root, version, id);
        assert_eq!(event, "SessionStart", "{version}");
        assert!(offered == recorded, "{version}: compact drift");
        true
    });
    assert!(compared >= 1, "compact sets compared: {compared}");
}

/// The recorded agents read, as a test literal name.
const AGENTS_RECORDED: &str = "Agents.recorded-1.json";

/// A JSON value's type word, for comparing a row's shape without its values.
fn type_word(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(n) if n.is_u64() || n.is_i64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Each key of a row with its type word, in key order.
fn shape(row: &Value) -> Vec<(String, &'static str)> {
    let mut keys: Vec<(String, &'static str)> = row
        .as_object()
        .expect("a row object")
        .iter()
        .map(|(k, v)| (k.clone(), type_word(v)))
        .collect();
    keys.sort();
    keys
}

/// Every set recorded with its agents read: the fake agent planted beside a copy of that read
/// answers `agents --json` with exactly its bytes, and once started interactively over the set its
/// own registry row has the read's key set and types.
#[test]
fn fake_agent_agents_replay_matches_every_recorded_agents_set() {
    let root = workspace_path("fixtures/claude");
    // Every set's `claude` is planted before any set's thread starts. Where the plant is a copy
    // (no hard link across filesystems, as from the build dir to a tmpfs home), a child another
    // thread forks while the copy's write handle is open holds that handle until its own exec, and
    // the copy's exec fails with ETXTBSY.
    let planted: Vec<(String, TestHome, PathBuf, Vec<u8>)> = recorded_versions(&root)
        .into_iter()
        .filter_map(|version| {
            let recorded = fs::read(root.join(&version).join(AGENTS_RECORDED)).ok()?;
            let home = TestHome::new();
            let bin = fake::claude_bin(home.scratch(), "claude-bin", None, Some(&recorded));
            Some((version, home, bin, recorded))
        })
        .collect();
    let compared = each_set_at_once(&recorded_versions(&root), |version| {
        let Some((_, home, bin, recorded)) = planted.iter().find(|(v, ..)| v == version) else {
            return false;
        };
        let claude = fake::claude_in(bin);
        let answered = Command::new(&claude)
            .args(["agents", "--json"])
            .stdin(Stdio::null())
            .output()
            .expect("agents --json");
        assert_eq!(answered.status.code(), Some(0), "{version}");
        assert!(answered.stdout == *recorded, "{version}: agents drift");

        let doc: Value = serde_json::from_slice(recorded).expect("the agents read's JSON");
        let recorded_row = &doc[0];
        let plugin = home.scratch().join("plugin");
        write_spine_plugin(&plugin);
        let receipt = fake::receipt_path(home.path(), "agents");
        let mut child = Command::new(&claude)
            .args(["--cli-version", version, "--fixtures"])
            .arg(&root)
            .arg("--plugin-dir")
            .arg(&plugin)
            .arg("--receipt")
            .arg(&receipt)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("fake agent");
        fake::wait_for(&receipt, "the launch hook", |l| {
            !of_kind(l, "hook").is_empty()
        });
        let row_path = bin.join("sessions").join(format!("{}.json", child.id()));
        let row: Value =
            serde_json::from_slice(&fs::read(&row_path).expect("the registry row")).expect("json");
        assert_eq!(
            shape(&row),
            shape(recorded_row),
            "{version}: registry row shape"
        );
        let launch: Value = serde_json::from_slice(
            &fs::read(root.join(version).join("SessionStart.default.json")).expect("the launch"),
        )
        .expect("the launch's JSON");
        assert_eq!(
            row["cwd"], launch["cwd"],
            "{version}: the row replays the recorded cwd, never this host's directory"
        );
        drop(child.stdin.take());
        assert_eq!(child.wait().expect("exit").code(), Some(0), "{version}");
        assert!(!row_path.exists(), "{version}: the row outlived its agent");
        true
    });
    assert!(compared >= 1, "agents sets compared: {compared}");
}

/// The status line variants in replay order, as test literals: the launch's payload, then the first
/// turn's.
const STATUSLINE_VARIANTS: [&str; 2] =
    ["StatusLine.default-1.json", "StatusLine.rate-limits-1.json"];

/// A set's recorded status line variants in replay order, each with its bytes; empty for a set
/// recorded before them.
fn statusline_replay(set: &Path) -> Vec<(String, Vec<u8>)> {
    STATUSLINE_VARIANTS
        .iter()
        .filter_map(|name| Some(((*name).to_owned(), fs::read(set.join(name)).ok()?)))
        .collect()
}

/// What drifted between a receipt's `statusline` lines and the set's recorded replay: each variant
/// whose position was offered other bytes, then `count` when the lengths differ.
fn statusline_drift(set: &Path, receipt: &[Value]) -> Vec<String> {
    let expected = statusline_replay(set);
    let sent: Vec<Vec<u8>> = of_kind(receipt, "statusline")
        .iter()
        .map(|l| l["stdin_hex"].as_str().map(unhex).unwrap_or_default())
        .collect();
    let mut drifted: Vec<String> = expected
        .iter()
        .zip(&sent)
        .filter(|((_, bytes), got)| *got != bytes)
        .map(|((name, _), _)| name.clone())
        .collect();
    if expected.len() != sent.len() {
        drifted.push("count".to_owned());
    }
    drifted
}

/// Every set recorded with its status line variants replays them byte for byte where the status
/// line is wrapped: `viola verify` against the fake agent's `--framing` hands Run B's status line
/// command the launch's recorded payload, then the first turn's.
#[cfg(unix)]
#[test]
fn fake_agent_statusline_replay_matches_every_recorded_statusline_set() {
    let root = workspace_path("fixtures/claude");
    let compared = each_set_at_once(&recorded_versions(&root), |version| {
        let set = root.join(version);
        let recorded = statusline_replay(&set);
        if recorded.is_empty() {
            return false;
        }
        assert_eq!(
            recorded.len(),
            STATUSLINE_VARIANTS.len(),
            "{version}: a partial status line set"
        );
        let (lines, ran) = verify_receipt(&root, version, true);
        assert_eq!(
            ran.code,
            Some(0),
            "{version}: verify exit\n{}",
            ran.stdout_text()
        );
        assert_eq!(
            statusline_drift(&set, &lines),
            Vec::<String>::new(),
            "{version}: status line drift"
        );
        true
    });
    assert!(compared >= 1, "status line sets compared: {compared}");
}

#[test]
fn statusline_drift_names_a_changed_byte_a_swapped_order_and_a_short_replay() {
    let home = TestHome::new();
    let fixtures = home.scratch().join("fixtures");
    support::verify::write_framing_set(&fixtures, "9.9.9");
    let set = fixtures.join("9.9.9");
    let clean: Vec<Value> = statusline_replay(&set)
        .iter()
        .map(|(_, bytes)| json!({"v": 1, "kind": "statusline", "stdin_hex": hex(bytes)}))
        .collect();
    assert_eq!(clean.len(), 2);
    assert_eq!(statusline_drift(&set, &clean), Vec::<String>::new());

    let mut changed = clean.clone();
    let mut bytes = unhex(changed[1]["stdin_hex"].as_str().expect("hex"));
    let last = bytes.len() - 1;
    bytes[last] ^= 0x01;
    changed[1]["stdin_hex"] = json!(hex(&bytes));
    assert_eq!(
        statusline_drift(&set, &changed),
        ["StatusLine.rate-limits-1.json"]
    );

    let mut swapped = clean.clone();
    swapped.swap(0, 1);
    assert_eq!(
        statusline_drift(&set, &swapped),
        ["StatusLine.default-1.json", "StatusLine.rate-limits-1.json"]
    );

    assert_eq!(statusline_drift(&set, &clean[..1]), ["count"]);
    assert!(statusline_replay(&home.scratch().join("missing")).is_empty());
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
