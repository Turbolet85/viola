# Fan-out results — 2026-09-24-epoch-1-cleanup

Seven doc-agents, one batch, report `viola-0.1.0/chunks/2026-09-24-epoch-1-cleanup/report.md`. Every return was clean YAML with no
preamble, and 0 entities after decode.

| doc | verdict | detectors |
|---|---|---|
| architecture | `proposals: []` | D-arch-resources, D-arch-decisions: no drift. Out-of-scope note: `architecture.md:515` (§Infrastructure Patterns → CI/CD approach) restates the union as the only mutation red path; handled by the cascade step-2 sweep |
| security-plan | `proposals: []` | D-security-input, D-security-auth, D-security-deps: no drift |
| design-system | `proposals: []` | D-design-tokens: no drift |
| layout-templates | `proposals: []` | D-layout-surface: no drift |
| test-plan | **12 proposals** (raw twin `.raw-fanout-test-plan.md`) | D-tests-obs-harness ×12: :557, :555, :563, :1496, :655, :437, :796, :1434, :1468, :1529, :1609 + a §12 entry. D-tests-coverage, D-tests-framework: no drift |
| obs-plan | `proposals: []` | D-obs-instrumentation, D-obs-stack, D-obs-pii: no drift. Out-of-scope note: `obs-plan.md:1253` (§9 Pipeline integration, Mutation row) says "a mutant is red only when no leg caught it"; handled by the cascade step-2 sweep |
| a11y-plan | `proposals: []` | D-a11y-surface, D-a11y-obs-schema: no drift |
