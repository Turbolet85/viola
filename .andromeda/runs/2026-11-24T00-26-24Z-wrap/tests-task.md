@task_prose part=session
Added the CLI verb viola open: it opens one terminal window (foot) per session beside its parent, over a closed argument allowlist and no shell; any word after -- but claude alone is a usage error (exit 2) before any home read, log, pin or spawn.
Added two hidden viola run flags, --origin and --place, and a wrapper window step: under --origin the wrapper finds its terminal's window through the Hyprland seam, moves it to the target workspace, and records a window line after the launch line and a window field in the snapshot.
The replay recovers the window record (the last window line after the last launch wins, a later launch clears it); list --json and the MCP list result gain window and origin per item; the diag-line schema gains the subject terminal and the exit-1 details terminal-not-found and not-live.
Test-only: the fake agent gains a foot stand-in persona and the fake hyprctl a terminal mode; a new cli_ integration binary holds eleven open cases; three hypr_tap move refusals are now tested; an operator R2 script was added and mended twice after implement, gated by bash -n only.
Measured: v2-08 (the open's allowlist) met on the fakes and by construction; v2-07 (a real window placed on its parent's workspace) met over the fakes only and stays gated behind R2, which fired once and read red on the rig's own defect before reaching the open.
Nothing retired; no IPC method, MCP tool, socket, env var, CI job or verdict shape added.
@task_file n=1 path=src/cmd/open.rs
what: added — the viola open verb: its argument allowlist, parent target read, closed terminal argv, terminal spawn and bounded live wait, its exit codes and JSON documents
@task_file n=2 path=src/run/window.rs
what: added — the wrapper's window step: find its terminal's window by pid within a bounded wait and place it on a workspace, one placement word as outcome
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: added — the fake agent's terminal stand-in persona (foot): an argv and pid receipt, runs its words as a direct child in the working directory
@task_file n=4 path=tests/cli_open.rs
what: added — the root integration binary for viola open, eleven cases under a seeded test-owned user home with every role-file line schema-checked
@task_file n=5 path=scripts/r2-open.sh
what: added, then edited after implement — the operator's R2 live-rig script for open: a stand-in parent program, every VIOLA_ name unset, the board read with no claude on PATH, a rehearse form, never run by a gate
@task_file n=6 path=Cargo.toml
what: edited — the root package manifest: viola-hyprland moved from dev-dependencies to dependencies of the release build
@task_file n=7 path=crates/viola-core/src/lib.rs
what: edited — the core vocabulary: the normalised event kinds gain window (log-only, not a wait wake kind), the closed placement words and origin words
@task_file n=8 path=crates/viola-state/Cargo.toml
what: edited — the state crate manifest: a new crate edge to viola-hyprland for the one window address validator
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: edited — the instance snapshot: an optional window record (address, workspace, placement, origin)
@task_file n=10 path=crates/viola-state/src/replay.rs
what: edited — the snapshot replay from the event log: recovers the window record; the replayed state now boxed
@task_file n=11 path=crates/viola-state/src/board.rs
what: edited — the board read behind list --json and MCP list: items gain window and origin, an address that does not re-parse dropped
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: edited — the instance liveness reads: a bounded process ancestor chain
@task_file n=13 path=src/cmd/mod.rs
what: edited — the CLI verb dispatch and help: the Open command, its cli log sink, and open in the help's setup group
@task_file n=14 path=src/cmd/run.rs
what: edited — the viola run start: the hidden --origin and --place flags, the window step after the trust step and before the slots, the window line after launch
@task_file n=15 path=src/run/mod.rs
what: edited — the wrapper run module: the window step module registered
@task_file n=16 path=src/human/hints.rs
what: edited — the human hint lines per exit cause: one hint each for terminal-not-found and not-live
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent entry: dispatches to the foot persona by its file name
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: edited — the fake hyprctl stub: a terminal mode listing one window on the stand-in's pid
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: edited — the Hyprland tap recorder tests: the move-refused, move-unseen and window-left refusals each over a planted stub
@task_file n=20 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema: subject terminal, process-exit details terminal-not-found and not-live
@task_file n=21 path=tests/cli_output_plain.rs
what: edited — the plain CLI output literal checks: the help's setup row with open
@task_file n=22 path=tests/cross_exit_causes.rs
what: edited — the cross-surface exit cause table: two exit-1 rows terminal-not-found and not-live
@task_file n=23 path=tests/contract_error_bodies.rs
what: edited — the error-body census: five CLI documents of open and two not-error bodies
@task_file n=24 path=tests/contract_diag_schema.rs
what: edited — the diagnostics schema conformance tests: the open's exit details accepted
@task_file n=25 path=crates/viola-state/src/strict.rs
what: edited — the strict-modes checks: one line, the snapshot's new window field in a test constructor
@task_file n=26 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP handler tests: one line, the snapshot's new window field in a constructor
@task_file n=27 path=src/cmd/hook.rs
what: edited — the hook verbs: four lines, the snapshot's new window field in constructors
@task_file n=28 path=src/run/dialog.rs
what: edited — the wrapper's dialog hold: one line, the snapshot's new window field in a constructor
@task_file n=29 path=src/run/wheel.rs
what: edited — the wrapper's wheel: one line, the snapshot's new window field in a constructor
@task_file n=30 path=src/run/send/slot.rs
what: edited — the wrapper's send slot: one line, the snapshot's new window field in a constructor
@task_file n=31 path=tests/hook_fail_open.rs
what: edited — the hook fail-open tests: one line, the snapshot's new window field
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: edited — the server identity security negatives: one line, the snapshot's new window field
@task_file n=33 path=crates/viola-state/tests/state_replay.rs
what: edited — the state replay integration tests: constructors moved to the boxed replayed state
@task_file n=34 path=src/cmd/revive.rs
what: edited — the viola revive verb: hands no window ask to the start, the boxed replayed state
@task_file n=35 path=src/cmd/list.rs
what: edited — the viola list verb: items with the window and origin fields
@task_file n=36 path=src/human/board/test_support.rs
what: edited — the human board test support: items with the window and origin fields
@task_file n=37 path=src/human.rs
what: edited — the human output module: the open hint pairs exposed
@task_file n=38 path=Cargo.lock
what: edited — the lockfile: the state crate's new viola-hyprland edge
@task_file n=39 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited — the fake agent mode tests: the hyprctl terminal mode names the stand-in pid
@task_item n=1 part=touches
contract/viola-help/verb-table: the help's setup group gains open (four verbs to five: open, run, revive, verify, plugin install).
@task_item n=2 part=touches
contract/viola-list-json/items: each list --json item gains optional window {address, workspace} (only an address that re-parses) and origin (only an origin word).
@task_item n=3 part=touches
contract/viola-list-json/mirrors-channel and decision/cli-mcp-identical-payloads: the MCP list result gains the same window and origin through the shared board item.
@task_item n=4 part=touches
decision/snapshot-replay-fields: the fixed field set the replay recovers now includes window; the last window line after the last launch wins, a launch clears it, an unknown word drops the line.
@task_item n=5 part=touches
decision/snapshot-replayed-from-log, decision/snapshot-replay-from-log and decision/replayed-snapshot-has-no-endpoint: a replayed snapshot now carries the window record; open reads its own and its parent's snapshot through read-or-replay (decision/snapshot-read-through-replay).
@task_item n=6 part=touches
contract/run-start-events/order and contract/boot-readiness/events: a start under --origin appends one window line after the launch line, before the slots.
@task_item n=7 part=touches
decision/run-start-verify-and-trust-after-bind and decision/run-checks-before-child-spawn: the start gains a window step after the trust step and before the slots and the first snapshot, only under --origin.
@task_item n=8 part=touches
contract/viola-run-exit-codes/1 and the run and revive exit-1 causes (contract/start-exit-1/already-live, stale-heartbeat, pinned-hash-mismatch, strict-modes-failed): open now shares these refusals and adds exit-1 terminal-not-found and not-live.
@task_item n=9 part=touches
decision/hint-names-its-cause and decision/each-exit-cause-own-hint: two new exit causes, each with its own fixed hint pair.
@task_item n=10 part=touches
decision/error-bodies-held-by-surface-witnesses and decision/error-writers-each-named: the CLI document census gains five open rows and the not-error list two; open's pinned-hash-mismatch is witnessed by run's tampered-pin case alone.
@task_item n=11 part=touches
decision/diagnostics-schema-conformance and decision/jsonschema-validator: the diag-line schema subject enum gains terminal (9 to 10) and the process-exit detail enum gains two (19 to 21).
@task_item n=12 part=touches
decision/strict-modes-before-any-log and contract/strict-modes-refusal/other-verbs: open refuses strict-modes-failed before any log is opened.
@task_item n=13 part=touches
decision/home-read-rule and decision/home-readable-by-others-refused: open refuses a home another user may read, tested in the open binary.
@task_item n=14 part=touches
contract/fake-agent-cli/argv-only-options and decision/fake-agent-argv-only: the fake agent gains a persona chosen by its file name (foot) that writes foot-argv and foot-pid receipts.
@task_item n=15 part=touches
decision/external-processes-real-stubs and decision/mocks-only-at-product-seams: the fake hyprctl stub gains a terminal mode; the Hyprland seam now has two product callers (open's parent read, the wrapper's window step).
@task_item n=16 part=touches
decision/run-under-test-owned-user-home and decision/per-test-fresh-home: the open cases start viola run under a seeded test-owned user home with --trust-config.
@task_item n=17 part=touches
decision/integration-binary-surface-prefix: a new root integration binary joins the integration layer by its cli_ prefix.
@task_item n=18 part=touches
decision/test-started-viola-drops-env and decision/test-viola-drops-session-vars: open reads VIOLA_NAME for its parent default and origin agent; tests set it deliberately.
@task_item n=19 part=touches
decision/r8-claude-env-strip and decision/claude-env-strip: the wrapper's hyprctl reads run with the R8 strip names removed.
@task_item n=20 part=touches
decision/release-build-viola-only, decision/sync-crates-without-tokio and decision/tokio-banned-per-sync-crate: the sync crate viola-hyprland enters the release graph, with a new state-to-hyprland crate edge.
@task_item n=21 part=touches
contract/wait/wake-kinds, contract/viola-wait/non-waking-records and contract/event-line/round-trip: a new event kind window (17 to 18 kinds), log-only and not a wake kind.
@task_item n=22 part=touches
decision/fake-agent-not-proof-of-real-cli and decision/live-failing-reading-not-verification: v2-07 is met only over the fakes; R2's one red reading is recorded as a reading, v2-07 stays gated.
@task_item n=23 part=touches
decision/standing-live-rig and decision/real-claude-local-only: the R2 rig's board read ran the real claude agents list under the operator's config; mended with an empty PATH dir; the rig is gated by bash -n only.
@task_item n=24 part=touches
decision/cli-json-one-document and contract/cli-json-output/stdout-document: open prints one ok document {name, parent, window, placement} or one error document.
@task_item n=25 part=touches
decision/cli-env-vector-negative-tests and decision/cli-env-test-layers: a new argv surface (open's allowlist, run's hidden --origin and --place) with negative cases for each refusal.
@task_item n=26 part=touches
decision/child-spawn-test-layers: a new spawn, the terminal, with a closed argv one element per value and no shell.
@task_item n=27 part=touches
decision/e2e-home-link-base: a verb that opens its log before its home exists re-canonicalises the home after its log; open now does so as run does.
@task_fact n=1 slug=integration-test-strategy
fact: viola open accepts only its allowlisted flags and claude alone after --; any other word is exit 2 before any home read, log, pin or spawn, and a self parent is exit 2.
@task_fact n=2 slug=e2e-test-strategy
fact: open's exit-1 causes terminal-not-found and not-live each have a fixed hint and a named JSON error document.
@task_fact n=3 slug=integration-test-strategy
fact: viola run's hidden --place without --origin is a usage error; --origin takes only an origin word (cli or agent).
@task_fact n=4 slug=test-scope-summary
fact: the window event line holds address, workspace, placement and origin; placement is one of eight closed words (placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused).
@task_fact n=5 slug=test-data-fixtures
fact: the terminal stand-in is the fake agent under the file name foot; the fake hyprctl's terminal mode lists one window at a fixed address on the stand-in's pid, empty before it.
@task_fact n=6 slug=e2e-test-strategy
fact: the Hyprland tap's move refusals move-refused, move-unseen and window-left are each tested over a planted stub: exit 1, nothing written, the stub terminal gone.
@task_fact n=7 slug=e2e-test-strategy
fact: a real window placed on its parent's workspace on Hyprland (v2-07) is proven only by the R2 live reading; the fakes prove placement words and records only.
@task_fact n=8 slug=e2e-test-strategy
fact: a live rig's stand-in parent program must take the wrapper's own arguments, the rig unsets every inherited VIOLA_ name, and its board read runs with no claude on PATH.
@task_fact n=9 slug=11-test-anti-patterns-never-do-these
fact: an oracle asserting a refusal echoes none of the words skips the one allowed word when the fixed line itself names it.
@task_item n=1 part=untouched
CI workflows, jobs, artifacts and SHA pinning: no CI job, artifact or verdict shape changed.
@task_item n=2 part=untouched
Coverage floors, mutation gates and performance budgets: no threshold, floor or budget moved.
@task_item n=3 part=untouched
The test harness commands (boot, status, cleanup, run, gate, pre-push) and their documents and exit codes: unchanged.
@task_item n=4 part=untouched
Fuzz targets, corpora and property-test seeds: none added or changed.
@task_item n=5 part=untouched
Recorded CLI-version fixtures, verify stamps and the ledger probes: no fixture recorded, no stamp rule changed.
@task_item n=6 part=untouched
The wrapper channel JSON-RPC methods, error codes and link and unlink: no IPC method or code added.
@task_item n=7 part=untouched
The absences (Windows and macOS, database, outbound HTTP, browser suite, deploy stage): still absent.
@task_item n=8 part=untouched
Process log line format, corr fields and log file modes: no log line field changed.
