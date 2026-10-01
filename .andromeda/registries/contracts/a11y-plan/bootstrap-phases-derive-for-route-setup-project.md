### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from
the contract above. Listed for explicitness — route may reorder /
combine, setup-project may add stack-specific intermediate steps.

- **a11y-tooling-install:**
  - Add `@axe-core/playwright@4.13.0` to `e2e-web/package.json`. Tests declared only `@playwright/test@1.63.0` when the browser pipe landed (chunk 2026-09-27-browser-verdict-reachability); the axe pin is this phase's.
  - Add the devDependencies `colorjs.io@0.7.1`, `tabbable@6.5.0`, `@guidepup/virtual-screen-reader@0.33.0`, `html-validate@11.16.0` and `eslint-plugin-jsx-a11y` (its version OPEN, owned by the route's frontend-toolchain entry), plus its `eslint` core peer, pinned exactly through the lockfile (Decisions Log → Resolved questions → ESLint core version).
  - Create `e2e-web/fixtures/a11y.ts` with `makeAxeBuilder` (tags `['wcag2a','wcag2aa','wcag21a','wcag21aa','wcag22aa']` + the five best-practice rules by id), the scrubber, and the violation-row writer.
  - Create the html-validate config declaring any `viola-*` elements the rendered page carries (how the components map onto React's output is OPEN, owned by the frontend-toolchain entry).
- **focus-management-library-install:** none at runtime (native `:focus-visible`, `<details>`/`<summary>`, fragment links). The only install is the test-side oracle `tabbable@6.5.0`. The v1.x Raised-3 confirmation, if built, uses native `<dialog>.showModal()` with no library.
- **aria-component-library-install:** none. Native HTML plus WAI-ARIA APG patterns, rendered by the page's React + TypeScript components (founder ruling 2026-09-30).
- **contrast-verification-harness-setup:** the `a11y-tokens` test in `e2e-web/tests/bay-steady-state.spec.ts`, using colorjs.io over the Section 6 token-pair list, with the forced-colors and negative-placement DOM walk.
- **screen-reader-test-spec-setup:**
  - VSR route plus import helper in `e2e-web/fixtures/a11y.ts`, with the MutationObserver fallback helper.
  - `a11y/sr-pass/TEMPLATE.json` with one row per Section 4 path step for NVDA 2026.2 / VoiceOver / Orca.
- **a11y-ci-gate-wire:**
  - a11y specs are tagged `@a11y` + `@sc-*` inside the `playwright` suite, and `gate --require playwright` is unchanged.
  - `ci.yml` ubuntu leg gets a lint step before `run --browser`: `npx --prefix e2e-web eslint -c e2e-web/eslint.config.js -f json` over the JSX/TSX sources of the `viola-ui` crate (`crates/viola-ui/`, following the workspace `crates/<name>` layout; the exact source glob is the config's `files` entry). `eslint.config.js` lives in `e2e-web/` so `eslint-plugin-jsx-a11y` resolves from `e2e-web/node_modules`. The step also runs `npx --prefix e2e-web html-validate --config e2e-web/.htmlvalidate.json --formatter json` on the `assets/index.html` that `viola-ui` embeds. Both write to `e2e-web/test-results/lint/`.
  - The html-validate config `e2e-web/.htmlvalidate.json` (created in a11y-tooling-install) declares every `viola-*` element in `elements`: `viola-session-row` and `viola-transfer` with permitted content `tr` only, with `tbody` extended to permit those two hosts (their `display: contents` host sits between `<tbody>` and `<tr>`); `viola-atis` and `viola-event-feed` as flow content; `viola-readback` as flow and phrasing content with no interactive descendants permitted, because it renders inside the tape send line's `<summary>` (phrasing content only) as well as inside the `<viola-transfer>` `<td>`. The same config serves the lint stage and the rendered-DOM check (SC 4.1.1 row), so `no-unknown-elements` and the content-model rules do not misfire.
  - This phase creates `e2e-web/a11y/sc-coverage.json` with one entry per Section 3 per-SC map row (ranges expanded; SC id, level and "Applies in v1" value). A post-gate `jq` SC-coverage and budget step runs over the Playwright JSON and `e2e-web/test-results/a11y/*.ndjson` against that file. It runs with `if: always()`, as does the `e2e-web/test-results/` upload step (`actions/upload-artifact` v7.0.1, SHA-pinned per zizmor; this phase adds it to the ubuntu leg if `ci.yml` lacks it), so it still runs and reports after `gate --require playwright` has failed the job. It writes `sc-coverage-report.json` (Section 9 → Aggregation row; Section 10 budget failure condition).
- **violation-json-emission-wire:** the fixture writes `e2e-web/test-results/a11y/<test-id>.ndjson` rows in the Section 3 shape (obs field names unrenamed, `event:"a11y-violation"`, `process:"ui"`, `version` from `GET /health`).

route uses this list to plan phase ordering (typically:
a11y-tooling-install → focus-management-library-install →
aria-component-library-install → contrast-verification-harness-setup
→ screen-reader-test-spec-setup → a11y-ci-gate-wire →
violation-json-emission-wire). setup-project uses this list to
materialize each phase's bootstrap script + dependency list +
verification command.

---
