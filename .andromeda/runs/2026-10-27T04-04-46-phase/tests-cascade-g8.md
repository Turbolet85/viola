# tests · cascade read · group 8 of 8

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:9) The external-reader enumeration and the no-`unbounded_depth` ban must be held as a check a reader can run; where the plan makes either a graph-wide lint gate, it runs per OS in the Lint stage beside clippy `-D warnings`, with a probe that fires and a fixed last line, as the orphans and rmcp-features gates do.
  > `bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh` (cargo-modules 0.27.0: `cargo modules orphans -p <pkg> {--lib | --bin <name>} [--features <required features>] --deny` for every lib/bin target from `cargo metadata --no-deps`, last line `orphans-check: N/N targets clean`; the probe fires on each of its two planted orphans, last line `orphans-check probes: 2/2 fired, control clean`).
  (.andromeda/test-plan-detail/ci-integration.md:10)
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints
  (.andromeda/test-plan.md:744)
  > - **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, per OS; the `--acyclic` review and the dev-host Windows- and macOS-target check-only clippy runs are not gates.
- (.andromeda/test-plan-detail/ci-integration.md:15) A cargo-fuzz target added for a parser the chunk lands lives only in the separate `fuzz/` workspace, and its `fuzz/Cargo.lock` must pass the supply-chain audit run from the repo root.
  > `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, run from the repo root (under `fuzz/`, rustup would demand the fuzz nightly);
  (.andromeda/test-plan.md:794)
  > - Supply-chain errors: a cargo deny ban/advisory/licence/source finding, a `fuzz/Cargo.lock` advisory or source finding,
- (.andromeda/test-plan-detail/ci-integration.md:19) The per-property, per-boundary unit witnesses must run green under the local Unit stage on each OS.
  > - **Runs:** `scripts/agent-run.sh run --unit` (pwsh `scripts/agent-run.ps1 run --unit` on windows)
  (.andromeda/test-plan.md:790)
  > - Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
- (.andromeda/test-plan-detail/ci-integration.md:23) The integration witnesses (the wrapper-channel paste-ordering case, `tests/channel_links.rs`, `crates/viola-mcp/tests/handlers.rs`, `tests/chaos_feed_panic.rs`) must run green under the Integration stage.
  > - **Runs:** `agent-run run --integration`
- (.andromeda/test-plan-detail/ci-integration.md:27) Path 1's `path1_run_start_one_home_in_order`, the WATCH's subject, is an E2E test, and it must keep passing on every OS of the E2E matrix.
  > - **Runs:** `agent-run run --e2e` on all 3 OSes
- (.andromeda/test-plan-detail/ci-integration.md:31) The Path 1 WATCH is tallied only on settled reads of the per-OS `test` job, whose one instrumented run replaces the Unit, Integration and E2E rows and ends in `gate --require coverage,doctest`; the kill line does not move, and a recurrence is a red under the zero-flakiness budget, never retried.
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`
  (.andromeda/test-plan.md:750)
  > - **Coverage report:** `agent-run run --coverage` is the per-OS test job, on `windows-2025` after the ConPTY vendor verification; its one instrumented nextest run replaces the unit, integration and E2E rows, and each job ends in `viola-harness gate --require <suites>`.
  (.andromeda/test-plan.md:854)
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place. Agent-driven dev cannot distinguish a flake from a real bug; retry policies mask actual failures.
- (.andromeda/test-plan-detail/ci-integration.md:39) A bound the chunk adds to `hook`'s stdin or file reads lies in every timed row, so the `perf` job must stay green under its own `gate --require perf`.
  > Its own job `perf (${{ matrix.os }})` (`fail-fast: false`, `AGENT_RUN_KEEP_HOMES=1`): checkout, toolchain, rust-cache, `cargo install --locked hyperfine@1.20.0`, `jq --version`, `run --perf`, G2 (probe then check), `schema-check`, `secret-scan`, the scan-gated uploads, then `gate --require perf`.
- (.andromeda/test-plan-detail/ci-integration.md:47) A cargo-fuzz target the plan adds must replay its committed corpus in the `fuzz-replay` job and join the nightly per-target `fuzz` run.
  > - **Runs:** ci.yml `fuzz-replay` (ubuntu, push and PR): `rustup toolchain install` (root, then `working-directory: fuzz`), `cargo install --locked cargo-fuzz@0.13.2`, `agent-run run --fuzz-replay` (`-runs=0` corpus replay), then `gate --require fuzz-replay`. `nightly.yml` job `fuzz` runs `cargo +<channel> fuzz run --fuzz-dir fuzz <t> fuzz/corpus/<t> -- -max_total_time=120` per target (channel from `fuzz/rust-toolchain.toml`), uploading `fuzz/artifacts/` on `failure()`; no a11y step
  (.andromeda/test-plan.md:758)
  > - **Fuzz replay:** a corpus replay on push and PR, and a nightly time-boxed `cargo fuzz run` per target.
  (.andromeda/test-plan.md:792)
  > - A hyperfine budget breach, or a fuzz corpus replay crash
- (.andromeda/test-plan-detail/ci-integration.md:55) The chunk's suites are judged by `viola-harness gate`, and a check the chunk adds that greps the source takes the pinned ripgrep of the `lint` job and any `jq` the runner provides, never a second version source.
  > - **Runs:** `viola-harness gate`, which reads `junit-*.xml`, `run-summary.json`, the llvm-cov JSON summary and the hyperfine JSON, and exits non-zero on any §10 breach
  (.andromeda/test-plan-detail/ci-integration.md:59)
  > ripgrep 15.2.0, the G1/G3 tool, comes from `scripts/install-ripgrep.sh` (pinned, checksum-verified official release) in the `lint` job.
  (.andromeda/test-plan-detail/ci-integration.md:59)
  > cargo-deny and zizmor run as the pinned CLIs above; cargo-deny-action and zizmor-action are not used, so each tool has exactly one version source.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) Every timed hook's bounded stdin and `config.json` reads stay on the real logging and channel path after its strict-modes and server checks, and each row's `max` must stay under `SPINE_DEADLINE`; the fail-open matrix's oversize-stdin and malformed-JSON cases the chunk re-asserts hold the same bound at unit speed.
  > Every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init, and makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`,
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25)
  > so every timed hook takes the real logging and channel path, never the no-instance no-op (obs-plan §10 obs-overhead row)
  (.andromeda/test-plan.md:838)
  > The fail-open matrix's `Instant` bound (`hook_fails_open_silently_within_the_spine_bound`, each case `< 1.0 s`) still asserts the spine bound at unit speed as well.

## Bindings
- **Coverage report stage ↔ zero-flakiness budget**: the Path 1 WATCH is read only on the per-OS `test` job's single instrumented run (.andromeda/test-plan-detail/ci-integration.md:31), and a recurrence there keeps the chunk red with no retry or `#[ignore]` (.andromeda/test-plan.md:854).
- **Fuzz replay ↔ Supply-chain**: a fuzz target added in `fuzz/` both replays its corpus in `fuzz-replay` (.andromeda/test-plan-detail/ci-integration.md:47) and moves `fuzz/Cargo.lock`, which the root-run fuzz audit must pass (.andromeda/test-plan-detail/ci-integration.md:15).
- **Perf session ↔ Perf stage**: the bounded hook reads sit in the rows the perf session times (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26), and the `perf` job gates them by `gate --require perf` (.andromeda/test-plan-detail/ci-integration.md:39); the fail-open `Instant` bound repeats the spine at unit speed (.andromeda/test-plan.md:838).
- **Lint stage ↔ Quality gates' tool sources**: an enumeration or depth-ban check added to Lint (.andromeda/test-plan-detail/ci-integration.md:9) takes its search tool from the one pinned ripgrep (.andromeda/test-plan-detail/ci-integration.md:59).
