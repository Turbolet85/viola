
## 2026-09-24-quality-gates — fuzz workspace, coverage and fuzz tooling, MSRV job, two-leg mutation, download-artifact
**Section:** Stack and Technologies (Language / runtime · CI/CD · Code quality rows) · Established Decisions [Module Boundaries] · Occupied Resources (Workspace crates · Repository) · Infrastructure Patterns (Build system Dependency policy · Project directory structure · CI/CD approach: workflows, Setup steps, Jobs wired today)
**Change:**
- `fuzz/` (`viola-fuzz`) is a separate cargo workspace excluded from the root (`exclude = ["fuzz"]`), registered with its toolchain file, targets, corpus and gitignored outputs. It is not a member.
- `artifacts/` also holds `llvm-cov-summary.json` and `mutants-verdict-<leg>.json`; `target/lcov.info` is registered.
- The runtime row records the CI `msrv` check (rustup 1.96 + `RUSTUP_TOOLCHAIN`) and the fuzz-only `nightly-2026-09-20`.
- The Code quality row adds cargo-llvm-cov 0.9.1 and cargo-fuzz 0.13.2 plus the fuzz lockfile exemption.
- The CI/CD row and Setup steps add actions/download-artifact 8.0.1 (5 pinned actions) and the new installs.
- The Dependency policy bullet records `fuzz/Cargo.lock` outside `cargo deny`: the operator-ratified test-only exemption, with its audit owed to "Workspace tree and code-graph planes".
- CI/CD: `nightly.yml` runs advisories + fuzz, with no `concurrency:` block (declined, why). The 7 `ci.yml` jobs are `test` (coverage + gate), the two-leg `mutants` matrix, `mutants-verdict` (union), `msrv`, `fuzz-replay`, `lint` and `supply-chain`.
- The tree shows `fuzz/`, the root `exclude` and the workflow comments.
**Why:** chunk 2026-09-24-quality-gates (report Changes: Crates/modules, Dependencies, Schema/config, Harness/gate surface, Counts). Operator rulings at wrap P2: the fuzz-lock exemption plus an audit CARRY.
**Sweep:** `dtolnay` 0 · `runs only`/`only for advisories` 0 · "four" actions 0 in masters. The `nightly.yml` hits in masters that stay true are the Threat Model CI/CD line (amended in security-plan), test-plan `:486`/`:1429` and a11y `:1108`. Leaves re-derived: `.claude/docs/stack.md` (Language / runtime and CI/CD rows), `.claude/docs/commands.md`, `.claude/docs/workflow.md`. CLAUDE.md `GENERATED:setup:*` recomputed with no change (the stack line already says "MSRV floor 1.96", and no block names CI jobs, actions or fuzz).
