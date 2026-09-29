### A11y testing tool pick

- **Primary tool per surface:**
  - **web-spa:** `@axe-core/playwright` 4.13.0 (bundling axe-core 4.13.x) inside the tests' Playwright Test 1.63.0 headless-Chromium driver (a11y-scope Sec 2 and Sec 3; research "Per-Surface A11y Testing Library"). The same driver also hosts these complements:
    - Playwright 1.63.0 accessibility-tree assertions: `toMatchAriaSnapshot`, `ariaSnapshotJSON`, `toHaveRole`, `toHaveAccessibleName`, and the `forcedColors` / `reducedMotion` TestOptions.
    - html-validate 11.16.0 run over `page.content()` in Node.
    - colorjs.io 0.7.1 for token pairs.
    - tabbable 6.5.0 as the tab-order oracle.
    - @guidepup/virtual-screen-reader 0.33.0 as the announcement proxy.
    - eslint-plugin-lit-a11y 5.1.1 at lint time.
  - **cli:** no a11y tool exists for terminal output. Output discipline is checked with assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1, running in the `nextest-integration` / `nextest-e2e` suites locally, and in the CI `test` job's `coverage` suite (one instrumented nextest run), on all three OS legs.
  - **tui:** portable-pty outer-PTY boundary driver, using the inherited `=0.8.1` pin (Decisions Log D-A11Y-13).
  - **Rejected:** `lighthouse` 13.5.0, `pa11y` 10.0.0 / `pa11y-ci` 4.1.1 and `@lhci/cli` 0.15.1 each need a second Puppeteer/CDP client, which breaks Overseer Direction 1. `playwright-lighthouse` 4.0.0 is stale. The WAVE API cannot reach loopback.
- **Configuration:** one shared a11y-owned fixture, `makeAxeBuilder`, lives in `e2e-web/fixtures/a11y.ts`. Specs call only this fixture and never `withTags` / `disableRules`, which keeps the tests-plan rule.
  - **Tags:** `['wcag2a','wcag2aa','wcag21a','wcag21aa','wcag22aa']`. `wcag22aa` is required because `target-size` (SC 2.5.8) is `enabled:false` in a default run.
  - **Page-level best-practice rules, enabled by id and gating:** `region`, `landmark-one-main`, `landmark-banner-is-top-level`, `page-has-heading-one`, `heading-order`. They carry no SC tag (a11y-research, axe item (d)), so their rows map to `wcag_criterion:"1.3.1"` only, as a11y-owned SC 1.3.1 evidence. SC 2.4.6 is never credited to axe (research lists it under "Not covered by any axe rule"). Its evidence is html-validate `heading-level` / `empty-heading` and the aria snapshot (SC 2.4.6 map row). Because they are page-level rules, `analyze()` always runs on the full page with no `include` / `exclude`.
  - **Disabled rules:** none. Never pass `locale`: it triggers doT `new Function` under `require-trusted-types-for 'script'`. Never use `bypassCSP`.
  - **Rule-execution check (gating):** that `.options({rules})` before `.withTags()` keeps the five best-practice rules running under a tag-type `runOnly` is an assumption (a11y-research does not state it), and `target-size` runs only through the `wcag22aa` tag. After every `analyze()`, the fixture asserts that each of `region`, `landmark-one-main`, `landmark-banner-is-top-level`, `page-has-heading-one`, `heading-order` and `target-size` appears in one of `passes[]` / `violations[]` / `incomplete[]` / `inapplicable[]`. A missing id fails the test (`violation_type:"axe-rule-not-run"`). This also catches a merge-to-replace change in `options()` under the unpinned axe-core `4.13.x` range.
  - **Illustrative anchor** (assumed ordering: `options()` first, then `withTags()`; the rule-execution check above is the proof):
    ```ts
    export const makeAxeBuilder = (page: Page) => new AxeBuilder({ page })
      .options({ rules: Object.fromEntries(BP_RULES.map(id => [id, { enabled: true }])) })
      .withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa']);
    ```
  - **Verdict per `analyze()`:** `violations` must be `[]`, and `incomplete` filtered to `id === 'color-contrast'` must be `[]`. Other `incomplete` items become `level:"WARN"` rows that do not gate.
