## 1. A11y Scope

**Web-spa entities (`viola ui`, one route `/`)**

- **Entity:** Page shell at `/`: document `lang`, `<title>`, landmark and heading structure
  - **Source:** Arch excerpt, Surfaces ("minimal view-only local web GUI"). Layout excerpt, Heading Hierarchy Anchors ("No h1/h2/h3 levels are assigned"; "landmark roles: none assigned by the layout"; "semantic roles of racks, strips and the tape are owned by a11y"). Design excerpt, ARIA-Relevant Component Patterns → Document title.
  - **Assertability:** assertable

- **Entity:** ATIS header `<viola-atis>`: sticky header with `VIOLA`, the TAPE state cell (`TAPE connecting|live|stopped`), the budget reading and age, the `budget-paused` box, the `skipped N · N · N` counters and the `expired` box
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → ATIS header. Layout excerpt, Layout Types (the four fixed regions). Obs excerpt, Focus-Relevant Span Coverage → `<viola-atis>` tape state. Tests excerpt, Critical Path 6 (the ATIS cell shows `budget-paused`).
  - **Assertability:** assertable

- **Entity:** WRAPPED rack: a table of `<viola-session-row>` strips with a caption row `NAME LIVE STATUS WHEEL DIALOG CLI`, including the cocked, stale and `cli_verified:false` strip states
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Rack (`table` / `row` / `columnheader` / `cell`). Tests excerpt, E2E driver ("Tests query by role: `table`… `row`… `cell`, `columnheader`"). Tests excerpt, Coverage Triggers → multi-version-compat.
  - **Assertability:** assertable

- **Entity:** UNWRAPPED · READ-ONLY rack
  - **Source:** Layout excerpt, Layout Types → web-spa. Arch excerpt, Critical paths hint 1 ("wrapped, or read-only unwrapped").
  - **Assertability:** assertable

- **Entity:** Transfer marker `<viola-transfer>`, the link/unlink rows in the rack flow
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Transfer marker ("no role specified"). Tests excerpt, Critical Paths → E3.
  - **Assertability:** assertable

- **Entity:** Readback box `<viola-readback>` and its adjacent word cell, in the states `open` / `read back` / `unable · <reason> · <detail>` / `unconfirmable`
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Readback box (`aria-hidden="true"`, named by the word cell). Obs excerpt, Focus-Relevant Span Coverage → `<viola-readback>`. Tests excerpt, Critical Path 2 (`data-rb`).
  - **Assertability:** assertable

- **Entity:** Tower tape `<viola-event-feed>`: a `role="log"` element whose lines are native `<details>`/`<summary>`. It also contains the trim notice, the `N new lines below` anchor and `#tape-end`.
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Tower tape and `N new lines below` anchor. Layout excerpt, Focus Management Anchors → tape end zone and tape.
  - **Assertability:** assertable

- **Entity:** Status announcer: visually hidden, `role="status"`, `aria-live="polite"`
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Status announcer (announces only a strip turning cocked, a readback refusal, and `TAPE stopped`). Overseer Direction 4 in the creator brief excerpt adds the 401 access strip, announced.
  - **Assertability:** assertable

- **Entity:** Skip link `skip to tape`, the first tab stop
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Skip link. Layout excerpt, Focus Management Anchors.
  - **Assertability:** assertable

- **Entity:** State strips: first reading / empty rack / empty tape, 503 `state-unreadable`, defensive 404/405, 401 access strip, and `TAPE stopped`
  - **Source:** Design excerpt, Loading / Error / Empty State Patterns and User-Facing Error Surfaces. Layout excerpt, Error Boundary Placement. Security excerpt, Anti-Patterns Rejected (cookie/`?t=` row: the page can load and then get a 401 on `/api/*`). Obs excerpt, Sentry section (list of Problem URNs).
  - **Assertability:** assertable

- **Entity:** Global token layer: `:focus-visible` ring, reduced-motion override, forced-colors fallback, dark-only palette aliases
  - **Source:** Design excerpt, A11y-Relevant Design Tokens (focus ring, motion, colour pairs). Creator brief excerpt, Overseer Direction 6 ("Dark only").
  - **Assertability:** assertable

- **Entity:** 403 host-not-allowed page
  - **Source:** Design excerpt, User-Facing Error Surfaces ("The page cannot load; the browser shows raw Problem JSON"). Obs excerpt, Sentry section.
  - **Assertability:** not-assertable
  - **Reason:** viola serves no markup here. The browser renders raw `application/problem+json`, so there is no viola UI surface to assert.

- **Entity:** Reserved v1.x brake controls (`I HAVE CONTROL`, `UNLINK`, and the `POST /api/sessions/{name}/pause|unlink` UI)
  - **Source:** Design excerpt, ARIA-Relevant Component Patterns → Reserved v1.x brake controls ("not built in v1"). Arch excerpt, Surfaces → deferred surfaces.
  - **Assertability:** not-assertable
  - **Reason:** These are not built in v1, so there is nothing to assert. They re-enter scope in v1.x as native `button` elements that must handle 401/403 Problem Details (security excerpt, Anti-Patterns Rejected).

**CLI and terminal entities**

- **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `ui`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes
  - **Source:** Arch excerpt, Stack → clap. Design excerpt, Surfaces → cli. Layout excerpt, Layout Types → cli. Creator brief excerpt, Overseer Direction 5.
  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.

- **Entity:** `viola run` TUI passthrough: the unmodified `claude` TUI hosted in ConPTY/openpty
  - **Source:** Arch excerpt, Stack → portable-pty + vt100 ("viola does not own the rendered TUI content"). Tests excerpt, E2E driver → tui ("Tests never parse the rendered child screen for content"). Creator brief excerpt, A11y Anti-Patterns ("Viola parses no screen for content (R7)").
  - **Assertability:** boundary-only
  - **Reason:** The rendered content belongs to the vendor (`claude`) and is not traced. The assertion boundary is viola's wrapper:
    - viola writes zero bytes to the terminal except the child's output;
    - human keystrokes are never blocked, refused or delayed past the current atomic paste;
    - focus, mouse and resize sequences pass through and do not count as human editing.

**Not-assertable (no UI surface)**

- **Entity:** GUI HTTP API: `/api/*`, SSE `/api/events`, `/ready`, `/health`, `/api/info` and the RFC 9457 bodies
  - **Source:** Arch excerpt, Stack → axum. Tests excerpt, E2E driver → api-service.
  - **Assertability:** not-assertable
  - **Reason:** API service with no UI surface. Its Problem `title`/`detail` reach users only through the web-spa state strips, which are asserted there.

- **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
  - **Source:** Arch excerpt, Surfaces ("a Claude Code plugin (hooks + stdio MCP server)"). Tests excerpt, E2E driver → ipc-internal and cli (`viola hook`).
  - **Assertability:** not-assertable
  - **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.

- **Entity:** Internal crates (modular monolith)
  - **Source:** Arch excerpt, Project Intent ("one binary with compiler-enforced internal crates"). Tests excerpt, E2E driver → library-only.
  - **Assertability:** not-assertable
  - **Reason:** Library-only with no UI surface.

- **Entity:** Phone / remote view
  - **Source:** Arch excerpt, Stack → Mobile framework N/A and Surfaces → deferred. Tests excerpt, Coverage Triggers → Deferred / N/A.
  - **Assertability:** not-assertable
  - **Reason:** Deferred and not built in v1. When it lands it is the same web page behind authentication, so it will inherit web-spa tool reach.

**Boundary-only vendor zones:** none on web-spa. The security excerpt reports "no login forms… no captcha". The enforced CSP (`frame-ancestors 'none'`, `default-src 'none'`, and only `'self'` for script, style, connect and img) blocks third-party iframes and widgets. The only vendor-content boundary is the `viola run` TUI above.

**Compliance cross-check:** the security excerpt's A11y Compliance Triggers name no regime (no Section 508 / ADA / EAA / EN 301 549 / AODA / JIS X 8341). No regime-mandated SC applies to any surface.

## 2. A11y Surfaces & Assistive Tech Reach

- **Surface:** web-spa (`viola ui` strip bay, loopback `127.0.0.1:47319`, Lit 3.3.3 custom elements in light DOM)
  - **Automated tool reach:**
    - **axe-core through `@axe-core/playwright`.** Version 4.13.0 is already declared in the tests E2E driver, running on Playwright Test 1.63.0 with headless Chromium on ubuntu only (tests excerpt, E2E driver). The standalone `axe-core` engine runs against the rendered DOM.
    - **Lighthouse a11y category (`lighthouse` CLI)** and **pa11y (`pa11y` / `pa11y-ci`)** can reach the page at `http://127.0.0.1:<port>/` or `http://localhost:<port>/`, the only hosts on the Host allowlist (arch excerpt, Stack → axum).
    - **Constraint on Lighthouse and pa11y:** Overseer Direction 1 says "no second browser stack". Phase 2 must check whether they can attach to the tests' Playwright Chromium, or drop them.
    - **Other reach inside the same driver:** role-based locators, the `document.title` assertion, and forced-colors / reduced-motion media emulation (design excerpt, Focus ring tokens → forced-colors fallback; Motion tokens → reduced-motion override).
  - **Manual verification (supplemental):**
    - NVDA with Edge/Chrome on Windows, the live-supported target (arch excerpt, Surfaces → Platforms; design excerpt, Surfaces → "Edge/Chrome first").
    - VoiceOver with Safari on macOS.
    - Orca with Firefox/Chromium on Linux.
    - A Windows forced-colors pass.
    - These passes are founder-owned ("anything that needs human eyes on a GUI", creator brief excerpt, Rigor Hints → Appendix A). They are never the sole method.
  - **ARIA roles inventory:**
    - **Landmarks:** none assigned by layout. Candidates follow from existing native structure: `banner` from a native `<header>` for `<viola-atis>`, and `main` for racks plus tape. There is no `navigation`, `contentinfo`, `search` or `form` (layout excerpt, Heading Hierarchy Anchors; v1 is GET-only with `form-action 'none'`, security excerpt).
    - **Structure:** `table` / `row` / `columnheader` / `cell`.
    - **Interactive:** `link` (skip link, `N new lines below` anchor) and the native `<details>`/`<summary>` disclosure. There is no `button` in v1, and no `dialog` / `tab` / `combobox` / `menu`.
    - **Live regions:** `log` (tape) and `status` (announcer). There is no `alert`.
  - **Service identity tagging:**
    - Obs excerpt, Service Identity: `service.name` = `"viola"` (`viola_core::SERVICE_NAME`), version = `CARGO_PKG_VERSION`, `deployment.environment` N/A.
    - v1 builds no OTel Resource. Identity is carried as `service_name` / `version` / `os` / `pid` on `process-start`, with `process` = `ui` and `instance` on every line.
    - a11y violation artifacts tag with `service_name:"viola"`, `version` and `os:"linux"`, since axe runs only on ubuntu.
  - **Notes (scope gaps for Phase 3; each traces to an excerpt):**
    - **Implicit live region on the tape.** `role="log"` carries implicit `aria-live="polite"`. The design says the tape is "never an aria-live region as a whole", and the layout says "The whole tape is never announced". The implicit live behaviour of `log` conflicts with both, so it must be resolved and asserted.
    - **ARIA roles on custom-element hosts.** Rack rows are the custom element `<viola-session-row>` carrying `role="row"`, which is ARIA on a generic host (design excerpt, Rack). The Phase 3 principle is semantic HTML first. axe `aria-required-children` / `aria-required-parent` must pass on the rendered light DOM.
    - **Headings are typography only.** No heading levels are assigned. `WRAPPED`, `UNWRAPPED · READ-ONLY` and `TAPE` are typography roles only and are "the page's only wayfinding" (layout excerpt, Heading Hierarchy Anchors). This leaves a semantic-heading gap under SC 1.3.1 / 2.4.6.
    - **Sticky header may hide focus.** The sticky `<viola-atis>` header sits over the only scroll region (the tape) while focus moves through the tape `<summary>`s. This risks focus being obscured (SC 2.4.11, WCAG 2.2).
    - **Reflow below 760 px is not laid out.** Tests check reflow at 1024 and 760–1023 only, and the below-mid-width band "has no v1 layout" (layout excerpt, Layout Types; tests excerpt, Coverage Triggers → Deferred). SC 1.4.10 at 320 CSS px is not covered.
    - **Initial focus on mount** is "not specified" (layout excerpt, Focus Management Anchors).
    - **The 401 access strip has no role or focus spec** (design excerpt, User-Facing Error Surfaces). It is also outside the design's three-item announcer scope, but Overseer Direction 4 requires it announced after a `viola ui` restart, instead of `TAPE stopped`.
    - **axe must run under the enforced CSP and Trusted Types** with no `bypassCSP`, and the console must stay free of CSP/Trusted Types violations (tests excerpt, Coverage Triggers → Security vectors). Phase 2 must confirm that axe injection causes no Trusted Types violation.
    - **Event content has no semantic structure.** Assistant output reaches assistive technology as plain `textContent`, with no semantic headings, lists or links (security excerpt, Anti-Patterns Rejected → rendering event content as HTML). All structure comes from viola's own markup.

- **Surface:** cli (`viola` verbs)
  - **Automated tool reach:**
    - No automated a11y verification tool exists for terminal output; a11y tools assume a DOM or native widget tree.
    - The fallback is output-discipline assertions on the tests' existing CLI drivers: `assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1, and SGR-only checks through the portable-pty outer PTY (tests excerpt, E2E driver → cli). Their pass/fail results are machine-parseable through the harness `run` JSON.
    - These are not a11y tools. They produce no a11y-tool JSON, so no WCAG conformance claim may rest on them.
  - **Manual verification (supplemental):**
    - NVDA with Windows Terminal/conhost (live target).
    - VoiceOver with Terminal.app.
    - Orca with GNOME Terminal.
    - Overseer Direction 5: "keyboard and screen-reader discipline, no fake automation".
  - **ARIA roles inventory:** N/A (terminal output). The structural equivalents are:
    - the BAY context line, then uppercase column captions, then the `-- UNWRAPPED - READ-ONLY --` separator (heading-equivalent);
    - `--help` group labels: board / traffic / wheel / handoff / setup (layout excerpt, Heading Hierarchy Anchors → cli).
  - **Service identity tagging:** `service_name:"viola"` and `version`. Obs uses `process:"cli"` for short-lived verbs that have a resolved instance (obs excerpt, Log Format additions). The CLI a11y checks run on all three OSes, so `os` is tagged per matrix leg.
  - **Notes:**
    - Required behaviour (design excerpt, Surfaces → cli; Loading/Error/Empty → CLI waiting; security excerpt, C0/C1 escaping):
      - colour is only a second cue (amber `DIALOG`, dim `stale`, always beside the word);
      - there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`;
      - output is static and linear, with no spinner or redraw;
      - C0/C1 control characters are escaped except `\n` and `\t`.
    - This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).

- **Surface:** tui (`viola run` passthrough; boundary-only)
  - **Automated tool reach:**
    - No automated a11y verification tool applies. The rendered `claude` TUI is vendor content and parsing it is banned (R7).
    - Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
  - **Manual verification (supplemental):** NVDA with Windows Terminal (Windows is the live target), confirming the wrapped TUI reads the same as unwrapped. Creator brief excerpt, Must-Work → O1: "all rendered as without the wrapper".
  - **ARIA roles inventory:** N/A (terminal).
  - **Service identity tagging:** `service_name:"viola"`, `process:"run"`, `instance:<ViolaName>`.
  - **Notes:** viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline). This part of the scope is a boundary guarantee only, not a conformance claim about `claude`'s TUI.

## 3. A11y Assertion Harness Specification

- **A11y testing tool pick:**
  - **web-spa:** `@axe-core/playwright` 4.13.0, injected into the tests' own Playwright Test 1.63.0 driver. It is already declared (tests excerpt, E2E driver), and Overseer Direction 1 says "no second browser stack". a11y supplies the axe configuration the test plan is waiting for: rule tags, disabled rules and conformance target. The test plan says "specs must not add their own `withTags` / `disableRules`", and until a11y names a configuration axe's default rules apply.
  - Whether Lighthouse or pa11y is added as a second gate, and whether Playwright ARIA snapshots are used for the role and name tree, is left to Phase 2 research, under the single-browser-stack constraint.
  - **cli / tui:** no a11y tool. Use the existing `assert_cmd` / `trycmd` / portable-pty output-discipline assertions, with manual screen-reader passes as a supplement.

- **WCAG criteria mapping (tier Standard, see Section 6):**
  - **Baseline:** full WCAG 2.1 AA as the header label.
  - **Design-bound WCAG 2.2 AA additions** (from bindings the design and layout already made, not from a Section 5 trigger):
    - SC 2.5.8 Target Size (Minimum), from the Target size tokens (`--line-h` for `<summary>` hit height);
    - SC 2.4.11 Focus Not Obscured (Minimum), from the sticky ATIS header over the tape.
    - Phase 3 decides whether the conformance label is 2.1 AA with these two added, or 2.2 AA.
  - **Trigger-pulled AAA:** none. No `target-size`, `cognitive-accessibility` or `motion-sensitive` trigger fired (Section 5).
  - **Design contracts asserted at AA level:**
    - SC 2.3.1 has no flash sources, and the reduced-motion override targets `--cock-dur` and `--fade-dur` (design excerpt, Motion tokens).
    - SC 1.4.11 applies to the non-text state rules.
    - SC 1.4.12 applies to the fixed-height tokens `--line-h`, `--strip-h`, `--lh-display` and `--lh-label`, which are at risk of clipping under a text-spacing override.

- **Structured violation JSON schema:** binding from obs excerpt, Log Format JSON Schema. That section points to the tests Harness Contract, and a11y aligns to it (Overseer Direction 3). Reproduced verbatim:

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

  - **Obs additions a11y inherits (D-21):**
    - `event` values `sse-opened`, `sse-closed`, `liveness-changed`, `send-refused` and others; `process` value `cli`;
    - absent key means null, and `schemas/diag-line.v1.json` rejects a literal `null` for `corr`/`instance`;
    - no `trace_id`/`span_id`;
    - `message` is a static literal equal to the event name, and values are kebab-case codes;
    - no line may contain the GUI token, a `Cookie` header, `?t=` or a stripped `CLAUDE*` value.
  - **Constraints Phase 3 must honour:**
    - `event` and `process` are closed enums. An a11y violation line needs a Decisions Log entry to add, for example, an a11y event value, or it stays a harness test artifact outside `diagnostics/` whose line shape mirrors these field names without renaming any.
    - a11y-specific fields are additive only, in snake_case like `duration_ms` and `service_name`: `wcag_criterion`, `violation_type` (the axe rule id), `severity` (the axe impact), `surface` (`web-spa`), `selector`, `remediation`.
    - The tests excerpt, Coverage Triggers → Error sanitization, bans the token, `Cookie`, `?t=`, absolute paths and `CLAUDE*` values from a11y reports. Axe `url` and `nodes[].html` must therefore be scrubbed. `nodes[].html` can carry untrusted event text.

- **Focus management test harness:** reuses the tests' Playwright driver; there is no separate harness (tests excerpt, Test Harness Contract Summary).
  - Each test boots its own session: `boot --session pw-<spec>-<test id>-<workerIndex>`, then navigates to `/?t=<token>` from `ui/<port>.url`, then runs `cleanup`.
  - **Scripted Tab walk** asserting the `document.activeElement` sequence: skip link, then every tape `<summary>` from oldest to newest, then the `N new lines below` anchor (only while scrolled up). Strips, captions and transfer markers are never focused (layout excerpt, Focus Management Anchors; design excerpt, Global keyboard).
  - **No focus movement or theft** on state transitions: cocked strip, readback refusal, `TAPE stopped`, 503, 401 access strip, SSE arrivals and auto-follow. Focus must persist through chaos transitions such as stale liveness and snapshot recovery (tests excerpt, Coverage Triggers → chaos).
  - **Focus-ring visibility** checked through computed style against `--focus-ring`, `--focus-w` and `--focus-offset`.
  - **Focus not obscured:** the focused `<summary>` must not sit under the sticky `<viola-atis>` header.
  - **Waits** use auto-waiting locators or event offsets only, never timeouts, with Playwright `retries: 0` (tests excerpt, Quality Gates).
  - **No focus spans exist.** Obs excerpt, Focus-Relevant Span Coverage: no browser-side span can carry focus data, so focus state is asserted in the DOM only.

- **Keyboard test harness:**
  - **web-spa:** `page.keyboard.press()` with Tab and Shift+Tab over the full sequence. Enter on the skip link must land on `#tape-end`. Enter on the new-lines anchor must jump instantly. Enter and Space must expand and collapse a `<summary>`.
  - Arrow keys and Escape have no contract: there are no composite widgets, dialogs or modals (design excerpt, Global keyboard; layout excerpt, Modal focus traps N/A). Assert that no keyboard trap exists (SC 2.1.2).
  - **cli:** keyboard is not interactive. There is no prompt, TUI mode or pager. Assertions cover stdin/`--file` input only (arch excerpt, Critical paths hint 7).
  - **tui:** the automated boundary is that human keystrokes through the portable-pty outer PTY are never blocked. A manual keyboard pass is supplemental (Overseer Direction 5).

- **Screen reader test pattern:**
  - **Automated proxy (web-spa):**
    - assert the text of the `role="status"` region after each SSE-driven trigger: cocked ("builder: dialog pending, permission"), readback refusal ("send to builder unable, not-delivered, …"), `TAPE stopped`, and the 401 access strip (Overseer Direction 4);
    - assert the accessible name of each readback word cell while `<viola-readback>` stays `aria-hidden`;
    - assert `document.title` (`DIALOG <name> · viola`, `+N`).
  - **Manual pass (supplemental):** NVDA (Windows, Edge/Chrome and Windows Terminal), VoiceOver (macOS) and Orca (Linux) on every Section 4 path. It is founder-owned per the creator brief excerpt, Appendix A.

- **Contrast verification harness:**
  - **axe `color-contrast` rule** runs on the rendered DOM under the ubuntu CI DejaVu font fallback (Overseer Direction 2; tests excerpt, Coverage Triggers → multi-platform).
  - **Token-pair assertions by token name only** (Overseer Direction 2): read `getComputedStyle` custom properties and compute the ratio. Pairs from the design excerpt, Color tokens:
    - text at SC 1.4.3 4.5:1: `--ink`/`--surface-bay`, `--ink`/`--surface-strip`, `--ink-dim`/`--surface-bay`, `--ink-dim`/`--surface-inset`, `--ink-on-paper`/`--rb-fill`, `--attention`/`--surface-inset`, `--handoff`/`--surface-bay`;
    - non-text at SC 1.4.11 3:1: `--rule-info`/`--surface-bay`, `--rb-rule`/`--surface-inset`, `--rb-strike`, `--rule-field`/`--surface-strip`, `--focus-ring`/`--surface-bay`, `--focus-ring`/`--surface-strip`, and the `--attention` band on `--surface-strip`.
  - **Negative assertions** for banned placements: `--ink-dim`, `--attention` (as text) and `--handoff` (as text) must never render on `--surface-strip`. `--rule-deco` must never be the only carrier of information.
  - **Themes:** dark only, so there is one palette and no light theme (Overseer Direction 6). Forced-colors emulation must pass: borders survive, the strike becomes a dashed outline plus `unable`, and the cock band maps to the system highlight colour.

- **CI integration:**
  - **Runner:** tests' 5-command discipline, `scripts/agent-run.sh run --browser` (or `.ps1`), which runs `npx --prefix e2e-web playwright test`. It runs on the ubuntu leg of GitHub Actions `ci.yml` only; `--browser` on Windows or macOS exits 2 with `reason:"browser-linux-only"`, and a missing Chromium fails with `reason:"browser-missing"`.
  - **Where results land:** inside the existing `playwright` suite entry of the `run` JSON (`suites[{suite:"playwright",passed,failed,skipped,artifact}]`, with failures in `suites[].failures[]`).
  - **Gate:** `gate --require playwright` never allows skips.
  - **Axe verdict:** `violations` must equal `[]` after each state strip renders: empty, 503 and cocked. a11y extends this to the 401 access strip, `TAPE stopped` and steady state.
  - **No new suite:** no new `suite` enum value (such as `a11y`) is added without a Decisions Log entry.
  - **CLI output discipline** runs inside `nextest-integration` / `nextest-e2e` on all three OS legs.

## 4. Critical Paths (must-be-accessible)

**1. Founder reads the bay and navigates the tape**
- **Flow:** the page shows active and linked sessions, liveness, status, wheel, budget and `skipped`. The reader then skips to the tape, expands a line, and jumps to new lines.
- **Surfaces involved:** web-spa, plus cli (`viola list` must use the same captions and slot order).
- **Required ARIA roles:** banner/main candidates (native elements; roles TBD in Phase 3), `table` / `row` / `columnheader` / `cell`, `log`, `link` (skip link and new-lines anchor), native `<details>`/`<summary>`.
- **Required focus order:** initial focus at document start (currently unspecified; Phase 3 decides). Then skip link, then tape `<summary>` from oldest to newest, then the `N new lines below` anchor. No focus theft from auto-follow or SSE arrivals.
- **Required WCAG SC coverage per tier:** SC 1.3.1, 1.3.2, 1.4.1, 1.4.3, 1.4.10, 1.4.11, 1.4.12, 1.4.13, 2.1.1, 2.1.2, 2.4.1, 2.4.3, 2.4.6, 2.4.7, 2.4.11 (design-bound), 2.5.8 (design-bound), 3.1.1, 4.1.2.
- **Source:**
  - Creator brief excerpt, Must-Work: "a minimal GUI showing which sessions are active and which are linked" and "`bridge ui` … the sessions, the links, a feed of recent events".
  - Arch excerpt, Critical paths hints 1–3.
  - Tests excerpt, Critical Paths 6 (ATIS `budget-paused`) and 7 (unverified CLI column), E3 (transfer markers), and E4 (SSE `Last-Event-ID` resume).
  - Tests excerpt, Coverage Triggers → cross-surface.

**2. Confirmed send and readback, or refusal**
- **Surfaces involved:** web-spa (tape send line and transfer marker `<viola-readback>`) and cli (`viola send` readback mirror `[RB]`).
- **Required ARIA roles:** `log`, `status` (polite refusal announcement), native `<details>` holding the full refusal text. The readback box is `aria-hidden` and named by its word cell.
- **Required focus order:** focus stays where it is. The refusal is announced politely, and the full text is reachable by Tab to its `<summary>` and Enter/Space.
- **Required WCAG SC coverage per tier:** SC 1.1.1, 1.3.1, 1.4.1, 1.4.11, 3.3.1, 4.1.2, 4.1.3.
- **Source:**
  - Tests excerpt, Critical Path 2 (`data-rb` goes `open` → `read`, or → `refused` on failure).
  - Obs excerpt, Focus-Relevant Span Coverage → `<viola-readback>` (`send-issued` / `send-confirmed` / `send-refused` joined on `corr`).
  - Design excerpt, User-Facing Error Surfaces → readback refusal.
  - Creator brief excerpt, Must-Work §6: "the overseer sends `/andromeda-new-session` … and reads the dashboard".

**3. Dialog pending, then `answer` (question / permission / plan), including unverified-CLI withholding**
- **Surfaces involved:** web-spa (cocked strip) and cli (`viola answer`, `unverified-cli` refusal).
- **Required ARIA roles:** `row` / `cell` (DIALOG column), `status` (polite "builder: dialog pending, permission"). The document title changes to `DIALOG <name> · viola` (`+N`). No `dialog` role: the dialog renders in the `claude` TUI, not the page.
- **Required focus order:** no focus move when a strip cocks or reverts. The out-of-view cue is `document.title` plus the announcement.
- **Required WCAG SC coverage per tier:** SC 1.4.1, 1.4.3 (`--attention`/`--surface-inset`), 1.4.11 (cock band), 2.2.2 (the cock animation plays once), 2.3.1, 2.4.2, 4.1.3. The reduced-motion override on `--cock-dur` is asserted as a design contract.
- **Source:**
  - Tests excerpt, Critical Path 4 (`data-dialog="pending"`, `DIALOG <kind>`, the `document.title` revert) and Path 7 (answers withheld on unverified CLI).
  - Design excerpt, Loading/Error/Empty → cocked strip.
  - Arch excerpt, Critical paths hint 6.

**4. Human takes the wheel, automation is refused, and `release` returns it**
- **Surfaces involved:** tui (boundary-only), cli (`pause`, `release`, `send` refused with `human-typing`) and web-spa (WHEEL column).
- **Required ARIA roles:** web: `cell` for WHEEL, with the word `human` carrying the state. cli/tui: N/A (terminal).
- **Required focus order:**
  - tui: human keystrokes are never blocked or delayed, and focus/mouse/resize sequences are ignored for wheel purposes.
  - web: no focus change on a wheel change.
- **Required WCAG SC coverage per tier:** SC 2.1.1 (the human keyboard path through the wrapper is never blocked), 1.4.1 (the word `human` in strong weight, not colour), 1.3.1. Also CLI output discipline: colour as a second cue only, and the refusal written to stderr as `unable … hint:` with a typed exit code.
- **Source:**
  - Tests excerpt, Critical Path 5.
  - Arch excerpt, Critical paths hints 4–5, and A11y-Relevant Conventions → Human input priority.
  - Creator brief excerpt, Must-Work: "a one-driver wheel the human can take at any moment" and D4 "the human can watch and take over at any moment".
  - Security excerpt: "NEVER let a security refusal block the human".

**5. Degraded and access states: 503, `TAPE stopped`, stale, and the 401 after a `viola ui` restart**
- **Surfaces involved:** web-spa.
- **Required ARIA roles:** `status` (polite) for `TAPE stopped` and the 401 access strip. The access-strip role is otherwise unspecified in design; Phase 3 decides it. There is no `alert` by default: the layout specifies "no role=alert" for 503.
- **Required focus order:**
  - no focus move on any degraded transition;
  - focus stays on its current element through stale and recovery transitions;
  - the recovery instruction is readable text (there is no in-page control).
- **Required WCAG SC coverage per tier:** SC 1.3.1, 1.4.1, 1.4.11 (`--rule-info` boxed state), 3.3.1, 3.3.3 (plain instruction line), 4.1.3.
- **Source:**
  - Design excerpt, User-Facing Error Surfaces (401 / 503 / `TAPE stopped`).
  - Layout excerpt, Error Boundary Placement.
  - Tests excerpt, Coverage Triggers → chaos, and Critical Paths → E5.
  - Obs excerpt, `sse-closed{close_cause}` for TAPE state.
  - Security excerpt: the page can load and then get a 401 on `/api/*`, and that state "needs to be perceivable".
  - Creator brief excerpt, Overseer Direction 4: the SSE 401 after restart must show the announced access strip, not `TAPE stopped`.

**6. CLI board, wait/last, verify and launch-line output**
- **Surfaces involved:** cli.
- **Required ARIA roles:** N/A (terminal). The structural equivalents are the caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels.
- **Required focus order:** N/A. Output is linear and static: `waiting: <name>` once on stderr (TTY only), `verify` step lines `[NN/NN] … pass|fail`, and `viola ui` prints one launch line to stderr.
- **Required WCAG SC coverage per tier:** WCAG does not apply directly to terminal output, so there is no conformance claim. Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
- **Source:**
  - Arch excerpt, Critical paths hint 7, and A11y-Relevant Conventions.
  - Tests excerpt, Critical Path 3 (`wait` / `last`).
  - Design excerpt, Loading/Error/Empty → CLI waiting, and User-Facing Error Surfaces → CLI refusals.
  - Creator brief excerpt, Overseer Direction 5.

**Not carried as paths:** Tests Critical Path 1 (`run` start sequence), E1 (unwrapped hooks no-op) and E2 (`CLAUDE*` env strip) run on tui/ipc-internal/api-service with no viola-owned UI. Security vectors 1–9 are also omitted as not-assertable. Their user-visible outcomes are covered by Paths 1, 4 and 5.

## 5. A11y Triggers

**Triggered**

- **Trigger type:** visual-discrimination
  - **Source:** design excerpt, State color tokens. There are seven state encodings, none carried by hue alone: success / error / warning / info / attention / stale, plus the CLI amber and dim SGR. Tests excerpt, Coverage Triggers → cross-surface: readback and refused state must be conveyed by more than colour on both CLI and web.
  - **Required assertion:**
    - axe `color-contrast` rule plus the token-pair ratio assertions listed in Section 3;
    - for each state, assert the printed word is present in the DOM (`read back`, `unable · <reason> · <detail>`, `stale`, `DIALOG <kind>`, `expired`, `unconfirmable`);
    - forced-colors emulation keeps each state distinguishable;
    - no SGR under non-TTY, `NO_COLOR` or `TERM=dumb`, and the word always sits beside any SGR on the CLI.

- **Trigger type:** keyboard-only (scoped)
  - **Source:**
    - Creator brief excerpt, Overseer Direction 5 ("keyboard … discipline, no fake automation") and Must-Work D4 ("the human can … take over at any moment").
    - Arch excerpt, A11y-Relevant Conventions → Human input priority.
    - The target users are developer power users: arch excerpt, Project Intent ("individual Claude Code subscribers").
  - **Required assertion:**
    - a scripted Tab/Shift+Tab walk of the full web tab sequence (skip link → summaries → new-lines anchor) with the Enter/Space contracts;
    - SC 2.1.1, 2.1.2 and 2.4.7 checked through the focus-ring computed style;
    - the portable-pty boundary assertion that human keystrokes are never blocked or delayed;
    - a manual keyboard pass for CLI/TUI as a supplement.

- **Trigger type:** screen-reader-priority (scoped)
  - **Source:** creator brief excerpt, Overseer Direction 5 ("keyboard and screen-reader discipline"). Design excerpt, Status announcer (a limited polite announcement scope). Overseer Direction 4 (the 401 strip is announced). No blind or low-vision target users are named (arch excerpt, Project Intent: "no a11y-priority user signals"), so this is scoped to announcement correctness and CLI/TUI linearity, not a full assistive-tech matrix.
  - **Required assertion:**
    - automated status-region text assertions per announcing trigger;
    - resolve the implicit `aria-live` of `role="log"` so the whole tape is never announced;
    - a manual NVDA / VoiceOver / Orca pass on Section 4 Paths 1–6 as a supplement.

**Not triggered**

- **wcag-mapping:** none. Security excerpt: "Compliance triggers: None"; no regime is named.
- **regulated-compliance:** none. The security tier is Minimal and there is no regime.
- **cognitive-accessibility:** none. The brief has no cognitive, older-user or language-proficiency signal (creator brief excerpt, Rigor Hints: "no cognitive-accessibility ask"). Auth has no login form and no session timeout (security excerpt), so SC 3.3.7 / 3.3.8 have no surface.
- **motion-sensitive:** not triggered. Motion tokens `--cock-dur`, `--fade-dur`, `--cock-ease` and `--fade-ease` are present, but the creator brief never mentions vestibular issues. The design's own contract (reduced-motion override, no flash sources, no looped animation) is still asserted through `prefers-reduced-motion` emulation under SC 2.3.1 / 2.2.2. There is no SC 2.3.3 AAA escalation.
- **multi-language:** not triggered. The arch has no i18n, RTL or `lang` conventions, and the design has no language tokens. Cyrillic, emoji and CJK appear only as passthrough or event content (creator brief excerpt, Must-Work O1). SC 3.1.1 page `lang` stays in the AA baseline. SC 3.1.2 for tape content of unknown language is left for Phase 3.
- **target-size:** not triggered. v1 has no touch surface, and the phone view is deferred (arch excerpt, Surfaces). The SC 2.5.8 check on `--line-h` for summary hit height is carried as a design-bound AA addition in Section 3, not as this trigger. Re-evaluate when the phone view or the v1.x brake buttons (`--strip-h`, 44×44) land.

## 6. A11y Tier

**Tier: Standard (1)**

**Justification:**
- **Surfaces and components.** There are three surfaces with a11y reach: web-spa with full axe reach, cli with output discipline only, and tui as a boundary only. Four are not-assertable: API, IPC/plugin, library and the deferred phone view. web-spa carries about 11 assertable entities, inside Standard's 5–15 component band.
- **Paths.** There are six must-be-accessible paths, above Minimal's 2–5 band.
- **Why not Minimal.** The security tier is Minimal, but the tests tier is Comprehensive, which already mandates an axe verdict of `violations: []` per state strip. The obs tier is Standard. Standard is the lowest tier within the tests tier ±1 band. The design excerpt already binds AA-level contracts that Minimal's three-SC baseline would leave unasserted: SC 1.4.3 / 1.4.11 token pairs, SC 1.4.12 spacing tokens, SC 2.5.8, SC 2.3.1, forced colours and reduced motion. The later public version (individual Claude Code subscribers) "must not need a rewrite" (arch excerpt, Project Intent).
- **Why not Comprehensive.** There is no compliance regime (security excerpt), no WCAG target or disability user signal in the creator brief (Rigor Hints: "No WCAG target is stated"), and no multi-language or cognitive trigger.
- **Scope of the Standard label.** It applies to web-spa only. For cli and tui, Standard means output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
