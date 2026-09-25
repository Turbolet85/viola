# Fan-out results — 2026-09-25-security-prerequisites

Seven Explore doc-agents, one batch, prompt from amendment-flow.md verbatim. Returns were stripped of the harness
indentation. Entity probe: `&lt;`/`&gt;`/`&amp;` = 0 in every return, so no decode was needed.

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 10 (D-arch-decisions ×9, D-arch-resources ×1) | `.raw-fanout-architecture.md` |
| security-plan | drift | 7 (D-security-auth ×6, D-security-deps ×1, escalate) | `.raw-fanout-security-plan.md` |
| design-system | clean | 0 — D-design-tokens: every new surface `tokens n/a`, no UI rendered | — |
| layout-templates | clean | 0 — D-layout-surface: no user-facing surface or region; harness verdict JSON is internal tooling | — |
| test-plan | drift | 6 (D-tests-obs-harness ×5, D-tests-coverage ×2 → 7 lines, see twin) | `.raw-fanout-test-plan.md` |
| obs-plan | clean | 0 — instrumentation n/a (test-only harness), no telemetry dep, no PII; obs-plan names no mutants verdict value | — |
| a11y-plan | clean | 0 — no interactive UI element, no schema change | — |
