group: a11y-plan
read: .claude/docs/a11y-summary.md · 63 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 40 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:7   class: stale
  leaf-text: «applies to web-spa only; cli and tui get output-discipline and boundary assertions with no conformance claim.»
  source: .andromeda/a11y-plan.md:19 «_Justification: cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim._»
  replace-with: «cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim; the frontend is deferred to 0.2.0 and its technology is not chosen.»

- leaf: .claude/docs/a11y-summary.md:9   class: retired
  leaf-text: «**WCAG target:** WCAG 2.1 AA + SC 2.4.11 + SC 2.5.8 (WCAG 2.2 AA), with one documented exception: SC 1.4.10 below 760 CSS px (no v1 layout there).»
  source: .andromeda/registries/contracts/a11y-plan/wcag-criteria-mapping.md:3 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «**WCAG target:** none in 0.1.0: the frontend is deferred to 0.2.0 (its technology is not chosen), and the cli and the `viola run` TUI make no conformance claim.»

- leaf: .claude/docs/a11y-summary.md:13   class: stale
  leaf-text: «The a11y harness runs inside the tests' Playwright driver (no second browser stack) and emits violation rows that reuse the obs field names.»
  source: .andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md:5 «- **cli / tui:** no a11y rows. Output-discipline failures surface only as nextest failures in `suites[].failures[]`.»
  replace-with: «The a11y harness checks cli and tui output discipline with assert_cmd / trycmd / portable-pty and emits no a11y rows (failures surface only as nextest failures in `suites[].failures[]`); the frontend's harness is deferred to 0.2.0.»

- leaf: .claude/docs/a11y-summary.md:16   class: retired
  leaf-text: «**web-spa:** `@axe-core/playwright` 4.13.0 via one shared `makeAxeBuilder` fixture (tags `wcag2a wcag2aa wcag21a wcag21aa wcag22aa` + five best-practice rules by id), plus Playwright aria snapshots / `toHaveAccessibleName` / `forcedColors` / `reducedMotion`, html-validate 11.16.0, colorjs.io 0.7.1 token pairs, tabbable 6.5.0 tab-order oracle, @guidepup/virtual-screen-reader 0.33.0 announcement proxy, eslint-plugin-jsx-a11y (lint stage; version OPEN, owned by the route's frontend-toolchain entry)»
  source: .andromeda/registries/a11y-plan-contracts.toml:9 «labels = ["Primary tool per surface"]»
  replace-with: «**web-spa:** none in 0.1.0: the frontend is deferred to 0.2.0 and its technology is not chosen»

- leaf: .claude/docs/a11y-summary.md:20   class: retired
  leaf-text: «**AA base SCs:** every row of the a11y-plan §3 per-SC map, each needing ≥ 1 passing `@sc-<id>`-tagged test (`sc-coverage.json` → `sc-coverage-report.json`); absence rows met by the `surface-absence` spec»
  source: .andromeda/registries/contracts/a11y-plan/wcag-criteria-mapping.md:3 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «**AA base SCs:** none in 0.1.0 (the frontend is deferred to 0.2.0); cli / tui output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 with no conformance claim»

- leaf: .claude/docs/a11y-summary.md:22   class: retired
  leaf-text: «**Structured violation JSON:** harness-only rows in `e2e-web/test-results/a11y/<test-id>.ndjson` (`event:"a11y-violation"`, `process:"ui"`, plus `wcag_criterion`, `violation_type`, `severity`, `surface`, `selector`, `remediation`, `check_source`), validated by `e2e-web/schemas/a11y-row.v1.json`, scrubbed; never in `diagnostics/`.»
  source: .andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md:5 «- **cli / tui:** no a11y rows. Output-discipline failures surface only as nextest failures in `suites[].failures[]`.»
  replace-with: «**Structured violation JSON:** the frontend is deferred to 0.2.0; cli / tui write no a11y rows, and output-discipline failures surface only as nextest failures in `suites[].failures[]`.»

- leaf: .claude/docs/a11y-summary.md:26   class: retired
  leaf-text: «- **a11y-p1 — founder reads the bay and navigates the tape** — web-spa (+ cli caption parity)»
  source: .andromeda/a11y-plan.md:232 «**cli mirror:** a literal pins the caption order the deferred 0.2.0 page must share (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), not a trycmd case»
  replace-with: «(delete the line: Path 1 was web-spa only; its cli caption-order mirror now sits under P6)»

- leaf: .claude/docs/a11y-summary.md:27   class: stale
  leaf-text: «**a11y-p2 — confirmed send and readback, or refusal** — web-spa + cli readback mirror»
  source: .andromeda/a11y-plan.md:110 «- **Surfaces involved:** cli (`viola send` readback mirror `[RB]`).»
  replace-with: «**P2 — confirmed send and readback, or refusal** — cli (`viola send` readback mirror `[RB]`)»

- leaf: .claude/docs/a11y-summary.md:28   class: stale
  leaf-text: «**a11y-p3 — dialog pending, then `answer`** — web-spa cocked strip + cli refusal text»
  source: .andromeda/a11y-plan.md:115 «- **Surfaces involved:** cli (`viola answer`, `unverified-cli` refusal).»
  replace-with: «**P3 — dialog pending, then `answer`** — cli (`viola answer`, `unverified-cli` refusal)»

- leaf: .claude/docs/a11y-summary.md:29   class: stale
  leaf-text: «**a11y-p4 — human takes the wheel** — tui boundary + cli + web WHEEL cell»
  source: .andromeda/a11y-plan.md:121 «- **Surfaces involved:** tui (boundary-only) and cli (`pause`, `release`, `send` refused with `human-typing`).»
  replace-with: «**P4 — human takes the wheel** — tui boundary + cli (`pause`, `release`, `send` refused with `human-typing`)»

- leaf: .claude/docs/a11y-summary.md:30   class: retired
  leaf-text: «- **a11y-p5 — degraded and access states** (503, 404/405, `TAPE stopped`, stale, 401 after a `viola ui` restart) — web-spa»
  source: .andromeda/a11y-plan.md:143 «Security vectors 1–9 are also omitted as not-assertable. Their user-visible outcomes are covered by Path 4.»
  replace-with: «(delete the line: Path 5 was web-spa only and left with the frontend, deferred to 0.2.0)»

- leaf: .claude/docs/a11y-summary.md:31   class: stale
  leaf-text: «**P6 — CLI board, wait/last, verify, launch line** — cli output discipline»
  source: .andromeda/a11y-plan.md:229 «- **P6 — CLI board, wait/last, verify** (cli). No ARIA.»
  replace-with: «**P6 — CLI board, wait/last, verify** — cli output discipline; `viola list`'s `CAPTION` literal pins the caption order the deferred 0.2.0 page must share»

- leaf: .claude/docs/a11y-summary.md:37   class: stale
  leaf-text: «**aria-component-library-install** — none (native HTML + WAI-ARIA APG, rendered by the page's React + TypeScript components)»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:13 «- **aria-component-library-install:** none. The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «**aria-component-library-install** — none; the frontend is deferred to 0.2.0 and its technology is not chosen»

- leaf: .claude/docs/a11y-summary.md:39   class: stale
  leaf-text: «**screen-reader-test-spec-setup** — VSR route + import helper (MutationObserver fallback) + `a11y/sr-pass/TEMPLATE.json`»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:16 «  - VSR route plus import helper in `e2e-web/fixtures/a11y.ts`, with the MutationObserver fallback helper.»
  replace-with: «**screen-reader-test-spec-setup** — VSR route + import helper in `e2e-web/fixtures/a11y.ts` (MutationObserver fallback)»

- leaf: .claude/docs/a11y-summary.md:40   class: omission
  leaf-text: «`sc-coverage.json` + post-gate aggregation (`if: always()`)»
  source: .andromeda/a11y-plan.md:319 «stands until the route entry "Remove the browser leg" removes it; that entry's wrap amends this text.»
  replace-with: «`sc-coverage.json` + post-gate aggregation (`if: always()`); the browser leg (lint, E2E, aggregation, `run --browser`, `e2e-web/`) stands until the route entry "Remove the browser leg" removes it»

- leaf: .claude/docs/a11y-summary.md:45   class: stale
  leaf-text: «NEVER use manual screen-reader / keyboard / contrast passes as the only evidence — every SC needs a passing tagged test.»
  source: .andromeda/a11y-plan.md:410 «- NEVER use manual screen reader / keyboard / contrast as ONLY»
  replace-with: «NEVER use manual screen-reader / keyboard / contrast passes as the only method — they are supplemental to automated checks.»

- leaf: .claude/docs/a11y-summary.md:46   class: stale
  leaf-text: «NEVER claim WCAG conformance for the cli or the `viola run` TUI, or label web-spa "WCAG 2.2 AA".»
  source: .andromeda/a11y-plan.md:367 «- NEVER claim WCAG conformance for the cli or the `viola run` TUI (the wrapper-hosted TUI, whether `viola run` or a passed `viola revive` started it).»
  replace-with: «NEVER claim WCAG conformance for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it.»

- leaf: .claude/docs/a11y-summary.md:49   class: retired
  leaf-text: «NEVER write a11y rows into `<home>/diagnostics/` or rename an obs binding field.»
  source: .andromeda/a11y-plan.md:372 «- NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste»
  replace-with: «NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing.»

- leaf: .claude/docs/a11y-summary.md:53   class: retired
  leaf-text: «- D-A11Y-02 — semantic HTML first: native `<header>`/`<main>`/`<table>`/`<caption>`/`<dl>`, headings `VIOLA` (h1) + three h2; row hosts are role-less `display: contents` rendering native `<tr>` (overrides design's `role="row"` on the host).»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «- The frontend is deferred to 0.2.0 and its technology is not chosen, so no page structure, live-region, announcer or focus decision binds in 0.1.0.»

- leaf: .claude/docs/a11y-summary.md:54   class: retired
  leaf-text: «- D-A11Y-03 — the tape keeps `role="log"` with explicit `aria-live="off"`; only the single polite `role="status"` region speaks.»
  source: .andromeda/a11y-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the tape and the status region left with the frontend, deferred to 0.2.0)»

- leaf: .claude/docs/a11y-summary.md:55   class: retired
  leaf-text: «- D-A11Y-06 — the announcer scope: cocks, refusals, `TAPE stopped`, the 401 access strip, and post-render 503/404/405 strips; no `alert`.»
  source: .andromeda/a11y-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the announcer left with the frontend, deferred to 0.2.0)»

- leaf: .claude/docs/a11y-summary.md:56   class: retired
  leaf-text: «- D-A11Y-07 — initial focus stays on `<body>`; the skip link targets `#tape-end` (`tabindex="-1"`, visually hidden "end of tape").»
  source: .andromeda/a11y-plan.md:245 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the page's focus order left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:12   class: stale
  leaf-text: «Label: WCAG 2.1 AA + SC 2.4.11 + SC 2.5.8 for web-spa only, with the SC 1.4.10 < 760 CSS px exception; the CLI and TUI make no conformance claim.»
  source: .andromeda/registries/contracts/a11y-plan/wcag-criteria-mapping.md:3 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «No WCAG label in 0.1.0: the frontend (the `viola ui` page) is deferred to 0.2.0 and its technology is not chosen; the CLI and TUI make no conformance claim.»

- leaf: .claude/rules/a11y.md:14   class: retired
  leaf-text: «## Semantic HTML first, ARIA second»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «## The frontend (deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:15   class: retired
  leaf-text: «- `<html lang="en">`; `<header>` (banner) rendered by the `viola-atis` component with `<h1>VIOLA</h1>` and a native `<dl>`; one `<main>` wrapping racks, tape and announcer; `<h2>` = `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE`; no `<h3>`, no nav/footer/aside/form.»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «- The frontend is deferred to 0.2.0 and its technology is not chosen: no page structure, ARIA, announcement, focus or visual-token rule binds in 0.1.0.»

- leaf: .claude/rules/a11y.md:16   class: retired
  leaf-text: «- Racks are native `<table>` + `<caption>` + six `<th scope="col">`. `viola-session-row` / `viola-transfer` render native `<tr>`/`<td>`; any element left between `<tbody>` and the `<tr>` is role-less with `display: contents` — never `role="row"`/`table`/`cell` on a wrapper, never a role set imperatively (`setAttribute` from an effect or ref; a static JSX lint cannot see it). Roles belong in the JSX.»
  source: .andromeda/a11y-plan.md:209 «| cli | N/A (terminal). Structural equivalents: BAY context line → uppercase caption line → `-- UNWRAPPED - READ-ONLY --` separator;»
  replace-with: «(delete the line: the racks left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:17   class: retired
  leaf-text: «- The only ARIA roles: `log` (the tape, with explicit `aria-live="off"` and `aria-labelledby` the `TAPE` `<h2>`) and `status` (one visually hidden `aria-live="polite"` region, present and empty at load, clip-hidden — never `display:none`). No `alert`, no `aria-live="assertive"`.»
  source: .andromeda/a11y-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the tape log and the status region left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:18   class: retired
  leaf-text: «- Only the readback `.rb` square is `aria-hidden="true"`; the host and its word stay exposed; nothing focusable inside it.»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the readback box left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:19   class: retired
  leaf-text: «- Tape lines are native `<details>`/`<summary>` — no `aria-expanded`, no scripted disclosure. No `aria-label` where visible text already names the element.»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the tape lines left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:21   class: retired
  leaf-text: «## Announcements (exactly these)»
  source: .andromeda/a11y-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the heading: no announcement rule binds in 0.1.0)»

- leaf: .claude/rules/a11y.md:22   class: retired
  leaf-text: «- A strip turning cocked · a readback refusal · `TAPE stopped` · the 401 access strip (instead of `TAPE stopped` after a `viola ui` restart) · 503/404/405 rack strips appearing after first render. Never tape arrivals, stale, `budget-paused`, `expired`, `skipped`, the new-lines count, or recovery. Never the token, URL, `?t=` or a path.»
  source: .andromeda/a11y-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the status announcer left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:25   class: stale
  leaf-text: «- Tab order: skip link `skip to tape` (first, visible on focus, `href="#tape-end"`) → every tape `<summary>` oldest to newest → `N new lines below` anchor (only while scrolled up). Strips, captions, markers, ATIS cells and the announcer are never tab stops; no positive `tabindex`; `#tape-end` is `tabindex="-1"`.»
  source: .andromeda/a11y-plan.md:248 «- **tui:** the terminal belongs to the child. Human keystrokes are never blocked or delayed past the atomic paste.»
  replace-with: «- cli: no interactive prompt, pager or redraw, and input is stdin / `--file` only; tui: the terminal belongs to the child, and human keystrokes are never blocked or delayed past the atomic paste. The page's tab order is deferred with the frontend to 0.2.0.»

- leaf: .claude/rules/a11y.md:26   class: retired
  leaf-text: «- Initial focus stays on `<body>`; no autofocus. No focus movement on SSE arrival, cock/revert, refusal, strips, resume or recovery; trimming and re-render never drop the focused `<summary>`; auto-follow never scrolls it out of view; jumps are instant.»
  source: .andromeda/a11y-plan.md:245 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the page's focus rules left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:27   class: retired
  leaf-text: «- `:focus-visible` uses `--focus-ring` / `--focus-w` / `--focus-offset`; the sticky header never covers the focused element (`scroll-padding-top`). No single-key shortcuts, no trap.»
  source: .andromeda/a11y-plan.md:245 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the focus-ring and sticky-header rules left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:30   class: stale
  leaf-text: «- State is always a printed word (`read back`, `unable · <reason> · <detail>`, `stale`, `DIALOG <kind>`, `expired`, `unconfirmable`, `human`, `unverified-cli`, `budget-paused`); colour is a second cue. Contrast pairs and negative placements per a11y-plan §6; forced colours keep borders, the strike becomes dashed + `unable`, the cock band maps to `Highlight`.»
  source: .andromeda/a11y-plan.md:258 «amber SGR only on the `DIALOG` word, dim SGR on stale rows and bold SGR only on NAME column values (the callsign; design cli), always beside the word.»
  replace-with: «- On the CLI, state is always a printed word (`stale`, `DIALOG <kind>`, `expired`, `budget-paused`, `unverified-cli`); colour is a second cue only (amber SGR only on `DIALOG`, dim on stale rows, bold only on NAME values, always beside the word), `unable` / `fail` are never coloured, and viola emits no SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`. The page's contrast pairs are deferred with the frontend to 0.2.0.»

- leaf: .claude/rules/a11y.md:31   class: retired
  leaf-text: «- Text spacing (SC 1.4.12): fixed-height tokens (`--line-h`, `--strip-h`, `--lh-display`, `--lh-label`) must not clip; truncated cells keep full text in the DOM, no `title` tooltips.»
  source: .andromeda/a11y-plan.md:256 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete the line: the text-spacing tokens left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:34   class: stale
  leaf-text: «- axe only through `makeAxeBuilder` in `e2e-web/fixtures/a11y.ts` (tags `wcag2a, wcag2aa, wcag21a, wcag21aa, wcag22aa` + the five best-practice rules by id; rule-execution check after each `analyze()`); specs never call `withTags` / `disableRules` / `options`. Never `bypassCSP`, never an axe `locale`.»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:10 «Create `e2e-web/fixtures/a11y.ts` with `makeAxeBuilder` (tags `['wcag2a','wcag2aa','wcag21a','wcag21aa','wcag22aa']` + the five best-practice rules by id), the scrubber, and the violation-row writer.»
  replace-with: «- axe comes through `makeAxeBuilder` in `e2e-web/fixtures/a11y.ts` (tags `wcag2a, wcag2aa, wcag21a, wcag21aa, wcag22aa` + the five best-practice rules by id); the browser leg it serves stands until the route entry "Remove the browser leg" removes it.»

- leaf: .claude/rules/a11y.md:35   class: retired
  leaf-text: «- Verdict per state: `violations: []` and `color-contrast` `incomplete: []`; a CSP / Trusted Types console violation fails the test. Test stylesheets and the VSR bundle come through same-origin `page.route` URLs (`/assets/__a11y/*`), never inline.»
  source: .andromeda/registries/a11y-plan-contracts.toml:9 «labels = ["Primary tool per surface"]»
  replace-with: «(delete the line: the per-state axe verdict, the CSP console check and the routed test assets left with the frontend, deferred to 0.2.0)»

- leaf: .claude/rules/a11y.md:36   class: retired
  leaf-text: «- Violation rows go to `e2e-web/test-results/a11y/<test-id>.ndjson` (`event:"a11y-violation"`, `process:"ui"`, harness-only, schema `e2e-web/schemas/a11y-row.v1.json`), scrubbed (`url` → path, drop `nodes[].html`) — never into `diagnostics/`.»
  source: .andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md:5 «- **cli / tui:** no a11y rows. Output-discipline failures surface only as nextest failures in `suites[].failures[]`.»
  replace-with: «- cli / tui write no a11y rows: output-discipline failures surface only as nextest failures in `suites[].failures[]`; the web row shape is deferred with the frontend to 0.2.0.»

- leaf: .claude/rules/a11y.md:37   class: stale
  leaf-text: «- Tags: `@a11y`, one `@sc-<id>` per verified SC, path ids `a11y-p1`…`a11y-p5` in titles; no `a11y` suite, no `webServer`, no second browser stack (no Lighthouse / pa11y).»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:18 «  - a11y specs are tagged `@a11y` + `@sc-*` inside the `playwright` suite, and `gate --require playwright` is unchanged.»
  replace-with: «- Tags: `@a11y` + `@sc-*` inside the `playwright` suite; no `a11y` suite, no `webServer`, no second browser stack (no Lighthouse / pa11y).»
