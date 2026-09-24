# arch extract

## Relevance
Partial. obs-plan §9/§11 owns the gates themselves. Architecture owns the rules they sit inside: CI workflow discipline, the workspace/lint build system, which output channels exist per role, hook exit semantics, and where test homes and harness paths live.

## Constraints
- New `ci.yml` steps must follow the existing workflow rules: `permissions: {}` at the top with `contents: read` per job; every `uses:` pinned by full commit SHA with a version comment; event-payload values passed only through `env:`; every gate step `shell: bash` and fail-closed. zizmor checks all of this. The upload-artifact version is v7.0.1 (per architecture §Infrastructure Patterns → CI/CD approach; §Stack and Technologies CI/CD row).
- The lint gate stays `cargo clippy --workspace --all-targets -- -D warnings` on all three OSes (target job 2). Any new `clippy.toml` and `[workspace.lints.clippy]` must pass under that exact command on windows-2025, macos-latest and ubuntu-latest (per §Infrastructure Patterns → Build system, → CI/CD approach target jobs 1–6).
- Which roles may print is fixed per role, so the list of `#[allow]` sites must match it:
  - `hook`: its decision body on stdout only, never stderr.
  - `run`: nothing on the terminal while the child runs.
  - `mcp`: only MCP frames on stdout, and stderr JSON only when `diagnostics/mcp.ndjson` cannot be opened.
  - `ui` and short-lived CLI verbs: may use stderr for human output.

  An `#[allow]` that lets `hook` or `run` print outside these is a violation (per §Cross-cutting Patterns → Diagnostic output channels; §Established Decisions [Hook Contract]).
- `viola hook` always exits 0, including on panics caught at the edge. Exit 2 is forbidden. The panic-hook-first gate and the G3 `panic = "abort"` ban both protect this. Abort would stop a panic from being caught at the edge (per §Conventions → `viola hook` exit codes; §Established Decisions [Hook Contract]; §Cross-cutting Patterns → Fail open toward the human).
- anyhow stays in the root bin only: `main`, `cmd::dispatch` and `viola::obs::report_internal_error`. A context chain goes only to `instances/<name>/diagnostics/detail-<role>.ndjson`, never to a role line, stdout or stderr. The mutants fix touches `main`/`dispatch` and must keep this (per §Established Decisions [Error Handling]; §Conventions → Rust error types).
- Workspace membership and package placement: the members are `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui` and the test-only `viola-e2e`. Each product crate is created by its first consumer, so "every product member" means the members present at HEAD. `viola-fake-agent` is a `[[bin]]` of the root package behind feature `fake-agent`. Whether it inherits the root package's lints at HEAD is research's question (per §Established Decisions [Module Boundaries]; §Occupied Resources → Binary, → Workspace crates).
- The plan names `target/e2e-home/viola-session-*/home` (harness) and `target/e2e-home/viola-test-*/home` (root rstest, `tests/support/home.rs`) as every harness and test home, kept in CI for the obs gates. Harness-only variables use the `AGENT_RUN_` prefix and are never read by `viola`. Whether every integration test's home actually lands under `target/e2e-home/` at HEAD is research's question (per §Occupied Resources → Repository, → Environment variables; §Conventions → Naming, environment variables).

## Patterns to follow
- The `supply-chain` job pattern: gate output goes as JSON under a registered `target/<dir>/` and is uploaded with `if: always()` under a fixed artifact name for 7 days. The `secret-scan-<os>` hit report and the scan-gated uploads should follow the same shape (per §Infrastructure Patterns → CI/CD approach, jobs wired today).
- Gates proven both ways: `scripts/deny-probes.sh` runs one throwaway negative probe per ban plus a clean control that must pass. The `obs_event!`-clean / raw-`tracing::info!`-fails and `println!`-fails / `#[allow]`-passes proofs follow the same pattern (per §Infrastructure Patterns → Build system).
- Fail-closed list assertions: `scripts/sync-crates.txt` job 3 fails on an empty list or an absent package. The member-list `workspace = true` assertion and G2's non-empty scope check should fail closed the same way (per §Infrastructure Patterns → CI/CD approach target job 3).
- One place for workspace-wide policy: `[workspace.dependencies]` pins every version in one place. `[workspace.lints]` with `[lints] workspace = true` per member is the analogue (per §Infrastructure Patterns → Build system).
- `scripts/agent-run.{sh,ps1}` are identical shims over `viola-harness`. Any change to the local gate set (the G3 `rg`/grep equivalent) goes into the harness or lands identically in both shims (per §Infrastructure Patterns → Project directory structure).

## Anti-patterns to avoid
- Mutable action refs, event-payload values inlined into `run:`, or a gate step that is not fail-closed (per §Infrastructure Patterns → CI/CD approach).
- Content-bearing detail or anyhow chains on stdout, stderr or home-level role lines, and any stderr write or exit 2 from `hook` (per §Cross-cutting Patterns → Diagnostic output channels; §Established Decisions [Error Handling], [Hook Contract]).
- A new path or env var used without registration, or a harness variable read by the `viola` binary (per §Occupied Resources → Environment variables, → Repository).

## Contract bindings
- arch §Cross-cutting Patterns → Diagnostic output channels ↔ obs §11 Logs (print bans, output-module `#[allow]` sites).
- arch §Occupied Resources → Repository (`target/e2e-home/*`, `target/agent-run/*`) and `AGENT_RUN_KEEP_HOMES` ↔ obs §9 G2/G4/secret-scan scope ↔ test-plan §3 (cleanup step 6, homes kept).
- arch §Occupied Resources → Repository (`schemas/diag-line.v1.json`, `diag-detail.v1.json`, pinned by `tests/contract_diag_schema.rs`) ↔ obs §9 G4 `schema-conformance`.
- arch §Infrastructure Patterns → CI/CD approach (SHA pins, least privilege, zizmor) ↔ security-plan workflow rules for the new steps. The secret scan's subject is the security-plan NEVER-log floor.
- arch §Established Decisions [Hook Contract] / §Conventions exit codes ↔ obs panic hook, G2 zero-panic and G3 abort ban.
- Placement check: the disallowed-macros exemption names `viola_core::obs`. Arch §Infrastructure Patterns → Crate dependency direction lists no tracing dependency for `viola-core`, only nutype plus the shared serde/serde_json/chrono/thiserror. Research should confirm where `obs_event!` lives at HEAD. If `viola-core` depends on tracing, that is a wrap amendment to arch.

## Acceptance criteria contributions
- Every `uses:` added to `ci.yml` is pinned by full SHA with a version comment. Workflow and job permissions are unchanged. Each new gate step is fail-closed `shell: bash`. zizmor over `.github/workflows/` stays clean (per architecture §Infrastructure Patterns → CI/CD approach).
- `cargo clippy --workspace --all-targets -- -D warnings` passes on all three CI OSes with the new workspace lints and `clippy.toml` in force (per architecture §Infrastructure Patterns → Build system).
- No `#[allow(clippy::print_stdout|print_stderr)]` covers a `hook` stderr path or a `run` terminal write made while the child runs. Allowed sites are limited to the channels the plan grants (per architecture §Cross-cutting Patterns → Diagnostic output channels).
- Any new repo path this chunk lands (for example the secret-scan report dir) or any new harness env var is registered at wrap. A new env var carries the `AGENT_RUN_` prefix and is not read by `viola` (per architecture §Occupied Resources; §Conventions → Naming).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, CI setup and wired jobs:** set up rustup from `rust-toolchain.toml`, SHA-pinned checkout / rust-cache / install-action / upload-artifact v7.0.1, `permissions: {}` with `contents: read`, and the `test` and `mutants` jobs (base via `env:`). Why: the chunk shipped `ci.yml`. It is the baseline this chunk extends.
- **Same chunk, test-only crate, bins, env vars, paths:** registered `viola-e2e`, the root `[[bin]]` `viola-fake-agent` (feature `fake-agent`), `AGENT_RUN_KEEP_HOMES`, `target/agent-run/` and `target/e2e-home/`. Why: registry sync. It bears on the fake-agent lint exemption and the gate scope.
- **2026-09-24-fake-agent-and-test-data-fixtures:** e2e-home names both `viola-session-*/home` and `viola-test-*/home`. `AGENT_RUN_KEEP_HOMES` is read by `viola-harness` and the root test chain. `AGENT_RUN_KEEP_FAILED` was added. Why: a disproved claim that one prefix held every test home. Directly relevant to scope item 6 (test-home placement).
- **2026-09-24-supply-chain-and-workflow-gates:** `ci.yml` jobs became `test`, `mutants`, `lint`, `supply-chain`. zizmor 1.30.1 and the `if: always()` JSON-report upload pattern were added, and deny-probes became the both-ways proof pattern. Why: the chunk shipped them. It defines the job layout these gates join.
- **2026-09-24-diagnostics-plane:** home-level role files (`run-`/`hook-`/`mcp`/`ui-`/`cli-`), owner-only `detail-<role>.ndjson`, `schemas/diag-line|diag-detail.v1.json`, and the rule that role lines carry no `v`. Why: the chunk shipped them. These are the files G2/G4/secret-scan read.
- **2026-09-24-log-redaction-and-never-log-floor:** the anyhow scope now names `viola::obs::report_internal_error`, and chains go only to detail files. Why: the dispatch error is routed through `src/obs.rs`. It constrains the `main`/`dispatch` mutants fix.
