@task_prose part=session
The session removed the status-line interception whole: the per-session settings override and the child's --settings on a start, the status-line source read, the snapshot's statusline_command, the viola hook statusline arm with its /bin/sh -c shell-out and its 5 s deadline, and the hidden hook statusline --capture --shell probe arm.
It retired the two capability-ledger rows statusline-override and statusline-shell (relied rows 23 to 21), the fake agent's statusline-echo mode, --statusline-stdin and statusline receipt kind, and the harness boot --statusline-echo option.
The diag-line v1 schema lost the subject statusline-shell, the hook_event word statusline and the process-exit field shell_exit_status; the spans hook.statusline and statusline.shell_out are gone; deadline and deadline_hit stay for the dialog path.
viola now runs no shell at all; the word statusline under viola hook maps to no event and takes the unknown-event exit 0, reading and writing nothing; the hook keeps one capture arm, hook <event> --capture.
The snapshot envelope v and the stamps stay at 1 under the founder's widened exception: a snapshot holding the retired key reads whole skipping it, and a stamp holding the two retired row ids reads on the relied rows alone.
--settings stays a flag of the agent crate, passed only by verify's Runs C and D; CI read green 7/7 on the commit.
@task_file n=1 path=crates/viola-agent-claude/src/statusline.rs
what: deleted; the status-line shell-out (/bin/sh -c argv) and the per-session settings override and probe documents of the claude agent crate
@task_file n=2 path=src/cmd/hook/statusline.rs
what: deleted; the viola hook statusline arm, its statusline command deadline and its hidden --capture --shell probe arm
@task_file n=3 path=src/bin/viola-fake-agent/statusline.rs
what: deleted; the fake agent's statusline-echo mode and statusline stdin replay
@task_file n=4 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: deleted; a recorded claude StatusLine fixture (scrub-required claude fixtures)
@task_file n=5 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: deleted; a recorded claude StatusLine fixture (scrub-required claude fixtures)
@task_file n=6 path=fuzz/corpus/hook_stdin/statusline-absent
what: deleted; a synthetic hook_stdin fuzz seed for the statusline arm
@task_file n=7 path=fuzz/corpus/hook_stdin/statusline-epoch
what: deleted; a synthetic hook_stdin fuzz seed for the statusline arm
@task_file n=8 path=schemas/claude-statusline-fixture.v1.json
what: deleted; the schema of the claude StatusLine fixture
@task_file n=9 path=tests/hook_statusline.rs
what: deleted; the integration tests of viola hook statusline (passthrough, deadline, malformed and oversize stdin, shell-out lines)
@task_file n=10 path=tests/support/statusline.rs
what: deleted; the test support helpers for the statusline arm
@task_file n=11 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: its rows, the relied set (now 21) and the stamps it reads and writes
@task_file n=12 path=crates/viola-agent-claude/src/lib.rs
what: the claude flag literals of the agent crate (--plugin-dir, and --settings moved here for verify's Runs C and D)
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the tests pinning the relied-row verdict vector of the capability ledger
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the tests of the ledger stamps read and write, now with stamps holding the retired row ids
@task_file n=15 path=crates/viola-agent-claude/src/hook.rs
what: the claude hook event words viola hook maps, statusline no longer among them
@task_file n=16 path=src/cmd/hook.rs
what: the viola hook dispatch: event and dialog arms, the one capture arm, the unknown-event silent exit 0
@task_file n=17 path=src/cmd/run.rs
what: the viola run start: the status-line source read, the settings override write and the snapshot record removed from it
@task_file n=18 path=src/run/mod.rs
what: the claude child launch of viola run (the claude-child spawn: --plugin-dir then the caller's args, no settings path)
@task_file n=19 path=src/cmd/verify.rs
what: viola verify: its runs, step lines and summary over the relied ledger rows (21), Run B's probe settings removed
@task_file n=20 path=src/cmd/verify/typed.rs
what: verify's typed runs inputs, its statusline settings and captures removed
@task_file n=21 path=src/cmd/verify/typed/runs.rs
what: verify's typed Runs C and D, the remaining callers passing --settings to the claude child
@task_file n=22 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot.json write and read of the state crate, statusline_command dropped, unknown keys skipped on read
@task_file n=23 path=crates/viola-state/src/replay.rs
what: the snapshot replay of the state crate, by which product readers read the snapshot
@task_file n=24 path=schemas/diag-line.v1.json
what: the diag-line v1 schema: the subject enum, the hook_event enum and the process-exit fields
@task_file n=25 path=fixtures/viola-0.1.0/retired-words.json
what: the fixture of retired words (snapshot key, ledger rows, hook word, source file, diag subject) the removal's tests read
@task_file n=26 path=fixtures/viola-0.1.0/snapshot.json
what: the 0.1.0-shaped snapshot fixture, holding the retired statusline_command for the compat read
@task_file n=27 path=src/bin/viola-fake-agent.rs
what: the fake agent: its mode words, options and receipt kinds; --settings accepted and ignored
@task_file n=28 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness command line, boot losing --statusline-echo
@task_file n=29 path=crates/viola-e2e/src/harness/run.rs
what: the harness run: its LEDGER_ROWS and live verify pass expectation at 21 rows
@task_file n=30 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot of a session in a test home
@task_file n=31 path=crates/viola-e2e/src/harness/session.rs
what: the harness session options
@task_file n=32 path=Cargo.toml
what: the workspace manifest and its test targets, hook_statusline removed
@task_file n=33 path=.config/nextest.toml
what: the nextest config with the ci kill-line override filter, binary(hook_statusline) removed
@task_file n=34 path=tests/hook_fail_open.rs
what: the hook fail-open integration tests, now with the retired hook word as an unknown event writing nothing
@task_file n=35 path=tests/cli_instance_state.rs
what: the instance state integration tests: no settings file in the instance dir, user settings left as found
@task_file n=36 path=tests/contract_diag_schema.rs
what: the diag-line schema contract test over emitted lines
@task_file n=37 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract (MAX_FRAME and size caps over every reader), its statusline rows removed
@task_file n=38 path=tests/contract_os_gated_tests.rs
what: the OS-gated test census, its statusline rows removed
@task_file n=39 path=tests/tui_passthrough.rs
what: the run terminal passthrough tests (zero viola bytes in the outer stream), the statusline passthrough case removed
@task_file n=40 path=tests/cli_verify/runs.rs
what: the verify run tests, summary counts restated on 21 rows
@task_item n=1 part=touches
The statusline command rows of section 1 retire with their subject: decision/statusline-command-confined, decision/statusline-command-duration-exit-only, decision/statusline-command-string-diagnostics-only, decision/statusline-command-never-in-home-logs, decision/statusline-command-only-in-diagnostics, decision/statusline-shell-out-instrumentation.
@task_item n=2 part=touches
The statusline rows of section 4 retire: decision/statusline-command-never-logged, decision/statusline-deadline, decision/statusline-stdout-passthrough, decision/no-statusline-settings-in-telemetry, decision/rejected-statusline-stdin-runs-nothing, decision/statusline-settings-steps-unlogged, decision/statusline-silent-without-instance.
@task_item n=3 part=touches
The statusline contracts of section 4 retire: contract/budget-span-attributes/statusline-shell-out, contract/statusline-decision-detail/deadline, /malformed-json, /oversize-stdin, contract/statusline-log-fields/hook-decision, /hook-invoked, /parse-rejected, /statusline-shell, contract/statusline-spans/span-chain.
@task_item n=4 part=touches
The hook stdout and output rows: contract/hook-stdout/statusline and contract/hook-output/statusline-stdout retire; contract/hook-stdout/capture-arms and contract/stdout-by-role/hook name one capture arm, hook <event> --capture.
@task_item n=5 part=touches
The capture-arm rows go from two arms to one: decision/capture-arms-uninstrumented, decision/hidden-capture-arms-uninstrumented, decision/hook-capture-arms-unlogged, and decision/uninstrumented-components where it lists the hidden capture arms.
@task_item n=6 part=touches
The diag-line schema words removed: contract/diag-subject/statusline-shell retires; contract/diag-line-fields/process-exit loses shell_exit_status (deadline, deadline_hit stay for the dialog path); contract/diag-line-fields/process-start loses the statusline-shell subject; contract/diag-line-fields/hook-events and contract/corr-by-event/hook lose the statusline hook_event word; contract/hook-decision-detail/statusline-codes retires.
@task_item n=7 part=touches
contract/child-spawn-log/subject (child and shell spawn log events): viola spawns no shell any more, so the shell spawn subject leaves; contract/process-log/child-and-probe-lifecycle likewise where it names the status-line shell.
@task_item n=8 part=touches
The schema-evolution rules take a removal: decision/diag-schema-additive-only, decision/log-schema-additive-only, decision/log-fields-additive-only and decision/log-field-names-frozen meet three enum and field removals with v staying 1; decision/unemittable-schema-words-removed is the rule they follow.
@task_item n=9 part=touches
decision/span-naming and decision/span-naming-skip-all lose the statusline area and the spans hook.statusline and statusline.shell_out; contract/hook-spans/span-chain drops any statusline span; decision/static-span-names where it lists them.
@task_item n=10 part=touches
contract/replayed-snapshot/live-fields drops statusline_command from the snapshot rebuilt by replay; decision/no-raw-snapshot-reads loses hook statusline as a product reader of the snapshot (six readers to five where a row counts them).
@task_item n=11 part=touches
decision/statusline-command-deadline (section 6) retires with the 5 s statusline deadline; decision/hook-latency-budget and the hook perf budget rows where they name the statusline arm.
@task_item n=12 part=touches
decision/statusline-command-out-of-home-logs (section 8) retires; decision/endpoint-and-pinned-bin-never-logged and the snapshot field lists in section 8 where they name statusline_command.
@task_item n=13 part=touches
decision/child-settings-never-logged: viola run no longer passes a settings argument to the claude child; only verify's Runs C and D still pass --settings. contract/run-process-log/process-start-claude-child and contract/claude-child-process-log/handle-wait-exit where they describe the child's argv.
@task_item n=14 part=touches
viola run start rows: contract/run-start-spans/span-chain, contract/run-span-attributes/state-snapshot-write and contract/run-start-events/order lose the status-line source read and the settings override write where they name them.
@task_item n=15 part=touches
The hook unknown-event path gains the word statusline: decision/hook-always-exits-zero, contract/hook-fail-open/exit and decision/hook-silent-no-op-without-instance now cover viola hook statusline as an unknown event, exit 0, writing nothing.
@task_item n=16 part=touches
The capability ledger relied set is 21 rows: contract/verify-stdout/step-line and contract/verify-stdout/summary-line where a cell holds 23; decision/ledger-contents-never-logged and decision/unreadable-ledger-is-unverified where they name the statusline-override or statusline-shell rows.
@task_item n=17 part=touches
decision/claude-fixtures-scrub-required, decision/fixtures-not-telemetry and decision/fixtures-not-a-telemetry-source where they name the StatusLine fixtures or their workspace.repo field: that fixture family is gone.
@task_item n=18 part=touches
decision/os-gated-test-census and decision/fuzz-artifacts-synthetic-seeds-only where they list the statusline tests or the two hook_stdin statusline seeds.
@task_item n=19 part=touches
decision/hook-instance-check-before-obs-init, decision/instance-check-before-log-init and decision/hook-single-log-open where they list the statusline arm among the arms checked before obs init: the event and dialog arms remain.
@task_fact n=1 slug=1-obs-scope-summary
fact: Absence: a status-line interception; viola neither overrides the child's settings, nor reads a status-line source, nor runs a status-line command, nor serves the status line from a hook arm; the wrapped status line is claude's own, exactly as unwrapped.
@task_fact n=2 slug=log-coverage
fact: viola runs no shell at all: no diag-line subject, field or span records a shell-out.
@task_fact n=3 slug=span-trace-coverage
fact: viola hook statusline is an unknown hook event: exit 0, nothing read, no line written, the home's tree byte-equal.
@task_fact n=4 slug=log-coverage
fact: Exception to the additive-only diag schema: the status-line removal took the subject statusline-shell, the hook_event word statusline and the process-exit field shell_exit_status out of diag-line v1 with v staying 1, by the founder's word.
@task_fact n=5 slug=span-trace-coverage
fact: A snapshot.json written by 0.1.0 holding statusline_command reads whole with the key skipped, and a ledger stamp holding the retired rows statusline-override and statusline-shell reads verified on the 21 relied rows alone; snapshot and stamp v stay 1.
@task_item n=1 part=untouched
The channel, send, wait, last, link and wheel paths: their log fields, detail codes, spans and corr rows are not reached.
@task_item n=2 part=untouched
The dialog hook path keeps deadline and deadline_hit and its fail-open shape; dialog log fields and the dialog span chain do not change.
@task_item n=3 part=untouched
The MCP server rows: stdout frames, tool refusals, MAX_FRAME on mcp stdin and the mcp log sink.
@task_item n=4 part=untouched
Panic capture and error reporting (section 7): the panic line, panic outcome by role, the detail-file line and the absence of an error reporter.
@task_item n=5 part=untouched
The logging stack itself: the tracing subscriber, the obs_event macro, the file sink per role, levels and targets, owner-only modes and home resolution.
@task_item n=6 part=untouched
The CI gates of section 9 and the SLO budgets of section 10 other than any statusline perf row: G1, G2, G4, the secret scan, artifact retention and the zero-panics budget.
@task_item n=7 part=untouched
Metrics (section 5), heartbeat liveness and the absences of OTel, network egress, a database and gRPC.
@task_item n=8 part=untouched
The global status line of the user stays untouched by viola; rows saying viola leaves the user's claude settings as found stay true and only grow stronger.
