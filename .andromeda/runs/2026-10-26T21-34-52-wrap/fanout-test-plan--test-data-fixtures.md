proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "In the On-disk product state row, replace the sentence from `A home that seed made is not one viola created` through `owed to the working-route entry \"Home and code-bearing file integrity\")` with: `seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so a seeded home passes `check_instance` (green on `windows-2025`, ci#40403389629, the three `hook_statusline` cases included; chunk 2026-10-26-home-and-code-bearing-file-integrity). A home a test makes first with `create_dir_all` is one the product must refuse: on `windows-2025` it is refused `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE (ci#40398601594, case `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`); the reading is that named ACE, not the home's SDDL, which was not recorded."
    sidecar: "test-data-fixtures.md:10 — the seed carve-out restated as built (viola's `create_private_dir` makes the seeded home and seed dir; seeded homes pass `check_instance`); the owed DACL reading closed by the measured refusal `Writable` on an inherited Users write ACE (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Counts 15 (pattern `seed`, test-data-fixtures.md:10 named stale) and Spec claims disproved by measurement (test-data-fixtures.md:10 «the refused home's DACL was not read», measured on ci#40398601594); Outcome: `seed_conpty` homes pass `check_instance` on `windows-2025` and the three `hook_statusline` cases pass (MET), the SDDL itself not recorded (Deviation 6); the row's owner, this entry, is closed (Count 1, pattern `owner-title`)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New item following the table: On Windows x64 an inheritable `icacls` grant (`(OI)(CI)`) on a seeded home's `bin/` (or on the home) propagates through `seed_conpty`'s hard links into the shared seed companion under `target/conpty-seed/<key>/`, widening every seeded home until it is removed (measured on `windows-2025`, ci#40398601594 and ci#40400887151); so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded, as `run_refuses_a_widened_home_tree_before_its_log` does (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "test-data-fixtures.md — new item: the seed hard-link propagation hazard and the unseeded start of a test that widens a home tree above `bin/<key>/conpty/` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Spec claims disproved by measurement (third bullet: «No master states the opposite; test-plan §5 / §7's seed carve-out (pattern `seed`) owes the hazard»), Deviations 10 and Decisions & corrections (the measured propagation and the unseeded start)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:74
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New item following the table: A test widens or read-grants a viola-made home tree only through `tests/support/home.rs`'s `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; Windows `icacls`, its exit asserted), the helpers the strict-modes negatives (`tests/security_negatives_strict_modes.rs`, `crates/viola-mcp/tests/handlers.rs` and the hook, run and verify cases) use (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "test-data-fixtures.md — new item: the home-widening test helpers `widen` · `grant_read` · `unwiden` · `still_widened` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Harness / gate surface: `tests/support/home.rs` gains test helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes, Windows `icacls` with its exit asserted); Deviation 9."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New item following the table: A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a 0755 home is now refused), as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do; a catch-site case holds a viola-made home with a regular file where the checked entry goes (`tests/cli_wait_last.rs`, `diagnostics/`; `tests/cli_verify.rs`, `ledger/`), and a home under a regular file takes a per-OS oracle, because on Windows such a path stats `NotFound` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "test-data-fixtures.md — new item: test-made homes through `create_private_dir`, catch-site cases in a viola-made home, the per-OS oracle for a home under a regular file (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report scope record (companions `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs`, `tests/tui_env_strip.rs` serve step 1; `tests/cli_wait_last.rs` serves step 3), Deviation 8 and Decisions & corrections (a path under a regular file stats `NotFound` on Windows, a per-OS oracle owed); Count 13."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:108
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness find no drift in this category (nextest runner unchanged, no harness change); body test-plan.md:631's seed sentence is the body's own row for the `seed` pattern, outside this file."
