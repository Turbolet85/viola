//! The fixture scrub-and-schema walk (test-plan §7 Fixture hygiene): every committed scenario
//! script, every recorded `fixtures/claude/*/*.json` payload and every recorded `Screen.*.json` is
//! scrubbed and schema-valid, and every rejection arm is proven on a planted input. The recorded set is walked at run time, not
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

fn is_screen(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with("Screen."))
}

/// `<root>/*/*.json` but the screens, sorted: the recorded hook payloads.
fn claude_fixtures(root: &Path) -> Vec<PathBuf> {
    recorded(root)
        .into_iter()
        .filter(|p| !is_screen(p))
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
fn claude_fixtures_walks_every_version_dir_and_only_json_files() {
    let tmp = tempfile::tempdir().expect("tempdir");
    for (rel, body) in [
        ("2.1.0/Stop.default.json", "{}"),
        ("2.1.0/notes.txt", "x"),
        ("9.9.9/SessionStart.default.json", "{}"),
        ("9.9.9/Screen.ready.json", "{}"),
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
    assert!(claude_fixtures(&tmp.path().join("missing")).is_empty());
}

/// The compiled signature literals, as test literals: the input box and the two modals.
const SIGNATURE_LITERALS: [&str; 3] = [
    "for agents",
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
