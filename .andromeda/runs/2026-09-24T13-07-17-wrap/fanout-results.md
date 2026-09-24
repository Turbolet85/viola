# Fan-out results — 2026-09-24-observability-gates

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 7 (D-arch-resources ×3, D-arch-decisions ×4; 3 carry `dependent-of`) | `.raw-fanout-architecture.md` |
| security-plan | drift | 2 (D-security-auth primary + 1 dependent). D-security-input and D-security-deps: no drift | `.raw-fanout-security-plan.md` |
| design-system | clean | `proposals: []`. D-design-tokens: every new surface's tokens flag is n/a | — |
| layout-templates | clean | `proposals: []`. D-layout-surface: no user-facing surface; `viola-harness` is test tooling outside the cli surface | — |
| test-plan | drift | 7 (D-tests-obs-harness ×4, D-tests-framework ×3; 2 dependents) | `.raw-fanout-test-plan.md` |
| obs-plan | drift | 7 (D-obs-stack primary + 6 dependents). D-obs-instrumentation and D-obs-pii: no drift | `.raw-fanout-obs-plan.md` |
| a11y-plan | clean | `proposals: []`. D-a11y-surface: no interactive element. D-a11y-obs-schema: schemas unchanged | — |

Entity-decode probe: the returns carried no `&lt;` / `&gt;` / `&amp;` (entities=0). Stripping removed only the harness's 2-space frame indent.
