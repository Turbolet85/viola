@task_prose part=session
The session made /context a listed local command whose send is confirmed by its local-command-stdout tag in the transcript, and it submits no prompt.
The capability ledger gained a 25th relied row, local-command-context; a stamp holding only the 24 earlier rows now reads unverified, and the stamps file entry gains the row and a measured context latency.
verify's Run B now pastes /context as its eighth paste. The step lines now count to 25 and the all-pass summary reads 25 pass. verify still spawns 9 claude processes, and its child set is otherwise unchanged.
Before its transcript exists, a /context send reads unconfirmable (send-issued, then send-confirmed with confirmed false, exit 0). Under --framing the fake agent echoes the /context probe literal.
The test-only live rig gained the CS and CC labels, a kept stamp and its seed, a config-top reading of names and file counts, and hint-cell attribute words. One live round on claude 2.1.328 measured the row pass in a scratch home. The live home was not re-stamped.
No IPC method, endpoint, env var, log event, log field or dependency was added, and nothing was retired.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger, which holds the relied rows and their checks, the local-command list with its post-conditions, the verify probe literals, and the stamps merge
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the capability ledger
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the capability ledger's row checks
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit cases of the ledger's verify probe literals and local-command list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit cases of the ledger checks over verify's typed probe runs
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the ledger stamps, the verified or unverified reading of a stamped version
@task_file n=7 path=src/cmd/verify.rs
what: viola verify, which owns the step lines, the summary line and the stamp write, the only stamps writer
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: verify's typed probe runs, which thread each run's measured latencies into the stamp
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: verify's Run B pastes and their transcript echo reads
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake agent that CI verify runs against; its --framing echo of the verify probe commands
@task_file n=11 path=src/run/send/classify.rs
what: the send's confirmation classification, the run.confirm_window close for an ordinary send and for each listed local command
@task_file n=12 path=src/run/version_gate.rs
what: the run version gate, which reads the ledger stamps to decide a verified or unverified CLI
@task_file n=13 path=tests/cli_send.rs
what: send integration tests, covering confirmed and unconfirmable sends and their send-issued and send-confirmed lines
@task_file n=14 path=tests/cli_verify.rs
what: verify integration tests, covering the step lines, the summary line and the spawns
@task_file n=15 path=tests/cli_verify/runs.rs
what: verify integration tests of the typed probe runs
@task_file n=16 path=tests/cli_verify/stamps.rs
what: verify integration tests of the stamps and the summary pass and fail counts
@task_file n=17 path=tests/contract_ledger_probes.rs
what: the ledger probe contract tests, which stamp every recorded fixture set over the fake agent
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: the E2E harness boot, its stamping verify and its list of ledger rows
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig, which runs the real-CLI verify and wrapped starts outside CI, its labels, kept stamp and readings
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: the test-only live rig's screen model, which reads hint cells as attribute words, never text
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig binary, with its rehearse and live words
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests, run over the fake agent
@task_item n=1 part=touches
The send confirmation window rows (run.confirm_window close for /clear, /compact and an ordinary send, and the compact confirm window decision) now have a fourth listed local command: /context closes on a new local-command-stdout tag within the fallback window and submits no prompt.
@task_item n=2 part=touches
The unconfirmable send line now covers a /context send made before its transcript exists. It writes send-issued, then send-confirmed with confirmed false and detail unconfirmable, and exits 0.
@task_item n=3 part=touches
The verify stdout step-line and summary-line rows: the relied row count is now 25 (step lines n/25, all-pass summary 25 pass 0 fail). Flag them only where a cell holds the figure rather than the relied row count.
@task_item n=4 part=touches
The verify spawn subject rows (verify-pty-probe) and the verify child set: Run B pastes /context as an eighth compiled literal in the same PTY probe. No subject was added, and the claude process count stays 9.
@task_item n=5 part=touches
Transcript echo read uninstrumented: verify now reads /context's stdout tag from the transcript end in the same uninstrumented way. An unreadable end skips only that paste.
@task_item n=6 part=touches
The verify-lines-carry-no-content and verify-lines-redacted decisions apply to the new local-command-context step line. It carries only the row id and pass or fail.
@task_item n=7 part=touches
Ledger contents never logged: the stamps file entry gains the local-command-context row and measured.typed_probe.context_latency_ms. This is additive, and it is merged by replacing the version's entry whole.
@task_item n=8 part=touches
The ledger-stamps parse-rejected row and the unreadable-ledger-means-unverified decisions: a stamp lacking the 25th row, or with it failing, reads as an unverified CLI on every binary built after this chunk. The live home's 24-row stamp is among those stamps.
@task_item n=9 part=touches
Indirectly, the unverified-CLI readings (the run log cli_verified field, the unverified-cli refusals of answer and dialog, the run.version_gate span) now apply to any version not re-stamped with 25 rows.
@task_item n=10 part=touches
CI verify against the fake agent only, and real-CLI verify never in CI: the fake's --framing echo now takes the /context probe literal. Run B's /context was read on the real CLI only by the test-only live rig, outside CI.
@task_item n=11 part=touches
verify is the only stamps writer: the test-only live rig keeps the stamp verify wrote in its scratch home and seeds it into a wrapped start's scratch ledger. It copies that stamp and writes none of its own.
@task_item n=12 part=touches
No telemetry from screen content: the test-only rig reads the attribute words of drawn hint cells past the cursor's cell, never their text. This is a test reading and no product telemetry.
@task_item n=13 part=touches
Wrapped claude internals and transcripts are not a contract: /context's stdout tag and the absence of a UserPromptSubmit were measured on claude 2.1.328 alone, and nothing is claimed for another version.
@task_item n=14 part=touches
Recorded fixtures are no telemetry source: every stamped recorded set stamps 25 of 25 over the fake with no new recording and no change to the set of stamped versions.
@task_fact n=1 slug=span-trace-coverage
fact: The run.confirm_window close for /context, a listed local command, is a new local-command-stdout tag in the transcript within the fallback window with no prompt submitted. Before its transcript exists, the send reads unconfirmable.
@task_fact n=2 slug=4-span-trace-coverage
fact: verify's Run B pastes /context among its compiled probe literals, reads its stdout tag uninstrumented from the transcript end, and records it as the relied row local-command-context.
@task_item n=1 part=untouched
Panic capture, the panic line, panic outcomes by role and the error-capture and reporting decisions. No panic path or error chain changed.
@task_item n=2 part=untouched
The process log line schema, the corr-by-event rows and the diagnostic line additive fields. No log event, field, detail code or level was added.
@task_item n=3 part=untouched
Channel, hook, MCP, dialog, wait, wheel and link rows. No IPC method, endpoint, socket or MCP tool changed.
@task_item n=4 part=untouched
PII scrubbing, veil redaction, the secret scan and the CI artifact upload gates, and the CI perf and schema gates. No upload, scan or gate step changed.
@task_item n=5 part=untouched
Metric coverage, SLO budgets, heartbeat and liveness, service identity, log file location and home resolution, and the OTel, exporter and reporter absences.
@task_item n=6 part=untouched
Exit codes and their detail words (exit 1, exit 20, exit 21, strict-modes and socket-dir refusals). No exit cause was added or moved.
