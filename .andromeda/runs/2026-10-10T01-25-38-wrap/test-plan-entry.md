
## 2026-10-09-epoch-3-cleanup-ii — the shared root helpers named, the first whole-member viola-e2e score, the waits counted by pattern
**Section:** §2 Test directory + naming conventions (the sync E2E suites bullet) · §10 Mutation gate · §3 → 5-command implementation · §3 → Bootstrap phases (derive for route / setup-project)
**Change:**
- §2: the helper list is `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}` (was without `cli.rs`). `events.rs` holds the one `events.ndjson` reader, its wait and the `boot` that waits for the session-start record; `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), stdout and stderr two captures beside the exit code. No root test file outside `tests/support/` defines its own copy.
- §10: the first whole-member `viola-e2e` score on the Linux dev host: 718 mutants in 5356 s (89 min 16 s), 656 caught, 2 missed, 0 timeout, 60 unviable. The 2 missed are in `prepare` (`crates/viola-e2e/src/harness/run/mutants/scratch.rs`), reachable only on a Windows host, recorded not measured here and owed to the route entry "Windows mutation grade".
- §3 → 5-command implementation: `WITHIN` is "shared by the root waits on a child", 22 sites of `Instant::now() + WITHIN` in 16 files, the rule stated in architecture §Occupied Resources → Filesystem (was "shared by all 9 root waits on a child"); `wait_endpoint_gone` is named without "the ninth wait".
- §3 → Bootstrap phases: beside "78 m against 23 m under `immediate`" stands this chunk's whole-member run, 718 mutants in 5356 s with 0 timeout.
**Why:** the chunk lifted the per-file scaffolding into two shared files, took the member's first whole-unit score on the dev host, and moved the number of waits on `WITHIN`; the 9 was a named list no wrap had re-taken. `events.rs` was named in §2 before the file existed.
**Kept:** "78 m" and "0 Timeout grades over 711 Linux viola-e2e mutants" stand as measured at their chunk. The mutation gate's rule and the boundary tier's form are unchanged.
**Ref:** .andromeda/runs/2026-10-10T01-25-38-wrap/
