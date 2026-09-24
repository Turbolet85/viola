## 3. Design System Excerpt

### Surfaces
- **web-spa: `viola ui`** (desktop-browser SPA, loopback only at `http://127.0.0.1:47319`). A single page with no routes, built from Lit 3.3.3 light-DOM `viola-*` elements that are vendored and embedded via `include_bytes!`, with no JS build step. It runs on Windows 10/11 (Edge/Chrome) first, plus macOS and Linux (Chromium, Firefox, Safari 17.5+). Headless-browser checks run in CI on ubuntu. It shows session strips, transfer markers and an SSE-fed event tape. Telemetry hint: browser OTel SDK + web-vitals is the candidate, but it must be vendored and same-origin. The CSP is `default-src 'none'; script-src 'self'; style-src 'self'; font-src 'none'; require-trusted-types-for 'script'`, with no CDN assets, inline script or `innerHTML`. `/ready` (`ok` / `unavailable` / `error`) and `/health` are probe routes for tests and obs, not shown on the page.
- **cli: `viola` binary** (cli; Rust stable, clap 4.6.7). Used by a human on a TTY and by LLM drivers via `--json` / MCP. Results go to stdout; context lines, refusals, `hint:` lines, errors and the `viola ui` launch line go to stderr. `--json` prints one JSON document on stdout with a typed exit code. `viola run` prints nothing while the child `claude` TUI runs. `viola hook` never writes stderr and always exits 0. `viola mcp` writes only MCP frames to stdout. Stack traces never print, and full detail goes only to `instances/<name>/diagnostics/`. Telemetry hint: stdout/stderr only, no frontend.

### Loading / Error / Empty State Patterns
- **Initial read (before `/api/sessions` answers, typically under 100ms)**: page-replace text. Each rack shows `sessions: no reading yet` and the ATIS fields show `unknown`. Skeletons and spinners are banned. Telemetry hook: a span from page load to the first `/api/sessions` response (TTI proxy).
- **Failed reading (`unknown`)**: inline word in the field cell. Examples: STATUS `unknown` when `claude agents --json` is missing or fails; CLI version `unknown`; budget `5H unknown  7D unknown  read unknown`. `n/a` means absent by contract on unwrapped rows, not a failure. Telemetry hook: a counter for `unknown` per field.
- **Empty rack**: inline `no wrapped sessions — start one with  viola run <name> -- claude`. The CLI prints `no wrapped sessions  start one: viola run <name> -- claude`. Telemetry hook: a counter for empty-state occurrences.
- **Empty tape**: inline `TAPE live since 19:40:02Z — no events yet`. The SSE stream starts at each file's current end. Telemetry hook: a span from SSE open to the first event.
- **Tape connection (SSE `EventSource`)**: an inline ATIS cell showing `TAPE connecting`, then `TAPE live since <time>`, or `TAPE stopped · viola ui not answering` (boxed). Telemetry hook: a span around connecting → open, and a counter for transitions to closed with a cause attribute.
- **Readback (send lifecycle)**: an inline 16px box plus a word at the far right of tape send lines and transfer markers. Changes are instant. The four states are:
  - `open`, from CL-1 `send-issued`.
  - `read back`, when the matching `prompt-submitted` with origin `driver` arrives, or `session-start` cause `clear` for `/clear`.
  - `unable · <reason> · <detail>`, from CL-1 `send-refused`.
  - `unconfirmable`, from an `ok` send with `confirmed:false`.

  The CLI mirrors these as `[  ]` / `[RB]` / `[/ ]`. Until CL-1 lands, the page shows only `read` boxes and prints `unknown` in the marker's last-send cell. Telemetry hook: a send → readback latency span, a counter per terminal state, and the refusal reason and detail as attributes.
- **Stale liveness**: an instant, inline lamp-off dimming of the whole strip; the strip keeps its slot. The CLI row is SGR-dimmed with `stale` in the LIVE column. Telemetry hook: a counter for live → stale transitions.
- **Budget expired / paused**: inline ATIS words. `5H 62 % expired` appears once `resets_at` has passed. A boxed `budget-paused` replaces `gate open`, and the reading age `read 4m ago` is always shown. Telemetry hook: a counter for gate transitions and a gauge for reading age.
- **Skipped counts (unknown kinds / unknown fields / torn lines)**: inline ATIS `skipped 0 · 0 · 0`, boxed when nonzero and never auto-cleared. The tape shows `skipped  1 unknown kind` in place. Telemetry hook: counters for unknown kinds, unknown fields and torn ndjson lines.
- **Tape line cap (2000 DOM lines)**: an inline notice `older lines trimmed from view: N — the full tape is events.ndjson`. Telemetry hook: a counter for trim events.
- **Tape follow rule**: a static inline line `N new lines below` when the reader is not at the bottom. Telemetry hook: none specified.
- **CLI `viola wait`**: a static stderr line `waiting: <name>` (TTY only), then one result line: `turn-ended …`, `question … dialog N …` or `timed out  <name>  30000 ms`. There is no spinner. Telemetry hook: a span around the wait with the outcome as an attribute.
- **CLI `viola verify`**: step-counter lines `[03/14] S3 … pass` / `fail`, then a summary `stamped 2.1.280  14 pass  0 fail`. Telemetry hook: a span per step and pass/fail counters.

### User-Facing Error Surfaces
- **401 `unauthorized` access strip** (in-page, full-width strip in the rack position). Appears for: a missing or invalid GUI session. It prints `UNAUTHORIZED  this page has no session for 127.0.0.1:47319`, then `open the launch line printed by "viola ui" at start, or restart viola ui`. Recovery affordance: reopen the launch line or restart; there is no retry control or input field. It never shows the token, launch URL, `?t=` query, `ui/<port>.url` path or cookie, and never reads `location.search`. Feedback widget candidate: no, because v1 has no page controls.
- **503 `state-unreadable` rack strip** (in-page). Appears for: viola home could not be read. It prints `unable · state-unreadable  viola home could not be read`. Recovery affordance: none. Feedback widget candidate: no.
- **404 `not-found` / 405 `method-not-allowed` rack strips** (in-page, defensive only; the page makes GET requests to its own routes only). They print `unable · not-found` / `unable · method-not-allowed`. Recovery affordance: none. Feedback widget candidate: no.
- **403 `host-not-allowed`** (dedicated error page: the browser shows the raw Problem JSON). Appears for: a failed Host check, so the page never loads. Recovery affordance: none. Feedback widget candidate: no.
- **`Problem::cross-origin-forbidden` (403)**: reserved for the v1.x brake controls and not used in v1.
- **Tape stopped** (in-page: a boxed ATIS cell `TAPE stopped · viola ui not answering` plus a strip at the top of the tape). Appears for: the SSE connection closing or the server not answering. Strips keep their last values and the ATIS age keeps counting. It is announced through a polite `aria-live` region. Recovery affordance: none. Feedback widget candidate: no.
- **Readback refusal** (in-page, inline on the tape send line and the transfer-marker box). Appears for: a typed refusal shown as `unable · <reason> · <detail>`, announced via `aria-live`.
  - Reasons: `human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`, `unknown` (its detail is opaque text).
  - Details: `manual-pause`, `five-hour`, `seven-day`, `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`.

  Recovery affordance: none. Feedback widget candidate: no.
- **CLI refusals** (stderr line `unable  <name>  <reason>  <detail>` followed by one `hint:` line keyed by the reason). Exit codes:
  - 10 `human-typing`
  - 11 `budget-paused`
  - 12 `unverified-cli`
  - 13 `not-delivered`
  - 14 `unknown` (no hint)
  - 21 `instance-unreachable`
  - 1 `unable: <name> is already live`, followed by `hint: viola list`

  Under `--json` a refusal is `{"v":1,"refusal":…,"detail":…}` on stdout, with no hint. Hints never quote sent or upstream text. Recovery affordance: the hint names the next command (`viola release` / `wait` / `verify`). Feedback widget candidate: no.
- **CLI faults** (stderr line). Exit 1 prints `error: internal error` (fixed message, no chain, no paths, no hint). Exit 20 prints `error: wrapper fault  <code>`, or `{"v":1,"error":"wrapper-fault","detail":{…}}` under `--json`. There are no stack traces, and detail goes only to `instances/<name>/diagnostics/`. Errors never contain upstream text, paths, pids or anyhow chains with serde sources. Recovery affordance: none. Feedback widget candidate: no.
- **CLI Git Bash path warning** (stderr line `warning: argument looks like a Git Bash rewritten path`). Recovery affordance: none. Feedback widget candidate: no.
- **Redaction rules for every error surface**: never display or log the GUI token, launch URL, `?t=` query, `.url` path or cookie values. The one exception is the single unstyled `viola ui` stderr launch line. Never render `viola_home` from `/api/info`, because it holds the OS username, the only PII in scope.
