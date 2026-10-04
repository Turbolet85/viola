//! The Windows mutation workflow (`.github/workflows/windows-mutants.yml`, the founder's C2 ruling):
//! it scores every source file that carries a Windows gate, installs the mutation tools at ci.yml's
//! pins, and runs only when dispatched (test-plan §10 Mutation gate).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn workflow(name: &str) -> String {
    read(&workspace().join(".github").join("workflows").join(name))
}

fn rs_files(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, root, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let rel = path
                .strip_prefix(root)
                .expect("under the workspace")
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, read(&path)));
        }
    }
}

/// Root `src/` and every `crates/*/src/`, as (repo-relative path, text).
fn product_sources() -> Vec<(String, String)> {
    let root = workspace();
    let mut files = Vec::new();
    rs_files(&root.join("src"), &root, &mut files);
    for entry in fs::read_dir(root.join("crates"))
        .expect("crates/")
        .flatten()
    {
        rs_files(&entry.path().join("src"), &root, &mut files);
    }
    files
}

fn is_windows_gate(line: &str) -> bool {
    ["cfg(windows", "cfg(all(windows", "cfg(any(windows"]
        .iter()
        .any(|gate| line.contains(gate))
        || line.contains("cfg!(windows)")
        || line.contains("target_os = \"windows\"")
}

/// The lines before the file's first column-0 `#[cfg(test)]`: a gate inside the test module
/// gates only tests, which cargo-mutants never mutates.
fn product_lines(text: &str) -> Vec<&str> {
    text.lines()
        .take_while(|l| !l.starts_with("#[cfg(test)]"))
        .collect()
}

/// `mod x;` / `pub mod x;` / `pub(crate) mod x;` → `x`.
fn declared_module(line: &str) -> Option<&str> {
    let line = line.trim();
    let rest = line.strip_prefix("mod ").or_else(|| {
        line.split_once(" mod ")
            .filter(|(vis, _)| vis.starts_with("pub"))
            .map(|(_, rest)| rest)
    })?;
    let name = rest.strip_suffix(';')?.trim();
    let ident = !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    ident.then_some(name)
}

/// The out-of-line modules a file declares, each with whether an attribute line directly above
/// its `mod` line (comments between allowed) is a Windows gate.
fn declared_modules(text: &str) -> Vec<(&str, bool)> {
    let lines = product_lines(text);
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some(name) = declared_module(line) else {
            continue;
        };
        let gated = lines[..i]
            .iter()
            .rev()
            .map(|l| l.trim())
            .take_while(|l| l.starts_with("#[") || l.starts_with("//"))
            .any(|l| l.starts_with("#[") && is_windows_gate(l));
        out.push((name, gated));
    }
    out
}

/// Where `mod name;` in `declarer` lives: beside a `lib.rs` / `main.rs` / `mod.rs`, else under
/// the declarer's stem directory (`server.rs` + `mod win;` → `server/win.rs`).
fn module_paths(declarer: &str, name: &str) -> [String; 2] {
    let (dir, file) = declarer.rsplit_once('/').unwrap_or(("", declarer));
    let base = if ["lib.rs", "main.rs", "mod.rs"].contains(&file) {
        dir.to_owned()
    } else {
        format!("{dir}/{}", file.trim_end_matches(".rs"))
    };
    [format!("{base}/{name}.rs"), format!("{base}/{name}/mod.rs")]
}

/// Every file among `files` that carries Windows code: a gate on a product line, or a whole
/// module gated at its `mod` line — and, transitively, every module such a module declares.
fn windows_gated(files: &[(String, String)]) -> BTreeSet<String> {
    let texts: BTreeMap<&str, &str> = files
        .iter()
        .map(|(p, t)| (p.as_str(), t.as_str()))
        .collect();
    let resolve = |declarer: &str, name: &str| {
        module_paths(declarer, name)
            .into_iter()
            .find(|p| texts.contains_key(p.as_str()))
            .unwrap_or_else(|| panic!("{declarer}: `mod {name};` resolves to no file"))
    };
    let mut gated: BTreeSet<String> = files
        .iter()
        .filter(|(_, text)| product_lines(text).iter().any(|l| is_windows_gate(l)))
        .map(|(path, _)| path.clone())
        .collect();
    let mut queue: Vec<String> = files
        .iter()
        .flat_map(|(path, text)| {
            declared_modules(text)
                .into_iter()
                .filter(|(_, g)| *g)
                .map(|(name, _)| resolve(path, name))
        })
        .collect();
    let mut whole = BTreeSet::new();
    while let Some(module) = queue.pop() {
        if whole.insert(module.clone()) {
            queue.extend(
                declared_modules(texts[module.as_str()])
                    .into_iter()
                    .map(|(name, _)| resolve(&module, name)),
            );
        }
    }
    gated.extend(whole);
    gated
}

/// The package owning a source path: `viola` under `src/`, `<m>` under `crates/<m>/src/`.
fn package_of(path: &str) -> Option<&str> {
    if path.starts_with("src/") {
        return Some("viola");
    }
    let rest = path.strip_prefix("crates/")?;
    let (member, rest) = rest.split_once('/')?;
    rest.starts_with("src/").then_some(member)
}

/// The workflow matrix's (package, file) pairs: each `- package:` item and its one-line,
/// space-separated `files:`.
fn workflow_scope(yml: &str) -> Vec<(String, String)> {
    let mut package: Option<String> = None;
    let mut out = Vec::new();
    for line in yml.lines().map(str::trim) {
        if let Some(p) = line.strip_prefix("- package:") {
            package = Some(p.trim().to_owned());
        } else if let Some(files) = line.strip_prefix("files:") {
            let p = package
                .take()
                .expect("a `files:` line follows its `- package:`");
            out.extend(files.split_whitespace().map(|f| (p.clone(), f.to_owned())));
        }
    }
    out
}

/// Every way the gated set and the workflow's list disagree.
fn scope_problems(
    gated: &BTreeSet<String>,
    listed: &[(String, String)],
    known: &BTreeSet<String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    let names: BTreeSet<&str> = listed.iter().map(|(_, f)| f.as_str()).collect();
    for file in gated {
        if !names.contains(file.as_str()) {
            problems.push(format!("unlisted: {file}"));
        }
    }
    let mut seen = BTreeSet::new();
    for (package, file) in listed {
        if !seen.insert(file) {
            problems.push(format!("listed twice: {file}"));
        }
        if !known.contains(file) {
            problems.push(format!("missing: {file}"));
        } else if !gated.contains(file) {
            problems.push(format!("ungated: {file}"));
        }
        if package_of(file) != Some(package.as_str()) {
            problems.push(format!("outside its package: {file} under {package}"));
        }
    }
    problems
}

/// A `tool:` line's `name@version` pins.
fn tool_pins(yml: &str, marker: &str) -> Option<BTreeMap<String, String>> {
    let line = yml.lines().find_map(|l| {
        l.trim()
            .strip_prefix("tool:")
            .filter(|tools| tools.contains(marker))
    })?;
    line.split(',')
        .map(|pin| {
            let (name, version) = pin.trim().split_once('@')?;
            Some((name.to_owned(), version.to_owned()))
        })
        .collect()
}

/// The workflow installs exactly cargo-nextest and cargo-mutants, each at ci.yml's test-job pin
/// (the single source pre-push also reads).
fn pins_problem(ci_yml: &str, workflow_yml: &str) -> Option<String> {
    let ci = tool_pins(ci_yml, "cargo-llvm-cov@")?;
    let Some(ours) = tool_pins(workflow_yml, "cargo-mutants@") else {
        return Some("no cargo-mutants tool line".to_owned());
    };
    let wanted: BTreeMap<String, String> = ["cargo-nextest", "cargo-mutants"]
        .iter()
        .filter_map(|name| Some((name.to_string(), ci.get(*name)?.clone())))
        .collect();
    (wanted.len() != 2 || ours != wanted).then(|| format!("{ours:?} against ci.yml {wanted:?}"))
}

/// The trimmed lines of the top-level `on:` block (or its inline value).
fn triggers(yml: &str) -> Vec<String> {
    let mut lines = yml.lines().skip_while(|l| !l.starts_with("on:"));
    let Some(head) = lines.next() else {
        return Vec::new();
    };
    let inline = head["on:".len()..].trim();
    if !inline.is_empty() {
        return vec![inline.to_owned()];
    }
    lines
        .take_while(|l| l.is_empty() || l.starts_with(' ') || l.starts_with('#'))
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

fn dispatch_only(yml: &str) -> bool {
    triggers(yml) == ["workflow_dispatch:"] || triggers(yml) == ["workflow_dispatch"]
}

fn synthetic(files: &[(&str, &str)]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|(p, t)| ((*p).to_owned(), (*t).to_owned()))
        .collect()
}

#[test]
fn windows_gated_sources_equal_the_workflow_scope_both_ways() {
    let fake = synthetic(&[
        (
            "crates/demo/src/lib.rs",
            "#[cfg(windows)]\npub mod side;\nmod plain;\n\n#[cfg(test)]\nmod tests {\n    #[cfg(windows)]\n    fn t() {}\n}\n",
        ),
        ("crates/demo/src/side.rs", "mod inner;\npub fn f() {}\n"),
        ("crates/demo/src/side/inner.rs", "pub fn g() {}\n"),
        (
            "crates/demo/src/plain.rs",
            "#[cfg(test)]\nmod tests {\n    #[cfg(windows)]\n    fn only_a_test() {}\n}\n",
        ),
        ("src/server.rs", "#[cfg(windows)]\nmod win;\n"),
        ("src/server/win.rs", "pub fn listen() {}\n"),
        ("src/env.rs", "let on = cfg!(windows);\n"),
    ]);
    let gated = windows_gated(&fake);
    assert_eq!(
        gated.iter().map(String::as_str).collect::<Vec<_>>(),
        [
            "crates/demo/src/lib.rs",
            "crates/demo/src/side.rs",
            "crates/demo/src/side/inner.rs",
            "src/env.rs",
            "src/server.rs",
            "src/server/win.rs",
        ],
        "a test-module gate does not count; a gated `pub mod` resolves to its file"
    );
    let known: BTreeSet<String> = fake.iter().map(|(p, _)| p.clone()).collect();
    let listed = workflow_scope(
        "        include:\n          - package: demo\n            files: crates/demo/src/lib.rs crates/demo/src/side.rs crates/demo/src/side/inner.rs crates/demo/src/plain.rs\n          - package: demo\n            files: src/env.rs src/server.rs crates/demo/src/gone.rs\n",
    );
    assert_eq!(
        scope_problems(&gated, &listed, &known),
        [
            "unlisted: src/server/win.rs",
            "ungated: crates/demo/src/plain.rs",
            "outside its package: src/env.rs under demo",
            "outside its package: src/server.rs under demo",
            "missing: crates/demo/src/gone.rs",
        ]
    );

    let sources = product_sources();
    let known: BTreeSet<String> = sources.iter().map(|(p, _)| p.clone()).collect();
    let listed = workflow_scope(&workflow("windows-mutants.yml"));
    assert!(!listed.is_empty(), "the workflow lists no file");
    let problems = scope_problems(&windows_gated(&sources), &listed, &known);
    assert!(problems.is_empty(), "{problems:#?}");
}

#[test]
fn workflow_tool_pins_equal_the_ci_test_job_line() {
    let ci = "          tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1\n";
    assert_eq!(
        pins_problem(
            ci,
            "          tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0\n"
        ),
        None
    );
    assert!(
        pins_problem(
            ci,
            "          tool: cargo-nextest@0.9.146,cargo-mutants@27.0.0\n"
        )
        .is_some()
    );
    assert!(
        pins_problem(
            ci,
            "          tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1\n"
        )
        .is_some()
    );
    assert!(pins_problem(ci, "          tool: cargo-nextest@0.9.146\n").is_some());

    let ci = workflow("ci.yml");
    assert!(
        tool_pins(&ci, "cargo-llvm-cov@").is_some(),
        "ci.yml's test-job tool line"
    );
    assert_eq!(pins_problem(&ci, &workflow("windows-mutants.yml")), None);
}

#[test]
fn workflow_trigger_is_dispatch_only_without_inputs() {
    let base = "name: x\n\non:\n  workflow_dispatch:\n\npermissions: {}\n";
    assert!(dispatch_only(base));
    assert!(!dispatch_only(
        "name: x\n\non:\n  workflow_dispatch:\n  push:\n\npermissions: {}\n"
    ));
    assert!(!dispatch_only(
        "name: x\n\non:\n  workflow_dispatch:\n    inputs:\n      package:\n\npermissions: {}\n"
    ));
    assert!(!dispatch_only("name: x\n\non: [push, workflow_dispatch]\n"));

    assert!(dispatch_only(&workflow("windows-mutants.yml")));
}
