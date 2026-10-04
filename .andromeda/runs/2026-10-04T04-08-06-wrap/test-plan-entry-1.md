
## 2026-10-04-windows-boundary-mutation-workflow — the mutants profile waits for running tests
**Section:** §3 → Bootstrap phases (derive for route / setup-project) · §3 → Test data bootstrap (Cleanup)
**Change:**
- `[profile.mutants]` was `fail-fast = { max-fail = 1, terminate = "immediate" }`; now `{ max-fail = 1, terminate = "wait" }`. `slow-timeout` (5 s × 2; viola-e2e 15 s × 2) is unchanged.
- New reason: the first failure stops scheduling and running tests finish, so their temp dirs and session guards drop. The slow-timeout kill bounds any hang below cargo-mutants' 20 s floor, so a caught mutant ends at the kill line and is never graded Timeout.
- The retired reason was that a plain `fail-fast = true` let a caught mutant hang into a Timeout grade; it now reads as history, bounded by the slow-timeout kill.
- Measured: 0 Timeout grades over 711 Linux viola-e2e and 508 Windows mutants; 170 vs 0 leftover temp dirs two-sided; after a full viola-e2e run, 38 `.tmp*` and 0 nested copies against 25 275 and 62. The 38 are 17 kill-path leftovers by design and 21 half-removed fixture git repos (a `terminate`-independent class). A full Linux viola-e2e run takes 78 m against 23 m, counts identical.
- Cleanup: the killed-test example was "nextest `terminate = \"immediate\"`"; it now names the slow-timeout kill or a mutant-made kill, with `wait` letting every other running test finish.
**Why:** a REVERSAL of the 2026-09-24 chunk-level locked choice, ratified by the overseer as operator (founder-delegated) on 2026-10-04 on the measured basis. It is the leak's mechanism fix, not a cleanup step. The 21-repo remainder is an `[inferred]` hypothesis owned by the next chunk.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/
