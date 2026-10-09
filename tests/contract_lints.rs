//! Lint coverage and panic-hook placement (obs-plan §3 `obs-ci-gate-wire`, §7 Panic hooks), and
//! test deadlines below the nextest kill line (testing.md 2026-09-24).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The body of a `[name]` table: the lines after its header, up to the next header.
fn table(manifest: &str, name: &str) -> Option<Vec<String>> {
    tables(manifest, name).into_iter().next()
}

/// The body of every `[name]` table, in file order: an array of tables repeats its header.
fn tables(manifest: &str, name: &str) -> Vec<Vec<String>> {
    let header = format!("[{name}]");
    let lines: Vec<&str> = manifest.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.trim() == header)
        .map(|(at, _)| {
            lines[at + 1..]
                .iter()
                .take_while(|l| !l.trim_start().starts_with('['))
                .map(|l| l.trim().to_owned())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect()
        })
        .collect()
}

fn inherits_workspace_lints(manifest: &str) -> bool {
    table(manifest, "lints").is_some_and(|body| body == ["workspace = true"])
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

#[test]
fn workspace_members_inherit_the_lint_table_except_the_harness_crate() {
    let root = workspace();
    assert!(inherits_workspace_lints(&read(&root.join("Cargo.toml"))));

    let mut checked = Vec::new();
    for entry in fs::read_dir(root.join("crates")).expect("crates/") {
        let dir = entry.expect("entry").path();
        let manifest = dir.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        let name = dir
            .file_name()
            .expect("name")
            .to_string_lossy()
            .into_owned();
        let text = read(&manifest);
        if name == "viola-e2e" {
            assert!(
                table(&text, "lints").is_none(),
                "viola-e2e must not inherit the print bans"
            );
            assert!(
                table(&text, "lints.clippy").is_some(),
                "viola-e2e carries its own clippy table"
            );
        } else {
            assert!(
                inherits_workspace_lints(&text),
                "{name} lacks [lints] workspace = true"
            );
        }
        checked.push(name);
    }
    assert!(
        checked.iter().any(|n| n == "viola-core"),
        "checked {checked:?}"
    );
    assert!(
        checked.iter().any(|n| n == "viola-e2e"),
        "checked {checked:?}"
    );
}

#[test]
fn panic_hook_is_the_first_statement_of_main() {
    let main = read(&workspace().join("src").join("main.rs"));
    let first = main
        .lines()
        .skip_while(|l| l.trim() != "fn main() -> ExitCode {")
        .skip(1)
        .find(|l| !l.trim().is_empty())
        .expect("fn main body");
    assert!(
        first.trim().starts_with("std::panic::set_hook("),
        "first statement of main: {first}"
    );
}

#[test]
fn table_reads_the_body_up_to_the_next_header() {
    let text = "[a]\nx = 1\n\n# note\n[lints]\nworkspace = true\n[b]\ny = 2\n";
    assert_eq!(
        table(text, "lints"),
        Some(vec!["workspace = true".to_owned()])
    );
    assert_eq!(table(text, "missing"), None);
    let repeated = "[[o]]\nf = 1\n# note\n[[o]]\nf = 2\n[p]\nf = 3\n[[o]]\n";
    assert_eq!(
        tables(repeated, "[o]"),
        vec![vec!["f = 1".to_owned()], vec!["f = 2".to_owned()], vec![]]
    );
    assert_eq!(table(repeated, "[o]"), Some(vec!["f = 1".to_owned()]));
    assert!(inherits_workspace_lints(text));
    assert!(!inherits_workspace_lints("[lints]\nworkspace = false\n"));
    assert!(!inherits_workspace_lints(
        "[lints.rust]\nunused_must_use = \"deny\"\n"
    ));
}

/// cargo-mutants' auto timeout floors here: a wait at or above it grades a hang Timeout. Seconds,
/// not a `Duration` constant, so the lint never reads it as a test deadline.
const MUTANTS_FLOOR_SECS: u64 = 20;

const MUTANTS_OVERRIDES: &str = "[profile.mutants.overrides]";
const E2E_FILTER: &str = "filter = 'package(viola-e2e)'";
const VERIFY_WINDOW_FILTER: &str = "filter = 'test(/verify_window_/)'";

/// `period` × `terminate-after` of a table's
/// `slow-timeout = { period = "<n>s", terminate-after = <n> }` line.
fn kill_line(body: &[String]) -> Option<Duration> {
    let line = body.iter().find(|l| l.starts_with("slow-timeout"))?;
    let period: u64 = line
        .split("period = \"")
        .nth(1)?
        .split('s')
        .next()?
        .parse()
        .ok()?;
    let after: u64 = line
        .split("terminate-after = ")
        .nth(1)?
        .trim_end_matches(|c: char| !c.is_ascii_digit())
        .parse()
        .ok()?;
    Some(Duration::from_secs(period * after))
}

/// A named `Duration` constant declared on one line in the `from_secs` / `from_millis` form.
fn deadline_in(line: &str) -> Option<(String, Duration)> {
    let line = line.trim();
    let line = line.strip_prefix("pub(crate) ").unwrap_or(line);
    let line = line.strip_prefix("pub ").unwrap_or(line);
    let (name, value) = line
        .strip_prefix("const ")?
        .split_once(": Duration = Duration::from_")?;
    let (unit, n) = value.split_once('(')?;
    let n: u64 = n.split(')').next()?.trim().replace('_', "").parse().ok()?;
    let value = match unit {
        "secs" => Duration::from_secs(n),
        "millis" => Duration::from_millis(n),
        _ => return None,
    };
    Some((name.to_owned(), value))
}

struct Deadline {
    at: String,
    package: String,
    value: Duration,
}

/// Every deadline on the test side of `file` (repository-relative, `/`-separated): the whole of a
/// `tests/` file; in a `src/` file only what follows its `#[cfg(test)] mod tests {` line, the
/// inline test module the crates keep last in the file.
fn deadlines_of(rel: &str, text: &str) -> Vec<Deadline> {
    let package = rel
        .strip_prefix("crates/")
        .and_then(|r| r.split('/').next())
        .unwrap_or("viola")
        .to_owned();
    let lines: Vec<&str> = text.lines().collect();
    let from = if rel.contains("/tests/") || rel.starts_with("tests/") {
        Some(0)
    } else {
        lines
            .windows(2)
            .position(|w| w[0].trim() == "#[cfg(test)]" && w[1].trim() == "mod tests {")
    };
    let Some(from) = from else {
        return Vec::new();
    };
    lines[from..]
        .iter()
        .filter_map(|l| deadline_in(l))
        .map(|(name, value)| Deadline {
            at: format!("{rel}:{name}"),
            package: package.clone(),
            value,
        })
        .collect()
}

/// Each deadline at or past its package's kill line, or at or past cargo-mutants' floor.
fn past_the_line(deadlines: &[Deadline], kill_of: impl Fn(&str) -> Duration) -> Vec<String> {
    deadlines
        .iter()
        .filter(|d| {
            d.value >= kill_of(&d.package) || d.value >= Duration::from_secs(MUTANTS_FLOOR_SECS)
        })
        .map(|d| format!("{} {:?} vs kill {:?}", d.at, d.value, kill_of(&d.package)))
        .collect()
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

/// Every named test-side `Duration` deadline (`const NAME: Duration = Duration::from_secs(N)` or
/// `from_millis`) sits strictly below the nextest `mutants` profile's kill line for its package —
/// the `package(viola-e2e)` override's for the harness, the profile's own for every other — and
/// below cargo-mutants' 20 s floor, so a stuck wait fails its test with the report instead of
/// being killed. Scope: the files under `tests/` and `crates/*/tests/`, and the `#[cfg(test)] mod
/// tests` blocks of `src/` and `crates/*/src/`. Product constants (`PROBE_DEADLINE`,
/// `VERSION_DEADLINE`, the harness's boot deadlines) are out: they bound the product, not a test.
#[test]
fn test_deadlines_sit_below_the_nextest_kill_line() {
    let root = workspace();
    let nextest = read(&root.join(".config").join("nextest.toml"));
    let mutants = kill_line(&table(&nextest, "profile.mutants").expect("[profile.mutants]"))
        .expect("the mutants kill line");
    let overrides = tables(&nextest, MUTANTS_OVERRIDES);
    let e2e_body = overrides
        .iter()
        .find(|body| body.iter().any(|l| l == E2E_FILTER))
        .unwrap_or_else(|| panic!("no {E2E_FILTER} among {overrides:?}"));
    let e2e = kill_line(e2e_body).expect("the viola-e2e kill line");

    let mut files = Vec::new();
    rs_files(&root.join("tests"), &root, &mut files);
    rs_files(&root.join("src"), &root, &mut files);
    for entry in fs::read_dir(root.join("crates"))
        .expect("crates/")
        .flatten()
    {
        rs_files(&entry.path().join("tests"), &root, &mut files);
        rs_files(&entry.path().join("src"), &root, &mut files);
    }
    let deadlines: Vec<Deadline> = files
        .iter()
        .flat_map(|(rel, text)| deadlines_of(rel, text))
        .collect();
    assert!(!deadlines.is_empty(), "no test-side deadline found");
    let kill_of = |package: &str| if package == "viola-e2e" { e2e } else { mutants };
    assert_eq!(past_the_line(&deadlines, kill_of), Vec::<String>::new());
}

/// The position of the override carrying `filter` among `overrides`, in file order.
fn override_at(overrides: &[Vec<String>], filter: &str) -> Option<usize> {
    overrides
        .iter()
        .position(|body| body.iter().any(|l| l == filter))
}

/// nextest applies the first override that matches a test. A harness test of the `verify_window_`
/// class matches `package(viola-e2e)` too, so in the `mutants` profile the class's override stands
/// first: such a test takes the class's kill, above its designed wait, not the package's.
#[test]
fn mutants_verify_window_override_stands_before_the_harness_package_override() {
    let nextest = read(&workspace().join(".config").join("nextest.toml"));
    let overrides = tables(&nextest, MUTANTS_OVERRIDES);
    let class = override_at(&overrides, VERIFY_WINDOW_FILTER).expect("the verify_window_ override");
    let package = override_at(&overrides, E2E_FILTER).expect("the viola-e2e override");
    assert!(class < package, "{overrides:?}");
    assert!(
        kill_line(&overrides[class]) > kill_line(&overrides[package]),
        "{overrides:?}"
    );

    let swapped =
        "[[o]]\nfilter = 'package(viola-e2e)'\n[[o]]\nfilter = 'test(/verify_window_/)'\n";
    let swapped = tables(swapped, "[o]");
    assert_eq!(override_at(&swapped, E2E_FILTER), Some(0));
    assert_eq!(override_at(&swapped, VERIFY_WINDOW_FILTER), Some(1));
    assert_eq!(override_at(&swapped, "filter = 'none'"), None);
}

/// The check fires on a deadline at the kill line or at the floor and passes one below both, and
/// the reader takes only the named-constant forms it states.
#[test]
fn past_the_line_fires_at_the_kill_line_and_passes_below_it() {
    let at = |package: &str, secs: u64| Deadline {
        at: format!("planted:{secs}"),
        package: package.to_owned(),
        value: Duration::from_secs(secs),
    };
    let kill_of = |package: &str| Duration::from_secs(if package == "viola-e2e" { 30 } else { 10 });
    assert_eq!(past_the_line(&[at("viola", 7)], kill_of).len(), 0);
    assert_eq!(past_the_line(&[at("viola", 10)], kill_of).len(), 1);
    assert_eq!(past_the_line(&[at("viola-e2e", 20)], kill_of).len(), 1);
    assert_eq!(past_the_line(&[at("viola-e2e", 19)], kill_of).len(), 0);

    let body = vec![r#"slow-timeout = { period = "5s", terminate-after = 2 }"#.to_owned()];
    assert_eq!(kill_line(&body), Some(Duration::from_secs(10)));
    assert_eq!(
        deadline_in("    pub const W: Duration = Duration::from_millis(1_500);"),
        Some(("W".to_owned(), Duration::from_millis(1500)))
    );
    assert_eq!(deadline_in("let w = Duration::from_secs(3);"), None);

    let src = "fn a() {}\nconst P: Duration = Duration::from_secs(120);\n#[cfg(test)]\nmod tests {\n    const T: Duration = Duration::from_secs(4);\n}\n";
    let found = deadlines_of("crates/viola-state/src/x.rs", src);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].at, "crates/viola-state/src/x.rs:T");
    assert_eq!(found[0].package, "viola-state");
}
