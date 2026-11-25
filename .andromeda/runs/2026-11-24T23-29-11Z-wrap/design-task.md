@task_prose part=session
The session built the not-resumed end of viola adopt: a child that ends before its session resumed is now said and recorded, not passed off as a clean exit 0.
viola adopt gains a third own refusal, not-resumed, beside strict-modes-failed and session-live: the fixed pair unable: <name>'s session did not resume / hint: check the session id - claude --resume <id> must open it by hand, under the same claude config (the word id literal), or under --json the document with error not-resumed and detail null, stderr empty; exit 1.
The refusal is written after the child ended and released the terminal, the criterion being the wrapper's own instance snapshot holding no agent session id; a resumed child (or a replayed, absent or unreadable snapshot) still ends exit 0, a worker panic still ends internal-error, exit 1, no document.
The child pump was split: a new end-reporting pump returns after the host terminal is restored and writes no self exit; run and revive keep the old mapping over it unchanged; only adopt uses the new one.
The process-exit log line's closed detail enum gains not-resumed (self, exit 1, after the child's process-exit); the session id appears in no line.
Measured: the premise that adopt must start its child in the session's original folder was found false three times, so adopt's spawn folder was not changed; the end is proven on the fake agent alone, no capability claimed.
@task_file n=1 path=src/cmd/run.rs
what: the viola run verb: its start sequence and the child pump that holds the terminal while the child runs, logs the child's process-exit and maps the end to viola's own exit, shared by run, revive and adopt
@task_file n=2 path=src/cmd/adopt.rs
what: the viola adopt verb: its own refusals with their fixed unable/hint pairs and --json refusal documents, and how its start and end map to exit codes
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, including process-exit's closed detail enum of code words
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: the fake agent, the test double standing in for the claude CLI, driven by argv-only options
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing unit tests
@task_file n=6 path=tests/cli_adopt.rs
what: viola adopt's integration tests: its start, its refusals, their stderr pairs and --json documents and the log lines they write
@task_file n=7 path=tests/cross_exit_causes.rs
what: the exit-1 cause table check: each cause code word with its exit code, hint and writer
@task_file n=8 path=tests/contract_error_bodies.rs
what: the CLI --json error document check: the body each refusal code word prints
@task_file n=9 path=scripts/r5-adopt.sh
what: the R5 adopt rig: its live and rehearse-only arms over viola adopt and their verdict lines
@task_item n=1 part=touches
decision/no-json-after-child-held-terminal: adopt's not-resumed end now writes its pair, or its --json document, after the child held and released the terminal; an exception scoped to adopt's not-resumed end only (the operator's own answer), nothing of run or revive.
@task_item n=2 part=touches
contract/viola-run-output/child-started: it says adopt prints nothing once the child starts; that is now false for adopt's not-resumed end, which prints its pair or document after the child ended.
@task_item n=3 part=touches
contract/cli-exit-codes/1-adopt-start: adopt's exit 1 now also covers not-resumed, an end after the child ran rather than a start refusal; the row says only a start refusal of viola adopt exits 1 with its fixed pair.
@task_item n=4 part=touches
contract/viola-adopt-refusals/session-live and the viola adopt refusals family: adopt's own refusals go from two to three; not-resumed has no row of its own yet.
@task_item n=5 part=touches
contract/cli-exit-codes/1: the exit-1 cause list gains not-resumed for viola adopt.
@task_item n=6 part=touches
The unable/hint form rows (contract/cli-refusal-line/fixed-unable-form, contract/cli-refusal-line/hint, contract/cli-stderr/fixed-message-refusal, contract/cli-stderr/refusal-line, contract/cli-stderr/hint-line): a new fixed pair joins them, its hint naming the command claude --resume with the literal placeholder id.
@task_item n=7 part=touches
decision/hint-never-quotes-text and decision/no-internals-in-errors: the not-resumed hint names an external command with a placeholder word and quotes no session id; a picker should check the hint against both.
@task_item n=8 part=touches
decision/start-refusals-show-no-path-or-pid: the not-resumed pair holds the instance name only, no path, pid or session id.
@task_item n=9 part=touches
decision/json-one-document-on-stdout and decision/json-failure-documents-clean: adopt --json's not-resumed end prints exactly one document with error not-resumed and detail null, stderr empty.
@task_item n=10 part=touches
contract/cli-catch-site/json-internal-error: adopt's worker-panic end still maps to internal-error, exit 1, no document, now through the new end-reporting pump.
@task_item n=11 part=touches
contract/log-detail-codes rows: process-exit's closed detail enum gains the code word not-resumed, written by adopt as its self exit 1 after the child's process-exit.
@task_item n=12 part=touches
Indirect, through the split child pump: decision/run-prints-nothing-while-wrapped and contract/viola-revive-output/after-child-start rule over the pump run and revive use; their mapping (exit 0 on a clean end, internal-error on a panic) is unchanged, a picker should confirm they hold.
@task_item n=13 part=touches
decision/no-success-before-confirmation: adopt no longer ends exit 0 when its child ended before its session resumed.
@task_fact n=1 slug=surface-cli
fact: viola adopt refusals · not-resumed: when the adopted child ends before its session resumed (the wrapper's own snapshot holds no agent session id), adopt writes unable: <name>'s session did not resume / hint: check the session id - claude --resume <id> must open it by hand, under the same claude config, or under --json the document with error not-resumed and detail null, stderr empty, and exits 1.
@task_fact n=2 slug=surface
fact: Exception to no document once the child held the terminal: adopt's not-resumed end writes its fixed pair or --json document after the child released the terminal; run and revive keep writing nothing past the child.
@task_fact n=3 slug=surface-cli
fact: viola adopt's exit 1 covers not-resumed, an end after the child ran and not a start refusal; a resumed child still ends exit 0 and a pump fault ends internal-error, exit 1, no document.
@task_fact n=4 slug=surface
fact: log detail codes · not-resumed: viola adopt logs process-exit subject self, exit 1, detail not-resumed after the child's own process-exit, with no session id in any line.
@task_item n=1 part=untouched
Brand identity, the flight-progress strip, the readback box and send readback rows: no strip, board or readback output changed.
@task_item n=2 part=untouched
Colour palette, colour tokens, colour decision order and depth, typography, spacing and iconography: the new pair is plain ASCII text with no colour or glyph.
@task_item n=3 part=untouched
viola list board and table rows, NAME truncation and width source: the board is not reached.
@task_item n=4 part=untouched
viola revive refusals and its --json exit-1 codes, and viola run start refusals: no refusal of run or revive was added, changed or retired.
@task_item n=5 part=untouched
Exit 21 causes, frame verb refusals, wrapper fault detail, peer reply screening and refusal hints of send: no IPC method, socket, peer reply or wrapper fault changed.
@task_item n=6 part=untouched
viola verify, up, link, unlink, focus, wait and last output rows, and the help verb groups and name/command rows: no verb was added, renamed or changed beyond adopt.
@task_item n=7 part=untouched
Anti-patterns and self-validation rows: no spinner, progress pattern, sorting or colour use was introduced.
