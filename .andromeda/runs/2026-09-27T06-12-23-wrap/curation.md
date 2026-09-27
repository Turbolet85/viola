CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A test that scans a home byte by byte (a canary or secret scan) skips files byte-identical to the built `viola` …; remove the slow work, never raise the timeout." (confidence 0.8)
    Proof: CI run 36296402785 job 108555954043 — `run_cli run_never_writes_a_claude_canary_anywhere` TIMEOUT 10.005 s in the ubuntu mutants baseline; isolated in the WSL clone 5.089 / 5.082 / 5.064 s over a 38 354 560 B pinned copy; fixed in ed359cd (0.325 s on CI run 36298052174); overseer: "Keep any timeout raise out of the fix unless measurement says the work itself is legitimately that slow".
  Extended: T2/testing.md: "2026-09-24: Under the zero-missed mutation gate every function needs an effect a test can observe" + "a guard that repeats what the call already guarantees is just as unobservable" (confidence 0.8; no-other-home +0.2)
    Proof: pre-push union run 2026-09-27T03:0xZ breaches — `replace <impl Drop for Heartbeat>::drop with ()` and `delete ! in create_private_dir` (crates/viola-state) survived because the join and the ancestor chmod were redundant; both removed/reworked, union 0 breaches after.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup/home (C1 in test-plan §3 + verification-harness body; C5 amended into masters; C10 in arch) · 2 one-off/task-specific (C11, C13) · 1 carried as a route note (C4) · 0 conflict · 0 deferred
  Recurrences (→ handoff Deferred learnings): events.md "append + exclusive lock fails on Windows" (the lock opener was written append-only anyway) · verification-harness.md 2026-09-25 "Never pipe agent-run.sh boot" (a subprocess capture hung) · host-win32.md 2026-09-25 "Stop a process by its exact ExecutablePath" (a substring stop killed its own shell) · host-win32.md §Paths MSYS conversion (a `wsl.exe --exec /usr/bin/...` call mangled)
  No-other-home: "a guard that repeats what the call already guarantees is mutation-unobservable"
  CLAUDE.md size: 122/200 (unchanged)
