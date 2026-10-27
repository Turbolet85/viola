proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 36
    section: "## 6. E2E Test Strategy › #### Scenario: Path 1 — `run` start sequence"
    change: "Add a Verification-signal bullet after the settings.json bullet: As of chunk 2026-10-26-home-and-code-bearing-file-integrity the start runs a home-tree strict-modes check (`check_home_tree`: the home, `check_instance`'s set, `ledger/`, `bin/`, `plugin/`, then `bin/<key>/`, the pinned copy, `plugin/<key>/` with its `hooks/` and `.claude-plugin/`) before the wrapper log opens; a refused tree exits 1 with the home pair (home level) or the instance pair (instance level), no log opened, no line and no child (`run_cli`, `security_negatives_strict_modes`); step 5's tampered pinned exe still exits 1."
    sidecar: "Path 1 signal: the start's home-tree strict-modes check before the wrapper log (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: run.rs `checked_exe` / `refuse_tree` called by `run` before `open_wrapper_log`, exit 1, no log, no line, no child; Outcome (security) `run` refuses a 0770 home, widened instances/<name>, bin/ or pinned copy — MET."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 174
    section: "## 6. E2E Test Strategy › #### Scenario: Path 6 — budget governor"
    change: "Extend the Step 6 bullet: `statusline_command` never runs when strict-modes fails; and since chunk 2026-10-26-home-and-code-bearing-file-integrity a statusline over a replayed snapshot (an unreadable or newer `snapshot.json`) runs nothing and prints nothing, because the replay carries no `statusline_command` (unit `statusline_over_a_replayed_snapshot_runs_nothing`)."
    sidecar: "Path 6 step 6: a statusline over a replayed snapshot runs nothing."
    rationale: "Report Changes, Symbols: hook/statusline.rs `serve` reads `read_snapshot_or_replay`; a replayed snapshot has no `statusline_command`, so nothing runs or prints."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 236
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Head `As landed (…): built` (was `landed in part`, the chunk 2026-10-26-home-and-code-bearing-file-integrity added); replace `The replay has two product readers, … and stays covered below the surface by state_replay.rs. run's collision check still calls read_snapshot, so step 3's viola run builder does not take the replay.` with: the replay's product readers are six — `viola revive`, `viola list`'s `board::read`, hook `connect`, statusline `serve`, `run`'s `collision_check` and `liveness::live_endpoint` — `read_snapshot` has no product caller under `src/`, and it stays covered below the surface by `crates/viola-state/tests/state_replay.rs`; and drop `; steps 1–3 stay unbuilt for the read_snapshot reason below` from the link-leg parenthesis."
    sidecar: "E5 as landed: six replay readers, read_snapshot retired as a product reader (3 → 0)."
    rationale: "Report Retired: `read_snapshot` as a product reader under src/ (3 → 0); Counts 5: replay product readers 2 → 6, e2e-test-strategy.md:236 stale."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:236
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 237
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Replace `Steps 1–3 stay unbuilt while run's collision check calls read_snapshot; the three other snapshot readers (…) stay on read_snapshot, and their move onto the replay (…) is one CARRY on the route entry Home and code-bearing file integrity.` with: Steps 1–3 are built (`tests/chaos_snapshot_replay.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity); a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so `live_endpoint` reads none, hook `connect` is unreachable (fail open), the statusline runs nothing, and `run`'s collision check reads the beat alone (`beat_age ≤ STALE_AFTER` → `already-live`, else free). Replace the closing `Two of its signals are set against what landed, for the entry that builds it to restate: …` with: Its two signals are restated as built: every start appends `wheel{holder:\"driver\", cause:\"start\"}`, so `wheel:\"human\"` does not hold after step 3, and the replay writes no file, so no snapshot is rebuilt from it."
    sidecar: "E5: steps 1–3 built, the readers' CARRY closed, the two signals restated."
    rationale: "Report Retired (read_snapshot as a product reader); Counts 5 and 19 (e2e-test-strategy.md:237 stale on both); Symbols: liveness, hook connect, statusline serve, collision_check over a replay; Outcome (tests) E5 steps 1–3 MET."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:237
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 586
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Restate the first signal as built: the replay reads `links`, `budget_paused:true` and `agent_session_id` from the log with `dialog_pending:false`; it writes no file, so no snapshot is rebuilt from it; every start appends `wheel{holder:\"driver\", cause:\"start\"}`, so `wheel:\"human\"` does not hold after step 3."
    sidecar: "E5 signal restated as built (wheel driver start; no file rebuilt)."
    rationale: "Report Counts 19: E5's two signals restated, test-plan.md:586 stale (lists the wheel:human signal)."
    basis: .andromeda/test-plan.md:586
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan.md
    near: 590
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Replace the As-landed line with: **As landed: built** (chunk 2026-10-26-home-and-code-bearing-file-integrity). Steps 1–3 run in `tests/chaos_snapshot_replay.rs`; the replay's product readers are `viola revive`, `viola list`, hook `connect`, statusline `serve`, `run`'s collision check (a replayed snapshot read by its beat alone) and `live_endpoint`, `read_snapshot` has no product caller under `src/`, and the CARRY on the route entry \"Home and code-bearing file integrity\" is closed."
    sidecar: "E5 broad line: built; readers on the replay; CARRY closed."
    rationale: "Report Retired (read_snapshot as a product reader, 3 → 0); Counts 5 (test-plan.md:590 stale, owner-title row); Outcome (tests) E5 steps 1–3 MET."
    basis: .andromeda/test-plan.md:590
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 592
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Remove the set-against note (`Two of the signals above are set against what landed, for the entry that builds it to restate: …`): the entry built it and the restatement now stands in the signal above."
    sidecar: "E5: the set-against note removed, restated in the signal."
    rationale: "Report Counts 19: test-plan.md:592 is the set-against note «for the entry that builds it to restate», stale now the entry landed."
    basis: .andromeda/test-plan.md:592
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 246
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add to the fail-open matrix cases: a refused or absent instance (since chunk 2026-10-26-home-and-code-bearing-file-integrity the event and dialog paths run the instance check before any `config.json` read or obs init): exit 0, empty streams, no frame, no line, no SessionEnd append; and a `no_instance_dir` row: a `VIOLA_DIR` naming an instance directory that does not exist writes nothing. The `--capture` arms are unchanged."
    sidecar: "Security sweep: hook fail-open matrix gains the refused/absent instance and no_instance_dir rows."
    rationale: "Report Symbols (hook.rs `canonical` / `trusted` before obs init; a VIOLA_DIR naming no directory writes nothing) and Deviations 5; Outcome (security) viola hook event/dialog refused — MET, hook_fail_open green."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 598
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "In the Security sweep line's fail-open list add `a refused or absent instance (a VIOLA_DIR naming no directory included)` after `a missing VIOLA_NAME`."
    sidecar: "Security sweep broad line: refused or absent instance joins the fail-open list."
    rationale: "Report Symbols, hook.rs: a refused or absent instance returns exit 0 at once, no line, no frame."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 270
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Change the cause's parenthesis to `(Unix a widened mode; Windows a Users grant through icacls on windows-2025)` and add after the link as-landed line: As landed for `send`, `wait`, `last`, `answer`, `pause` and `release` (chunk 2026-10-26-home-and-code-bearing-file-integrity), cases of `tests/security_negatives_strict_modes.rs` on three OSes, not `cross_exit_causes.rs` rows: a widened instance is refused first (`check_instance`, the home, the instance dir and `diagnostics/` under the read rule), exit 21 with link's instance-state pair or the `instance-unreachable` document with stderr empty, no frame, no line, mode or ACL as found."
    sidecar: "Exit-cause matrix: the six frame verbs' strict-modes exit-21 cause as landed."
    rationale: "Report Symbols (`refuse_untrusted` at the six verbs, exit 21, no line); Counts 8 (the six verbs join link's instance pair); Outcome (security) six frame verbs — MET on three CI legs."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 283
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Extend `the --home fails strict-modes at run`: the home tree (`check_home_tree`: the home, the instance set, `ledger/`, `bin/`, `plugin/`, the pinned copy and `plugin/<key>/`) before the wrapper log, exit 1 with the home pair or, at instance level, the instance pair, no log, no line, no child (`run_cli`, `security_negatives_strict_modes`, a read-granted pre-existing home on every OS); beside it `viola verify` exits 1 with the home pair on a refused home tree, no step line, no summary (`cli_verify`)."
    sidecar: "Exit-cause matrix: run's and verify's home-tree exit-1 causes (home / instance pair)."
    rationale: "Report Symbols (run.rs `checked_exe` / `refuse_tree`; verify.rs `check_home_tree` then `refuse_home`, exit 1); Outcome (security) run and verify refusals — MET."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 286
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Extend the revive strict-modes bullet: the check now also holds the home, the instance directory and its `diagnostics/` to the read rule (another user may read them), and `revive`'s start runs `run`'s home-tree check before the wrapper log (exit 1, no log opened, no line, no child; `--json` keeps `strict-modes-failed`)."
    sidecar: "Exit-cause matrix: revive's strict-modes cause widened (read rule, start tree check)."
    rationale: "Report Symbols: `check_instance`'s set widened (read rule on the home, the instance dir, diagnostics/); revive.rs's start calls `checked_exe` / `refuse_tree` before `open_wrapper_log`, revive `--json` keeps `strict-modes-failed`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 604
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add to the Exit-cause matrix line: the six frame verbs' strict-modes cause (exit 21, the instance pair) is a case of `tests/security_negatives_strict_modes.rs` on three OSes."
    sidecar: "Exit-cause broad line: the six verbs' strict-modes cause."
    rationale: "Report Outcome (security) six frame verbs — MET, binary(security_negatives_strict_modes) on three CI legs."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 303
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Replace `the RUNNER_TEMP negative case as worded here stays owed to that route entry` and `(read back with GetNamedSecurityInfoW)` with the case as landed (chunk 2026-10-26-home-and-code-bearing-file-integrity, `tests/security_negatives_strict_modes.rs`, windows-2025): `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` reads the DACL back through `icacls <home> /save` as `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` (canonical SDDL spells the runner's RID-500 user `LA`), and a pre-existing directory carrying a `BUILTIN\\Users` read grant is refused with exit 1."
    sidecar: "Security negatives: the Windows --home outside the profile case built; no longer owed."
    rationale: "Report Outcome (tests) Windows --home outside the profile — MET, the DACL read D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY); Counts 1 (owner-title rows closed); Decisions & corrections: Get-Acl printed nothing, icacls /save read it back."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 305
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Replace `With the wrapper stopped (SIGSTOP on Unix; on Windows the wrapper is suspended with the fake agent idle)` with `With the wrapper stopped (Wrapper::stop), never SIGSTOPped`, and add: as landed in `tests/security_negatives_strict_modes.rs` on three OSes, both files read byte-identical after every other writer-capable entry point."
    sidecar: "Security negatives: single writers witnessed with the wrapper stopped, not SIGSTOPped."
    rationale: "Report Counts 16: single writers witnessed with the wrapper STOPPED (Wrapper::stop), not SIGSTOPped, e2e-test-strategy.md:305 stale; Outcome (tests) single writers — MET."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:305
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 310
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add a bullet: **Strict-modes before the first read or open** (`tests/security_negatives_strict_modes.rs`, three OSes, chunk 2026-10-26-home-and-code-bearing-file-integrity). `send`, `wait`, `last`, `answer`, `pause` and `release` each refuse a widened instance with exit 21, the instance pair or `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}`, no frame and no line, the mode or ACL left as found. The same binary holds the umask-independent modes, the single-writer and the Windows protected-DACL cases above."
    sidecar: "Security negatives: the new binary security_negatives_strict_modes (F1)."
    rationale: "Task New facts F1; report Changes (Files: new tests/security_negatives_strict_modes.rs) and Outcome (security, tests) — MET on three CI legs."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 310
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add a bullet: **ConPTY companion owner/DACL** (`#[cfg(windows)]` x64, windows-2025). `run_on_a_companion_another_user_may_write_falls_back_to_conhost` leaves the child on the inbox ConPTY with `process-start.sideload_fallback` `strict-modes-failed`, the exit unchanged; in the kept homes exactly one `strict-modes-failed` appears, from that test."
    sidecar: "Security negatives: the conpty/ companion owner/DACL fallback case (F15)."
    rationale: "Task New facts F15; report Outcome (security) conpty/ companion — MET on windows-2025, kept homes: exactly one strict-modes-failed."
    fact: F15
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 606
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "In the Security control negatives line write `single writers of stamps.json and snapshot.json (the wrapper stopped)` and add `the six frame verbs refusing a widened instance before any frame (tests/security_negatives_strict_modes.rs)` and `a conpty/ companion another user may write falling back to the inbox ConPTY (Windows x64)`."
    sidecar: "Security negatives broad line: the strict-modes binary, the stopped-wrapper method, the conpty fallback."
    rationale: "Report Counts 16 (test-plan.md:606 stale, the method); Outcome (security, tests) — MET."
    basis: .andromeda/test-plan.md:606
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 338
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add a bullet: snapshot replay (`tests/chaos_snapshot_replay.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity): the built E5 steps 1–3 and the replayed readers — a truncated or newer `snapshot.json` read through `read_snapshot_or_replay` by `run`'s collision check, hook `connect`, the statusline and `live_endpoint`."
    sidecar: "Chaos suite gains chaos_snapshot_replay (F2)."
    rationale: "Task New facts F2; report Changes (new root test binary tests/chaos_snapshot_replay.rs) and Outcome (tests) E5 steps 1–3 and the replayed readers — MET."
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "In the On-disk product state row replace `A home that seed made is not one viola created, and a first boot and stop does not change that: hook statusline's instance check refuses it (as measured at …, CI run 38080061631, three cases; the refused home's DACL was not read, and that reading is owed to the working-route entry …).` with: `seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so its homes pass `check_instance` on windows-2025 and the three `hook_statusline` cases pass (chunk 2026-10-26-home-and-code-bearing-file-integrity). A home a test makes itself goes through `create_private_dir` too: one made with `create_dir_all` before viola is refused — on Unix (0755) by the read rule; on windows-2025 `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE (ci#40398601594, `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`); that home's canonical SDDL was not recorded."
    sidecar: "Test data: seed_conpty makes a viola-made home; the owed DACL reading measured (Writable, inherited Users write ACE)."
    rationale: "Report Spec claims disproved by measurement (test-data-fixtures.md:10, «the refused home's DACL was not read»); Counts 15 (seed); Changes, Harness surface (seed_conpty makes the home and the seed dir with create_private_dir); Scope record (create_dir_all homes refused by the read rule); Outcome (tests) the seed MET, the SDDL not recorded."
    basis: .andromeda/test-plan-detail/test-data-fixtures.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after On-disk product state: | A widened seeded home (Windows x64) | Measured on windows-2025 (ci#40398601594, ci#40400887151): an inheritable `icacls` grant (`(OI)(CI)`) on a seeded home's `bin/` or on the home propagates through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until removed; a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded (`run_refuses_a_widened_home_tree_before_its_log`). | per-test |"
    sidecar: "Test data: the seeded-home hard-link hazard (F14)."
    rationale: "Task New facts F14; report Spec claims disproved (the measured propagation, owed by §5 / §7's seed carve-out) and Deviations 10."
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 13
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after the statusline source row: | Widened test homes (the strict-modes negatives) | Set by the test, never by viola, through `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted). | per-test |"
    sidecar: "Test data: the widen / grant_read / unwiden / still_widened helpers (F13)."
    rationale: "Task New facts F13; report Changes, Harness / gate surface: tests/support/home.rs gains the four helpers."
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 26
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Extend the timed-path sentence: since chunk 2026-10-26-home-and-code-bearing-file-integrity every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init; with both checks in the timed path the five rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms, green under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`)."
    sidecar: "Perf: the instance check in the timed hook path; reading max 2.5–3.3 ms."
    rationale: "Report Outcome (obs): `run --perf` keeps the five hook rows under SPINE_DEADLINE with the check in the timed path — max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms; Symbols (hook.rs `trusted` before `viola_obs_init`)."
note: "Group-1 or other masters own the rest: F3–F12, F16–F18 (unit / integration / test-strategy); the state.snapshot_recover span placement (Count 6) is obs-plan's; body test-plan.md:613's chaos summary may take the snapshot-replay case beside F2."
