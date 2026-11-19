# a11y-plan · the rows that bind 2026-11-19-the-2-1-328-recorded-fixture-set — picked 77 of 235 (59 by every draw) · rule any · pin 2980

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips.
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
decision/critical-path-dialog-answer · Critical path: dialog pending, then answer — A pending dialog answered with viola answer (question, permission, plan), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
decision/critical-path-human-takes-wheel · Critical path: human takes the wheel — The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
decision/critical-path-send-readback · Critical path: confirmed send and readback — Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/single-vendor-content-boundary · Single vendor-content boundary — The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone.
decision/tui-boundary-assertions-only · TUI boundary assertions only — The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/entry · viola run TUI passthrough · entry — The passthrough is entered by viola run and by a viola revive whose preflight passed.
contract/tui-passthrough/focus-mouse-resize-sequences · viola run TUI passthrough · focus, mouse and resize sequences — Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes.
contract/tui-passthrough/human-keystrokes · viola run TUI passthrough · human keystrokes — Human keystrokes are never blocked, refused or delayed past the current atomic paste.
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/tui-passthrough/vendor-content · viola run TUI passthrough · vendor content — The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — viola writes zero bytes of its own to the terminal; only the child's output reaches it.
contract/viola-verify/step-line · viola verify output · step line — Each verify step prints a line [NN/NN] … pass|absent|fail; absent is printed only on the browser-tools-prompt row, when no run met the prompt.
decision/no-assertions-for-plugin-and-ipc · Plugin and IPC channel not assertable — The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions. · why: ipc-internal with no UI surface, machine-to-machine traffic only
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw.
contract/tui-passthrough/vt100-feed-panic · viola run TUI passthrough · vt100 feed panic — The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
contract/interrupted-verify/exit · viola verify interrupted · exit 1 — A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
contract/interrupted-verify/stderr · viola verify interrupted · stderr lines — A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal.
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
contract/tui-passthrough/after-feed-panic · TUI passthrough · after a contained vt100 feed panic — The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/human-keystrokes-never-blocked · Human keystrokes never blocked — A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current atomic paste.
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
decision/send-refused-while-human-typing · Send refused while the human types — When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code. · guard: std::process::Command asserts
decision/tui-boundary-only · Tui accessibility is boundary-only — The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
contract/cli-exit-codes/10 · CLI exit codes · 10 — human-typing: the human holds the wheel, so the automated send is refused with nothing typed.
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.

## 5-keyboard-navigation · ## 5. Keyboard Navigation
decision/tui-terminal-belongs-to-child · Tui terminal belongs to the child — On the tui, the terminal belongs to the wrapped child process, not to viola.

## 7-screen-reader-support · ## 7. Screen Reader Support
decision/wrapped-tui-reads-as-unwrapped · Wrapped TUI reads as unwrapped — The wrapped `claude` TUI must read to a screen reader the same as unwrapped. · why: Must-Work O1 · except: in an untrusted folder a wrapped `claude` opens on its session box, viola's start having written the CLI's trust record (the flag alone) and typed no key; an unwrapped one opens on the trust form

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/dialog-hold-bounds-the-driver · Dialog hold bounds the driver — A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold.
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/send-refusal/no-prompt-submitted · viola send refusal · no-prompt-submitted — A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — Whole seconds 60 to 3600, default 60, read once at the wrapper's start; any other value gives 60 and one parse-rejected.
contract/event-line/dialog-expired · the event line · dialog-expired — Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait.
contract/mcp-wait-tool/timeout-ms · MCP wait tool · timeout_ms — Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s.
contract/send-refusal/input-not-ready · viola send refusal · input-not-ready — A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
contract/viola-wait/timed-out · viola wait · timed_out — viola wait returns timed_out true only for a timeout the caller passes in.

## 9-ci-integration · ## 9. CI Integration
decision/a11y-ci-github-actions · A11y checks in the GitHub Actions push/PR workflow — A11y checks run in the one GitHub Actions push/PR workflow on an `ubuntu-latest` matrix; the scheduled nightly workflow carries no a11y step.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
decision/zero-flakiness · Zero flakiness, no retries — Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed. · why: retry-once policies mask real signals (the agent-driven principle)

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only standing live rig reads the trust selector's place, the input box as empty, holds-text or absent, and modal dialogs, as closed words only, its raw frames kept local and gitignored
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing
decision/no-soft-a11y-slo · No soft a11y SLO — An a11y SLO is never soft: it triggers a build or deploy failure on its threshold.
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text.

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.
decision/tui-boundary-driver-pin · TUI boundary driver — The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins.

## ci-integration · key of ## 3. A11y Assertion Harness Contract
decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content.
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.
decision/stdin-classifier-non-editing-list · Closed non-editing stdin list — Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel. · why: ratified, the exact grammars chosen after a narrower option was shown
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run.

# a11y-plan · its absences, names and overviews, whole

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
absence/internationalization — Internationalization: no i18n, RTL or lang conventions
absence/touch-surface — A touch surface
absence/web-or-graphical-frontend — A web or graphical frontend

## 5-keyboard-navigation · ## 5. Keyboard Navigation
N/A — viola has no web or graphical frontend; the terminal fleet console's keyboard reach is the route entry «Fleet a11y verification»'s.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
N/A — viola has no web or graphical frontend, so no page token pair or layout to verify.

## 7-screen-reader-support · ## 7. Screen Reader Support
N/A — viola has no web or graphical frontend.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
absence/user-time-limit — A time limit on the user, and any timeout-extension component

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
absence/rendered-page — A rendered page, and with it any browser a11y tooling

## contrast-verification-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## focus-management-test-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## structured-violation-json-schema · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## wcag-criteria-mapping · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.
