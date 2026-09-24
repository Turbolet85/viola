# architecture — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — toolchain floor and exact pin
**Section:** §Stack and Technologies (Language / runtime) · §Infrastructure Patterns (Build system; Project directory structure `rust-toolchain.toml` comment) · §Inherited Defaults (Framework)
**Change:** Rust 1.98.1 is pinned exactly by `rust-toolchain.toml` (rustfmt + clippy). The workspace `rust-version` is 1.96, not 1.89. The "1.89 is the highest floor" claim is replaced by "1.96 is the declared floor", with sysinfo 1.95, `File::lock` 1.89, rmcp 1.88 and axum 1.80 listed below it.
**Why:** intent F-19 / matrix v1-19; the chunk shipped `rust-version = "1.96"` and `channel = "1.98.1"`. Sweep `1\.89|rust-version = "1\.89"|MSRV 1\.89|channel = "stable"|host 1\.95|Rust stable` over all 7 masters: 4 arch sites amended; obs-plan.md:1565 no change (§12 Decisions Log history); 0 elsewhere.

## 2026-09-24-three-os-ci-headless-harness-skeleton — CI setup and wired jobs
**Section:** §Stack and Technologies (CI/CD row) · §Infrastructure Patterns (CI/CD approach)
**Change:**
- CI now installs the toolchain with `rustup toolchain install` from `rust-toolchain.toml` (no toolchain action) and uses SHA-pinned actions/checkout 7.0.1, Swatinem/rust-cache 2.9.2, taiki-e/install-action 2.87.19 and actions/upload-artifact 7.0.1.
- The workflow sets `permissions: {}` at the top and `contents: read` per job.
- Two jobs are wired today: `test` (3-OS harness unit/integration plus the lifecycle leg, with an `agent-run-<os>` upload) and `mutants` (ubuntu, push + PR, base via `env:`).
- The six target jobs are kept and marked as owned by later chunks.

**Why:** the chunk shipped `.github/workflows/ci.yml` (report: Harness / gate surface). Sweep `dtolnay|rust-toolchain@stable|@stable` over all 7 masters:
- 2 arch sites amended; 2 security sites amended (security-plan sidecar);
- test-plan.md:1398 and :1403 amended (test-plan sidecar); test-plan.md:756 (nightly fuzz toolchain) and :1393 (MSRV 1.96 job) no change, because they install a different toolchain;
- security-plan.md:548 no change (the mutable-ref ban still holds).

## 2026-09-24-three-os-ci-headless-harness-skeleton — test-only crate, bins, env vars, paths
**Section:** §Established Decisions [Module Boundaries] · §Occupied Resources (Binary; Workspace crates; Environment variables; Filesystem; Repository) · §Conventions (Naming — environment variables) · §Infrastructure Patterns (Project directory structure)
**Change:**
- Registered the test-only crate `viola-e2e` (harness library + `viola-harness`, no-op `fake-agent` feature) and the test-only bins `viola-fake-agent` (root `[[bin]]`, feature `fake-agent`) and `viola-harness`.
- Registered the harness-only env vars `AGENT_RUN_CHUNK_BASE` and `AGENT_RUN_KEEP_HOMES`, and the `AGENT_RUN_` prefix for harness-only variables.
- Registered the home-level `diagnostics/` (`run-<name>.ndjson`, 0700/0600) and the repository paths `target/agent-run/`, `target/e2e-home/` and `target/harness/`.
- Added `src/bin/viola-fake-agent.rs`, `tests/`, `crates/viola-e2e/`, `scripts/agent-run.{sh,ps1}` and `.config/nextest.toml` to the tree.
- Module Boundaries states that each product crate is created by its first consumer.

**Why:** report Changes (Crates, Symbols, Env vars; expected amendment 3; `grep -c` of each name in architecture was 0). The child env the harness sets (`PATH`, `CARGO_TARGET_DIR`, `NEXTEST_PROFILE`) was not registered: registry over-reach, because the arch registry tracks variables the product reads or sets.

## 2026-09-24-three-os-ci-headless-harness-skeleton — run's process log location, logging and serialization rows
**Section:** §Cross-cutting Patterns (Diagnostic output channels) · §Stack and Technologies (Serialization; new Logging row)
**Change:**
- `run`'s codes-only process log is the home-level `diagnostics/run-<name>.ndjson`; content-bearing detail stays in the instance's `diagnostics/`, per obs D-08.
- New Stack row: tracing 0.1.44 + tracing-subscriber 0.3.23 (root only).
- serde_json carries `preserve_order` (indexmap), so printed documents keep their declared key order.

**Why:** report Symbols (`viola run` appends the home-level role file), Dependencies (tracing, tracing-subscriber, serde_json `preserve_order`) and Spec claims disproved 5 (key order). Sweep `Its diagnostics go to the instance|diagnostics go to`: 1 arch site amended; 0 in other masters.

## 2026-09-24-fake-agent-and-test-data-fixtures — test homes, test env vars and test-side paths registered
**Section:** §Occupied Resources (Environment variables · Filesystem · Repository) · §Infrastructure Patterns directory tree
**Change:**
- Env vars: `AGENT_RUN_KEEP_FAILED` registered (read only by the root test chain). `AGENT_RUN_KEEP_HOMES` is now read by `viola-harness` and the root test chain. None is read by `viola`.
- e2e-home: `viola-session-*/home` (harness) and `viola-test-*/home` (root rstest) both named.
- Repository gains `fixtures/fake-scripts/`, `schemas/fake-script.v1.json` and `crates/viola-core/proptest-regressions/`.
- Filesystem gains the test-home-only `fake/<name>.control` / `fake/<name>.receipt.ndjson`.
- The tree gains `tests/support/`, `schemas/`, `fixtures/fake-scripts/` and viola-core `proptest-regressions/`.

**Why:** report Spec claims disproved 1 (architecture.md:374 claimed `viola-session-*` held every test home; root tests use `viola-test-*`) and 2 (:359 said only `viola-harness` reads `AGENT_RUN_KEEP_HOMES`); report Files / Env vars / Symbols (Wrapper::boot paths); plan expected amendment.
- Rejected: `tests/support/` as a Repository entry (registry over-reach; the tree carries it), and a §Stack Testing row (the invariant holds, because [Deferred] :106 hands test libraries to test-plan, which pins them).
- Sweep `every harness and test home`, `read by \`viola-harness\` and never`, `e2e-home`, `AGENT_RUN_KEEP` over all seven masters: 2 arch sites amended (:359, :374). The CI-jobs line :448 (`AGENT_RUN_KEEP_HOMES=1` set by ci.yml) needs no change. The test-plan and obs-plan `e2e-home` sites name the directory, not a prefix claim, so they need no change. test-plan :596 (harness cleanup removes its own `viola-session-*` parent) is accurate as written, and `viola-session-row` hits in a11y/test-plan are UI element names.

## 2026-09-24-supply-chain-and-workflow-gates — sole-root tokio ban, four-family deny policy, nightly.yml
**Section:** §Stack Code-quality row · §Established Decisions [Concurrency] · §Occupied Resources → Repository · §Infrastructure Patterns → Build system · directory tree · CI/CD approach
**Change:**
- Build system: the tokio ban moved to `deny-sync.toml`, run once per crate in `scripts/sync-crates.txt` as the sole root. It is no longer checked over a graph with the async crates excluded. `--exclude` false-fails under feature unification, as measured at research.md §Measured facts; the `viola-channel` own-root limit is recorded. `deny.toml` now carries four families: advisories, licences, sources, and bans (C, telemetry, feature).
- [Concurrency] enforcement text names the sole-root ban and the sync-crate list.
- Code-quality row: cargo-deny families corrected; zizmor 1.30.1 added (workflow lint).
- Occupied Resources: `target/deny-probes/` and `target/supply-chain/` registered.
- Tree: `deny-sync.toml`, `scripts/sync-crates.txt`, `scripts/deny-probes.sh` and `workflows/nightly.yml` added; the `deny.toml` comment is corrected.
- CI/CD: two workflows (`ci.yml` push/PR, `nightly.yml` weekly + dispatch). `ci.yml` jobs are now `test`, `mutants`, `lint`, `supply-chain`. Target job 3 reads `scripts/sync-crates.txt`. zizmor is installed by `cargo install --locked`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates shipped these. The report's "Spec claims disproved" #1 falsified the excluded-graph mechanism. The P4 operator decision placed the weekly run in `nightly.yml`. Of the fan-out's 9 D-arch proposals, 8 were applied as re-derived. The Occupied-Resources proposal was applied only in part: the two `target/` dirs were registered, and its config files were routed to the tree (playbook "Registry over-reach").

Sweep over all seven masters, patterns `wrappers list`, `excluded, not with`, `one workflow .ci`, `single workflow`, `separate workflow trigger`, `licences, C-crate bans, tokio`, `licences, C-dependency`, `-p viola-core -p viola-pty`, `check bans -c`, `tokio wrappers`, plus the claim reads `tokio.{0,40}(ban|wrappers)`, `(ban|deny).{0,60}tokio`, `all three targets`:
- Before apply, 17 amend-sites were found: arch :36, :44, :392, :418, :456, :463; security :134, :161, :308, :314, :327; test :486, :1399, :1411, :1434, :1617; obs :1217; a11y :1108. The remaining hits were left unchanged: arch :35; test :98, :164, :363, :396; obs :406, :777, :1239, :1378, :1443; a11y :175, :280, :499, :624, :793, :807, :813, :1063, :1153. They are still true or unrelated "all three OSes" wording.
- After apply: 0 hits in the masters and 0 in the preserve-verbatim homes, playbook and drift-base. The control fired on leaves commands.md:47, stack.md:32 and services/viola-mcp.md:17, which are re-derived in cascade step 3.
