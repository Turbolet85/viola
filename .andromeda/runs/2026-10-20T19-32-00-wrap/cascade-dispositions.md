# Cascade dispositions — 2026-10-20-epoch-5-cleanup-ii

## Step 2 — the sweep
Search: `cascade-patterns.toml` (17 patterns: `typed-file`, `typed-path`, `board-file`, `relied-here`, `mcp-serde`,
`shared-pins`, `record-only`, `count-302`, `probe-line`, `prepare-win`, `handshake`, `walk`, `e2e-bin`, `schema-enum`,
`class-enum`, `mcp-suite`, `env-rule`) — each the old wording or the retired mechanism of an amendment of this pass:
the single-file `typed.rs` / `board.rs` and their flat paths, `relied_here`, the `viola-mcp` → `serde` edge and the
shared-pins claim, `fixtures/claude` recorded only by `verify --record`, the 2.1.302 count, the orphans probe line,
`prepare` Windows-only, the MCP opening, the hygiene walk and the hook-payload schema enumeration, viola-e2e's one bin,
the schema / class enumerations the `Mcp.` class joins, the MCP stdio suite named alone, the env-var rule. Baseline
844de976 (the parent of the oldest pre-CI commit). Every control fired.

Listing (first full call): `total (17 patterns) · 71 rows over 16 files` · `per class · new 2/2 · standing 39/7 ·
leaf 30/9 · curation 0/0 · base 0/0`. Zero-row patterns (control fired): `board-file`, `shared-pins`, `count-302`,
`prepare-win` — each a statement about its pattern. One fold followed the listing (`build-system.md:10`, below); the
`probe-line` pattern re-run after it: `total (1 pattern) · 1 rows over 1 files` · `per class · new 0/0 · standing 0/0 ·
leaf 1/1`.

Master rows:
- typed-file · `architecture.md:101` edited — this pass's text (architecture 14/16) · no change
- typed-file · `test-plan.md:1096` edited — this pass's text (test-plan 21) · no change
- typed-file · `project-directory-structure.md:33` edited — this pass's row (architecture 14) · no change
- mcp-serde · `crate-dependency-direction.md:7` edited, `:8` new — this pass's text (architecture 11, 12) · no change
- record-only · `architecture.md:375` @c58 "hook-payload fixtures recorded by `viola verify --record`" — true of the hook
  payloads; the bullet now states the `Mcp.` class apart · no change; @c4160 this pass's text
- record-only · `security-plan.md:258` @c60 — this pass's text (security-plan 1) · no change
- record-only · `test-plan.md:349`, `:379` @c27 — the fake agent vs the recorded `verify` fixtures: true · no change
- record-only · `test-plan.md:936` — the hook-payload row: true; the `Mcp.` exchange has its own row (`:937`) · no change
- record-only · `project-directory-structure.md:113` edited — this pass's row · no change
- probe-line · `build-system.md:10` "proves it fires on a planted orphan" — the probe plants two (the report's disproved
  claim 2) → amended in this pass (folded: "on each of its two planted orphans"), its entry in the architecture sidecar
- handshake · `architecture.md:152` ×4 — @c1529 "2.1.287 negotiates protocol `2026-07-28`" true; the rest this pass's
  text (architecture 9) · no change
- walk · `architecture.md:379` "every other hook payload against `claude-fixture.v1.json`" — true (an `Mcp.` file is no
  hook payload) · no change
- walk · `test-plan.md:379` new, `:967` edited — this pass's text · no change
- e2e-bin · `architecture.md:315` edited — this pass's text · no change; `project-directory-structure.md:85` — the
  viola-e2e row, extended by this pass on the lines below it · no change
- schema-enum · `architecture.md:375` @c2213, `:380`; `test-plan.md:936` — the agents class's own sentences, true; the
  `Mcp.` class stated beside them · no change; `test-plan.md:379`, `:971`, `project-directory-structure.md:55` edited —
  this pass's text · no change
- class-enum · `architecture.md:101` @c14920, `:375` ×2, `security-plan.md:115` @c1398, `test-plan.md:936`, `:963`
  @c5045, `crate-dependency-direction.md:6` @c2215 — Run B's agents read and its fixture: true · no change;
  `test-plan.md:379` ×2, `:971`, `project-directory-structure.md:113` edited · no change
- mcp-suite · `test-plan.md:624` edited — this pass's text · no change; `test-plan.md:657`, `:678`, `:698`, `:725`,
  `:772`, `bootstrap-phases-derive-for-route-setup-project.md:10` @c1689 — the `mcp_tools` legs: true · no change
- env-rule · `architecture.md:422` "the only other variables any `viola` build reads" — true; the new test-only marker
  is read by no `viola` build · no change

Leaf rows → step 3's set: `.claude/docs/services/viola.md` (:6 ×2, :47), `.claude/docs/tests-summary.md` (:21, :26–30,
:42), `.claude/docs/services/viola-agent-claude.md` (:26 ×2, :39), `.claude/docs/services/viola-mcp.md` (:27, :32,
:36), `CLAUDE.md` (:15), `.claude/rules/verification-harness.md` (:41, :45), `.claude/rules/security.md` (:17, :37),
`.claude/docs/commands.md` (:23, :53), `.claude/docs/security-summary.md` (:72).
Curation homes and judgment bases: 0 rows.

## Leaves
Set: `cascade.py leaves` pass mode (Epoch 6 does not close: `route.py epoch` reads 7 markerless under its header),
`--add` the step-2 leaf rows; `total · pass · 10 group(s) · 25 leaves · 331745 B · unmapped 2`. Unmapped:
`.claude/rules/host-linux.md` (the host leaf, no comparer's) · `.claude/rules/testing-src.md` (a pointer leaf to
`testing.md`, naming no master — no comparer). The leaf batch ran inside its own snapshot pair (before `tree 21c3dbcf…`,
after `tree 02d8559c…`, index and head unchanged): exactly the ten `A {run_dir}/leaf-{group}.md` lines. The apply's
`total` line: `total · blocks 25 · applied 20 · anchor absent 1 · held 4`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 4 | 3 (viola-agent-claude.md :26 retired, :39, :14) | 0 | 1 — conventions.md:13: the leaf already points to arch §Occupied Resources → Environment variables, where the marker stands; a detail stays in its master |
| architecture-2 | pass | 7 | 7 (viola.md :6 ×2, :47; viola-mcp.md :11, :27, :35, :36) | 0 | 0 |
| architecture-3 | pass | 1 | 1 (workflow.md:49) | 0 | 0 |
| architecture+security-plan | pass | 1 | 0 | 0 | 1 — api.md:35 `anchor absent`, and the 2.1.302 opening is rmcp's to handle: the leaf's rule (the two keys on the result) stands as the source says |
| architecture+security-plan+test-plan+a11y-plan | pass | 2 | 2 (CLAUDE.md :15, :28) | 0 | 0 |
| architecture+test-plan | pass | 2 | 0 | 2 — commands.md:53 the probe line `1/1` → `2/2` alone; commands.md:23 one shorter sentence naming the tap and pointing to arch §Occupied Resources → Binary | 0 |
| security-plan | pass | 2 | 2 (security.md:17, security-summary.md:72) | 0 | 0 |
| test-plan-1 | pass | 5 | 5 (tests-summary.md :21, :42 ×3; testing.md:24) | 0 | 0 |
| test-plan-2 | pass | 1 | 0 | 0 | 1 — verification-harness.md:45 states the fake agent's drift replay, which the `Mcp.` exchange is no part of (it is replayed against `viola mcp`): nothing in the line is false |
| a11y-plan | pass | 0 | 0 | 0 | 0 |
