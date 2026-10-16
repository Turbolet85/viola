
## 2026-10-14-resume-and-compact-ledger-rows — Run E, the resume variant and /compact in the tests
**Section:** §1 Test Scope Summary; §3 → `5-command-implementation`, `bootstrap-phases-derive-for-route-setup-project`; §4 Unit Test Strategy; §5 Integration Test Strategy; §6 Path 2; §7 Test Data & Fixtures
**Change:**
- **§4:**
  - The `resume-session` arm gains five `check_resume_session_*` cases, `resume_target_*` and `resume_variants_*`.
  - Relied is 22 / 19, and the older-stamp oracle covers the 21 / 18-id stamps.
  - The local-command decision names the eight `send_local_command_` functions. Was "eight functions, 15 cases"; the master now states no case count, because the code's `#[case]` rows are the count.
- **§5 `cli_verify`:**
  - five interactive runs; MM 22 / 19; `STEPS_PASS` / `ROW_IDS` `[&str; 22]` on Unix and 19 on Windows;
  - summaries 18/4 (Windows 18/1) and 20/2 (Windows 17/2); seven spawn pairs;
  - the stop case plants `.viola-verify-0-resume`;
  - new `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, its killed child asserted on `duration_ms` only.
- **§5 `contract_ledger_probes`:** `ROW_IDS` is 22 and n is 22 / 19.
- **§1:** Run E records the resume variant, and the drift contract replays it.
- **§6 Path 2:** `/compact` is `unconfirmable` at once (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`).
- **§7:**
  - The fake agent's `--resume` builds on `SessionStart.resume-1.json` where the set holds it (2.1.287, 10 key names), else `SessionStart.default` (2.1.288). The owed-payload sentence is retired.
  - The hygiene walk and the Recorded payloads row carry the variant; the 2.1.287 set is 32 files.
- **§3:**
  - Boot's stamp runs Run E (22 / 19).
  - `--local-live`'s `LEDGER_ROWS` is 22 (19-id form elsewhere) and `LIVE_PASS` names `resume-session`.
  - The twenty-two-row live firing was six starts and read `22 pass  0 fail`.
  - The no-screen `verify_window_` case went 34.42 s → 42.98 s with Run E's settle, against kills of 60 s and 45 s (mutants). No kill moved.
**Why:**
- The literals and cases follow Run E, the founder's fork 2 answer (`inputs#I1`).
- The case count is dropped on the overseer's founder-delegated word (`inputs#I7`). Its rule was unrecoverable, and by `#[case]` rows it read 14 → 24 → 32.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
