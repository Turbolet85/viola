# a11y-plan · the rows that bind 2026-11-18-trust-record-row-read-live — picked 69 of 235 (39 by every draw) · rule any · pin 2511

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips.
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/single-vendor-content-boundary · Single vendor-content boundary — The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone.
contract/cli-output/control-characters · CLI output · C0/C1 control characters — C0/C1 control characters are escaped except \n and \t, which list rows escape too.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/viola-verify/step-line · viola verify output · step line — Each verify step prints a line [NN/NN] … pass|fail.
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
decision/colour-second-cue-only · Colour only a second cue — Colour is never the sole cue on the CLI: the word always sits beside any SGR.
decision/no-a11y-tool-output-discipline-assertions · Output-discipline assertions instead of an a11y tool — cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement. · why: no automated a11y tool reaches a terminal
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/paths-not-carried · Paths not carried — Tests Critical Path 1 (the `run` start sequence), E1 (unwrapped hooks no-op), E2 (`CLAUDE*` env strip) and security vectors 1–9 are not carried as paths; their user-visible outcomes are covered by Path 4. · why: they run on tui/ipc-internal with no viola-owned UI
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw.
contract/tui-passthrough/vendor-content · viola run TUI passthrough · vendor content — The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/control-character-escaping · C0/C1 control characters escaped — CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two. · why: the list-row exception is ratified (T5) · except: in `viola list` table rows `\n` and `\t` are escaped too
contract/interrupted-verify/exit · viola verify interrupted · exit 1 — A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
contract/interrupted-verify/stderr · viola verify interrupted · stderr lines — A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
decision/linux-only · Linux only — Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
decision/output-discipline-a11y-fallback · Output-discipline assertions stand in for a11y tools — CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON. · why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
contract/start-refusal/socket-dir · viola run, revive start and verify refusals · socket directory failure — Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
decision/word-beside-every-sgr · Word beside every SGR span — Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/c0-c1-escaped · C0 and C1 escaped in CLI output — C0 and C1 control characters in CLI output are escaped. · guard: literal pins over std::process::Command on ubuntu-latest
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest
decision/no-terminal-conformance-claim · No conformance claim for the terminal — The cli checks make no conformance claim for the terminal; they only stand in for SC 1.3.2, 1.4.1 and 3.3.1.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/control-bytes-escaped · Control bytes printed escaped — Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line. · why: security control for C0/C1 escaping, ratified for list rows · guard: escape_row_shows_every_control_character_as_hex_text
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/sgr-beside-its-word · SGR only beside its word — Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured. · guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/unable-fail-never-coloured · Refusal and failure words never coloured — The words `unable` and `fail` are never coloured.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read) falls on the driver or viola's child; no key is blocked, refused or delayed by it.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first.

## 9-ci-integration · ## 9. CI Integration
decision/a11y-cases-inside-coverage-suite · A11y cases run inside the coverage suite — The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict. · why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
decision/a11y-ci-github-actions · A11y checks in the GitHub Actions push/PR workflow — A11y checks run in the one GitHub Actions push/PR workflow on an `ubuntu-latest` matrix; the scheduled nightly workflow carries no a11y step.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
decision/cli-invariants-pass-in-ci · CLI output-discipline invariants pass in CI — The CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside the `coverage` suite.
decision/no-a11y-performance-budget · No performance budget per a11y CI run — A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
decision/zero-flakiness · Zero flakiness, no retries — Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed. · why: retry-once policies mask real signals (the agent-driven principle)

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured.
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only standing live rig reads the trust selector's place, the input box as empty, holds-text or absent, and modal dialogs, as closed words only, its raw frames kept local and gitignored
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
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
