# Fan-out results — 2026-09-24-diagnostics-plane

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 9 (D-arch-resources ×6 incl. 2 dependents · D-arch-decisions ×3 incl. 1 dependent) | `.raw-fanout-architecture.md` |
| security-plan | clean | 0 (notes: §Input Validation config row + MAX_FRAME consumer list omit the new config read — optional completeness) | none (empty, YAML comments only) |
| design-system | clean | 0 | none |
| layout-templates | clean | 0 | none |
| test-plan | clean | 0 (§3 `logs` + log format already agree with obs §3 and the shipped wrapper) | none |
| obs-plan | drift | 4 (D-obs-pii ×2 incl. 1 dependent · D-obs-stack ×2 incl. 1 dependent) | `.raw-fanout-obs-plan.md` |
| a11y-plan | drift | 1 (D-a11y-obs-schema) | `.raw-fanout-a11y-plan.md` |

Entity probe: every return decoded clean (entities=0); the empty returns carried only YAML `#` comments (no stripping needed).
