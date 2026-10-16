
## 2026-10-16-remove-the-browser-leg — the browser a11y leg leaves; cli / tui cases gated in the coverage suite
**Section:** §1 → CI integration (Runner, Where results land, Gate); §3 → CI integration (Pipeline integration, Command, Artifact); §3 → Bootstrap phases (all seven, labels kept); §9 (the stand-until paragraph, Per-pipeline-stage artifacts, Pipeline integration, Artifact retention); §10 (Performance budget, Cross-input parallel, Build / deploy failure conditions); §11 → Strategy and CI
**Change:**
- Was a browser a11y leg in the tests' Playwright suite: `run --browser`, `npm ci` and Chromium on every CI leg with the verdict on ubuntu, `gate --require playwright`, axe / html-validate / jsx-a11y lint, `a11y-violations` NDJSON rows with `process:"ui"`, a `jq` SC-coverage aggregation and the `e2e-web/test-results/` upload. Now 0.1.0 runs no browser a11y leg: it renders no page, and the frontend's a11y tooling is a 0.2.0 input, its technology not chosen.
- The cli output-discipline and TUI boundary cases run as nextest cases in the per-OS `test` job's `coverage` suite on all three legs, each leg's `gate --require coverage,doctest` its own verdict; results land in `suites[].failures[]`; no a11y artifact is uploaded; no stage, budget or second driver of its own.
- §9's table keeps the Unit / integration row alone. §10's failure condition is a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`); the zero-flakiness parallel bans `#[ignore]`.
- Every Bootstrap phase reads none in 0.1.0; violation-json-emission-wire: cli / tui emit no a11y violation rows, and obs's `process` enum is `run · hook · mcp · cli`.
- §11 drops the Playwright second-stack ban, the runtime-axe ban, the `webServer` / `npm test:a11y` ban and the `browser-missing` ban; it bans a `suite` value or a separate a11y driver outside `scripts/agent-run.* run`.
**Why:** This chunk removed the browser test leg and the obs `ui` process (founder ruling 2026-10-16, item 3). No 0.2.0 ban is reserved (the plan's Constraints).
**Kept:** The tier `Standard (1)`, the founder's to re-decide; the manual NVDA / VoiceOver / Orca pass, founder-owned.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/
