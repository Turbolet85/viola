//! The fixture scrub-and-schema walk (test-plan §7 Fixture hygiene): every committed scenario
//! script is synthetic and schema-valid, and every rejection arm is proven on a planted input.
//! The `fixtures/claude/*/*.json` walk joins with the first recorded fixture.

#[allow(dead_code)]
mod support;

use std::path::PathBuf;

use rstest::rstest;
use serde_json::{Value, json};
use support::home::workspace_path;
use support::hygiene::{
    PLACEHOLDER_USER, Violation, check, has_absolute_path, has_username, host_user, load_schema,
};

fn script_schema() -> Value {
    load_schema(&workspace_path("schemas/fake-script.v1.json"))
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
