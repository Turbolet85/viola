## 3. Design System Excerpt

### Surfaces
- **web-spa** (desktop-browser single page, `viola ui`, loopback only) — a view-only strip-bay page built from Lit custom elements (`viola-*`) rendered into light DOM over semantic HTML, with a sticky ATIS header, a WRAPPED rack, an UNWRAPPED · READ-ONLY rack and a TAPE event log. Targets Edge/Chrome first, plus Chromium, Firefox and Safari on macOS/Linux, and is verified in a headless browser (CI renders on Linux). Testing reach: axe-core + Lighthouse + screen reader (NVDA / VoiceOver) + keyboard + forced-colors check.
- **cli** (terminal, `viola` binary) — flat verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `ui`, `run`). Output is static, fixed-column and ASCII only, with no TUI, prompt or pager. Human TTY output uses colour only as a second cue. `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` and `viola run` produce no colour, glyphs or cursor control. Testing reach: manual keyboard/screen-reader-in-terminal checks, no automated tools; assert output text and streams.

### Loading / Error / Empty State Patterns
- **Initial read (loading)** — page-replace text: each rack prints one ruled line `sessions: no reading yet` and the ATIS fields print `unknown`. Spinners, skeletons and "Loading…" are banned. a11y impact: the loading state is a plain text word, so content must be readable as text; design specifies no `aria-busy`.
- **Empty rack** — inline text `no wrapped sessions — start one with  viola run <name> -- claude`, with the command in the code style. a11y impact: static text, read in table/document order.
- **Empty tape** — inline text `TAPE live since <time> — no events yet`. a11y impact: static text inside the `role="log"` panel.
- **Missing reading** — the inline word `unknown` in the field; absent-by-contract fields print `n/a`. Fields are never blank. a11y impact: every cell has a text value, so no empty cells reach assistive technology.
- **Readback (send outcome)** — inline box plus word: `open` → `read back` / `unable · <reason> · <detail>` / `unconfirmable`. The change is instant. a11y impact: the box is `aria-hidden` and the word cell carries the name; refusals are announced through the polite status region.
- **Cocked strip (dialog pending)** — the strip is offset in place with a band and the word `DIALOG <kind>`, and keeps its slot. a11y impact: announced politely ("builder: dialog pending, permission"); `document.title` becomes `DIALOG <name> · viola` (`+N` when more strips are cocked).
- **Stale session** — the whole strip dims in place and LIVE prints `stale`. a11y impact: the word carries the state; it is not announced.
- **Tape stopped (connection lost)** — a boxed ATIS cell `TAPE stopped · viola ui not answering` plus a strip at the top of the tape. Strips keep their last values. a11y impact: announced through the polite status region.
- **Tape overflow / not following** — static lines `N new lines below` (an anchor to `#tape-end`) and `older lines trimmed from view: N — the full tape is events.ndjson`. Auto-follow runs only while the reader is at the bottom, with no smooth scroll. a11y impact: no focus theft; the anchor is keyboard reachable.
- **CLI waiting** — one static `waiting: <name>` line on stderr (TTY only); `verify` appends static `[NN/NN] … pass|fail` step lines. No spinner or redraw. a11y impact: linear output that works with screen readers.

### User-Facing Error Surfaces
- **Access strip, 401 `unauthorized`** (in-page, full-width strip in the rack position) — appears for: a missing or invalid session. Shows `UNAUTHORIZED …` plus one plain instruction line, and never shows the token, URL or path. a11y requirements: focus management not specified in design; ARIA role not specified; recovery affordance keyboard reachable: no (recovery is the terminal launch line or a restart; no in-page control or input).
- **Rack error strip, 503 `state-unreadable`** (in-page) — appears for: the viola home cannot be read. Shows `unable · state-unreadable  viola home could not be read`. a11y requirements: focus not specified; role not specified; recovery keyboard reachable: no (no control).
- **Rack error strip, 404 / 405** (in-page, defensive) — appears for: unexpected route or method. Shows `unable · not-found` / `unable · method-not-allowed`. a11y requirements: as for 503.
- **Host-not-allowed, 403** (dedicated browser page) — appears for: a bad Host header. The page cannot load; the browser shows raw Problem JSON. a11y requirements: no in-page surface.
- **Tape stopped** (in-page ATIS cell + tape-top strip) — appears for: the SSE stream closed. a11y requirements: role=status aria-live=polite announcement (design-specified); focus does not move; no retry control.
- **Readback refusal** (inline, in the tape send line and the transfer marker) — appears for: a send refused with a typed reason/detail. a11y requirements: role=status polite announcement ("send to builder unable, not-delivered, no-prompt-submitted"); focus does not move; the full text is reachable in the tape line's `<details>` body (Enter/Space).
- **CLI refusals and errors** (stderr) — appear for: typed refusals (`unable  <reason>  <detail>`, then one `hint:` line) and faults (`error: …`, fixed messages, no stack traces), with typed exit codes. Under `--json` a single JSON document goes to stdout with no hint. a11y requirements: text-only and linear; results go to stdout and messages to stderr; colour is never the only cue.

### A11y-Relevant Design Tokens

#### Color tokens (foreground / background pairs)
Components use only role aliases. The eight `--c-*` palette tokens (`--c-anthracite`, `--c-holder`, `--c-rail`, `--c-buff`, `--c-graphite`, `--c-lampoff`, `--c-amber`, `--c-departure`) are read only where aliases are assigned or reassigned.
- **--ink / --surface-bay** — context: primary text on page, tape, ATIS and rack gap; meets SC 1.4.3 4.5:1.
- **--ink / --surface-strip** — context: field values and callsign on a lit wrapped strip; meets 4.5:1.
- **--ink-dim / --surface-bay** — context: secondary text (column captions, timestamps, reading ages, `n/a`, `activity`/`harness` lines, stale strip ink); allowed only on anthracite. Fails AA 4.5:1 on `--surface-strip`, which is banned.
- **--ink-dim / --surface-inset** — context: DIALOG kind word on a cocked + stale strip.
- **--ink-on-paper / --rb-fill** — context: `RB` text inside a filled readback box; the only text on buff fill.
- **--attention / --surface-inset** — context: the `DIALOG` word of a cocked strip (inset cell on anthracite). Fails AA as text on `--surface-strip`; there it is only the non-text band (SC 1.4.11 3:1 passes).
- **--handoff / --surface-bay** — context: transfer-marker arrows/names, the marker's left tick, and names in `link`/`unlink` tape lines. Text is allowed only on anthracite; fails AA on `--surface-strip`, which is banned.
- **--rule-info / --surface-bay** — context: state-carrying borders (boxed `budget-paused`, nonzero `skipped`, `TAPE stopped`, error strips); non-text, SC 1.4.11.
- **--rb-rule / --surface-inset** — context: readback box outline (the state indicator); **--rb-strike** is the refused-state strike line; non-text, SC 1.4.11.
- **--rule-field / --surface-strip** — context: in-strip field-grid dividers (printed grid; the `expired` box uses the same ink on anthracite).
- **--rule-deco / --surface-bay** — context: decorative rack rails, strip outer edge, dashed unwrapped outline, stale field grid. Below 3:1; never text and never the sole carrier of information.
- **--focus-ring / --surface-bay, --surface-strip** — context: focus indicator on both surfaces.

#### Focus ring tokens
- **--focus-ring:** focus indicator color token (buff alias; passes on both surfaces).
- **--focus-w:** focus indicator width token.
- **--focus-offset:** focus indicator offset token.
- **--radius:** corner token applied to the focus indicator (square everywhere).
- Applied globally through `:focus-visible` in the base layer (outline composition specified in design). The ring appears instantly. There is also a forced-colors mode fallback: borders survive, the strike becomes a dashed outline plus the word `unable`, and the cock band maps to the system highlight colour.

#### Target size tokens
- No dedicated target-size token. v1 has no buttons; the focusable elements are the skip link, the tape `<summary>` rows and the `new lines below` anchor.
- **--line-h:** tape line / transfer marker row height token (governs `<summary>` hit height; relevant to SC 2.5.8 24×24).
- **--strip-h:** strip height token; also the minimum-height token for the reserved v1.x brake buttons (`I HAVE CONTROL`, `UNLINK`), relevant to SC 2.5.8 / 2.5.5 44×44.
- **--rb-size:** readback box size token (non-interactive, `aria-hidden`).

#### Motion / transition tokens
- **--cock-dur:** cocked-strip offset transition duration token (plays once, only into the pending state; the return is instant); reduced-motion override defined in design.
- **--cock-ease:** cocked-strip easing curve token.
- **--cock-offset:** cocked-strip position offset token (the offset remains under reduced motion because it is state).
- **--fade-dur:** entrance fade duration token for newly arrived tape lines and transfer markers (live arrivals only); reduced-motion override defined in design.
- **--fade-ease:** entrance fade easing token.
- Reduced-motion override targets `--cock-dur` and `--fade-dur`. Every other state change is instant. Banned: spinners, pulses, blinking, shimmer, looped/staggered/scroll-driven animation, and hover colour transitions (SC 2.3.1 has no flash sources).

#### State color tokens (error / warning / success / info)
No generic hue tokens exist. State is carried by ink, fill, rule, strike and a printed word, never by hue alone.
- **--rb-fill / --ink-on-paper:** success (read back); not-color-alone supplement: `RB` text plus the word `read back`.
- **--rb-strike / --rule-info:** error (refusal/HTTP problem); supplement: strike shape plus the word `unable · <reason> · <detail>`.
- **--rule-field (box) / --ink:** warning (`expired`, `unverified-cli`, `unconfirmable`); supplement: printed word.
- **--ink-dim:** info (activity, harness, zero `skipped`, `TAPE connecting`); supplement: printed word and position.
- **--attention:** attention (dialog pending only); supplement: the word `DIALOG <kind>` plus the position offset.
- **--ink swap (stale):** stale liveness; supplement: the word `stale` in LIVE.
- CLI: amber SGR for the `DIALOG` word and dim SGR for stale rows, always beside the word. No colour appears in machine or plain modes.

#### Typography tokens (readability)
- **--font-label:** label/heading/display font stack token (per-OS fallback stack defined in design; the Linux fallback is the CI render).
- **--font-field:** monospace field/tape/code font token (no ligatures, tabular figures).
- **--fs-display / --fs-callsign / --fs-field / --fs-code / --fs-label:** font-size tokens for display, session-name, body/data, expanded code body and label/caption roles.
- **--fw-regular / --fw-strong:** weight tokens (strong for callsign, `human`, `DIALOG`, `RB`).
- **--track-display / --track-rack / --track-label / --track-callsign:** letter-spacing tokens (relevant to SC 1.4.12 text-spacing user override).
- **--stretch-label:** semi-condensed width token for labels.
- **--lh-display / --lh-label / --line-h:** line-height tokens (fixed row heights; relevant to SC 1.4.12 text-spacing user override).
- **--strip-h:** fixed strip row height token (callsign line).
- **--strip-cols / --strip-cols-wrapped / --strip-rows-wrapped / --strip-areas-wrapped:** character-width strip track tokens. Strips wrap to two lines at the narrow breakpoint, with no horizontal scroll at supported widths. The unwrapped NAME truncates with an ellipsis.
- **--tape-cols:** character-width tape track token (the INSTANCE and TEXT cells truncate with an ellipsis; the full text is in the `<details>` body).
- Session names are never text-transformed. Uppercase is used only for labels and headings.

### ARIA-Relevant Component Patterns
- **Rack (session table)** — ARIA role: `table` containing `row` (`<viola-session-row>`), `columnheader` (one caption row per rack) and `cell`. Key states/props: state is in `data-*` attributes and in printed words, not aria-*. Keyboard contract: none (rows are not focusable in v1).
- **Tower tape (`<viola-event-feed>`)** — ARIA role: `log`. Each line is a native `<details>`/`<summary>` disclosure (disclosure marker hidden). Key states/props: native open/closed (aria-expanded equivalent); never an aria-live region as a whole. Keyboard contract: Tab moves between summaries; Enter/Space expands.
- **Status announcer** — ARIA role: `status`, `aria-live="polite"`, visually hidden. Announces only strips turning cocked, readback refusals and `TAPE stopped`. Keyboard contract: n/a.
- **Readback box (`<viola-readback>`)** — the box is `aria-hidden="true"`; the accessible name comes from the adjacent word cell (the line reads "send to builder, read back"). Keyboard contract: n/a.
- **Skip link** — ARIA role: link, `skip to tape`, the first tab stop, visible on focus. Keyboard contract: Enter.
- **`N new lines below` anchor** — ARIA role: link to `#tape-end` (jump is instant). Keyboard contract: Enter.
- **Links (general)** — ARIA role: link; underline on hover/focus only, with no colour change. No hover-only information.
- **Transfer marker (`<viola-transfer>`)** — plain text line in the rack flow (no role specified); contains a readback word cell.
- **ATIS header (`<viola-atis>`)** — sticky header with plain text cells (no role specified). Cells wrap at cell boundaries and never truncate.
- **Document title** — updates to `DIALOG <name> · viola` (`+N`) while any strip is cocked; otherwise `viola`.
- **Reserved v1.x brake controls** (not built in v1) — ARIA role: native `button`, with the standard focus ring and no pressed fill. Keyboard contract: Enter/Space.
- **Global keyboard** — no command palette, no shortcuts, no credential inputs; there is a single tab sequence (skip link → tape summaries → new-lines anchor).
