@task_prose part=session
viola adopt gained a third own refusal, not-resumed: when the resumed claude child ends before its session starts, adopt exits 1 with a fixed unable/hint pair, or under --json the document {"v":1,"error":"not-resumed","detail":null}, written after the child released the terminal.
Whether the session resumed is read from the wrapper's own instance snapshot once the child ends; only a real snapshot with no agent session id counts as not resumed, a replayed, absent or unreadable snapshot counts as resumed and exits 0.
The process-exit diagnostics line's closed detail enum gained the word not-resumed, logged on the self process-exit after the child's own process-exit; no session id appears in any line, the pair names the instance only.
The start's child pump was split so adopt receives the child's end without a self exit written for it; run and revive keep the old pump mapping unchanged. No IPC method, endpoint, event kind, socket, env var or dependency was added, and no capability was claimed.
The fake agent gained a test-only option --resume-absent, and the R5 adopt rig gained a rehearse-only arm r5c over the fakes plus a start= field on r5a; no real claude ran.
The chunk's premise that adopt must start its child in the session's original folder was measured false, so adopt's spawn folder did not change.
@task_file n=1 path=src/cmd/run.rs
what: the start of viola run and the pump that carries the claude child's terminal and logs the child's process-exit, now also the end-of-child step adopt calls
@task_file n=2 path=src/cmd/adopt.rs
what: the viola adopt command, its own closed refusals and their fixed pairs and --json documents, and its read of the instance snapshot to tell whether the session resumed
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema, the closed detail enum of the process-exit line
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary under the fake-agent feature, its argv-only options
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option parsing tests
@task_file n=6 path=tests/cli_adopt.rs
what: the integration tests of viola adopt, run in test-owned homes
@task_file n=7 path=tests/cross_exit_causes.rs
what: the exit-1 cause table test, every exit-1 detail with its writer
@task_file n=8 path=tests/contract_error_bodies.rs
what: the CLI --json error document bodies test, every refusal document listed
@task_file n=9 path=scripts/r5-adopt.sh
what: the R5 adopt rig, which hosts adopt starts and ends only the processes it recorded, its live arms and its rehearse-only arms over the fakes
@task_item n=1 part=touches
viola adopt's own refusals grow from two (strict-modes-failed, session-live) to three with not-resumed; any row listing adopt's own refusals or its --json codes as two, or as the eight codes, is now short by one.
@task_item n=2 part=touches
The strict-modes refusal of viola adopt and the adopt refusal order: not-resumed is a refusal written after the child ran and ended, not a start refusal before the spawn; a row saying every adopt refusal comes before the child starts now has an exception.
@task_item n=3 part=touches
Refusals hold no paths, pids or ids and error messages interpolate nothing: the new not-resumed pair names the instance only and holds no session id, its hint names the literal word id; a row on these floors gains a case to hold.
@task_item n=4 part=touches
Fixed error display messages and every product error writer named: a new fixed pair and its writer in adopt join the closed set, and the exit-1 cause table and error-body list each gain a row.
@task_item n=5 part=touches
The process-exit detail word set, a closed enum: it gains not-resumed; a row whose cell lists the closed detail words or the exit-1 details of adopt now misses one.
@task_item n=6 part=touches
Replayed snapshot is inert and its fallbacks: adopt now reads the instance snapshot through the snapshot-or-replay read to decide resumed, and a replayed, absent or unreadable snapshot is taken as resumed (exit 0), never a refusal.
@task_item n=7 part=touches
Strict-modes check before the snapshot is read, snapshot reference values only after the strict-modes check, and state readers never write: adopt's new snapshot read happens after its start's checks and writes nothing; a row ruling over who reads the snapshot gains adopt's end-of-child read.
@task_item n=8 part=touches
The internal-error path of adopt: a pump worker panic still logs internal-error and exits 1 with no --json document; a row on the catch-site internal error output keeps holding but now reaches adopt through the new end-of-child step.
@task_item n=9 part=touches
Adopt acts on no process it did not start: the not-resumed end acts only on adopt's own child, which already ended; no signal to any foreign process.
@task_item n=10 part=touches
R5 adopt script ends only its own processes: the rig gains a rehearse-only arm r5c over the fake agent that ends only what the script recorded, and r5a's verdict line gains a start= closed word; its live arms type what and where they typed before.
@task_item n=11 part=touches
Test seams only under the fake-agent feature and compiled out of release, release build free of test-only features: the fake agent gains the argv-only option --resume-absent, inside the test-only binary.
@task_item n=12 part=touches
Real claude CLI only locally and CI holds no real claude: the not-resumed end is proven on the fake agent alone; no real claude ran in the session or its CI.
@task_item n=13 part=touches
The rule that a --json document is never written once the child held the terminal: adopt's not-resumed end writes its pair or document after the child released the terminal, an added named case scoped to adopt only, nothing of run or revive.
@task_fact n=1 slug=error-handling
fact: viola adopt's third own refusal not-resumed exits 1 after the resumed child ended before its session started, with the fixed pair unable: <name>'s session did not resume / hint: check the session id - claude --resume <id> must open it by hand, under the same claude config, or under --json the document with error not-resumed and detail null, written after the child released the terminal, holding the instance name only and no session id.
@task_fact n=2 slug=input-validation
fact: adopt decides resumed from its own instance snapshot only; a real snapshot with no agent session id is not resumed, while a replayed, absent or unreadable snapshot fails open to resumed and exit 0, never a refusal.
@task_item n=1 part=untouched
The IPC channel: no wrapper channel method, frame, params, error code, peer euid check or socket directory rule was added or changed.
@task_item n=2 part=untouched
The MCP server: no tool, tool input, stdin cap or isError structuredContent code changed.
@task_item n=3 part=untouched
Dependency security and CI workflows: no crate, pin, advisory ignore, licence, toolchain or workflow changed.
@task_item n=4 part=untouched
viola hook and the hook payload: no hook event, decision detail or fail-open rule changed.
@task_item n=5 part=untouched
viola verify, its probes, stamps and recorded fixture hygiene and refusal codes: untouched.
@task_item n=6 part=untouched
config.json keys, the board and viola list, link and unlink, send, answer and paste text checks: untouched.
@task_item n=7 part=untouched
viola revive's --json codes and its pump-child error contract, and viola run's own refusals: unchanged; the not-resumed exception is scoped to adopt alone.
