//! `viola plugin install` over the CLI (architecture §Established Decisions [Plugin Scope]; test-plan
//! §5 Cross-module patterns (CLI)): the verb writes the MCP-only plugin's local marketplace under
//! `<home>/marketplace/` — two manifests and a `.mcp.json` naming the exe that ran it by its
//! canonical absolute path, no hooks — and prints the CLI's own two commands. A home another user
//! could write refuses before anything is created or logged. It spawns no `claude`.

#[allow(dead_code)]
mod support;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};
use support::cli::{Ran, Running, viola};
use support::home::{TestHome, UNWRAPPED, VIOLA, resolved};
use support::hygiene::has_absolute_path;
use support::ndjson::read_lines;

const OK: &str = "{\"v\":1,\"ok\":{\"marketplace\":\"viola\",\"plugin\":\"viola@viola\"}}\n";
#[cfg(unix)]
const UNREACHABLE: &str =
    "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"strict-modes-failed\"}\n";

const FILES: [&str; 3] = [
    ".claude-plugin/marketplace.json",
    "viola/.claude-plugin/plugin.json",
    "viola/.mcp.json",
];
const DIRS: [&str; 3] = [".claude-plugin", "viola", "viola/.claude-plugin"];

fn install(home: &Path, json: bool, from: Option<&str>) -> Ran {
    let args: &[&str] = if json {
        &["plugin", "install", "--json"]
    } else {
        &["plugin", "install"]
    };
    viola(home, args, None, from)
}

/// `viola --home <home> plugin install [--json]` with `VIOLA_*` and the colour facts removed, then
/// `env` set.
fn install_with(home: &Path, json: bool, env: &[(&str, &str)]) -> Ran {
    let mut command = Command::new(VIOLA);
    command.arg("--home").arg(home).args(["plugin", "install"]);
    if json {
        command.arg("--json");
    }
    for name in UNWRAPPED.iter().chain(&["NO_COLOR", "TERM", "COLORTERM"]) {
        command.env_remove(name);
    }
    command
        .envs(env.iter().copied())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    Running::over(command.spawn().expect("viola")).finish()
}

fn fwd(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Every entry under `dir`, by its forward-slash path below it: a dir maps to `None`, a file to its
/// bytes.
fn entries(dir: &Path) -> BTreeMap<String, Option<Vec<u8>>> {
    let mut found = BTreeMap::new();
    let mut walk = vec![dir.to_path_buf()];
    while let Some(at) = walk.pop() {
        for entry in std::fs::read_dir(&at).expect("a dir") {
            let path = entry.expect("entry").path();
            let below = fwd(path.strip_prefix(dir).expect("below"));
            if path.is_dir() {
                found.insert(below, None);
                walk.push(path);
            } else {
                found.insert(below, Some(std::fs::read(&path).expect("a file")));
            }
        }
    }
    found
}

fn marketplace(home: &Path) -> PathBuf {
    home.join("marketplace")
}

fn mcp_json(home: &Path) -> Value {
    let text = std::fs::read_to_string(marketplace(home).join("viola/.mcp.json")).expect("mcp");
    serde_json::from_str(&text).expect("json")
}

#[test]
fn plugin_install_writes_the_mcp_only_marketplace() {
    let tmp = TestHome::new();
    let home = tmp.path();
    let ran = install(home, false, None);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);

    let under_home = entries(home);
    let mut want: Vec<String> = vec!["marketplace".to_owned()];
    want.extend(
        DIRS.iter()
            .chain(&FILES)
            .map(|p| format!("marketplace/{p}")),
    );
    want.sort();
    assert_eq!(under_home.keys().cloned().collect::<Vec<_>>(), want);
    for dir in DIRS {
        assert!(under_home[&format!("marketplace/{dir}")].is_none(), "{dir}");
    }
    for file in FILES {
        assert!(
            under_home[&format!("marketplace/{file}")].is_some(),
            "{file}"
        );
    }
    assert!(
        under_home.keys().all(|path| path
            .split('/')
            .all(|part| part != "hooks" && part != "hooks.json")),
        "{under_home:?}"
    );

    let exe = fwd(&resolved(Path::new(VIOLA)));
    assert!(Path::new(&exe).is_absolute(), "{exe}");
    let server = &mcp_json(home)["mcpServers"]["viola"];
    assert_eq!(server["command"], exe.as_str());
    assert_eq!(
        server["args"],
        json!(["--home", fwd(&resolved(home)), "mcp"])
    );
    assert_eq!(
        mcp_json(home)["mcpServers"].as_object().map(|m| m.len()),
        Some(1)
    );
}

#[cfg(unix)]
fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::metadata(path).expect("meta").permissions().mode() & 0o777
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod");
}

/// Every mode under the home and the home's own, by forward-slash path.
#[cfg(unix)]
fn modes(home: &Path) -> BTreeMap<String, u32> {
    let mut found: BTreeMap<String, u32> = entries(home)
        .into_keys()
        .map(|below| {
            let mode = mode_of(&home.join(&below));
            (below, mode)
        })
        .collect();
    found.insert(String::new(), mode_of(home));
    found
}

#[cfg(unix)]
#[test]
fn plugin_install_writes_owner_only_modes() {
    let tmp = TestHome::new();
    let home = tmp.path();
    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg("umask 000; exec \"$0\" \"$@\"")
        .arg(VIOLA)
        .arg("--home")
        .arg(home)
        .args(["plugin", "install"]);
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let ran = Running::over(command.spawn().expect("sh")).finish();
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let found = modes(home);
    assert_eq!(found.len(), 1 + 1 + DIRS.len() + FILES.len(), "{found:?}");
    for (path, mode) in &found {
        let file = FILES.iter().any(|f| path == &format!("marketplace/{f}"));
        assert_eq!(*mode, if file { 0o600 } else { 0o700 }, "{path:?}");
    }
}

#[test]
fn plugin_install_rerun_rewrites_the_same_tree() {
    let tmp = TestHome::new();
    let home = tmp.path();
    assert_eq!(install(home, false, None).code, Some(0));
    let first = entries(home);
    #[cfg(unix)]
    let first_modes = modes(home);
    let again = install(home, true, None);
    assert_eq!(again.code, Some(0), "{}", again.stderr);
    assert_eq!(entries(home), first);
    #[cfg(unix)]
    assert_eq!(modes(home), first_modes);
}

#[test]
fn plugin_install_json_prints_the_ok_document() {
    let tmp = TestHome::new();
    let home = tmp.path();
    let ran = install(home, true, None);
    assert_eq!(ran.code, Some(0));
    assert_eq!(ran.stdout, OK);
    assert!(ran.stderr.is_empty(), "{}", ran.stderr);
    let spelt = resolved(home).to_string_lossy().into_owned();
    for stream in [&ran.stdout, &ran.stderr] {
        assert!(!has_absolute_path(stream), "{stream}");
        assert!(!stream.contains(&spelt), "{stream}");
        assert!(!stream.contains(&fwd(&resolved(home))), "{stream}");
    }
}

#[test]
fn plugin_install_prints_the_two_commands() {
    let tmp = TestHome::new();
    let home = tmp.path();
    let ran = install(home, false, None);
    assert_eq!(ran.code, Some(0));
    assert!(ran.stderr.is_empty(), "{}", ran.stderr);
    let dir = marketplace(&resolved(home)).to_string_lossy().into_owned();
    // A test home is spelt with no character a shell would split on, so the dir prints bare.
    assert!(
        dir.chars()
            .all(|c| c.is_ascii_alphanumeric() || "_-./:\\".contains(c)),
        "{dir}"
    );
    assert_eq!(
        ran.stdout,
        format!(
            concat!(
                "viola@viola  written  {dir}\n",
                "run these to add it to Claude Code:\n",
                "  claude plugin marketplace add {dir}\n",
                "  claude plugin install viola@viola\n",
            ),
            dir = dir
        )
    );
}

/// The refusal comes before the log would open: under a name, a home at 0770 stays 0770, its
/// marketplace unchanged and no `diagnostics/` file created, on both forms.
#[cfg(unix)]
#[test]
fn plugin_install_refuses_a_widened_home() {
    let tmp = TestHome::new();
    let home = tmp.path();
    assert_eq!(install(home, false, None).code, Some(0));
    set_mode(home, 0o770);
    let before = entries(home);

    let human = install(home, false, Some("overseer"));
    assert_eq!(human.code, Some(21));
    assert!(human.stdout.is_empty(), "{}", human.stdout);
    assert_eq!(
        human.stderr,
        "unable: the viola home is not private to you\n\
         hint: the viola home must be readable only by you; viola does not change its permissions\n"
    );
    let json = install(home, true, Some("overseer"));
    assert_eq!(json.code, Some(21));
    assert_eq!(json.stdout, UNREACHABLE);
    assert!(json.stderr.is_empty(), "{}", json.stderr);

    assert_eq!(entries(home), before, "a file was added or changed");
    assert!(!home.join("diagnostics").exists());
    assert_eq!(mode_of(home), 0o770);
    set_mode(home, 0o700);
}

#[test]
fn plugin_install_logs_the_self_pair_under_a_name() {
    let tmp = TestHome::new();
    let home = tmp.path();
    assert_eq!(install(home, true, Some("overseer")).code, Some(0));
    let lines = read_lines(&home.join("diagnostics").join("cli-overseer.ndjson"));
    let of = |event: &str| -> Vec<&Value> {
        lines
            .iter()
            .filter(|l| l["event"] == event && l["subject"] == "self")
            .collect()
    };
    assert_eq!(of("process-start").len(), 1, "{lines:?}");
    let exits = of("process-exit");
    assert_eq!(exits.len(), 1, "{lines:?}");
    assert_eq!(exits[0]["exit_code"], 0);

    let unnamed = TestHome::new();
    assert_eq!(install(unnamed.path(), true, None).code, Some(0));
    assert!(!unnamed.path().join("diagnostics").exists());
}

#[test]
fn plugin_install_writes_no_escape_byte() {
    let tmp = TestHome::new();
    let home = tmp.path();
    let piped: &[(&str, &str)] = &[];
    for env in [piped, &[("NO_COLOR", "1")], &[("TERM", "dumb")]] {
        for json in [false, true] {
            let ran = install_with(home, json, env);
            assert_eq!(ran.code, Some(0), "{env:?} {json}: {}", ran.stderr);
            for stream in [&ran.stdout, &ran.stderr] {
                assert!(
                    stream.bytes().all(|b| b.is_ascii() && b != 0x1B),
                    "{env:?} {json}: {stream:?}"
                );
            }
        }
    }
}

#[test]
fn plugin_alone_is_a_usage_error() {
    let tmp = TestHome::new();
    assert_eq!(viola(tmp.path(), &["plugin"], None, None).code, Some(2));
}
