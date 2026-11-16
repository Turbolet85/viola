# a11y-plan · the rows that bind 2026-11-16-wait-wake-condition — picked 40 of 235 (22 by every draw) · rule any · pin 1668

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips.
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw.
contract/viola-verify/step-line · viola verify output · step line — Each verify step prints a line [NN/NN] … pass|fail.
contract/viola-wait/waiting-line · viola wait output · waiting: <name> — The waiting line is written once on stderr, on a TTY only.
decision/critical-path-dialog-answer · Critical path: dialog pending, then answer — A pending dialog answered with viola answer (question, permission, plan), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
decision/no-assertions-for-internal-crates · Internal crates not assertable — The internal crates of the modular monolith are not-assertable and carry no assertions. · why: library-only, with no UI surface
decision/no-assertions-for-plugin-and-ipc · Plugin and IPC channel not assertable — The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions. · why: ipc-internal with no UI surface, machine-to-machine traffic only
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
contract/cli-output/control-characters · CLI output · C0/C1 control characters — C0/C1 control characters are escaped except \n and \t, which list rows escape too.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/control-character-escaping · C0/C1 control characters escaped — CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two. · why: the list-row exception is ratified (T5) · except: in `viola list` table rows `\n` and `\t` are escaped too
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/c0-c1-escaped · C0 and C1 escaped in CLI output — C0 and C1 control characters in CLI output are escaped. · guard: literal pins over std::process::Command on ubuntu-latest
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/control-bytes-escaped · Control bytes printed escaped — Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line. · why: security control for C0/C1 escaping, ratified for list rows · guard: escape_row_shows_every_control_character_as_hex_text
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read) falls on the driver or viola's child; no key is blocked, refused or delayed by it.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
contract/event-line/dialog-expired · the event line · dialog-expired — Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait.
contract/mcp-wait-tool/timeout-ms · MCP wait tool · timeout_ms — Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s.
contract/viola-wait/timed-out · viola wait · timed_out — viola wait returns timed_out true only for a timeout the caller passes in.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.

## 9-ci-integration · ## 9. CI Integration
decision/a11y-cases-inside-coverage-suite · A11y cases run inside the coverage suite — The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict. · why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
decision/zero-flakiness · Zero flakiness, no retries — Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed. · why: retry-once policies mask real signals (the agent-driven principle)

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
decision/no-soft-a11y-slo · No soft a11y SLO — An a11y SLO is never soft: it triggers a build or deploy failure on its threshold.
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.

## ci-integration · key of ## 3. A11y Assertion Harness Contract
decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.

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
