# a11y-plan · the rows that bind 2026-11-22-lineage-registry — picked 117 of 238 (85 by every draw) · rule any · pin 4060

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/colour-second-cue-only · Colour only a second cue — Colour is never the sole cue on the CLI: the word always sits beside any SGR.
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw.
contract/cli-output/control-characters · CLI output · C0/C1 control characters — C0/C1 control characters are escaped except \n and \t, which list rows escape too.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/human-keystrokes · viola run TUI passthrough · human keystrokes — Human keystrokes are never blocked, refused or dropped once the wrapped child holds the terminal: queued with the paste in arrival order, they wait behind at most its in-flight chunk and closing marker; the one exception is a start's automatic verify window, before the child starts.
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.
contract/viola-list/unwrapped-separator · viola list output · -- UNWRAPPED - READ-ONLY -- — The separator appears only when unwrapped rows exist.
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
decision/critical-path-human-takes-wheel · Critical path: human takes the wheel — The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
decision/critical-path-send-readback · Critical path: confirmed send and readback — Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
decision/terminal-structural-equivalents · Terminal structural equivalents of ARIA roles — In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator and the --help group labels.
decision/tui-boundary-assertions-only · TUI boundary assertions only — The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
contract/viola-revive-json/start-refusal · viola revive --json output · start refusal — Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/control-character-escaping · C0/C1 control characters escaped — CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two. · why: the list-row exception is ratified (T5) · except: in `viola list` table rows `\n` and `\t` are escaped too
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
contract/cli-help/group-labels · viola --help · group labels — The `--help` verb table is grouped under the labels board / traffic / wheel / handoff / setup.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
contract/log-identity/process · log identity tags · process — `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run`.
contract/log-identity/service-name · log identity tags · service_name — Records carry `service_name:"viola"` and `version`.
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output and, before it, a keyed start's automatic-verify lines and verify's own output; a passed `viola revive` writes no line of its own while the child holds the terminal.
contract/viola-list-board/colour · viola list board · colour — On a terminal `viola list` writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`.
contract/viola-list-board/empty-board · viola list board · no wrapped sessions — With no wrapped row, the board prints the BAY line, the caption row and `no wrapped sessions  start one: viola run <name> -- claude`.
contract/viola-list-board/order · viola list board · structure — The board is the BAY context line, the uppercase caption row and the wrapped rows, then the unwrapped read-only rows behind their separator.
contract/viola-list-board/unwrapped-separator · viola list board · -- UNWRAPPED - READ-ONLY -- — The separator and unwrapped rows print only when the one `claude agents --json` read lists a session no wrapped row joins; unwrapped rows show the escaped `sessionId` as NAME and `n/a` in WHEEL, DIALOG and CLI; a failed, oversize or malformed read adds no separator and no unwrapped row.
contract/revive-human-refusal/pair · viola revive human refusals · unable/hint pair — Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
contract/revive-json-refusal/document · viola revive --json refusals · exit-1 refusal document — Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr.
contract/revive-json-refusal/socket-dir-refused · viola revive --json refusals · socket-dir-refused — One exit-1 document with `socket-dir-refused` its code on stdout, in place of the human pair, before any snapshot, bind or child spawn, holding no path, pid or uid, with stderr empty and no hint.
contract/revive-list/human-rows · viola revive --list · human rows — Without `--json`, `viola revive <name> --list` prints static ASCII rows `<ts>  <cause>  <id>`, with no header, no colour and no ESC byte.
contract/revive-list/json · viola revive --list · --json output — `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order.
contract/start-refusal/socket-dir · viola run, revive start and verify refusals · socket directory failure — Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/human-keystrokes-never-blocked · Human keystrokes never blocked — A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current paste's in-flight chunk and closing marker. · except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
decision/word-beside-every-sgr · Word beside every SGR span — Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/c0-c1-escaped · C0 and C1 escaped in CLI output — C0 and C1 control characters in CLI output are escaped. · guard: literal pins over std::process::Command on ubuntu-latest
decision/cli-structural-landmark-equivalents · CLI structural equivalents of landmarks — The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels.
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed. · except: The checks hold from the child's start: before it, a keyed start's automatic-verify lines and verify's own output reach the terminal, and a key waits in the terminal's own buffer.
decision/send-refused-while-human-typing · Send refused while the human types — When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code. · guard: std::process::Command asserts
decision/stale-row-dim · Dim only on stale rows — Dim SGR is applied only to stale rows, beside the word that carries the state. · guard: unit-tier tests and outer-PTY tests on ubuntu-latest
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest · except: Before the child starts, a keyed start's automatic-verify lines and verify's own output reach the human's terminal.
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/control-bytes-escaped · Control bytes printed escaped — Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line. · why: security control for C0/C1 escaping, ratified for list rows · guard: escape_row_shows_every_control_character_as_hex_text
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
decision/sgr-beside-its-word · SGR only beside its word — Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured. · guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal
contract/cli-exit-codes/10 · CLI exit codes · 10 — human-typing: the human holds the wheel, so the automated send is refused with nothing typed.
contract/cli-help/usage-line · viola --help · usage line — `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the group rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, then `Options:`, plain ASCII.
contract/viola-list-board/caption · viola list board · caption order — The caption order is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, reused by the terminal fleet console.

## 5-keyboard-navigation · ## 5. Keyboard Navigation
decision/tui-terminal-belongs-to-child · Tui terminal belongs to the child — On the tui, the terminal belongs to the wrapped child process, not to viola.
decision/cli-no-interactive-input · No interactive CLI input — The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/dialog-amber-sgr · Amber only on the DIALOG word — Amber SGR is applied only to the `DIALOG` word.
decision/name-bold-sgr · Bold only on NAME values — Bold SGR is applied only to NAME values.
decision/name-truncation · NAME truncation only on a narrow terminal — An unwrapped NAME is cut with `...` only on a terminal too narrow and never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` are never cut.
decision/unable-fail-never-coloured · Refusal and failure words never coloured — The words `unable` and `fail` are never coloured.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/colour-decision-inputs · Colour decision inputs — The colour decision reads NO_COLOR (present, empty included), TERM (exactly dumb; 256color), COLORTERM and whether stdout is a terminal, and picks a truecolor, 256 or 16 depth.
decision/state-carried-by-printed-word · State carried by its printed word — Each board state is carried by its printed word (stale, DIALOG <kind>, expired, unverified-cli, idle, busy, n/a), never by colour alone.
decision/unwrapped-name-truncation · Unwrapped NAME truncation — On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus ..., never inside an escape; a wrapped NAME is never cut, and piped output and --json keep every name whole. · guard: cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it
contract/viola-list-stale-row/escape-sequences · viola list stale row on a terminal · escape sequences — Exactly DIM, BOLD, RESET, DIM, RESET, with the word stale printed beside the dim.

## 7-screen-reader-support · ## 7. Screen Reader Support
decision/wrapped-tui-reads-as-unwrapped · Wrapped TUI reads as unwrapped — The wrapped `claude` TUI must read to a screen reader the same as unwrapped. · why: Must-Work O1 · except: in an untrusted folder a wrapped `claude` opens on its session box, viola's start having written the CLI's trust record (the flag alone) and typed no key; an unwrapped one opens on the trust form

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.
contract/viola-list-caption/order · viola list caption · order — The caption order is `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`, pinned by the `CAPTION` literal.
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release
decision/one-hint-per-code · One hint per code — No two codes share a hint. · guard: no_two_codes_share_a_hint
decision/printed-names-are-parsed-arguments · Printed names are the parsed arguments — Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
contract/send-refusal/input-not-ready · viola send refusal · input-not-ready — A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
contract/send-refusal/no-prompt-submitted · viola send refusal · no-prompt-submitted — A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
contract/start-json-refusal/document · run / revive start under --json · refusal document — Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
decision/claude-agents-read-fails-open · claude agents read fails open — A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's preflight.
decision/link-pair-validated-by-wrapper · Link pairs validated by the wrapper — viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint.
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design
contract/cli-exit-codes/2 · CLI exit codes · 2 — A usage error, or an unusable answer response or send text written as one fixed stderr line with no document.
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint.
contract/cli-fixed-message/form · CLI fixed-message refusal · form — viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there.
contract/start-holder-refusal/already-live · start against a holder, exit 1 · already-live — The holder is live, or a fresh replayed beat was read.
contract/start-holder-refusal/stale-heartbeat · start against a holder, exit 1 · stale-heartbeat — The holder is stale; stale heartbeat is exit 1's cause alone.

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/list-captions-fixed · Board captions fixed by their literal — The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4).
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured.
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text.
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or KR resize
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/tui-boundary-driver-pin · TUI boundary driver — The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content.
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run. · except: A keyed start's automatic-verify lines and verify's own output, which precede the child's bytes.
decision/stdin-classifier-non-editing-list · Closed non-editing stdin list — Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel. · why: ratified, the exact grammars chosen after a narrower option was shown

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
