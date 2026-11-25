@task_prose part=session
The session built the not-resumed end of viola adopt: when the resumed claude child ends before its session starts, adopt now refuses with a third own refusal, not-resumed, and exits 1.
Adopt's start logs the child's process-exit (subject claude-child) and then one self process-exit with exit_code 1 and detail not-resumed; a resumed child still ends exit 0 with no detail.
The refusal is written after the child released the terminal: the fixed unable and hint pair on stderr, or under --json the document with error not-resumed and detail null.
The not-resumed criterion is the wrapper's own snapshot, read through replay: not resumed only when a real snapshot holds no agent session id; a replayed, absent or unreadable snapshot counts as resumed.
The diagnostic line schema's closed process-exit detail enum gains the word not-resumed; the fake agent gains the argv-only option --resume-absent; the R5 adopt rig gains a start field on r5a and a rehearse-only r5c line.
No event kind, subject, field, span, IPC method, env var or config key was added; run and revive keep their pump and exit mapping unchanged.
@task_file n=1 path=src/cmd/run.rs
what: edited - viola run's start and the pump of the claude child: the child's process-exit logged on every end, the self process-exit mapping for run and revive (exit 0, or exit 1 internal-error on a worker panic), now split so adopt can decide its own self exit
@task_file n=2 path=src/cmd/adopt.rs
what: edited - the viola adopt cli verb: its own refusals (strict-modes-failed, session-live, now not-resumed), their fixed unable and hint pairs and --json refusal documents, and its start's self process-exit detail
@task_file n=3 path=schemas/diag-line.v1.json
what: edited - the diag-line v1 schema of the diagnostic log line: the closed process-exit detail enum gains not-resumed
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: edited - the fake agent standing in for claude in tests: under --resume-absent a resumed start ends exit 1 before any SessionStart
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited - the fake agent's option-parsing unit tests: a case for --resume-absent
@task_file n=6 path=tests/cli_adopt.rs
what: edited - the viola adopt integration tests: the not-resumed pair, the --json document, the self and claude-child process-exit lines schema-valid with no session id, and a later adopt taking the name over
@task_file n=7 path=tests/cross_exit_causes.rs
what: edited - the exit-1 cause table checking each cause has its own detail word and writer: one row for not-resumed
@task_file n=8 path=tests/contract_error_bodies.rs
what: edited - the cli --json error-body contract table: one body for not-resumed
@task_file n=9 path=scripts/r5-adopt.sh
what: edited - the R5 adopt rig script: r5a's verdict line ends with the last self process-exit detail as start, and a rehearse-only arm r5c over the fakes
@task_item n=1 part=touches
decision/adopt-start-logs-as-run: adopt's start no longer adds no detail word of its own - it adds not-resumed, an exception for adopt's not-resumed end only (the operator's own answer).
@task_item n=2 part=touches
contract/process-exit-detail/exit-1-adopt: the exit-1 details of viola adopt gain not-resumed beside session-live, written after the claude-child process-exit.
@task_item n=3 part=touches
contract/process-exit-detail/exit-1 and the other exit-1 detail rows (exit-1-detail, run-exit-1-detail): check whether a list of exit-1 detail words is held closed; not-resumed is adopt's own, not shared with run or revive.
@task_item n=4 part=touches
The closed process-exit detail enum in the diag-line schema gains one word; rows holding the enum's words or its closedness (detail-codes-only, process-log-line/enum-values, diag-detail words, unemittable-schema-words-removed, diag-schema-additive-only) may need the word.
@task_item n=5 part=touches
decision/distinct-detail-code-per-exit-cause and decision/exit-1-detail-codes: a new exit-1 cause with its own distinct detail code, not-resumed, and its own writer (adopt).
@task_item n=6 part=touches
contract/claude-child-process-log/handle-wait-exit and contract/run-process-log/process-exit-self: the claude-child process-exit still logged on every end; on adopt the self process-exit now follows from adopt, not from run's pump mapping.
@task_item n=7 part=touches
decision/run-silent-terminal, decision/run-silent-on-terminal and contract/run-terminal-output rows: adopt now writes a refusal pair or document after the child ended and released the terminal; run itself stays silent.
@task_item n=8 part=touches
contract/cli-stderr/refusal-pair and contract/list-stderr/unable-hint-pair style rows, decision/fixed-stderr-messages and decision/stderr-fixed-strings: a new fixed unable and hint pair, unable: name's session did not resume, hint: check the session id - claude --resume id must open it by hand.
@task_item n=9 part=touches
decision/adopt-session-id-never-logged: the not-resumed pair and lines name the instance only, never the session id (held, asserted by a test).
@task_item n=10 part=touches
decision/no-raw-snapshot-reads and decision/replayed-snapshot-unreachable: adopt's new resumed check reads the snapshot through replay and treats a replayed snapshot as resumed.
@task_item n=11 part=touches
decision/session-live-check-fails-open: adopt's resumed check likewise fails open - an absent or unreadable snapshot counts as resumed, exit 0.
@task_item n=12 part=touches
contract/panic-outcome-by-role/run and decision/run-worker-panic-ends-run: on adopt a worker panic still ends with internal-error, exit 1, no document.
@task_item n=13 part=touches
decision/ci-verify-fake-agent-only: the not-resumed end is proven on the fake agent alone; no real claude ran.
@task_fact n=1 slug=log-coverage
fact: process-exit detail not-resumed - viola adopt's own exit-1 cause, a self process-exit exit_code 1 written after the claude-child process-exit when the resumed child ends before its session starts; the stderr pair or --json document follows once the child released the terminal.
@task_fact n=2 slug=log-coverage
fact: Adopt's not-resumed check fails open - only a real snapshot with no agent session id is not resumed; a replayed, absent or unreadable snapshot counts as resumed and ends exit 0.
@task_item n=1 part=untouched
viola revive's start and its pump: its exit mapping and process log are unchanged.
@task_item n=2 part=untouched
Hook, MCP, channel, send, wait, dialog and wheel log lines and spans: no event, field, span or corr changed.
@task_item n=3 part=untouched
Event stream events.ndjson and its start product order: no event added or reordered.
@task_item n=4 part=untouched
Exit 20 and exit 21 detail words, socket-dir and server-verify refusals, strict-modes refusals: no change.
@task_item n=5 part=untouched
Panic capture, PII scrubbing regime, CI gates, perf budgets and SLO thresholds: no gate, budget or threshold moved.
@task_item n=6 part=untouched
Logging stack, sinks, file paths, modes, levels and service identity: no change.
