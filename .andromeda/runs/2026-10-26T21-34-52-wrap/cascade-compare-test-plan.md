# Cascade compare — test-plan · wrap · 2026-10-26-home-and-code-bearing-file-integrity
E items 47 · held 17 · lost at classify 3 · lost at strong 1 · beside a placed fact 1 · cascade only 7 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### mocking-stubbing-discipline#2.4 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: "## 8. Mocking & Stubbing Discipline"
    change: "Add a table row after the Fake servers row: | Filesystem faults | none (a real tempfile home, real modes and ACLs) | a widened or read-granted home tree is made on the real file and refused by the product's own check, never a stubbed one: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; Windows `icacls`, its exit asserted), so on `windows-2025` an instance tree granted Users write, or a `diagnostics/` granted Users read, is refused beside a `create_private_dir` tree that passes (chunk 2026-10-26-home-and-code-bearing-file-integrity); only the foreign-owner SID and non-persistent-ACL descriptors stay synthetic SDDL fed to the pure verdicts (`windows_read_verdict` among them); a test that widens a home tree above `bin/<key>/conpty/` starts its home unseeded, because an inheritable grant reaches the shared seed companion through `seed_conpty`'s hard links (binds with §7 Seed strategies, the seed carve-out) |"
    sidecar: "2026-10-26-home-and-code-bearing-file-integrity: §8 detail gains a Filesystem faults row — widened and read-granted homes made by the real-file helpers widen/grant_read/unwiden/still_widened (Unix modes, Windows icacls), the Windows Users write/read cases now real on windows-2025, only foreign-owner and non-persistent-ACL descriptors synthetic; a widening test above bin/<key>/conpty/ starts unseeded."
    rationale: "Report Changes, Harness / gate surface (the four helpers in tests/support/home.rs, Unix modes and Windows icacls with exit asserted), Symbols (strict.rs cfg(windows) cases 708-830: a tree granted Users write, a diagnostics/ granted Users read), Spec claims disproved (a widened DACL on a Windows instance tree now covered) and Deviations 10 / Decisions (the inheritable grant propagating through seed_conpty's hard links); the file's table has no filesystem row, and body §8 line 707 states only that the filesystem is not mocked."
    basis: .andromeda/test-plan.md:707

### test-data-fixtures#1.12 · lost at classify · beside a placed fact F13 (near 13, test-data-fixtures#1.4) · .andromeda/test-plan-detail/test-data-fixtures.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New item following the table: On Windows x64 an inheritable `icacls` grant (`(OI)(CI)`) on a seeded home's `bin/` (or on the home) propagates through `seed_conpty`'s hard links into the shared seed companion under `target/conpty-seed/<key>/`, widening every seeded home until it is removed (measured on `windows-2025`, ci#40398601594 and ci#40400887151); so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded, as `run_refuses_a_widened_home_tree_before_its_log` does (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "test-data-fixtures.md — new item: the seed hard-link propagation hazard and the unseeded start of a test that widens a home tree above `bin/<key>/conpty/` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Spec claims disproved by measurement (third bullet: «No master states the opposite; test-plan §5 / §7's seed carve-out (pattern `seed`) owes the hazard»), Deviations 10 and Decisions & corrections (the measured propagation and the unseeded start)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:74

### test-data-fixtures#1.12 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New item following the table: A test widens or read-grants a viola-made home tree only through `tests/support/home.rs`'s `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; Windows `icacls`, its exit asserted), the helpers the strict-modes negatives (`tests/security_negatives_strict_modes.rs`, `crates/viola-mcp/tests/handlers.rs` and the hook, run and verify cases) use (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "test-data-fixtures.md — new item: the home-widening test helpers `widen` · `grant_read` · `unwiden` · `still_widened` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Harness / gate surface: `tests/support/home.rs` gains test helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes, Windows `icacls` with its exit asserted); Deviation 9."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:55

### test-data-fixtures#1.12 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New item following the table: A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a 0755 home is now refused), as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do; a catch-site case holds a viola-made home with a regular file where the checked entry goes (`tests/cli_wait_last.rs`, `diagnostics/`; `tests/cli_verify.rs`, `ledger/`), and a home under a regular file takes a per-OS oracle, because on Windows such a path stats `NotFound` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "test-data-fixtures.md — new item: test-made homes through `create_private_dir`, catch-site cases in a viola-made home, the per-OS oracle for a home under a regular file (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report scope record (companions `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs`, `tests/tui_env_strip.rs` serve step 1; `tests/cli_wait_last.rs` serves step 3), Deviation 8 and Decisions & corrections (a path under a regular file stats `NotFound` on Windows, a per-OS oracle owed); Count 13."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:108

### test-scope-summary#5.3 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:69
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 69
    section: '## 1. Test Scope Summary'
    change: '  - **As landed (strict-modes):** the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'test-scope-summary MCP surface: the frame tools strict-modes refusal as landed (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols for viola-mcp tools.rs frame (check_instance before live_endpoint, result::unreachable, no frame, no line), Retired gap (8), and Outcome criterion 3 MET by binary(handlers).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14

### test-strategy#3.6 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:63
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 63
    section: "## 2. Test Strategy"
    change: "In the Filesystem (V7) row's plan location, add §6 Security control negatives (root `tests/security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line (`frame_verb_refuses_a_widened_instance_before_any_log`); a home another user may read refusing `run` (`a_home_another_user_may_read_refuses_run`); a start under umask 000 making every home path owner-only (Unix); a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`; only the wrapper writing `snapshot.json` and only `verify` writing `stamps.json`, byte-identical after every other writer-capable entry point) and E5's built steps 1–3 (root `tests/chaos_snapshot_replay.rs`)."
    sidecar: "test-strategy trigger map V7 row: the strict-modes security negatives landed in tests/security_negatives_strict_modes.rs and E5 steps 1–3 in tests/chaos_snapshot_replay.rs (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "The report's Changes (Files: new tests/security_negatives_strict_modes.rs, 475 lines, and tests/chaos_snapshot_replay.rs; New text, by line: its five test functions) and Outcome ((security) six frame verbs refuse a widened instance; (tests) single writers, umask-independent modes, the Windows --home protected DACL, a pre-existing Users-read dir refused; (tests) E5 steps 1–3, each MET by these binaries on three CI legs) land the Vector 7 filesystem negatives, while the V7 row's plan location names only §5 Module ↔ disk and §6 Scenario 1, E5, where the V1 row names its security_negatives files."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:134

### test-strategy#2.1 · lost at strong · beside a placed fact F18 (near 29, test-strategy#1.7) · .andromeda/test-plan-detail/test-strategy.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: "## 2. Test Strategy"
    change: "In the sync E2E suites' shared-helpers sentence, add: `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened."
    sidecar: "test-strategy naming: tests/support/home.rs gains the widen / grant_read / unwiden / still_widened helpers (chunk 2026-10-26-home-and-code-bearing-file-integrity, Deviation 9)."
    rationale: "The report's Changes (Harness / gate surface: tests/support/home.rs gains test helpers widen · grant_read · unwiden · still_widened, 331-389, Unix modes, Windows icacls with its exit asserted) and Deviation 9 add shared helpers to tests/support/home.rs; the file's line 42 states what events.rs, cli.rs, fake.rs and ndjson.rs hold, but nothing of home.rs's widening helpers."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:55

## Held

- e2e-test-strategy#11.1
- e2e-test-strategy#12.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- integration-test-strategy#1.5
- integration-test-strategy#1.6
- integration-test-strategy#2.0
- integration-test-strategy#3.1
- integration-test-strategy#3.2
- integration-test-strategy#4.1
- quality-gates-coverage-targets#3.1
- test-data-fixtures#1.1
- test-scope-summary#10.1
- test-scope-summary#3.1
- test-scope-summary#8.1
- unit-test-strategy#1.2
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#17.1
- e2e-test-strategy#2.2
- e2e-test-strategy#7.2
- test-data-fixtures#1.4
- test-scope-summary#7.1
- test-scope-summary#9.2
- test-strategy#1.7
