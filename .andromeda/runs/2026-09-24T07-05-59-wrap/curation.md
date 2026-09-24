CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + host-win32.md: "A running .exe cannot be relinked on Windows … uses a separate CARGO_TARGET_DIR"
      Proof: implement gate 4 log `failed to remove file …\target\debug\viola-harness.exe … Access is denied (os error 5)`; CARGO_LOG fingerprint trace `UnitDependencyInfoChanged` (.andromeda/runs/2026-09-24T06-07-48-implement/fp.log); fixed by target/harness, gates green.
    + host-win32.md: "Stopping a Monitor/background task leaves its tail.exe/grep.exe running …"
      Proof: gate 14 logs `move "…\mutants.out" to "…\mutants.out.old" … Access is denied` twice; `Get-CimInstance Win32_Process` showed tail.exe 14096 (`tail -n +1 -F mutants.out/missed.txt …`) + grep 50388/54492 alive after TaskStop; stopped by pid → the next run completed (286 mutants).
    + testing.md: "Under the zero-missed mutation gate every function needs an effect a test can observe …"
      Proof: CoreError / VIOLA_DIR / detach flags dropped as unkillable; run/boot cargo orchestration tested via mini temp projects; mutation run 286 mutants, 0 missed, 0 timeout.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 4 dup (nextest fail-fast Timeout, nextest unmatched binary() parse error, serde_json preserve_order, mutation diff form — each now in an amended master) · 0 task-specific · 0 conflict · 2 deferred (→ handoff: rg absent from the gate shell PATH; `grep | grep -c` under pipefail reads green on a missing file)
  CLAUDE.md size: 120/200 · T1 0 new
