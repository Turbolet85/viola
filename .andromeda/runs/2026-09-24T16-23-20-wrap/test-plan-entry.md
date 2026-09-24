
## 2026-09-24-workspace-tree-and-code-graph-planes — Lint row disposed, release-check job, fuzz lock audit, jq consumers
**Section:** §1 (dependency and boundary policy entity; the traced-to note) · §2 (V9 row) · §3 Bootstrap (`ci-tool-install` jq bullet, `quality-gate-config-emit`) · §9 Pipeline structure (Lint, Supply-chain, Release build rows; tool-install paragraph) · §9 Build failure conditions · §10 Build failure conditions · §12 (new entry; the Initial entry's "No separate test crate" request marked retired)
**Change:**
- Lint row: the orphans gate `scripts/orphans-check.sh` (+ `--probe`) runs per lib/bin target.
- `--acyclic` is an on-demand review with its measured reason; the rmcp `cargo tree` assertion joins with the "MCP server for drivers" chunk.
- Supply-chain row: the `fuzz/Cargo.lock` advisories + sources step (from the repo root, into `deny-fuzz.json`); weekly advisories cover both lockfiles; the artifact is admissible by content.
- Release build row: the `release` job runs `scripts/release-check.sh --probe` then the default mode (`--locked --bin viola`, judged on its own artifact records); a plain exit-code job.
- Build failure conditions: the cycle and rmcp-release-graph conditions are retired; the per-target orphan, fuzz lockfile and release-build conditions are added.
- §3: the gate-as-last-step rule is scoped to suite jobs (`lint`, `supply-chain`, `release` are plain exit-code jobs); the gate list names orphans-check, the fuzz lock audit and release-check; the jq bullet lists the two new consumers with the runner-image versions read.
- §1 and §2 updated to match.
- §12 gains this chunk's entry, and the Initial entry's "needs updating" request is marked retired.
**Why:** chunk 2026-09-24-workspace-tree-and-code-graph-planes (report Changes: Symbols/APIs, Harness/gate surface, Dev-tool versions; Spec claims disproved 1-2). Measured at cargo-modules 0.27.0: `--acyclic` exits 1 on 3 of 4 targets, each a type ↔ its own inherent method, whatever the filters. `cargo tree … | grep -c rmcp` = 0 at HEAD. Operator P4 decisions 1 and 3.
**Sweep** (same pass `sweep.py`): test-plan hits after the apply:
- `acyclic` 4 hits, all this pass's text (:98, :1428, :1815, :1820);
- `cargo tree -e features` 3 hits, all new (:1428, :1816, :1821);
- `last step of every job` 1 hit, amended (:799, now scoped);
- `No separate test crate` 2 hits, amended (:164 note re-derived; :1655 marked retired);
- the jq `G2 uses the runner-provided` hit at :786 amended in place.

0 stale. Fanned 13 proposals (8 `dependent-of`), all applied, re-derived from the report. Leaves: `.claude/docs/commands.md` Lint/release lines (shared with the arch cascade); `.claude/rules/testing.md` and `verification-harness.md` checked, 0 hits on the swept claims (unchanged).
