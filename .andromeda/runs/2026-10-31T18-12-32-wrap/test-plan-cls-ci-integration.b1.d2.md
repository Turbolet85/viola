# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the Lint commands, the sync-crates list and the macOS recipe's non-gate status hold; no CI step, `.github` or `scripts` file moved
ci-integration#1.2 · no · the supply-chain jobs and deny probes hold; `deny.toml` and the scripts are not reached, and the new root `[[test]]` adds no dependency
ci-integration#1.3 · no · `agent-run run --unit` and nextest process-per-test hold; the `ci` profile's kill-line operand in `.config/nextest.toml` is not named by this stage
ci-integration#1.4 · no · `agent-run run --integration` and its parallelism hold; the new root `cross_exit_causes` binary is not named by this stage
ci-integration#1.5 · no · `agent-run run --e2e` on three OSes and the per-OS matrix hold; the stage names no test binary or count that the new `cross_exit_causes` binary moves
ci-integration#1.6 · no · the `conpty_sideload` `[[test]]` with `fake-agent` still holds in the changed Cargo.toml; the new `cross_exit_causes` `[[test]]` is one more binary the coverage run takes with the rest
ci-integration#1.7 · no · `windows-mutants.yml` is unchanged, so the ten jobs and five root items hold; the named `src/cmd/verify/stop.rs` is not in the reached files
ci-integration#1.8 · no · the perf job steps, `run --perf` and the hyperfine gate hold; the `schemas/diag-line.v1.json` enum growth is not named here
ci-integration#1.9 · no · the MSRV 1.96 steps and the workspace `rust-version` hold; CI is green 15 of 15 on `8a4c1a2c`
ci-integration#1.10 · no · the fuzz-replay commands and corpus replay hold; `fuzz/` is not in the reached files, and CI is green 15 of 15 on `8a4c1a2c`
ci-integration#1.11 · no · the release-check `--bin viola` build and its feature judgement hold; the new `[[test]]` adds no bin or feature, and no dependency changed under `--locked`
ci-integration#1.12 · no · the gate's inputs and its §10 breach rule hold; no coverage floor, exclusion or verdict moved, and the entity names no test count
