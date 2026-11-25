@task_prose part=session
The session built the not-resumed end of viola adopt: when the resumed claude child ends before its session ever starts, adopt now refuses with a third own refusal, not-resumed, exit 1.
Human form is a fixed pair on stderr: unable: <name>'s session did not resume, then hint: check the session id - claude --resume <id> must open it by hand, under the same claude config (the word id is literal, the session id is never printed).
Under --json the one document is {"v":1,"error":"not-resumed","detail":null} on stdout, stderr empty; the pair or the document is written after the child released the terminal, a departure scoped to adopt's not-resumed end only, nothing of run or revive.
A resumed child still ends exit 0 and the name is taken over; the start sequence, adopt's spawn folder and the run and revive pump mapping are unchanged.
The diag-line process-exit detail enum gains not-resumed; the fake agent gains a --resume-absent option; the R5 adopt rig gains a start= field on r5a and a rehearse-only r5c line.
Measured: the premise that adopt must start its child in the session's own folder was measured false three times, so the folder was not changed; no capability is claimed.
@task_file n=1 path=src/cmd/run.rs
what: the viola run start and its pump of the claude child under the host terminal, now split so a caller gets the child's end before viola writes its own exit
@task_file n=2 path=src/cmd/adopt.rs
what: the viola adopt verb — its start, its own refusals and their fixed unable and hint pair and --json refusal document
@task_file n=3 path=schemas/diag-line.v1.json
what: the diag-line schema, the closed process-exit detail word list
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: the fake claude agent used by tests, its argv-only options
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing unit tests
@task_file n=6 path=tests/cli_adopt.rs
what: the viola adopt CLI integration tests
@task_file n=7 path=tests/cross_exit_causes.rs
what: the exit-1 cause table check across verbs
@task_file n=8 path=tests/contract_error_bodies.rs
what: the CLI --json error document bodies check
@task_file n=9 path=scripts/r5-adopt.sh
what: the R5 adopt rig script and its verdict lines
@task_item n=1 part=touches
contract/cli-verbs/adopt: viola adopt gains a third own refusal, not-resumed, exit 1, after a child that ended before its session started; its own refusals are now strict-modes-failed, session-live and not-resumed.
@task_item n=2 part=touches
Any row listing adopt's --json refusal codes or its own refusals as two: the count moves to three own, and the codes from eight to nine with not-resumed.
@task_item n=3 part=touches
contract/cli-refusal-line/form and decision/shared-refusal-word-column: the new unable line for not-resumed must keep the shared refusal line form.
@task_item n=4 part=touches
decision/hint-for-every-cause, decision/hints-name-next-verb and decision/hints-never-quote-or-locate: the new cause carries its own hint, which names claude --resume <id> with id a literal word, quoting no text and naming no path.
@task_item n=5 part=touches
decision/hint-last-on-stderr and decision/stdout-results-stderr-rest: the not-resumed pair goes on stderr with the hint last; under --json the document goes to stdout with stderr empty.
@task_item n=6 part=touches
contract/cli-json-mode/one-document and contract/cli-exit-codes/json-form: the not-resumed --json end is one document {"v":1,"error":"not-resumed","detail":null}, exit 1.
@task_item n=7 part=touches
Rows saying viola prints nothing once the child holds the terminal, or that adopt prints nothing after the child starts: adopt's not-resumed end now writes its pair or document after the child released the terminal, an exception added for that one case, not run or revive.
@task_item n=8 part=touches
Any exit-code row for exit 1 that names adopt's start refusals: not-resumed exits 1 with its pair or document after the child ended, not as a start refusal before it.
@task_item n=9 part=touches
contract/run-revive-start-json rows and revive's pump error rows: indirect, through the split of the shared child pump; run and revive keep their mapping and print nothing new.
@task_fact n=1 slug=surface-cli
fact: viola adopt's not-resumed end: when the resumed child ends before its session starts, adopt exits 1 with the fixed pair unable: <name>'s session did not resume and hint: check the session id - claude --resume <id> must open it by hand, under the same claude config; under --json one document {"v":1,"error":"not-resumed","detail":null}, stderr empty; the session id is printed in no line.
@task_fact n=2 slug=surface-cli
fact: Exception to viola writing nothing once the child held the terminal: adopt's not-resumed end writes its pair or its document after the child has released the terminal; the exception is that one case, nothing of run or revive.
@task_item n=1 part=untouched
The board and viola list rows, BAY line, columns, colour and token rendering: no board or styling output changed.
@task_item n=2 part=untouched
viola send, wait, last, answer, switch, focus, open, link and the readback mirror rows: none of these verbs changed.
@task_item n=3 part=untouched
viola verify output and refusal rows: verify is unchanged.
@task_item n=4 part=untouched
The MCP tool rows and MCP decisions: no MCP tool, result or method added.
@task_item n=5 part=untouched
Exit-21 instance-unreachable details and frame verb hints, and the exit codes 10 to 21: no code or detail there moved.
@task_item n=6 part=untouched
viola revive refusals, revive --list and the run start output rows: revive's and run's own refusals and passed start output are unchanged.
@task_item n=7 part=untouched
absence/web-surface and absence/cli-deep-links: no web surface or link added.
