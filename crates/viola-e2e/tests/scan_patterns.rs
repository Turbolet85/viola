//! One source for the canaries: every `canary-<kind>-value-<hex4>` literal the root tests and
//! sources plant must be a pattern the CI secret scan looks for (test-plan §6 Canary).

use std::fs;
use std::path::{Path, PathBuf};

use viola_e2e::harness::Workspace;
use viola_e2e::harness::secret_scan::{CONTENT, CRITICAL};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `canary-[a-z]+-value-[0-9a-f]{4}` in `text`.
fn canaries(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("canary-") {
        let tail = &rest[at + "canary-".len()..];
        let kind = tail.bytes().take_while(u8::is_ascii_lowercase).count();
        if kind > 0
            && let Some(hex) = tail[kind..].strip_prefix("-value-")
            && hex.bytes().take_while(u8::is_ascii_hexdigit).count() == 4
        {
            found.push(format!("canary-{}-value-{}", &tail[..kind], &hex[..4]));
        }
        rest = &rest[at + 1..];
    }
    found
}

#[test]
fn scan_covers_every_canary_the_root_tests_plant() {
    let root = Workspace::from_build().root;
    let mut files = Vec::new();
    rust_files(&root.join("tests"), &mut files);
    rust_files(&root.join("src"), &mut files);
    let mut planted: Vec<String> = files
        .iter()
        .filter_map(|p| fs::read_to_string(p).ok())
        .flat_map(|text| canaries(&text))
        .collect();
    planted.sort();
    planted.dedup();
    assert!(!planted.is_empty(), "no canary found under tests/ or src/");

    let scanned: Vec<&str> = CRITICAL
        .iter()
        .chain(CONTENT.iter())
        .map(|(_, n)| *n)
        .collect();
    let missing: Vec<&String> = planted
        .iter()
        .filter(|c| !scanned.contains(&c.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "canaries the scan never looks for: {missing:?}"
    );
}

#[test]
fn canaries_match_only_the_full_shape() {
    let text =
        "a canary-token-value-7f3a b canary-x-value-12 c canary-Up-value-abcd canary--value-0000";
    assert_eq!(canaries(text), ["canary-token-value-7f3a"]);
}
