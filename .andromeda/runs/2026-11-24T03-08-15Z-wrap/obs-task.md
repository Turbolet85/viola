@task_prose part=session
Added the CLI verb viola focus <name> [--json]: it raises a session's window and shows its workspace through Hyprland, by the recorded window address while its pid is in the wrapper's ancestry, else by the nearest ancestor pid's window.
Focus runs as the cli role: strict-modes check before any log, then logs into the instance's cli process log under span cli.focus (no fields), with process-start and a process-exit carrying a closed detail code, plus the hyprctl seam's hyprctl-probe pairs (at most three seam reads: clients, workspace dispatch, window dispatch).
Six new exit-1 refusal codes: window-not-found, unsupported, unreachable, oversize, malformed, refused; the diag-line schema's process-exit detail enum grew from 21 to 27 words. Exit 21 reuses instance-dead, unwrapped-name and strict-modes-failed.
Focus prints and logs no window address, pid or workspace; it writes no snapshot and no event line; its ancestry walk is held in memory only. The help's board row reads list, focus.
The fake hyprctl (test-only) gained focus answers and a hyprctl-asked receipt of each dispatch argv; a gated live reading script R3 was added, not fired. The verb, its help row and its refusal wording are PROVISIONAL on the operator's forks F-A, F-B, F-C and F-E.
@task_file n=1 path=src/cmd/focus.rs
what: the viola focus cli verb: its dispatch, span cli.focus, liveness and snapshot read, the hyprctl seam reads and dispatches, the ancestry walk choosing the window, its exit-1 and exit-21 refusal codes
@task_file n=2 path=src/cmd/mod.rs
what: the cli command set and dispatch, the verb list and the help template's board row
@task_file n=3 path=src/human/hints.rs
what: the fixed unable and hint stderr pairs per refusal code, now focus's six pairs
@task_file n=4 path=src/human.rs
what: the human-output module re-exporting the fixed refusal pairs
@task_file n=5 path=schemas/diag-line.v1.json
what: the diag-line v1 schema, the process-exit detail enum of closed detail codes
@task_file n=6 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test-only fake hyprctl of the fake agent, its terminal mode answers and the hyprctl-asked dispatch receipt
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: unit tests of the fake agent's modes, here the fake hyprctl's focus answers and receipt
@task_file n=8 path=scripts/r3-focus.sh
what: the gated live reading R3 of focus on the operator's screen, with a rehearse form over the fakes
@task_file n=9 path=tests/cli_focus.rs
what: integration tests of viola focus, including its cli log lines checked against the diag-line schema and no snapshot or event write
@task_file n=10 path=tests/cli_output_plain.rs
what: the plain-output check of the cli, including the help text's board row
@task_file n=11 path=tests/cross_exit_causes.rs
what: the cross-verb table of exit causes and their detail codes
@task_file n=12 path=tests/contract_error_bodies.rs
what: the census of cli --json error documents checked for clean bodies
@task_file n=13 path=tests/cli_error_bodies.rs
what: the cli error-body tables, including the loosened socket-directory table
@task_file n=14 path=tests/contract_diag_schema.rs
what: the diag-line schema contract tests, here that the schema takes focus's exit details
@task_item n=1 part=touches
contract/process-exit-detail/exit-1 and the exit-1 detail code rows (contract/exit-1-detail/*): six new words window-not-found, unsupported, unreachable, oversize, malformed, refused, emitted by focus.
@task_item n=2 part=touches
Any row holding the process-exit detail enum's size or word list: 21 words became 27; the schema change is additive (decision/diag-schema-additive-only, decision/log-schema-additive-only, decision/unemittable-schema-words-removed: every new word is emittable by focus).
@task_item n=3 part=touches
decision/exit-1-detail-codes and decision/distinct-detail-code-per-exit-cause: each focus refusal cause has its own distinct code.
@task_item n=4 part=touches
contract/cli-log-file/open and its siblings (link-unlink, revive-list, revive-start): focus logs into the same cli process log, a new verb beside them.
@task_item n=5 part=touches
decision/cli-manual-dispatch-span and decision/static-span-names, decision/span-field-allow-list: a new manual cli span cli.focus with no fields.
@task_item n=6 part=touches
Exit 21 rows (contract/exit-21-detail/instance-dead, unwrapped-name, strict-modes-failed; contract/unreachable-cause/*; contract/cli-exit-21/*; contract/no-endpoint-instance/classification; contract/strict-modes-refusal-exit/21; contract/exit-21-document/*): focus now emits these for a dead or unknown name and a home failing strict modes.
@task_item n=7 part=touches
Strict-modes ordering rows (decision/strict-modes-before-diagnostics, decision/home-tree-check-before-log-init, decision/pre-open-strict-modes-refusal-silent, decision/strict-modes-failure-lines, decision/instance-check-before-log-init): focus checks strict modes before any log opens.
@task_item n=8 part=touches
contract/diag-subject/hyprctl-probe and contract/parse-rejected/hyprctl-json: focus is a new caller of the hyprctl seam, up to three reads each logged as a hyprctl-probe pair; oversize and malformed seam words map to focus's codes.
@task_item n=9 part=touches
decision/detail-codes-no-paths, decision/no-paths-or-pids-in-detail-codes, decision/home-level-lines-path-free: focus logs and prints no window address, pid or workspace.
@task_item n=10 part=touches
contract/cli-stderr/refusal-pair and decision/fixed-stderr-messages, decision/stderr-fixed-strings: six new fixed unable and hint pairs for focus (PROVISIONAL, F-C).
@task_item n=11 part=touches
contract/stdout-by-role/cli and decision/short-lived-verbs-stderr: focus prints name then focused, or one --json document, on stdout.
@task_item n=12 part=touches
decision/no-raw-snapshot-reads, decision/no-endpoint-over-replayed-snapshot, decision/replayed-snapshot-unreachable: focus reads the snapshot through replay after a live-endpoint check.
@task_item n=13 part=touches
decision/cli-log-only-under-viola-name and decision/instance-from-name-not-cwd: focus takes one ViolaName positional and resolves the instance from it.
@task_item n=14 part=touches
decision/error-bodies-clean and contract/cli-json-error rows: focus's --json refusal documents carry detail null, one document each.
@task_item n=15 part=touches
decision/uninstrumented-components and decision/liveness-transitions-not-logged: focus's in-memory ancestry walk writes no line.
@task_item n=16 part=touches
decision/diag-lines-schema-checked-in-unit-tests and decision/schema-conformance-gate: focus's emitted cli lines are schema-checked in its integration tests.
@task_item n=17 part=touches
decision/fixtures-not-telemetry and decision/fixtures-not-a-telemetry-source: the fake hyprctl answers focus with recorded FocusWindow and Workspace fixtures and keeps a test-only dispatch receipt.
@task_fact n=1 slug=4-span-trace-coverage
fact: viola focus logs as the cli role into the instance's cli process log under the manual span cli.focus with no fields, a process-start and a closed-detail process-exit (PROVISIONAL, F-A).
@task_fact n=2 slug=log-coverage
fact: focus's exit-1 detail codes: window-not-found (no window of the session, nothing dispatched), unsupported (not on Hyprland, nothing spawned), unreachable (no compositor answering, PROVISIONAL F-E), oversize, malformed and refused (the hyprctl seam's words).
@task_fact n=3 slug=pii-scrubbing-compliance
fact: focus never prints or logs the window address, a pid or a workspace, in success or refusal.
@task_fact n=4 slug=4-span-trace-coverage
fact: focus's ancestry walk is held in memory, never written or logged; a recorded window address outside the wrapper's ancestry is never focused.
@task_fact n=5 slug=4-span-trace-coverage
fact: focus writes no snapshot and no event line; it is no product event and leaves the events log unchanged.
@task_fact n=6 slug=otel-sdk-init
fact: focus checks home and instance strict modes before any log line (exit 21 strict-modes-failed), and a dead or unknown name exits 21 before any hyprctl read.
@task_fact n=7 slug=log-coverage
fact: focus makes at most three hyprctl seam reads (clients, then a workspace dispatch only for a workspace 1 or above, then the window dispatch), each a hyprctl-probe pair under the hyprctl read deadline.
@task_item n=1 part=untouched
The viola-mcp stdio tools and MCP refusal rows: no MCP focus tool was added; a driver-facing focus is left out.
@task_item n=2 part=untouched
Hook rows: hook stdin, decision, fail-open, latency budgets and capture arms are unchanged.
@task_item n=3 part=untouched
Channel, JSON-RPC, send, wait, last, wheel and dialog log rows and their span chains.
@task_item n=4 part=untouched
viola run start rows, its span attributes, claude-child lines and the run process log.
@task_item n=5 part=untouched
viola verify, revive, link and unlink rows beyond sharing the cli log file.
@task_item n=6 part=untouched
config.json keys, log levels, service identity, heartbeat and liveness thresholds.
@task_item n=7 part=untouched
Performance budgets, secret scan and upload rows, and the no-exporter and no-reporter absences.
