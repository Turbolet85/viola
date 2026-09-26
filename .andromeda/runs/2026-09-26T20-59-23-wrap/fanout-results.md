# Fan-out results — 2026-09-26-ci-chunk-base-and-union-verdict

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 8 (D-arch-resources ×6, D-arch-decisions ×2; 4 `dependent-of`) | `.raw-fanout-architecture.md` |
| security-plan | drift | 2 (D-security-auth ×2; 1 `dependent-of`); D-security-deps and D-security-input no drift; flagged security-plan.md:340 `AGENT_RUN_CHUNK_BASE` env example (no detector) | `.raw-fanout-security-plan.md` |
| design-system | no drift (`proposals: []`) — D-design-tokens: every new surface `tokens n/a`, no UI | — |
| layout-templates | no drift (`proposals: []`) — D-layout-surface: no user-facing surface; the `no leg` hit at :288 is the footer's "no links" | — |
| test-plan | drift | 10 (D-tests-obs-harness ×10; 9 `dependent-of`); D-tests-coverage and D-tests-framework no drift | `.raw-fanout-test-plan.md` |
| obs-plan | drift | 3 (D-obs-pii ×3, severity escalate; 2 `dependent-of`); D-obs-instrumentation and D-obs-stack no drift; flagged §9 Mutation row (obs-plan.md:1253) as expected but outside its detectors | `.raw-fanout-obs-plan.md` |
| a11y-plan | no drift (`proposals: []`) — D-a11y-surface / D-a11y-obs-schema hold; the one `union` hit (:1145) is the axe per-state union | — |

Entity probe: all seven returns decoded with 0 `&lt;`/`&gt;`/`&amp;` escapes.
