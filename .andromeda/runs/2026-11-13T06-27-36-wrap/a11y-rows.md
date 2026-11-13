# a11y-plan · the rows 2026-11-13-verify-readiness-on-the-input-box touches — picked 5 of 241 · rule any · pin 881

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/a11y-inside-existing-suites · A11y assertions inside existing suites — A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry. · r1
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim. · r1
decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips. · r1
decision/cli-not-keyboard-interactive · CLI not keyboard-interactive — The CLI has no prompt, TUI mode or pager; its input comes only from stdin or --file. · r1
decision/cognitive-accessibility-not-triggered · Cognitive accessibility not triggered — The cognitive-accessibility trigger is none: no cognitive-accessibility rigor is required. · why: the brief has no cognitive, older-user or language-proficiency signal · r1
decision/colour-second-cue-only · Colour only a second cue — Colour is never the sole cue on the CLI: the word always sits beside any SGR. · r1
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path. · r1
decision/critical-path-dialog-answer · Critical path: dialog pending, then answer — A pending dialog answered with viola answer (question, permission, plan), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path. · r1
decision/critical-path-human-takes-wheel · Critical path: human takes the wheel — The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path. · r1
decision/critical-path-send-readback · Critical path: confirmed send and readback — Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path. · r1
decision/manual-keyboard-pass · Manual keyboard pass as supplement — A manual keyboard pass over CLI and TUI supplements the automated keystroke boundary assertion. · r1
decision/manual-orca-pass · Manual Orca screen-reader pass — A manual Orca screen-reader pass on Section 4 P4 and P6, founder-owned, supplements the automated assertions. · why: no blind or low-vision target users are named, so screen-reader priority is scoped to CLI/TUI linearity, not a full assistive-tech matrix · r1
decision/no-a11y-tool-output-discipline-assertions · Output-discipline assertions instead of an a11y tool — cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement. · why: no automated a11y tool reaches a terminal · r1
decision/no-assertions-for-internal-crates · Internal crates not assertable — The internal crates of the modular monolith are not-assertable and carry no assertions. · why: library-only, with no UI surface · r1
decision/no-assertions-for-plugin-and-ipc · Plugin and IPC channel not assertable — The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions. · why: ipc-internal with no UI surface, machine-to-machine traffic only · r1
decision/no-browser-a11y-leg · No browser a11y leg — CI runs no browser a11y leg. · why: viola renders no page · r1
decision/no-compliance-regime · No accessibility compliance regime — No accessibility compliance regime applies, so no regime-mandated WCAG success criterion binds any surface. · r1
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`. · r1
decision/no-wcag-conformance-claim · No WCAG conformance claim — No WCAG conformance is claimed for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it; the output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 and are not a11y-tool evidence. · why: WCAG does not apply directly to terminal output, and no automated a11y tool reaches terminal output · r1
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output · r1
decision/paths-not-carried · Paths not carried — Tests Critical Path 1 (the `run` start sequence), E1 (unwrapped hooks no-op), E2 (`CLAUDE*` env strip) and security vectors 1–9 are not carried as paths; their user-visible outcomes are covered by Path 4. · why: they run on tui/ipc-internal with no viola-owned UI · r1
decision/single-vendor-content-boundary · Single vendor-content boundary — The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone. · r1
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw. · r1
decision/terminal-structural-equivalents · Terminal structural equivalents of ARIA roles — In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator and the --help group labels. · r1
decision/tier-single-source-of-rigor · Tier as single source of accessibility rigor — The tier is the single source of truth for how much accessibility rigor downstream skills apply, in the phase loop per chunk and in cross-cutting reads from route and setup-project. · r1
decision/tui-boundary-assertions-only · TUI boundary assertions only — The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim. · r1
contract/cli-output/control-characters · CLI output · C0/C1 control characters — C0/C1 control characters are escaped except \n and \t, which list rows escape too. · r1
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code. · r1
contract/tui-passthrough/entry · viola run TUI passthrough · entry — The passthrough is entered by viola run and by a viola revive whose preflight passed. · r1
contract/tui-passthrough/focus-mouse-resize-sequences · viola run TUI passthrough · focus, mouse and resize sequences — Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes. · r1
contract/tui-passthrough/human-keystrokes · viola run TUI passthrough · human keystrokes — Human keystrokes are never blocked, refused or delayed past the current atomic paste. · r1
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal. · r1
contract/tui-passthrough/vendor-content · viola run TUI passthrough · vendor content — The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI. · r1
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — viola writes zero bytes of its own to the terminal; only the child's output reaches it. · r1
contract/tui-passthrough/vt100-feed-panic · viola run TUI passthrough · vt100 feed panic — The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it. · r1
contract/viola-list/unwrapped-separator · viola list output · -- UNWRAPPED - READ-ONLY -- — The separator appears only when unwrapped rows exist. · r1
contract/viola-revive-json/pump-child-error · viola revive --json output · pump_child error — A pump_child error prints no document: the child has held the terminal, and viola writes nothing past it. · r1
contract/viola-revive-json/start-refusal · viola revive --json output · start refusal — Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error. · r1
contract/viola-verify/step-line · viola verify output · step line — Each verify step prints a line [NN/NN] … pass|fail. · r1
contract/viola-wait/waiting-line · viola wait output · waiting: <name> — The waiting line is written once on stderr, on a TTY only. · r1
absence/internationalization — Internationalization: no i18n, RTL or lang conventions · r1
absence/touch-surface — A touch surface · r1
absence/web-or-graphical-frontend — A web or graphical frontend · r1

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/control-character-escaping · C0/C1 control characters escaped — CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two. · why: the list-row exception is ratified (T5) · except: in `viola list` table rows `\n` and `\t` are escaped too · r1
decision/linux-only · Linux only — Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland. · r1
decision/manual-a11y-pass-gates-nothing · Manual screen-reader pass gates nothing — The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it. · r1
decision/output-discipline-a11y-fallback · Output-discipline assertions stand in for a11y tools — CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON. · why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree · r1
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal. · r1
contract/cli-help/group-labels · viola --help · group labels — The `--help` verb table is grouped under the labels board / traffic / wheel / handoff / setup. · r1
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · r1
contract/cli-json-failure/send-answer-exit-2 · CLI --json failures · send and answer exit 2 — The `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document. · r1
contract/cli-usage-error/exit-2 · CLI exit codes · 2 — A clap usage error exits 2; `viola plugin` alone is such an error. · r1
contract/frame-verb-refusal/server-verify-failed · frame verb refusals · server-verify-failed — send, wait, last, answer, pause, release, link and unlink refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty. · r1
contract/frame-verb-refusal/socket-dir-refused · frame verb refusals · socket-dir-refused — Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found. · r1
contract/frame-verb-refusal/strict-modes-failed · frame verb refusals · strict-modes-failed — send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found. · r1
contract/hook-call/server-verify-failed · viola hook event and dialog calls · server not the recorded wrapper — A hook event or dialog call that meets a server that is not the recorded wrapper sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s. · r1
contract/hook-call/socket-dir-refused · viola hook event and dialog calls · socket directory failure — A hook call against a failing socket directory sends no frame and fails open: exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`. · r1
contract/hook-call/strict-modes-failed · viola hook event and dialog calls · strict-modes failure — A hook call refused by the strict-modes check returns exit 0 at once, empty stdout and stderr, under 1.0 s, with no `config.json` read, no stdin read, no frame, no line and no SessionEnd append; a `VIOLA_DIR` naming no instance directory writes nothing. · r1
contract/interrupted-verify/exit · viola verify interrupted · exit 1 — A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1. · r1
contract/interrupted-verify/stderr · viola verify interrupted · stderr lines — A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII. · r1
contract/link-unlink/exit-20 · viola link and viola unlink · exit 20 — A pair the wrapper refuses as invalid params reaches `viola link` or `viola unlink` as a fault: exit 20, no hint. · r1
contract/link-unlink/exit-21 · viola link and viola unlink · exit 21 — `viola link` and `viola unlink` refuse with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance, and a server that is not the recorded wrapper. · r1
contract/link-unlink/json-ok · viola link and viola unlink · --json output — Under `--json`, `viola link` and `viola unlink` each print one `{"v":1,"ok":<ok>}` document. · r1
contract/link-unlink/result-line · viola link and viola unlink · result line — `viola link` and `viola unlink` each print one plain-ASCII result line in one write, with no SGR in any mode. · r1
contract/log-identity/process · log identity tags · process — `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run`. · r1
contract/log-identity/service-name · log identity tags · service_name — Records carry `service_name:"viola"` and `version`. · r1
contract/mcp-frame-tool/server-verify-failed · MCP server frame tools · server-verify-failed — An MCP frame tool (send, wait, last, answer) meeting a server that is not the recorded wrapper returns `isError` with code `instance-unreachable`, `detail` `server-verify-failed` (text `refused: instance-unreachable`) and sends no frame. · r1
contract/mcp-frame-tool/socket-dir-refused · MCP server frame tools · socket-dir-refused — An MCP frame tool against a failing socket directory returns `isError` with code `instance-unreachable`, `detail` `socket-dir-refused`, sends no frame and writes no line. · r1
contract/mcp-frame-tool/strict-modes-failed · MCP server frame tools · strict-modes-failed — An MCP frame tool against an instance failing the strict-modes check returns `isError` with code `instance-unreachable`, `detail` `strict-modes-failed`, sends no frame and writes no line. · r1
contract/plugin-install/exit-21 · viola plugin install · exit 21 — The exit-21 refusal of `viola plugin install` is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty. · r1
contract/plugin-install/human-output · viola plugin install · human output — `viola plugin install` prints its four planning-form lines as one plain-ASCII write, no SGR, and no ESC or non-ASCII byte piped, under `NO_COLOR` or under `TERM=dumb`. · r1
contract/plugin-install/json-output · viola plugin install · --json output — Under `--json`, `viola plugin install` prints exactly `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`. · r1
contract/revive-human-refusal/pair · viola revive human refusals · unable/hint pair — Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical. · r1
contract/revive-json-refusal/already-live · viola revive --json refusals · already-live — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries. · r1
contract/revive-json-refusal/cwd-missing · viola revive --json refusals · cwd-missing — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries. · r1
contract/revive-json-refusal/document · viola revive --json refusals · exit-1 refusal document — Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr. · r1
contract/revive-json-refusal/internal-error · viola revive --json refusals · internal-error — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; a `claude` not found at a start reads as this document and has no human pair. · r1
contract/revive-json-refusal/no-session · viola revive --json refusals · no-session — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; `viola revive <name> --list --json` over an empty chain prints this document, exit 1. · r1
contract/revive-json-refusal/pinned-hash-mismatch · viola revive --json refusals · pinned-hash-mismatch — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns. · r1
contract/revive-json-refusal/session-live · viola revive --json refusals · session-live — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries. · r1
contract/revive-json-refusal/socket-dir-refused · viola revive --json refusals · socket-dir-refused — One exit-1 document with `socket-dir-refused` its code on stdout, in place of the human pair, before any snapshot, bind or child spawn, holding no path, pid or uid, with stderr empty and no hint. · r1
contract/revive-json-refusal/squatted-name · viola revive --json refusals · squatted-name — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns. · r1
contract/revive-json-refusal/stale-heartbeat · viola revive --json refusals · stale-heartbeat — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries. · r1
contract/revive-json-refusal/strict-modes-failed · viola revive --json refusals · strict-modes-failed — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries. · r1
contract/revive-list/human-rows · viola revive --list · human rows — Without `--json`, `viola revive <name> --list` prints static ASCII rows `<ts>  <cause>  <id>`, with no header, no colour and no ESC byte. · r1
contract/revive-list/json · viola revive --list · --json output — `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order. · r1
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary. · r1
contract/start-refusal/socket-dir · viola run, revive start and verify refusals · socket directory failure — Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found. · r1
contract/tui-passthrough/after-feed-panic · TUI passthrough · after a contained vt100 feed panic — The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do. · r1 · stale
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal. · r1
contract/tui-passthrough/pump-child-error · TUI passthrough · pump_child error under revive --json — Under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it. · r1
contract/tui-passthrough/refused-peer · TUI passthrough · refused peer — A peer the wrapper refuses is dropped before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel. · r1
contract/viola-list-board/colour · viola list board · colour — On a terminal `viola list` writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`. · r1
contract/viola-list-board/empty-board · viola list board · no wrapped sessions — With no wrapped row, the board prints the BAY line, the caption row and `no wrapped sessions  start one: viola run <name> -- claude`. · r1
contract/viola-list-board/order · viola list board · structure — The board is the BAY context line, the uppercase caption row and the wrapped rows, then the unwrapped read-only rows behind their separator. · r1
contract/viola-list-board/unwrapped-separator · viola list board · -- UNWRAPPED - READ-ONLY -- — The separator and unwrapped rows print only when the one `claude agents --json` read lists a session no wrapped row joins; unwrapped rows show the escaped `sessionId` as NAME and `n/a` in WHEEL, DIALOG and CLI; a failed, oversize or malformed read adds no separator and no unwrapped row. · r1

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal. · r1
decision/tui-boundary-driver-pin · TUI boundary driver — The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins. · r1 · stale

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own. · r1
decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own. · r1
decision/manual-screen-reader-pass · Manual screen reader pass — The screen reader check is a manual Orca pass owned by the founder, with no automated test spec. · r1
absence/rendered-page — A rendered page, and with it any browser a11y tooling · r1

## ci-integration · key of ## 3. A11y Assertion Harness Contract
decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed. · r1

## contrast-verification-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## focus-management-test-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content. · r1
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case. · r1
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins · r1
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating. · r1
decision/statusline-output-passed-byte-for-byte · Statusline output passed byte for byte — viola hook statusline hands the user's own status line output to the child byte for byte and writes nothing to the terminal itself. · guard: passthrough_with_a_statusline_source_adds_no_viola_bytes · r1
decision/stdin-classifier-non-editing-list · Closed non-editing stdin list — Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel. · why: ratified, the exact grammars chosen after a narrower option was shown · r1
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run. · r1

## screen-reader-test-pattern · key of ## 3. A11y Assertion Harness Contract
decision/cli-screen-reader-test-setup · CLI screen-reader test setup — The cli surface is screen-reader tested with Orca on GNOME Terminal. · r1
decision/tui-screen-reader-test-setup · TUI screen-reader test setup — The tui surface is screen-reader tested with Orca on foot under Hyprland. · r1

## structured-violation-json-schema · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.
contract/structured-violation-json/cli-tui-a11y-rows · Structured violation JSON · a11y rows for cli and tui — The cli and tui surfaces emit no a11y rows. · r1
contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[]. · r1

## wcag-criteria-mapping · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/c0-c1-escaped · C0 and C1 escaped in CLI output — C0 and C1 control characters in CLI output are escaped. · guard: literal pins over std::process::Command on ubuntu-latest · r1
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg. · r1
decision/cli-structural-landmark-equivalents · CLI structural equivalents of landmarks — The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels. · r1
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest · r1
decision/no-terminal-conformance-claim · No conformance claim for the terminal — The cli checks make no conformance claim for the terminal; they only stand in for SC 1.3.2, 1.4.1 and 3.3.1. · r1
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed. · r1 · stale
decision/send-refused-while-human-typing · Send refused while the human types — When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code. · guard: std::process::Command asserts · r1
decision/stale-row-dim · Dim only on stale rows — Dim SGR is applied only to stale rows, beside the word that carries the state. · guard: unit-tier tests and outer-PTY tests on ubuntu-latest · r1
decision/tui-boundary-only · Tui accessibility is boundary-only — The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own. · r1
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest · r1

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/control-bytes-escaped · Control bytes printed escaped — Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line. · why: security control for C0/C1 escaping, ratified for list rows · guard: escape_row_shows_every_control_character_as_hex_text · r1
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY. · r1
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json · r1
decision/sgr-beside-its-word · SGR only beside its word — Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured. · guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal · r1
contract/cli-exit-codes/10 · CLI exit codes · 10 — human-typing: the human holds the wheel, so the automated send is refused with nothing typed. · r1
contract/cli-help/handoff-row · viola --help · handoff row — The help `handoff:` row is `handoff:  link, unlink`, between `wheel:` and `setup:`. · r1
contract/cli-help/served-verbs-only · viola --help · served verbs only — The help group rows list the served verbs only: `plugin install` is one entry in `setup:` and a bare `plugin` is unserved. · r1
contract/cli-help/usage-line · viola --help · usage line — `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the group rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, then `Options:`, plain ASCII. · r1
contract/viola-list-board/caption · viola list board · caption order — The caption order is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, reused by the terminal fleet console. · r1

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose · r1
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design. · r1
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text. · r1
decision/no-reading-level-gate · No reading-level gate — No reading-level target is gated and no readability scorer is introduced. · why: SC 3.1.5 Reading Level (AAA) is not claimed, and no readability scorer is in a11y-research · r1
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`. · r1
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest · r1
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, budget pause) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it. · r1
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code. · r1
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint. · r1
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document. · r1
contract/cli-refusal/unknown-refusal · CLI refusal output · the `unknown` refusal — The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing. · r1
contract/viola-list-caption/order · viola list caption · order — The caption order is `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`, pinned by the `CAPTION` literal. · r1
absence/user-time-limit — A time limit on the user, and any timeout-extension component · r1

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/budget-gate · Budget gate on automated sends — While a five-hour or seven-day window stands at or over its threshold with a known reset ahead, an automated viola send is refused budget-paused with nothing typed, until that reset or viola release --budget; no threshold value switches the gate off. · guard: path6_a_reading_over_the_threshold_refuses_the_send · r1
decision/budget-gate-only-on-send · Only send reads the budget gate — answer, wait, last and pause do not read the budget gate and give under a closed gate what they give under an open one. · guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone · r1
decision/claude-agents-read-fails-open · claude agents read fails open — A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's preflight. · r1
decision/client-euid-refusal-test-strategy · Client euid refusal test strategy — No foreign peer is driven: the client euid refusal's decision is tested over injected euids and its real read on Linux, and each client seam's wiring is read only by forced refusals in remove-the-guard runs. · r1
decision/dialog-hold-bounds-the-driver · Dialog hold bounds the driver — A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold. · provisional · r1
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand · r1
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists. · r1
decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint. · r1
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose. · r1
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text · r1
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document · r1
decision/link-pair-validated-by-wrapper · Link pairs validated by the wrapper — viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint. · r1
decision/link-result-lines-one-write-no-sgr · Link result lines in one write, no SGR — The link and unlink result lines are each one write with no SGR in any mode. · guard: link_verb_lines_are_the_design_lines_in_one_write · r1
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release · r1
decision/one-hint-per-code · One hint per code — No two codes share a hint. · guard: no_two_codes_share_a_hint · r1
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP). · r1
decision/printed-names-are-parsed-arguments · Printed names are the parsed arguments — Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log. · r1
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start. · r1
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify. · r1
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing · r1
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, budget pause, statusline deadline, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel. · r1
decision/unknown-refusal-has-no-hint · unknown refusal carries no hint — The unknown refusal prints no hint. · why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess · r1
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design · r1
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21. · r1
contract/cli-exit-codes/11 · CLI exit codes · 11 — budget-paused, with detail five-hour or seven-day. · r1
contract/cli-exit-codes/13 · CLI exit codes · 13 — An answer for an expired or absent dialog is refused not-delivered / unknown-dialog. · r1
contract/cli-exit-codes/14 · CLI exit codes · 14 — unknown: a refusal whose word is outside the closed refusal list. · r1
contract/cli-exit-codes/2 · CLI exit codes · 2 — A usage error, or an unusable answer response or send text written as one fixed stderr line with no document. · r1
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint. · r1
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail. · r1
contract/cli-fixed-message/form · CLI fixed-message refusal · form — viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last. · r1
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code. · r1
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first. · r1
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — Whole seconds 60 to 3600, default 60, read once at the wrapper's start; any other value gives 60 and one parse-rejected. · r1
contract/config-json/five-hour · config.json · five_hour — The five-hour window's budget threshold, default 90. · r1
contract/config-json/seven-day · config.json · seven_day — The seven-day window's budget threshold, default 85. · r1
contract/event-line/dialog-expired · the event line · dialog-expired — Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait. · r1
contract/fault-body/message · fault body under --json and on MCP · message — A code outside the five known reads the fixed message wrapper fault with null data. · r1
contract/human-fault-line/wrapper-fault · human fault line · wrapper fault — error: wrapper fault  <code>. · r1
contract/instance-unreachable-detail/endpoint-lost · instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open. · r1
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — The name has no live endpoint but its instance directory stands, or the instance is stale. · r1
contract/instance-unreachable-detail/server-verify-failed · instance-unreachable detail · server-verify-failed — The process serving the endpoint is not the recorded wrapper and was not contacted. · r1
contract/instance-unreachable-detail/socket-dir-refused · instance-unreachable detail · socket-dir-refused — The socket directory was refused; the verb sent no frame. · r1
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — The instance failed the strict-modes check; refused before any frame or log, no log line written. · r1
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — The name has no live endpoint and no instance directory (absent, a regular file or a symlink). · r1
contract/instance-unreachable-json/document · exit 21 under --json · document — One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC. · r1
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there. · r1
contract/mcp-wait-tool/timeout-ms · MCP wait tool · timeout_ms — Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s. · r1
contract/send-refusal/input-not-ready · viola send refusal · input-not-ready — A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed. · r1 · stale
contract/send-refusal/no-prompt-submitted · viola send refusal · no-prompt-submitted — A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted. · r1
contract/start-holder-refusal/already-live · start against a holder, exit 1 · already-live — The holder is live, or a fresh replayed beat was read. · r1
contract/start-holder-refusal/stale-heartbeat · start against a holder, exit 1 · stale-heartbeat — The holder is stale; stale heartbeat is exit 1's cause alone. · r1
contract/start-json-refusal/document · run / revive start under --json · refusal document — Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr. · r1
contract/viola-hook-statusline/exit · viola hook statusline · exit status — Runs the user's own status line command under a 5 s deadline and exits 0 with empty stderr in every case. · r1
contract/viola-link-line/time · viola link line · time — The reply's since as UTC clock time in whole seconds with Z. · r1
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI. · r1
contract/viola-wait/timed-out · viola wait · timed_out — viola wait returns timed_out true only for a timeout the caller passes in. · r1

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning. · r1
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed · r1
decision/list-captions-fixed · Board captions fixed by their literal — The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4). · r1
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle · r1
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK. · r1
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured. · r1
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1). · r1
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled input-box and modal signatures, never for other content, and screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · r1 · add-to · F1
decision/no-soft-a11y-slo · No soft a11y SLO — An a11y SLO is never soft: it triggers a build or deploy failure on its threshold. · r1
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban · r1
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text. · r1
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1). · r1
decision/no-vendor-locked-a11y-tool · No vendor-locked a11y tool as only method — A vendor-locked a11y tool without machine output or API (Stark Visual Studio plugin, Tenable.io UI-only) is never the ONLY method. · r1
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4). · r1
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing · r1
