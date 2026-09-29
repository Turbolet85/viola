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
