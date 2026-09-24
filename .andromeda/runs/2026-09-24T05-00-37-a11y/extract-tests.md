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
