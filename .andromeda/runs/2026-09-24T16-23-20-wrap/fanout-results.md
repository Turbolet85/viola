# Fan-out results — 2026-09-24-workspace-tree-and-code-graph-planes

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 13 (D-arch-resources 3, D-arch-decisions 10; 8 dependent-of) | `.raw-fanout-architecture.md` |
| security-plan | clean | 0. The agent noted the stale "owed" / "root lock only" sites :134 :315 :327 :328 :648 (+ :160, :399) as the report's expected-amendments channel | — (clean return) |
| design-system | clean | 0 (tokens n/a on every new surface) | — |
| layout-templates | clean | 0 (no new surface or region) | — |
| test-plan | drift | 13 (D-tests-framework; 8 dependent-of) | `.raw-fanout-test-plan.md` |
| obs-plan | drift | 1 (D-obs-pii, severity escalate): §8 item 6 unscanned-uploads inventory gains the supply-chain artifact | `.raw-fanout-obs-plan.md` |
| a11y-plan | clean | 0 (no interactive element, no schema change) | — |

Entity decode: every return was checked for `&lt;` / `&gt;` / `&amp;` (entities=0 across all seven; `<` and `>` arrived literal).
