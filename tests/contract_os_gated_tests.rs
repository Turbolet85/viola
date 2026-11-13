//! Every OS-gated test names why its behaviour is one OS's (architecture §Cross-cutting Patterns,
//! Cross-platform discipline; test-plan §1 multi-os-compat): `GATES` names each gated test function
//! and each gated test file under `tests/` and `crates/*/tests/` with its predicate and the reason,
//! and a census of those files reads red on a gate the table does not name, on a row whose gate no
//! longer stands, and on any gate naming Windows or macOS, named or not: the tree is Linux-only. The
//! table is a literal, never derived from the tree it checks.
//!
//! andromeda:walks-tree — it reads every `.rs` file directly under `tests/` and each
//! `crates/*/tests/`, named or not.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use rstest::rstest;
use source_scan::{Mask, masked};

#[allow(dead_code)]
#[path = "support/source_scan.rs"]
mod source_scan;

/// One OS-gated test: a function (`test` its name) or a whole file (`test` empty).
struct Gate {
    file: &'static str,
    test: &'static str,
    /// The predicate as written inside `cfg(…)`, `cfg_attr(…, …)` or `#![cfg(…)]`.
    gate: &'static str,
    /// The behaviour that makes it one OS's.
    reason: &'static str,
}

const UNIX_MODES: &str =
    "the widening is made through Unix mode bits, the strict-modes rule's Unix form";
const SOCKET_DIR: &str = "the socket directory and its mode check are the Unix endpoint's";
const UNIX_STATUSLINE: &str =
    "the status line is wrapped on Unix only, the user's command run through /bin/sh";

const GATES: &[Gate] = &[
    Gate {
        file: "crates/viola-e2e/tests/harness_lifecycle.rs",
        test: "cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone",
        gate: "unix",
        reason: "a force-killed wrapper leaves its Unix socket file, which cleanup reads by a refused connect",
    },
    Gate {
        file: "crates/viola-e2e/tests/mcp_tools.rs",
        test: "mcp_tools_a_widened_home_exits_1_and_prints_nothing",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "crates/viola-mcp/tests/handlers.rs",
        test: "handlers_frame_tools_refuse_a_loosened_socket_directory",
        gate: "unix",
        reason: SOCKET_DIR,
    },
    Gate {
        file: "tests/cli_controls_not_disableable.rs",
        test: "setting_does_not_disable_the_home_check",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_controls_not_disableable.rs",
        test: "setting_does_not_disable_the_socket_dir_check",
        gate: "unix",
        reason: SOCKET_DIR,
    },
    Gate {
        file: "tests/cli_controls_not_disableable.rs",
        test: "viola_dir_does_not_disable_the_home_scan",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_error_bodies.rs",
        test: "cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text",
        gate: "unix",
        reason: SOCKET_DIR,
    },
    Gate {
        file: "tests/cli_fake_agent.rs",
        test: "fake_agent_receipt_lists_open_fds",
        gate: "unix",
        reason: "the fds receipt kind is written on Unix alone, where file descriptors are inherited",
    },
    Gate {
        file: "tests/cli_instance_state.rs",
        test: "home_base_backing_gone_target_is_made_again_owner_only",
        gate: "unix",
        reason: "the home backing is a Unix symlink whose target is made again at mode 0700",
    },
    Gate {
        file: "tests/cli_instance_state.rs",
        test: "home_base_backing_group_or_other_bit_is_refused",
        gate: "unix",
        reason: "the refused backing is told by its target's group and other mode bits",
    },
    Gate {
        file: "tests/cli_instance_state.rs",
        test: "home_base_backing_owner_only_target_is_kept_untouched",
        gate: "unix",
        reason: "the home backing is a symlink whose target's owner and mode are read",
    },
    Gate {
        file: "tests/cli_instance_state.rs",
        test: "home_base_backing_relative_target_is_refused",
        gate: "unix",
        reason: "the home backing is a Unix symlink, refused for its relative target",
    },
    Gate {
        file: "tests/cli_instance_state.rs",
        test: "home_base_backing_target_that_is_a_link_is_refused",
        gate: "unix",
        reason: "the home backing is a symlink chain, and the refusal reads its target's file type",
    },
    Gate {
        file: "tests/cli_instance_state.rs",
        test: "path6_wrapped_statusline_prints_the_user_output_unchanged",
        gate: "unix",
        reason: UNIX_STATUSLINE,
    },
    Gate {
        file: "tests/cli_links.rs",
        test: "link_verbs_refuse_a_widened_instance_before_any_log",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_list.rs",
        test: "list_refuses_a_home_another_user_could_write",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_list.rs",
        test: "list_refuses_an_instance_another_user_could_write",
        gate: "unix",
        reason: "the instance is widened through its Unix mode bits, the write rule's Unix form",
    },
    Gate {
        file: "tests/cli_machine_contract.rs",
        test: "a_home_reached_through_a_link_is_canonicalised",
        gate: "unix",
        reason: "the home is reached through a Unix symlink",
    },
    Gate {
        file: "tests/cli_machine_contract.rs",
        test: "verbs_over_a_home_another_user_could_write_print_their_documents",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_plugin_install.rs",
        test: "plugin_install_refuses_a_widened_home",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_plugin_install.rs",
        test: "plugin_install_writes_owner_only_modes",
        gate: "unix",
        reason: "the written tree is read by its Unix mode bits under an open umask",
    },
    Gate {
        file: "tests/cli_revive.rs",
        test: "revive_json_refuses_state_another_user_could_write_on_both_arms",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_revive.rs",
        test: "revive_list_refuses_state_another_user_could_write",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_revive.rs",
        test: "revive_over_a_home_another_user_could_write_is_refused_and_leaves_it_as_found",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_revive.rs",
        test: "revive_refuses_state_another_user_could_write",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/cli_verify.rs",
        test: "verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1",
        gate: "unix",
        reason: "the stop is a Unix SIGINT or SIGTERM sent to the viola the case started",
    },
    Gate {
        file: "tests/cli_verify.rs",
        test: "verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1",
        gate: "unix",
        reason: "the stop is a Unix SIGINT or SIGTERM sent to the viola the case started",
    },
    Gate {
        file: "tests/cli_version_gate.rs",
        test: "run_with_world_writable_stamps_is_unverified_whatever_they_hold",
        gate: "unix",
        reason: UNIX_MODES,
    },
    Gate {
        file: "tests/contract_fake_agent_drift.rs",
        test: "fake_agent_statusline_replay_matches_every_recorded_statusline_set",
        gate: "unix",
        reason: "the status line is wrapped on Unix only, so its variants replay there",
    },
    Gate {
        file: "tests/cross_exit_causes.rs",
        test: "a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused",
        gate: "unix",
        reason: SOCKET_DIR,
    },
    Gate {
        file: "tests/cross_exit_causes.rs",
        test: "a_loosened_socket_directory_on_a_start_is_socket_dir_refused",
        gate: "unix",
        reason: SOCKET_DIR,
    },
    Gate {
        file: "tests/cross_exit_causes.rs",
        test: "strict_modes_on_plugin_install_is_exit_21_and_creates_nothing",
        gate: "unix",
        reason: "the home is widened through its Unix mode bits, the read rule's Unix form",
    },
    Gate {
        file: "tests/cross_exit_causes.rs",
        test: "strict_modes_on_the_board_is_exit_21_and_opens_no_log",
        gate: "unix",
        reason: "the instance is widened through its Unix mode bits, the read rule's Unix form",
    },
    Gate {
        file: "tests/hook_statusline.rs",
        test: "hook_statusline_a_failing_command_prints_nothing",
        gate: "unix",
        reason: UNIX_STATUSLINE,
    },
    Gate {
        file: "tests/hook_statusline.rs",
        test: "hook_statusline_a_home_another_user_can_write_runs_nothing_however_often",
        gate: "unix",
        reason: UNIX_STATUSLINE,
    },
    Gate {
        file: "tests/hook_statusline.rs",
        test: "hook_statusline_passes_the_user_output_through_and_records_the_reading",
        gate: "unix",
        reason: UNIX_STATUSLINE,
    },
    Gate {
        file: "tests/hook_statusline.rs",
        test: "hook_statusline_without_rate_limits_writes_no_reading_and_still_passes_through",
        gate: "unix",
        reason: UNIX_STATUSLINE,
    },
    Gate {
        file: "tests/run_cli.rs",
        test: "run_creates_home_0700_and_role_file_0600",
        gate: "unix",
        reason: "the home and the role file are read by their Unix mode bits",
    },
    Gate {
        file: "tests/security_negatives_socket_dir.rs",
        test: "",
        gate: "unix",
        reason: SOCKET_DIR,
    },
    Gate {
        file: "tests/tui_channel_fds.rs",
        test: "",
        gate: "unix",
        reason: "inherited file descriptors are Unix's",
    },
    Gate {
        file: "tests/tui_env_strip.rs",
        test: "tui_env_strip_keeps_a_pass_listed_name_on_unix",
        gate: "unix",
        reason: "the pass-list is the Unix strip's",
    },
];

/// `(test, gate)`: a found gate, `test` empty for a whole file.
type Found = BTreeSet<(String, String)>;

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The `.rs` files directly in `dir`, sorted.
fn rs_files_in(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|e| e == "rs"))
        .collect();
    out.sort();
    out
}

/// Every census file, repo-relative and `/`-separated, with its text: `tests/*.rs` and each
/// `crates/*/tests/*.rs` (`tests/support/` holds no test).
fn census_files(root: &Path) -> Vec<(String, String)> {
    let mut files = rs_files_in(&root.join("tests"));
    let mut members: Vec<PathBuf> = fs::read_dir(root.join("crates"))
        .expect("crates")
        .flatten()
        .map(|entry| entry.path().join("tests"))
        .filter(|dir| dir.is_dir())
        .collect();
    members.sort();
    for dir in members {
        files.extend(rs_files_in(&dir));
    }
    files
        .into_iter()
        .map(|path| {
            let text =
                fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let rel = path
                .strip_prefix(root)
                .expect("under the workspace")
                .to_string_lossy()
                .replace('\\', "/");
            (rel, text)
        })
        .collect()
}

/// The bracket depth `text` ends at, string and raw-string literals skipped; `None` when a literal
/// is still open.
fn depth(text: &str) -> Option<i32> {
    let bytes = text.as_bytes();
    let (mut depth, mut i) = (0, 0);
    while i < bytes.len() {
        match bytes[i] {
            b'r' if bytes.get(i + 1).is_some_and(|b| *b == b'"' || *b == b'#') => {
                let hashes = bytes[i + 1..].iter().take_while(|b| **b == b'#').count();
                if bytes.get(i + 1 + hashes) != Some(&b'"') {
                    i += 1;
                    continue;
                }
                let close = format!("\"{}", "#".repeat(hashes));
                let from = i + 2 + hashes;
                i = from + text[from..].find(&close)? + close.len();
            }
            b'"' => {
                let mut j = i + 1;
                while *bytes.get(j)? != b'"' {
                    j += if bytes[j] == b'\\' { 2 } else { 1 };
                }
                i = j + 1;
            }
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                depth -= 1;
                i += 1;
            }
            _ => i += 1,
        }
    }
    Some(depth)
}

/// The predicate of a `#[cfg(…)]`, `#[cfg_attr(…, …)]` or `#![cfg(…)]` attribute, trimmed.
fn predicate(attr: &str) -> Option<String> {
    let body = attr
        .strip_prefix("#![")
        .or_else(|| attr.strip_prefix("#["))?
        .strip_suffix(']')?;
    let (attr_kind, rest) = body.split_once('(')?;
    let inner = rest.strip_suffix(')')?;
    match attr_kind.trim() {
        "cfg" => Some(inner.trim().to_owned()),
        "cfg_attr" => {
            let mut level = 0;
            for (at, ch) in inner.char_indices() {
                match ch {
                    '(' | '[' => level += 1,
                    ')' | ']' => level -= 1,
                    ',' if level == 0 => return Some(inner[..at].trim().to_owned()),
                    _ => {}
                }
            }
            Some(inner.trim().to_owned())
        }
        _ => None,
    }
}

/// The OS word a gate in this tree may not name, held apart so this file names no such gate
/// itself.
const WINDOWS: &str = "windows";

/// Whether `predicate` names one of `words` as a word outside its string values
/// (`feature = "unix-like"` names none).
fn names_a_word(predicate: &str, words: &[&str]) -> bool {
    let code: String = predicate
        .split('"')
        .step_by(2)
        .collect::<Vec<_>>()
        .join(" ");
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    words.iter().any(|os| {
        code.match_indices(os).any(|(at, _)| {
            let before = code[..at].chars().next_back();
            let after = code[at + os.len()..].chars().next();
            !before.is_some_and(word) && !after.is_some_and(word)
        })
    })
}

/// Whether `predicate` names `windows`, `unix` or `target_os`.
fn names_an_os(predicate: &str) -> bool {
    names_a_word(predicate, &[WINDOWS, "unix", "target_os"])
}

/// Whether `predicate` names Windows or macOS, which a Linux-only tree gates nothing on.
fn names_another_os(predicate: &str) -> bool {
    let spaced: String = predicate.split_whitespace().collect();
    names_a_word(predicate, &[WINDOWS]) || spaced.contains("target_os=\"macos\"")
}

fn is_test_attr(attr: &str) -> bool {
    let Some(body) = attr.strip_prefix("#[") else {
        return false;
    };
    let path: String = body
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
        .collect();
    matches!(path.as_str(), "test" | "rstest" | "tokio::test")
}

/// The name of the function `line` (trimmed) declares, if it declares one.
fn declared_fn(line: &str) -> Option<&str> {
    let mut rest = line;
    for prefix in ["pub(crate) ", "pub ", "async ", "unsafe "] {
        rest = rest.strip_prefix(prefix).unwrap_or(rest);
    }
    let name = rest.strip_prefix("fn ")?;
    let end = name
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(name.len());
    (end > 0).then(|| &name[..end])
}

/// One file's OS gates: each leading `#![cfg(…)]` naming an OS (test empty), and each test
/// function whose contiguous attribute run holds a test attribute and a `cfg` or `cfg_attr` naming
/// an OS. Comments are masked first; a line the mask blanked reads as a comment line.
fn census(text: &str) -> Found {
    let mut found = Found::new();
    let code = String::from_utf8(masked(text, Mask::Comments)).expect("masking keeps UTF-8");
    let lines: Vec<&str> = code.lines().map(str::trim).collect();
    let comment: Vec<bool> = code
        .lines()
        .zip(text.lines())
        .map(|(code, raw)| code.trim().is_empty() && !raw.trim().is_empty())
        .collect();
    let mut i = 0;
    let attribute = |i: &mut usize| {
        let mut attr = lines[*i].to_owned();
        while depth(&attr) != Some(0) && *i + 1 < lines.len() {
            *i += 1;
            attr.push(' ');
            attr.push_str(lines[*i]);
        }
        attr
    };
    while i < lines.len() {
        let line = lines[i];
        if line.is_empty() {
            i += 1;
            continue;
        }
        if !line.starts_with("#![") {
            break;
        }
        let attr = attribute(&mut i);
        if let Some(p) = predicate(&attr).filter(|p| names_an_os(p)) {
            found.insert((String::new(), p));
        }
        i += 1;
    }
    let mut run: Vec<String> = Vec::new();
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("#[") {
            run.push(attribute(&mut i));
        } else if !comment[i] {
            if let Some(name) = declared_fn(line)
                && run.iter().any(|a| is_test_attr(a))
            {
                for p in run.iter().filter_map(|a| predicate(a)) {
                    if names_an_os(&p) {
                        found.insert((name.to_owned(), p));
                    }
                }
            }
            run.clear();
        }
        i += 1;
    }
    found
}

/// The census of `files` against `table`, both ways: each gate the table does not name, each row
/// whose gate no longer stands, and each gate naming Windows or macOS, named or not.
fn faults(files: &[(String, String)], table: &[Gate]) -> Vec<String> {
    let found: BTreeSet<(String, String, String)> = files
        .iter()
        .flat_map(|(file, text)| {
            census(text)
                .into_iter()
                .map(move |(test, gate)| (file.clone(), test, gate))
        })
        .collect();
    let named: BTreeSet<(String, String, String)> = table
        .iter()
        .map(|g| (g.file.to_owned(), g.test.to_owned(), g.gate.to_owned()))
        .collect();
    let mut out: Vec<String> = found
        .difference(&named)
        .map(|(file, test, gate)| format!("unnamed: {file} {test} cfg({gate})"))
        .collect();
    out.extend(
        named
            .difference(&found)
            .map(|(file, test, gate)| format!("no longer stands: {file} {test} cfg({gate})")),
    );
    out.extend(
        found
            .iter()
            .filter(|(_, _, gate)| names_another_os(gate))
            .map(|(file, test, gate)| format!("another OS: {file} {test} cfg({gate})")),
    );
    out
}

#[test]
fn os_gated_tests_are_each_named_with_their_reason() {
    let faults = faults(&census_files(&workspace()), GATES);
    assert!(faults.is_empty(), "{faults:#?}");
    for g in GATES {
        assert!(
            !g.reason.trim().is_empty(),
            "{} {}: no reason",
            g.file,
            g.test
        );
    }
}

#[test]
fn gates_table_rows_are_unique() {
    let mut seen = BTreeSet::new();
    for g in GATES {
        assert!(
            seen.insert((g.file, g.test, g.gate)),
            "{} {} cfg({}) twice",
            g.file,
            g.test,
            g.gate
        );
    }
}

const PLANTED_ROW: Gate = Gate {
    file: "tests/planted.rs",
    test: "planted",
    gate: "unix",
    reason: "planted",
};

#[rstest]
#[case::an_unnamed_gated_test(
    "/// doc\n#[cfg(unix)]\n#[test]\nfn planted() {}\n",
    &[],
    &["unnamed: tests/planted.rs planted cfg(unix)"]
)]
#[case::the_same_test_named(
    "/// doc\n#[cfg(unix)]\n#[test]\nfn planted() {}\n",
    &[PLANTED_ROW],
    &[]
)]
#[case::a_row_whose_gate_is_gone(
    "#[test]\nfn planted() {}\n",
    &[PLANTED_ROW],
    &["no longer stands: tests/planted.rs planted cfg(unix)"]
)]
#[case::a_gate_on_a_helper_is_no_hit("#[cfg(unix)]\nfn helper() {}\n", &[], &[])]
#[case::a_cfg_attr_over_a_case_row(
    "#[rstest]\n#[cfg_attr(\n    not(target_os = \"linux\"),\n    case::x(\"]\")\n)]\nfn planted(#[case] x: &str) {}\n",
    &[],
    &["unnamed: tests/planted.rs planted cfg(not(target_os = \"linux\"))"]
)]
#[case::a_gated_file("//! doc\n#![cfg(unix)]\nfn helper() {}\n", &[], &["unnamed: tests/planted.rs  cfg(unix)"])]
#[case::a_feature_gate_is_no_hit(
    "#[cfg(feature = \"unix-like\")]\n#[test]\nfn planted() {}\n",
    &[],
    &[]
)]
#[case::a_broken_run_is_no_hit("#[cfg(unix)]\nconst X: u8 = 0;\n#[test]\nfn planted() {}\n", &[], &[])]
fn census_finds_an_unnamed_gate_and_passes_a_named_one(
    #[case] text: &str,
    #[case] table: &[Gate],
    #[case] expected: &[&str],
) {
    let files = [("tests/planted.rs".to_owned(), text.to_owned())];
    assert_eq!(faults(&files, table), expected);
}

/// A gate naming Windows or macOS is a fault even with its row in the table, beside the control:
/// the same row over a `unix` gate passes.
#[rstest]
#[case::a_windows_test(format!("#[cfg({WINDOWS})]\n#[test]\nfn planted() {{}}\n"), WINDOWS, "planted")]
#[case::a_windows_file(format!("#![cfg({WINDOWS})]\nfn helper() {{}}\n"), WINDOWS, "")]
#[case::a_macos_test(
    "#[cfg(not(target_os = \"macos\"))]\n#[test]\nfn planted() {}\n".to_owned(),
    "not(target_os = \"macos\")",
    "planted"
)]
#[case::a_spaced_macos_test(
    "#[cfg(target_os=\"macos\")]\n#[test]\nfn planted() {}\n".to_owned(),
    "target_os=\"macos\"",
    "planted"
)]
fn census_reads_a_gate_naming_another_os_as_a_fault_even_when_named(
    #[case] text: String,
    #[case] gate: &'static str,
    #[case] test: &'static str,
) {
    let files = [("tests/planted.rs".to_owned(), text)];
    let row = Gate {
        file: "tests/planted.rs",
        test,
        gate,
        reason: "planted",
    };
    assert_eq!(
        faults(&files, &[row]),
        [format!("another OS: tests/planted.rs {test} cfg({gate})")]
    );
    let control = [(
        "tests/planted.rs".to_owned(),
        "#[cfg(unix)]\n#[test]\nfn planted() {}\n".to_owned(),
    )];
    assert!(faults(&control, &[PLANTED_ROW]).is_empty());
}
