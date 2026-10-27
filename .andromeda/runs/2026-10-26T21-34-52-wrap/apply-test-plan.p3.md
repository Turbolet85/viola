# Apply — test-plan · part 3 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 18 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:22#1
- fanout-test-plan--cascade-g1.md 19 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:25+#1
- fanout-test-plan--cascade-g1.md 20 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:32#1, .andromeda/test-plan-detail/integration-test-strategy.md:33#2
- fanout-test-plan--cascade-g1.md 21 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:33#1
- fanout-test-plan--cascade-g1.md 22 · folded into fanout-test-plan--cascade-g1.md 20
- fanout-test-plan--cascade-g1.md 23 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:48+#1
- fanout-test-plan--cascade-g1.md 24 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:58#1, .andromeda/test-plan-detail/integration-test-strategy.md:58#2
- fanout-test-plan--cascade-g1.md 25 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:59#1
- fanout-test-plan--cascade-g1.md 26 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:59+#1
- fanout-test-plan--cascade-g1.md 27 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:59+#2
- fanout-test-plan--cascade-g1.md 28 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:59+#3
- fanout-test-plan--cascade-g1.md 29 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:71+#1
- fanout-test-plan--cascade-g1.md 30 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:81#1

## Edits
- line: .andromeda/test-plan-detail/integration-test-strategy.md:22
  old: «`conpty_sideload` (5 cases: the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`)»
  new: «`conpty_sideload` (the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`, and `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion its owner/DACL check refuses leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed` and the exit unchanged)»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:25
  anchor: «  - **Test approach:** `viola hook <event>` fed fixture stdin with `VIOLA_NAME`/`VIOLA_DIR` against a live test wrapper. The event line, the `diagnostics/hook-<name>.ndjson` trace and `budget.json` are asserted.»
  add: « The event and dialog paths run the instance check before any `config.json` read or obs init, and a `VIOLA_DIR` naming no existing instance directory canonicalises to no session, so the hook writes nothing: in `tests/hook_fail_open.rs` the cases that expect lines run in a viola-made instance dir (`Env::Made`), the `no_instance_dir` row pins the silence, and the widened-instance refusal is one rstest function over the events (`hook_refused_by_the_instance_check_fails_open_and_writes_nothing`: `stop`, `session-end`, `pre-tool-use`, `permission-request`), not a row of each matrix.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:32
  old: «so for them the test creates the home before viola runs.»
  new: «so for them the test creates the home before viola runs, making it and the seed dir `target/conpty-seed/<key>/` through viola's own `viola_state::fs::create_private_dir` (owner-only modes and DACL).»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:33
  old: «Such a home fails `hook statusline`'s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home's DACL not read), so a case»
  new: «Such a home passes the instance check on Windows, where a home a test makes with plain `create_dir_all` before viola is refused `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE (as measured at chunk 2026-10-26-home-and-code-bearing-file-integrity on `windows-2025`: ci#40398601594 for the refusal, by `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, the plain home's canonical SDDL not recorded; ci#40403389629 for the seeded homes, the three `hook_statusline` cases green). A case»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:33
  old: «never seed. When viola sets its own home DACL at creation (the Epoch 6 entry "Home and code-bearing file integrity"), a seeded home skips it, so that entry owns making the seed let viola create the home first (route CARRY). A second carve-out»
  new: «never seed. A second carve-out»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:48
  anchor: «  - the MCP impostor case `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (`crates/viola-mcp/tests/handlers.rs`, a held process as the recorded pid) drives the transport-free `Tools` handle, no rmcp client: MCP `send`, `wait`, `last` and `answer` against a server that is not the recorded wrapper each get `isError` with `{"error":"instance-unreachable","detail":null}` and text `refused: instance-unreachable`; no frame reaches the server and no log line is written.»
  add: «  - the MCP widened-instance case `handlers_frame_tools_refuse_a_widened_instance_before_any_frame` (`crates/viola-mcp/tests/handlers.rs`) drives the MCP tools over stdio after an `initialize` handshake: with the instance directory widened and its recorded wrapper serving, MCP `send`, `wait`, `last` and `answer` each get `isError` with `{"error":"instance-unreachable","detail":null}` before any snapshot read or connection; no frame reaches the wrapper, no line is written and the widening is left as found.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:58
  old: «exit 1 for `run`/`mcp` and for `viola revive` (the start arm and `--list` alike»
  new: «exit 1 for `run`/`mcp`, for the start home-tree check of `run` and of `viola revive` (the home pair, or the instance pair for an instance-level refusal, and under `revive --json` the `strict-modes-failed` document; made before the wrapper log opens, so no line and no child), for `viola verify`'s home-tree check (the home pair before any step: no step line, no line, `stamps.json` unchanged) and for `viola revive` (the start arm and `--list` alike»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:58
  old: «however often it is invoked), CLI 21 for every other verb»
  new: «however often it is invoked; the event and dialog paths, their instance refused or absent, read no `config.json`, open no role file, read no stdin and write no frame, no line and no SessionEnd append), MCP frame tools `instance-unreachable` with `detail: null` and no frame or line, CLI 21 for every other verb»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:59
  old: «a check placed after it would pass a 0770 home»
  new: «a check placed after it would pass a 0770 home. `viola send`, `wait`, `last`, `answer`, `pause` and `release` run the same check before any frame, through the same `src/cmd/client.rs` `refuse_untrusted` the link verbs call, with the same refusal: exit 21, the instance pair or the `instance-unreachable` document, no frame, no line, the mode or ACL as found (`tests/security_negatives_strict_modes.rs`, all 3 OSes)»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:59
  anchor: «  - `viola link` / `viola unlink` run `check_instance` on the driven instance before any role file or log opens: a refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found; `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home»
  add: «  - the start and verify home-tree checks (their exits in the strict-modes item above): `tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log` widens the home, `instances/<name>`, `bin/` or the pinned copy after one clean start, and the next `viola run` is refused before its log opens with the home's or the instance's pair, no child and the widening as found; its home starts unseeded, because a seeded home's companions are hard links to one shared seed file that an inheritable grant on the home or `bin/` would reach. `tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step` widens `ledger/` on a stamped home, and the next `viola verify` is refused with the home pair, no step line or summary and `stamps.json` byte for byte.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:59
  anchor: «  - `viola link` / `viola unlink` run `check_instance` on the driven instance before any role file or log opens: a refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found; `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home»
  add: «  - a home made before viola that another user may read (Unix 0750, Windows a Users read grant) refuses `viola run` on all 3 OSes: exit 1 with the home pair, no `diagnostics/`, the grant as found (`tests/security_negatives_strict_modes.rs` `a_home_another_user_may_read_refuses_run`).»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:59
  anchor: «  - `viola link` / `viola unlink` run `check_instance` on the driven instance before any role file or log opens: a refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found; `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home»
  add: «  - Windows `--home` outside the profile: `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (`tests/security_negatives_strict_modes.rs`, `cfg(windows)`) reads the DACL of the home `viola verify` made under `target/e2e-home` back through `icacls <home> /save`, since `Get-Acl` printed nothing for a protected DACL, and requires it protected with exactly `(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)`, the user written `LA` for a user SID ending `-500`, as canonical SDDL spells the local Administrator (as measured at chunk 2026-10-26-home-and-code-bearing-file-integrity on `windows-2025`, ci#40400887151: `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`).»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:71
  anchor: « The stop's unit cases in `src/cmd/verify/stop.rs`, the `src/cmd/verify/typed/` module and `src/run/version_gate.rs` back it; on Windows the console routine is called directly (three cases on `windows-2025`), and real console event delivery is owed to "Windows-only live measurements". A trycmd case for the verify step counter arrives with a redaction-heavy consumer»
  add: «  - `tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home` has a per-OS oracle: on Unix a home under a regular file cannot be statted and is refused with exit 1 and the home pair; on Windows the same stat reads `NotFound`, an absent home the check skips, so the home's creation fails at the `cli` catch site's one line. `verify_under_a_regular_file_prints_only_the_internal_error_line` forces that catch-site line on every OS with a regular file where `ledger/` goes in a viola-made home.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:81
  old: «in `tests/cli_controls_not_disableable.rs`); the other verb negatives»
  new: «in `tests/cli_controls_not_disableable.rs`), its `--home` row the case `setting_does_not_disable_the_home_check` (`cfg(unix)`: under each `SETTINGS` row in turn, `viola send` against a 0770 home exits 21 with the instance pair before any log opens, the home's mode as found); the other verb negatives»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = '`conpty_sideload` (5 cases'

[[pattern]]
id = "tp3-02"
fixed = "the home's DACL not read"

[[pattern]]
id = "tp3-03"
fixed = "fails `hook statusline`'s instance check on Windows"

[[pattern]]
id = "tp3-04"
fixed = 'a seeded home skips it'
