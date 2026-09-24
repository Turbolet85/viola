# tests extract

## Relevance
partial. The chunk is CI and policy gates, not product test code. test-plan owns the supply-chain gate as a testable entity and fixes its CI stage shape, tool pins, artifacts and failure conditions.

## Constraints
- The "dependency and boundary policy" entity (cargo-deny tokio and C-crate bans, the tokio-free `cargo check` of the sync crates, SHA-pinned Actions and zizmor) is classed as testable through exit-code gates in CI (per test-plan §1 Coverage scope). The supply-chain security-vector trigger requires four CI gates that act on exit code: `cargo deny check` with RUSTSEC-2017-0008 as the only ignore, `cargo check` of the sync crates without tokio, zizmor on the workflows, and an assertion that Actions are SHA-pinned (per test-plan §1 security-vector-coverage (supply chain)).
- Placement:
  - `cargo deny --format json check` and `zizmor --format=json .github/workflows/` run on ubuntu only.
  - The no-tokio `cargo check` runs in the Lint stage of the per-OS matrix (per test-plan §9 Pipeline structure, Lint row; §9 Matrix builds).
  - The Lint row lists `-p viola-core -p viola-pty -p viola-state -p viola-channel -p viola-agent-claude`, and most of those packages do not exist yet. How to name only the crates present now, without a vacuous pass, is research's question (see scope inferred premise 1).
- Tool sourcing:
  - cargo-deny 0.20.2 comes through the SHA-pinned taiki-e/install-action v2.87.19 `tool:` list.
  - zizmor 1.30.1 comes from `cargo install --locked zizmor@1.30.1` in the job that runs it.
  - cargo-deny-action and zizmor-action are not used, so each tool has exactly one version source (per test-plan §9 tool paragraph; §3 Bootstrap `ci-tool-install`).
- Workflow hygiene: `permissions: {}` at the top of the workflow, `contents: read` on each job, and every `uses:` SHA-pinned, with zizmor `unpinned-uses` as the assertion (per test-plan §9 paragraph after the Pipeline table; §3 Bootstrap `quality-gate-config-emit`).
- Weekly advisories: test-plan names a scheduled `nightly.yml` as their home, alongside fuzz (per test-plan §9 Platform). The scope leaves open whether they go there or in a `ci.yml` `schedule:`. A choice that departs from §9 needs a test-plan amendment.
- Build-failure conditions include: the sync crates failing to build without tokio, a cargo-deny ban, advisory or licence finding, and any zizmor finding (per test-plan §9 Build failure conditions; §10 Build failure conditions).
- Caching: Swatinem/rust-cache is the only allowed cache and belongs in `ci.yml` only (per test-plan §9 Pipeline structure, Cache column; §11 CI). A scheduled advisory workflow therefore gets no cache.

## Patterns to follow
- Upload the cargo-deny and zizmor JSON outputs as CI artifacts, so the agent reads them with `gh run download` and not from log text (per test-plan §9 Test report format).
- A gate tool that is missing must exit non-zero with `reason:"tool-missing"` and the tool name, never pass silently (per test-plan §3 Bootstrap `ci-tool-install`, the `--perf` / `--fuzz-replay` precedent). The same rule applies if the dependency-policy gate is exposed through `scripts/agent-run.{sh,ps1}` and cargo-deny or zizmor is absent on the host.
- Event-payload values reach steps only through `env:`, as the Mutation row already does with `AGENT_RUN_CHUNK_BASE` (per test-plan §9 Pipeline structure, Mutation row).
- If the gate is exposed through the harness, both shell variants must have identical semantics (per test-plan §3 Bootstrap `5-command-discipline-wire`). Adding a new `run` / `gate` `suite` value needs a Decisions Log entry first (per test-plan §3 Closed enums).

## Anti-patterns to avoid
- NEVER reference an Action by a mutable tag (`@v2`, `@main`); SHA-pin everything, with `permissions: {}` (per test-plan §11 CI).
- NEVER skip a quality gate "just this once". This covers an advisory ignore or a skip added to get the deny gate green, beyond the single RUSTSEC-2017-0008 exception (per test-plan §11 Quality; §1 supply-chain trigger).
- NEVER add a cache beyond Swatinem/rust-cache in `ci.yml` (per test-plan §11 CI).

## Contract bindings
- tests ↔ security §Dependency Security: cargo-deny runs as an exit-code gate in the same `ci.yml` as the test jobs. The advisory and ban set is authored by security, and tests owns the gate wiring and artifacts (per test-plan §1 supply-chain trigger; §9 Lint row).
- tests ↔ arch §Build system / §CI/CD: the tokio ban and the sync-crate `cargo check` come from the arch Stack. test-plan §9 fixes the stage placement: CI job 3 per OS and job 4 on ubuntu.
- tests ↔ obs otel-sdk-install / pii-scrubbing-wire: the telemetry-crate bans and the `veil` `toggle` feature ban are enforced by the deny gate that tests wires. A failing deny check is a build failure (per test-plan §9 Build failure conditions).
- tests ↔ harness mutation gate: this chunk's diff goes through `agent-run run --mutants` like every other chunk (per test-plan §10 Mutation gate).

## Acceptance criteria contributions
- On ubuntu, `cargo deny --format json check` over advisories, bans, licenses and sources exits 0 with RUSTSEC-2017-0008 as the only ignore, and its JSON is uploaded as a CI artifact. Each ban's live effect is shown by a negative probe that makes the gate exit non-zero (per test-plan §1 security-vector-coverage (supply chain); §9 Test report format).
- On ubuntu, `zizmor --format=json .github/workflows/` exits 0 over every workflow, the new scheduled one included, and its JSON is uploaded. Every `uses:` is SHA-pinned, and each workflow has `permissions: {}` at the top and `contents: read` on each job (per test-plan §9 paragraph after the Pipeline table).
- The tokio-free `cargo check` of the sync crates that exist runs and passes in the Lint stage on windows-2025, macos-latest and ubuntu-latest (per test-plan §9 Pipeline structure, Lint row; §9 Matrix builds).
- The `mutants` job verdict for this chunk's diff is `no-rust-delta` if no `.rs` path is touched. If a `.rs` path is touched, the verdict is `counted`, with a fresh `outcomes.json` showing `missed == 0` and `timeout == 0` (per test-plan §10 Mutation gate; §3 Closed enums).

## Relevant amendment history
- 2026-09-24-fake-agent-and-test-data-fixtures, "mutation verdict for Rust-free diffs" (§3 `run` step 4, §10 Mutation gate): a diff with no `.rs` path now passes as `{"tested":0,"verdict":"no-rust-delta",…}` and never reaches cargo-mutants. The reason is that cargo-mutants 27.1.0 exits 0 on a Rust-free diff and leaves a stale or missing `outcomes.json` (CI run 35973118026). This chunk's diff is mostly `deny.toml` and workflow YAML, so this is the verdict path it will likely take unless it adds `.rs` probes.
- 2026-09-24-three-os-ci-headless-harness-skeleton, "integration filterset, mutation diff, base and trigger" (§9 Mutation row): the `mutants` job runs on push and pull_request, and its base sha is passed through `env:`. This is the existing precedent for routing event payloads through `env:`, which this chunk's least-privilege and zizmor template-injection rules must keep when editing `ci.yml`.
- 2026-09-24-three-os-ci-headless-harness-skeleton, "nextest mutants profile and toolchain source" (§9 CI Integration): rustfmt and clippy come from `rust-toolchain.toml` through `rustup toolchain install` (no toolchain action), and the pin is exactly 1.98.1. The deny, zizmor and tokio-free jobs this chunk adds or edits should use the same toolchain source and not add a toolchain Action.
