@task_prose part=session
Added the CLI verb viola open: it starts one terminal (foot) per session beside its parent, takes a closed flag allowlist, accepts only the word claude after the dashes, and spawns the terminal directly with a closed argv, one element per value, no shell.
viola run gained two hidden flags, --origin (closed words cli or agent) and --place (a workspace number); under --origin the wrapper runs a window step after the trust step and before the slots: it finds its own terminal's window through the Hyprland seam and moves it once, then records a window line in the event log and a window field in the snapshot.
The replay recovers the window record; viola list --json and the MCP list item gain window and origin, shown only when the address re-parses and the origin is a closed word; the parent window is read from the parent's snapshot or from up to 8 ancestor pids held in memory.
New exit-1 codes terminal-not-found and not-live, a diag-line subject terminal, a new event kind window (17 to 18 kinds); the Hyprland crate now enters the release graph and the state crate depends on it for its one address validator; no IPC method, MCP tool, socket, port or env var added.
After implement only the operator's R2 rig script changed: it now strips every inherited VIOLA_* name, runs the board read with an empty PATH so no real claude agents read is spawned, uses a stand-in parent program that takes the wrapper's arguments, and has a rehearse form; R2 fired once red before reaching the open, so the real-terminal capability stays gated and unproven.
@task_file n=1 path=src/cmd/open.rs
what: the viola open verb, the launch authority: its flag allowlist, the only-claude check after the dashes, the self-parent refusal, the strict-modes and pinned-hash checks, the closed terminal argv, the direct terminal spawn without a shell, the parent window lookup and the bounded live wait
@task_file n=2 path=src/run/window.rs
what: the wrapper's window step: finds the window of its own terminal through the Hyprland seam within a bounded wait and makes one move to the asked workspace, yielding a closed placement word
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: test-only stand-in terminal under the fake-agent feature, recording its argv and running its words as a direct child
@task_file n=4 path=tests/cli_open.rs
what: integration tests of viola open over the fakes, including the allowlist refusal, the closed argv, the strict-modes refusal of a readable home and every written line schema-checked
@task_file n=5 path=scripts/r2-open.sh
what: the operator's live rig for the real-terminal round: scratch home and cwd, every VIOLA_* name stripped, board read with an empty PATH so no real claude is run, a stand-in parent program, cleanup of its own terminals on every exit path, a rehearse form; never run by a gate
@task_file n=6 path=Cargo.toml
what: the root crate manifest, where the Hyprland seam crate moved from a dev-dependency into the release dependency graph
@task_file n=7 path=crates/viola-core/src/lib.rs
what: the core crate's shared closed vocabularies: the event kinds (gains window) and the closed placement and origin word lists
@task_file n=8 path=crates/viola-state/Cargo.toml
what: the state crate manifest, gaining the Hyprland seam crate for its window address validator
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot document, gaining an optional window record (address, workspace, placement, origin)
@task_file n=10 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the event log, recovering the window record from closed words and a re-parsed address, a launch line clearing it
@task_file n=11 path=crates/viola-state/src/board.rs
what: the board read behind viola list --json and the MCP list tool, showing window and origin only when they re-parse
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: process liveness reads, gaining a bounded ancestor-pid walk held in memory
@task_file n=13 path=src/cmd/mod.rs
what: the CLI verb table and its diagnostics sinks, gaining the open verb and its cli log
@task_file n=14 path=src/cmd/run.rs
what: viola run's argument checks and start order, gaining the hidden origin and place flags, the hyprctl read under the R8 strip and the window line written after the launch line
@task_file n=15 path=src/run/mod.rs
what: the wrapper's run module, registering the window step
@task_file n=16 path=src/human/hints.rs
what: the fixed human hint lines, gaining the terminal-not-found and not-live pairs
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary under the fake-agent feature, dispatching to the stand-in terminal by its file name
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test-only fake hyprctl, gaining a terminal mode that lists one window on the stand-in's pid
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: the Hyprland recorder's tests, gaining move-refused, move-unseen and window-left refusals over a planted stub, nothing written
@task_file n=20 path=schemas/diag-line.v1.json
what: the diagnostics line schema closing subjects and process-exit details, gaining subject terminal and details terminal-not-found and not-live
@task_file n=21 path=tests/cli_output_plain.rs
what: the help output tests, the setup row gaining open
@task_file n=22 path=tests/cross_exit_causes.rs
what: the cross-check of exit causes, gaining the two exit-1 rows
@task_file n=23 path=tests/contract_error_bodies.rs
what: the census of every product error document, gaining open's five documents and two non-error bodies
@task_file n=24 path=tests/contract_diag_schema.rs
what: the diagnostics schema contract tests, gaining open's exit details
@task_file n=25 path=crates/viola-state/src/strict.rs
what: the strict-modes check's tests, a fixture snapshot gaining the empty window field
@task_file n=26 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler tests, a fixture snapshot gaining the empty window field
@task_file n=27 path=src/cmd/hook.rs
what: viola hook's tests, fixture snapshots gaining the empty window field
@task_file n=28 path=src/run/dialog.rs
what: the wrapper's dialog handling tests, a fixture snapshot gaining the empty window field
@task_file n=29 path=src/run/wheel.rs
what: the wheel (human typing takes the wheel) tests, a fixture snapshot gaining the empty window field
@task_file n=30 path=src/run/send/slot.rs
what: the send slot tests, a fixture snapshot gaining the empty window field
@task_file n=31 path=tests/hook_fail_open.rs
what: the hook fail-open tests, a fixture snapshot gaining the empty window field
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: the server-identity security negative tests, a fixture snapshot gaining the empty window field
@task_file n=33 path=crates/viola-state/tests/state_replay.rs
what: the replay tests, the replayed state now boxed
@task_file n=34 path=src/cmd/revive.rs
what: viola revive, handing no window ask to the start and constructing the boxed replayed state
@task_file n=35 path=src/cmd/list.rs
what: viola list, carrying the board item's window and origin
@task_file n=36 path=src/human/board/test_support.rs
what: the human board's test support, its items gaining window and origin
@task_file n=37 path=src/human.rs
what: the human output module, exposing the new hint pairs
@task_file n=38 path=Cargo.lock
what: the lockfile, the state crate's new edge to the Hyprland seam crate
@task_file n=39 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests, gaining the fake hyprctl terminal-mode case
@task_item n=1 part=touches
The CLI input trust boundary and the launch-flag checks gain a new verb: viola open's flags are a closed allowlist refused by the parser before dispatch, and any word after the dashes but claude alone is a usage error before any home read, log, pin or spawn.
@task_item n=2 part=touches
viola run's lineage and launch flag checks gain two hidden flags, --origin over closed words and --place as an integer, --place without --origin refused; the started run's argv is widened by them (boundary widening W1).
@task_item n=3 part=touches
Rules on spawning without a shell and on the child argv as one element per value (session id argv, revive trailing words, verify children direct spawn) gain a new spawn: open spawns the terminal directly with a closed argv.
@task_item n=4 part=touches
Programs resolved from PATH: open resolves the terminal program from PATH (refused terminal-not-found when absent) and the wrapper's window step resolves hyprctl on PATH, with the R8 strip's names removed from each read; rows on child PATH trust, claude resolved from PATH and the pinned folder first on PATH sit beside this.
@task_item n=5 part=touches
The Hyprland recorder rows (moves only its own terminal's window, string allowlist, reply text dropped) and the hyprctl read deadline now have product callers: open's parent window read and the wrapper's window step, which moves only the window of its own terminal, once, under a 5 s bound.
@task_item n=6 part=touches
Lineage and launch pair kept to the snapshot and the event log, and the self-reported parent and launch pair authenticating nothing: the new window record and origin are kept the same way, and origin is self-reported and authenticates nothing.
@task_item n=7 part=touches
Board rows that show values only when they re-parse (launch pair, lineage and links) gain window (address re-parsed by the one validator) and origin (closed word) on viola list --json and the MCP list tool (boundary widening W3); the MCP tools list contract gains these optional fields.
@task_item n=8 part=touches
Replayed-snapshot rows (inert, fallbacks, runs nothing and frames nothing, classified read only) now cover the window record recovered from the log by closed words.
@task_item n=9 part=touches
Strict-modes-first rules (strict modes at start, home tree before log opens, unstattable or readable home refused) apply to the new open verb: strict-modes-failed before any log, a home another user may read refused.
@task_item n=10 part=touches
The start collision check (already-live, stale-heartbeat) and pinned-hash-mismatch refusals are now also emitted by open, each as the fixed --json error document with no detail.
@task_item n=11 part=touches
Every product error writer named and the error-document census: open adds documents for strict-modes-failed, terminal-not-found, not-live, pinned-hash-mismatch and already-live, and its fixed usage line; refusals hold no paths, pids or ids.
@task_item n=12 part=touches
Process-exit details and diag subjects closed by the schema: subject terminal and exit-1 details terminal-not-found and not-live added; the run.window span carries the outcome only, no address, pid or workspace in a process-log line.
@task_item n=13 part=touches
Rules requiring a log entry for a new seam or environment read: the Hyprland seam gains product callers, open reads the existing VIOLA_NAME through the existing own-name read for the parent default and origin agent, and parent pids are read through an ancestor walk.
@task_item n=14 part=touches
Bounded external readers census: new product reads are the parent's snapshot, the hyprctl clients list, and the ancestor pid walk capped at 8.
@task_item n=15 part=touches
Sync crates free of tokio, cargo deny gate on the root graph and the release graph: the Hyprland seam crate (a sync crate) moved into the release dependency graph, and the state crate now depends on it; no outside dependency added, cargo deny green.
@task_item n=16 part=touches
Test seams under the fake-agent feature and compiled out of release, release build refusing test-only binaries: a new stand-in terminal named foot and a fake hyprctl terminal mode were added under the fake agent.
@task_item n=17 part=touches
Live rig rows (isolated child, config seeded, credentials copy, kept stamp, tests use their own home): the R2 rig measured that viola list under a scratch home still ran the real claude agents read under the user's real config, and that an inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN leaked into the rig; the rig now strips every VIOLA_* name and empties PATH for the board read.
@task_item n=18 part=touches
claude agents read resolved from the caller's PATH and fail-open: the measured R2 finding shows a board read under a scratch home still reaches the user's real claude config unless PATH holds no claude.
@task_item n=19 part=touches
Snapshot written only by the instance's wrapper and the event kinds: the wrapper's first snapshot and one new window line after launch, a new log-only event kind window (event kinds 17 to 18).
@task_fact n=1 slug=input-validation
fact: viola open takes a closed flag allowlist (name, --parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json) and only the single word claude after the dashes; any other word, or a parent equal to the name, is a usage error (exit 2) before any home read, log, pin or spawn.
@task_fact n=2 slug=input-validation
fact: viola open spawns the terminal directly, never through a shell, with a closed argv, one element per value, and starts only the pinned viola run with claude.
@task_fact n=3 slug=input-validation
fact: viola run's hidden --origin takes only the closed words cli or agent and --place only an integer, --place without --origin refused; these are a widening of the started run's argv (W1).
@task_fact n=4 slug=authentication-authorization
fact: The recorded origin (cli or agent) is self-reported by the caller and authenticates nothing.
@task_fact n=5 slug=authentication-authorization
fact: The wrapper's window step moves only the window of its own terminal, found by its parent pid, at most once, and never a window it did not open.
@task_fact n=6 slug=data-protection
fact: The window record (address, workspace, placement, origin) is kept to the snapshot and the event log; no address, pid or workspace enters a process-log line.
@task_fact n=7 slug=data-protection
fact: Open's parent-window lookup walks at most 8 ancestor pids held in memory only and never recorded (W2).
@task_fact n=8 slug=input-validation
fact: viola list --json and the MCP list item carry window only when its address re-parses through the one address validator and origin only when it is a closed origin word (W3).
@task_fact n=9 slug=input-validation
fact: The wrapper's hyprctl read for the window step resolves hyprctl on PATH and removes the R8 strip's names from each read, under a 5 s window wait.
@task_fact n=10 slug=error-handling
fact: viola open's exit-1 refusals terminal-not-found and not-live are fixed documents with no detail and one fixed hint each, naming no path or pid.
@task_fact n=11 slug=secret-management
fact: A live rig that runs viola commands strips every inherited VIOLA_* name first and runs the board read with no claude on its PATH, since a board read under a scratch home otherwise runs the real claude agents read under the user's real config.
@task_item n=1 part=untouched
Channel and IPC surface: no wrapper channel method, frame, socket, peer euid check or server verification changed; no new listener or port.
@task_item n=2 part=untouched
MCP tools other than list, the MCP stdin cap and rmcp features: no tool added or changed beyond list's two optional fields.
@task_item n=3 part=untouched
Paste and send text validation, control-character sets, dialog free text and answer behaviour: untouched.
@task_item n=4 part=untouched
Hook payload parsing, hook fail-open and the hook capture arm: untouched beyond fixture snapshots.
@task_item n=5 part=untouched
CI workflows, action pins, toolchain pins, fuzzing, advisory lists, licence allow list and MSRV: none changed; no outside dependency added.
@task_item n=6 part=untouched
Secrets, credentials strip of CLAUDE values, canary scan classes and pre-push gate: untouched.
@task_item n=7 part=untouched
Verify, its stamps, probes and fixture refusal codes, and plugin install and pinned-copy placement: untouched.
@task_item n=8 part=untouched
Absences: still no network listener, outbound call, model API call, accounts or secrets; no config key or env var added.
