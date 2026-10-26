# tests extract

## Relevance
partial — the chunk adds no product surface and no new test layer; the tests domain binds it through the supply-chain, lint and coverage gates that must read green after the tokio pin and lock move.

## Constraints
- test-plan §1 Test Scope Summary (trigger: security-vector-coverage, supply chain) requires the supply-chain trigger to be held by CI exit-code gates: `cargo deny check` with RUSTSEC-2017-0008 as the only ignore, the tokio-free `cargo check` of the sync crates, zizmor and the SHA-pinned Actions assertion; the dependency and boundary policy entity is "testable (exit-code gates in CI)" (per test-plan §1 Test Scope Summary).
- test-plan §9 CI Integration (Supply-chain stage) requires the ubuntu `supply-chain` job to run fail-closed: `cargo deny --format json check` over the root lockfile, the fuzz lockfile's `check advisories sources` from the repo root, the sole-root `deny-sync.toml` tokio ban once per crate of `scripts/sync-crates.txt`, `bash scripts/channel-tokio-free.sh` (presence read from the printed `tokio v` line, never the exit), `bash scripts/deny-probes.sh` and zizmor, with their JSON under `target/supply-chain/` (per test-plan §9 CI Integration).
- test-plan §9 CI Integration (Build failure conditions) counts as red: any cargo deny ban/advisory/licence/source finding, a `fuzz/Cargo.lock` advisory or source finding, a sync crate failing the tokio ban, `channel-tokio-free.sh` not ending `channel-tokio-free: ok`, `deny-probes.sh` not ending `deny-probes: 13/13 banned, control clean`, `rmcp-features.sh` not ending `rmcp-features: server transport-io`, and `release-check.sh` not ending `release-check: viola only` (per test-plan §9 CI Integration).
- test-plan §9 CI Integration (tool pins) requires ci.yml to be each tool's one version source (`cargo-deny@0.20.2` on the `test` job's `tool:` line); cargo-deny 0.20 takes `--config <path>` as a global option before the subcommand, so a local deny reading is taken at that pinned version with that spelling (per test-plan §9 CI Integration).
- test-plan §10 Quality Gates & Coverage Targets requires the per-OS coverage gate (`--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`) and lists a cargo-deny failure as a build failure; the bump must hold both, and the zero-flakiness budget (`retries = 0`) means a test that turns red after the bump is fixed at its cause inside this chunk, never retried (per test-plan §10 Quality Gates & Coverage Targets).
- test-plan §8 Mocking & Stubbing Discipline has `viola-mcp` use tokio's paused clock for a timer it owns, and test-plan §5 Integration Test Strategy has the MCP handler tests (`crates/viola-mcp/tests/handlers.rs`) drive the Tokio client behind `viola-channel`'s `tokio` feature; these suites are where a tokio behaviour change would show, so they must stay green on the new version (per test-plan §8 Mocking & Stubbing Discipline; per test-plan §5 Integration Test Strategy).
- test-plan §2 Test Strategy (directory pattern) requires Tokio-based test clients to live only in the test-only `crates/viola-e2e`, so the root package and the sync crates carry no tokio dev-dependency; the bump must not add a tokio edge anywhere else (per test-plan §2 Test Strategy).

## Patterns to follow
- The local Linux gate is the harness's `pre-push` (stages `host` · `tools` · `linux-tests`, the last `run --coverage` then `gate --require coverage,doctest`), its verdict the one JSON document and exit code, never a human message (per test-plan §3 `pre-push`).
- The per-OS test row is `agent-run run --coverage` ending in `viola-harness gate --require <suites>`; unit / integration / E2E locally through `agent-run run --unit` / `--integration` / `--e2e` (per test-plan §9 CI Integration).
- CI verdicts are read with `gh run view --json jobs` and artifacts (the `supply-chain` JSON) fetched with `gh run download`, never inferred from a local run (per test-plan §9 CI Integration).
- The Lint stage's tokio-free `cargo check` takes one `-p` per crate of `scripts/sync-crates.txt` with no `--features`, `viola-channel`'s normal graph held by the tree form instead (per test-plan §9 CI Integration).

## Anti-patterns to avoid
- NEVER skip quality gates "just this once" or lower a threshold to pass the build: a deny, lint or coverage red after the bump is fixed, not waived (per test-plan §11 Test Anti-Patterns, Quality).
- NEVER treat a Linux-only local run as proof for Windows or macOS: the tokio move reaches `viola-mcp`, `viola-channel`'s `tokio` feature and `viola-e2e` on all three runners, so the full CI row is the evidence (per test-plan §11 Test Anti-Patterns, Test Strategy).
- NEVER introduce a failpoint crate (or any dependency) that pulls tokio into the sync crates (per test-plan §11 Test Anti-Patterns, Mocking).

## Contract bindings
- tests §9 Supply-chain stage ↔ security-plan §Dependency Security: the `cargo deny` advisories/bans run in the same CI job, the ignore list (RUSTSEC-2017-0008 only) and the no-widening rule are security's; tests owns the gate's red/green conditions and artifacts.
- tests §9 Lint (`scripts/rmcp-features.sh`, the sync-crate tokio-free check) ↔ architecture Stack (Tokio only in `viola-mcp`, rmcp `server` + `transport-io`).
- tests §3 `pre-push` `tools` stage ↔ tests §9 tool pins: the host's tools are checked against ci.yml's `tool:` line, so ci.yml stays the one version source.

## Acceptance criteria contributions
- CI `supply-chain` green on the chunk's push: root `cargo deny check` with no finding (no RUSTSEC-2026-0386) and the ignore list unchanged, the fuzz lockfile's audit clean, every sync crate passing the sole-root tokio ban, `channel-tokio-free: ok`, `deny-probes: 13/13 banned, control clean`, zizmor clean (per test-plan §9 CI Integration).
- Lint holds on every OS: `cargo fmt --check`, clippy `--workspace --all-targets --features fake-agent -- -D warnings` (whether any viola call site uses the now-deprecated safe `AsyncFd` constructors is research's question), the tokio-free `cargo check` per sync crate, and `rmcp-features: server transport-io` (per test-plan §9 CI Integration).
- `bash scripts/agent-run.sh pre-push` reads `ok:true` on the Linux dev host, and each OS's `test` row's `gate --require` passes at ≥ 85% lines / ≥ 95% functions / ≥ 80% regions, including the `viola-mcp` handler and `viola-e2e` MCP suites (per test-plan §10 Quality Gates & Coverage Targets).
- `scripts/release-check.sh` ends `release-check: viola only` and its probe `release-check probes: 5/5 refused, control clean` (per test-plan §9 CI Integration).
