proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: '## 1. Test Scope Summary'
    change: 'Replace the closing parenthesis «(Unix: its 0755 mode fails the read rule; Windows: its inherited Users write ACE)» with «(Unix: its 0755 mode fails the read rule; Windows: an inherited write grant to a SID viola does not trust, whichever SID the runner gives it: Users on one windows-2025 host, Authenticated Users `S-1-5-11` on another)».'
    sidecar: 'test-scope-summary: the plain home''s Windows refusal no longer names the Users SID; it is the inherited untrusted write grant, read by its right (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds, CI red 2).'
    rationale: 'Report, Spec claims disproved by measurement: on ci#40425866096 Users held read only and Authenticated Users (`S-1-5-11`, mask `0x001301BF`) held the inherited write, and the case now reads the grant by its right; this site names Users as the refused grant and is not among the report''s nine `plain-home` sites.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 26
    section: '## 4. Unit Test Strategy'
    change: 'In the instance-check bullet, replace «its refused entry an inherited Users (`S-1-5-32-545`) write ACE» with «its oracle reading the inherited allow ACE that carries a write right for a SID viola does not trust, whichever SID holds it (Authenticated Users `S-1-5-11` on one windows-2025 host, Users on another)».'
    sidecar: 'unit-test-strategy: the plain-home case''s Windows refused entry is no longer the Users SID; the oracle reads the inherited untrusted write grant by its right (CI red 2, folded PROVISIONAL).'
    rationale: 'Report, Spec claims disproved by measurement: `unit-test-strategy.md:26` is one of the six sites marked changed; the product verdict `Err(Writable)` was right, and the oracle demanding `S-1-5-32-545` failed on ci#40425866096, where Authenticated Users held the write.'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 26
    section: '## 4. Unit Test Strategy'
    change: 'After the instance-check bullet, add the item «On Windows the plain-home case''s oracle is the `cfg(windows)` test helper `strict::win::tests::inherited_untrusted_writes(home)`: the allow ACEs that are inherited, carry a `WRITE_RIGHTS` bit and name a SID `trusted_sid` does not trust; its control, a home `instance_tree()` makes, reads none, so the oracle cannot pass vacuously (green on `windows-2025` at ci#40439188053).»'
    sidecar: 'unit-test-strategy: adds the Windows plain-home oracle `inherited_untrusted_writes` and its non-vacuous control (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds).'
    rationale: 'Report, Symbols / APIs (test-only, `cfg(windows)`, `strict.rs:746-760`) and Outcome Red 2: the case reads the inherited untrusted write grant by its right, and its control (a viola-made home) reads none.'
    basis: crates/viola-state/src/strict.rs:746
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 28
    section: '## 4. Unit Test Strategy'
    change: 'After the stamps-and-pin bullet, add the item «Placement: `place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp` (0700 / 0600 on Unix, the mode set before any byte), `place_private_new_over_a_present_name_is_present_and_leaves_it` (`Placed::Present`, the target byte-identical, no temp left), `place_private_new_under_a_missing_dir_fails`, and the forced-window cases `place_private_new_placed_copy_execs_at_once` (Unix) and `place_private_new_placed_copy_runs_at_once` (Windows) (§5); the lost placement: `place_absent_over_a_copy_another_start_placed` (same bytes `Ok`, other bytes `PinError::HashMismatch` with the file byte-identical) and `place_absent_on_a_free_path_writes_the_copy`.»'
    sidecar: 'unit-test-strategy: adds the viola-state cases for the pinned copy''s no-clobber placement (`fs::tests::place_private_new_*`) and lost-placement re-hash (`pin::tests::place_absent_*`).'
    rationale: 'Report, Coverage of new surfaces: `place_private_new` is tested at unit level by `fs::tests::place_private_new_*` and `pin::tests::place_absent_*`; Outcome rows 2-3 read each case green; the group lists the stamps and pin refusals but not these cases.'
    basis: crates/viola-state/src/fs.rs:382
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 382
    section: '## 4. Unit Test Strategy'
    change: 'In the viola-state broad line, replace the closing «and the stamps and pin strict-modes refusals.» with «the stamps and pin strict-modes refusals, and the pinned copy''s no-clobber placement (`place_private_new`) and its lost-placement re-hash (`place_absent`).»'
    sidecar: 'test-plan §4: the viola-state broad line names the pinned copy''s placement and lost-placement cases.'
    rationale: 'Report, Coverage of new surfaces (unit tests `fs::tests::place_private_new_*`, `pin::tests::place_absent_*`): the body line the unit-test-strategy group details lists every other viola-state case family, but not this one.'
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: '## 5. Integration Test Strategy'
    change: 'After the Module ↔ PTY test approach, add «A filesystem-placement forced-window witness sits beside the PTY ones (it makes no PTY spawn): `place_private_new_placed_copy_execs_at_once` (`crates/viola-state/src/fs.rs`, Unix) execs the placed copy right after a `Placed::Written` placement through `place_private_new_with`''s `placed` hook; with the guard removed (the copy persisted with its write handle still open) it is red on the Linux dev host with `ExecutableFileBusy` (raw 26, `ETXTBSY`), and green with the guard restored; its Windows form `place_private_new_placed_copy_runs_at_once` is green-only on `windows-2025`, its red leg read on the Linux dev host only.»'
    sidecar: 'integration-test-strategy: adds the filesystem-placement forced-window witness for the pinned copy (two-sided on the Linux dev host, green-only on Windows).'
    rationale: 'Report, Expected amendments (test-plan §5, carried): `forced-window` has 5 standing sites, all PTY or tui witnesses, and no site holds a filesystem-placement witness; Outcome row 1 reads the red leg `Os { code: 26, kind: ExecutableFileBusy }` and green restored (`evidence/red-green.md`), with CI green on ubuntu, macOS and windows-2025.'
    basis: crates/viola-state/src/fs.rs:431
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 34
    section: '## 5. Integration Test Strategy'
    change: 'Replace «the refused entry an inherited `S-1-5-32-545` (Users) write ACE» with «the refused entry an inherited write grant to a SID viola does not trust, read by its right whichever SID holds it (on ci#40425866096 Authenticated Users `S-1-5-11` held the write and Users held read only)».'
    sidecar: 'integration-test-strategy: the plain home''s Windows refused entry is no longer the Users SID; it is the inherited untrusted write grant (CI red 2, folded PROVISIONAL).'
    rationale: 'Report, Spec claims disproved by measurement: `integration-test-strategy.md:34` is one of the six sites marked changed; ci#40425866096 measured Authenticated Users holding the inherited write, and the case is green at ci#40439188053 reading the grant by its right.'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:34
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 34
    section: '## 5. Integration Test Strategy'
    change: 'After the plain-home sentence, add «On one image (`windows-2025-vs2026` 20261021.278.1) `windows-2025` runners differ in a temp dir''s inherited ACL by host: Users write on ci#40403389629 (westus3), and Authenticated Users `S-1-5-11` write with Users read on ci#40425866096 (northcentralus). An oracle that names one SID for a runner-inherited grant therefore depends on the runner, so it reads the grant by its right, with a control that cannot pass vacuously.»'
    sidecar: 'integration-test-strategy: records that windows-2025 hosts differ in a temp dir''s inherited ACL, and the rule that a runner-inherited grant is read by its right.'
    rationale: 'Report, Decisions & corrections (the windows-2025 runners differ in a temp dir''s inherited ACL by host): an oracle that names one SID for a runner-inherited grant depends on the runner.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 196
    section: '## 6. E2E Test Strategy › #### Scenario: Path 7 — unverified CLI version gate'
    change: 'After step 3, add «The MCP leg''s own case, `mcp_tools_path7_unverified_cli_answer` (`crates/viola-e2e/tests/mcp_tools.rs`), boots the default `overseer` + `builder` pair unstamped (`SessionOptions { unstamped: true, ..SessionOptions::default() }`), so it also witnesses two first starts on one fresh unstamped home both landing (the pinned-copy race), green on all three CI OSes; step 1''s CLI-leg boot is unchanged.»'
    sidecar: 'e2e-test-strategy Path 7: its MCP leg boots the default pair unstamped (the race workaround of booting `builder` alone is retired).'
    rationale: 'Report, Retired and Counts / qualifiers moved: `mcp_tools_path7_unverified_cli_answer` now boots the default `overseer` + `builder` pair unstamped and its race comment is removed; Outcome reads the default-pair session ready on all three CI OSes (ci#40439188053); the step-1 CLI-leg procedure is unchanged (report, Counts).'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: '## 7. Test Data & Fixtures'
    change: 'In the on-disk product state row, replace «on windows-2025 `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE» with «on windows-2025 `Writable`, the refused entry an inherited write grant to a SID viola does not trust, read by its right whichever SID holds it (Authenticated Users `S-1-5-11` on one runner host, Users on another)».'
    sidecar: 'test-data-fixtures: the plain home''s Windows refused entry is no longer the Users SID (CI red 2, folded PROVISIONAL).'
    rationale: 'Report, Spec claims disproved by measurement: `test-data-fixtures.md:10` is one of the six sites marked changed; ci#40425866096 measured Authenticated Users holding the inherited write, with Users read only.'
    basis: .andromeda/test-plan-detail/test-data-fixtures.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 634
    section: '## 7. Test Data & Fixtures'
    change: 'In the On-disk product state seed line, replace «its refused entry an inherited `S-1-5-32-545` write ACE» with «its refused entry an inherited write grant to a SID viola does not trust, whichever SID the runner gives it».'
    sidecar: 'test-plan §7: the plain home''s Windows refused entry is no longer the Users SID.'
    rationale: 'Report, Spec claims disproved by measurement: `.andromeda/test-plan.md:634` is one of the six sites marked changed, and it is the broad line the test-data-fixtures row details.'
    basis: .andromeda/test-plan.md:634
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 59
    section: '## 7. Test Data & Fixtures'
    change: 'In the `hook` kind, after «when it ran», add «, plus `spawn_os_error` when the spawn of an absolute `command` fails (the raw OS error as an integer, `null` when the OS gives none); a successful spawn and the non-absolute arm add nothing»; the `statusline` kind is left as it stands.'
    sidecar: 'test-data-fixtures: the fake agent''s `hook` receipt carries `spawn_os_error` on a failed absolute spawn.'
    rationale: 'Report, Symbols / APIs (test-only) and Harness / gate surface: `run_receipted` adds `spawn_os_error` (`json!(e.raw_os_error())`) to a hook receipt when an absolute spawn fails; the report marks the hook receipt description changed and the `statusline` receipt no-change.'
    basis: src/bin/viola-fake-agent.rs:642
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 18
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'In the Process mocking row''s discipline cell, append «; a forced-window test parameter on a private function (`place_private_new_with`''s `placed` hook, run right after a `Written` placement) is not a seam: it has no env read, no feature and no mockall double, and it adds nothing to the test-seam list».'
    sidecar: 'mocking-stubbing-discipline: the `placed` hook of `place_private_new_with` is recorded as a test parameter, not a seam.'
    rationale: 'Report, Symbols / APIs: `place_private_new_with`''s `placed` hook «is the forced-window test parameter, not a seam: no env read, no feature»; Schema / config: no env var or flag was added.'
    basis: crates/viola-state/src/fs.rs:326
    fact: F7
note: 'Outside these entities: the key file `.andromeda/registries/contracts/test-plan/5-command-implementation.md:26` («whose Path 7 MCP leg boots an unstamped `builder` alone») is marked changed by the report and still needs its own edit. The report gives the `hook` receipt description as `test-data-fixtures.md:42`, but the picked entity shows it at line 59, so the orchestrator should check that anchor.'
