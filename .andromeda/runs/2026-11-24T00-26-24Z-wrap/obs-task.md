@task_prose part=session
Added the CLI verb viola open: it spawns one foot terminal window running viola run for a named session beside its parent, from a closed flag allowlist, with claude the only word allowed after the double dash, and no shell.
The wrapper (viola run) gains two hidden flags, --origin and --place; under --origin it runs a new window step (span run.window) after the trust record and before the slots, then appends one window event line after the launch line and writes the window into the first snapshot.
The diag-line schema gains the spawn subject terminal and two exit-1 process-exit details, terminal-not-found and not-live; EventKind gains window (17 to 18 kinds), log-only and outside the wait wake set.
The replay recovers the window record (the last window line after the last launch wins, a launch clears it); list --json and the MCP list tool carry window and origin per item; the Hyprland seam gains its first two product callers.
After implement only the operator's R2 rig script changed; R2 fired once and read red on the rig before reaching the open, so v2-07 stays claimed gated and v2-08 is met on the fakes and by construction.
@task_file n=1 path=src/cmd/open.rs
what: the new cli verb viola open: argument allowlist, the cli.open dispatch span, the terminal spawn with its process-start subject terminal, the parent window target, the live wait and its exit-1 refusals
@task_file n=2 path=src/run/window.rs
what: the wrapper's new window step and its run.window span (field outcome only) that finds and moves the terminal window through the Hyprland seam
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: the test-only stand-in for the foot terminal inside the fake agent
@task_file n=4 path=tests/cli_open.rs
what: the integration tests of viola open, schema-checking every role-file line written
@task_file n=5 path=scripts/r2-open.sh
what: the operator's one-shot R2 rig script for a real terminal on Hyprland, never run by a gate (edited after implement: stand-in parent, stripped VIOLA names, rehearse form)
@task_file n=6 path=Cargo.toml
what: the root package manifest; the viola-hyprland crate moves from a dev dependency to a dependency
@task_file n=7 path=crates/viola-core/src/lib.rs
what: the core event kind enum (gains window) and the closed placement and origin word lists
@task_file n=8 path=crates/viola-state/Cargo.toml
what: the state crate manifest; gains the viola-hyprland dependency for the window address validator
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot document; gains the optional window record
@task_file n=10 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the events log; recovers the window record and boxes the replayed state
@task_file n=11 path=crates/viola-state/src/board.rs
what: the board read behind list --json and the MCP list tool; items gain window and origin
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: the liveness reader; gains a bounded process ancestry walk
@task_file n=13 path=src/cmd/mod.rs
what: the cli verb dispatch and its help groups; adds the open verb with its cli log sink
@task_file n=14 path=src/cmd/run.rs
what: the viola run start sequence; hidden origin and place flags, the window step call, the window event line after the launch line
@task_file n=15 path=src/run/mod.rs
what: the wrapper run module; registers the window step module
@task_file n=16 path=src/human/hints.rs
what: the fixed unable and hint stderr pairs; gains the terminal-not-found and not-live pairs
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary; dispatches to the foot stand-in by its file name
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake hyprctl test double; gains a terminal mode listing the stand-in's window
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: the Hyprland seam recording tests; three move-refusal cases over a planted stub
@task_file n=20 path=schemas/diag-line.v1.json
what: the diag-line v1 schema; subject enum gains terminal, process-exit detail gains terminal-not-found and not-live
@task_file n=21 path=tests/cli_output_plain.rs
what: the plain cli output and help-text tests; the setup help row gains open
@task_file n=22 path=tests/cross_exit_causes.rs
what: the exit cause census test; gains two exit-1 causes
@task_file n=23 path=tests/contract_error_bodies.rs
what: the census of cli error documents; gains open's documents and two non-error bodies
@task_file n=24 path=tests/contract_diag_schema.rs
what: the diag-line schema contract tests; gains a case for open's exit details
@task_file n=25 path=crates/viola-state/src/strict.rs
what: a snapshot constructor in the strict-modes checks' tests gains the empty window field
@task_file n=26 path=crates/viola-mcp/tests/handlers.rs
what: a snapshot constructor in the MCP handler tests gains the empty window field
@task_file n=27 path=src/cmd/hook.rs
what: snapshot constructors in the hook verb's tests gain the empty window field
@task_file n=28 path=src/run/dialog.rs
what: a snapshot constructor in the wrapper dialog tests gains the empty window field
@task_file n=29 path=src/run/wheel.rs
what: a snapshot constructor in the wrapper wheel tests gains the empty window field
@task_file n=30 path=src/run/send/slot.rs
what: a snapshot constructor in the wrapper send slot tests gains the empty window field
@task_file n=31 path=tests/hook_fail_open.rs
what: a snapshot constructor in the hook fail-open tests gains the empty window field
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: a snapshot constructor in the server identity negative tests gains the empty window field
@task_file n=33 path=crates/viola-state/tests/state_replay.rs
what: the state replay tests; constructors box the replayed state
@task_file n=34 path=src/cmd/revive.rs
what: the revive verb's start; hands no window ask to the launch and boxes a replayed state
@task_file n=35 path=src/cmd/list.rs
what: the list verb's tests construct board items with the window and origin fields
@task_file n=36 path=src/human/board/test_support.rs
what: the human board test support builds items with the window and origin fields
@task_file n=37 path=src/human.rs
what: the human output module; re-exports the two new hint pairs
@task_file n=38 path=Cargo.lock
what: the lockfile; the state crate's new edge to viola-hyprland
@task_file n=39 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent mode tests; a case for the fake hyprctl terminal mode
@task_item n=1 part=touches
The diag-line subject enum (the contract/diag-subject rows, one subject each) gains terminal, the foot terminal spawned by viola open; the enum moves from 9 to 10 subjects.
@task_item n=2 part=touches
contract/process-exit-detail/exit-1 and the exit-1 detail rows gain terminal-not-found and not-live (process-exit detail enum 19 to 21); decision/distinct-detail-code-per-exit-cause and decision/exit-1-detail-codes now cover open's two new causes.
@task_item n=3 part=touches
contract/run-start-events/order: under --origin the start appends one window line after the launch line in the events log.
@task_item n=4 part=touches
contract/run-start-spans/span-chain and the run start span attribute rows: a new run.window span sits after the trust record step and before the slots, with the single field outcome.
@task_item n=5 part=touches
decision/launch-line-untraced and decision/lineage-line-untraced: the new window event line is log-only product audit data, untraced the same way as the launch line.
@task_item n=6 part=touches
decision/cli-manual-dispatch-span: the new verb open opens its manual dispatch span cli.open.
@task_item n=7 part=touches
contract/diagnostics-files/cli and decision/cli-log-only-under-viola-name: open logs into the cli role file cli-name.ndjson under the opened name.
@task_item n=8 part=touches
contract/child-spawn-log/subject and contract/process-log/child-and-probe-lifecycle: the terminal spawn writes a process-start line with subject terminal.
@task_item n=9 part=touches
contract/diag-subject/hyprctl-probe and contract/parse-rejected/hyprctl-json: the Hyprland seam now has two product callers (open's parent window read and the wrapper's window step), so hyprctl-probe pairs appear in product role files, not only in tests.
@task_item n=10 part=touches
decision/strict-modes-refusal-writes-no-line and decision/pre-open-strict-modes-refusal-silent: open refuses strict-modes-failed before any log opens, writing no line.
@task_item n=11 part=touches
contract/cli-json-error/strict-modes-failed and the run start failure detail rows (already-live, stale-heartbeat, pinned-hash-mismatch): open returns the same exit-1 codes and documents as a start.
@task_item n=12 part=touches
contract/cli-stderr/refusal-pair and contract/stdout-by-role/cli: open prints one human live line or its ok json document on stdout and two new unable and hint pairs (terminal-not-found, not-live) on stderr.
@task_item n=13 part=touches
contract/viola-mcp-tools/list and contract/list-json/liveness: list --json and MCP list items gain window (address and workspace, only an address that re-parses) and origin (only an origin word).
@task_item n=14 part=touches
decision/no-raw-snapshot-reads, contract/replayed-snapshot/live-fields and the state-recovered snapshot-replayed rows: the replay now recovers the window record, the last window line after the last launch winning and a launch clearing it.
@task_item n=15 part=touches
decision/event-payload-not-in-process-logs and decision/no-event-payloads-in-process-logs: the window address, pid and workspace stay in the events log and snapshot; the run.window span and process lines carry only the outcome word.
@task_item n=16 part=touches
decision/r8-strip-logged-as-names-only and decision/stripped-claude-values-never-logged: the wrapper's hyprctl reads in the window step remove the R8 strip's names from each read's environment.
@task_item n=17 part=touches
decision/instance-from-name-not-cwd and decision/self-reported-from-not-identity: open reads the caller's VIOLA_NAME as the default parent and records origin agent; the origin word is self-reported, not an identity.
@task_item n=18 part=touches
decision/home-tree-check-before-start-log and decision/home-resolution-order: open re-canonicalises its home after its cli log created it, as run does.
@task_item n=19 part=touches
decision/diag-schema-additive-only and decision/diag-schema-matches-event-enum: the schema's subject and detail enums grew additively and the event enum gained window (17 to 18 kinds).
@task_item n=20 part=touches
decision/lineage-names-confined: open passes the parent name and role on the started run's argv; they reach the wrapper's own records only.
@task_fact n=1 slug=log-coverage
fact: The diag-line subject enum holds terminal: the foot terminal window that viola open spawns, logged by its process-start and process-exit lines in the cli role file.
@task_fact n=2 slug=log-coverage
fact: The exit-1 process-exit details hold terminal-not-found (no foot terminal on PATH, nothing spawned) and not-live (the opened session never came live within the 30 s bound), both written by viola open.
@task_fact n=3 slug=span-trace-coverage
fact: The run start span chain holds run.window, entered only under the hidden --origin flag after the trust record step and before the slots, carrying the field outcome alone and no address, pid or workspace.
@task_fact n=4 slug=span-trace-coverage
fact: viola open opens the manual dispatch span cli.open and logs into the cli role file of the opened name.
@task_fact n=5 slug=span-trace-coverage
fact: The window event line (address, workspace, placement, origin; source wrapper) is log-only product audit data written after the launch line under --origin, untraced as the launch line and never copied into a process log.
@task_fact n=6 slug=log-coverage
fact: The Hyprland seam has product callers, open's parent window read and the wrapper's window step, so hyprctl-probe process pairs are written by product verbs.
@task_fact n=7 slug=pii-scrubbing-compliance
fact: The origin word (cli or agent) on the window record is self-reported by the caller and is not an identity.
@task_fact n=8 slug=telemetry-strategy
fact: viola open's strict-modes refusal falls before any log opens and writes no line, like the other pre-log strict-modes refusals.
@task_item n=1 part=untouched
Panic capture, the panic line and panic outcome by role: no panic path or catch site changed.
@task_item n=2 part=untouched
CI gates, the secret scan, artifact uploads and retention, mutation testing and pinned actions: no CI job, gate or verdict shape changed.
@task_item n=3 part=untouched
The OTel, exporter, error reporter and network egress decisions and absences: nothing exports, reports or reaches the network.
@task_item n=4 part=untouched
Metric derivation, histograms, quantiles and the hook perf budgets: no metric, budget or threshold moved.
@task_item n=5 part=untouched
Channel JSON-RPC lines, conn and corr propagation and channel params: no IPC method, socket, port or channel field added.
@task_item n=6 part=untouched
veil redaction, the detail-file line fields and the secret-scan hit report: no detail-file field or redaction rule changed.
@task_item n=7 part=untouched
config.json keys, the diagnostics level and log level filters: no config key or env-driven level added.
@task_item n=8 part=untouched
viola verify and plugin install lines, spawn subjects of verify and the capability ledger: untouched.
