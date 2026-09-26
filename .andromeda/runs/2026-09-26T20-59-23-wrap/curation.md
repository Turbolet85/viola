# Curation — 2026-09-26-ci-chunk-base-and-union-verdict

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "judge a mutation leg by its verdict, never by comparing its counts with another run's or another host's" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 3 below threshold (outcomes.json-in-evidence 0.5; `-G` pickaxe hazard — test-plan §3 now carries it; cascade id length — tool telemetry, not a project learning)
  Load-bearing: "judge a mutation leg by its verdict, not by count comparison" → Local Linux pre-push gate
  Extended: T2/host-win32.md: "2026-09-24: Stopping a Monitor/background task … blocks renames of … `mutants.out`" + "rust-analyzer holds it too and restarts after every edit — stop it by exact ExecutablePath before each run --mutants" (confidence 0.9)
  CLAUDE.md size: 122/200 · T1 1.5 KB, 0 over 600 B

## Proofs
- host-win32.md extension: implement gate run 1, entry 7 (`bash scripts/agent-run.sh run --mutants`) failed `move "…\mutants.out" to "…\mutants.out.old" … Access is denied. (os error 5)` with three rust-analyzer processes running; after stopping them by exact ExecutablePath the re-run passed the rename (`Found 81 mutants to test`). rust-analyzer was running again (3 processes) at the next check, after file edits; stopped before runs 3 and 4 (`evidence/mutants-leg-windows-local.md`). Signals: verified by a real gate failure +0.4, 3 separate instances +0.3, specific detail +0.2 = 0.9. Filter 1: additive facet (a holder the matched entry lacks) — not a recurrence of that entry, whose tail/grep remedy did not fail; the same fact sat uncurated in the previous handoff's Deferred learnings (0.7), which this extension retires.
- verification-harness.md entry: two local windows runs over the identical final tree read caught 73 / unviable 6 and caught 71 / unviable 8 (79 tested both); CI's two legs on the same tree read 74 / 5 each (`evidence/mutants-leg-windows-local.md`, `evidence/operator-pass.md`). Signals: verified by measurement +0.4, specific detail +0.2 = 0.6, + load-bearing for "Local Linux pre-push gate" (it runs the ubuntu leg locally before the push, whose counts will not match CI's; the entry carries no such annotation) +0.2 = 0.8.
