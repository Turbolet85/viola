# Fan-out results — 2026-09-27-instance-state-and-start-order

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim (report
`viola-0.1.0/chunks/2026-09-27-instance-state-and-start-order/report.md`). Returns were clean YAML (no preamble to
strip; no HTML entities: the `entities=0` probe needed no decode — the returns carried literal `<` `>` `&` inside
backticks only).

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 19 (D-arch-decisions 7, D-arch-resources 12) | `.raw-fanout-architecture.md` |
| security-plan | drift | 5 (D-security-auth 1 + 4 dependents); D-security-input, D-security-deps: no drift | `.raw-fanout-security-plan.md` |
| design-system | no drift | 0 (D-design-tokens: every new surface `tokens n/a`) | — |
| layout-templates | no drift | 0 (D-layout-surface: the new refusals are causes on the existing `viola run` surface, layout-templates.md:532-533) | — |
| test-plan | drift | 20 (D-tests-obs-harness 12, D-tests-framework 4, D-tests-coverage 4) | `.raw-fanout-test-plan.md` |
| obs-plan | drift | 4 (D-obs-instrumentation 1 + 1 primary + 2 dependents); D-obs-stack, D-obs-pii: no drift | `.raw-fanout-obs-plan.md` |
| a11y-plan | no drift | 0 (D-a11y-surface, D-a11y-obs-schema: no interactive element; schema unchanged) | — |

Total: 48 proposals over 4 docs. Validation and dispositions: `validation.md`.
