//! The CLI's machine contract, read side by side per verb (test-plan §5 Cross-module patterns
//! (CLI); architecture §Standard Contracts, CLI `--json` output, and §Conventions, CLI exit codes):
//! for `send`, `wait`, `last`, `answer`, `list`, `pause`, `release`, `revive` and `revive --list`,
//! each outcome a stamped home reaches prints one `--json` document of the documented shape on
//! stdout, nothing on stderr, and its literal exit — 0, 1, 2 (no document: clap's usage), 10, 13,
//! 20 and 21 here. Exits 11, 12 and 14 are pinned by name where their cases live:
//! `tests/cli_budget.rs` (11, `budget-paused`), `tests/cli_answer.rs` (12, `unverified-cli`) and
//! `src/cmd/send.rs` `exit_of_is_the_closed_table` (14, `unknown`, and the whole 10-14 map).
//! `revive`'s other exit-1 documents, `session-live` and `cwd-missing`, and its `no-session` for an
//! `--id` the log does not hold, are read in `tests/cli_revive.rs` (`revive_json_…`). Then
//! the global home: the resolution order (`--home`, the `VIOLA_DIR` grandparent, the default), two
//! homes isolated, and a `--home` reached through a link canonicalised. Every oracle is a literal.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;
use support::cli::{Ran, Running};
use support::fake::empty_bin;
use support::home::{
    StampedHome, TestHome, UNWRAPPED, VIOLA, resolved, snapshot_data, stamped_home,
};

const CANARY: &str = "canary-chain-value-5c1e";
const UNREACHABLE: &str = "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n";

/// What one row asserts on stdout.
enum Out {
    /// The whole document, as a literal line.
    Doc(&'static str),
    /// One document whose `ok` member holds exactly these keys, in this order.
    OkKeys(&'static [&'static str]),
    /// clap's usage refusal: no document at all, the message on stderr.
    Usage,
}

/// One outcome: a label, the verb's words, its stdin, its `VIOLA_NAME`, the literal exit and what
/// stdout holds.
struct Row {
    label: &'static str,
    args: &'static [&'static str],
    stdin: Option<&'static str>,
    from: Option<&'static str>,
    exit: i32,
    out: Out,
}

/// `viola [--home <home>] <args…>` with none of the wrapped session's names, `PATH` = `bin` and
/// `env` set over it: its exit and both streams.
fn viola_in(
    home: Option<&Path>,
    args: &[&str],
    stdin: Option<&str>,
    env: &[(&str, OsString)],
    bin: &Path,
) -> Ran {
    let mut command = Command::new(VIOLA);
    if let Some(home) = home {
        command.arg("--home").arg(home);
    }
    command.args(args);
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    command.env("PATH", bin);
    for (name, value) in env {
        command.env(name, value);
    }
    command
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("viola");
    if let Some(text) = stdin {
        let mut input = child.stdin.take().expect("stdin");
        match input.write_all(text.as_bytes()) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {}
            Err(e) => panic!("stdin: {e}"),
        }
    }
    Running::over(child).finish()
}

/// `row` against `home`, every assertion naming its label.
fn assert_row(home: &Path, bin: &Path, row: &Row) {
    let env: Vec<(&str, OsString)> = row
        .from
        .iter()
        .map(|from| ("VIOLA_NAME", OsString::from(from)))
        .collect();
    let ran = viola_in(Some(home), row.args, row.stdin, &env, bin);
    let label = row.label;
    assert_eq!(
        ran.code,
        Some(row.exit),
        "{label}: stdout {} stderr {}",
        ran.stdout,
        ran.stderr
    );
    match &row.out {
        Out::Doc(doc) => {
            assert_eq!(ran.stdout, *doc, "{label}");
            assert_eq!(ran.stderr, "", "{label}");
        }
        Out::OkKeys(keys) => {
            assert_eq!(ran.stdout.lines().count(), 1, "{label}: one document");
            let doc: Value = serde_json::from_str(&ran.stdout).expect("a JSON document");
            let top: Vec<&str> = doc
                .as_object()
                .expect("an object")
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(top, ["v", "ok"], "{label}");
            assert_eq!(doc["v"], 1, "{label}");
            let ok: Vec<&str> = doc["ok"]
                .as_object()
                .expect("an ok object")
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(ok, *keys, "{label}");
            assert_eq!(ran.stderr, "", "{label}");
        }
        Out::Usage => {
            assert_eq!(ran.stdout, "", "{label}");
            assert!(ran.stderr.starts_with("error: "), "{label}: {}", ran.stderr);
        }
    }
}

/// One stamped wrapper's life, verb by verb: what a live instance answers, then, once it is
/// stopped, what a dead one does.
#[rstest::rstest]
fn every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life(
    stamped_home: StampedHome,
) {
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let home = wrapper.home().to_path_buf();
    let bin = empty_bin(wrapper.stamped.home.scratch());
    let live: [Row; 13] = [
        Row {
            label: "list ok",
            args: &["list", "--json"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::OkKeys(&["items", "budget", "skipped"]),
        },
        Row {
            label: "last ok",
            args: &["last", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::Doc("{\"v\":1,\"ok\":{\"last_assistant_message\":null,\"ts\":null}}\n"),
        },
        Row {
            label: "wait ok, timed out",
            args: &["wait", "builder", "--json", "--timeout-ms", "100"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::Doc("{\"v\":1,\"ok\":{\"timed_out\":true}}\n"),
        },
        Row {
            label: "answer refused, no such dialog",
            args: &["answer", "builder", "1", "--json"],
            stdin: Some("{\"behavior\": \"deny\", \"message\": \"canary-chain-value-5c1e\"}"),
            from: None,
            exit: 13,
            out: Out::Doc(
                "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n",
            ),
        },
        Row {
            label: "send refused, a control character",
            args: &["send", "builder", "--json"],
            stdin: Some("canary-chain-value-5c1e\u{1b}[201~"),
            from: None,
            exit: 13,
            out: Out::Doc(
                "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"control-character\"}\n",
            ),
        },
        Row {
            label: "send ok",
            args: &["send", "builder", "--json"],
            stdin: Some(CANARY),
            from: None,
            exit: 0,
            out: Out::OkKeys(&["submitted_at", "cursor"]),
        },
        Row {
            label: "pause ok",
            args: &["pause", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::Doc("{\"v\":1,\"ok\":{\"wheel\":\"human\"}}\n"),
        },
        Row {
            label: "send refused, the human has the wheel",
            args: &["send", "builder", "--json"],
            stdin: Some(CANARY),
            from: None,
            exit: 10,
            out: Out::Doc("{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n"),
        },
        Row {
            label: "release by a driver, a wrapper fault",
            args: &["release", "builder", "--json"],
            stdin: None,
            from: Some("overseer"),
            exit: 20,
            out: Out::Doc(
                "{\"v\":1,\"error\":\"wrapper-fault\",\"detail\":{\"code\":-32602,\
                 \"message\":\"invalid params\",\"data\":{\"reason\":\"release-from-driver\"}}}\n",
            ),
        },
        Row {
            label: "release ok",
            args: &["release", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::Doc("{\"v\":1,\"ok\":{\"wheel\":\"driver\",\"budget_paused\":false}}\n"),
        },
        Row {
            label: "revive refused, the name is live",
            args: &["revive", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 1,
            out: Out::Doc("{\"v\":1,\"error\":\"already-live\",\"detail\":null}\n"),
        },
        Row {
            label: "send usage, a name that is no name",
            args: &["send", "Not-A-Name", "--json"],
            stdin: Some(CANARY),
            from: None,
            exit: 2,
            out: Out::Usage,
        },
        Row {
            label: "revive usage, --list beside --fork",
            args: &["revive", "builder", "--list", "--fork", "--json"],
            stdin: None,
            from: None,
            exit: 2,
            out: Out::Usage,
        },
    ];
    for row in &live {
        assert_row(&home, &bin, row);
    }
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let dead: [Row; 7] = [
        Row {
            label: "send unreachable",
            args: &["send", "builder", "--json"],
            stdin: Some(CANARY),
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "wait unreachable",
            args: &["wait", "builder", "--json", "--timeout-ms", "100"],
            stdin: None,
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "last unreachable",
            args: &["last", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "answer unreachable",
            args: &["answer", "builder", "1", "--json"],
            stdin: Some("{\"behavior\": \"deny\", \"message\": \"canary-chain-value-5c1e\"}"),
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "pause unreachable",
            args: &["pause", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "release unreachable",
            args: &["release", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "revive --list ok",
            args: &["revive", "builder", "--list", "--json"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::OkKeys(&["sessions"]),
        },
    ];
    for row in &dead {
        assert_row(stamped.home.path(), &bin, row);
    }
}

/// A home no instance ever lived in: the board is empty and `revive` finds no session on either
/// arm, its start arm never reaching a program.
#[test]
fn verbs_over_a_home_with_no_instance_print_their_documents() {
    let tmp = TestHome::new();
    let bin = empty_bin(tmp.scratch());
    let rows = [
        Row {
            label: "list ok, an empty board",
            args: &["list", "--json"],
            stdin: None,
            from: None,
            exit: 0,
            out: Out::OkKeys(&["items", "budget", "skipped"]),
        },
        Row {
            label: "revive refused, no session",
            args: &["revive", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 1,
            out: Out::Doc("{\"v\":1,\"error\":\"no-session\",\"detail\":null}\n"),
        },
        Row {
            label: "revive --list refused, no session",
            args: &["revive", "builder", "--list", "--json"],
            stdin: None,
            from: None,
            exit: 1,
            out: Out::Doc("{\"v\":1,\"error\":\"no-session\",\"detail\":null}\n"),
        },
    ];
    for row in &rows {
        assert_row(tmp.path(), &bin, row);
    }
}

/// A home another user could write: `list` refuses the whole board (exit 21), `revive` on both
/// arms refuses `strict-modes-failed` (exit 1).
#[cfg(unix)]
#[test]
fn verbs_over_a_home_another_user_could_write_print_their_documents() {
    use std::os::unix::fs::PermissionsExt as _;
    let tmp = TestHome::new();
    let home = tmp.path();
    let dir = home.join("instances").join("builder");
    std::fs::create_dir_all(&dir).expect("the instance dir");
    for path in [home, &home.join("instances"), &dir] {
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
    std::fs::set_permissions(home, std::fs::Permissions::from_mode(0o770)).expect("chmod");
    let bin = empty_bin(tmp.scratch());
    let strict = "{\"v\":1,\"error\":\"strict-modes-failed\",\"detail\":null}\n";
    let rows = [
        Row {
            label: "list refused, the home is widened",
            args: &["list", "--json"],
            stdin: None,
            from: None,
            exit: 21,
            out: Out::Doc(UNREACHABLE),
        },
        Row {
            label: "revive refused, the home is widened",
            args: &["revive", "builder", "--json"],
            stdin: None,
            from: None,
            exit: 1,
            out: Out::Doc(strict),
        },
        Row {
            label: "revive --list refused, the home is widened",
            args: &["revive", "builder", "--list", "--json"],
            stdin: None,
            from: None,
            exit: 1,
            out: Out::Doc(strict),
        },
    ];
    for row in &rows {
        assert_row(home, &bin, row);
    }
    let mode = std::fs::metadata(home).expect("meta").permissions().mode() & 0o777;
    std::fs::set_permissions(home, std::fs::Permissions::from_mode(0o700)).expect("chmod back");
    assert_eq!(mode, 0o770, "a refused verb changed the home's mode");
}

/// `list --json`'s item names over `home` (or the resolved home with none), with `env`.
fn listed(home: Option<&Path>, env: &[(&str, OsString)], bin: &Path) -> Vec<String> {
    let ran = viola_in(home, &["list", "--json"], None, env, bin);
    assert_eq!(
        ran.code,
        Some(0),
        "stdout {} stderr {}",
        ran.stdout,
        ran.stderr
    );
    let doc: Value = serde_json::from_str(&ran.stdout).expect("a document");
    doc["ok"]["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| item["name"].as_str().expect("a name").to_owned())
        .collect()
}

/// architecture §Cross-cutting Patterns, Config management: `--home` first, then the home of the
/// wrapped session `VIOLA_NAME` and `VIOLA_DIR` name, then `<user home>/.viola`; a pair that fails
/// the shape check is no home and falls through.
#[rstest::rstest]
fn home_resolution_takes_the_flag_then_the_session_then_the_default(stamped_home: StampedHome) {
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let scratch = wrapper.stamped.home.scratch().to_path_buf();
    let bin = empty_bin(&scratch);
    let other = scratch.join("other-home");
    let user_home = scratch.join("user");
    std::fs::create_dir_all(&user_home).expect("the user home");
    let user_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    let dir = wrapper.instance_dir();
    // Under a `VIOLA_NAME` the board logs its start in the home it resolved: the log says which.
    let session = |dir: &Path| -> Vec<(&str, OsString)> {
        vec![
            ("VIOLA_NAME", OsString::from("driver")),
            ("VIOLA_DIR", dir.as_os_str().to_owned()),
            (user_var, user_home.as_os_str().to_owned()),
        ]
    };
    let logged_in = |home: &Path| home.join("diagnostics").join("cli-driver.ndjson").is_file();
    let default = user_home.join(".viola");

    let another_name = dir.parent().expect("instances").join("other");
    let relative = PathBuf::from("instances").join("builder");
    for malformed in [another_name, relative] {
        assert_eq!(
            listed(None, &session(&malformed), &bin),
            Vec::<String>::new(),
            "{malformed:?} falls to the default"
        );
        assert!(logged_in(&default), "{malformed:?}: the default home");
        assert!(
            !logged_in(wrapper.home()),
            "{malformed:?}: not the session's"
        );
    }
    // With a well-formed pair the session names its own home, so the name must be the dir's.
    let named = |dir: &Path| -> Vec<(&str, OsString)> {
        let mut env = session(dir);
        env[0] = ("VIOLA_NAME", OsString::from("builder"));
        env
    };
    let logged_as_builder = |home: &Path| {
        home.join("diagnostics")
            .join("cli-builder.ndjson")
            .is_file()
    };
    assert_eq!(
        listed(None, &named(&dir), &bin),
        ["builder"],
        "the session's home"
    );
    assert!(
        logged_as_builder(wrapper.home()),
        "the session's home took the log"
    );
    assert!(
        !logged_as_builder(&default),
        "the default took no session log"
    );
    assert_eq!(
        listed(Some(&other), &named(&dir), &bin),
        Vec::<String>::new(),
        "--home beats the session"
    );
    assert!(logged_as_builder(&other), "--home took the log");
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// A wrapper under home A, a board under home B: B shows no item, and every state file the
/// wrapper wrote is under A's canonical spelling.
#[rstest::rstest]
fn two_homes_are_isolated(stamped_home: StampedHome) {
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let scratch = wrapper.stamped.home.scratch().to_path_buf();
    let bin = empty_bin(&scratch);
    let other = scratch.join("home-b");
    assert_eq!(listed(Some(&other), &[], &bin), Vec::<String>::new());
    assert_eq!(listed(Some(wrapper.home()), &[], &bin), ["builder"]);
    assert!(!other.exists(), "home B was made");
    let canonical = resolved(wrapper.home());
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("the snapshot");
    let pinned = PathBuf::from(snapshot["pinned_bin"].as_str().expect("pinned_bin"));
    assert!(pinned.starts_with(&canonical), "{pinned:?}");
    assert_eq!(
        snapshot["endpoint"],
        support::start::endpoint_of(wrapper.home())
    );
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// A `--home` reached through a link: the wrapper hashes its endpoint over the link's target, so a
/// verb that names the target finds the same instance.
#[cfg(unix)]
#[test]
fn a_home_reached_through_a_link_is_canonicalised() {
    use support::fake::{FAKE, of_kind, wait_for};
    use support::home::{claude_dir, path_with, seed_conpty};
    use support::outer_pty::{EXIT_WITHIN, OuterPty};
    use viola_core::ViolaName;
    let tmp = TestHome::new();
    let target = tmp.path().to_path_buf();
    viola_state::fs::create_private_dir(&target).expect("the home");
    let link = tmp.scratch().join("home-link");
    std::os::unix::fs::symlink(&target, &link).expect("the link");
    let receipt = tmp.scratch().join("link.receipt.ndjson");
    let args: Vec<OsString> = vec![
        "--home".into(),
        link.clone().into(),
        "run".into(),
        "builder".into(),
        "--".into(),
        FAKE.into(),
        "--receipt".into(),
        receipt.clone().into(),
    ];
    seed_conpty(&target);
    let path = path_with(&claude_dir(tmp.scratch()));
    let mut pty = OuterPty::spawn_without(
        Path::new(VIOLA),
        &args,
        &[("PATH", path.as_str())],
        &UNWRAPPED,
        viola_pty::Size::DEFAULT,
        &std::env::current_dir().expect("cwd"),
    );
    wait_for(&receipt, "the start receipt", |l| {
        !of_kind(l, "start").is_empty()
    });
    let snapshot = snapshot_data(&target.join("instances").join("builder")).expect("the snapshot");
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let over_target =
        viola_channel::endpoint_path(&name, &resolved(&target)).expect("the endpoint");
    let over_link = viola_channel::endpoint_path(&name, &link).expect("the endpoint");
    assert_ne!(
        over_target, over_link,
        "the two spellings name two endpoints"
    );
    assert_eq!(snapshot["endpoint"], over_target.as_str());
    let pinned = snapshot["pinned_bin"].as_str().expect("pinned_bin");
    assert!(Path::new(pinned).starts_with(resolved(&target)), "{pinned}");
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
}
