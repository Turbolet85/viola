@task_prose part=session
Added the cli verb viola adopt: a plain claude session, given by its session id, resumed under viola with an operator-given name, an optional parent and an optional model and effort pair.
Adopt starts like run and revive start: it logs as process run into the run log file of its name, through run's own process-start, process-exit and agents-probe lines and run's spans.
Its preflight order: home-tree strict-modes check, pinned exe check, run log open, name-holder collision check, then the session-live check over claude agents; a failed agents read still starts.
Its own refusals are two closed details, strict-modes-failed and session-live (exit 1, fixed unable and hint pair, or the --json document); run's shared details already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error also reach it.
No new event, detail word, field, subject, span, env read, time bound or MCP tool; the diag-line schema is unchanged; the session id appears in no line, pair or detail.
The real-CLI check (R5) stays gated and unproven; the W1 to W4 widenings stand PROVISIONAL on the delegate's word.
@task_file n=1 path=src/cmd/adopt.rs
what: the viola adopt cli verb, new: a resume start of a plain claude session under a name and parent, its preflight order, its refusals strict-modes-failed and session-live, its lineage
@task_file n=2 path=src/cmd/revive.rs
what: viola revive: the session-id parser and the live-elsewhere (session-live) check, now shared with adopt, bodies unchanged
@task_file n=3 path=src/cmd/mod.rs
what: cli verb dispatch with its detail sink of process run, and the help's setup row of served verbs
@task_file n=4 path=src/main.rs
what: the role taken from argv before clap and the silent roles at the internal-error catch site; adopt added as a named role
@task_file n=5 path=scripts/r5-adopt.sh
what: new test-only live-reading script for the real-CLI adopt check R5, with a rehearsal over the fake agent; never run by a gate
@task_file n=6 path=tests/cli_adopt.rs
what: new adopt integration test binary: resume under name and parent, session-live refusals, strict-modes refusal before any log, the internal-error document, schema-checked run lines
@task_file n=7 path=tests/cli_output_plain.rs
what: plain cli output test: the help text literal and the served verb list
@task_file n=8 path=tests/cross_exit_causes.rs
what: the cross-verb exit-cause table test: exit code, detail and hint per cause
@task_file n=9 path=tests/contract_error_bodies.rs
what: the error-body census test over each writer of a refusal or error body
@task_file n=10 path=tests/security_negatives_env_capture.rs
what: security negative env-capture tests, the verify_window class with its 80 s kill
@task_file n=11 path=tests/support/home.rs
what: integration test support: the test home and wrapper harness, now with an adopt start in an outer PTY
@task_item n=1 part=touches
contract/strict-modes-refusal-exit/1: adopt's start joins the starts whose home or instance strict-modes refusal exits 1.
@task_item n=2 part=touches
decision/home-tree-check-before-log-init and decision/home-tree-check-before-start-log: adopt's start runs the home-tree check before its run log opens.
@task_item n=3 part=touches
Strict-modes refusal before a log opens writes no line (decision/strict-modes-refusal-writes-no-line, decision/pre-open-strict-modes-refusal-silent, decision/pre-log-strict-refusals-write-no-line, decision/refused-home-tree-writes-no-line, decision/strict-modes-failure-lines): adopt's refusal is one more such start.
@task_item n=4 part=touches
decision/revive-start-logs-as-run: adopt's start likewise logs as process run; contract/cli-log-file/revive-start has an adopt counterpart.
@task_item n=5 part=touches
contract/run-log-file/path and contract/diagnostics-files/run: adopt writes run-<name>.ndjson, role run.
@task_item n=6 part=touches
session-live rows (contract/exit-1-detail/session-live, contract/run-exit-1-detail/session-live, contract/revive-preflight-detail/session-live): adopt now emits session-live, exit 1, with its own hint pair.
@task_item n=7 part=touches
decision/revive-preflight-order: adopt's preflight is a second order of the same shape (strict-modes, pinned exe, log open, holder, session-live, launch).
@task_item n=8 part=touches
decision/session-live-check-fails-open and contract/revive-process-log/agents-probe: adopt's session-live check fails open on a failed agents read and logs the agents-probe line.
@task_item n=9 part=touches
Run start failure detail rows (contract/run-start-failure-detail/*, contract/process-exit/socket-dir-refused-on-start): already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused now also reach adopt's start.
@task_item n=10 part=touches
decision/role-classified-before-clap and the internal-error catch rows (contract/cli-internal-error/json-document, contract/cli-internal-error/human-line): the word adopt is a named role, silent at the catch site; under --json an error before the child pumps prints the internal-error document.
@task_item n=11 part=touches
claude agents output rows (decision/agents-cli-output-untrusted, decision/agents-probe-output-untrusted, decision/agents-cli-parse-failure-unknown, decision/claude-agents-rows-never-logged): adopt is a new reader of claude agents for its session-live check.
@task_item n=12 part=touches
Run start events and child rows (contract/run-start-events/order, contract/claude-child-process-start/*, contract/run-process-log/process-start-claude-child): adopt's start launches the child with the resume argument and the typed-in id through run's start.
@task_item n=13 part=touches
decision/detail-codes-no-paths and decision/typed-ids-never-in-detail: the operator-typed session id appears in no line, pair or detail of adopt.
@task_item n=14 part=touches
Lineage rows (decision/lineage-line-untraced, decision/lineage-names-confined): adopt writes lineage parent from --parent else the inherited name.
@task_item n=15 part=touches
contract/run-terminal-output/start-refusal and contract/cli-stderr/refusal-pair: adopt's refusals are one fixed unable and hint pair.
@task_fact n=1 slug=span-trace-coverage
fact: viola adopt's start logs as process run, into the run log file of its name, through run's own lines and spans; it adds no event, detail word, field, subject or span.
@task_fact n=2 slug=log-coverage
fact: adopt's preflight order is home-tree strict-modes check, pinned exe check, run log open, name-holder collision check, then the session-live check, then the launch; the first refusal wins.
@task_fact n=3 slug=log-coverage
fact: adopt refuses session-live, exit 1, when claude agents lists the id or a live adopted instance holds it; a failed agents read starts (fails open).
@task_fact n=4 slug=log-coverage
fact: process-exit detail for exit 1 from viola adopt: strict-modes-failed, session-live, and run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
@task_fact n=5 slug=error-capture-reporting
fact: the role word adopt is classified before clap and silent at the internal-error catch site; under --json an error before the child is pumped prints the internal-error document.
@task_fact n=6 slug=pii-scrubbing-compliance
fact: the operator-typed session id of viola adopt is never logged: no line, pair or detail carries it.
@task_item n=1 part=untouched
The diag-line schema and its enums: no event, subject, detail word or field added; schema conformance rows stand.
@task_item n=2 part=untouched
MCP server, hook, channel, send, wait, last, dialog and wheel rows: adopt reaches none of these paths.
@task_item n=3 part=untouched
CI gates, perf budgets, secret scan and artifact rows: no gate, job, threshold or budget moved.
@task_item n=4 part=untouched
Heartbeat, liveness and staleness threshold rows: no time bound or threshold changed.
@task_item n=5 part=untouched
Span naming and span attribute rows: adopt opens no span of its own and runs run's start spans unchanged.
@task_item n=6 part=untouched
Error reporter, OTel, exporter and network egress absences: nothing added.
@task_item n=7 part=untouched
viola verify and plugin install rows: unchanged by this session.
