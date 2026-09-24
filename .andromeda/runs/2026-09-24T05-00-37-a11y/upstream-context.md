## 1. Architecture Excerpt

### Stack (a11y reach)
- **axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, `text/event-stream` excluded)**: serves the view-only local web GUI on `127.0.0.1:47319`: `/` (`text/html`), `/assets/*` (static files embedded in the binary), JSON `/api/*` routes, and RFC 9457 Problem Details errors. This is the only web surface, so axe-core, Lighthouse and pa11y can reach it at `http://127.0.0.1:<port>/`. The Host allowlist accepts only `127.0.0.1:<port>` and `localhost:<port>`, so a11y tooling must use one of those hosts.
- **SSE through axum `Sse::keep_alive`, fed by notify 8.2.0**: drives the live GUI event feed `GET /api/events`. It is one-way, sends a keep-alive comment every 15 s, and resumes on reconnect via `Last-Event-ID`. On first load the page reads state from `/api/sessions` and `/api/links`. Because the page updates dynamically, live-region (`aria-live`) patterns and announcement testing apply. The arch does not use WebSocket or polling.
- **Frontend framework: none chosen in arch.** It is "Deferred to specialists … the frontend framework (design specialist)". The UI framework, and so the ARIA pattern source, comes from design, not arch.
- **clap 4.6.7 (derive)**: the CLI parser for subcommands `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`. CLI surface, so only manual keyboard and terminal-output discipline apply. Output is human text by default, `--json` gives machine output, and each refusal has a typed exit code.
- **portable-pty `=0.8.1` + vt100 0.16.2**: hosts the unmodified `claude` TUI in ConPTY (Windows) or openpty (Unix). vt100 is used only for the pre-send readiness gate and modal detection, "never used to read content". The terminal carries the child's own screen: `run` writes nothing to the terminal except the child's output. Terminal/TUI surface, so only manual keyboard and screen-reader checks apply. viola does not own the rendered TUI content.
- **Mobile framework: N/A.** "The phone view is a later version: the same web page behind authentication." The future mobile reach is the same web page, so it would be covered by web a11y tooling.
- **Test framework / E2E driver: not in the arch stack.** It is "Deferred … the test framework, fake-agent harness and `viola verify` probe design (tests)". The arch declares no Playwright, Cypress or similar harness for a11y to reuse.

### Surfaces
> - **Product type.** Hybrid local developer tool made of three parts:
>   - a native cross-platform CLI binary;
>   - a Claude Code plugin (hooks + stdio MCP server) that calls that binary;
>   - a minimal view-only local web GUI served by the same binary.

Deferred surfaces:
- "[Mobile] N/A in v1. Rationale: the phone view is the same web page behind authentication, in a later version."
- "A phone or remote view comes later, behind authentication."
- v1.x adds the GUI brake: "the reserved `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink` routes".

Platforms:
- "Windows is the live-supported target. The macOS and Linux paths build and pass CI on every commit against the fake agent."

### Project Intent Summary
- **Core functionality:** "Lets one interactive Claude Code session drive another on the user's own subscription."
- **Target users:** The arch has no "Growth model" user sentence; it describes architecture only: "Modular monolith: one binary with compiler-enforced internal crates … and no daemon." Users are named elsewhere:
  - "The founder runs v1 on their own subscription, with no accounts, no hosting and the GUI on 127.0.0.1."
  - "the later public version (individual Claude Code subscribers) must not need a rewrite."
  - "v1 is personal (Windows live, macOS and Linux CI-tested)."
  - The arch contains no a11y-priority user signals (no low-vision, elderly, cognitive-disability, assistive-tech or language-proficiency hints).
- **Critical paths hint:**
  1. GUI view: "The page shows each session row (wrapped, or read-only unwrapped), its liveness (`live` · `stale`), idle/busy status, wheel holder and any pending dialog."
  2. GUI view: "It also shows the budget reading and its age, the link set, the `skipped` counts, and a live event feed over SSE."
  3. GUI scope: "In v1 it is view-only, and no control accepts input."
  4. Human takeover: "Any human editing key since the last turn boundary moves the wheel to the human, and `send` is refused with `human-typing`. Focus, mouse and resize sequences do not count."
  5. Manual pause and release: "CLI `viola pause` moves the wheel to the human without a keystroke … The wheel returns only through CLI `viola release`."
  6. Dialog fallback to the human: "While the wheel is `human`, the wrapper answers `hook.dialog` with `null` at once, so the dialog renders for the human."
  7. CLI input and output: "Human text by default, `--json` for agents, a typed exit code per refusal. Prompt text comes from stdin or `--file`, never from a leading-slash argument."

### CI/CD Platform
- **Platform:** GitHub Actions
- **Pipeline note:** One workflow, `ci.yml`, runs on push and PR across a native-runner matrix `[windows-2025, macos-latest, ubuntu-latest]` (`dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2.9.2`). Jobs per OS: fmt → clippy `-D warnings` → sync-crate `cargo check` → `cargo deny` (ubuntu only) → workspace tests against the fake agent replaying `fixtures/claude/*` → `cargo build --release`. There is no deploy stage in v1, and the real `claude` CLI never runs in CI.

### A11y-Relevant Conventions
(No a11y-specific conventions in arch — Phase 3 will derive defaults from design + WCAG.)

The arch states no semantic HTML, i18n/RTL/`lang` or focus-management conventions. The following terminal and CLI output conventions affect assistive-tech reach:
- **CLI output mode:** human-readable text by default, `--json` for agents, and a typed exit code per refusal (`0`, `1`, `2`, `10`–`14`, `20`, `21`).
- **Terminal output discipline:** "While the child runs, `run` writes nothing to the terminal except the child's own output". `run`'s diagnostics go to the instance `diagnostics/` directory. `ui` and short-lived CLI verbs may use stderr.
- **Human input priority:** "Human keystrokes are never blocked, refused or delayed past the current atomic paste". Focus, mouse and resize sequences do not count as human editing input.
- **GUI error format:** RFC 9457 Problem Details (`application/problem+json`) with a `type` URN and a human `title`/`detail` (403 host-not-allowed, 404, 405, 503 state-unreadable). `/ready` returns 503 with its own readiness body.

## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal (stated verbatim as "`Minimal (0), with targeted elevations for the local privilege boundary`")
- **Justification:** "viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data (Sections 1, 3, 4, 5), which rules out Standard's user-account and HTTPS concerns and Hardened's compliance drivers."
  - The elevations cover the local boundary only: IPC endpoint access control, bracketed-paste breakout in `send.text`, GUI output encoding, GUI cross-user and cross-origin readability, v1.x per-launch brake auth, and integrity of `~/.viola/`. They do not raise the tier to Standard.
  - Context: viola is a local CLI binary with a local IPC channel, a stdio MCP server, a Claude Code hook contract and a loopback-only, view-only web GUI (`viola ui` on `127.0.0.1:47319`, crate `viola-ui`). It has no accounts, no login forms, no password reset, no MFA and no captcha. The plan marks account-flow bans N/A.

### Anti-Patterns Rejected (a11y-relevant)
- **Rendering event content as HTML (`innerHTML`, `v-html`, `dangerouslySetInnerHTML`, or any Markdown-to-HTML renderer)**: rejected because the GUI renders untrusted upstream text (prompts, `last_assistant_message`, plan text, tool `input`, question text, unwrapped session names) and no HTML sanitizer was researched. Every event field must be rendered with `textContent` or the framework's text interpolation, and assistant Markdown is never rendered to HTML. A11y impact: assistant output reaches assistive technology as plain text, not as semantic headings, lists or links. Any structure or landmarks around conversation content must come from the GUI's own markup, not from the rendered content.
- **Inline `<script>`, inline event handlers and `eval`**: rejected under the enforced CSP: `default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'`. All JS is served from embedded `/assets/*`. A11y impact:
  - Keyboard and ARIA behaviour must live in external JS, with no inline handler attributes.
  - Inline `<style>` blocks and `style` attributes in markup are blocked (`style-src 'self'`).
  - Images and icons must be same-origin assets.
  - `font-src` falls back to `default-src 'none'`.
  - Third-party or CDN scripts are blocked, including runtime a11y overlays and injected checkers in the served page.
  - DOM writes must comply with Trusted Types.
- **Cookie without `HttpOnly`/`SameSite=Strict`, or leaving the token in the address bar after the `?t=` exchange**: rejected. Access goes through a one-time launch URL `http://127.0.0.1:<port>/?t=<token>`, which is printed to stderr and written to a 0600 `ui/<port>.url` file. The exchange sets a browser-session cookie (no `Max-Age`) and redirects 303 to `/`. A11y impact:
  - There is no login form. The only auth UI is the launch URL.
  - A missing or invalid cookie returns 401 `urn:viola:problem:unauthorized` with a fixed `detail`. No route re-issues the token, and error text must never contain the token or the `.url` path. The only recovery is to reuse the stderr line or the `.url` file, or to restart `viola ui`.
  - `/` loads ungated, while `/api/*` and SSE `/api/events` are gated. The page can therefore load and then hit a 401 on its data calls. That unauthorized state and its recovery instructions need to be perceivable.
  - The cookie has no timed expiry, so auth brings no session timeout.
- **Serving `/api/*` or SSE without the cookie check, and state-changing routes without cookie + `Sec-Fetch-Site`/`Origin` check**: rejected. v1 is GET-only (405 for other methods, `form-action 'none'`). The v1.x brake routes (`POST /api/sessions/{name}/pause`, `/unlink`) return 403 `urn:viola:problem:cross-origin-forbidden` on a cross-origin request. A11y impact: v1 has no forms or state-changing controls. The v1.x pause/unlink controls will have to handle 401/403 Problem Details error states (RFC 9457, fixed `detail` strings).
- **Printing `wait`/`last` human-mode output, or `list` rows from `claude agents --json`, to a terminal without escaping C0/C1 controls**: rejected (terminal-injection class). Controls are escaped except `\n` and `\t`. A11y impact: CLI output read by terminal screen readers can contain visible escape sequences where upstream text held control characters.
- **Writing `send.text` or answer free text containing C0 (other than LF/CR/TAB), DEL or C1 controls into the PTY**: rejected with refusal `not-delivered`, detail `control-character`, with no silent stripping. LF, CR and TAB must never be refused. A11y impact: multi-line input is allowed. Automation gets the refusal code, and security refusals never block the human (`NEVER let a security refusal block the human`).

### A11y Compliance Triggers
(No a11y compliance triggers in security plan — Phase 1 will derive a11y tier from creator brief + project intent + surface count.)
- The security plan states "Compliance triggers: None: no compliance-regulated data detected". It names no accessibility regime (no Section 508, ADA, EU Accessibility Act, EN 301 549, AODA or JIS X 8341). It also says the v1.x public distribution adds no compliance trigger, because all data stays local on each user's machine.

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

## 4. Layout Templates Excerpt

### Layout Types per Surface
- **web-spa:** bay-steady-state (primary-width band), bay-narrow (mid-width band), bay-first-reading/empty, bay-degraded, tape-line-expanded (in-place `<details>` disclosure, not a route). One route (`/`), so these are states of a single page, not separate routes. The four fixed regions, in this order: ATIS header (`<viola-atis>`), WRAPPED rack (`<viola-session-row>` strips + `<viola-transfer>` markers), UNWRAPPED · READ-ONLY rack, TAPE (`<viola-event-feed>`). Also present: state strips (first reading / empty / 503 / tape stopped / 401) and the tape end zone (`N new lines below` anchor, `#tape-end`). None of these exist: nav chrome, tabs, sidebar, breadcrumbs, search, hero, page footer, modals or dialogs. The below-mid-width band is reserved for a later phone view and has no v1 layout.
- **cli:** `viola list` (board: human TTY / piped-NO_COLOR-TERM=dumb / `--json`), `viola send` (readback mirror), `viola wait` / `viola last`, wheel/handoff/dialog verbs (`pause`, `release`, `link`, `unlink`, `answer`), `viola verify` (step counter), `viola --help` (grouped verb table), `viola run` (passthrough, prints nothing), `viola ui` (one launch line to stderr; content deferred).

### Error Boundary Placement
- **web-spa bay-degraded (503 `unable · state-unreadable  viola home could not be read`):** error boundary placement = section-level (a full-width error strip in the rack position it affects; strips keep their last values, nothing greys out). a11y impact: the layout gives no focus move and no role=alert. Announcement mechanics are owned by a11y.
- **web-spa bay-degraded (`TAPE stopped · viola ui not answering`):** error boundary placement = section-level (a strip at the top of the tape region, paired with a boxed `TAPE stopped` cell in the ATIS header; the last received lines stay). a11y impact: included in the polite-announcement scope ("polite announcements limited to a strip turning cocked, a readback refusal and `TAPE stopped`"). No focus move is specified.
- **web-spa 401 access strip:** error boundary placement = section-level (same rack-position anatomy). a11y impact: copy and recovery path are deferred to implementation with the security plan.
- **web-spa readback refusal (`send-refused`):** error boundary placement = per-component (a `/` strike in `<viola-readback>` on the tape send line and on the outbound transfer marker, with `unable · <reason> · <detail>` in the adjacent word cell). a11y impact: polite announcement of the refusal (e.g., "send to builder unable, not-delivered, input-not-ready"). The full refusal text is in the expanded `<details>` body when the word cell is truncated.
- **web-spa state strips, general rule:** words only. No "Loading…", no spinner, no skeleton, no toast and no auto-dismissing notice. Waiting states are printed words (`unknown`, `open`, `TAPE connecting`, `sessions: no reading yet`).
- **cli refusals:** error boundary placement = per-command output line: `unable  <name>  <reason>  <detail>` on stderr, followed by a `hint:` line as the last stderr line, with typed exit codes (10/11/12/13/14/20/21/1). Results go to stdout and errors to stderr, never mixed. `--json` puts one document on stdout with no hint text. Colour is never used for errors (`unable` / `fail` are never red).

### Focus Management Anchors
- **web-spa bay (all states):** Skip links: `skip to tape` is the first tab stop. It is hidden until focused, then drawn over the header's inline start with the focus ring. Its target is the tape region / `#tape-end`.
- **web-spa bay (all states):** Tab order after the skip link: every tape `<summary>` from oldest to newest, then the `N new lines below` anchor. Strips, captions and transfer markers are not focusable (v1 has no controls).
- **web-spa bay (all states):** Focus indicator: a `--focus-w` outline in `color-border-focus`, offset `--focus-offset`, appearing instantly on the skip link, the summaries and the new-lines anchor.
- **web-spa bay (all states):** Initial focus on layout mount: not specified.
- **web-spa bay (all states):** Route-change focus: N/A. There is one route and no client-side routes or history entries. The only in-page fragments are `#tape-end` and the skip-link target.
- **web-spa bay (all states):** Modal focus traps: N/A. The layout has no modals or dialogs, and no control slot is reserved in v1 (brake controls and Raised-3 confirmation are deferred).
- **web-spa tape end zone:** `#tape-end` is a zero-height anchor after the newest line, and the target of the skip link, the follow rule and the new-lines jump. Jumps are instant. The `N new lines below` anchor is link-styled and pinned as the tape region's last line. It is present only while the reader is scrolled up, its count updates as a text swap, and it underlines on hover/focus.
- **web-spa tape:** the tape is the page's only bounded scroll region. The follow rule auto-follows the bottom only while the reader is already at the bottom. It is capped at 2000 DOM lines, and past the cap a static trim notice is the first line.
- **web-spa tape-line-expanded:** Focus restoration / disclosure: native `<details>` / `<summary>` expands in place. The native disclosure marker is hidden and nothing replaces it. No focus movement is specified.
- **web-spa bay (content shifts):** the header gains a line when `expired`, `TAPE stopped` or a nonzero `skipped` box appears, and content below moves down one line instantly. Transfer-marker link/unlink also shifts strips below. This is the logged deviation: order is kept, pixel position is not.
- **web-spa out-of-view attention:** `document.title` becomes `DIALOG <name> · viola` (plus `+N`) when a strip is cocked. Polite announcements are limited to: a strip turning cocked, a readback refusal, and `TAPE stopped`. The whole tape is never announced. The mechanics are owned by a11y.
- **cli:** N/A for focus. There is no interactive prompt, no TUI mode, no pager and no in-place redraw. `viola run` hands the terminal to the wrapped claude TUI until it exits.

### Heading Hierarchy Anchors
- **web-spa bay:** No h1/h2/h3 levels are assigned. The layout gives only typography roles:
  - `VIOLA` in the ATIS header uses the Display role (no logo, no link).
  - The rack separator labels `WRAPPED`, `UNWRAPPED · READ-ONLY` and `TAPE` use the Heading role (uppercase) and are the page's only wayfinding. `UTC` sits beside `TAPE` in the Label role.
  - The caption row (`NAME LIVE STATUS WHEEL DIALOG CLI`) uses the Label role, sits outside the strips on the same grid as the strips, and is always drawn.
- **web-spa bay landmark roles:** none assigned by the layout. The layout does specify:
  - `<viola-atis>` is a sticky header region.
  - Racks are single-column lists of fixed-field grid rows (same six fields per row, stable name order; not a card grid).
  - The tape is a list of `<details>` lines.
  - There is no navigation and no page footer (contentinfo).
  - The "semantic roles of racks, strips and the tape are owned by a11y".
- **cli `viola list`:** a static BAY context header line, then uppercase column captions (Label), then rows, then the `-- UNWRAPPED - READ-ONLY --` separator (Heading-equivalent, plain). The WRAPPED rack has no separator, because it is first and the caption row heads it. `--help` groups verbs under board / traffic / wheel / handoff / setup labels. Landmark roles: N/A (terminal output).

## 5. Test Plan Excerpt

### Tests Tier
- **Tier:** Comprehensive
- **Justification:** "Five drivers beyond the Standard baseline push the tier to Comprehensive: the creator brief requires multi-platform CI (Founder Direction 4, D3); crash-safe self-healing state needs chaos tests; hook deadlines and SSE keep-alive need performance-budget tests; multi-version CLI fixtures need compat tests; the founder mandates mutation testing from chunk 1." Compliance verification is the one Comprehensive component marked not applicable: "No compliance triggers".

### Test Harness Contract Summary
- **5-command names:** `boot`, `run`, `status`, `cleanup`, `logs`. They are invoked as `scripts/agent-run.sh <command>` (POSIX) or `scripts/agent-run.ps1 <command>` (PowerShell). Both are thin shims over `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]`. Every command exits 0 on success, 1 on failure and 2 on a usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}`.
  - `run` suite selectors: `--unit|--integration|--e2e|--browser|--mutants|--coverage|--perf|--fuzz-replay|--all`.
  - The `--browser` step is `npx --prefix e2e-web playwright test`. It runs on Linux only. On Windows and macOS, `--browser` exits 2 with `reason:"browser-linux-only"`.
  - A missing Chromium is a failure (`reason:"browser-missing"`), never a skip.
  - `run` output: `{"v":1,"cmd":"run","ok":<bool>,"suites":[{"suite":"nextest-unit"|"nextest-integration"|"doctest"|"nextest-e2e"|"playwright"|"mutants"|"coverage"|"perf"|"fuzz-replay","passed":n,"failed":n,"skipped":n,"survived":n,"artifact":"<path>"}]}`. Failing test names are listed in `suites[].failures[]`.
  - The `run` / `gate` `suite` enum is closed. A new value needs a Decisions Log entry.
  - `gate --require <suite>[,…]` is an internal harness subcommand. It is not one of the 5 commands. Its output is `{"v":1,"cmd":"gate","ok":<bool>,"breaches":[{"gate":"suite-missing"|"suite-failed"|"suite-skipped"|"coverage"|"mutants"|"perf"|"artifact-missing","suite":…,"detail":…}]}`. Skips are never allowed for `playwright`.
  - Sessions: each Playwright test boots its own session, `boot --session pw-<spec>-<test id>-<workerIndex>`, and tears it down with `cleanup` using the same id. No session is shared across tests.
- **Status JSON shape:**
```json
{
  "v": 1, "cmd": "status", "ok": true,
  "state": "ready | degraded | down",
  "pid": "<viola ui pid from /api/info, or null without --ui>",
  "uptime_ms": "<now - /api/info.started_at, or null>",
  "last_error": "null | <harness code, e.g. ready-503, list-exit-21, sessions-mismatch>",
  "list": "<viola list --json document>",
  "ui": { "ready": "<GET /ready body>", "sessions": "<GET /api/sessions body>" },
  "api_sessions_equal_list": true
}
```
  - `state` is `ready` when every booted instance has `liveness:"live"` and `/ready` is 200.
  - `state` is `degraded` when any item is `stale`, `/ready` is 503, or `api_sessions_equal_list` is false.
  - `state` is `down` when `list` exits non-zero.
  - Exit 0 only for `ready`. Exit 1 for `degraded` / `down`, and the full document is still printed.
- **E2E driver(s):**
  - web-spa (`viola ui` "strip bay"): **Playwright Test 1.63.0**, headless Chromium, JSON and JUnit reporters, with **@axe-core/playwright 4.13.0** already declared. It runs on ubuntu only.
    - There is no `webServer`. Harness `boot` starts `viola ui`, and the test navigates to `/?t=<token>` read from `ui/<port>.url`.
    - Not used: `bypassCSP`, `toHaveScreenshot`, codegen and UI mode.
    - Tests query by role: `table` (racks), `row` (strips), `cell`, `columnheader` (the captions NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), `log` (tape) and `status` (live region).
    - Tests also assert `data-*` attributes and `document.title` (`DIALOG <name> · viola` / `viola`). CSS classes and xpath are never used.
  - Where the test plan places axe: "`AxeBuilder({page}).analyze()` runs after each state strip renders (empty, 503, cocked), with `violations` required to be `[]`. The test plan owns only where axe runs and the verdict... Rule tags, disabled rules and the conformance target come from the a11y plan's configuration, and specs must not add their own `withTags` / `disableRules`. Until the a11y plan names a configuration, axe's default rule set applies unchanged."
  - cli (verbs + `--json`): assert_cmd 2.2.2 + predicates 3.1.4, and trycmd 1.2.1 for human layouts. TTY rendering (bold NAME, amber `DIALOG`, dim `stale`) is checked through the portable-pty outer PTY by asserting on SGR sequences only. There must be no SGR output under non-TTY, `NO_COLOR` or `TERM=dumb`.
  - cli (`viola hook` / `hook statusline`): assert_cmd 2.2.2 with an `Instant` deadline.
  - tui (`viola run` passthrough): portable-pty `=0.8.1` outer PTY. Tests never parse the rendered child screen for content.
  - ipc-internal (wrapper channel): viola-channel sync client + interprocess 2.4.4 + jsonschema 0.57.0.
  - ipc-internal (MCP stdio): rmcp 3.4.1 client + jsonschema.
  - api-service (GUI HTTP + SSE): reqwest 0.13.5 + curl + eventsource-client 0.18.0.
  - library-only: cargo-nextest 0.9.146 + `cargo test --doc`.

### Critical Paths (must-be-accessible)
- **Path 1, `run` start sequence:** "Collision check, pinned copy and plugin folder, version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn." Surfaces: tui, ipc-internal, cli (`list --json`), api-service.
- **Path 2, confirmed `send` with CL-1 events:** "Driver `send` is accepted, then `send-issued (cursor, from)` is logged, one bracketed paste + Enter is written, and the matching `prompt-submitted{origin:"driver"}` confirms it." Surfaces: cli, ipc-internal (MCP), tui, api-service (SSE), web-spa.
  - Web signal: the tape `viola-readback` goes `data-rb="open"` → `"read"`, or to `"refused"` on failure.
- **Path 3, `wait` / `last` event-driven readback:** "`wait` wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end`, returns at once if the event is already logged at or after `after`, and times out with `{timed_out:true}`." Surfaces: cli, ipc-internal, api-service, web-spa.
  - `/api/sessions` shows `status` changing busy → idle.
- **Path 4, dialog → `answer` (question / permission / plan):** "The hook raises `hook.dialog`, exactly one dialog is pending per instance, and the driver answers by `dialog_id`." Surfaces: cli, ipc-internal, api-service, web-spa.
  - Web strip: `data-dialog="pending"`, `DIALOG <kind>`, and `document.title` = `DIALOG <name> · viola` while pending. These revert after the answer.
- **Path 5, human takes the wheel, automation is refused, `release` returns it:** "A human editing key in the `run` terminal since the last turn boundary moves the wheel to the human." Surfaces: tui, cli, ipc-internal, web-spa.
- **Path 6, budget governor:** "`viola hook statusline` writes `rate_limits` to `budget.json` and passes the user's statusline stdout through unchanged." Surfaces: cli, ipc-internal, api-service, web-spa.
  - The ATIS header cell shows `budget-paused`.
- **Path 7, unverified CLI version gate:** "On a version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers, and drivers get `unverified-cli`." Surfaces: tui, cli, ipc-internal, api-service.
- **Expansion scenarios (E1–E5, beyond the 7):**
  - E1: R5, unwrapped session hooks are a silent no-op.
  - E2: R8, the `CLAUDE*` env strip.
  - E3: `link` / `unlink` transfer markers (`/api/links`, `<viola-transfer>` readback).
  - E4: SSE `Last-Event-ID` resume.
  - E5: snapshot corruption → replay recovery.
  - Browser-owned tests default to `bay-steady-state.spec.ts` (Paths 3, 4, 5, 6, E1, E3). Each test title contains its scenario id, for example `path2`.

### Coverage Triggers Summary
- **Security vectors 1–9: IPC, paste injection, loopback GUI, hook stdin, MCP, CLI/env, filesystem, child spawn, supply chain** (security-vector-coverage). A11y implication: event text containing HTML must render as text in the headless browser. A11y checks must run under the enforced CSP and Trusted Types with no `bypassCSP`, and the console must stay free of CSP / Trusted Types violations. Human-mode CLI output escapes C0/C1 characters.
- **Error sanitization and secret logging** (security-vector-coverage). A11y implication: a11y violation reports and logs must not contain the token, `Cookie`, `?t=`, absolute paths or stripped `CLAUDE*` values.
- **Property / fuzz on 7 parser surfaces** (property-test). A11y implication: none directly. These cover parsers, not the UI.
- **Crash-safe state: torn lines, snapshot corruption, no-EOF child, endpoint vanishing, stale heartbeat** (chaos). A11y implication: fault and degraded states must stay perceivable and operable. Examples are the 503 state strip, `data-liveness="stale"` and `instance-unreachable` surfacing. Focus and live-region announcements must persist through these state transitions.
- **Hook deadlines, SSE keep-alive 15 s, heartbeat >5 s** (performance-budget). A11y implication: SSE-driven live-region (`role="status"`, `role="log"`) updates must announce state changes. Any a11y timing claims must key on logged events, not sleeps.
- **Cross-surface coordination** (cross-surface). The same captions and slot order (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) appear in CLI `viola list` and in the web `columnheader`s. Readback shows `[RB]` on the CLI and `data-rb="read"` on both the tape line and the transfer marker. A11y implication: accessible names and roles must match across the CLI and web surfaces. The readback / refused state must be conveyed by more than colour on both surfaces.
- **Multi-OS matrix `[windows-2025, macos-latest, ubuntu-latest]`** (multi-platform). The headless-browser suite runs on ubuntu only, with DejaVu fonts. A11y implication: axe runs for web-spa on ubuntu only. CLI TTY a11y (`NO_COLOR` / `TERM=dumb` / non-TTY with no SGR) needs checking on all three OSes.
- **Multi-version CLI fixtures and the stamped / unstamped gate** (multi-version-compat). A11y implication: the `cli_verified:false` and unverified states must be perceivable in the web strip (CLI column).
- **Fake agent vs recorded `viola verify` fixtures** (contract-test-against-sandbox). A11y implication: none directly.
- **Mutation testing scoped to each chunk diff, where any surviving mutant fails `run`** (discipline). A11y implication: a11y tests that live in crate tests fall under the mutation gate. Playwright / axe specs do not run under mutation.
- **No screen-content parsing, no blanket approval, no flakes** (discipline). A11y implication: a11y assertions must use the DOM, roles and attributes, never scraped child-screen content or screenshots. Waits must use auto-waiting locators or event offsets, never timeouts.
- **Compliance** is not triggered. The security excerpt reports "No compliance triggers".
- **Deferred / N/A:** mobile and `<760px` layouts are not laid out in v1. Layout checks are `scrollWidth <= clientWidth` at 1024 and at 760–1023 widths.

### Quality Gates Summary
- **Zero-flakiness statement:** "flaky tests are NOT tolerated. The nextest `retries = 0`, and Playwright `retries: 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` and `test.skip` are not allowed as a parking place."
- **Coverage thresholds:**
  - Comprehensive tier, enforced per OS job via `cargo llvm-cov nextest --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`: ≥85% line, ≥80% branch (as region coverage), ≥95% function.
  - Excluded from coverage: `viola-fake-agent`, `crates/viola-e2e`, `tests/support`, `fuzz/`.
- **Mutation gate:** `missed == 0 && timeout == 0`.
- **Performance gate:** hyperfine `max < 1.0 s` for hooks.
- **Build fails on:** any test failure (any level, any OS), coverage below threshold, a perf regression, a missed or timed-out mutant, or a lint / clippy / cargo-deny / zizmor / cargo-modules failure.
- **Axe verdict:** the web-spa axe verdict is binary. `violations` must be `[]` after each state strip renders (empty, 503, cocked).

## 6. Obs Plan Excerpt

### Obs Tier
- **Tier:** Standard (obs plan wording: "`Standard (1)`, with Minimal-tier exporter carve-outs")
- **Justification:** "The tests tier is Comprehensive (2), and obs sits one step below it, within the ±1 band. The security tier is Minimal (the passed variable, and "no compliance triggers"), so there is no Comprehensive driver". The carve-outs: no network OTLP exporter and no opt-in error reporter, because upstream bans outbound network calls and new listeners.

### Log Format JSON Schema (binding)
Obs Section 6 does not define its own schema. It points to tests: "The binding text is reproduced verbatim in Section 3 → Log format JSON schema, from upstream-context Section 5 Test Plan Excerpt → Test Harness Contract Summary. This section only **adds** fields and the D-01…D-06 enum values; it renames and removes nothing." The binding block reads:

```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>}.ndjson`. Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
    - `process-start`, `process-exit`, `http-request`, `panic`: `corr` = null
  - `process` (`run|hook|mcp|ui`) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```

Obs additions to the binding schema. All were accepted on 2026-09-24 and carried to tests as amendment D-21.
- **New `event` values:** `release-from-driver`, `liveness-changed`, `state-recovered`, `sse-opened`, `sse-closed`, `parse-rejected`.
- **New `process` value:** `cli`, for short-lived verbs that have a resolved instance.
- **Null encoding:** a null `corr` or `instance` is written by leaving the key out. "absent = null". `schemas/diag-line.v1.json` rejects a literal `null` for either key.
- **Fields deliberately not emitted (D-12):**
  - `trace_id` / `span_id`
  - `service.name`: emitted once, as `service_name` on `process-start`
  - `event_type`: "this is what `event` already does, and a duplicate would amount to a rename"
- **Constraints:** `level` never includes TRACE. `message` is a static literal equal to the event name. Values are codes only, in kebab-case. No line may contain the GUI token, a `Cookie` header, `?t=`, or any stripped `CLAUDE*` value.
- **`http-request` additive fields:** `method`, `path` (from `uri.path()` only), `route`, `status`, `problem` (a Problem URN), `duration_ms`, `skipped`.

### Service Identity
- **service.name:** hardcoded `"viola"`, as `viola_core::SERVICE_NAME`. It is the binary name and is already served as `"name":"viola"` by `/health` and `/api/info`. It is never read from `OTEL_SERVICE_NAME` / `SERVICE_NAME`, because env vars are not a configuration channel.
- **service.version:** set at compile time: `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` in viola-core. The same value is used for the log `version`, the channel `sender`, the snapshot `writer` and `/health.version`.
- **deployment.environment:** N/A. viola is local-only with no hosting, and there is no env-var fallback.
- **Where identity appears in logs:** v1 builds no OTel Resource. Identity is emitted as fields on `process-start`: `service_name`, `version`, `os` (`windows|macos|linux`) and `pid`. Every line carries the role in `process` and the instance in `instance`.

### Sentry User-Feedback Widget (if applicable)
- **Widget pick:** N/A. The obs plan has no user-feedback widget. Its platform pick is "local error capture only, with no external platform". sentry 0.49.3 / sentry-tracing 0.49.3 are "**not admissible**" because they break the egress ban ("viola makes no outbound network calls") and the cargo-deny C-crate and Tokio bans.
- **Trigger surface:** N/A. Nothing appears in the UI. Errors that reach the GUI page show up as Problem URNs, each mirrored by `http-request{status, problem}`:
  - `urn:viola:problem:state-unreadable` (503, rack strip)
  - `urn:viola:problem:unauthorized` (401, access strip)
  - `not-found` (404) and `method-not-allowed` (405), defensive rack strips
  - `host-not-allowed` (403) never renders the page; the browser shows the raw Problem JSON.
- **Default state:** "there is no reporter, so there is nothing to opt into." A future reporter would need a `ureq` + `rustls` transport, the NEVER-log floor in `before_send`, off-by-default behaviour, no env-var enablement, and a security Decisions Log entry for the egress.

### Focus-Relevant Span Coverage (filtered)
(No focus-relevant spans in obs plan Section 4. a11y Phase 3 will recommend focus tracing spans that obs may add later: focus.shift / focus.trap.enter / focus.trap.exit / focus.restore.)

Frontend limits in v1: the web-spa frontend (Lit 3.3.3) is instrumented only at its boundary. "@opentelemetry/sdk-trace-web 2.11.0 is not admissible (OTLP POST impossible under GET-only/405, and it needs a bundler)". web-vitals 6.2.2 is not vendored in v1 (D-17). No browser-side span can therefore carry focus, keyboard, modal or route data. Obs Section 4 describes a "Frontend observable boundary" that Playwright reads and that a11y uses "to derive state and error surfaces; attribute names and announcements stay design/a11y-owned". It relates to state and status, not focus:
- **`<viola-atis>` tape state** (`TAPE connecting|live|stopped`, plus the `skipped N · N · N` counters): a11y correlation: status changes that need announcing. Server-side matches: `sse-opened` (→ `live`), `sse-closed{close_cause}` (→ `stopped`; `close_cause` ∈ `client-gone|server-shutdown|tail-error`), and `http-request{route:"/api/sessions", skipped}`.
- **`<viola-readback>`** (on `<viola-event-feed>` send lines and `<viola-transfer>` markers; states `open|read back|unable|unconfirmable`, with `unable · <reason> · <detail>`): a11y correlation: readback status announcements. Server-side matches, joined by the send `cursor` in `corr`: `send-issued` (open), `send-confirmed{confirmed:true}` (read back), `send-confirmed{confirmed:false}` (unconfirmable), `send-refused{refusal, detail}` (unable).
- **`ui.http_request`** for `/` and `/assets/*` (page load): a11y correlation: none for focus. The time-to-interactive proxy is read from Playwright `performance.getEntriesByType('navigation'|'resource')` and joined to server `http-request` / `sse-opened` lines by timestamp.

## 7. Creator Brief Excerpt

_Source: `.andromeda/input.md` (User Input + Extracted + folded refs) and project-root `refs/viola-brief.md`, `refs/viola-prior-art.md`, read at full fidelity. The brief carries no explicit accessibility requirement; the quotes below are its only a11y-bearing signals._

### Must-Work Scenarios

- input.md, User Input: *"holds a one-driver wheel the human can take at any moment, and shows the active and linked sessions in a minimal local GUI."*
- refs/viola-brief.md §1 (founder, translated): *"a minimal GUI showing which sessions are active and which are linked."*
- refs/viola-brief.md §2 D4: *"both agents stay interactive terminal sessions the human can watch and take over at any moment."*
- refs/viola-brief.md §3.2 item 4: *"`bridge ui` — a local web page served by the same binary: the sessions, the links, a feed of recent events. Read-only first; take-the-wheel, pause and unlink come next."*
- refs/viola-brief.md §5 O1: pass-through to a real terminal — *"Cyrillic, emoji, CJK, box tables, a ~300-character wrapped line, code blocks, a folded long output, a diff and a question dialog all rendered as without the wrapper"* (the wrapped `claude` TUI must render unchanged through `viola run`).
- refs/viola-brief.md §6: the first live test — *"the overseer sends `/andromeda-new-session` to the builder, waits for `Stop`, and reads the dashboard."*

### Rigor Hints

- No WCAG target is stated anywhere in the brief (no "WCAG 2.1 AA", no "AAA", no cognitive-accessibility ask).
- refs/viola-brief.md §3.4: *"The GUI as a local web page rather than a native window: no GUI toolkit on any platform, and an agent can verify it through a headless browser, which is how the founder's projects verify UI by default."*
- refs/viola-brief.md §7: *"The GUI is a UI surface, so `/andromeda-design` runs and a11y's UI machinery switches on; the page itself is minimal."*
- refs/viola-brief.md §3.4: *"CI on all three OSes against a FAKE AGENT … Tests spend no tokens and do not flake."*
- refs/viola-brief.md Appendix A: *"Always the founder's … anything that needs human eyes on a GUI."*

### A11y Anti-Patterns (creator's explicit asks)

- refs/viola-prior-art.md §4: *"Overclaiming. The GUI shows only what the hooks and the wrapper observed."*
- refs/viola-prior-art.md §4: *"Screen parsing as the primary channel … Viola parses no screen for content (R7)."*
- No explicit a11y anti-pattern (captcha, ARIA-without-semantics, manual-only review) is named by the creator.

### Overseer Directions (founder-delegated, recorded 2026-09-24 at the start of this a11y run; apply from Phase 1 on)

Verbatim from the overseer:

1. *"Inject axe into the tests plan own Playwright driver (@axe-core/playwright), no second browser stack; a11y supplies the axe configuration the test plan is waiting for."*
2. *"Bind to design token NAMES only; contrast/focus/size assertions must hold on the ubuntu CI DejaVu font fallback (founder: ubuntu is primary, CI must pass)."*
3. *"Violation JSON aligned to the obs plan log schema and event list (obs aligns to tests; a11y aligns to obs)."*
4. *"Cross-plan audit item: after a `viola ui` restart the SSE stream gets 401, and the page must show the 401 access strip (announced), not \"TAPE stopped / not answering\"."*
5. *"CLI/TUI: keyboard and screen-reader discipline, no fake automation."*
6. *"Dark only."*

