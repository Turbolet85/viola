# a11y-plan · the rows that bind 2026-11-27-epoch-5-cleanup-2 — picked 41 of 244 (17 by every draw) · rule any · pin 5179

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/viola-up-output/instance-line · viola up output · instance line — `viola up` prints one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, with no SGR, each line in one write.
decision/no-assertions-for-internal-crates · Internal crates not assertable — The internal crates of the modular monolith are not-assertable and carry no assertions. · why: library-only, with no UI surface
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/entry · viola run TUI passthrough · entry — The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.
contract/viola-switch-output/result-line · viola switch output · result line — `viola switch` prints one line, `<name>  switched  <model> <effort>`, on stdout in plain ASCII with no SGR.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
contract/log-identity/process · log identity tags · process — `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run` and a `viola adopt` start.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output and, before it, a keyed start's automatic-verify lines and verify's own output; a passed `viola revive` writes no line of its own while the child holds the terminal.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/human-keystrokes-never-blocked · Human keystrokes never blocked — A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current paste's in-flight chunk and closing marker. · except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola adopt`'s three own refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed. · except: The checks hold from the child's start: before it, a keyed start's automatic-verify lines and verify's own output reach the terminal, and a key waits in the terminal's own buffer.
decision/send-refused-while-human-typing · Send refused while the human types — When the human takes the wheel, `send` and `switch` are refused with `human-typing` on stderr and a typed exit code. · guard: std::process::Command asserts
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest · except: Before the child starts, a keyed start's automatic-verify lines and verify's own output reach the human's terminal; after the child ends, a not-resumed viola adopt's refusal pair or document does.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
contract/cli-exit-codes/10 · CLI exit codes · 10 — human-typing: the human holds the wheel, so the automated send or switch is refused with nothing typed and no child replaced.

## 5-keyboard-navigation · ## 5. Keyboard Navigation
decision/tui-terminal-belongs-to-child · Tui terminal belongs to the child — On the tui, the terminal belongs to the wrapped child process, not to viola.

## 7-screen-reader-support · ## 7. Screen Reader Support
decision/wrapped-tui-reads-as-unwrapped · Wrapped TUI reads as unwrapped — The wrapped `claude` TUI must read to a screen reader the same as unwrapped. · why: Must-Work O1 · except: in an untrusted folder a wrapped `claude` opens on its session box, viola's start having written the CLI's trust record (the flag alone) and typed no key; an unwrapped one opens on the trust form

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/time-bounds-never-on-human · Time bounds never fall on the human — Eight time bounds viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) fall on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by them.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer`, `viola send` and `viola open` exit-2 paths, which print a fixed stderr line and no document.
contract/cli-refusal/unknown-refusal · CLI refusal output · the `unknown` refusal — The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/hyprctl-read-deadline · Each compositor read is bounded at 5 s — Each compositor read through the Hyprland seam is bounded at 5 s a reply; it falls on viola's own start or verb (the window step, viola open, viola focus), never on a key typed into a session, and never moves the wheel.
decision/respawn-reap-bound · A switch's reap wait is bounded at 10 s — A `viola switch` waits at most 10 s (RESPAWN_REAP_BOUND) for the old child to exit after its hang-up; a child not exited is not replaced, and the switch is refused not-delivered child-not-ended, exit 13. The bound falls on the wrapper, never on the human.
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Eight time bounds (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) fall on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
decision/window-wait-bound · The window step's wait is bounded at 5 s — A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list; the wait falls on viola's own start, never on a key typed into a session, and never moves the wheel.
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: a frame verb, viola focus before any compositor read, or viola up on the board's strict-modes refusal, was refused before reaching the instance, the cause in detail.
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run, revive and adopt start.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs, link/unlink and focus, exit 1 for run, revive's and adopt's start and verify.
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — The instance failed the strict-modes check; refused before any frame or log, no log line written.
contract/instance-unreachable-json/document · exit 21 under --json · document — One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's and adopt's start, mcp and hook stay silent there.
contract/start-json-refusal/document · run / revive start under --json · refusal document — Every refusal of the start revive and adopt share with run, and adopt's own strict-modes-failed, session-live and not-resumed, gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/mutation-audit-covers-crate-invariants · Mutation audit covers invariants held in crate tests — Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them.

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
