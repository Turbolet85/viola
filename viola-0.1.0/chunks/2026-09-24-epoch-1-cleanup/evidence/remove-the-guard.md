# Remove-the-guard runs — 2026-09-24-epoch-1-cleanup

Founder rule: every new guard test carries its remove-the-guard mutation run.

## `read_diagnostics_level_reads_at_most_sixteen_mib` (src/obs.rs, plan step 3)

The guard is `file.take(MAX_FRAME).read_to_end(&mut bytes)` at `src/obs.rs:195`.

| run | tree | command | reading |
|---|---|---|---|
| 1 | guard in place | `bash scripts/agent-run.sh run --unit --filter 'test(/max_frame_is_sixteen_mib\|read_diagnostics_level_reads_at_most/)'` | exit 0 · `2 tests run: 2 passed` |
| 2 | guard removed (`Ok(mut file) => match file.read_to_end(&mut bytes)`) | `… run --unit --filter 'test(/read_diagnostics_level_reads_at_most/)'` | exit 1 · `FAIL obs::tests::read_diagnostics_level_reads_at_most_sixteen_mib` · `left: (Level(Debug), None)` / `right: (Level(Info), Some(Malformed))`: the object past 16 MiB was read |
| 3 | guard restored | same as run 2 | exit 0 · `1 test run: 1 passed` |

Restore proof: `git diff -U0 src/obs.rs` has no changed line naming `take(MAX_FRAME)`, and `grep -n 'take(MAX_FRAME)' src/obs.rs` finds only
`195:        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {`.

## `max_frame_is_sixteen_mib` (crates/viola-core/src/lib.rs, plan step 2)

The guard is the constant's value (`lib.rs:9`). Its remove-the-guard run is the dedicated cargo-mutants gate entry over `lib.rs`. Its
reading is recorded in `mutants-max-frame.md` once the gate block runs.
