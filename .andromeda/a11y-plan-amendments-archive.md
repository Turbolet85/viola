# a11y-plan — archived amendment originals

Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-09-27-wrapper-channel wrap — 4 re-worded · 0 pruned

## 2026-09-24-supply-chain-and-workflow-gates — Platform: ci.yml is the one push/PR workflow
**Section:** §9 Pipeline integration → Platform
**Change:** "one workflow `ci.yml`" now reads "one push/PR workflow `ci.yml`". The scheduled `nightly.yml` carries no a11y step, and the E2E/a11y leg stays in `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates added `nightly.yml`, the weekly `cargo deny check advisories` run. The orchestrator raised this site because the same workflow-count grep hit it; no detector proposed it. Sweep: see architecture-amendments.md, same entry heading. For this master, :1108 was amended; the other "all three OS legs" hits were left unchanged as unrelated.

## 2026-09-24-diagnostics-plane — stale a11y-violation resolved-question bullet
**Section:** §12 A11y Decisions Log → Resolved questions
**Change:** The `a11y-violation` bullet now states what Z7 / D-A11Y-09 decided: it is not a product `event` value. It is a harness-only row validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`, and the shipped `ObsEvent` and `diag-line.v1.json` enum (19 values) carry none.
**Why:** chunk 2026-09-24-diagnostics-plane, report Spec claims disproved #3 (contract test green). The D-a11y-obs-schema proposal was applied as routine.

Sweep: `accepted as a tests \+ obs enum` over all seven masters gives 1 remaining hit, a11y :1389. It is a Decisions-Log history entry, left unchanged. The §3 wording it names ("new closed-enum value") has 0 hits outside that entry.

## 2026-09-24-quality-gates — CLI output-discipline evidence reports under the CI `coverage` suite
**Section:** §1 (CLI output discipline bullet) · §1 cli surface (Automated tool reach) · §9 Per-pipeline-stage table (Unit / integration row) · §10 Standard+ invariants
**Change:** the CLI output-discipline tests still run on all three OS legs. Locally they report under `nextest-integration` / `nextest-e2e`; in CI they report inside the per-OS `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`. The run JSON `suites[]` enum in the table row gains `coverage`.
**Why:** chunk 2026-09-24-quality-gates replaced the CI unit and integration runs with one `run --coverage` (report Harness / gate surface). Raised by the orchestrator at Validate: the a11y detector flagged it as outside both D-a11y invariants.
**Sweep:** `nextest-integration|nextest-e2e` 4 hits, all amended (`:280, :499, :1115, :1153`). `:604` ("failures surface only as nextest failures in `suites[].failures[]`") stays true under `coverage`, no change. `.claude/docs/a11y-summary.md` and `.claude/rules/a11y.md`: 0 hits, no change.

## 2026-09-25-pty-wrapper-on-windows — Windows zero-viola-bytes oracle is literal absence
**Section:** §3 A11y Assertion Harness Contract → Keyboard test harness → Tooling · §6 State color tokens → CLI equivalent
**Change:** the zero-viola-bytes clause means viola's own literals absent on every leg; "no SGR or cursor control the child did not emit" (byte-for-byte against an unwrapped run) holds on Linux and macOS only. On Windows ConPTY emits its own `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title and `ESC[?25h` on every spawn and re-renders nested output, so the windows-2025 `viola run` check is literal absence; §6 "zero SGR under `viola run`" becomes zero SGR of viola's own.
**Why:** chunk 2026-09-25-pty-wrapper-on-windows report Spec claims disproved 4 (research fact 4, measured on the Windows host), expected amendment 7. Rejected (E4): the §1 edits at :98 and :179 — §1 is verbatim from a11y-scope.md per D-A11Y-15 and those lines are not deferral clauses; §3 and §6 carry the truth. Not amended (E3): "ConPTY swallows focus reports" — no measured basis in the chunk's artifacts; carried as a labelled HYPOTHESIS on the route entry that builds focus/mouse-sequence handling.
**Sweep:** `no SGR or cursor control|zero SGR|byte for byte|viola-originated` over the 7 masters: a11y :624, :947 amended; :98, :179 (§1 verbatim) no change per E4; no hit in the other six. Leaves: `.claude/docs/a11y-summary.md`, `.claude/rules/a11y.md` recomputed — neither states the zero-bytes oracle: no change.

## Registry migration (U35) — 2026-09-29

<!-- U35 · a11y-plan.md · ## 3. A11y Assertion Harness Contract · sha256 51d15d80385ffe2ff75fed99cc236a34f555f4a8bac82a16c31536dc450f6874 -->

## 3. A11y Assertion Harness Contract

This section specifies the SPECIFIC contract for how a11y assertions
run from CI / dev / and emit machine-parseable violation JSON.

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

### WCAG criteria mapping

- **Tier coverage (Standard):** the conformance label for web-spa is **WCAG 2.1 AA + SC 2.4.11 + SC 2.5.8 (WCAG 2.2 AA)**, with one documented exception: SC 1.4.10 at viewport widths below 760 CSS px (Decisions Log D-A11Y-04). The other WCAG 2.2 AA additions have no v1 surface: SC 2.5.7 (no dragging), SC 3.2.6 (no help mechanism), SC 3.3.7 and SC 3.3.8 (no inputs, no authentication form). Their absence is machine-asserted, but the label does not claim full 2.2 AA. AAA is not claimed: SC 1.4.6, 1.4.8, 2.3.3, 2.4.13 and 2.5.5 are out of tier.
- **Per-SC verification map.** "Absence" means the `surface-absence` spec, which asserts zero `video, audio, iframe, object, embed, img, input, select, textarea, form, dialog, button` in the rendered DOM in every state.
- **Levels** (for auditing the label row by row; `sc-coverage.json` carries the level per row): Level A — 1.1.1, 1.2.1–1.2.3, 1.3.1–1.3.3, 1.4.1, 1.4.2, 2.1.1, 2.1.2, 2.1.4, 2.2.1, 2.2.2, 2.3.1, 2.4.1–2.4.4, 2.5.1–2.5.4, 3.1.1, 3.2.1, 3.2.2, 3.3.1, 3.3.2, 4.1.1, 4.1.2, 3.2.6 (2.2), 3.3.7 (2.2). Level AA — 1.2.4, 1.2.5, 1.3.4, 1.3.5, 1.4.3–1.4.5, 1.4.10–1.4.13, 2.4.5–2.4.7, 3.1.2, 3.2.3, 3.2.4, 3.3.3, 3.3.4, 4.1.3, 2.4.11 (2.2), 2.5.7 (2.2), 2.5.8 (2.2), 3.3.8 (2.2). No row is AAA.

| SC | Applies in v1 | Agent-runnable verification path (JSON) |
|----|---------------|------------------------------------------|
| 1.1.1 Non-text Content | yes | axe `image-alt`/`svg-img-alt`/`role-img-alt`; `toHaveAccessibleName` on readback word cells while only the `.rb` square is `aria-hidden` |
| 1.2.1–1.2.5 Time-based media | no media | absence |
| 1.3.1 Info and Relationships | yes | axe `aria-required-children`/`aria-required-parent`/`td-has-header`/`th-has-data-cells`/`list`/`listitem` + best-practice landmark/heading rules; html-validate `no-missing-references` (the log's `aria-labelledby` → `TAPE` `<h2>`), `wcag/h63`, `heading-level`, `unique-landmark`, `no-multiple-main` (`prefer-native-element` is SC 4.1.2 evidence per a11y-research); `toMatchAriaSnapshot` of banner/main/table/log tree |
| 1.3.2 Meaningful Sequence | yes | `ariaSnapshotJSON` order equals the design's four-region order; virtual-screen-reader reading order |
| 1.3.3 Sensory Characteristics | yes | Playwright copy assertions: every instruction string (empty rack, 401 strip, 503 strip) names the action in text; `N new lines below` is itself the actuator |
| 1.3.4 Orientation | yes | axe `css-orientation-lock` |
| 1.3.5 Identify Input Purpose | no inputs | absence |
| 1.4.1 Use of Color | yes | state-word assertions per state (`read back`, `unable · <reason> · <detail>`, `stale`, `DIALOG <kind>`, `expired`, `unconfirmable`, `human`, `unverified-cli`, `budget-paused`, and the ATIS words `TAPE connecting` (the pre-`sse-opened` state, held by the harness and never reached by a timeout), `TAPE live`, `TAPE stopped` and `skipped N · N · N`, each asserted in its ATIS `<dd>` by `a11y-p1` / `a11y-p5`) + `forcedColors: 'active'` checks; axe `link-in-text-block` |
| 1.4.2 Audio Control | no audio | absence |
| 1.4.3 Contrast (Minimum) | yes | axe `color-contrast` (+ `incomplete` gate); colorjs.io token pairs (Section 6) |
| 1.4.4 Resize text | yes | axe `meta-viewport`; `scrollWidth <= clientWidth` at 760–1023 (a 1536 CSS px desktop at 200 %), the tests' check re-asserted in `a11y-p1` under `@sc-1.4.4` (Section 2 → Test tags) |
| 1.4.5 Images of Text | no images | absence |
| 1.4.10 Reflow | yes (≥ 760 CSS px) | the tests' reflow checks at 1024 and 760–1023, re-asserted in `a11y-p1` under `@sc-1.4.10`; < 760 is a documented exception |
| 1.4.11 Non-text Contrast | yes | colorjs.io non-text token pairs + `forcedColors` border/strike/band checks |
| 1.4.12 Text Spacing | yes | text-spacing override stylesheet via same-origin `page.route` + `addStyleTag({url})`, then no-clip assertions; axe `avoid-inline-spacing` |
| 1.4.13 Content on Hover or Focus | yes | hover each strip/summary/link and assert `ariaSnapshotJSON` is unchanged; DOM walk asserts no `title` attribute |
| 2.1.1 Keyboard | yes | keyboard walk + tabbable oracle; axe `scrollable-region-focusable`; lint `click-events-have-key-events` / `tabindex-no-positive` / `no-autofocus` (Lint stage) |
| 2.1.2 No Keyboard Trap | yes | a full Tab cycle and a full Shift+Tab cycle both leave the document |
| 2.1.4 Character Key Shortcuts | yes (none exist) | press `a`–`z`, `?`, `/` with focus on body and on a summary; assert URL, title and `ariaSnapshotJSON` unchanged |
| 2.2.1 Timing Adjustable | yes (none exist) | axe `meta-refresh`; there is no cookie expiry, so no timer (Section 8) |
| 2.2.2 Pause, Stop, Hide | yes | axe `blink`/`marquee`; lint `no-distracting-elements` (Lint stage); computed `animation-iteration-count` never `infinite`; the cock/fade transition runs once and resolves ≤ 5 s; auto-follow runs only while the reader is at the bottom, and while focus is inside the tape it never scrolls the focused `<summary>` out of the tape viewport (bounding-rect check after N injected events; Section 5 → Focus restoration → Auto-follow; D-A11Y-17) |
| 2.3.1 Three Flashes | yes | no infinite or looped animation (computed style walk, `motion-policy`) and the `reducedMotion: 'reduce'` override check (Section 6 Motion, `reduced-motion-override`); no flash source exists by design |
| 2.4.1 Bypass Blocks | yes | axe `bypass`; Enter on skip link → `#tape-end` focused |
| 2.4.2 Page Titled | yes | axe `document-title`; `expect(page).toHaveTitle('viola')` / `DIALOG <name> · viola` / `+N`; html-validate `empty-title` |
| 2.4.3 Focus Order | yes | keyboard walk sequence equals the tabbable oracle equals the design set |
| 2.4.4 Link Purpose | yes | axe `link-name`; `toHaveAccessibleName('skip to tape')`, `N new lines below` |
| 2.4.5 Multiple Ways | single-page exception | the tests' route check, re-asserted in `a11y-p1` under `@sc-2.4.5`: only `/` serves HTML (404 elsewhere) |
| 2.4.6 Headings and Labels | yes | html-validate `heading-level`/`empty-heading`; `toMatchAriaSnapshot` headings `VIOLA`, `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE` |
| 2.4.7 Focus Visible | yes | computed `outlineColor/Width/Offset` equal the resolved `--focus-ring`/`--focus-w`/`--focus-offset` after each Tab |
| 2.4.11 Focus Not Obscured (Min) | yes | after each Tab, after each header-growth trigger (`budget-paused`, `expired`, `TAPE stopped`, nonzero `skipped`: every boxed ATIS state that can wrap a header cell), singly and combined (Section 5 Sticky header), and after skip-link activation, the focused element's rect is not fully covered by the `<viola-atis>` rect. The zero-height `#tape-end` counts as obscured if its rect lies within the header rect |
| 2.5.1 / 2.5.2 / 2.5.4 / 2.5.7 | no custom pointer/motion/drag | tabbable oracle proves the only actuators are native `<a>`/`<summary>` |
| 2.5.3 Label in Name | yes | `toHaveAccessibleName` equals visible text for skip link, anchor and each summary |
| 2.5.8 Target Size (Min) | yes | axe `target-size` (via `wcag22aa`); `boundingBox()` of each summary, skip link (when focused) and anchor ≥ 24×24 CSS px. An undersized target passes only under the SC 2.5.8 spacing exception as the SC defines it: a 24 CSS px diameter circle centred on its bounding box intersects no other target and no other undersized target's circle. This is computed from the `boundingBox()` of every tabbable-oracle target in the same state (`target-size-bbox`) |
| 3.1.1 Language of Page | yes | axe `html-has-lang`/`html-lang-valid` |
| 3.1.2 Language of Parts | indeterminate-language content | axe `valid-lang`; DOM assertion that event text carries no guessed `lang` (Section 7) |
| 3.2.1 On Focus | yes | keyboard walk asserts URL/title unchanged and no scroll jump outside the tape on focus (Section 8) |
| 3.2.2 On Input | no inputs | absence; `<details>` toggles and SSE arrivals cause no context change (Section 8) |
| 3.2.3 Consistent Navigation | single page | same as 2.4.5; region order identical across all states (Section 8) |
| 3.2.4 Consistent Identification | yes | the readback word for one `corr` is identical in the tape line and in the `<viola-transfer>` marker; web `columnheader`s equal CLI captions (Section 8) |
| 3.3.1 Error Identification | yes | text assertions on 503/404/405/401 strips, `TAPE stopped` and readback refusal words |
| 3.3.2 Labels or Instructions | no inputs | absence |
| 3.3.3 Error Suggestion | yes (401) | 401 strip contains the plain instruction line and never the token, URL or `.url` path |
| 3.3.4 Error Prevention | no submissions | absence (`form-action 'none'`) |
| 4.1.1 Parsing (2.1 label) | yes | html-validate on rendered DOM (`no-dup-id`, content model with `viola-*` declared) |
| 4.1.2 Name, Role, Value | yes | axe 4.13.0 rules tagged for SC 4.1.2 (`summary-name`, `aria-allowed-attr`, `aria-hidden-focus`, `nested-interactive` …); html-validate; lint-a11y |
| 4.1.3 Status Messages | yes | `role="status"` text assertions + virtual-screen-reader `spokenPhraseLog()` per trigger |
| 3.2.6, 3.3.7, 3.3.8 (2.2) | no surface | absence (Section 8) |

- **Compliance trigger override:** none. The security plan names no regime (no Section 508 / ADA / EAA / EN 301 549 / AODA / JIS X 8341), so no regime-mandated level adds to or overrides the label above.

### Structured violation JSON schema

The binding contract is the obs Log Format JSON Schema (upstream-context Section 6 Obs Plan Excerpt → Log Format JSON Schema), reproduced verbatim in Section 1 of this plan together with the D-21 additions. a11y emissions align to that schema; obs does not align to a11y.

- **Emission location:** a11y rows are harness test artifacts. They are written to `e2e-web/test-results/a11y/<test-id>.ndjson` and attached through `testInfo.attach('a11y-violations', …)`. They are never written to `<home>/diagnostics/` and never emitted by the viola binary.
- **Row shape:** one JSON object per line. It uses the binding field names unrenamed, plus additive snake_case fields:
  ```json
  {"timestamp":"<RFC3339 ms Z>","level":"ERROR","target":"viola_a11y","message":"a11y-violation","event":"a11y-violation","process":"ui","service_name":"viola","version":"<GET /health .version>","os":"linux","surface":"web-spa","check_source":"axe","wcag_criterion":"1.4.3","violation_type":"color-contrast","severity":"serious","selector":"<scrubbed axe target>","remediation":"<axe helpUrl>"}
  ```
- **Binding fields:**
  - `timestamp`, `level`, `target`, `message`, `event` and `process` are the binding fields.
  - `event:"a11y-violation"` marks a harness-only row. It is **not** a value of the product `event` enum, has no `ObsEvent` variant, and is never emitted by a product process. Each row is validated against the tests-owned `e2e-web/schemas/a11y-row.v1.json` (tests Log format → Harness-side a11y rows), never against obs `schemas/diag-line.v1.json` (Decisions Log D-A11Y-09).
  - `message` equals the event name (obs constraint).
  - `process:"ui"` names the process whose served page is under test.
  - `instance` and `corr` are omitted (absent = null). A literal `null` is never written.
  - There is no `trace_id` / `span_id`.
- **Additive fields:**
  - `service_name`, `version` and `os` come from obs Service Identity.
  - `surface` is `web-spa`.
  - `check_source` is one of `axe | html-validate | eslint-lit-a11y | token-contrast | keyboard-walk | aria-snapshot | media-emulation | virtual-sr | dom-assert`.
  - `wcag_criterion` is the bare SC ID. For axe it comes from the `wcagNNN` tag. The five best-practice rules map to `"1.3.1"`. The three check-integrity ids `csp-console`, `scrub-leak` and `axe-rule-not-run` have no SC: their rows omit `wcag_criterion` (absent = null, never a literal `null`). The Section 9 SC coverage ignores rows without it, and they are counted only in `violation_type_counts`.
  - `violation_type` holds the axe or html-validate rule id as-is. Non-axe checks use a11y-owned kebab-case ids from a closed list: `token-pair-contrast`, `token-placement`, `focus-order`, `focus-visible`, `focus-not-obscured`, `focus-lost`, `keyboard-trap`, `unexpected-tab-stop`, `status-announcement`, `log-announced`, `accessible-name`, `aria-tree`, `text-spacing-clip`, `forced-colors-state`, `reduced-motion-override`, `target-size-bbox`, `hover-content`, `character-key-shortcut`, `state-word-missing`, `surface-absence`, `motion-policy`, `auto-follow-scroll`, `keyboard-activation`, `context-change`, `copy-mismatch`, `identification-mismatch`, `lang-guessed`, `timed-expiry`, `reflow-overflow`, `route-not-single`, `csp-console`, `scrub-leak`, `axe-rule-not-run`. The ids from `motion-policy` to `route-not-single` cover gating checks that would otherwise have no id: `motion-policy` = an `infinite` or looped animation, or one running longer than the SC 2.2.2 five-second threshold (SC 2.2.2, 2.3.1); `auto-follow-scroll` = auto-follow scrolls the focused `<summary>` out of the tape viewport (SC 2.2.2, 2.4.11; Section 5 → Auto-follow); `keyboard-activation` = an Enter / Space contract fails (skip link not landing on `#tape-end`, anchor jump not instant, `<summary>` not toggling; SC 2.1.1, 2.4.1); `context-change` = URL, title, scroll outside the tape, or focus changes on focus, on `<details>` toggle or on SSE arrival (SC 3.2.1, 3.2.2); `copy-mismatch` = pinned page text other than a state word is wrong: a fixed-copy string, the document title, an error strip or the 401 instruction line differs from its pinned value, or page text shows a token, `?t=`, URL, `.url` path or Problem URN (SC 1.3.3, 2.4.2, 3.3.1, 3.3.3); `identification-mismatch` = the readback word differs between the tape line and `<viola-transfer>`, or the web `columnheader` order differs from the CLI captions (SC 3.2.4); `lang-guessed` = a tape descendant carries a `lang` attribute (SC 3.1.2); `timed-expiry` = the session cookie has a timed expiry or a notice auto-dismisses (SC 2.2.1); `reflow-overflow` = `scrollWidth > clientWidth` at the 1024 or 760–1023 test viewport in the `a11y-p1` re-assertion of the tests' reflow check (SC 1.4.4, 1.4.10); `route-not-single` = a path other than `/` serves HTML instead of the 404 in the `a11y-p1` re-assertion of the tests' route check (SC 2.4.5, 3.2.3). Region-order differences across states (SC 1.3.2, 3.2.3) are `aria-tree`. Each id has exactly one meaning: `csp-console` is only a `securitypolicyviolation` / Trusted Types console error; `scrub-leak` is only a scrubber match (below); `axe-rule-not-run` is only the rule-execution check (§ A11y testing tool pick → Configuration).
  - `severity` uses the axe impact vocabulary (`minor|moderate|serious|critical`). Non-axe checks, including `scrub-leak` and `axe-rule-not-run`, emit `serious`.
  - `selector` is the axe `target`, which 4.13.0 escapes for control characters, or a Playwright role locator description.
  - `remediation` is the axe `helpUrl`, or the Section-number anchor in this plan.
  - `ci_run_id` / `git_sha` are set from `GITHUB_RUN_ID` / `GITHUB_SHA` and absent on local runs.
- **Scrubbing (Error sanitization trigger):** the fixture scrubs results before any attach or write:
  - axe `url` is reduced to its path (`/`);
  - `nodes[].html` is dropped;
  - any string that matches the token, `Cookie`, `?t=`, an absolute filesystem path (for example the viola home, a user home or the runner workspace; the reduced `url` path `/` and the axe `helpUrl` in `remediation` are not filesystem paths) or `CLAUDE*` fails the test and writes one row with `violation_type:"scrub-leak"`, `severity:"serious"`, the `check_source` of the check whose output leaked, and a `selector` naming the attachment and JSON path of the match — never the matched value. The raw axe JSON attachment is the scrubbed object.
- **Agent parsing:** `jq -c 'select(.event=="a11y-violation" and .level=="ERROR")' e2e-web/test-results/a11y/*.ndjson`.
- **cli / tui:** no a11y rows. Output-discipline failures surface only as nextest failures in `suites[].failures[]`.

### Focus management test harness

- **Driver:** the tests' Playwright Test 1.63.0 driver, per the upstream-context Section 5 Test Harness Contract Summary (binding). There is no separate harness. Each test boots `pw-<spec>-<test id>-<workerIndex>`, navigates `/?t=<token>`, runs `cleanup`, and uses `retries: 0`. Waits use auto-waiting locators and SSE event offsets only.
- **Library:** no runtime focus library. Native platform focus (`:focus-visible`, `<details>`/`<summary>`, fragment links) under Lit 3.3.3 light DOM. tabbable 6.5.0 is test-side only: it is injected with `page.evaluate(source)` and its `tabbable(document.body)` list is the expected-sequence oracle.
- **Pattern:**
  - **Tab walk:** scripted Tab / Shift+Tab traversal from `page.locator('body').focus()`, asserting `toBeFocused()` at each step against the oracle.
  - **Initial focus:** after load, `document.activeElement` is `<body>`.
  - **No focus theft:** for each transition (SSE arrival, auto-follow, cock/revert, readback refusal, 503, 401 strip, `TAPE stopped`, `liveness-changed` stale, E4 `Last-Event-ID` resume, E5 `state-recovered`), focus a mid-tape `<summary>` first, trigger through the harness/fake agent, wait on the DOM signal (`data-dialog`, `data-rb`, strip text), then assert the same element is still focused (`focus-lost` otherwise).
  - **Focus survives trimming:** push beyond the 2000-line DOM cap while the oldest `<summary>` is focused, then assert the same `<summary>` is still focused (`focus-lost` otherwise; Section 5 → Focus restoration). In this trimmed state also assert: the design trim-notice text (`older lines trimmed from view: N — the full tape is events.ndjson`) is a `<p>` inside the `role="log"` `<div>` before the `<ol>` (after the `TAPE stopped` `<p>` when present); it is not a tab stop; and the axe verdict holds (Section 10 state list).
  - **Modal patterns:** no focus trap entry, exit or restoration assertions, because v1 has no modals.

### Keyboard test harness

- **Sequences per ARIA pattern (only patterns present in v1):**
  - Link (skip link, `N new lines below` anchor): Enter activates. The skip link lands focus on `#tape-end`. The anchor jumps to `#tape-end` instantly: assert `scrollTop` reached the bottom in the same frame, with no smooth scroll.
  - Disclosure (native `<summary>`): Enter and Space both toggle `<details>.open`, and focus stays on the `<summary>`.
  - Button, Dialog, Tabs, Combobox, Menu: not present in v1. Their absence is asserted by the `surface-absence` spec. The v1.x brakes get the Button contract (Enter / Space) when built.
  - Global: no single-key shortcuts (SC 2.1.4 assertion), and no trap (SC 2.1.2 full-cycle assertion).
- **Tooling:** Playwright `page.keyboard.press('Tab' | 'Shift+Tab' | 'Enter' | 'Space')` and `expect(locator).toBeFocused()`. For cli, trycmd/assert_cmd cover stdin / `--file` input only. For tui, the portable-pty outer PTY injects human keystrokes during a driver `send` and asserts they are delivered unblocked, and that `\x1b[I`/`\x1b[O`, mouse and resize sequences do not move the wheel. A third boundary assertion covers the zero-viola-bytes clause: with the tests' `viola-fake-agent` as the child, the outer-PTY byte stream across start, a driver `send`, a refused automation send (`human-typing`) and exit contains no viola-originated bytes. On every leg that means none of viola's own message literals (diagnostics, refusal text, `hint:`). On the Linux and macOS legs it also means no SGR or cursor control the child did not emit: the stream is compared byte for byte with an unwrapped fake-agent run under the same outer PTY. On Windows the check is that viola's own literals are absent, because the ConPTY host itself emits bytes on every spawn and re-renders nested output — neither viola's bytes nor the child's. The inbox host (kernel32's ConPTY, conhost) emits `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title holding the exe path and `ESC[?25h` (as measured at chunk 2026-09-25-pty-wrapper-on-windows research fact 4). The sideloaded host, `bin/<key>/conpty/OpenConsole.exe` (the Windows x64 default, `pty_backend` `conpty-sideload`), emits a different preamble, `ESC[1t ESC[c ESC[?1004h ESC[?9001h`, and `ESC[?1004l ESC[?9001l` at exit (as measured at chunk 2026-09-29-sideloaded-conpty, `evidence/da1-stall.md`); its DA1 query `ESC[c` is answered by the terminal, or in the tests by the piped driver, never by viola, which stays silent. The Windows check runs on both backends: the default case on the sideload, and `conpty_sideload`'s tampered-companion case on the inbox fallback. The check never inspects `claude` content (R7). Each of the three boundary clauses is its own nextest case on all three OS legs.

### Screen reader test pattern

- **Automated proxy (gating):** @guidepup/virtual-screen-reader 0.33.0 runs in the same Playwright Chromium.
  - **Loading:** `page.route('**/assets/__a11y/vsr.js', …)` fulfils the package's `browser.js`, and `page.evaluate(() => import('/assets/__a11y/vsr.js'))` loads it from a same-origin path, which satisfies `script-src 'self'`.
  - **Run:** `virtual.start({ container: document.body })`, trigger the state, then assert `spokenPhraseLog()`.
  - **Checks:**
    - the four design/overseer announcements plus the post-render rack error strips (Section 7) appear once each;
    - appending tape lines produces zero live phrases (`log-announced`);
    - the reading order follows the four regions.
  - **CSP guard:** the shared fixture in `e2e-web/fixtures/a11y.ts` records `securitypolicyviolation` events and Trusted Types console errors in every `@a11y` test, not only the VSR specs. It fails the test at teardown with one row per event: `violation_type:"csp-console"`, `severity:"serious"`, the `check_source` of the check running when the event fired, and a `selector` naming the violated directive, never the blocked URI (it can carry `?t=`). This is the runtime evidence for the Section 1 `[resolved: …]` claim that axe injection reaches no Trusted Types sink, and it feeds the Section 10 CSP / Trusted Types failure condition. The first VSR spec's clean-console assertion remains the D-A11Y-11 check on the VSR import.
  - **Fallback:** if TT rejects the import, a `page.addInitScript` MutationObserver recorder scoped to `[role=status],[role=log],[aria-live]` replaces it (Decisions Log).
  - **DOM assertions alongside the proxy:** `role="status"` `toHaveText(...)` per trigger, and `toHaveAccessibleName` per readback word cell.
- **Per-surface test spec (supplemental, founder-owned):**
  - web-spa: NVDA 2026.2 + Edge/Chrome (Windows, live target); VoiceOver + Safari (macOS); Orca (GNOME 50 cycle) + Firefox/Chromium (Linux).
  - cli: NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal.
  - tui: NVDA + Windows Terminal. The wrapped `claude` TUI must read the same as unwrapped (Must-Work O1).
- **Manual pass spec format:** one JSON row per step in `a11y/sr-pass/<date>-<at>.json`, shaped like ARIA-AT rows. Example: `{path:"P2", step, at:"nvda-2026.2", browser:"edge", surface:"web-spa", expected:"send to builder unable, not-delivered, …", heard, result:"PASS"|"FAIL"}`. Record the Orca version per pass.
- **Supplemental to automated:** the SR pass is NEVER the sole verification. Axe and html-validate cover SC 4.1.2 programmatically, and VSR plus the status-text assertions cover SC 4.1.3. The SR pass verifies real-AT announcement quality. `@guidepup/guidepup` 0.34.0 + `@guidepup/playwright` 0.19.1 (real NVDA/VoiceOver) are optional and founder-local only, not in the CI gate (see the Decisions Log).

### Contrast verification harness

- **Source-of-truth tokens:** design token names from upstream-context Section 3 (reproduced verbatim in Section 6):
  - colour pairs over `--surface-bay`, `--surface-strip`, `--surface-inset` and `--rb-fill`;
  - focus tokens `--focus-ring`, `--focus-w`, `--focus-offset`, `--radius`;
  - target tokens `--line-h`, `--strip-h`, `--rb-size`.
  - Dark only: one palette (Overseer Direction 6).
- **Verification tool:**
  - axe 4.13.0 `color-contrast` (SC 1.4.3 on rendered text, DejaVu fallback on ubuntu), gated on both `violations` and `incomplete`.
  - A colorjs.io 0.7.1 token checker test (`a11y-tokens` in `e2e-web/tests/bay-steady-state.spec.ts`):
    - reads each token with `getComputedStyle(document.documentElement).getPropertyValue('<name>')`;
    - composites alpha over its declared surface;
    - computes `Color.contrast(fg, bg, 'WCAG21')`;
    - attaches one JSON row per pair, `{pair, ratio, min, sc, pass}`;
    - fails on any `pass:false` (row `violation_type:"token-pair-contrast"`).
  - A DOM walk for negative placements (`token-placement`).
- **WCAG SC mapping:** SC 1.4.3 (AA 4.5:1 text; 3:1 large text as computed by axe from font size and weight) and SC 1.4.11 (AA 3:1 non-text: rules, strike, cock band, focus ring). SC 1.4.6 (AAA) is not claimed, and `color-contrast-enhanced` stays disabled.

### CI integration

- **Pipeline integration:** a11y runs inside the tests' E2E pipeline per the upstream-context Section 5 Test Harness Contract Summary (binding 5-command discipline: `boot`, `run`, `status`, `cleanup`, `logs`).
  - a11y specs are ordinary files in `e2e-web/`, so they run in the existing `playwright` suite.
  - There is no separate driver and no new `suite` enum value.
- **Command:**
  - `scripts/agent-run.sh run --browser` (POSIX) or `scripts/agent-run.ps1 run --browser`, which invokes the locked Playwright CLI (`node node_modules/@playwright/test/cli.js test` in `e2e-web/`), followed by `gate --require …playwright` (skips never allowed).
  - Per-test session lifecycle is `boot` → test → `cleanup`. `status` is the readiness precondition, and `logs` is the failure-triage read.
  - The browser suite runs on every leg of `ci.yml`'s `test` job; the a11y verdict is judged on the ubuntu leg only. `browser-missing` is a failure.
- **Artifact:**
  - Playwright JSON report at the suite `artifact` path, with `attachments[]` holding scrubbed axe JSON, token-pair JSON, html-validate JSON, VSR phrase logs and the `a11y-violations` NDJSON.
  - `e2e-web/test-results/` is uploaded by `actions/upload-artifact` v7.0.1, SHA-pinned per zizmor.
  - Per-PR new-violation diff: the verdict is binary (`violations: []` on base and head), so every violation on a PR is new by construction and fails. No fingerprint baseline file exists.

### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from
the contract above. Listed for explicitness — route may reorder /
combine, setup-project may add stack-specific intermediate steps.

- **a11y-tooling-install:**
  - Add `@axe-core/playwright@4.13.0` to `e2e-web/package.json`. Tests declared only `@playwright/test@1.63.0` when the browser pipe landed (chunk 2026-09-27-browser-verdict-reachability); the axe pin is this phase's.
  - Add the devDependencies `colorjs.io@0.7.1`, `tabbable@6.5.0`, `@guidepup/virtual-screen-reader@0.33.0`, `html-validate@11.16.0` and `eslint-plugin-lit-a11y@5.1.1`, plus its `eslint` core peer, pinned exactly through the lockfile (Decisions Log → Resolved questions → ESLint core version).
  - Create `e2e-web/fixtures/a11y.ts` with `makeAxeBuilder` (tags `['wcag2a','wcag2aa','wcag21a','wcag21aa','wcag22aa']` + the five best-practice rules by id), the scrubber, and the violation-row writer.
  - Create the html-validate config declaring the `viola-*` elements.
- **focus-management-library-install:** none at runtime (native `:focus-visible`, `<details>`/`<summary>`, fragment links). The only install is the test-side oracle `tabbable@6.5.0`. The v1.x Raised-3 confirmation, if built, uses native `<dialog>.showModal()` with no library.
- **aria-component-library-install:** none. Native HTML plus WAI-ARIA APG patterns, rendered by Lit 3.3.3 into light DOM (`createRenderRoot(){ return this; }`).
- **contrast-verification-harness-setup:** the `a11y-tokens` test in `e2e-web/tests/bay-steady-state.spec.ts`, using colorjs.io over the Section 6 token-pair list, with the forced-colors and negative-placement DOM walk.
- **screen-reader-test-spec-setup:**
  - VSR route plus import helper in `e2e-web/fixtures/a11y.ts`, with the MutationObserver fallback helper.
  - `a11y/sr-pass/TEMPLATE.json` with one row per Section 4 path step for NVDA 2026.2 / VoiceOver / Orca.
- **a11y-ci-gate-wire:**
  - a11y specs are tagged `@a11y` + `@sc-*` inside the `playwright` suite, and `gate --require playwright` is unchanged.
  - `ci.yml` ubuntu leg gets a lint step before `run --browser`: `npx --prefix e2e-web eslint -c e2e-web/eslint.config.js -f json` over the Lit sources of the `viola-ui` crate (`crates/viola-ui/`, following the workspace `crates/<name>` layout; the exact source glob is the config's `files` entry). `eslint.config.js` lives in `e2e-web/` so `eslint-plugin-lit-a11y` resolves from `e2e-web/node_modules`. The step also runs `npx --prefix e2e-web html-validate --config e2e-web/.htmlvalidate.json --formatter json` on the `assets/index.html` that `viola-ui` embeds. Both write to `e2e-web/test-results/lint/`.
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

<!-- U35 · a11y-plan.md · ## 12. A11y Decisions Log · sha256 d8ad4f4408f7d2e6ef512c3161c6355073342877dcd03a9d34a9593e99396015 -->

## 12. A11y Decisions Log

_Records key decisions during plan generation + manual additions
between phase loops._

**Initial entry:**

`2026-09-24` — Initial a11y plan generated by `/andromeda-a11y`
- **Tier:** Standard (1) — justified by: about 11 assertable web-spa entities (Standard's 5–15 band); six must-be-accessible paths (above Minimal's 2–5); the tests tier is Comprehensive and already mandates `violations: []`, and the obs tier is Standard, so Standard is the lowest tier in the ±1 band; design binds AA contracts (SC 1.4.3 / 1.4.11 / 1.4.12 / 2.5.8 / 2.3.1, forced colours, reduced motion); no regime, WCAG target, disability signal, multi-language or cognitive trigger, so Comprehensive does not apply. The label is web-spa only.
- **Key decisions:**
  - **A11y testing tool:** `@axe-core/playwright` 4.13.0 (axe-core 4.13.x) — chosen because:
    - it is already declared in the tests' Playwright Test 1.63.0 driver;
    - its CDP `page.evaluate` injection runs under the enforced CSP and Trusted Types with no `bypassCSP`, provided no `locale` is passed;
    - it emits axe-core JSON;
    - it covers 24 SCs with 70 rules under `['wcag2a','wcag2aa','wcag21a','wcag21aa','wcag22aa']`. `wcag22aa` is required for `target-size`.
    - The axe gaps (SC 1.3.2, 1.4.10, 1.4.11, 1.4.13, 2.1.2, 2.3.1, 2.4.3, 2.4.6, 2.4.7, 2.4.11, 3.3.1, 3.3.3, 4.1.3) are closed by Playwright 1.63.0 aria/keyboard/emulation, colorjs.io 0.7.1, tabbable 6.5.0, @guidepup/virtual-screen-reader 0.33.0 and html-validate 11.16.0.
  - **Focus management library:** none at runtime (native `:focus-visible`, `<details>`/`<summary>`, fragment links under Lit 3.3.3 light DOM), with tabbable 6.5.0 as a test-side oracle only — chosen because v1 has one route, no modals and one tab sequence, so a trap library (`focus-trap` 8.2.2) has nothing to manage. Native `<dialog>.showModal()` covers the v1.x Raised-3 confirmation without a library or CSP impact.
  - **ARIA component library:** none (native HTML + WAI-ARIA APG patterns rendered by Lit 3.3.3 into light DOM) — chosen because every v1 pattern (table, disclosure, link) has a native element, and `log` / `status` need only a role. `@lion/ui` 0.21.1 adds nothing native elements lack and is shadow-DOM based. Material Web is in maintenance mode.
- **Further decisions (D-A11Y-01 … D-A11Y-20):**
  - **D-A11Y-01** Five `best-practice` axe rules (`region`, `landmark-one-main`, `landmark-banner-is-top-level`, `page-has-heading-one`, `heading-order`) are enabled by id and gating. They carry no SC tag, so their rows are a11y-owned SC 1.3.1 evidence only. SC 2.4.6 is never credited to axe: its evidence is html-validate `heading-level` / `empty-heading` plus the aria snapshot (Section 3 per-SC map). This is the Decisions Log entry for the partial `runOnly` configuration; no rule is disabled.
  - **D-A11Y-02** Landmarks and headings (a11y-owned per layout):
    - `<header>` via `<viola-atis>` gives banner;
    - `<main>` wraps the racks, tape and announcer;
    - `<h1>` is `VIOLA`, and `<h2>` are `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE`;
    - racks are native `<table>` + `<caption>`;
    - `viola-*` row hosts are role-less `display: contents` elements rendering native `<tr>`/`<td>`, which overrides design's `role="row"`-on-host (semantic HTML first);
    - ATIS label/value cells are native `<dl>`.
    - Accepted by the overseer (founder-delegated) on 2026-09-24. The override of design's `role="row"`-on-host is a design amendment the overseer applies to design-system.md after this run.
  - **D-A11Y-03** Tape `role="log"` carries explicit `aria-live="off"`, which resolves the implicit-polite conflict with design and layout.
  - **D-A11Y-04** Conformance label: WCAG 2.1 AA + SC 2.4.11 + SC 2.5.8, with the SC 1.4.10 < 760 CSS px exception. The other 2.2 AA SCs are absent-by-surface; AAA is not claimed.
  - **D-A11Y-05** Lighthouse 13.5.0, pa11y 10.0.0 / pa11y-ci 4.1.1, `@lhci/cli` 0.15.1, `playwright-lighthouse` 4.0.0 and the WAVE API are dropped: each needs a second automation client, is stale, or cannot reach loopback.
  - **D-A11Y-06** Announcer scope is the design's three messages, plus the 401 access strip (Overseer Direction 4), plus 503/404/405 rack strips that appear after first render. The last group is added under SC 4.1.3, because layout assigns announcement mechanics to a11y. There is no `alert`.
  - **D-A11Y-07** Initial focus stays on `<body>` with no autofocus. The skip link targets `#tape-end`, which is `tabindex="-1"` with visually hidden text `end of tape` (a11y-owned copy; design's zero-height geometry is unchanged).
  - **D-A11Y-08** SC 3.1.2: event and passthrough text has indeterminate language, so no `lang` is guessed on it.
  - **D-A11Y-09** Violation rows use `event:"a11y-violation"` and `process:"ui"`. They are harness-only rows under `e2e-web/test-results/a11y/`, never in `diagnostics/`, and are validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`. `a11y-violation` is not a product `event` enum value and has no `ObsEvent` variant (overseer fix pass 3, Z7). The additive fields are `service_name`, `version`, `os`, `surface`, `check_source`, `wcag_criterion`, `violation_type`, `severity`, `selector`, `remediation`, `ci_run_id`, `git_sha`.
  - **D-A11Y-10** colorjs.io 0.7.1 is chosen over `culori` 4.0.2 as the single token-pair checker, because it parses every CSS Color 4 computed value.
  - **D-A11Y-11** @guidepup/virtual-screen-reader 0.33.0 is the gating announcement proxy. Its Trusted Types safety is unverified at runtime, so the first spec asserts a clean CSP/TT console. If it fails, the fallback is a `page.addInitScript` MutationObserver recorder on `[role=status],[role=log],[aria-live]`.
  - **D-A11Y-12** `@guidepup/guidepup` 0.34.0 / `@guidepup/playwright` 0.19.1 (real NVDA/VoiceOver) are founder-local only. Adding them to CI needs a tests-harness change: the a11y verdict is judged on the ubuntu leg only (the `--browser` pipe itself runs on all three OSes since 2026-09-27), and Guidepup has no Orca support.
  - **D-A11Y-13** Inherited risk: portable-pty is pinned `=0.8.1` (2023-03-13) by arch/tests, while 0.9.0 is the maintained line. a11y inherits the driver and does not re-pin.
  - **D-A11Y-14** A lint stage is added on the ubuntu leg of `ci.yml` (eslint-plugin-lit-a11y 5.1.1, html-validate 11.16.0 CLI). It sits before `run --browser`, like clippy/zizmor, and adds no harness `suite`.
  - **D-A11Y-15** Section 1 is reproduced verbatim from a11y-scope.md, except that deferral clauses addressed to later synthesis steps are replaced by `[resolved: …]` pointers.
  - **D-A11Y-16** Section 8 Cognitive Accessibility is included, as required at Standard+. It is scoped honestly to v1 surfaces, because no cognitive trigger fired and there is no AAA escalation (SC 2.2.6, 3.1.5, 3.3.9 not claimed). It covers:
    - no time limits (SC 2.2.1, asserted via axe `meta-refresh`, the session-cookie `expires === -1` check and the absence of auto-dismissing notices);
    - plain error and recovery text (SC 3.3.1 / 3.3.3, via Playwright `toHaveText` / `toHaveAccessibleName`, the VSR phrase log and trycmd `hint:` cases);
    - consistency and predictability (SC 3.2.1–3.2.4, via aria snapshot, keyboard walk and trycmd caption parity);
    - the absence of inputs and auth forms (SC 3.3.7 / 3.3.8, via the `surface-absence` spec).
    - No readability scorer exists in a11y-research, so no reading-level metric is gated. Fixed copy is pinned by Playwright and trycmd instead.
  - **D-A11Y-17** Auto-follow must not scroll a focused `<summary>` out of the tape viewport, and tape trimming must never drop the focused line. This is an a11y requirement on the layout follow rule (SC 2.2.2, 2.4.3, 2.4.11), flagged to layout.
  - **D-A11Y-18** The `--rule-field`/`--surface-bay` pair (the `expired` box) is derived from the design text "the `expired` box uses the same ink on anthracite". The `--rb-strike`/`--surface-inset` pairing is derived from the design row it shares with `--rb-rule`. Both use existing token names only.
  - **D-A11Y-19** Focus colour naming: layout says `color-border-focus` and design says `--focus-ring`. design-system.md is the token authority, so this plan binds to `--focus-ring`. layout-templates.md is renamed to `--focus-ring` in the overseer's cross-plan fix pass.
  - **D-A11Y-20** The text-spacing override and the VSR bundle are served through same-origin `page.route` URLs (`/assets/__a11y/*`), so CSP `script-src` / `style-src 'self'` holds without `bypassCSP`.
- **Resolved questions (overseer, founder-delegated, 2026-09-24):**
  - **Reflow below 760 CSS px:** v1 keeps the documented SC 1.4.10 exception from D-A11Y-04. Data tables may use the two-dimensional exception; the ATIS, tape and strips get a layout when the phone view lands.
  - **`a11y-violation` event value:** not a product `event` value. It is a harness-only row validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json` (overseer fix pass 3, Z7; see D-A11Y-09). The shipped `ObsEvent` and the `schemas/diag-line.v1.json` event enum (19 values) carry no `a11y-violation`.
  - **401 access strip copy:** bound to design-system.md, which already specifies `UNAUTHORIZED  this page has no session for <host>` plus one instruction line. The announcement reads that visible text. It must not contain the token, URL or `.url` path.
  - **Unwrapped NAME ellipsis:** truncation is CSS-only. The full name stays in the DOM as the accessible name, with no `title` tooltip (SC 1.4.13 stays free of hover content).
  - **ESLint core version:** pinned exactly through the lockfile (dev tool, lockfile-pinnable) when the lint stage lands.
- **Deferred (out of scope for 0.1.0):**
  - The v1.x brakes (`--strip-h`, native `<button>`) and the phone view. When they land: re-evaluate the `target-size` trigger (SC 2.5.5), add Button / `<dialog>` keyboard contracts, and re-open SC 3.3.8 for the phone view's authentication.

**Subsequent entry format (for manual additions or re-runs):**

`YYYY-MM-DD` — short title of decision
- **Decision:** what was decided
- **Rationale:** why — reference a11y-scope / research / org constraint
- **Impact:** which sections affected; downstream skills affected
- **By:** `/andromeda-a11y` re-run / manual edit by the named editor

(Append new entries at the bottom; do not modify historical entries.)

`2026-09-24` — Audited Phase 0 deviation: design excerpt clause removal
- **Decision:** the design-system upstream excerpt's second attempt failed `[NO_RECOMMENDATIONS]` (4 extractor-added advice clauses: `aria-busy` on racks, "candidate alert" for the 401 strip, and two SC 1.4.12 "must not break / verify" clauses). The skill rule is halt; instead the orchestrator removed only those 4 clauses and the run continued.
- **Rationale:** overseer (founder-delegated) accepted it as an audited deviation; everything else in the excerpt stayed verbatim. None of the removed clauses carried design content, so no design fact was lost.
- **Impact:** upstream-context Section 3 only; audit trail in the run's `.scrub-design.log` and `.deviations.log`. No plan section depends on the removed text.
- **By:** `/andromeda-a11y` orchestrator, on overseer direction

`2026-09-24` — Overseer directions recorded for this run
- **Decision:** six founder-delegated overseer directions (axe inside the tests' Playwright driver; token names only, holding on the ubuntu CI DejaVu fallback; violation JSON aligned to obs; the announced 401 access strip after a `viola ui` restart instead of `TAPE stopped`; CLI/TUI keyboard and screen-reader discipline with no fake automation; dark only) were carried into upstream-context Section 7 → Overseer Directions and applied from Phase 1 on.
- **Rationale:** founder delegation to the overseer for this run.
- **Impact:** Sections 1–11 wherever a direction applies.
- **By:** `/andromeda-a11y` orchestrator, on overseer direction

`2026-09-24` — P3.5 review 1 (overseer, founder-delegated): tier, harness, paths and budgets accepted. D-A11Y-02/03/04/06/14 accepted, with D-A11Y-02 logged as a design amendment. D-A11Y-09 accepted by the D-21 route. D-A11Y-19 binds `--focus-ring`, and layouts are renamed in the fix pass. Open questions resolved (≤760 SC 1.4.10 exception kept; 401 copy bound to design-system.md; CSS-only ellipsis with no `title`; ESLint lockfile pin), and v1.x brakes and the phone view deferred.

`2026-09-24` — overseer fix pass 3, 2026-09-24 (cross-plan findings "a11y vs upstreams", founder-delegated; each item checked against the cited line first)
- **Decision:**
  - Z1 (design wins): only the `.rb` square is `aria-hidden`. The `<viola-readback>` host and its word stay exposed, so "read back" keeps its accessible name. Updated in the §4 catalog row, the §3 SC 1.1.1 map row, the §11 ARIA ban and a §1 `[resolved: …]` pointer.
  - Z3: `list` table rows also escape `\n` / `\t` (ratified T5); `wait` / `last` keep them. Updated in §4 P6 and a §1 pointer.
  - Z5: path ids become `a11y-p1` … `a11y-p5`, so `--grep path5` selects only the tests' scenario. a11y tests live in the tests' `<bay-layout-type>.spec.ts` specs with the tests' title form, and `a11y-tokens` / `surface-absence` are test ids in `bay-steady-state.spec.ts`. §2 Test tags, §3, and the ids everywhere they appear.
  - Z11: `unknown` (exit 14) is the one refusal without a hint (design cli). §8 Error recovery and §11 Cognitive.
  - Z12: bold on NAME is the third CLI SGR, beside amber `DIALOG` and dim stale rows. §4 P6 and §6 State colour.
  - Z13: `viola ui` prints design's two-line launch block exactly once. §4 P6 and a §1 pointer.
  - Z14: the `RB` pair reads `--rb-ink` / `--rb-fill` (design `data-rb="read"` rule), with `--ink-on-paper` noted as the same alias. §6 pair table and state colours, and a §1 pointer.
- **Rationale:** the overseer's audit "a11y vs upstreams" (2026-09-24 07:05); design and tests are upstream.
- **Impact:** §1 (pointers only, per D-A11Y-15), §2, §3, §4, §6, §8, §11. Not changed here: Z7 (the `a11y-violation` event moves out of the product enum into its own harness-side schema, applied in tests and obs by this pass). The §3 wording "new closed-enum value, accepted as a tests + obs enum amendment" and D-A11Y-09 are left for the overseer to reconcile, because Z7 was not routed to a11y.
- **By:** manual edit, overseer fix pass 3, 2026-09-24 (founder-delegated).

`2026-09-24` — overseer fix pass 3, 2026-09-24: `a11y-violation` is a harness-only row (Z7 leftover, founder-delegated)
- **Decision:** §3 Binding fields and D-A11Y-09 now describe `a11y-violation` as a harness-only row, validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`. It is not a product `event` enum value and has no `ObsEvent` variant.
- **Rationale:** this matches the test-plan and obs-plan Z7 changes from the same pass.
- **Impact:** §3 Binding fields and D-A11Y-09 only. Nothing else is amended.
- **By:** manual edit, overseer fix pass 3, 2026-09-24 (founder-delegated).
