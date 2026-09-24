# Session Handoff

**Last Updated:** 2026-09-24T18:00:38Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 0-pending wrap — chore(route): operator-requested adaptation — the Epoch 1 cleanup chunk goes in at the Epoch 2 head

## Position
- Done: the Epoch 1 boundary work. There was no chunk to wrap: 0 pending, Epoch 1 is 8/8 complete.
  - `/andromeda-evolve-diagnose` ran for Epoch 1: `runs/2026-09-24T17-37-22-evolve-diagnose/proposals.md` (10 proposals, 8 level candidates, 2 extension candidates).
  - `/andromeda-code-audit` wrote the first ledger record (baseline) to `.andromeda/code-metrics.ndjson`; the report is `runs/2026-09-24T17-47-08-code-audit/proposals.md`.
  - Operator route adaptation: a new first Epoch 2 entry, "Epoch 1 cleanup", sits ahead of "Security prerequisites". The record is `runs/2026-09-24T18-00-38-wrap/adaptation-record.md`.
- Next: `/andromeda-phase` to promote and plan "Epoch 1 cleanup". It carries the Rust gate-deferral PREREQ, which moved here from Security prerequisites: this is the first chunk with a Rust delta.
  - A CARRY holds the measured coordinates.
  - A second CARRY asks that chunk's wrap P2 to propose a playbook rule keeping obs-plan §1 out of the cascade sweep.

## Work done
- No source changed. Route: 1 entry inserted, 1 PREREQ moved (the diff is 3 added / 1 removed; no frozen line touched).
- The v1-23 condition is met: all three `release` legs read `success` on a28f696 (CI run 36032014621). The matrix reads 3/53 verified.

## Notes
- **Operator decisions:**
  - the cleanup chunk as printed, including the run.rs split (the overseer's reason: it gives viola-e2e its first mutation witness);
  - items 4–5 are out of the chunk.
- **The run.rs split puts 154 of viola-e2e's 431 mutants in the in-diff scope, per CI leg.** The estimate is 50+ min per leg, from the slowest viola-e2e test at 20.6 s.
- **Host, operator-owned:** cargo-nextest 0.9.133 → the CI pin 0.9.146 (`cargo install --locked cargo-nextest@0.9.146`). The overseer installs it.
- **Code-metrics ledger correction owed:** record `ts 2026-09-24T17:51:10Z` (sha a28f696) lists `mutation.survivors` sites without their column (`lib.rs:9`), so its 4 rows read as 2 duplicated pairs. The true sites are `:9:31`/`:9:38` × {+, /}. The next ledger-mode record carries this in its `corrections[]`.
- **Unmeasured hypothesis:** the claim that fuzz's `arbitrary` reaches `fuzz_target!` through libfuzzer-sys's re-export. libfuzzer-sys is not in the host registry. The item-4 disposition stands on the measured spec pin.
- **Deferred learnings (filtered at this wrap's curation):**
  - The epoch-boundary cleanup-chunk convention scored 0.5. It is kept in the operator's auto-memory, not the repo.
  - The fuzz `arbitrary` machete false positive scored 0.2, because its mechanism is unmeasured.
  - The prior `git check-ignore` learning stays unreproduced.
- The operator's viola-lab prototype was running earlier this session; it is not this project's.
- Last failed command: none open.
