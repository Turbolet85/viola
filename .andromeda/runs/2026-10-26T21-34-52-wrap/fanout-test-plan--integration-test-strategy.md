proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: '## 5. Integration Test Strategy'
    change: 'Replace «Such a home fails `hook statusline`''s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home''s DACL not read), so» with «A home a test makes before viola with `create_dir_all` is refused by the instance check on Windows as `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE (measured on windows-2025, ci#40398601594, the `cfg(windows)` case `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`; the plain home''s canonical SDDL not recorded), while a seeded home made through `create_private_dir` passes `check_instance` there and the three `hook_statusline` cases pass (ci#40403389629); so» — the rest of the sentence (`stamped_home`, §7 Seed strategies) kept.'
    sidecar: 'integration-test-strategy §5 setup lifecycle: the seeded-home refusal reading «the home''s DACL not read» replaced by the measured refusal (`Writable`, an inherited Users write ACE) and the seeded home passing `check_instance` on windows-2025 (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Spec claims disproved by measurement (integration-test-strategy.md:33 «the refused home''s DACL was not read», measured on windows-2025 ci#40398601594) and Counts 15 (pattern `seed`, :33 stale); Outcome (tests) row: `seed_conpty` homes pass `check_instance` on windows-2025 and the three `hook_statusline` cases pass; Deviation 6 (the SDDL not recorded, the refused ACE asserted instead).'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:33
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: '## 5. Integration Test Strategy'
    change: 'Delete the sentence «When viola sets its own home DACL at creation (the Epoch 6 entry "Home and code-bearing file integrity"), a seeded home skips it, so that entry owns making the seed let viola create the home first (route CARRY).» — the CARRY is closed: the seed now makes the home through viola''s own `create_private_dir` (stated in the carve-out at line 32).'
    sidecar: 'integration-test-strategy §5 setup lifecycle: the seed carve-out''s CARRY owed to «Home and code-bearing file integrity» removed as closed by that chunk.'
    rationale: 'Report Counts 15 (pattern `seed`: «a seeded home skips it, so that entry owns making the seed let viola create the home first» stale) and 1 (`owner-title`: every row naming this entry as owner of a CARRY it closed is stale); Harness / gate surface: `seed_conpty` makes the home and the seed dir with `create_private_dir`.'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:33
    dependent-of: D-tests-count-literals
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 32
    section: '## 5. Integration Test Strategy'
    change: 'Replace «so for them the test creates the home before viola runs.» with «so for them the test creates the home before viola runs, and since chunk 2026-10-26-home-and-code-bearing-file-integrity `seed_conpty` makes that home, and the seed dir `target/conpty-seed/<key>/`, through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so a seeded home holds the owner-only modes and DACL viola gives a home it creates.»'
    sidecar: 'integration-test-strategy §5 setup lifecycle: the Windows x64 seed carve-out states that `seed_conpty` makes the home and the seed dir through `create_private_dir` (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Counts 15 (pattern `seed`: «viola makes the seeded home (`create_private_dir`) before `bin/<key>/conpty/`, and the seed dir `target/conpty-seed/<key>/` too»; integration-test-strategy.md:32 re-read for the home-creation step) and Harness / gate surface.'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:32
    dependent-of: D-tests-count-literals
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: '## 5. Integration Test Strategy'
    change: 'After line 33 add the paragraph «**Seed hazard (Windows):** an inheritable `icacls` grant (`(OI)(CI)`) on a seeded home''s `bin/` (or on the home) propagates through `seed_conpty`''s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until it is removed (measured on windows-2025, ci#40398601594 and ci#40400887151); so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded, as `tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log` does (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'integration-test-strategy §5: the seed hazard added — an inheritable grant reaches every seeded home through the hard-linked seed, so a widening test starts its home unseeded.'
    rationale: 'Report Spec claims disproved by measurement, third bullet («test-plan §5 / §7''s seed carve-out (pattern `seed`) owes the hazard»), Deviation 10 and Decisions & corrections (measured on windows-2025).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: '## 5. Integration Test Strategy'
    change: 'After line 33 add the paragraph «**A home made before viola:** a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: a 0755 (Unix) or inherited-ACL (Windows) home now fails the read rule of `check_instance` and the start tree check, and the product must refuse it (chunk 2026-10-26-home-and-code-bearing-file-integrity: `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs`, `tests/tui_env_strip.rs` moved onto it).»'
    sidecar: 'integration-test-strategy §5: a test-made home goes through `create_private_dir`, since a `create_dir_all` home is refused by the read rule.'
    rationale: 'Report scope record (companions `contract_diag_schema.rs`, `cli_machine_contract.rs`, `tui_env_strip.rs`: «each made its home with `create_dir_all` (0755), which the read rule now refuses; each makes it through `viola_state::fs::create_private_dir`») and Deviation 6 (CARRY 13: a home a test made first is one the product must refuse).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'In the strict-modes refusals item replace «exit 1 for `run`/`mcp` and for `viola revive`» with «exit 1 for `run`/`mcp`/`verify` and for `viola revive`» and add after the revive parenthesis: «`run` and `revive`''s start run the start tree check (`check_home_tree`: the home, the instance set, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin dir) before the wrapper log opens, printing the home pair, or the instance pair for an instance-level refusal, with no log, line or child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`); `verify` checks the home, `ledger/`, `bin/` and `plugin/` before its log and prints the home pair with no step line, summary or `stamps.json` change (`tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step`; chunk 2026-10-26-home-and-code-bearing-file-integrity)».'
    sidecar: 'integration-test-strategy §5 On-disk: `verify` joins the exit-1 strict-modes refusals; the start tree check of `run` / `revive` and `verify` named with their root cases.'
    rationale: 'Report Symbols (`run.rs` `checked_exe` / `refuse_tree`, `verify.rs` 64-70), Counts 9 (home-pair) and 20 (no-line; integration-test-strategy.md:58 re-read for the new sites), Outcome (security) row 4, Coverage of new surfaces (`run_cli`, `cli_verify`).'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'In the strict-modes refusals item replace «hook 0 with no output (the statusline arm reads and opens nothing under the home after a refused check and writes no line, however often it is invoked)» with «hook 0 with no output (the statusline arm, and since chunk 2026-10-26-home-and-code-bearing-file-integrity the event and dialog arms, read and open nothing under the home after a refused instance check — no `config.json` read, no role file, no stdin read — and write no line, frame or SessionEnd append, however often invoked: `tests/hook_fail_open.rs` `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`, `tests/hook_statusline.rs` `hook_statusline_a_widened_instance_runs_and_logs_nothing`)».'
    sidecar: 'integration-test-strategy §5 On-disk: the hook event and dialog arms join the statusline arm''s silent refusal, with their root cases.'
    rationale: 'Report Symbols (`src/cmd/hook.rs` `canonical` / `trusted` before obs init), Count 20 (no-line, integration-test-strategy.md:58), Outcome (security) row 2, Coverage of new surfaces (`hook_fail_open`, `hook_statusline`).'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: '## 5. Integration Test Strategy'
    change: 'After line 59 add the item «  - the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` run the same check first (`client::refuse_untrusted`, shared with `link` / `unlink` since chunk 2026-10-26-home-and-code-bearing-file-integrity): on an instance another user may write or read, exit 21 with the instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`), no frame, no line and the mode or ACL as found, on all 3 OSes: `tests/security_negatives_strict_modes.rs` `frame_verb_refuses_a_widened_instance_before_any_log`»'
    sidecar: 'integration-test-strategy §5 On-disk: the six frame verbs'' instance check (`refuse_untrusted`) and its three-OS root case added.'
    rationale: 'Report Symbols (`src/cmd/client.rs` `refuse_untrusted`, called first by the six verbs; `link_call` moved onto it), Count 8 (instance-pair) and 20 (no-line), Outcome (security) row 1, Coverage of new surfaces.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: '## 5. Integration Test Strategy'
    change: 'After line 59 add the item «  - a home under a regular file takes a per-OS oracle: on Unix `verify` refuses it with the home pair (exit 1), on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home`, chunk 2026-10-26-home-and-code-bearing-file-integrity); any test that plants a regular file above a checked path owes the same per-OS oracle»'
    sidecar: 'integration-test-strategy §5 On-disk: the per-OS oracle for a home under a regular file added.'
    rationale: 'Report Count 13 (an unstattable home a strict-modes refusal on Unix; on Windows it stats `NotFound`), Deviation 8 and Decisions & corrections («a per-OS oracle is owed wherever a test plants a regular file above a checked path»).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 48
    section: '## 5. Integration Test Strategy'
    change: 'After line 48 add the item «  - the MCP instance check (chunk 2026-10-26-home-and-code-bearing-file-integrity): `handlers_frame_tools_refuse_a_widened_instance_before_any_frame` (`crates/viola-mcp/tests/handlers.rs`) — MCP `send`, `wait`, `last` and `answer` on an instance another user may write or read each get `isError` with `{"error":"instance-unreachable","detail":null}`; `check_instance` runs before the liveness read, no frame reaches the wrapper and no log line is written»'
    sidecar: 'integration-test-strategy §5 Wrapper channel: the MCP frame tools'' instance-check case added beside the impostor case.'
    rationale: 'Report Symbols (`viola-mcp` `tools.rs` `frame` 337-339: `check_instance` before `live_endpoint`, a refusal `instance-unreachable`, `detail: null`, no frame, no line), New text by line (handlers.rs 436-478), Outcome (security) row 3.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: '## 5. Integration Test Strategy'
    change: 'Replace «the root integration binary `conpty_sideload` (5 cases: the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`)» with «the root integration binary `conpty_sideload` (the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`, and since chunk 2026-10-26-home-and-code-bearing-file-integrity `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion its owner/DACL check refuses leaves the child on the inbox ConPTY with `process-start.sideload_fallback` `strict-modes-failed`, exit unchanged)» — the bare case count dropped: the code''s functions and `#[case]` rows are the count.'
    sidecar: 'integration-test-strategy §5 Module ↔ PTY: the `conpty_sideload` companion owner/DACL case added; the bare case count removed.'
    rationale: 'Report Symbols (`pin_companions` `check_present`, `PinError::StrictModes` → `sideload_fallback` `strict-modes-failed`), Count 18 (conpty-dacl) and 7 (sideload-fb), Outcome (security) row 6 (MET on windows-2025); CLAUDE.md Session Learnings (a master states no test-case count, it names the functions).'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 25
    section: '## 5. Integration Test Strategy'
    change: 'Append to the Hook exec test approach: «The instance directory is one viola made: since chunk 2026-10-26-home-and-code-bearing-file-integrity the hook runs the instance check before any line, so `tests/hook_fail_open.rs` runs its line-expecting cases in a viola-made instance dir (`Env::Made`), and a `VIOLA_DIR` naming an instance directory that does not exist writes nothing (its `no_instance_dir` row).»'
    sidecar: 'integration-test-strategy §5 Hook exec ↔ channel ↔ disk: the viola-made instance dir and the silent nonexistent `VIOLA_DIR` added.'
    rationale: 'Report Symbols (`src/cmd/hook.rs`: «A `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing»), Count 23 (`viola-dir-none` stands in no master), Deviation 5.'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 83
    section: '## 5. Integration Test Strategy'
    change: 'After line 83 add the sub-item «    - `setting_does_not_disable_the_home_check` (chunk 2026-10-26-home-and-code-bearing-file-integrity): every `SETTINGS` row is set against the start tree check, which still refuses a widened home tree.»'
    sidecar: 'integration-test-strategy §5 CLI: the controls table gains the home-check row `setting_does_not_disable_the_home_check`.'
    rationale: 'Report New text by line (`tests/cli_controls_not_disableable.rs` 576-641 `setting_does_not_disable_the_home_check`, iterating `for (setting, _) in SETTINGS`), Coverage of new surfaces (`cli_controls_not_disableable` covers the start tree check).'
note: 'The body''s §5 summary lines (.andromeda/test-plan.md:427 the seed carve-out, :437 the On-disk strict-modes refusals) carry the same facts in brief and are another detector''s file; no D-retired hit here (this file names none of the eight retired gaps nor `read_snapshot`).'
