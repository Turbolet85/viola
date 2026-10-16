proposals:
  - detector: D-tests-coverage
    severity: warning
    section: '§5. Integration Test Strategy'
    change: 'tests/cli_verify.rs pins verify against the fake agent print mode and its FIVE interactive runs (Run E: a `--resume <Run B first session id>` start through `resume_args`, input empty, no `--settings`, no option added); `MM` the relied count 22 on Unix (the last `[22/22] resume-session …`) and 19 on Windows (the last `[19/19] resume-session …`), `STEPS_PASS` / `ROW_IDS` `[&str; 22]` on Unix and the 19-id per-OS literal elsewhere; a run without the framing replay `stamped 2.1.0  18 pass  4 fail` on Unix and `18 pass  1 fail` on Windows; the tag-turn-modal case `20 pass  2 fail` on Unix and `17 pass  2 fail` on Windows; an uninterrupted verify closes seven spawn pairs (`version-probe`, `verify-probe`, five `verify-pty-probe`; the dispatch-error case renamed `…_seven_spawn_pairs`); and the stop cases gain `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` (tests/cli_verify.rs:1071-1178: Run E held open by a set with no default SessionStart and a shaped `clear-1` id, its `-resume` dir removed, exit 1, its killed child spawn pair asserting `duration_ms` only).'
    sidecar: '§5 cli_verify: five interactive runs (Run E), relied 22/19, summaries 18/4 · 18/1 and 20/2 · 17/2, seven spawn pairs, the resume-run stop case (chunk 2026-10-14-resume-and-compact-ledger-rows).'
    rationale: 'The report Symbols add Run E (`resume_run`, src/cmd/verify/typed.rs:235-266) and `LedgerRow::ResumeSession` as the 22nd row; Counts move ledger rows 21 → 22 (relied 22/19), interactive runs 4 → 5 and spawn pairs 6 → 7 with test-plan.md:657 a standing site of `rows21`, `rows18`, `verify-runs` and `spawn-pairs`; Deviations give the measured summary forecasts (no-framing 18/4, Windows 18/1; tag modal 20/2, Windows 17/2); Coverage and Spec claims disproved name the new stop case and its `duration_ms`-only assert.'
    basis: '.andromeda/test-plan.md:657'
  - detector: D-tests-coverage
    severity: warning
    section: '§4. Unit Test Strategy'
    change: 'Beside the statusline arms, name the `resume-session` arm (since chunk 2026-10-14-resume-and-compact-ledger-rows): it passes only when Run E first SessionStart capture (`Probes.resumed`) has source `resume` and the id of Run B first SessionStart capture accepted by `is_session_id` (`resume_target`); five `check_resume_session_*` cases (the recorded shape passes; another source, another id, no Run E capture and a Run B first id off the session-id shape fail; `test(/check_resume_session_/)` reads 5 passed), `resume_target_is_run_b_first_session_id_of_the_shape` and `resume_variants_name_run_e_first_session_start_resume_1`; the verdict `verified` reads `LedgerRow::relied()`: 22 rows on Unix, 19 on Windows, and the `version_gate.rs` oracle holds that an all-`pass` older stamp of 21 (Unix) or 18 (Windows) ids reads unverified while an 18-plus-resume stamp verifies on Windows only.'
    sidecar: '§4 unit: the resume-session arm and its seven unit cases; relied 22/19; the older-stamp oracle (chunk 2026-10-14-resume-and-compact-ledger-rows).'
    rationale: 'Report Symbols: `resume_target`, `session_resumed`, `resume_variants` (ledger.rs:842-873) and the check arm (:934); Coverage names 5 `check_resume_session_*`, `resume_target_*`, `resume_variants_*` units; Outcome reads `passed 5` and the version_gate oracle; Counts list test-plan.md:568 as a `rows21` / `rows18` site.'
    basis: '.andromeda/test-plan.md:568'
  - detector: D-tests-coverage
    severity: warning
    section: '§5. Integration Test Strategy'
    change: '`contract_ledger_probes.rs` lists each relied literal row id once with `ROW_IDS: [&str; 22]` (the three statusline ids relied on Unix only, `resume-session` relied on every OS) and ends `stamped <ver>  <n> pass  0 fail`, `<n>` 22 on Unix and 19 on Windows.'
    sidecar: '§5 contract_ledger_probes: ROW_IDS 22, resume-session relied on every OS, n 22/19.'
    rationale: 'Report Counts: ledger rows 21 → 22, relied 22 Unix / 19 Windows, test-plan.md:667 a standing `rows21` / `rows18` site; Outcome: contract_ledger_probes names `resume-session` once per OS literal and ends `22 pass` / `19 pass`.'
    basis: '.andromeda/test-plan.md:667'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§1. Test Scope Summary'
    change: 'Live `viola verify` also refreshes, through its Run E, the recorded resume variant `SessionStart.resume-1.json`, and the fake-agent drift contract also replays every recorded resume set (`fake_agent_resume_replay_matches_every_recorded_resume_set`, tests/contract_fake_agent_drift.rs:415-434); the run count reads five interactive runs.'
    sidecar: '§1 drift-contract critical path: Run E refreshes SessionStart.resume-1.json; the resume replay joins the contract.'
    rationale: 'Report Counts: interactive runs 4 → 5 with test-plan.md:408 a `verify-runs` site; Schema: new fixture variant stem `SessionStart.resume-<n>.json`; Coverage names the drift-contract resume replay.'
    basis: '.andromeda/test-plan.md:408'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7. Test Data & Fixtures'
    change: 'Under `--resume <id>` the launch SessionStart base is the set recorded `SessionStart.resume-1.json` when it exists (2.1.287, 10 key names), else the recorded `SessionStart.default` as before (a set without the variant, 2.1.288, 5 key names), with `source` set to `resume` and `session_id` to the id given (the compiled fork id under `--fork-session`), the trailing newline kept (`launch_variant`; unit `fake_resume_launches_on_the_recorded_resume_variant_when_the_set_holds_one`, src/bin/viola-fake-agent.rs:1356-1378; integration `fake_agent_resume_launches_on_the_recorded_resume_variant`, tests/cli_fake_agent.rs:533-565; no option added).'
    sidecar: '§7 fake agent: the --resume launch builds on the recorded SessionStart.resume-1.json where the set holds it; the default rewrite kept for a set without it.'
    rationale: 'Report Symbols (Fake agent: `RESUME_VARIANT`, `Agent::launch_variant` at viola-fake-agent.rs:437-445, `resume_fields` unchanged, the count stays twelve) and Counts (fake-resume: 5 → 10 key names on 2.1.287, a set without the variant keeps the default-based payload; test-plan.md:1077 a standing site).'
    basis: '.andromeda/test-plan.md:1077'
  - detector: D-tests-coverage
    severity: warning
    section: '§7. Test Data & Fixtures'
    change: 'In the fake-agent options paragraph: the eight options serve verify five interactive runs; `--resume <id>` serves `viola revive` tests and verify Run E; and the sentence "One payload is set without one: no recorded SessionStart has source `resume` … owed to the route entry Resume and compact ledger rows" is replaced by: 2.1.287 holds the recorded `SessionStart.resume-1.json` (10 key names, recorded live by Run E at chunk 2026-10-14-resume-and-compact-ledger-rows), the `--resume` launch builds on it, and the rewritten 5-key payload stays only for a set without the variant; the option count stays twelve.'
    sidecar: '§7 fake-agent options: five interactive runs; the resume payload no longer owed, recorded at 2.1.287.'
    rationale: 'Report Counts: `verify-runs` (test-plan.md:1094), `fake-resume` (test-plan.md:1077, :1094) and `resume-norow` (test-plan.md:1094, the route entry title: "One relied-on dependence has no row" is no longer true); Symbols: no option added, count stays twelve.'
    basis: '.andromeda/test-plan.md:1094'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7. Test Data & Fixtures'
    change: 'The fixture-hygiene walk also covers 2.1.287 recorded resume variant `SessionStart.resume-1.json` (one today) with the hook payloads and the same path, username and email walk (`claude_resume_variants_are_walked_for_every_resume_set`, tests/contract_fixture_hygiene.rs:533-564).'
    sidecar: '§7 hygiene walk: the resume variant joins it, with its own walked-set case.'
    rationale: 'Report Schema: new fixture variant stem `SessionStart.resume-<n>.json`, scrubbed and walked by the hygiene contract; Coverage names `claude_resume_variants_are_walked_for_every_resume_set`.'
  - detector: D-tests-coverage
    severity: warning
    section: '§7. Test Data & Fixtures'
    change: 'The Recorded hook payloads row adds Run E resume payload as `SessionStart.resume-<n>.json` (one today, under `2.1.287` only, scrubbed and `unclean`-checked; the 2.1.287 set now 32 files).'
    sidecar: '§7 Recorded hook payloads: Run E records SessionStart.resume-<n>.json.'
    rationale: 'Report Files (new fixtures/claude/2.1.287/SessionStart.resume-1.json, recorded live), Schema (new stem) and Counts (the 2.1.287 set 31 → 32 files).'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6. E2E Test Strategy → Scenario: Path 2 — confirmed `send` with CL-1 events'
    change: 'The `local` case names `/compact` beside `/remote-control` as a ledger-listed command with no measured post-condition on any version: on a verified CLI it exits 0 `[  ] unconfirmable`, logs `send-issued` then `send-confirmed{confirmed:false}` with no confirmation window (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`, tests/cli_send.rs:735-769).'
    sidecar: 'Path 2: /compact is unconfirmable at once on a verified CLI, its own integration case.'
    rationale: 'Report Symbols: `LOCAL_COMMANDS` appends `("/compact", None)` (ledger.rs:197, :200), mapping to `Confirms::Nothing` on a verified and an unverified CLI; Counts `local-cmd` lists test-plan.md:755; Coverage names the new cli_send case.'
    basis: '.andromeda/test-plan.md:755'
  - detector: D-tests-coverage
    severity: warning
    section: '§4. Unit Test Strategy'
    change: 'The local-command decision names the compiled list as three entries (`/clear`, `/remote-control`, `/compact`), `/compact` giving `unconfirmable` with no window on a verified and an unverified CLI, read by the `classify` rstest rows added at src/run/send/classify.rs:250-270; the function and case count of `send_local_command_` is re-read against those rows.'
    sidecar: '§4 local-command decision: /compact joins the list and its classify rows.'
    rationale: 'Report Symbols (`LOCAL_COMMANDS` 2 → 3, no product line of classify.rs changed, unit rows only) and Counts (`local-cmd` site test-plan.md:584); Coverage cites the `classify` rstest rows. The report states no new case count, so the count is for the orchestrator to re-read.'
    basis: '.andromeda/test-plan.md:584'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project)'
    change: 'The `verify_window_` class note reads that the no-screen case `verify_window_without_screens_fails_every_interactive_row` now also waits out the gate maximum in Run E settle (by design), 42.98 s on the dev host at chunk 2026-10-14-resume-and-compact-ledger-rows (34.42 s before), against the class 60 s kill and the mutants profile 45 s (about 2 s of margin locally), no kill moved; the unknown-CLI-version harness case run count is re-read (Run E spawns nothing without a Run B SessionStart target).'
    sidecar: 'quality-gate config: the no-screen verify_window_ case measured 42.98 s with Run E; no kill moved.'
    rationale: 'Report Deviations, Step 12 CI sizing (34.42 s → 42.98 s, Run E settle waits out `GATE_MAX_WAIT` with no screen, 60 s / 45 s kills unchanged); Counts `verify-runs` lists bootstrap-phases-derive-for-route-setup-project.md:10; Symbols: with no target Run E spawns nothing.'
    basis: '.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10'
    dependent-of: D-tests-coverage
  - detector: D-tests-obs-harness
    severity: warning
    section: '§3 → 5-command implementation'
    change: 'boot step 4: the stamp against the fake agent runs the print probe and all five interactive runs (Run A, Run B with its four added pastes, the dialog Runs C and D, and Run E, the `--resume` start of Run B first session) and stamps every relied ledger row (all twenty-two on Unix, the first eighteen and `resume-session` elsewhere).'
    sidecar: '§3 boot: the verify stamp runs Run E; relied 22 on Unix, 19 elsewhere.'
    rationale: 'Report Harness / gate surface: boot verify stamp now runs Run E, every relied row (22 Unix / 19 elsewhere); Counts list 5-command-implementation.md:8 as a `rows21` and `verify-runs` site. No command, status shape or log format changed, so the obs-plan §3 side needs no harness-shape change from this detector.'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:8'
  - detector: D-tests-obs-harness
    severity: warning
    section: '§3 → 5-command implementation'
    change: '`--local-live`: `LEDGER_ROWS` is `[&str; 22]` under `cfg(unix)` ending `… statusline-rate-limits · resume-session`, and the 19-id form (the first eighteen and `resume-session`) under `cfg(not(unix))`; `LIVE_PASS` names the `resume-session` line; the live firing at twenty-two rows was made at chunk 2026-10-14-resume-and-compact-ledger-rows (`evidence/live-round.md`: six starts per round, 12 of 12 under the cap, `"suite":"local-live","passed":1,"failed":0`, `stamped 2.1.287  22 pass  0 fail`, the census empty).'
    sidecar: '§3 --local-live: LEDGER_ROWS 22/19, LIVE_PASS names resume-session, the twenty-two-row live firing.'
    rationale: 'Report Harness / gate surface (crates/viola-e2e/src/harness/run.rs `LEDGER_ROWS` :335, :357, :360; `LIVE_PASS` :944-987); Counts list 5-command-implementation.md:53 as a `rows21` / `verify-runs` site and live starts per round 5 → 6; Outcome and Gates read the live round (record entry `22 pass  0 fail`, `--local-live` passed 1, census empty, the second by hand at 07:19:50Z).'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:53'
    dependent-of: D-tests-obs-harness
note: 'D-tests-framework found no drift (runner and frameworks unchanged; Dependencies none). test-plan.md:233 is a `local-cmd` site but states the list generically and stays true. The disproved killed-exit claim has no test-plan site. The obs-plan side of the harness agreement (obs-plan.md:739 and the spawn-pairs sites) is the obs-plan detector to amend.'
