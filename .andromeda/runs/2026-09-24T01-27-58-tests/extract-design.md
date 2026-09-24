## 3. Design System Excerpt

### Surfaces
- **web-spa** (`viola ui`, loopback page at `http://127.0.0.1:47319`; Lit 3.3.3 light-DOM `viola-*` custom elements, hand-written `@layer` CSS, no JS build step, headless-browser verified; CI headless GUI checks run on ubuntu, where the fonts resolve to DejaVu Sans Condensed + DejaVu Sans Mono) — a single view-only page, the "strip bay". It shows a sticky ATIS/budget header, WRAPPED and UNWRAPPED · READ-ONLY session racks, transfer markers and the event tape (SSE, most recent at the bottom). v1 has no controls.
- **cli** (Rust clap 4.6.7 terminal binary; human TTY at expression 0.2, and `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough at 0.0) — flat verbs (`run · send · wait · last · list · answer · verify · pause · release · link · unlink · ui · plugin install`). `viola list` prints a k9s-style fixed-column board. Results go to stdout. Context lines, refusals, `hint:` lines, errors and the `viola ui` launch line go to stderr. `--json` prints one JSON document on stdout with typed exit codes (0, 1, 2, 10–14, 20, 21). `viola hook` and `viola mcp` have no human design surface.

### Layout Categories
(The design system has no explicit Layout Categories section. The categories below come from its Navigation Pattern and Component Patterns.)
- **Sticky status header (ATIS / fuel-state)** — used in: web-spa (`<viola-atis>`), cli (the `BAY` line at the top of `viola list`)
- **List / fixed-column table (session racks of strips, grouped WRAPPED vs UNWRAPPED · READ-ONLY, ordered by name, stable slots)** — used in: web-spa (`<viola-session-row>` in `role="table"` racks), cli (`viola list`)
- **Inline handoff marker lines (transfer markers in the rack gap)** — used in: web-spa (`<viola-transfer>`)
- **Log / feed with expandable detail (tower tape of `<details>` lines, capped at 2000 lines, follows the bottom)** — used in: web-spa (`<viola-event-feed>`, `role="log"`)
- **Status / error strips (401 access strip, 503 state-unreadable, tape stopped)** — used in: web-spa
- **Empty and initial-read states (printed phraseology, never spinners or skeletons)** — used in: web-spa (`sessions: no reading yet`, `no wrapped sessions — start one with  viola run <name> -- claude`, `TAPE live since … — no events yet`), cli (`no wrapped sessions  start one: viola run <name> -- claude`)
- **Single result / verb-output lines (readback mirror, wait/last, wheel/link verbs, verify step counter)** — used in: cli
- **Reserved v1.x controls (brake buttons `I HAVE CONTROL` / `UNLINK`, Raised-3 confirmation)** — used in: web-spa (not built in v1)

Navigation: web-spa is one page with no routes. Its vertical order is ATIS header → WRAPPED rack → UNWRAPPED · READ-ONLY rack → TAPE. Width breakpoints: ≥1024px is the primary layout. At 760–1023px each strip wraps to two lines. Below 760px is reserved for a later phone view. Horizontal scroll never happens.

### Brand Identity Anchors
- **Arrival amber** (`--attention` / #D97706) — drives selector for: the cocked strip's `.band` and its `DIALOG` word (`viola-session-row[data-dialog="pending"]`, shifted by `translateX(12px)`), and the CLI `DIALOG` word in `viola list`. It is used for `dialog_pending` only.
- **Departure blue** (`--handoff` / #5B8DB8) — drives selector for: transfer-marker arrows and names (`→ builder`, inbound `← overseer`) in `<viola-transfer>`, and the names in `link` / `unlink` tape lines. It is never used in the CLI.
- **Readback box states** (`viola-readback[data-rb="open|read|refused|unconfirmable"]`, `.rb` 16px square, buff #E6D8AE fill with `RB` in graphite #3B3A36 when read) — drives selector for: the send-line and transfer-marker readback. Each state has a printed word: `open` / `read back` / `unable · <reason> · <detail>` / `unconfirmable`. The CLI mirror is `[RB]` / `[  ]` / `[/ ]`.
- **Session-row state attributes** (`data-liveness="stale"`, `data-dialog="pending"`, `data-wrapped="false"`, `data-live` on SSE arrivals) — drives selector for: stale (lamp-off #9C9278 ink on anthracite), cocked, unwrapped (dashed rail outline, no band) and newly arrived tape lines or markers. All state lives only in `data-*` attributes. `style="…"` and `styleMap` are never used.
- **ARIA roles** (`role="table"` rack, `role="row"` strip, `role="cell"`, `role="columnheader"` captions NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, `role="log"` tape, visually hidden `role="status" aria-live="polite"` region) — drives selector for: role-based E2E queries. The live region announces only strips turning cocked, readback refusals and `TAPE stopped`. The first tab stop is the skip link `skip to tape`. `document.title` becomes `DIALOG <name> · viola` (plus `+N`) while a strip is cocked and `viola` otherwise.
- **Callsign typography** (NAME cell, Bahnschrift stack 600 15px, never text-transformed) — drives selector for: the session-name text match. A `ViolaName` is lower-case `[a-z0-9-]` and appears exactly as typed. Only unwrapped names may end in an ellipsis. In the CLI the NAME column is bold.
