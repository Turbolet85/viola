@task_prose part=session
The session built the not-resumed end of viola adopt: when the resumed claude child ends before its session starts, adopt now refuses with a third refusal of its own, not-resumed, exit 1.
On a human terminal it prints the fixed pair «unable: <name>'s session did not resume» then «hint: check the session id - claude --resume <id> must open it by hand, under the same claude config»; under --json it prints exactly the document {"v":1,"error":"not-resumed","detail":null} with stderr empty.
That pair or document is written after the child ended and released the terminal, an exception scoped to adopt's not-resumed end alone; run and revive keep writing nothing past the child, and a pump error under adopt still prints no document.
The run diagnostics log gains the process-exit detail word not-resumed (self, exit 1), logged after the child's own process-exit; no session id appears in any line.
The fake claude agent gains the argv-only option --resume-absent, and the R5 adopt rig gains a start= field on r5a and a rehearse-only third verdict line r5c.
It measured the premise that adopt must start its child in the session's own folder false, so the spawn folder is unchanged; it claims no capability.
@task_file n=1 path=src/cmd/run.rs
what: the child pump of the TUI passthrough (pump_child, now over a new pump_end) that viola run, revive start and adopt start hand the terminal to; run and revive keep their exit mapping, adopt now reads the child's end itself
@task_file n=2 path=src/cmd/adopt.rs
what: the viola adopt command: its own refusals (strict-modes-failed, session-live, now not-resumed), their fixed unable/hint pairs, the hint text, and its --json refusal document
@task_file n=3 path=schemas/diag-line.v1.json
what: the run diagnostics log line schema, holding the closed detail enum of the process-exit event
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: the fake claude agent the test suites drive in place of the real CLI, and its argv-only options
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing unit tests
@task_file n=6 path=tests/cli_adopt.rs
what: the viola adopt integration suite, pinning by literal its refusal pair, its --json refusal document, its empty stderr under --json and the absence of the session id in log lines
@task_file n=7 path=tests/cross_exit_causes.rs
what: the exit-1 cause table check, each code word with its exit code and hint
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body check, each CLI error document with its writer and its witness
@task_file n=9 path=scripts/r5-adopt.sh
what: the R5 adopt live rig script and its verdict lines, its new arm run only under --rehearse over the fakes
@task_item n=1 part=touches
contract/cli-refusal/unable-line names viola adopt's own refusals as two (strict-modes-failed, session-live); they are now three, not-resumed the third, with the unable line «unable: <name>'s session did not resume».
@task_item n=2 part=touches
contract/cli-fixed-message/form counts adopt's own fixed-message refusals as two; not-resumed is a third fixed pair, unable then hint, ASCII, the instance name its only interpolated value.
@task_item n=3 part=touches
contract/start-json-refusal/document lists adopt's own --json refusal codes as two; not-resumed is a third, its document {"v":1,"error":"not-resumed","detail":null}, exit 1, stderr empty.
@task_item n=4 part=touches
contract/tui-passthrough/pump-child-error and contract/viola-revive-json/pump-child-error hold that viola writes nothing past the child once it held the terminal; adopt's not-resumed end now writes its pair or document after the child released the terminal, an exception for adopt alone (revive and run unchanged, and a pump error under adopt still prints no document).
@task_item n=5 part=touches
contract/cli-refusal/hint-line and decision/one-hint-per-code: the new code not-resumed carries one fixed hint of its own, naming claude --resume <id> with <id> the literal word.
@task_item n=6 part=touches
decision/refusal-ends-with-hint and decision/reason-code-beside-unable: the new refusal ends with its hint and its code word not-resumed stands beside it in the log and under --json.
@task_item n=7 part=touches
decision/error-bodies-have-writer-and-witness: the not-resumed body gains its writer (adopt's refusal) and its witness rows in the error-body and exit-1 cause tables.
@task_item n=8 part=touches
decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal and decision/refusal-layout-literal-pins: the new pair is pinned by literal in the adopt integration suite and a unit case.
@task_item n=9 part=touches
contract/cli-refusal/exit-code, contract/cli-refusal/json and decision/json-one-document-no-hint: not-resumed exits 1 and under --json prints one document with no hint and nothing on stderr.
@task_item n=10 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: the not-resumed process-exit line goes to the run diagnostics log only, never to the terminal.
@task_item n=11 part=touches
contract/code-word/format: not-resumed is a new kebab-case code word, in the process-exit detail enum and as a --json error.
@task_item n=12 part=touches
decision/tui-terminal-belongs-to-child and decision/zero-viola-bytes-on-the-terminal: indirectly, through the child pump, adopt writes its refusal only once the child has ended and handed the terminal back, never while the child holds it.
@task_fact n=1 slug=2-a11y-strategy
fact: viola adopt has three refusals of its own, strict-modes-failed, session-live and not-resumed; not-resumed prints «unable: <name>'s session did not resume» then «hint: check the session id - claude --resume <id> must open it by hand, under the same claude config», exit 1.
@task_fact n=2 slug=cognitive-accessibility
fact: under --json, adopt's not-resumed refusal is exactly {"v":1,"error":"not-resumed","detail":null} on stdout, exit 1, stderr empty, and no session id in the document or in any log line.
@task_fact n=3 slug=a11y-scope-summary
fact: the rule that viola writes nothing past a child that held the terminal has one named exception: adopt's not-resumed end writes its pair or document after the child ended and released the terminal; run and revive keep the rule, and a pump error under adopt still prints no document.
@task_item n=1 part=untouched
The viola list board, its captions, colour, SGR, stale-row dim and NAME truncation: no output of the board changed.
@task_item n=2 part=untouched
viola --help group labels, usage line and served verbs: no verb or flag added.
@task_item n=3 part=untouched
viola link, unlink, plugin install, switch, up, wait and verify output and their exit codes: none of their lines changed.
@task_item n=4 part=untouched
Hook calls, frame verb refusals, MCP frame and wait tools and the instance-unreachable details: no IPC method, event kind, socket or endpoint changed.
@task_item n=5 part=untouched
viola revive's own start refusals and their --json codes (already-live, cwd-missing, no-session, squatted-name and the rest) and its --list output: revive's refusals are unchanged.
@task_item n=6 part=untouched
Human keystrokes, the wheel, focus, mouse and resize sequences, the closed non-editing stdin list and the vt100 feed panic containment: the passthrough's handling of the human's terminal is unchanged.
@task_item n=7 part=untouched
CI integration, the a11y gate, zero-flakiness and mutation-audit decisions, the manual Orca and keyboard passes and the screen-reader targets: no gate, threshold or suite changed.
@task_item n=8 part=untouched
Time bounds (dialog hold, open live wait, reap, window wait, compositor read): no bound added or moved.
@task_item n=9 part=untouched
The absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit): no capability added.
