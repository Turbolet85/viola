
## 2026-09-24-workspace-tree-and-code-graph-planes — fuzz lockfile audit wired, release-check and orphans in the CI job list
**Section:** §Threat Model Summary (Supply chain trust boundary; Infrastructure CI/CD workflows and jobs) · §Dependency Security (the `fuzz/` bullet; CI integration Job 4 and the nightly workflow) · §Security Decisions Log 2026-09-24 (Conditions)
**Change:**
- The `fuzz/Cargo.lock` audit is no longer "owed". The ci.yml `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, and weekly `nightly.yml` `advisories` runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`.
- Both run from the repo root, where cargo-deny resolves the root `deny.toml`. The root run's families (the C-build ban included) still do not reach the fuzz graph.
- Job 4 states the root-lock scope and the same job's separate fuzz step.
- The Threat Model CI job list adds the cargo-modules orphans gate, the fuzz lockfile audit, and the per-OS release build through `scripts/release-check.sh` (refuses any test-only binary) in place of a bare `cargo build --release`.
- The Decisions Log Conditions record the CARRY as delivered.
**Why:** chunk 2026-09-24-workspace-tree-and-code-graph-planes (report Changes: Harness/gate surface, Schema/config; Expected amendments). No detector proposed these: the orchestrator raised them under Validate check 5, routine because the report substantiates each. Operator P4 decision 3 added the nightly advisories.
**Sweep** (same pass `sweep.py`): security-plan hits after the apply:
- `root Cargo.lock only` 2 hits, amended (:134 now "root graph covers the root lock only … gets its own audit"; :327 plus the fuzz step);
- `outside cargo deny` 2 hits, :315 amended; :646 no change (Decisions Log history, true);
- bare `cargo build --release` :161 amended;
- nightly advisories without fuzz, 2 hits, no change: :399 (the bootstrap phase that wired the weekly run, true) and :617 (manual review driven by the weekly run, true);
- `owed` (word-bounded) 0 after the apply.

Leaves re-derived: `.claude/rules/security.md` (weekly advisories over both lockfiles; the fuzz lockfile's own audit; a new release-build-carries-`viola`-only line). `.claude/docs/security-summary.md` was checked: 0 hits (unchanged).
