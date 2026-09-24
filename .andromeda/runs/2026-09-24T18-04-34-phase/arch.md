# arch extract

## Relevance
Partial. This is a refactor that adds no new surface. Arch applies through workspace placement, the lint and boundary gates, the rule against registering new resources, and the dependency direction for the `viola-core` pin test.

## Constraints
- **Where the code lives.** The `run.rs` split stays inside the test-only `viola-e2e` crate, under `harness/`. No product crate may depend on `viola-e2e`, and the split needs no new workspace crate (per architecture §Established Decisions [Module Boundaries]; §Occupied Resources → Workspace crates). The fake agent stays a root-package `[[bin]]` behind feature `fake-agent` (per §Occupied Resources → Binary, subcommands and exit codes).
- **MAX_FRAME pin test.** It must stay a `viola-core` test that adds no dependency on another viola crate. `viola-core` depends on no viola crate (per §Infrastructure Patterns → Crate dependency direction), and `v`/frame constants live there (per §Infrastructure Patterns → Project directory structure, `viola-core/` comment).
- **MAX_FRAME consumer.** Arch requires `config.json` to be "read with a `MAX_FRAME` cap" (per §Occupied Resources → Filesystem, `config.json` row). Research must answer whether `src/obs.rs:195` is that config read. That answer decides whether a consumer-side boundary witness proves this arch row or only the constant.
- **Lint regime.** It applies to every new helper and submodule: `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` and `cargo fmt --all --check` (per §Infrastructure Patterns → Build system, Lint).
  - Product members inherit `[lints] workspace = true`, which bans `print_stdout`, `print_stderr` and `dbg_macro`. This covers root `src/` test modules and `tests/`.
  - `viola-e2e` opts out and denies only `dbg_macro`. The fake agent carries a crate-level print allow.
  - `clippy.toml` bans the tracing level macros.
- **Orphans gate.** Every new module file from the split, and any shared test-helper module, must be reachable. `cargo modules orphans --deny` runs per lib/bin target, including `viola-fake-agent` with its required feature (per §Infrastructure Patterns → Build system, Boundary review).
- **No new registered resources.** The chunk adds no env var, repository path, crate or third-party dependency. Any dependency goes through `[workspace.dependencies]` (per §Infrastructure Patterns → Build system; §Occupied Resources → Environment variables / Repository). Harness-only variables, if any ever appear, use the `AGENT_RUN_` prefix (per §Conventions → Naming patterns, Environment variables).
- **Byte-identical harness JSON documents.** Their key order depends on serde_json `preserve_order`. Extracted helpers must build each document in the same key-insertion order (per §Stack and Technologies, Serialization row). The harness command contract itself belongs to test-plan §3 (per §Occupied Resources → Binary).

## Patterns to follow
- Put shared root-test helpers, such as those behind the `tests/cli_fake_agent.rs` clone pairs, in the existing `tests/support/` fixture chain (per §Infrastructure Patterns → Project directory structure, `tests/support/` comment).
- Name new modules and files in snake_case, with the standard Rust item casing (per §Conventions → Naming patterns, Crates).
- `scripts/agent-run.{sh,ps1}` stay identical shims over `viola-harness`. The split must not change their invocation (per §Infrastructure Patterns → Project directory structure).
- Every OS-specific branch compiles and is tested on its CI runner. `cfg`-gated bodies moved by the split are judged by the two mutation legs, ubuntu and windows (per §Cross-cutting Patterns → Cross-platform discipline; §Infrastructure Patterns → CI/CD approach, `mutants` / `mutants-verdict`).

## Anti-patterns to avoid
- Adding print-ban opt-outs (`#[allow(clippy::print_stdout)]` and similar) or raw `tracing::` / `event!` calls to product-member test code to make extracted helpers compile (per §Infrastructure Patterns → Build system, Lint).
- Leaving a split-off `harness/*.rs` file undeclared (an orphan), or creating a new crate or product-crate dependency on `viola-e2e` to host the extracted code (per §Infrastructure Patterns → Build system, Boundary review; §Established Decisions [Module Boundaries]).
- Having a helper shell out through `sh`, `bash` or `cmd` instead of spawning directly (per §Cross-cutting Patterns → Cross-platform discipline).

## Contract bindings
- **arch ↔ tests:**
  - The `viola-harness` command contract is test-plan §3 (per §Occupied Resources → Binary).
  - `tests/contract_lints.rs` asserts `[lints] workspace = true` (per §Infrastructure Patterns → Build system, Lint).
  - `tests/contract_diag_schema.rs` pins the diag schemas. The operator-kept `LOG_FORMAT_EVENTS` clone sits here (per §Occupied Resources → Repository, `schemas/diag-*` row).
- **arch ↔ security:** the `MAX_FRAME`-capped `config.json` read (per §Occupied Resources → Filesystem) is the arch side of security-plan's `Read::take(MAX_FRAME)` invariant.
- **arch ↔ obs:** diagnostics line and detail formats are obs-owned (per §Cross-cutting Patterns → Diagnostic output channels). The `main.rs` and `obs.rs` test-module clones (panic-line asserts, diag-detail validator loading) exercise those schemas.

## Acceptance criteria contributions
- `scripts/orphans-check.sh --probe && scripts/orphans-check.sh` passes for every lib/bin target after the split, including `viola-e2e`, `viola-harness` and `viola-fake-agent` (per architecture §Infrastructure Patterns → Build system, Boundary review).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` and `cargo fmt --all --check` pass cold. `tests/contract_lints.rs` passes. The diff adds no new lint `allow` in product members (per §Infrastructure Patterns → Build system, Lint).
- The diff adds no new crate, workspace member, `[workspace.dependencies]` entry, env var or registered path. `crates/viola-core/Cargo.toml` gains no viola-crate dependency (per §Occupied Resources; §Infrastructure Patterns → Crate dependency direction).
- `scripts/release-check.sh` still yields `viola` only for `--bin viola` (per §Infrastructure Patterns → CI/CD approach, target job 6).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton.** Registered `viola-e2e` (harness library + `viola-harness`, no-op `fake-agent` feature), the `viola-fake-agent` root `[[bin]]` and the `AGENT_RUN_` prefix. These are the surfaces this chunk refactors. It also added `preserve_order`, which keeps printed documents in their declared key order and so underpins the byte-identical harness output.
- **2026-09-24-fake-agent-and-test-data-fixtures.** Added `tests/support/` as the root test chain, the home for extracted `tests/` helpers. It also corrected the claims about which processes read the `AGENT_RUN_KEEP_*` variables. Helper extraction must not change who reads them.
- **2026-09-24-diagnostics-plane.** Registered the `MAX_FRAME`-capped `config.json` read (`v` = 1, `parse-rejected`). That is the arch mandate the MAX_FRAME pin and any consumer witness speak to.
- **2026-09-24-observability-gates.** Landed the workspace print/dbg bans, the `viola-e2e` opt-out, the fake agent's print allow, `clippy.toml` `disallowed-macros` and `tests/contract_lints.rs`. This is the lint regime the extracted helpers must satisfy.
- **2026-09-24-quality-gates.** Introduced the two-leg `mutants` matrix and the `--in-diff` verdict union. These produce the cost and 0-missed obligation once `run.rs`'s mutants enter the diff.
- **2026-09-24-workspace-tree-and-code-graph-planes.** Wired the per-lib/bin orphans gate, which now governs the new submodules. This is also the chunk whose deferred Rust gates are closed here.
