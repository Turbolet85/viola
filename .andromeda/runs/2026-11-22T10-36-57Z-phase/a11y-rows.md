# a11y-plan · the rows that bind 2026-11-22-paste-write-bounded — picked 84 of 236 (61 by every draw) · rule any · pin 3869

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/critical-path-human-takes-wheel · Critical path: human takes the wheel — The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
decision/critical-path-send-readback · Critical path: confirmed send and readback — Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/single-vendor-content-boundary · Single vendor-content boundary — The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/focus-mouse-resize-sequences · viola run TUI passthrough · focus, mouse and resize sequences — Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes.
contract/tui-passthrough/human-keystrokes · viola run TUI passthrough · human keystrokes — Human keystrokes are never blocked, refused or delayed past the current atomic paste once the wrapped child holds the terminal; the one exception is a start's automatic verify window, before the child starts.
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/tui-passthrough/vendor-content · viola run TUI passthrough · vendor content — The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
decision/no-assertions-for-internal-crates · Internal crates not assertable — The internal crates of the modular monolith are not-assertable and carry no assertions. · why: library-only, with no UI surface
decision/no-assertions-for-plugin-and-ipc · Plugin and IPC channel not assertable — The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions. · why: ipc-internal with no UI surface, machine-to-machine traffic only
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/tui-boundary-assertions-only · TUI boundary assertions only — The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
contract/tui-passthrough/vt100-feed-panic · viola run TUI passthrough · vt100 feed panic — The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
contract/tui-passthrough/after-feed-panic · TUI passthrough · after a contained vt100 feed panic — The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal.
decision/linux-only · Linux only — Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
contract/frame-verb-refusal/server-verify-failed · frame verb refusals · server-verify-failed — send, wait, last, answer, pause, release, link and unlink refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
contract/frame-verb-refusal/socket-dir-refused · frame verb refusals · socket-dir-refused — Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
contract/frame-verb-refusal/strict-modes-failed · frame verb refusals · strict-modes-failed — send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/human-keystrokes-never-blocked · Human keystrokes never blocked — A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current atomic paste. · except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
decision/send-refused-while-human-typing · Send refused while the human types — When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code. · guard: std::process::Command asserts
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
decision/tui-boundary-only · Tui accessibility is boundary-only — The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
contract/cli-exit-codes/10 · CLI exit codes · 10 — human-typing: the human holds the wheel, so the automated send is refused with nothing typed.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json

## 5-keyboard-navigation · ## 5. Keyboard Navigation
decision/tui-terminal-belongs-to-child · Tui terminal belongs to the child — On the tui, the terminal belongs to the wrapped child process, not to viola.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/unable-fail-never-coloured · Refusal and failure words never coloured — The words `unable` and `fail` are never coloured.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release
decision/one-hint-per-code · One hint per code — No two codes share a hint. · guard: no_two_codes_share_a_hint
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, MCP call deadline, dialog hold, readiness gate, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint.
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
contract/instance-unreachable-detail/call-deadline · instance-unreachable detail · call-deadline — An MCP frame tool call (send, wait, last, answer) outlived the MCP call deadline (55 s) and ended; the outcome of a send is unknown. Written by the MCP frame tools alone, never by the CLI, whose exit-21 cause set stays six.
contract/instance-unreachable-json/document · exit 21 under --json · document — One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
contract/send-refusal/input-not-ready · viola send refusal · input-not-ready — A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
contract/send-refusal/no-prompt-submitted · viola send refusal · no-prompt-submitted — A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was budget-paused) and never reused; an older wrapper's budget-paused refusal reads as unknown, exit 14.
contract/cli-exit-codes/14 · CLI exit codes · 14 — unknown: a refusal whose word is outside the closed refusal list.
contract/human-fault-line/wrapper-fault · human fault line · wrapper fault — error: wrapper fault  <code>.
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/mutation-audit-covers-crate-invariants · Mutation audit covers invariants held in crate tests — Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them.
decision/zero-flakiness · Zero flakiness, no retries — Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed. · why: retry-once policies mask real signals (the agent-driven principle)

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured.
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or KR resize
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text.
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.

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
