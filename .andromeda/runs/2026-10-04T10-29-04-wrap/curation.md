CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "The fake agent reads its `--control` file from byte 0 at every start, so a test that restarts a wrapper over a gated `--script` replays every release already in that file — restart without the script (or with a fresh control file) when the turn must not fire again." (confidence 0.8)
    Proof: gate entry 8's first run at /implement (`implement-2026-10-04T09-55-57` trail, 8.log) — `last_survives_a_wrapper_restart` read a second `turn-ended` (ts 10:09:20.006Z vs 10:09:19.889Z) after `stop_keep` + a scripted re-boot; green once the re-boot dropped the script (report.md Deviations). Signals: +0.4 measured (a real gate failure), +0.2 technical detail, +0.2 no other durable home (no master, route annotation or matrix note carries it).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
    - rejected at exactly 0.6 (Filter 4): "update an in-memory view of a log under the same lock hold as the append, or a reader who saw the line on disk can read stale state" — architecture §Standard Contracts `wait` / `last` now carries it (this wrap's P2), so neither conditional signal applies.
    - rejected at exactly 0.6 (Filter 4): "an envelope struct that types a method param pre-empts the method's own -32602 with -32600" — architecture §Standard Contracts (`from`) and security-plan Channel frames now carry it.
    - rejected at 0.3 (Filter 4): `Instant::checked_add` of `u64::MAX` ms does not overflow on Linux (one-off, -0.3).
    - Filter 1 hit, not a recurrence: session-learnings.md "Run-dir hygiene: the operator's disposition for phase-run CI copies" — a disposition convention this pass followed (uncited copies deleted), not a defect record.
  No-other-home: "The fake agent reads its `--control` file from byte 0 at every start …"
  CLAUDE.md size: read at P7 (health check 1)
