# MAX_FRAME pin — remove-the-guard mutation run (plan step 2, gate entry 9)

The guard is the value of `pub const MAX_FRAME: u64 = 16 * 1024 * 1024` at `crates/viola-core/src/lib.rs:9`. cargo-mutants 27.1.0 mutates
its arithmetic. `max_frame_is_sixteen_mib` must catch every such mutant.

| run | tree | reading |
|---|---|---|
| P5 baseline (phase run, untouched tree) | no pin test | `12 mutants tested in 25s: 4 missed, 8 caught`. The 4 missed are `lib.rs:9:38` and `:9:31` × {`*`→`+`, `*`→`/`}. exit 2 |
| /implement gate run 1 (2026-09-24, `implement-2026-09-24T18-26-58/9.log`) | pin test added | `12 mutants tested in 25s: 12 caught`; `missed.txt` + `timeout.txt` count `0`; exit 0 |

The 4 audit survivors (code-audit B1) are now caught by viola-core's own tests, which is the package that owns the mutated file (test-plan
§3 `run` step 4, Test scope).
