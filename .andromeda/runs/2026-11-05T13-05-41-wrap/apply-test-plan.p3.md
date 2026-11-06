@apply_row n=1 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g2.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g2.md ref_n=5 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g2.md ref_n=6 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=5
@apply_row n=5 ref_file=fanout-test-plan--cascade-g2.md ref_n=8 v=applied edit=E4
@apply_row n=6 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=1 v=applied edit=E5
@apply_row n=7 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E6
@apply_row n=8 ref_file=fanout-test-plan--cascade-g3.md ref_n=2 v=applied edit=E7
@apply_row n=9 ref_file=fanout-test-plan--cascade-g3.md ref_n=4 v=applied edit=E8
@apply_row n=10 ref_file=fanout-test-plan--cascade-g3.md ref_n=7 v=applied edit=E9
@apply_row n=11 ref_file=fanout-test-plan--cascade-g3.md ref_n=9 v=applied edit=E10
@apply_row n=12 ref_file=fanout-test-plan--cascade-g3.md ref_n=10 v=applied edit=E11
@apply_row n=13 ref_file=fanout-test-plan--cascade-g3.md ref_n=11 v=applied edit=E12
@apply_row n=14 ref_file=fanout-test-plan--cascade-g3.md ref_n=14 v=applied edit=E13
@apply_row n=15 ref_file=fanout-test-plan--cascade-g3.md ref_n=15 v=applied edit=E14
@apply_row n=16 ref_file=fanout-test-plan--cascade-g3.md ref_n=17 v=applied edit=E15
@apply_row n=17 ref_file=fanout-test-plan--cascade-g3.md ref_n=18 v=folded_into to_file=fanout-test-plan--cascade-g3.md to_n=17
@fact id=fct-o57nkgzg expect_rev=1
old: with the Windows `\\.\pipe\` form and the Unix per-user 0700 directory forms `$XDG_RUNTIME_DIR/viola/viola-<h12>.sock` (Linux) and `$TMPDIR/viola/viola-<h12>.sock` (macOS), plus
new: in the per-user 0700 directory form `$XDG_RUNTIME_DIR/viola/viola-<h12>.sock`, plus
old: `peer_euid_admitted(observed, own, reported)` with injected values
new: `peer_euid_admitted(observed, own)` (`observed == Some(own)`) with injected values
old: cannot read is refused where the euid is reported (`PEER_EUID_REPORTED`, `cfg!(unix)`), held by
new: cannot read is refused, held by
old: The connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), asserted on each OS by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature); `SERVER_PID_REPORTED` is false on macOS.
new: The Tokio twins of those two cases, `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature), hold `TokioClient::server_pid`.
old: `#[cfg(windows)]` handle non-inheritance, which backs E2 and the §12 open question. After the `viola-channel` server creates a pipe instance and a client connects, `GetHandleInformation` (windows-sys) on both handles (via `AsRawHandle`) reports `HANDLE_FLAG_INHERIT` clear. This is the only Windows evidence; there is no child-side probe.
new: The connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid`, is read by `peer_creds().pid()` (Linux `SO_PEERCRED`), held by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`).

@fact id=fct-vbcc7bad expect_rev=1
old: from the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`, pinned by the decision table `server_verdict_rows`; where no peer pid is reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap.
new: from the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start)`, pinned by the decision table `server_verdict_rows`.
old: the content-bearing paths the instance check names below: `unix_read_verdict(mode, uid, euid)` is the write rule plus a refusal of `mode & 0o077` (`unix_read_verdict_needs_the_write_rule_and_no_foreign_bit`), and `windows_read_verdict` is the write rule plus a refusal of any allow ACE granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to an untrusted SID, inherit-only ACEs included (`windows_read_verdict_refuses_any_read_right_to_another_sid`, `windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule`, and the `cfg(windows)` `read_rights_are_the_named_flags`).
new: the content-bearing paths the instance check names below.
old: Windows strict-modes feeds synthetic security descriptors built from SDDL strings (via windows-sys) to the descriptor-check function. This covers a foreign-owner SID, a NULL DACL, an inherit-only GENERIC_WRITE ACE, a read grant on `diagnostics/`, and `CREATOR OWNER` treated as the user. These are the stubs for the cases hosted runners cannot provision (test-scope Sec 1). A Users write grant on an instance tree and a Users read grant on `diagnostics/` are also made for real on `windows-2025` (the instance check, below).
new: The read rule is `unix_read_verdict(mode, uid, euid)`: the write rule plus a refusal of `mode & 0o077` (`unix_read_verdict_needs_the_write_rule_and_no_foreign_bit`).
old: its `events.ndjson` (the write rule on Unix, the read rule on Windows);
new: its `events.ndjson` (the write rule);
old: is unreadable. Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk). On Windows the instance tree is covered by `cfg(windows)` cases green on `windows-2025` (as measured at ci#40403389629): a tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` refuses a home made with `create_dir_all` before viola, its refused entry an inherited write grant to a SID strict-modes does not trust, whichever SID holds it (its oracle below).
new: is unreadable.
old: The plain-home case's Windows oracle is the `cfg(windows)` test helper `strict::win::tests::inherited_untrusted_writes(home)`: the allow ACEs that are inherited, carry a `WRITE_RIGHTS` bit and name a SID `trusted_sid` does not trust, so it reads the grant by its right whichever SID holds it; its control, a home `instance_tree()` makes, reads none, so the oracle cannot pass vacuously (green on `windows-2025`, as measured at ci#40439188053).
new: Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the instance check's refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk).
old: a file to the read rule on Unix and the write rule on Windows, and an absent path passes
new: a file to the read rule, and an absent path passes
old: by `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found` and, off Unix, `check_socket_dir_off_unix_reads_nothing`;
new: by `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`;
old: to `check_path` on every OS, held by `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`.
new: to `check_path`, held by `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`.
old: `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).
new: `stamps_error_messages_are_fixed` and `pin_key_is_the_version_and_the_content_key`.
old: (0700 / 0600 on Unix, the mode set before any byte)
new: (0700 / 0600, the mode set before any byte)
old: and the forced-window cases `place_private_new_placed_copy_execs_at_once` (Unix) and `place_private_new_placed_copy_runs_at_once` (Windows) (§5);
new: and the forced-window case `place_private_new_placed_copy_execs_at_once` (§5);
old: its outcome a whole-file compare's on every OS for an equal
new: its outcome a whole-file compare's for an equal
old: runs on Unix only, Windows using a read-only file.
new: runs on Unix only.

@fact id=fct-hpqcf7r7 expect_rev=1
old: (the verdict `verified` reads `LedgerRow::relied()`: 24 rows on Unix, 21 on Windows, `agents-join` then `local-command-compact` appended last, both relied on every OS).
new: (the verdict `verified` reads `LedgerRow::relied()`: all 24 rows on every build, `agents-join` then `local-command-compact` appended last).
old: reads unverified on every OS, one of the eighteen, `resume-session` and `agents-join` too, and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` verifies on Windows only.
new: reads unverified, as do one of the eighteen, `resume-session` and `agents-join` and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact`.
old: the 11-name identity floor measured on the Windows host (
new: the 11-name identity floor (
old: every inherited `CLAUDE*` name is removed (ASCII case-insensitive on Windows, exact on Unix) unless it is in the persistent set, and
new: every inherited `CLAUDE*` name is removed by exact name match unless it is in the persistent set (`config.json` `claude_env_keep`), and
old: A Linux dev-host reading on `claude` 2.1.287 stands beside the Windows one and differs (12 names
new: A Linux dev-host reading on `claude` 2.1.287 differs from the floor (12 names
old:   - npm-shim → `claude.exe` resolution over fixture shim files in a tempdir.
new:   - `resolve_program`: a given path as given, a bare name on `PATH`.
old: `AgentError` holding `BatchScriptChild` and `ProgramNotFound`, their messages and outputs held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.
new: `AgentError` holding `ProgramNotFound`, its message and outputs held by `refusal_messages_are_fixed` and `tests/cli_verify.rs`.

@fact id=fct-id225cf7 expect_rev=1
old: (`src/main.rs`) runs on every OS under nextest:
new: (`src/main.rs`) runs under nextest:
old: (`assert_refused_inside`; compiled out on macOS, §2 Unit)
new: (`assert_refused_inside`)
old: stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, so these two run on macOS.
new: stands in the test module `refused_seen`.

@fact after=fct-id225cf7 as=n1
- **viola-e2e harness:**
  - `Workspace::ensure_e2e_home` (`crates/viola-e2e/src/harness/mod.rs`) keeps its signature for every caller and passes the free-bytes reading to a private function that takes it as an argument, so the unit cases `harness::tests::e2e_home_*` drive it on an injected figure, the below-floor refusal among them; the floor `HEADROOM_FLOOR_MIB`, `headroom_refusal` and its message `e2e-home backing: {free} MiB free, below the {floor} MiB floor` hold, with no env, flag, feature or config key.

@fact id=fct-hctsd5xl expect_rev=1
old: (obs-plan multi-platform item, all 3 OSes)
new: (obs-plan multi-platform item)
old: 8 in `detail-hook.ndjson`, on all 3 CI OSes)
new: 8 in `detail-hook.ndjson`, on `ubuntu-latest`)
old: each over 4 096 B, on all 3 CI OSes)
new: each over 4 096 B, on `ubuntu-latest`)

@fact id=fct-b4mgg72y expect_rev=1
old: in recorded, oversize and malformed modes. The Windows npm shim is a fixture `.cmd` shim next to a stub `claude.exe`.
new: in recorded, oversize and malformed modes.

@fact id=fct-ouc4a2b7 expect_rev=1
old: On Unix the directory is also checked before use:
new: The directory is also checked before use:
old: (§4 viola-state). On Windows the pipe DACL is set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` and read back as Windows' own canonical SDDL, `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`: protected, exactly two allow ACEs, the user and SYSTEM. The user SID may render as an alias (`LA` for the built-in Administrator, as measured in CI run 36313377307), so the test compares the canonical form, never the literal it set. The pipe's admission itself — remote clients rejected, an anonymous open denied — is asserted in §6 Security control negatives (`tests/security_negatives_channel.rs`). `viola_channel::Client::connect`
new: (§4 viola-state). `viola_channel::Client::connect`
old: jsonschema 0.57.0, windows-sys 0.61.2
new: jsonschema 0.57.0

@fact id=fct-vmzc7an2 expect_rev=1
old: Real ConPTY / openpty spawn
new: Real openpty spawn
old: 6/6 green after) — and a `.cmd` child is refused (`#[cfg(windows)]`).
new: 6/6 green after).
old: waits for its `size 120x40` line: 0 of 200 lost on the windows-2025 runner, ci#36529038462). Measured limit these tests do not cover (H2, chunk 2026-09-29-h2-conpty-resize-probe): a key written right after the resize returned was lost in 13 of 200 runner iterations (ci#36527891850), every loss with the resize seen, the key flushed and `dsr-cpr 0` — lost between the ConPTY input-pipe write and the Rust test child's console read, cause within that span not established; whether the real `claude` (Node/libuv, `ReadConsoleInputW`) loses a key typed right after a resize is unmeasured, owned by the working-route entry "Windows-only live measurements", blocked on an interactive Windows host (`.claude/docs/gotchas.md`); the harness's `run --local-live` does not claim it.
new: waits for its `size 120x40` line).
old: With and without the sideloaded ConPTY (chunk 2026-09-29-sideloaded-conpty), in one run on the windows-2025 runner (image `windows-2025-vs2026` 20260828.587, ci#36563868040, the measurement-only race of the pre-`dce98ad` shape, removed with its loop before the wrap): sideloaded 0 of 200 lost, inbox 14 of 200 — no rate claimed. On Windows x64 every root test that starts `viola run` hosts its child on the sideloaded `bin/<key>/conpty/OpenConsole.exe` (`pty_backend` `conpty-sideload`); viola-pty's own tests, `tests/support/outer_pty.rs`'s outer PTY and the harness `supervise::spawn_in_pty` stay on the inbox ConPTY. The sideload boundary: the root integration binary `conpty_sideload` (the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`, and `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion its owner/DACL check refuses leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed` and the exit unchanged) and viola-pty's two-sided unit test `restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load`. Under the sideload the child's start waits for a DA1 answer through viola's pump, so the piped test driver (`tests/support/piped.rs` `Piped`) answers DA1 the way a terminal would, and the forced-window resize test
new: The forced-window resize test

@fact id=fct-okfymegs expect_rev=1
old: and green on `ubuntu-latest` and `macos-latest`; its Windows form `place_private_new_placed_copy_runs_at_once` is green-only on `windows-2025` (as measured at ci#40439188053), the red leg read on the Linux dev host only.
new: and green on `ubuntu-latest`.

@fact id=fct-5qxwlgko expect_rev=2
old: (the `endpoint_gone` reading on Unix, §3 `cleanup` step 4)
new: (the `endpoint_gone` reading, §3 `cleanup` step 4)
old: on both Unix runners, `ubuntu-latest` and `macos-latest`,
new: on `ubuntu-latest`,

@fact id=fct-qmrx4mno expect_rev=1
old: This is not a third carve-out
new: This is not a second carve-out
old: inside the scans. One carve-out (Windows x64, chunk 2026-09-29-sideloaded-conpty, the operator's ruling at that wrap): `Wrapper::boot` and every piped or outer-PTY `viola run` start first seed `<home>/bin/<key>/conpty/{OpenConsole.exe,conpty.dll}` through `tests/support/home.rs` `seed_conpty` (hard links, copy fallback, from one per-run copy under `target/conpty-seed/<key>/`, byte-identical to the embedded companions), so for them the test creates the home before viola runs, making it and the seed dir `target/conpty-seed/<key>/` through viola's own `viola_state::fs::create_private_dir` (owner-only modes and DACL).
new: inside the scans.
old: Such a home passes the instance check on Windows, where a home a test makes with plain `create_dir_all` before viola is refused `Writable`, the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (as measured on `windows-2025`: ci#40439188053 for the refusal, by `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, the plain home's canonical SDDL not recorded; ci#40403389629 for the seeded homes, the three `hook_statusline` cases green). A case that drives a check-guarded path on every OS starts
new: A case that drives a check-guarded path starts
old: (§7 Seed strategies). The reason for the seed is the first start's companion write, as measured at that chunk on the dev host under a parallel suite: a median 1 193 ms, which pushed first-start tests past their 7 s waits; no bound was raised. The product's first-start write stays covered by the unseeded starts: `tests/conpty_sideload.rs` and `run_viola_unseeded` never seed. A second carve-out (chunk
new: (§7 Seed strategies). One carve-out (chunk
old: so the report reaches the JUnit. That state is the wrapper's last `run-<name>.ndjson` event; on macOS `/usr/bin/sample <pid> 1`'s call-graph stack lines (bounded at 4 s, at most 120 lines); on Linux `/proc/<pid>/wchan` and the `stat` state field (`note_stall`; `wrapper_pid` by one sysinfo read; `process_state` per OS, a no-op elsewhere). It signals nothing and adds nothing to a passing boot. It is the witness for the macOS readiness stall inside `run.start`, whose phase is not yet named (route WATCH).
new: so the report reaches the JUnit.
old: **DA1 answer:** under the sideloaded ConPTY the piped test driver (`tests/support/piped.rs` `Piped`) answers a DA1 query `ESC[c` with `ESC[?1;0c`, as a terminal would; viola sends no DA1 answer of its own.
new: That state is the wrapper's last `run-<name>.ndjson` event, its `/proc/<pid>/wchan` and the `stat` state field (`note_stall`; `wrapper_pid` by one sysinfo read; `process_state` per OS, a no-op elsewhere).
old: **Runner-inherited grants:** on one image (`windows-2025-vs2026` 20261021.278.1) `windows-2025` runners differ by host in a temp dir's inherited ACL: on one (westus3) Users holds the write, on another (northcentralus) Authenticated Users (`S-1-5-11`, mask `0x001301BF`) holds it and Users read only (as measured at ci#40403389629 and ci#40425866096). An oracle for a runner-inherited grant therefore names no SID: it reads the grant by its right, beside a control that cannot pass vacuously (§4 viola-state, the plain-home oracle).
new: It signals nothing and adds nothing to a passing boot.

@fact id=fct-ksdwqwkf expect_rev=1
old: After the close, Windows reads 0 bytes, Linux may see ECONNRESET, and on macOS the sender's tail write may get EPIPE (as measured in CI run 36313377307) or ENOTCONN (os error 57, as measured in CI run 36482322449), whichever state the close reached first; the test
new: After the close, Linux may see ECONNRESET; the test
old: the hook exits 0 with no body, on Linux and Windows: the CLI half
new: the hook exits 0 with no body: the CLI half
old: (§6 Security sweep). macOS reports no peer pid, so there the pid and start-time comparison is absent, the named carve-out, owned by no route entry, and the server-identity negatives stay compiled out; its peer euid and socket directory checks are built, a refused euid reading `server-verify-failed` and a refused directory `socket-dir-refused`. The MCP half
new: (§6 Security sweep). The MCP half
old: is `server-verify-failed` (Linux, Windows and macOS, where on macOS it means an euid mismatch alone), `Connect`
new: is `server-verify-failed`, `Connect`
old: an endpoint no one bound, because a Windows `Serving` has no `Drop` and its pipe lives in the accept thread until the process exits.
new: an endpoint no one bound.

@fact id=fct-bt3npe7x expect_rev=1
old: the mode or ACL as found (`tests/security_negatives_strict_modes.rs`, all 3 OSes)
new: the mode as found (`tests/security_negatives_strict_modes.rs`)
old: the widening as found; its home starts unseeded, because a seeded home's companions are hard links to one shared seed file that an inheritable grant on the home or `bin/` would reach.
new: the widening as found.
old: (Unix 0750, Windows a Users read grant) refuses `viola run` on all 3 OSes:
new: (0750) refuses `viola run`:
old: `a_home_another_user_may_read_refuses_run`). The same home refuses `viola list`, held to the read rule on all 3 OSes by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`), with and without an instance, the refusal the whole board's, `ScanRefused::Home`, and opening nothing.
new: `a_home_another_user_may_read_refuses_run`).
old: Windows `--home` outside the profile: `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (`tests/security_negatives_strict_modes.rs`, `cfg(windows)`) reads the DACL of the home `viola verify` made under `target/e2e-home` back through `icacls <home> /save`, since `Get-Acl` printed nothing for a protected DACL, and requires it protected with exactly `(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)`, the user written `LA` for a user SID ending `-500`, as canonical SDDL spells the local Administrator (as measured at chunk 2026-10-26-home-and-code-bearing-file-integrity on `windows-2025`, ci#40400887151: `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`).
new: The same home refuses `viola list`, held to the read rule by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`), with and without an instance, the refusal the whole board's, `ScanRefused::Home`, and opening nothing.

@fact id=fct-lzdm6e7w expect_rev=2
old: and, on Unix, its probe status line
new: and its probe status line
old: `MM` the relied count (`LedgerRow::relied()`: 24 on Unix, the last `[24/24] local-command-compact …`; 21 on Windows, the last `[21/21] local-command-compact …`; `STEPS_PASS` and `ROW_IDS` are `[&str; 24]` on Unix and `[&str; 21]` on Windows, `STATUSLINE_ROWS` the three statusline ids, the per-OS literals under `cfg(unix)` / `cfg(windows)`)
new: `MM` the relied count (`LedgerRow::relied()`: all 24 rows on every build, the last `[24/24] local-command-compact …`; `STEPS_PASS` and `ROW_IDS` are `[&str; 24]`, `STATUSLINE_ROWS` the three statusline ids, the literals ungated)
old: `local-command-clear` and, on Unix, the three statusline rows
new: `local-command-clear` and the three statusline rows
old: `stamped 2.1.0  19 pass  5 fail` on Unix, `19 pass  2 fail` on Windows)
new: `stamped 2.1.0  19 pass  5 fail`)
old: its one probe `--settings` on Unix only
new: its one probe `--settings`
old: (`21 pass  3 fail` on Unix, `18 pass  3 fail` on Windows,
new: (`21 pass  3 fail`,
old: (`19 pass  5 fail` on Unix, `16 pass  5 fail` on Windows,
new: (`19 pass  5 fail`,
old: (`ROW_IDS: [&str; 24]`, the three statusline ids relied on Unix only, `resume-session`, `agents-join` and `local-command-compact` relied on every OS)
new: (`ROW_IDS: [&str; 24]`, every row relied on every build)
old: (`LedgerRow::relied()`'s length: 24 on Unix, 21 on Windows)
new: (`LedgerRow::relied()`'s length, 24)
old: `batch-script-child`, `program-not-found`,
new: `program-not-found`,
old: back it; on Windows the console routine is called directly (three cases on `windows-2025`), and real console event delivery is owed to "Windows-only live measurements". A trycmd case
new: back it. A trycmd case
old: `verify_under_a_regular_file_refuses_the_home` has a per-OS oracle: on Unix a home under a regular file cannot be statted and is refused with exit 1 and the home pair; on Windows the same stat reads `NotFound`, an absent home the check skips, so the home's creation fails at the `cli` catch site's one line.
new: `verify_under_a_regular_file_refuses_the_home`: a home under a regular file cannot be statted and is refused with exit 1 and the home pair.
old: forces that catch-site line on every OS with
new: forces the `cli` catch site's one line with
old: is a guard on Windows and on Unix, a `SIGSTOP` on Unix; the cases that stand on it are `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (every OS), `run_refuses_a_stale_name` (Unix) and `run_refuses_a_frozen_wrapper_as_stale_on_windows` (Windows) in `tests/cli_instance_state.rs`, and `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (Unix) in
new: is a `SIGSTOP` guard; the cases that stand on it are `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` and `run_refuses_a_stale_name` in `tests/cli_instance_state.rs`, and `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` in
old: for clap's own output, all 3 OSes:
new: for clap's own output:
old: never the screen text — ConPTY adds its own `ESC[m`)
new: never the screen text)
old: of `send` and `answer`, on all 3 OSes.
new: of `send` and `answer`.
@pattern id=tp3-01
fixed: with the Windows `\\.\pipe\` form
@pattern id=tp3-02
fixed: reports `HANDLE_FLAG_INHERIT` clear
@pattern id=tp3-03
fixed: where no peer pid is reported
@pattern id=tp3-04
fixed: a refusal of any allow ACE granting a right in `READ_RIGHTS`
@pattern id=tp3-05
fixed: feeds synthetic security descriptors built from SDDL strings
@pattern id=tp3-06
fixed: the write rule on Unix, the read rule on Windows
@pattern id=tp3-07
fixed: verifies on Windows only
@pattern id=tp3-08
fixed: 21 on Windows
@pattern id=tp3-09
fixed: ASCII case-insensitive on Windows
@pattern id=tp3-10
fixed: resolution over fixture shim files
@pattern id=tp3-11
fixed: compiled out on macOS
@pattern id=tp3-12
fixed: on all 3 CI OSes
@pattern id=tp3-13
fixed: The Windows npm shim is a fixture
@pattern id=tp3-14
fixed: the pipe DACL is set from
@pattern id=tp3-15
fixed: a `.cmd` child is refused
@pattern id=tp3-16
fixed: lost on the windows-2025 runner
@pattern id=tp3-17
fixed: is green-only on `windows-2025`
@pattern id=tp3-18
fixed: on both Unix runners
@pattern id=tp3-19
fixed: answers a DA1 query
@pattern id=tp3-20
fixed: call-graph stack lines
@pattern id=tp3-21
fixed: Windows reads 0 bytes
@pattern id=tp3-22
fixed: macOS reports no peer pid
@pattern id=tp3-23
fixed: a Windows `Serving` has no `Drop`
@pattern id=tp3-24
fixed: the mode or ACL as found
@pattern id=tp3-25
fixed: refuses `viola run` on all 3 OSes
@pattern id=tp3-26
fixed: `[&str; 21]` on Windows
@pattern id=tp3-27
fixed: the console routine is called directly
@pattern id=tp3-28
fixed: has a per-OS oracle
@pattern id=tp3-29
fixed: a guard on Windows and on Unix
@pattern id=tp3-30
fixed: ConPTY adds its own
@pattern id=tp3-31
fixed: runs on every OS under nextest
@pattern id=tp3-32
fixed: first seed `<home>/bin/<key>/conpty/
