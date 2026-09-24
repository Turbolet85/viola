# Fan-out results — 2026-09-24-supply-chain-and-workflow-gates

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | drift | 9 (D-arch-resources ×2, D-arch-decisions ×7) | `.raw-fanout-architecture.md` |
| security-plan | clean | 0 — detectors hold; flagged out-of-detector sites :134, :161, :308-314, :314, :327 for the orchestrator | none (clean return) |
| design-system | clean | 0 — every new surface `tokens n/a` | none |
| layout-templates | clean | 0 — no UI surface | none |
| test-plan | drift | 5 (D-tests-framework ×5) | `.raw-fanout-test-plan.md` |
| obs-plan | clean | 0 — bans match obs-plan; flagged :1217 (plan-carried) for the orchestrator | none |
| a11y-plan | clean | 0 — no interactive element; flagged :1108 for the orchestrator | none |

All returns began at `proposals:` (no preamble, no trailing commentary beyond `#` notes on clean returns); entity probe: no `&lt;`/`&gt;`/`&amp;` in any return.
