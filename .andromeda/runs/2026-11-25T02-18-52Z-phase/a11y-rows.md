# a11y-plan · the rows that bind 2026-11-25-placing-without-moving-the-view — picked 101 of 244 (62 by every draw) · rule any · pin 4974

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/a11y-inside-existing-suites · A11y assertions inside existing suites — A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips.
decision/colour-second-cue-only · Colour only a second cue — Colour is never the sole cue on the CLI: the word always sits beside any SGR.
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
decision/critical-path-dialog-answer · Critical path: dialog pending, then answer — A pending dialog answered with viola answer (question, permission, plan, and a question first raised by PermissionRequest), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/viola-revive-json/pump-child-error · viola revive --json output · pump_child error — A pump_child error prints no document: the child has held the terminal, and viola writes nothing past it.
contract/viola-revive-json/start-refusal · viola revive --json output · start refusal — Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error.
contract/viola-up-output/instance-line · viola up output · instance line — `viola up` prints one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, with no SGR, each line in one write.
contract/viola-verify/step-line · viola verify output · step line — Each verify step prints a line [NN/NN] … pass|absent|fail; absent is printed only on the browser-tools-prompt row, when no run met the prompt.
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
decision/manual-keyboard-pass · Manual keyboard pass as supplement — A manual keyboard pass over CLI and TUI supplements the automated keystroke boundary assertion.
decision/manual-orca-pass · Manual Orca screen-reader pass — A manual Orca screen-reader pass on Section 4 P4 and P6, founder-owned, supplements the automated assertions. · why: no blind or low-vision target users are named, so screen-reader priority is scoped to CLI/TUI linearity, not a full assistive-tech matrix
contract/tui-passthrough/human-keystrokes · viola run TUI passthrough · human keystrokes — Human keystrokes are never blocked, refused or dropped once the wrapped child holds the terminal: queued with the paste in arrival order, they wait behind at most its in-flight chunk and closing marker; the one exception is a start's automatic verify window, before the child starts.
contract/tui-passthrough/vendor-content · viola run TUI passthrough · vendor content — The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/linux-only · Linux only — Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
decision/manual-a11y-pass-gates-nothing · Manual screen-reader pass gates nothing — The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it.
decision/output-discipline-a11y-fallback · Output-discipline assertions stand in for a11y tools — CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON. · why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
contract/interrupted-verify/exit · viola verify interrupted · exit 1 — A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
contract/interrupted-verify/stderr · viola verify interrupted · stderr lines — A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
contract/revive-human-refusal/pair · viola revive human refusals · unable/hint pair — Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
contract/revive-json-refusal/document · viola revive --json refusals · exit-1 refusal document — Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output and, before it, a keyed start's automatic-verify lines and verify's own output; a passed `viola revive` writes no line of its own while the child holds the terminal.
contract/tui-passthrough/pump-child-error · TUI passthrough · pump_child error under revive --json — Under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` or `viola adopt` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
contract/start-refusal/socket-dir · viola run, revive start and verify refusals · socket directory failure — Over a failing socket directory, `viola run` and a `viola revive` or `viola adopt` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/human-keystrokes-never-blocked · Human keystrokes never blocked — A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current paste's in-flight chunk and closing marker. · except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola adopt`'s three own refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
decision/sgr-beside-its-word · SGR only beside its word — Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured. · guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/unable-fail-never-coloured · Refusal and failure words never coloured — The words `unable` and `fail` are never coloured.

## 7-screen-reader-support · ## 7. Screen Reader Support
decision/wrapped-tui-reads-as-unwrapped · Wrapped TUI reads as unwrapped — The wrapped `claude` TUI must read to a screen reader the same as unwrapped. · why: Must-Work O1 · except: in an untrusted folder a wrapped `claude` opens on its session box, viola's start having written the CLI's trust record (the flag alone) and typed no key; an unwrapped one opens on the trust form

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
decision/time-bounds-never-on-human · Time bounds never fall on the human — Eight time bounds viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) fall on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by them.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer`, `viola send` and `viola open` exit-2 paths, which print a fixed stderr line and no document.
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/hyprctl-read-deadline · Each compositor read is bounded at 5 s — Each compositor read through the Hyprland seam is bounded at 5 s a reply; it falls on viola's own start or verb (the window step, viola open, viola focus), never on a key typed into a session, and never moves the wheel.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer, viola send and viola open exit-2 paths write a fixed line to stderr whatever --json says, and no document
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release
decision/one-hint-per-code · One hint per code — No two codes share a hint. · guard: no_two_codes_share_a_hint
decision/open-live-wait-bound · viola open's live wait is bounded at 30 s — viola open waits at most 30 s for the instance it started to come live, then reads not-live; the wait holds only the verb's own return, never a key typed into a session, and never moves the wheel.
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run, revive and adopt start.
decision/start-verify-stop-grace-off-the-human · Start's verify stop grace never falls on the human — The start's verify stop grace, the wait before a stopped start's verify child is killed, falls on viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs, link/unlink and focus, exit 1 for run, revive's and adopt's start and verify.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Eight time bounds (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) fall on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
decision/window-wait-bound · The window step's wait is bounded at 5 s — A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list; the wait falls on viola's own start, never on a key typed into a session, and never moves the wheel.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's and adopt's start, mcp and hook stay silent there.
contract/start-json-refusal/document · run / revive start under --json · refusal document — Every refusal of the start revive and adopt share with run, and adopt's own strict-modes-failed, session-live and not-resumed, gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
decision/dialog-hold-bounds-the-driver · Dialog hold bounds the driver — A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold.
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
contract/cli-exit-codes/14 · CLI exit codes · 14 — unknown: a refusal whose word is outside the closed refusal list.
contract/cli-fixed-message/form · CLI fixed-message refusal · form — viola revive's exit-1 refusals, viola adopt's strict-modes-failed, session-live and not-resumed refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
contract/start-holder-refusal/already-live · start against a holder, exit 1 · already-live — The holder is live, or a fresh replayed beat was read.
contract/start-holder-refusal/stale-heartbeat · start against a holder, exit 1 · stale-heartbeat — The holder is stale; stale heartbeat is exit 1's cause alone.
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.

## 9-ci-integration · ## 9. CI Integration
decision/a11y-cases-inside-coverage-suite · A11y cases run inside the coverage suite — The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict. · why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
decision/zero-flakiness · Zero flakiness, no retries — Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed. · why: retry-once policies mask real signals (the agent-driven principle)

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/compiled-shapes-over-signatures · Compiled shapes over the screen signatures — The compiled shapes the rendered `claude` TUI screen is parsed for over its compiled signatures are a modal label row's option shape and the box's rows below it; no row text is kept.
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured.
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or KR resize
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text.
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.

## ci-integration · key of ## 3. A11y Assertion Harness Contract
decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run. · except: A keyed start's automatic-verify lines and verify's own output, which precede the child's bytes.
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.

## structured-violation-json-schema · key of ## 3. A11y Assertion Harness Contract
contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].

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
