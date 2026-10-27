proposals:
  - detector: D-tests-count-literals
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery (Verification signal, first bullet)'
    change: 'The first E5 signal reads: the replay recovers `links`, `budget_paused:true` and `agent_session_id` from the log with `dialog_pending:false` and writes no file (no snapshot is rebuilt from it); every start appends `wheel{holder:"driver", cause:"start"}`, so after step 3 the wheel is the driver, never `wheel:"human"` (as restated by the chunk 2026-10-26-home-and-code-bearing-file-integrity that built steps 1–3).'
    sidecar: 'E5 first signal restated as built: the replay writes no file and every start appends wheel{driver,start}, so wheel:"human" is no longer a signal (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 19).'
    rationale: 'Report Counts / qualifiers moved 19 (pattern wheel-human): E5 two signals restated, `wheel:"human"` does not hold after step 3 and the replay writes no file; test-plan.md:586 listed as stale.'
    basis: '.andromeda/test-plan.md:586'
  - detector: D-tests-count-literals
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Remove the set-against note («Two of the signals above are set against what landed, for the entry that builds it to restate …»): the entry built E5 and the signals above now state what landed.'
    sidecar: 'E5 set-against note removed: its two signals are restated in the signal list by the entry that built steps 1–3 (Count 19).'
    rationale: 'Report Counts / qualifiers moved 19: test-plan.md:592 is the set-against note «for the entry that builds it to restate», stale now that this chunk built steps 1–3 (Outcome: E5 steps 1–3 MET, `binary(chaos_snapshot_replay)`).'
    basis: '.andromeda/test-plan.md:592'
    dependent-of: D-tests-count-literals
  - detector: D-retired
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery (As landed)'
    change: '**As landed: built.** Steps 1–3 run as `tests/chaos_snapshot_replay.rs` (`a_start_over_an_unreadable_snapshot_replays_the_log`, `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`); the snapshot readers moved onto `read_snapshot_or_replay` (the founder at P4, inputs#I3 Q1): the product readers of the replay are now six (`viola revive`, `viola list`, hook `connect`, statusline `serve`, the collision check of `run`, `live_endpoint`) and `read_snapshot(` has no product caller under `src/`; the budget, link and `list --json` legs stay covered as before. No CARRY remains.'
    sidecar: 'E5 As landed in part → built: steps 1–3 in tests/chaos_snapshot_replay.rs; replay product readers 2 → 6, read_snapshot product callers 3 → 0; the CARRY to "Home and code-bearing file integrity" retired.'
    rationale: 'Report Retired: `read_snapshot` as a product reader under `src/` (3 → 0); Counts 5 (read-snapshot, test-plan.md:590 stale: readers 2 → 6) and 1 (owner-title: every row naming this entry as owner of a CARRY is stale); Outcome: E5 steps 1–3 MET.'
    basis: '.andromeda/test-plan.md:590'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery (Verification signal, third bullet)'
    change: 'The «No refusal of start (exit is not 1)» signal is qualified: it holds once the heartbeat is older than `STALE_AFTER`; over a replayed snapshot the collision check reads liveness from the beat alone, so a fresh beat refuses the start `already-live` (exit 1, `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`).'
    sidecar: 'E5 start signal qualified by the beat: a replayed snapshot with a fresh beat refuses already-live (collision_check reads the beat alone).'
    rationale: 'Report Symbols (src/cmd/run.rs): `collision_check` takes one read, `Replayed` → liveness from the beat alone (`beat_age ≤ STALE_AFTER` → `already-live`, else free); new test `tests/chaos_snapshot_replay.rs:112-147`; companion `tests/cli_revive.rs` ages the beat for the same reason.'
    basis: 'src/cmd/run.rs:425-432'
  - detector: D-tests-count-literals
    severity: warning
    section: '§6 E2E Test Strategy → Non-path suites: Security control negatives'
    change: 'The single-writer clause reads: single writers of `stamps.json` and `snapshot.json`, byte-identical after every other writer-capable entry point with the wrapper stopped (`Wrapper::stop`, never SIGSTOPped; `tests/security_negatives_strict_modes.rs` `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`); and the bullet adds the strict-modes negatives of the same file on three OSes: each of the six frame verbs refusing a widened instance with exit 21, the instance pair or `instance-unreachable` document, no frame and no line; a start under umask 000 leaving every home path owner-only (Unix); and a pre-existing home another user may read refusing `viola run` exit 1.'
    sidecar: 'Security control negatives: single writers witnessed with the wrapper stopped, not SIGSTOPped; the strict-modes negatives of tests/security_negatives_strict_modes.rs listed.'
    rationale: 'Report Counts 16 (single-writer: test-plan.md:606 stale, the method) and Outcome (tests): single writers byte-identical after every other writer-capable entry point, the wrapper stopped; new file `tests/security_negatives_strict_modes.rs` (Coverage of new surfaces: `refuse_untrusted` at the six verbs, integ three OSes).'
    basis: '.andromeda/test-plan.md:606'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Non-path suites: Chaos suite'
    change: 'The chaos suite list adds the E5 start over a corrupted snapshot (`tests/chaos_snapshot_replay.rs`: a truncated or `v:99` snapshot replayed from the log at a start, one `state-recovered` line, and a fresh beat over the replayed snapshot refusing `already-live`).'
    sidecar: 'Chaos suite gains tests/chaos_snapshot_replay.rs (E5 steps 1–3).'
    rationale: 'Report Changes Files and Crates / modules: `tests/chaos_snapshot_replay.rs` is a new root test binary (147 lines); Coverage of new surfaces: the four readers on the replay → integ `chaos_snapshot_replay` (E5 steps 1–3).'
    basis: '.andromeda/test-plan.md:613'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Non-path suites: Exit-cause matrix'
    change: 'Beside the server-verification sentence: the strict-modes cause of exit 21 for `send` / `wait` / `last` / `answer` / `pause` / `release` (and `link` / `unlink`) is a case of `tests/security_negatives_strict_modes.rs`, and the exit-1 home-tree refusals of `run` and `verify` are cases of `tests/run_cli.rs` (`run_refuses_a_widened_home_tree_before_its_log`) and `tests/cli_verify.rs` (`verify_refuses_a_widened_ledger_before_any_step`).'
    sidecar: 'Exit-cause matrix: the new strict-modes causes (six frame verbs exit 21; run / verify exit 1) named by their test files until cross_exit_causes.rs lands.'
    rationale: 'Report Symbols (`refuse_untrusted`, `checked_exe` / `refuse_tree`, verify.rs `refuse_home`) and Counts 8 / 9: new exit-21 and exit-1 refusal causes; New text: tests/run_cli.rs:619-674, tests/cli_verify.rs:251-280.'
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → Cross-module patterns covered: On-disk'
    change: 'Strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `verify` (the home tree: the home, `ledger/`, `bin/`, `plugin/`), hook 0 with no output, and CLI 21 for every other verb (the six frame verbs printing the instance pair).'
    sidecar: 'On-disk cross-module pattern: verify joins the exit-1 strict-modes refusals.'
    rationale: 'Report Symbols (src/cmd/verify.rs): `check_home_tree(&home, None, None)` before its `cli` log, refused → home pair, exit 1; Count 9 (`run` and `verify` print the home pair, exit 1).'
    basis: 'src/cmd/verify.rs:64-70'
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → Cross-module patterns covered: Hooks'
    change: 'The hooks bullet list adds: a `VIOLA_DIR` instance refused by the instance check, or naming an instance directory that does not exist, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append (`hook_fail_open` `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`, `hook_statusline` `hook_statusline_a_widened_instance_runs_and_logs_nothing`).'
    sidecar: 'Hooks cross-module pattern: a refused or absent instance means an immediate silent exit 0.'
    rationale: 'Report Symbols (src/cmd/hook.rs): `canonical` / `trusted` before obs init, a refused or absent instance returns exit 0 at once; Count 23 (a `VIOLA_DIR` naming no directory writes nothing, a fact no master states); New text tests/hook_fail_open.rs:477-519, tests/hook_statusline.rs:283-301.'
    basis: 'src/cmd/hook.rs:83-86'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Non-path suites: Security sweep'
    change: 'The hook fail-open matrix adds a widened instance refused by the instance check (exit 0, no body, no line) beside the missing `VIOLA_NAME` case.'
    sidecar: 'Security sweep: hook fail-open matrix gains the refused-instance case.'
    rationale: 'Same fact as the §5 Hooks proposal (report Symbols, src/cmd/hook.rs; Outcome: `viola hook` event/dialog refused by the instance check MET); the security sweep restates the hook fail-open matrix.'
    dependent-of: D-tests-coverage
  - detector: D-tests-framework
    severity: warning
    section: '§5 Integration Test Strategy → Driver(s): Windows DACL read-back'
    change: 'Windows DACL read-back: windows-sys 0.61.2, plus `icacls` for the root tests: `tests/support/home.rs` `widen` / `grant_read` / `unwiden` / `still_widened` set and read grants through `icacls` with its exit asserted, and the protected DACL of a `--home` outside the profile is read back through `icacls <home> /save <file>` (canonical SDDL, the RID-500 user spelled `LA`; `Get-Acl` printed nothing).'
    sidecar: 'Integration drivers: icacls joins windows-sys for Windows grants and the protected-DACL read-back (measured on windows-2025).'
    rationale: 'Report Harness / gate surface: `tests/support/home.rs` gains `widen` · `grant_read` · `unwiden` · `still_widened` (Windows `icacls`); Cross-project claims: the protected DACL measured `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` on windows-2025; Deviation 10: read back through `icacls /save`.'
    basis: 'tests/support/home.rs:317-389'
  - detector: D-tests-count-literals
    severity: warning
    section: '§3 → Test data bootstrap'
    change: 'The Windows x64 seed bullet adds: `seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir`; its companions are hard links to one shared seed file, so an inheritable grant on a seeded home (or its `bin/`) reaches the shared companion and every seeded home — a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded.'
    sidecar: 'Test data bootstrap: seed_conpty creates the home and seed dir private; the hard-link grant hazard and the unseeded rule for widening tests recorded (measured on windows-2025).'
    rationale: 'Report Counts 15 (seed: test-data-bootstrap.md:11 re-read for the home-creation step), Harness / gate surface (`seed_conpty` makes the home and the seed dir with `create_private_dir`) and Spec claims disproved by measurement (the inheritable `icacls` grant propagating through the hard links; «test-plan §5 / §7 seed carve-out owes the hazard»).'
    basis: '.andromeda/registries/contracts/test-plan/test-data-bootstrap.md:11'
  - detector: D-tests-count-literals
    severity: warning
    section: '§5 Integration Test Strategy → Setup / teardown lifecycle'
    change: 'The first carve-out reads: Windows x64 starts seed the ConPTY companions before viola runs, into a home `seed_conpty` makes private; a test that widens a home tree above `bin/<key>/conpty/` starts unseeded, since an inheritable grant reaches every seeded home through the shared hard-linked companion.'
    sidecar: 'Integration setup carve-out carries the seeded-home grant hazard.'
    rationale: 'Report Spec claims disproved by measurement: «test-plan §5 / §7 seed carve-out (pattern seed) owes the hazard»; Deviation 10 (`run_refuses_a_widened_home_tree_before_its_log` starts its home unseeded).'
    dependent-of: D-tests-count-literals
  - detector: D-tests-count-literals
    severity: warning
    section: '§7 Test Data & Fixtures → Seed strategies: On-disk product state'
    change: 'Adds: a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all` — a plain home is one the product must refuse (Unix 0755 by the read rule; on Windows `Writable`, its refused entry an inherited `S-1-5-32-545` write ACE, measured on windows-2025).'
    sidecar: 'Seed strategies: test-made homes go through create_private_dir; a plain create_dir_all home is refused (measured on windows-2025, CARRY 13 answered).'
    rationale: 'Report Counts 15 (seed, test-plan.md:631 re-read for the home-creation step); Spec claims disproved by measurement (a home made with `create_dir_all` before viola is refused `Writable`, the inherited Users write ACE); Files: companions `tests/{contract_diag_schema,cli_machine_contract,tui_env_strip}.rs` moved onto `create_private_dir`.'
    basis: '.andromeda/test-plan.md:631'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → What unit tests cover: viola-state'
    change: 'The viola-state bullet adds: the Unix and Windows read verdicts (`unix_read_verdict`, `windows_read_verdict` over `READ_RIGHTS`), `check_home_tree` and `check_diagnostics` (widened, symlinked and absent paths), `update_stamps` refusing a widened `ledger/` before any read (`StampsError`), `live_endpoint` over a replayed snapshot being none, `pin_key`, and `pin_companions` refusing a path another user may write; the instance check holds the content folders to the read rule, and its `cfg(windows)` cases (an instance tree granted Users write beside one that passes, a `diagnostics/` granted Users read, a plain home made before viola) run on the `windows-2025` leg.'
    sidecar: 'Unit coverage of viola-state: read rule, home-tree and diagnostics checks, the stamps refusal, the replayed live_endpoint and the Windows instance-tree cases.'
    rationale: 'Report Symbols (viola_state::strict, stamps, liveness, pin) and New text unit ranges (strict.rs 708-830, 1110-1385; stamps.rs 283-326; liveness.rs 255-291; pin.rs 210-213, 453-473); Spec claims disproved: «Not covered: a widened DACL on an instance tree on Windows» now covered (Expected amendments: §4 the Windows `check_instance` case).'
    basis: 'crates/viola-state/src/strict.rs:1110-1385'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → What unit tests cover: root bin'
    change: 'The root bin bullet adds: the readers over a replayed snapshot (the collision check reading the beat alone, hook `connect` unreachable, statusline `serve` running nothing) and the diagnostics opens (`open_role_file`, `write_detail`) refusing a widened or symlinked path and leaving it as found.'
    sidecar: 'Unit coverage of the root bin: replayed-snapshot readers and the diagnostics-open refusals.'
    rationale: 'Report Symbols (src/cmd/run.rs, src/cmd/hook.rs, src/cmd/hook/statusline.rs, src/obs.rs) and New text: run.rs 1173-1224, hook.rs 640-694, statusline.rs 524-553, obs.rs 610-673.'
    basis: 'src/obs.rs:610-673'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Triggers: security-vector-coverage (filesystem), Vector 7'
    change: 'The Vector 7 negatives add: a home, instance directory or `diagnostics/` another user may read refused (Unix a group/other bit; Windows a read grant to another SID); a widened `ledger/`, `bin/`, `plugin/` or pinned copy refused by `run` / `verify` before any log; a symlinked diagnostics path refused; and a ConPTY companion another user may write leaving the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`.'
    sidecar: 'Vector 7 trigger: the read rule, the home-tree check, the diagnostics symlink refusal and the companion DACL fallback join the required negatives.'
    rationale: 'Report Counts 3 (the read rule built), 4 (the new tree checks), 7 / 18 (the `conpty/` owner/DACL leg, `strict-modes-failed`), and Coverage of new surfaces (each with its test tier).'
note: 'Detail-file sites the report names stale are the cascade standing groups, not proposed here: e2e-test-strategy.md:236, :237 (read-snapshot, wheel-human, owner-title), :305 (single-writer SIGSTOP); unit-test-strategy.md:19, :22 (read-snapshot), :25 (Windows instance-tree case now covered); integration-test-strategy.md:33 and test-data-fixtures.md:10 («the refused home DACL was not read», disproved by measurement). D-tests-obs-harness: no harness, status or log-format change (report Harness / gate surface: none).'
