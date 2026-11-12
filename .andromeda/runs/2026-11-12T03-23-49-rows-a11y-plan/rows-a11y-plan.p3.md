@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=S
@row_add n=1 row=decision/a11y-cases-inside-coverage-suite area=9-ci-integration rec=3,5
name: A11y cases run inside the coverage suite
rule: The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict.
why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
@row_none n=3 rec=4 type=S
@row_add n=2 row=decision/no-separate-a11y-driver area=9-ci-integration rec=5,30
name: No separate a11y driver or suite value
rule: A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
@row_add n=3 row=decision/no-own-a11y-artifact area=9-ci-integration rec=6
name: No a11y artifact of its own
rule: A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
@row_none n=4 rec=7 type=S
@row_none n=5 rec=8 type=S
@row_none n=6 rec=9 type=S
@row_add n=4 row=decision/cli-invariants-pass-in-ci area=10-slo-invariants-a11y-budgets rec=10
name: CLI output-discipline invariants pass in CI
rule: The CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside the `coverage` suite.
@row_add n=5 row=decision/mutation-audit-covers-crate-invariants area=10-slo-invariants-a11y-budgets rec=10
name: Mutation audit covers invariants held in crate tests
rule: Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them.
@row_add n=6 row=decision/no-a11y-performance-budget area=10-slo-invariants-a11y-budgets rec=12,11
name: No performance budget per a11y CI run
rule: A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
@row_add n=7 row=decision/zero-flakiness area=10-slo-invariants-a11y-budgets rec=13,31
name: Zero flakiness, no retries
rule: Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed.
why: retry-once policies mask real signals (the agent-driven principle)
@row_add n=8 row=decision/a11y-gate-breach-fails-build area=10-slo-invariants-a11y-budgets rec=15,14
name: Gate breach fails the build
rule: A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
@row_none n=7 rec=16 type=S
@row_none n=8 rec=17 type=S
@row_add n=9 row=decision/no-wcag-conformance-claim area=11-a11y-anti-patterns-never-do-these rec=18,50,52
name: No WCAG conformance claim
rule: No WCAG conformance is claimed for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it.
why: the Command, literal-pin and portable-pty results are output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not a11y-tool evidence
@row_add n=10 row=decision/human-keystrokes-never-blocked area=11-a11y-anti-patterns-never-do-these rec=19,44
name: Human keystrokes never blocked
rule: A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current atomic paste (P4).
@row_add n=11 row=decision/non-editing-sequences area=11-a11y-anti-patterns-never-do-these rec=19,44
name: Terminal sequences are not human editing
rule: Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).
@row_add n=12 row=decision/no-information-by-colour-alone area=11-a11y-anti-patterns-never-do-these rec=20,50
name: No information by colour alone
rule: Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
@row_add n=13 row=decision/no-colour-on-failure-words area=11-a11y-anti-patterns-never-do-these rec=21
name: Failure words are never coloured
rule: The CLI words `unable` and `fail` are never coloured.
@row_add n=14 row=decision/no-sgr-in-plain-modes area=11-a11y-anti-patterns-never-do-these rec=21,50
name: No SGR in plain modes
rule: No SGR, glyph or cursor control is emitted under non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`.
@row_add n=15 row=decision/no-unwarned-timeouts area=11-a11y-anti-patterns-never-do-these rec=22
name: No harsh timeouts without warning
rule: No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
@row_add n=16 row=decision/refusal-ends-with-hint area=11-a11y-anti-patterns-never-do-these rec=23
name: Every refusal ends with a hint
rule: Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3).
except: `unknown` (exit 14), whose detail is a code word or nothing
@row_add n=17 row=decision/codes-beside-plain-word area=11-a11y-anti-patterns-never-do-these rec=24
name: Kebab-case codes beside a plain word
rule: A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
@row_add n=18 row=decision/no-stack-trace-only-error area=11-a11y-anti-patterns-never-do-these rec=25
name: No stack trace as the only error text
rule: The CLI never renders a stack trace as the only error text.
@row_add n=19 row=decision/list-captions-fixed area=11-a11y-anti-patterns-never-do-these rec=26
name: Board captions fixed by their literal
rule: The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4).
@row_add n=20 row=decision/no-spinners area=11-a11y-anti-patterns-never-do-these rec=27,50
name: No spinners
rule: CLI output is static and linear, with no spinner or redraw.
why: design ban
@row_add n=21 row=decision/a11y-fail-fast-gate area=11-a11y-anti-patterns-never-do-these rec=28
name: A11y is a fail-fast gate
rule: A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
@row_add n=22 row=decision/machine-output-a11y-review area=11-a11y-anti-patterns-never-do-these rec=29
name: No human-only a11y review
rule: No a11y review is human-only without machine output.
why: the canonical agent-runnable principle
@row_add n=23 row=decision/no-soft-a11y-slo area=11-a11y-anti-patterns-never-do-these rec=32
name: No soft a11y SLO
rule: An a11y SLO is never soft: it triggers a build or deploy failure on its threshold.
@row_add n=24 row=decision/manual-checks-supplemental-only area=11-a11y-anti-patterns-never-do-these rec=33
name: Manual a11y checks are supplemental only
rule: Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
@row_add n=25 row=decision/no-vendor-locked-a11y-tool area=11-a11y-anti-patterns-never-do-these rec=34
name: No vendor-locked a11y tool as only method
rule: A vendor-locked a11y tool without machine output or API (Stark Visual Studio plugin, Tenable.io UI-only) is never the ONLY method.
@row_add n=26 row=decision/no-screen-content-parsing area=11-a11y-anti-patterns-never-do-these rec=35,52
name: No screen-content parsing
rule: The rendered `claude` TUI screen is never parsed for content, and screenshots are never taken as a11y evidence (R7).
why: the rendered TUI is vendor content
@row_none n=9 rec=36 type=S
@row_none n=10 rec=37 type=S
@row_add n=27 row=contract/revive-json-refusal/document area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: exit-1 refusal document
promise: Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr.
@row_add n=28 row=contract/revive-json-refusal/strict-modes-failed area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: strict-modes-failed
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=29 row=contract/revive-json-refusal/already-live area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: already-live
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=30 row=contract/revive-json-refusal/no-session area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: no-session
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; `viola revive <name> --list --json` over an empty chain prints this document, exit 1.
@row_add n=31 row=contract/revive-json-refusal/session-live area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: session-live
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=32 row=contract/revive-json-refusal/cwd-missing area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: cwd-missing
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=33 row=contract/revive-json-refusal/stale-heartbeat area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: stale-heartbeat
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=34 row=contract/revive-json-refusal/pinned-hash-mismatch area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: pinned-hash-mismatch
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
@row_add n=35 row=contract/revive-json-refusal/squatted-name area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: squatted-name
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
@row_add n=36 row=contract/revive-json-refusal/socket-dir-refused area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: socket-dir-refused
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before any snapshot, bind or child spawn.
@row_add n=37 row=contract/revive-json-refusal/internal-error area=a11y-scope-summary rec=38
boundary: viola revive --json refusals
item: internal-error
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; a `claude` not found at a start reads as this document and has no human pair.
@row_add n=38 row=contract/revive-list/json area=a11y-scope-summary rec=38
boundary: viola revive --list
item: --json output
promise: `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order.
@row_add n=39 row=contract/revive-list/human-rows area=a11y-scope-summary rec=38
boundary: viola revive --list
item: human rows
promise: Without `--json`, `viola revive <name> --list` prints static ASCII rows `<ts>  <cause>  <id>`, with no header, no colour and no ESC byte.
@row_add n=40 row=contract/revive-human-refusal/pair area=a11y-scope-summary rec=38
boundary: viola revive human refusals
item: unable/hint pair
promise: Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
@row_add n=41 row=contract/cli-usage-error/exit-2 area=a11y-scope-summary rec=38
boundary: CLI exit codes
item: 2
promise: A clap usage error exits 2; `viola plugin` alone is such an error.
@row_add n=42 row=contract/interrupted-verify/stderr area=a11y-scope-summary rec=38
boundary: viola verify interrupted
item: stderr lines
promise: A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
@row_add n=43 row=contract/interrupted-verify/exit area=a11y-scope-summary rec=38
boundary: viola verify interrupted
item: exit 1
promise: A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
@row_add n=44 row=contract/plugin-install/human-output area=a11y-scope-summary rec=38
boundary: viola plugin install
item: human output
promise: `viola plugin install` prints its four planning-form lines as one plain-ASCII write, no SGR, and no ESC or non-ASCII byte piped, under `NO_COLOR` or under `TERM=dumb`.
@row_add n=45 row=contract/plugin-install/json-output area=a11y-scope-summary rec=38
boundary: viola plugin install
item: --json output
promise: Under `--json`, `viola plugin install` prints exactly `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`.
@row_add n=46 row=contract/plugin-install/exit-21 area=a11y-scope-summary rec=38
boundary: viola plugin install
item: exit 21
promise: The exit-21 refusal of `viola plugin install` is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
@row_add n=47 row=contract/frame-verb-refusal/server-verify-failed area=a11y-scope-summary rec=38
boundary: frame verb refusals
item: server-verify-failed
promise: send, wait, last, answer, pause, release, link and unlink refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
@row_add n=48 row=contract/cli-json-failure/document area=a11y-scope-summary rec=38
boundary: CLI --json failures
item: failure document
promise: Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
@row_add n=49 row=contract/cli-json-failure/send-answer-exit-2 area=a11y-scope-summary rec=38
boundary: CLI --json failures
item: send and answer exit 2
promise: The `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document.
@row_add n=50 row=contract/link-unlink/result-line area=a11y-scope-summary rec=39
boundary: viola link and viola unlink
item: result line
promise: `viola link` and `viola unlink` each print one plain-ASCII result line in one write, with no SGR in any mode.
@row_add n=51 row=contract/link-unlink/json-ok area=a11y-scope-summary rec=39
boundary: viola link and viola unlink
item: --json output
promise: Under `--json`, `viola link` and `viola unlink` each print one `{"v":1,"ok":<ok>}` document.
@row_add n=52 row=contract/link-unlink/exit-21 area=a11y-scope-summary rec=39
boundary: viola link and viola unlink
item: exit 21
promise: `viola link` and `viola unlink` refuse with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance, and a server that is not the recorded wrapper.
@row_add n=53 row=contract/link-unlink/exit-20 area=a11y-scope-summary rec=39
boundary: viola link and viola unlink
item: exit 20
promise: A pair the wrapper refuses as invalid params reaches `viola link` or `viola unlink` as a fault: exit 20, no hint.
@row_add n=54 row=contract/frame-verb-refusal/strict-modes-failed area=a11y-scope-summary rec=40
boundary: frame verb refusals
item: strict-modes-failed
promise: send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
@row_add n=55 row=contract/frame-verb-refusal/socket-dir-refused area=a11y-scope-summary rec=41
boundary: frame verb refusals
item: socket-dir-refused
promise: Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
@row_add n=56 row=contract/start-refusal/home-strict-modes area=a11y-scope-summary rec=42
boundary: viola run, revive start and verify refusals
item: home strict-modes failure
promise: Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
@row_add n=57 row=contract/start-refusal/socket-dir area=a11y-scope-summary rec=43
boundary: viola run, revive start and verify refusals
item: socket directory failure
promise: Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.
@row_add n=58 row=contract/tui-passthrough/own-bytes area=a11y-scope-summary rec=44,52
boundary: TUI passthrough
item: terminal bytes
promise: viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal.
@row_add n=59 row=contract/tui-passthrough/pump-child-error area=a11y-scope-summary rec=44
boundary: TUI passthrough
item: pump_child error under revive --json
promise: Under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.
@row_add n=60 row=contract/tui-passthrough/refused-peer area=a11y-scope-summary rec=44
boundary: TUI passthrough
item: refused peer
promise: A peer the wrapper refuses is dropped before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel.
@row_add n=61 row=contract/tui-passthrough/after-feed-panic area=a11y-scope-summary rec=45
boundary: TUI passthrough
item: after a contained vt100 feed panic
promise: The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.
@row_add n=62 row=contract/hook-call/server-verify-failed area=a11y-scope-summary rec=46
boundary: viola hook event and dialog calls
item: server not the recorded wrapper
promise: A hook event or dialog call that meets a server that is not the recorded wrapper sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s.
@row_add n=63 row=contract/mcp-frame-tool/server-verify-failed area=a11y-scope-summary rec=46
boundary: MCP server frame tools
item: server-verify-failed
promise: An MCP frame tool (send, wait, last, answer) meeting a server that is not the recorded wrapper returns `isError` with code `instance-unreachable`, `detail` `server-verify-failed` (text `refused: instance-unreachable`) and sends no frame.
@row_add n=64 row=contract/hook-call/strict-modes-failed area=a11y-scope-summary rec=47
boundary: viola hook event and dialog calls
item: strict-modes failure
promise: A hook call refused by the strict-modes check returns exit 0 at once, empty stdout and stderr, under 1.0 s, with no `config.json` read, no stdin read, no frame, no line and no SessionEnd append; a `VIOLA_DIR` naming no instance directory writes nothing.
@row_add n=65 row=contract/mcp-frame-tool/strict-modes-failed area=a11y-scope-summary rec=47
boundary: MCP server frame tools
item: strict-modes-failed
promise: An MCP frame tool against an instance failing the strict-modes check returns `isError` with code `instance-unreachable`, `detail` `strict-modes-failed`, sends no frame and writes no line.
@row_add n=66 row=contract/hook-call/socket-dir-refused area=a11y-scope-summary rec=48
boundary: viola hook event and dialog calls
item: socket directory failure
promise: A hook call against a failing socket directory sends no frame and fails open: exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`.
@row_add n=67 row=contract/mcp-frame-tool/socket-dir-refused area=a11y-scope-summary rec=49
boundary: MCP server frame tools
item: socket-dir-refused
promise: An MCP frame tool against a failing socket directory returns `isError` with code `instance-unreachable`, `detail` `socket-dir-refused`, sends no frame and writes no line.
@row_add n=68 row=decision/output-discipline-a11y-fallback area=a11y-scope-summary rec=50
name: Output-discipline assertions stand in for a11y tools
rule: CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON.
why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
@row_add n=69 row=decision/manual-a11y-pass-gates-nothing area=a11y-scope-summary rec=50,52
name: Manual screen-reader pass gates nothing
rule: The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it.
@row_add n=70 row=contract/viola-list-board/order area=a11y-scope-summary rec=50,54,55
boundary: viola list board
item: structure
promise: The board is the BAY context line, the uppercase caption row and the wrapped rows, then the unwrapped read-only rows behind their separator.
@row_add n=71 row=contract/viola-list-board/empty-board area=a11y-scope-summary rec=50
boundary: viola list board
item: no wrapped sessions
promise: With no wrapped row, the board prints the BAY line, the caption row and `no wrapped sessions  start one: viola run <name> -- claude`.
@row_add n=72 row=contract/viola-list-board/unwrapped-separator area=a11y-scope-summary rec=50,54,55
boundary: viola list board
item: -- UNWRAPPED - READ-ONLY --
promise: The separator and unwrapped rows print only when the one `claude agents --json` read lists a session no wrapped row joins; unwrapped rows show the escaped `sessionId` as NAME and `n/a` in WHEEL, DIALOG and CLI; a failed, oversize or malformed read adds no separator and no unwrapped row.
@row_add n=73 row=contract/viola-list-board/colour area=a11y-scope-summary rec=50
boundary: viola list board
item: colour
promise: On a terminal `viola list` writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`.
@row_add n=74 row=contract/cli-help/group-labels area=a11y-scope-summary rec=50,54,55
boundary: viola --help
item: group labels
promise: The `--help` verb table is grouped under the labels board / traffic / wheel / handoff / setup.
@row_add n=75 row=contract/log-identity/service-name area=a11y-scope-summary rec=50,52
boundary: log identity tags
item: service_name
promise: Records carry `service_name:"viola"` and `version`.
@row_add n=76 row=contract/log-identity/process area=a11y-scope-summary rec=50,52
boundary: log identity tags
item: process
promise: `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run`.
@row_add n=77 row=decision/control-character-escaping area=a11y-scope-summary rec=50
name: C0/C1 control characters escaped
rule: CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two.
why: the list-row exception is ratified (T5)
except: in `viola list` table rows `\n` and `\t` are escaped too
@row_none n=11 rec=51 type=4
@row_add n=78 row=decision/portable-pty-outer-pty-driver area=a11y-scope-summary rec=52
name: portable-pty as the outer-PTY test driver
rule: TUI boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and never blocks or delays human keystrokes.
@row_add n=79 row=decision/run-diagnostics-off-terminal area=a11y-scope-summary rec=52
name: Run diagnostics never reach the terminal
rule: viola's diagnostics for `run` go to `diagnostics/`, never to the terminal.
@row_add n=80 row=decision/linux-only area=a11y-scope-summary rec=53
name: Linux only
rule: Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
@row_add n=81 row=contract/cli-help/usage-line area=aria-patterns-roles rec=55
boundary: viola --help
item: usage line
promise: `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the group rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, then `Options:`, plain ASCII.
@row_add n=82 row=contract/cli-help/served-verbs-only area=aria-patterns-roles rec=55
boundary: viola --help
item: served verbs only
promise: The help group rows list the served verbs only: `plugin install` is one entry in `setup:` and a bare `plugin` is unserved.
@row_add n=83 row=contract/cli-help/handoff-row area=aria-patterns-roles rec=55,54
boundary: viola --help
item: handoff row
promise: The help `handoff:` row is `handoff:  link, unlink`, between `wheel:` and `setup:`.
