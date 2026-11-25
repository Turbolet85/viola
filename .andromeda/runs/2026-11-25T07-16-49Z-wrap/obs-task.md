@task_prose part=session
The window step of a placing start now reads the Hyprland monitors once before its one move-to-workspace and once after, and sends one workspace request back to the earlier active workspace only when the move changed it; the placement word is still set by the move alone, and a failed read or a failed restore changes no word, field or line.
The run.window span keeps outcome alone; no seam request, placement word, argv, fixture, env var, CLI flag or crate was added; the seam's hyprctl-probe lines now pair one more or two more requests per placing start.
viola verify gains a refusal: when any of Runs B to E first read the trust form or the imports form, verify refuses with a fixed unable and hint pair, exits 1, prints no step lines and no stamped summary, leaves stamps.json untouched and, under a parsed VIOLA_NAME, logs one process-exit with detail untrusted-folder at ERROR.
The automatic verify of a start from an untrusted folder then prints its no-stamp unstamped line and leaves the version un-attempted.
The diag-line schema's process-exit detail enum gains untrusted-folder, additively; a test-only live script for the R7 placement check was added, rehearsed over the fakes only; R7 itself is unfired and the placement capability stays gated on it.
@task_file n=1 path=src/run/window.rs
what: the window step of viola run's start, the run.window span, the Hyprland seam calls that place the session's own terminal window on its workspace
@task_file n=2 path=src/cmd/verify.rs
what: viola verify, the capability-ledger stamps writer, its refusals, its stderr unable and hint pair and its process-exit line
@task_file n=3 path=schemas/diag-line.v1.json
what: the diag-line v1 schema of every process log line, its closed event, subject and detail enums
@task_file n=4 path=scripts/r7-place.sh
what: a test-only live check script for window placement on the real compositor, outside CI and the harness commands
@task_file n=5 path=tests/cli_open.rs
what: integration tests of viola open, its placing start and the seam dispatch list
@task_file n=6 path=tests/cli_verify.rs
what: integration tests of viola verify, its refusals, stamps and process log lines
@task_file n=7 path=tests/cli_verify/stamps.rs
what: integration tests of verify's stamps writing, a failing row still stamping, and verify's start and exit lines under an instance
@task_file n=8 path=tests/cli_verify/runs.rs
what: integration tests that verify leaves neither probe run dir behind
@task_file n=9 path=tests/cli_version_gate.rs
what: integration tests of the start's version gate, the automatic verify and cli_verified
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-verb exit-cause census, each detail code word with its exit code and hint
@task_file n=11 path=tests/contract_diag_schema.rs
what: the contract test of the diag-line schema enums against the emitted words
@task_file n=12 path=tests/support/verify.rs
what: test support that spawns a stamping verify, here in a given folder with no trusted root
@task_file n=13 path=tests/support/home.rs
what: test support of the integration-test home and wrapper starts as a user, here from a given folder
@task_item n=1 part=touches
contract/process-exit-detail/verify-refusal: verify's closed set of process-exit refusal detail codes gains untrusted-folder, beside program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture and interrupted.
@task_item n=2 part=touches
The verify process-exit line rows (verify-process-exit/interrupted and verify-process-exit/strict-modes-failed) take a sibling: detail untrusted-folder at ERROR, written only under a parsed VIOLA_NAME.
@task_item n=3 part=touches
contract/verify-exit-codes/1: exit 1 from verify now also covers the untrusted-folder refusal, where no row is read and no stamp is written, apart from a failing relied row that still stamps.
@task_item n=4 part=touches
contract/verify-stdout/summary-line and contract/verify-stdout/step-line: on the untrusted-folder refusal verify prints no step lines and no stamped summary line.
@task_item n=5 part=touches
contract/verify-stop-stderr/unable-hint-pair and contract/cli-stderr/refusal-pair: verify gains a second fixed refusal pair, unable: claude did not start ready in this folder: it is not trusted and hint: run viola verify from a folder you trust in Claude Code; its wording stands PROVISIONAL on a delegate's word for the epoch batch.
@task_item n=6 part=touches
The diag-line schema's process-exit detail enum gains one value, untrusted-folder; the additive-only schema decisions (diag-schema-additive-only, log-schema-additive-only, log-fields-additive-only) hold it as an additive change, and diag-schema-matches-event-enum and unemittable-schema-words-removed hold since the word is emitted.
@task_item n=7 part=touches
The distinct-detail-code rules (distinct-detail-code-per-exit-cause, exit-1-detail-codes, detail-codes-only, detail-codes-no-paths, closed-protocol-codes) gain a new exit-1 cause with its own code, carrying no path or pid.
@task_item n=8 part=touches
The diag-detail words rows (diag-detail/pinned-hash-mismatch, strict-modes-failed and siblings) and the exit-1 detail rows may take a sibling for untrusted-folder.
@task_item n=9 part=touches
decision/verify-sole-stamps-writer and decision/unreadable-ledger-means-unverified: a confounded verify now writes no stamp entry, so the version stays unverified and cli_verified reads false on the start that triggered it.
@task_item n=10 part=touches
contract/run-process-log/auto-verify-stop and the automatic verify's lines: a start from an untrusted folder leaves the version un-attempted and prints the no-stamp unstamped line, with no new run process log line.
@task_item n=11 part=touches
decision/verify-lines-carry-no-content, verify-lines-redacted and fixed-stderr-messages: the new refusal pair and its detail are fixed strings with no path.
@task_item n=12 part=touches
contract/run-span-attributes/run-window: the run.window span keeps outcome as its only attribute; a failed monitors read or a failed restore changes no outcome.
@task_item n=13 part=touches
decision/window-line-untraced and contract/run-start-events/window-line: the window line and the placement word are unchanged; no workspace id, address or pid reaches a line.
@task_item n=14 part=touches
contract/diag-subject/hyprctl-probe and contract/parse-rejected/hyprctl-json: a placing start now pairs two more monitors requests and, when the view moved, one workspace request through the seam, each parsed by the existing closed parsers.
@task_item n=15 part=touches
The CLI process-log file rows for viola open and viola up (cli-log-file/open, cli-log-file/up) and the revive and adopt start logging rows reach the changed window step through their one placing caller.
@task_item n=16 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the new live placement check runs only by hand on the host, never in CI; in CI it is rehearsed over the fakes alone.
@task_fact n=1 slug=log-coverage
fact: viola verify refuses with process-exit detail untrusted-folder at ERROR, exit 1, when any of Runs B to E first read the trust form or the imports form; it reads no row and writes no stamp.
@task_fact n=2 slug=span-trace-coverage
fact: viola verify on stderr for an untrusted folder prints the fixed pair unable: claude did not start ready in this folder: it is not trusted and hint: run viola verify from a folder you trust in Claude Code, PROVISIONAL wording.
@task_fact n=3 slug=span-trace-coverage
fact: The window step reads the monitors before and after its move and restores the earlier active workspace only when the move changed it; a failed read or restore changes no placement word, span field or log line.
@task_item n=1 part=untouched
The channel, send, wait, dialog, hook and MCP rows: no IPC method, frame, event kind, socket or correlation field changed.
@task_item n=2 part=untouched
The PII and redaction rows beyond the fixed verify pair: no new logged field, no content, path or id reaches any line.
@task_item n=3 part=untouched
The CI gate, secret-scan, artifact and perf-budget rows: no CI step, gate threshold, budget or harness command changed.
@task_item n=4 part=untouched
The panic capture and error reporting rows: no catch site, panic line or reporter changed.
@task_item n=5 part=untouched
The logging stack, init order, sink, level and service-identity rows: no subscriber, writer, config key, env var or dependency changed.
@task_item n=6 part=untouched
The other verbs' process-exit detail rows (focus, up, adopt, exit 20, exit 21, socket-dir-refused, server-verify-failed): they name other causes and stand as written.
@task_item n=7 part=untouched
The heartbeat, liveness and state-recovered rows: no snapshot, event log or heartbeat behaviour changed.
