//! The fixture scrub-and-schema walk (test-plan §7 Fixture hygiene): every committed scenario
//! script, every recorded `fixtures/claude/*/*.json` hook payload, every recorded `StatusLine.*.json`
//! status line payload, every recorded `Agents.*.json` agents read, every recorded `Mcp.*.json`
//! MCP exchange and every recorded `Screen.*.json` is scrubbed and schema-valid, each against
//! its own schema, and every rejection arm is proven on a planted input. The recorded set is walked at run time, not
//! by `#[files]`, which refuses to compile over a glob that matches nothing.
//! andromeda:walks-tree — it reads every file under `fixtures/`, named or not.

#[allow(dead_code)]
mod support;

use std::path::{Path, PathBuf};

use rstest::rstest;
use serde_json::{Value, json};
use support::home::workspace_path;
use support::hygiene::{
    PLACEHOLDER_USER, Violation, check, has_absolute_path, has_username, host_user, load_schema,
};

fn script_schema() -> Value {
    load_schema(&workspace_path("schemas/fake-script.v1.json"))
}

fn claude_schema() -> Value {
    load_schema(&workspace_path("schemas/claude-fixture.v1.json"))
}

fn screen_schema() -> Value {
    load_schema(&workspace_path("schemas/claude-screen.v1.json"))
}

fn statusline_schema() -> Value {
    load_schema(&workspace_path("schemas/claude-statusline-fixture.v1.json"))
}

fn agents_schema() -> Value {
    load_schema(&workspace_path("schemas/claude-agents-fixture.v1.json"))
}

fn mcp_schema() -> Value {
    load_schema(&workspace_path("schemas/claude-mcp-fixture.v1.json"))
}

fn named(path: &Path, prefix: &str) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with(prefix))
}

fn is_screen(path: &Path) -> bool {
    named(path, "Screen.")
}

fn is_statusline(path: &Path) -> bool {
    named(path, "StatusLine.")
}

fn is_agents(path: &Path) -> bool {
    named(path, "Agents.")
}

fn is_mcp(path: &Path) -> bool {
    named(path, "Mcp.")
}

/// `<root>/*/*.json` but the screens, the status line payloads, the agents reads and the MCP
/// exchanges, sorted: the recorded hook payloads.
fn claude_fixtures(root: &Path) -> Vec<PathBuf> {
    recorded(root)
        .into_iter()
        .filter(|p| !is_screen(p) && !is_statusline(p) && !is_agents(p) && !is_mcp(p))
        .collect()
}

/// `<root>/*/Mcp.*.json`, sorted: the recorded MCP exchanges.
fn mcp_fixtures(root: &Path) -> Vec<PathBuf> {
    recorded(root).into_iter().filter(|p| is_mcp(p)).collect()
}

/// `<root>/*/Agents.*.json`, sorted: the recorded agents reads.
fn agents_fixtures(root: &Path) -> Vec<PathBuf> {
    recorded(root)
        .into_iter()
        .filter(|p| is_agents(p))
        .collect()
}

/// `<root>/*/StatusLine.*.json`, sorted: the recorded status line payloads.
fn statusline_fixtures(root: &Path) -> Vec<PathBuf> {
    recorded(root)
        .into_iter()
        .filter(|p| is_statusline(p))
        .collect()
}

/// `<root>/*/Screen.*.json`, sorted: the recorded screens.
fn screen_fixtures(root: &Path) -> Vec<PathBuf> {
    recorded(root)
        .into_iter()
        .filter(|p| is_screen(p))
        .collect()
}

/// `<root>/*/*.json`, sorted: the recorded set, one dir per CLI version.
fn recorded(root: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|version| version.path().is_dir())
        .flat_map(|version| {
            std::fs::read_dir(version.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "json") && path.is_file())
        .collect();
    found.sort();
    found
}

#[test]
fn claude_fixtures_pass_hygiene() {
    let schema = claude_schema();
    let user = host_user();
    for path in claude_fixtures(&workspace_path("fixtures/claude")) {
        let bytes = std::fs::read(&path).expect("fixture");
        assert_eq!(
            check(&bytes, &schema, user.as_deref()),
            Ok(()),
            "{}",
            path.display()
        );
    }
}

#[test]
fn statusline_fixtures_pass_hygiene() {
    let schema = statusline_schema();
    let user = host_user();
    let walked = statusline_fixtures(&workspace_path("fixtures/claude"));
    assert!(!walked.is_empty(), "no status line payload walked");
    for path in walked {
        let bytes = std::fs::read(&path).expect("fixture");
        assert_eq!(
            check(&bytes, &schema, user.as_deref()),
            Ok(()),
            "{}",
            path.display()
        );
    }
}

#[test]
fn agents_fixtures_pass_hygiene() {
    let schema = agents_schema();
    let user = host_user();
    let walked = agents_fixtures(&workspace_path("fixtures/claude"));
    assert!(!walked.is_empty(), "no agents read walked");
    for path in walked {
        let bytes = std::fs::read(&path).expect("fixture");
        assert_eq!(
            check(&bytes, &schema, user.as_deref()),
            Ok(()),
            "{}",
            path.display()
        );
    }
}

fn agents_row(extra: &Value) -> Value {
    let mut row = json!({
        "cwd": "~/work/.viola-verify-7",
        "kind": "interactive",
        "name": "viola verify probe",
        "pid": 4242,
        "sessionId": "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f",
        "startedAt": 1_760_000_000_000_i64,
        "status": "idle",
    });
    if let (Some(row), Some(extra)) = (row.as_object_mut(), extra.as_object()) {
        row.extend(extra.clone());
    }
    row
}

#[rstest]
#[case::measured(json!({}))]
#[case::an_extra_key(json!({"later": 1}))]
fn planted_clean_agents_read_passes(#[case] extra: Value) {
    let doc = json!([agents_row(&extra)]);
    assert_eq!(
        check(
            doc.to_string().as_bytes(),
            &agents_schema(),
            Some("plantuser")
        ),
        Ok(())
    );
}

#[rstest]
#[case::not_an_array(agents_row(&json!({})))]
#[case::empty(json!([]))]
#[case::a_row_not_object(json!(["row"]))]
#[case::no_session_id(json!([{"pid": 1, "status": "idle"}]))]
#[case::pid_a_string(json!([agents_row(&json!({"pid": "4242"}))]))]
#[case::started_at_a_string(json!([agents_row(&json!({"startedAt": "1760000000000"}))]))]
#[case::status_a_number(json!([agents_row(&json!({"status": 1}))]))]
fn planted_agents_schema_violation_is_rejected(#[case] doc: Value) {
    assert_eq!(
        check(doc.to_string().as_bytes(), &agents_schema(), None),
        Err(Violation::Schema)
    );
}

#[rstest]
#[case::linux_home(json!({"cwd": "/home/plantuser/work"}))]
#[case::a_windows_profile(json!({"cwd": "C:\\Users\\plantuser\\w"}))]
fn planted_agents_absolute_path_is_rejected(#[case] extra: Value) {
    let doc = json!([agents_row(&extra)]);
    assert_eq!(
        check(doc.to_string().as_bytes(), &agents_schema(), None),
        Err(Violation::AbsolutePath)
    );
}

#[test]
fn planted_agents_username_is_rejected() {
    let doc = json!([agents_row(&json!({"name": "plantuser's session"}))]);
    assert_eq!(
        check(
            doc.to_string().as_bytes(),
            &agents_schema(),
            Some("plantuser")
        ),
        Err(Violation::Username)
    );
}

/// Every set recorded with a status line payload holds the two variants, each walked by
/// `statusline_fixtures_pass_hygiene`.
#[test]
fn statusline_variants_are_walked_for_every_statusline_set() {
    let walked = statusline_fixtures(&workspace_path("fixtures/claude"));
    let mut sets: Vec<&Path> = walked.iter().filter_map(|p| p.parent()).collect();
    sets.dedup();
    for set in sets {
        let names: Vec<String> = walked
            .iter()
            .filter(|p| p.parent() == Some(set))
            .filter_map(|p| p.file_name()?.to_str().map(str::to_owned))
            .collect();
        assert_eq!(
            names,
            ["StatusLine.default-1.json", "StatusLine.rate-limits-1.json"],
            "{}",
            set.display()
        );
    }
}

fn statusline_payload(extra: &Value) -> Vec<u8> {
    let mut doc = json!({"session_id": "s-1", "cwd": "~/work", "model": {"id": "m"}});
    if let (Some(doc), Some(extra)) = (doc.as_object_mut(), extra.as_object()) {
        doc.extend(extra.clone());
    }
    doc.to_string().into_bytes()
}

#[rstest]
#[case::no_rate_limits(json!({}))]
#[case::rate_limits(json!({"rate_limits": {
    "five_hour": {"used_percentage": 23, "resets_at": 1_760_400_000},
    "seven_day": {"used_percentage": 91, "resets_at": 1_760_900_000},
}}))]
fn planted_clean_statusline_payload_passes(#[case] extra: Value) {
    assert_eq!(
        check(
            &statusline_payload(&extra),
            &statusline_schema(),
            Some("plantuser")
        ),
        Ok(())
    );
}

#[rstest]
#[case::no_session_id(json!({"model": {"id": "m"}}))]
#[case::session_id_not_string(json!({"session_id": 1}))]
#[case::rate_limits_not_object(json!({"session_id": "s", "rate_limits": 91}))]
#[case::a_window_missing(json!({"session_id": "s", "rate_limits": {"five_hour": {}}}))]
#[case::a_window_not_object(json!({"session_id": "s", "rate_limits": {"five_hour": {}, "seven_day": 7}}))]
#[case::not_object(json!(["s"]))]
fn planted_statusline_schema_violation_is_rejected(#[case] doc: Value) {
    assert_eq!(
        check(doc.to_string().as_bytes(), &statusline_schema(), None),
        Err(Violation::Schema)
    );
}

#[rstest]
#[case::linux_home(json!({"cwd": "/home/plantuser/work"}))]
#[case::nested(json!({"workspace": {"current_dir": "C:\\Users\\plantuser\\w"}}))]
fn planted_statusline_absolute_path_is_rejected(#[case] extra: Value) {
    assert_eq!(
        check(&statusline_payload(&extra), &statusline_schema(), None),
        Err(Violation::AbsolutePath)
    );
}

#[test]
fn planted_statusline_username_is_rejected() {
    let bytes = statusline_payload(&json!({"transcript_path": "~/.claude/projects/plantuser/s"}));
    assert_eq!(
        check(&bytes, &statusline_schema(), Some("plantuser")),
        Err(Violation::Username)
    );
}

#[test]
fn claude_fixtures_walks_every_version_dir_and_only_json_files() {
    let tmp = tempfile::tempdir().expect("tempdir");
    for (rel, body) in [
        ("2.1.0/Stop.default.json", "{}"),
        ("2.1.0/notes.txt", "x"),
        ("9.9.9/SessionStart.default.json", "{}"),
        ("9.9.9/Screen.ready.json", "{}"),
        ("9.9.9/StatusLine.default-1.json", "{}"),
        ("9.9.9/Agents.recorded-1.json", "[]"),
        ("9.9.9/Mcp.tools-list-1.json", "{}"),
        ("top.json", "{}"),
    ] {
        let path = tmp.path().join(rel);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("dir");
        std::fs::write(path, body).expect("file");
    }
    let names: Vec<String> = claude_fixtures(tmp.path())
        .iter()
        .map(|p| {
            p.strip_prefix(tmp.path())
                .expect("under root")
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();
    assert_eq!(
        names,
        ["2.1.0/Stop.default.json", "9.9.9/SessionStart.default.json"]
    );
    assert_eq!(
        screen_fixtures(tmp.path()),
        [tmp.path().join("9.9.9").join("Screen.ready.json")]
    );
    assert_eq!(
        statusline_fixtures(tmp.path()),
        [tmp.path().join("9.9.9").join("StatusLine.default-1.json")]
    );
    assert_eq!(
        agents_fixtures(tmp.path()),
        [tmp.path().join("9.9.9").join("Agents.recorded-1.json")]
    );
    assert_eq!(
        mcp_fixtures(tmp.path()),
        [tmp.path().join("9.9.9").join("Mcp.tools-list-1.json")]
    );
    assert!(claude_fixtures(&tmp.path().join("missing")).is_empty());
}

/// Why a recorded MCP exchange is refused: the shared walk's class, or an email-shaped token.
#[derive(Debug, PartialEq, Eq)]
enum McpViolation {
    Hygiene(Violation),
    Email,
}

fn texts<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| texts(v, out)),
        Value::Object(map) => map.iter().for_each(|(k, v)| {
            out.push(k);
            texts(v, out);
        }),
        _ => {}
    }
}

/// An MCP exchange fixture: schema-valid, scrubbed, and no email-shaped token in a string or key.
fn check_mcp(bytes: &[u8], user: Option<&str>) -> Result<(), McpViolation> {
    check(bytes, &mcp_schema(), user).map_err(McpViolation::Hygiene)?;
    let doc: Value =
        serde_json::from_slice(bytes).map_err(|_| McpViolation::Hygiene(Violation::Schema))?;
    let mut all = Vec::new();
    texts(&doc, &mut all);
    if all.iter().any(|text| has_email(text)) {
        return Err(McpViolation::Email);
    }
    Ok(())
}

#[test]
fn mcp_fixtures_pass_hygiene() {
    let user = host_user();
    let walked = mcp_fixtures(&workspace_path("fixtures/claude"));
    assert!(!walked.is_empty(), "no MCP exchange walked");
    for path in walked {
        let bytes = std::fs::read(&path).expect("fixture");
        assert_eq!(
            check_mcp(&bytes, user.as_deref()),
            Ok(()),
            "{}",
            path.display()
        );
    }
}

/// A synthetic exchange in the recorded class's shape (claude 2.1.302 on protocol `2026-07-28`:
/// `server/discover`, then `tools/list`, each with its own `_meta`), `client_info` its client's.
fn mcp_exchange(client_info: &Value) -> Value {
    let meta = json!({"_meta": {"io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientInfo": client_info}});
    json!({
        "v": 1,
        "client": [
            {"jsonrpc": "2.0", "id": "d-1", "method": "server/discover", "params": meta},
            {"jsonrpc": "2.0", "id": 0, "method": "tools/list", "params": meta},
        ],
        "results": {
            "server/discover": {"capabilities": {"tools": {}}, "ttlMs": 0, "cacheScope": "private"},
            "tools/list": {"tools": [{"name": "send"}], "ttlMs": 0, "cacheScope": "private"},
        },
    })
}

#[test]
fn planted_clean_mcp_exchange_passes() {
    let doc = mcp_exchange(&json!({"name": "claude-code", "version": "2.1.302"}));
    assert_eq!(
        check_mcp(doc.to_string().as_bytes(), Some("plantuser")),
        Ok(())
    );
}

#[rstest]
#[case::home_path(json!({"name": "/home/plantuser/bin/claude"}), McpViolation::Hygiene(Violation::AbsolutePath))]
#[case::user_word(json!({"name": "plantuser's client"}), McpViolation::Hygiene(Violation::Username))]
#[case::email(json!({"name": "a.b@example.com"}), McpViolation::Email)]
fn planted_dirty_mcp_exchange_is_rejected(
    #[case] client_info: Value,
    #[case] expected: McpViolation,
) {
    let doc = mcp_exchange(&client_info);
    assert_eq!(
        check_mcp(doc.to_string().as_bytes(), Some("plantuser")),
        Err(expected)
    );
}

#[rstest]
#[case::wrong_version(json!({"v": 2}))]
#[case::client_not_array(json!({"client": {}}))]
#[case::a_frame_without_method(json!({"client": [{"jsonrpc": "2.0", "id": 0}]}))]
#[case::no_tools_list_frame(json!({"client": [{"jsonrpc": "2.0", "id": "d-1", "method": "server/discover"}]}))]
#[case::no_list_result(json!({"results": {"server/discover": {"capabilities": {"tools": {}}}}}))]
#[case::tools_not_array(json!({"results": {"tools/list": {"tools": {}}}}))]
fn planted_mcp_schema_violation_is_rejected(#[case] change: Value) {
    let mut doc = mcp_exchange(&json!({"name": "claude-code"}));
    if let (Some(doc), Some(change)) = (doc.as_object_mut(), change.as_object()) {
        doc.extend(change.clone());
    }
    assert_eq!(
        check_mcp(doc.to_string().as_bytes(), None),
        Err(McpViolation::Hygiene(Violation::Schema))
    );
}

/// The compiled signature literals, as test literals: the input box's two and the two modals.
const SIGNATURE_LITERALS: [&str; 4] = [
    "for agents",
    "(shift+tab to cycle)",
    "Yes, I trust this folder",
    "Yes, allow external imports",
];

#[derive(Debug, PartialEq, Eq)]
enum ScreenViolation {
    Schema,
    NotASignatureRow,
    Path,
    Username,
    Email,
}

fn has_email(text: &str) -> bool {
    text.match_indices('@').any(|(at, _)| {
        let local = text[..at]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_ascii_alphanumeric() || "._%+-".contains(c));
        let domain: String = text[at + 1..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-')
            .collect();
        let tld = domain.rsplit_once('.').is_some_and(|(host, tld)| {
            !host.is_empty() && tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic())
        });
        local && tld
    })
}

fn seam_char(c: char) -> bool {
    c == ' ' || ('\u{2500}'..='\u{257f}').contains(&c)
}

/// A screen fixture: schema-valid, every non-empty row a signature row, and no path, username or
/// email in a row or across the seam of two adjacent rows.
fn check_screen(bytes: &[u8], user: Option<&str>) -> Result<(), ScreenViolation> {
    let doc: Value = serde_json::from_slice(bytes).map_err(|_| ScreenViolation::Schema)?;
    let validator = jsonschema::validator_for(&screen_schema()).expect("valid schema");
    if !validator.is_valid(&doc) {
        return Err(ScreenViolation::Schema);
    }
    let rows: Vec<&str> = doc["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let mut texts: Vec<String> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        if !SIGNATURE_LITERALS.iter().any(|l| row.contains(l)) {
            return Err(ScreenViolation::NotASignatureRow);
        }
        texts.push((*row).to_owned());
        if let Some(next) = rows.get(i + 1).filter(|n| !n.is_empty()) {
            texts.push(format!(
                "{}{}",
                row.trim_end_matches(seam_char),
                next.trim_start_matches(seam_char)
            ));
        }
    }
    for text in &texts {
        if has_absolute_path(text) {
            return Err(ScreenViolation::Path);
        }
        if user.is_some_and(|u| has_username(text, u)) {
            return Err(ScreenViolation::Username);
        }
        if has_email(text) {
            return Err(ScreenViolation::Email);
        }
    }
    Ok(())
}

#[test]
fn claude_screens_pass_hygiene() {
    let user = host_user();
    let screens = screen_fixtures(&workspace_path("fixtures/claude"));
    assert!(!screens.is_empty(), "no recorded screen");
    for path in screens {
        let bytes = std::fs::read(&path).expect("screen");
        assert_eq!(
            check_screen(&bytes, user.as_deref()),
            Ok(()),
            "{}",
            path.display()
        );
    }
}

fn screen(rows: &[(usize, &str)]) -> Vec<u8> {
    let mut all = vec![String::new(); 24];
    for (i, row) in rows {
        all[*i] = (*row).to_owned();
    }
    json!({"screen_phase": "ready", "cols": 80, "rows": all})
        .to_string()
        .into_bytes()
}

#[test]
fn planted_clean_screen_passes() {
    let bytes = screen(&[
        (9, "   Yes, I trust this folder"),
        (23, "  ⏸ manual mode on · ← for agents"),
    ]);
    assert_eq!(check_screen(&bytes, Some("plantuser")), Ok(()));
}

#[rstest]
#[case::home_path(&[(23, "  ← for agents /home/plantuser/x")], ScreenViolation::Path)]
#[case::username(&[(23, "  ← for agents by plantuser")], ScreenViolation::Username)]
#[case::email(&[(23, "  ← for agents a.b@example.com")], ScreenViolation::Email)]
#[case::username_split_at_a_seam(
    &[(22, "   Yes, I trust this folder plant"), (23, "user ── ← for agents")],
    ScreenViolation::Username
)]
#[case::not_a_signature_row(&[(3, "  ~/work"), (23, "  ← for agents")], ScreenViolation::NotASignatureRow)]
fn planted_dirty_screen_is_rejected(
    #[case] rows: &[(usize, &str)],
    #[case] expected: ScreenViolation,
) {
    assert_eq!(
        check_screen(&screen(rows), Some("plantuser")),
        Err(expected)
    );
}

#[rstest]
#[case::short(json!({"screen_phase": "ready", "cols": 80, "rows": vec![""; 23]}))]
#[case::wide(json!({"screen_phase": "ready", "cols": 120, "rows": vec![""; 24]}))]
#[case::phase(json!({"screen_phase": "start", "cols": 80, "rows": vec![""; 24]}))]
#[case::row_not_string(json!({"screen_phase": "ready", "cols": 80, "rows": vec![1; 24]}))]
fn planted_screen_schema_violation_is_rejected(#[case] doc: Value) {
    assert_eq!(
        check_screen(doc.to_string().as_bytes(), None),
        Err(ScreenViolation::Schema)
    );
}

fn claude_payload(extra: &Value) -> Vec<u8> {
    let mut doc = json!({"hook_event_name": "Stop", "session_id": "s-1", "cwd": "~/work"});
    if let (Some(doc), Some(extra)) = (doc.as_object_mut(), extra.as_object()) {
        doc.extend(extra.clone());
    }
    doc.to_string().into_bytes()
}

#[test]
fn claude_fixture_scrubbed_payload_is_clean() {
    let bytes = claude_payload(
        &json!({"transcript_path": "~/.claude/projects/C--Users-<user>--w/s.jsonl"}),
    );
    assert_eq!(check(&bytes, &claude_schema(), Some("plantuser")), Ok(()));
}

#[rstest]
#[case::drive_path(json!({"cwd": "C:\\Users\\plantuser\\work"}))]
#[case::linux_home(json!({"transcript_path": "/home/plantuser/.claude/x.jsonl"}))]
#[case::macos_users(json!({"cwd": "/Users/plantuser/work"}))]
fn planted_claude_absolute_path_is_rejected(#[case] extra: Value) {
    assert_eq!(
        check(&claude_payload(&extra), &claude_schema(), None),
        Err(Violation::AbsolutePath)
    );
}

/// A recorded dialog payload carries upstream text deep inside `tool_input`: a path there is found
/// however deep it sits.
#[test]
fn planted_dialog_payload_with_a_nested_path_is_rejected() {
    let bytes = json!({"hook_event_name": "PreToolUse", "tool_name": "AskUserQuestion",
        "tool_input": {"questions": [{"question": "Probe color?",
            "options": [{"label": "red", "description": "see /home/plantuser/notes"}]}]}})
    .to_string()
    .into_bytes();
    assert_eq!(
        check(&bytes, &claude_schema(), None),
        Err(Violation::AbsolutePath)
    );
}

/// Every set recorded with its dialog tier holds the twelve variants, each walked by
/// `claude_fixtures_pass_hygiene`: the walk reaches the dialog classes.
#[test]
fn claude_dialog_variants_are_walked_for_every_dialog_set() {
    let walked: Vec<PathBuf> = claude_fixtures(&workspace_path("fixtures/claude"));
    // `<Event>.<stem>-<digits>.json`: the relayed `exit-plan-mode` names are not variants.
    let variant = |p: &PathBuf| {
        p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
            let middle = n.split('.').nth(1).unwrap_or_default();
            middle.rsplit_once('-').is_some_and(|(stem, k)| {
                ["questions", "parallel", "permission", "plan"].contains(&stem)
                    && !k.is_empty()
                    && k.bytes().all(|b| b.is_ascii_digit())
            })
        })
    };
    let mut sets: Vec<&Path> = walked
        .iter()
        .filter(|p| variant(p))
        .filter_map(|p| p.parent())
        .collect();
    sets.dedup();
    assert!(sets.len() >= 2, "dialog sets walked: {}", sets.len());
    for set in sets {
        let mut names: Vec<String> = walked
            .iter()
            .filter(|p| p.parent() == Some(set) && variant(p))
            .filter_map(|p| p.file_name()?.to_str().map(str::to_owned))
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                "PermissionRequest.permission-1.json",
                "PermissionRequest.plan-1.json",
                "PostToolUse.parallel-1.json",
                "PostToolUse.parallel-2.json",
                "PostToolUse.permission-1.json",
                "PostToolUse.plan-2.json",
                "PostToolUse.questions-1.json",
                "PreToolUse.parallel-1.json",
                "PreToolUse.parallel-2.json",
                "PreToolUse.plan-1.json",
                "PreToolUse.plan-2.json",
                "PreToolUse.questions-1.json",
            ],
            "{}",
            set.display()
        );
    }
}

/// Every set recorded with its framing variants holds the six, each walked by
/// `claude_fixtures_pass_hygiene`: the walk reaches the paste, compact and local-command classes.
#[test]
fn claude_framing_variants_are_walked_for_every_framing_set() {
    let walked: Vec<PathBuf> = claude_fixtures(&workspace_path("fixtures/claude"));
    // `<Event>.<stem>-<digits>.json` with a framing stem.
    let variant = |p: &PathBuf| {
        p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
            let middle = n.split('.').nth(1).unwrap_or_default();
            middle.rsplit_once('-').is_some_and(|(stem, k)| {
                ["paste", "compact", "clear"].contains(&stem)
                    && !k.is_empty()
                    && k.bytes().all(|b| b.is_ascii_digit())
            })
        })
    };
    let mut sets: Vec<&Path> = walked
        .iter()
        .filter(|p| variant(p))
        .filter_map(|p| p.parent())
        .collect();
    sets.dedup();
    assert!(!sets.is_empty(), "no framing set walked");
    for set in sets {
        let mut names: Vec<String> = walked
            .iter()
            .filter(|p| p.parent() == Some(set) && variant(p))
            .filter_map(|p| p.file_name()?.to_str().map(str::to_owned))
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                "SessionEnd.clear-1.json",
                "SessionStart.clear-1.json",
                "SessionStart.compact-1.json",
                "UserPromptSubmit.paste-1.json",
                "UserPromptSubmit.paste-2.json",
                "UserPromptSubmit.paste-3.json",
            ],
            "{}",
            set.display()
        );
    }
}

/// Every set recorded with a resume variant holds the one Run E records, walked by
/// `claude_fixtures_pass_hygiene`; the 2.1.287 set is one of them.
#[test]
fn claude_resume_variants_are_walked_for_every_resume_set() {
    let walked: Vec<PathBuf> = claude_fixtures(&workspace_path("fixtures/claude"));
    let variant = |p: &PathBuf| {
        p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
            n.split('.')
                .nth(1)
                .unwrap_or_default()
                .starts_with("resume-")
        })
    };
    let mut sets: Vec<&Path> = walked
        .iter()
        .filter(|p| variant(p))
        .filter_map(|p| p.parent())
        .collect();
    sets.dedup();
    assert!(
        sets.iter().any(|s| s.ends_with("2.1.287")),
        "the 2.1.287 set holds no resume variant"
    );
    for set in sets {
        let names: Vec<String> = walked
            .iter()
            .filter(|p| p.parent() == Some(set) && variant(p))
            .filter_map(|p| p.file_name()?.to_str().map(str::to_owned))
            .collect();
        assert_eq!(names, ["SessionStart.resume-1.json"], "{}", set.display());
    }
}

#[test]
fn planted_claude_username_is_rejected() {
    let bytes = claude_payload(
        &json!({"transcript_path": "~/.claude/projects/C--Users-plantuser--w/s.jsonl"}),
    );
    assert_eq!(
        check(&bytes, &claude_schema(), Some("plantuser")),
        Err(Violation::Username)
    );
}

#[rstest]
#[case::no_event(json!({"session_id": "s"}))]
#[case::unknown_event(json!({"hook_event_name": "Nope"}))]
#[case::event_not_string(json!({"hook_event_name": 1}))]
#[case::not_object(json!(["Stop"]))]
fn planted_claude_schema_violation_is_rejected(#[case] doc: Value) {
    assert_eq!(
        check(doc.to_string().as_bytes(), &claude_schema(), None),
        Err(Violation::Schema)
    );
}

#[rstest]
fn fake_script_passes_hygiene(#[files("fixtures/fake-scripts/*.json")] path: PathBuf) {
    let bytes = std::fs::read(&path).expect("fixture");
    let user = host_user();
    assert_eq!(
        check(&bytes, &script_schema(), user.as_deref()),
        Ok(()),
        "{}",
        path.display()
    );
}

fn script_with(text: &str) -> Vec<u8> {
    json!({"v": 1, "steps": [{"event": "Stop", "variant": text}]})
        .to_string()
        .into_bytes()
}

#[rstest]
#[case::windows_drive("C:\\work\\x")]
#[case::windows_drive_slash("d:/work/x")]
#[case::linux_home("see /home/someone/x")]
#[case::macos_users("see /Users/someone/x")]
#[case::windows_users("at \\Users\\someone\\x")]
fn planted_absolute_path_is_rejected(#[case] text: &str) {
    assert_eq!(
        check(&script_with(text), &script_schema(), None),
        Err(Violation::AbsolutePath)
    );
}

#[test]
fn planted_absolute_path_in_a_key_is_rejected() {
    let bytes = br#"{"v":1,"steps":[],"/home/x/y":1}"#;
    assert_eq!(
        check(bytes, &script_schema(), None),
        Err(Violation::AbsolutePath)
    );
}

#[test]
fn planted_username_is_rejected_placeholder_is_not() {
    let schema = script_schema();
    assert_eq!(
        check(
            &script_with("owned by Plantuser today"),
            &schema,
            Some("plantuser")
        ),
        Err(Violation::Username)
    );
    assert_eq!(
        check(&script_with("owned by <user>"), &schema, Some("plantuser")),
        Ok(())
    );
    assert_eq!(
        check(&script_with("plantusers"), &schema, Some("plantuser")),
        Ok(())
    );
    assert_eq!(
        check(&script_with("xplantuser"), &schema, Some("plantuser")),
        Ok(())
    );
}

#[rstest]
#[case::wrong_version(json!({"v": 2, "steps": []}))]
#[case::no_steps(json!({"v": 1}))]
#[case::unknown_event(json!({"v": 1, "steps": [{"event": "Nope"}]}))]
#[case::gate_not_bool(json!({"v": 1, "steps": [{"event": "Stop", "gate": "yes"}]}))]
fn planted_schema_violation_is_rejected(#[case] doc: Value) {
    assert_eq!(
        check(doc.to_string().as_bytes(), &script_schema(), None),
        Err(Violation::Schema)
    );
}

#[test]
fn non_json_is_a_schema_violation_and_unknown_fields_are_tolerated() {
    assert_eq!(
        check(b"not json", &script_schema(), None),
        Err(Violation::Schema)
    );
    let tolerant =
        json!({"v": 1, "steps": [{"event": "Stop", "extra": true}], "note": "synthetic"});
    assert_eq!(
        check(tolerant.to_string().as_bytes(), &script_schema(), None),
        Ok(())
    );
}

#[test]
fn violation_classes_have_kebab_names() {
    assert_eq!(Violation::AbsolutePath.as_str(), "absolute-path");
    assert_eq!(Violation::Username.as_str(), "username");
    assert_eq!(Violation::Schema.as_str(), "schema");
}

#[test]
fn absolute_path_and_username_predicates_hold_their_edges() {
    assert!(has_absolute_path("C:/x"));
    assert!(!has_absolute_path("C:x"));
    assert!(!has_absolute_path("1:/x"));
    assert!(!has_absolute_path("~/work"));
    assert!(!has_absolute_path("home/x"));
    assert!(!has_username("anything", ""));
    assert!(!has_username("<user>", PLACEHOLDER_USER));
    assert!(has_username("plantuser", "PlantUser"));
    assert!(has_username("a-plantuser-b", "plantuser"));
}
