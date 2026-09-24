# Accessibility Summary — viola

_Distilled from `.andromeda/a11y-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## A11y tier

**Tier:** Standard (1) — applies to web-spa only; cli and tui get output-discipline and boundary assertions with no conformance claim.

**WCAG target:** WCAG 2.1 AA + SC 2.4.11 + SC 2.5.8 (WCAG 2.2 AA), with one documented exception: SC 1.4.10 below 760 CSS px (no v1 layout there). No AAA escalation: no trigger fired.

## Harness contract (§3)

The a11y harness runs inside the tests' Playwright driver (no second browser stack) and emits violation rows that reuse the obs field names. See `.claude/rules/a11y.md` for path-scoped enforcement.

- **Automated tools per surface:**
  - **web-spa:** `@axe-core/playwright` 4.13.0 via one shared `makeAxeBuilder` fixture (tags `wcag2a wcag2aa wcag21a wcag21aa wcag22aa` + five best-practice rules by id), plus Playwright aria snapshots / `toHaveAccessibleName` / `forcedColors` / `reducedMotion`, html-validate 11.16.0, colorjs.io 0.7.1 token pairs, tabbable 6.5.0 tab-order oracle, @guidepup/virtual-screen-reader 0.33.0 announcement proxy, eslint-plugin-lit-a11y 5.1.1 (lint stage)
  - **Mobile:** N/A (phone view deferred)
  - **CLI / TUI:** assert_cmd + trycmd + portable-pty outer-PTY output-discipline checks on all three OSes; manual NVDA / VoiceOver / Orca passes supplemental only
- **WCAG criteria mapping:**
  - **AA base SCs:** every row of the a11y-plan §3 per-SC map, each needing ≥ 1 passing `@sc-<id>`-tagged test (`sc-coverage.json` → `sc-coverage-report.json`); absence rows met by the `surface-absence` spec
  - **AAA escalations:** none
- **Structured violation JSON:** harness-only rows in `e2e-web/test-results/a11y/<test-id>.ndjson` (`event:"a11y-violation"`, `process:"ui"`, plus `wcag_criterion`, `violation_type`, `severity`, `surface`, `selector`, `remediation`, `check_source`), validated by `e2e-web/schemas/a11y-row.v1.json`, scrubbed; never in `diagnostics/`.

## Critical paths (must-be-accessible)

- **a11y-p1 — founder reads the bay and navigates the tape** — web-spa (+ cli caption parity)
- **a11y-p2 — confirmed send and readback, or refusal** — web-spa + cli readback mirror
- **a11y-p3 — dialog pending, then `answer`** — web-spa cocked strip + cli refusal text
- **a11y-p4 — human takes the wheel** — tui boundary + cli + web WHEEL cell
- **a11y-p5 — degraded and access states** (503, 404/405, `TAPE stopped`, stale, 401 after a `viola ui` restart) — web-spa
- **P6 — CLI board, wait/last, verify, launch line** — cli output discipline

## Bootstrap phases (§3)

1. **a11y-tooling-install** — devDependencies (colorjs.io, tabbable, virtual-screen-reader, html-validate, eslint-plugin-lit-a11y + eslint), `e2e-web/fixtures/a11y.ts` (`makeAxeBuilder`, scrubber, row writer), html-validate config declaring the `viola-*` elements
2. **focus-management-library-install** — none at runtime; tabbable test-side only
3. **aria-component-library-install** — none (native HTML + WAI-ARIA APG, Lit light DOM)
4. **contrast-verification-harness-setup** — the `a11y-tokens` test in `bay-steady-state.spec.ts`
5. **screen-reader-test-spec-setup** — VSR route + import helper (MutationObserver fallback) + `a11y/sr-pass/TEMPLATE.json`
6. **a11y-ci-gate-wire** — `@a11y` + `@sc-*` tags, ubuntu lint step before `run --browser`, `sc-coverage.json` + post-gate aggregation (`if: always()`)
7. **violation-json-emission-wire** — the row writer in the fixture

## Universal anti-patterns

- NEVER use manual screen-reader / keyboard / contrast passes as the only evidence — every SC needs a passing tagged test.
- NEVER claim WCAG conformance for the cli or the `viola run` TUI, or label web-spa "WCAG 2.2 AA".
- NEVER add a second browser automation stack (Lighthouse, pa11y) beside Playwright.
- NEVER parse the rendered `claude` TUI or take screenshots as a11y evidence.
- NEVER write a11y rows into `<home>/diagnostics/` or rename an obs binding field.

## Critical decisions

- D-A11Y-02 — semantic HTML first: native `<header>`/`<main>`/`<table>`/`<caption>`/`<dl>`, headings `VIOLA` (h1) + three h2; row hosts are role-less `display: contents` rendering native `<tr>` (overrides design's `role="row"` on the host).
- D-A11Y-03 — the tape keeps `role="log"` with explicit `aria-live="off"`; only the single polite `role="status"` region speaks.
- D-A11Y-06 — the announcer scope: cocks, refusals, `TAPE stopped`, the 401 access strip, and post-render 503/404/405 strips; no `alert`.
- D-A11Y-07 — initial focus stays on `<body>`; the skip link targets `#tape-end` (`tabindex="-1"`, visually hidden "end of tape").
- D-A11Y-05 — Lighthouse / pa11y / WAVE dropped (second client, stale, or unable to reach loopback).

---

**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `.claude/rules/a11y.md`.
