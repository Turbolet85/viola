# a11y-plan · the rows that bind 2026-11-13-epoch-8-cleanup — picked 102 of 241 (75 by every draw) · rule any · pin 885

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — viola writes zero bytes of its own to the terminal; only the child's output reaches it.
contract/tui-passthrough/vt100-feed-panic · viola run TUI passthrough · vt100 feed panic — The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it.
contract/viola-verify/step-line · viola verify output · step line — Each verify step prints a line [NN/NN] … pass|fail.
decision/no-assertions-for-internal-crates · Internal crates not assertable — The internal crates of the modular monolith are not-assertable and carry no assertions. · why: library-only, with no UI surface
decision/no-assertions-for-plugin-and-ipc · Plugin and IPC channel not assertable — The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions. · why: ipc-internal with no UI surface, machine-to-machine traffic only
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
contract/viola-revive-json/start-refusal · viola revive --json output · start refusal — Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/linux-only · Linux only — Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal.
contract/frame-verb-refusal/server-verify-failed · frame verb refusals · server-verify-failed — send, wait, last, answer, pause, release, link and unlink refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
contract/frame-verb-refusal/socket-dir-refused · frame verb refusals · socket-dir-refused — Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
contract/frame-verb-refusal/strict-modes-failed · frame verb refusals · strict-modes-failed — send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
contract/hook-call/server-verify-failed · viola hook event and dialog calls · server not the recorded wrapper — A hook event or dialog call that meets a server that is not the recorded wrapper sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s.
contract/hook-call/socket-dir-refused · viola hook event and dialog calls · socket directory failure — A hook call against a failing socket directory sends no frame and fails open: exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`.
contract/hook-call/strict-modes-failed · viola hook event and dialog calls · strict-modes failure — A hook call refused by the strict-modes check returns exit 0 at once, empty stdout and stderr, under 1.0 s, with no `config.json` read, no stdin read, no frame, no line and no SessionEnd append; a `VIOLA_DIR` naming no instance directory writes nothing.
contract/interrupted-verify/exit · viola verify interrupted · exit 1 — A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
contract/interrupted-verify/stderr · viola verify interrupted · stderr lines — A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
contract/link-unlink/exit-20 · viola link and viola unlink · exit 20 — A pair the wrapper refuses as invalid params reaches `viola link` or `viola unlink` as a fault: exit 20, no hint.
contract/link-unlink/exit-21 · viola link and viola unlink · exit 21 — `viola link` and `viola unlink` refuse with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance, and a server that is not the recorded wrapper.
contract/link-unlink/json-ok · viola link and viola unlink · --json output — Under `--json`, `viola link` and `viola unlink` each print one `{"v":1,"ok":<ok>}` document.
contract/link-unlink/result-line · viola link and viola unlink · result line — `viola link` and `viola unlink` each print one plain-ASCII result line in one write, with no SGR in any mode.
contract/log-identity/process · log identity tags · process — `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run`.
contract/log-identity/service-name · log identity tags · service_name — Records carry `service_name:"viola"` and `version`.
contract/mcp-frame-tool/strict-modes-failed · MCP server frame tools · strict-modes-failed — An MCP frame tool against an instance failing the strict-modes check returns `isError` with code `instance-unreachable`, `detail` `strict-modes-failed`, sends no frame and writes no line.
contract/plugin-install/exit-21 · viola plugin install · exit 21 — The exit-21 refusal of `viola plugin install` is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
contract/revive-human-refusal/pair · viola revive human refusals · unable/hint pair — Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
contract/revive-json-refusal/document · viola revive --json refusals · exit-1 refusal document — Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr.
contract/revive-json-refusal/strict-modes-failed · viola revive --json refusals · strict-modes-failed — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
contract/tui-passthrough/after-feed-panic · TUI passthrough · after a contained vt100 feed panic — The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal.
decision/output-discipline-a11y-fallback · Output-discipline assertions stand in for a11y tools — CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON. · why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
contract/cli-json-failure/send-answer-exit-2 · CLI --json failures · send and answer exit 2 — The `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document.
contract/mcp-frame-tool/server-verify-failed · MCP server frame tools · server-verify-failed — An MCP frame tool (send, wait, last, answer) meeting a server that is not the recorded wrapper returns `isError` with code `instance-unreachable`, `detail` `server-verify-failed` (text `refused: instance-unreachable`) and sends no frame.
contract/mcp-frame-tool/socket-dir-refused · MCP server frame tools · socket-dir-refused — An MCP frame tool against a failing socket directory returns `isError` with code `instance-unreachable`, `detail` `socket-dir-refused`, sends no frame and writes no line.
contract/revive-json-refusal/pinned-hash-mismatch · viola revive --json refusals · pinned-hash-mismatch — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
contract/start-refusal/socket-dir · viola run, revive start and verify refusals · socket directory failure — Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.
contract/tui-passthrough/pump-child-error · TUI passthrough · pump_child error under revive --json — Under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.

## 2-a11y-strategy · ## 2. A11y Strategy
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, budget pause) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it.
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.
contract/cli-refusal/unknown-refusal · CLI refusal output · the `unknown` refusal — The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/client-euid-refusal-test-strategy · Client euid refusal test strategy — No foreign peer is driven: the client euid refusal's decision is tested over injected euids and its real read on Linux, and each client seam's wiring is read only by forced refusals in remove-the-guard runs.
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
decision/link-pair-validated-by-wrapper · Link pairs validated by the wrapper — viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint.
decision/link-result-lines-one-write-no-sgr · Link result lines in one write, no SGR — The link and unlink result lines are each one write with no SGR in any mode. · guard: link_verb_lines_are_the_design_lines_in_one_write
decision/printed-names-are-parsed-arguments · Printed names are the parsed arguments — Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint.
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail.
contract/cli-fixed-message/form · CLI fixed-message refusal · form — viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/instance-unreachable-detail/endpoint-lost · instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open.
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — The name has no live endpoint but its instance directory stands, or the instance is stale.
contract/instance-unreachable-detail/server-verify-failed · instance-unreachable detail · server-verify-failed — The process serving the endpoint is not the recorded wrapper and was not contacted.
contract/instance-unreachable-detail/socket-dir-refused · instance-unreachable detail · socket-dir-refused — The socket directory was refused; the verb sent no frame.
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — The instance failed the strict-modes check; refused before any frame or log, no log line written.
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — The name has no live endpoint and no instance directory (absent, a regular file or a symlink).
contract/instance-unreachable-json/document · exit 21 under --json · document — One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
contract/start-json-refusal/document · run / revive start under --json · refusal document — Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
contract/viola-hook-statusline/exit · viola hook statusline · exit status — Runs the user's own status line command under a 5 s deadline and exits 0 with empty stderr in every case.
contract/viola-link-line/time · viola link line · time — The reply's since as UTC clock time in whole seconds with Z.
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design
contract/fault-body/message · fault body under --json and on MCP · message — A code outside the five known reads the fixed message wrapper fault with null data.
contract/human-fault-line/wrapper-fault · human fault line · wrapper fault — error: wrapper fault  <code>.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/mutation-audit-covers-crate-invariants · Mutation audit covers invariants held in crate tests — Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them.

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content.
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.
decision/statusline-output-passed-byte-for-byte · Statusline output passed byte for byte — viola hook statusline hands the user's own status line output to the child byte for byte and writes nothing to the terminal itself. · guard: passthrough_with_a_statusline_source_adds_no_viola_bytes
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
