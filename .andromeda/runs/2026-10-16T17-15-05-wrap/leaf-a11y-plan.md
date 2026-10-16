group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:33   class: retired
  leaf-text: «1. **a11y-tooling-install** — `@axe-core/playwright@4.13.0` into `e2e-web/package.json` (the axe pin is this phase's), devDependencies (colorjs.io, tabbable, virtual-screen-reader, html-validate, eslint-plugin-jsx-a11y + eslint), `e2e-web/fixtures/a11y.ts` (`makeAxeBuilder`, scrubber, row writer), html-validate config declaring any `viola-*` elements the rendered page carries»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:7 «- **a11y-tooling-install:** none in 0.1.0. The cli / tui cases use the tests' own assert_cmd, predicates, trycmd and portable-pty pins; the browser a11y tooling left with the browser leg»
  replace-with: «1. **a11y-tooling-install** — none in 0.1.0: the cli / tui cases use the tests' own assert_cmd, predicates, trycmd and portable-pty pins; the browser a11y tooling left with the browser leg, and the frontend's is a 0.2.0 input, its technology not chosen»

- leaf: .claude/docs/a11y-summary.md:34   class: stale
  leaf-text: «2. **focus-management-library-install** — none at runtime; tabbable test-side only»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:8 «- **focus-management-library-install:** none.»
  replace-with: «2. **focus-management-library-install** — none»

- leaf: .claude/docs/a11y-summary.md:36   class: retired
  leaf-text: «4. **contrast-verification-harness-setup** — the `a11y-tokens` test in `bay-steady-state.spec.ts`»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:10 «- **contrast-verification-harness-setup:** none in 0.1.0: no page renders a token pair.»
  replace-with: «4. **contrast-verification-harness-setup** — none in 0.1.0: no page renders a token pair»

- leaf: .claude/docs/a11y-summary.md:37   class: retired
  leaf-text: «5. **screen-reader-test-spec-setup** — VSR route + import helper in `e2e-web/fixtures/a11y.ts` (MutationObserver fallback)»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:11 «- **screen-reader-test-spec-setup:** none in 0.1.0; the manual NVDA / VoiceOver / Orca pass stays founder-owned (Section 1 → Screen reader test pattern).»
  replace-with: «5. **screen-reader-test-spec-setup** — none in 0.1.0; the manual NVDA / VoiceOver / Orca pass stays founder-owned»

- leaf: .claude/docs/a11y-summary.md:38   class: retired
  leaf-text: «6. **a11y-ci-gate-wire** — `@a11y` + `@sc-*` tags, ubuntu lint step before `run --browser`, `sc-coverage.json` + post-gate aggregation (`if: always()`); the browser leg (lint, E2E, aggregation, `run --browser`, `e2e-web/`) stands until the route entry "Remove the browser leg" removes it»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:12 «- **a11y-ci-gate-wire:** none of its own: the cli / tui cases run inside the per-OS `test` job's `coverage` suite under `gate --require coverage,doctest`, and no a11y lint, aggregation or upload step exists.»
  replace-with: «6. **a11y-ci-gate-wire** — none of its own: the cli / tui cases run inside the per-OS `test` job's `coverage` suite under `gate --require coverage,doctest` (skips never allowed), and no a11y lint, aggregation or upload step exists»

- leaf: .claude/docs/a11y-summary.md:39   class: retired
  leaf-text: «7. **violation-json-emission-wire** — the row writer in the fixture»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:13 «- **violation-json-emission-wire:** none in 0.1.0 — cli / tui emit no a11y violation rows (Section 3 → Structured violation JSON schema: output-discipline failures surface only as nextest failures in»
  replace-with: «7. **violation-json-emission-wire** — none in 0.1.0: cli / tui emit no a11y violation rows (output-discipline failures surface only as nextest failures in `suites[].failures[]`)»

- leaf: .claude/docs/a11y-summary.md:45   class: retired
  leaf-text: «- NEVER add a second browser automation stack (Lighthouse, pa11y) beside Playwright.»
  source: .andromeda/a11y-plan.md:387 «- NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.»
  replace-with: «- NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.»

- leaf: .claude/rules/a11y.md:3   class: tree
  leaf-text: «  - "crates/viola-ui/assets/**"»
  source: .andromeda/registries/contracts/a11y-plan/ci-integration.md:27 «  - The CLI output-discipline and TUI boundary cases are ordinary nextest cases, so they run in the existing suites (CI: the per-OS `test` job's `coverage` suite).»
  replace-with: «  - "tests/cli_*.rs"»

- leaf: .claude/rules/a11y.md:4   class: tree
  leaf-text: «  - "e2e-web/fixtures/a11y.ts"»
  source: .andromeda/registries/contracts/a11y-plan/ci-integration.md:27 «  - The CLI output-discipline and TUI boundary cases are ordinary nextest cases, so they run in the existing suites (CI: the per-OS `test` job's `coverage` suite).»
  replace-with: «  - "tests/tui_*.rs"»

- leaf: .claude/rules/a11y.md:5   class: tree
  leaf-text: «  - "e2e-web/a11y/**"»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:12 «no a11y lint, aggregation or upload step exists.»
  replace-with: «»

- leaf: .claude/rules/a11y.md:6   class: tree
  leaf-text: «  - "e2e-web/.htmlvalidate.json"»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:7 «the browser a11y tooling left with the browser leg, and the frontend's is a 0.2.0 input, its technology not chosen.»
  replace-with: «»

- leaf: .claude/rules/a11y.md:7   class: tree
  leaf-text: «  - "e2e-web/eslint.config.js"»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:7 «the browser a11y tooling left with the browser leg, and the frontend's is a 0.2.0 input, its technology not chosen.»
  replace-with: «»

- leaf: .claude/rules/a11y.md:24   class: retired
  leaf-text: «- axe comes through `makeAxeBuilder` in `e2e-web/fixtures/a11y.ts` (tags `wcag2a, wcag2aa, wcag21a, wcag21aa, wcag22aa` + the five best-practice rules by id); the browser leg it serves stands until the route entry "Remove the browser leg" removes it.»
  source: .andromeda/registries/contracts/a11y-plan/ci-integration.md:30 «  - `scripts/agent-run.sh run` (POSIX) or `scripts/agent-run.ps1 run`, followed by `gate --require coverage,doctest` (skips never allowed), on every leg of `ci.yml`'s `test` job.»
  replace-with: «- The a11y harness is the tests' own assert_cmd / predicates / trycmd / portable-pty cases (cli output discipline, tui boundary), run by `scripts/agent-run.* run` inside the per-OS `test` job's `coverage` suite under `gate --require coverage,doctest` (skips never allowed) on every leg; 0.1.0 runs no browser a11y leg.»

- leaf: .claude/rules/a11y.md:26   class: retired
  leaf-text: «- Tags: `@a11y` + `@sc-*` inside the `playwright` suite; no `a11y` suite, no `webServer`, no second browser stack (no Lighthouse / pa11y).»
  source: .andromeda/a11y-plan.md:387 «- NEVER add a `suite` value (such as `a11y`) or a separate a11y driver outside `scripts/agent-run.* run`.»
  replace-with: «- No new `suite` enum value (such as `a11y`) without a Decisions Log entry, and no separate a11y driver outside `scripts/agent-run.* run`.»
