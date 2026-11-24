@task_prose part=session
The session added a new CLI verb, viola open: it opens one terminal window (foot) per session beside its parent's window on Hyprland, the program after -- is claude alone, flags come from a closed allowlist, and no shell is involved.
viola open waits up to 30 s for the new session to be live, then prints `<name>  live  <placement>` (or one --json ok document) and exits 0; exit 2 for usage (any program but claude, a self parent); exit 1 with a code word (strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live, internal-error).
viola run gained two hidden flags, --origin (cli or agent) and --place N; under --origin the wrapper finds its terminal's window, moves it to the place and records a window line and a snapshot window field with one of eight placement words.
list --json and the MCP list items gain an optional window (address, workspace) and origin; the help's setup: row gains open (four to five verbs); two new exit-1 codes with one fixed hint each.
After implement only the operator's live-check script for open changed; no product file moved. The real-desktop proof of open (matrix v2-07) stays gated and unproven; the allowlist proof (v2-08) is met over the fakes.
@task_file n=1 path=src/cmd/open.rs
what: added — the open verb: its closed flag allowlist, the one program claude, the parent default from the calling session, the terminal spawn, the live wait, its exit codes, its human live line and its --json documents
@task_file n=2 path=src/run/window.rs
what: added — the wrapper's window step: find the window of its own terminal, move it to the asked workspace, and name the outcome in one placement word
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: added — the test-only stand-in terminal that records the argv it was handed and runs its program as a child
@task_file n=4 path=tests/cli_open.rs
what: added — the integration tests of the open verb over the fakes: closed argv, placement words, parent and origin, the ok document, the refusals
@task_file n=5 path=scripts/r2-open.sh
what: added, then edited after implement — the operator's one live check of open on a real desktop, never run by a gate; now closes the child terminal on every exit path, reads the agent before the verdict, uses a stand-in parent, strips inherited session names and has a rehearse form
@task_file n=6 path=Cargo.toml
what: edited — the root crate's dependency list; the Hyprland seam crate moved from test-only to release dependencies
@task_file n=7 path=crates/viola-core/src/lib.rs
what: edited — the normalised event kinds (gains window) and the closed word lists for placement (8 words) and origin (cli, agent)
@task_file n=8 path=crates/viola-state/Cargo.toml
what: edited — the state crate's dependency list; gains the Hyprland seam crate for its one window address validator
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: edited — the instance snapshot document; gains an optional window record (address, workspace, placement, origin)
@task_file n=10 path=crates/viola-state/src/replay.rs
what: edited — the snapshot replay from the event log; recovers the window record, the last window line after the last launch winning
@task_file n=11 path=crates/viola-state/src/board.rs
what: edited — the board read behind viola list and the MCP list; each item gains an optional window and origin
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: edited — process liveness reads; gains the chain of ancestor processes of a pid
@task_file n=13 path=src/cmd/mod.rs
what: edited — the CLI verb table, its dispatch and the help's verb groups; the setup: row gains open
@task_file n=14 path=src/cmd/run.rs
what: edited — the run verb: two hidden flags, the window step in the start order, the window event line after launch
@task_file n=15 path=src/run/mod.rs
what: edited — the wrapper's run module root; registers the window step
@task_file n=16 path=src/human/hints.rs
what: edited — the fixed refusal and hint pairs; gains terminal-not-found and not-live
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent's entry; routes to the stand-in terminal by its file name
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: edited — the fake Hyprland control seam for tests; gains a terminal mode listing the stand-in's window
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: edited — the end-to-end Hyprland tap recorder tests; gains the refused, unseen and window-left move cases
@task_file n=20 path=schemas/diag-line.v1.json
what: edited — the diagnostic log line schema; subject terminal and process-exit details terminal-not-found and not-live added
@task_file n=21 path=tests/cli_output_plain.rs
what: edited — the plain human output tests, including the help's verb groups
@task_file n=22 path=tests/cross_exit_causes.rs
what: edited — the cross-check table of exit causes; gains the two new exit-1 rows
@task_file n=23 path=tests/contract_error_bodies.rs
what: edited — the census of --json error documents and the documents that are not error bodies; gains open's documents
@task_file n=24 path=tests/contract_diag_schema.rs
what: edited — the diagnostic schema contract tests; gains the open's exit details
@task_file n=25 path=crates/viola-state/src/strict.rs
what: edited — strict-modes checks; a test snapshot constructor gains the empty window field
@task_file n=26 path=crates/viola-mcp/tests/handlers.rs
what: edited — MCP handler tests; a snapshot constructor gains the empty window field
@task_file n=27 path=src/cmd/hook.rs
what: edited — the hook verb; its snapshot constructors gain the empty window field
@task_file n=28 path=src/run/dialog.rs
what: edited — the wrapper's dialog handling; a snapshot constructor gains the empty window field
@task_file n=29 path=src/run/wheel.rs
what: edited — the wrapper's wheel tracking; a snapshot constructor gains the empty window field
@task_file n=30 path=src/run/send/slot.rs
what: edited — the wrapper's send slot; a snapshot constructor gains the empty window field
@task_file n=31 path=tests/hook_fail_open.rs
what: edited — the hook fail-open tests; a snapshot constructor gains the empty window field
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: edited — the server identity security negatives; a snapshot constructor gains the empty window field
@task_file n=33 path=crates/viola-state/tests/state_replay.rs
what: edited — the replay tests; the replayed state now boxed
@task_file n=34 path=src/cmd/revive.rs
what: edited — the revive verb; hands no window ask and reads the boxed replayed state
@task_file n=35 path=src/cmd/list.rs
what: edited — the list verb; its board items carry the window and origin fields
@task_file n=36 path=src/human/board/test_support.rs
what: edited — the human board's test support; items carry the window and origin fields
@task_file n=37 path=src/human.rs
what: edited — the human output module; exposes the two new hint pairs
@task_file n=38 path=Cargo.lock
what: edited — the lock file; the state crate's new edge to the Hyprland seam crate
@task_file n=39 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited — the fake agent's mode tests; gains the terminal-mode case
@task_item n=1 part=touches
contract/help-output/verb-groups and contract/viola-help/verb-groups: the setup: row gains open, now open, run, revive, verify, plugin install (four to five verbs).
@task_item n=2 part=touches
contract/cli-exit-codes/1: exit 1 now also carries open's refusals, including two new codes terminal-not-found and not-live.
@task_item n=3 part=touches
contract/cli-exit-codes/2: open's usage errors are exit 2 — any word after -- but claude alone (a fixed usage line), a self parent, a flag outside the allowlist; run's --place without --origin is a usage error.
@task_item n=4 part=touches
contract/cli-exit-codes/0: open exits 0 once the session is live, whatever its placement word.
@task_item n=5 part=touches
The name/command set: the name open is now an occupied command name (no name/command/open row yet).
@task_item n=6 part=touches
decision/flat-cli-verbs: open joins as one more flat verb under setup:.
@task_item n=7 part=touches
contract/strict-modes-refusal/exit-21, contract/exit-21-cause/strict-modes-failed and contract/cli-strict-modes-refusal/instance-state-pair: open refuses strict-modes-failed as exit 1 before any log, not exit 21, as revive's --json code does.
@task_item n=8 part=touches
contract/viola-run-start-refusals/already-live, stale-heartbeat and pinned-hash-mismatch, and contract/cli-start-refusal/fixed-message: open reaches the same start refusals and code words as run and revive.
@task_item n=9 part=touches
decision/start-refusals-show-no-path-or-pid: open's refusals, terminal-not-found among them, follow the start-refusal form.
@task_item n=10 part=touches
contract/refusal-hints/* and contract/cli-refusal-line/hint: two new fixed hint pairs, terminal-not-found and not-live, one hint each.
@task_item n=11 part=touches
decision/no-hint-under-json, decision/hints-human-mode-only and decision/json-failure-documents-clean: open's --json refusals are each the clean document with error code and detail null.
@task_item n=12 part=touches
decision/json-one-document-on-stdout and contract/cli-catch-site/json-flag-scope: open's --json prints one ok document with name, parent, window (address, workspace) or null, and placement.
@task_item n=13 part=touches
contract/cli-separators/two-spaces and decision/cli-two-space-columns: open's human success line is name, live and the placement word separated by two spaces.
@task_item n=14 part=touches
contract/cli-streams/stdout and contract/cli-streams/stderr: open's live line and document are data, its refusals messages.
@task_item n=15 part=touches
decision/no-spinner and decision/no-interactive-prompt-or-pager: open waits silently up to 30 s for the session to be live; the wrapper's window step waits up to 5 s for the window.
@task_item n=16 part=touches
decision/run-prints-nothing-while-wrapped and contract/viola-run-output/child-started: run's start gains a window step under the hidden --origin, before the child starts.
@task_item n=17 part=touches
decision/mcp-hidden-verb: run now carries two hidden flags, --origin and --place, which stay off --help.
@task_item n=18 part=touches
decision/list-is-the-board, contract/viola-list-board/* and contract/mcp-verb/stdout: the list --json and MCP list items gain an optional window and origin; a window address that does not re-parse is dropped.
@task_item n=19 part=touches
contract/cli-callers/llm-driver: an open from an agent's shell names the calling session as the parent, origin agent.
@task_item n=20 part=touches
decision/linux-terminal-platform: open spawns a foot terminal and places windows through Hyprland; off Hyprland it is unsupported and still live.
@task_item n=21 part=touches
contract/log-detail-codes/*: the process-exit details gain terminal-not-found and not-live, and the spawn subject terminal is added.
@task_item n=22 part=touches
decision/no-internals-in-errors and decision/hint-never-quotes-text: open's refusals and hints name no terminal path, pid or window address.
@task_item n=23 part=touches
decision/prompt-text-from-stdin-or-file: open takes no prompt text; claude after -- is the only word accepted.
@task_fact n=1 slug=surface-cli
fact: open is an occupied command name — a setup verb that opens one terminal window per session beside its parent, the program after -- claude alone, a closed flag allowlist (parent, role overseer, builder or worker, cwd, workspace, model, effort, json), no shell.
@task_fact n=2 slug=surface-cli
fact: open's exit codes: 0 live; 2 usage (a word after -- other than claude alone, a self parent, a flag outside the allowlist); 1 with strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error, each the --json document with error code and detail null.
@task_fact n=3 slug=surface-cli
fact: open's success output is the human line name, live and placement separated by two spaces, or under --json one ok document with name, parent, window (address, workspace) or null, and placement.
@task_fact n=4 slug=surface-cli
fact: the placement word is one of eight — placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused — and the origin word one of two, cli or agent.
@task_fact n=5 slug=surface
fact: terminal-not-found and not-live are refusal codes with one fixed hint each.
@task_fact n=6 slug=surface-cli
fact: open's strict-modes-failed refusal is exit 1 before any log, an exception to the instance strict-modes refusal's exit 21.
@task_fact n=7 slug=surface-cli
fact: run carries two hidden flags, --origin (cli or agent) and --place N, kept off --help; --place without --origin is a usage error.
@task_fact n=8 slug=surface-cli
fact: list --json and MCP list items carry an optional window (address, workspace — only an address that re-parses) and an optional origin word.
@task_fact n=9 slug=surface-cli
fact: an open from inside a wrapped session names that session as its parent by default, origin agent; from a human shell the origin is cli.
@task_item n=1 part=untouched
Colour palette, colour tokens, colour depth and the colour decision order: no colour was added or moved.
@task_item n=2 part=untouched
Typography, spacing, depth, border radius and the text glyphs and arrows: no glyph, font or separator rule changed.
@task_item n=3 part=untouched
Send readback, the readback box and the send mirror: no readback form changed.
@task_item n=4 part=untouched
viola verify output and refusals, and the automatic-verify line: untouched.
@task_item n=5 part=untouched
viola link and unlink output and their --json ok documents: untouched.
@task_item n=6 part=untouched
viola wait and viola last output: untouched.
@task_item n=7 part=untouched
The wrapper fault --json detail rows, the wrapper socket refused peer and peer reply screening: untouched.
@task_item n=8 part=untouched
Exit codes 10 to 14, 20 and 21's other causes (endpoint-lost, instance-dead, server-verify-failed, socket-dir-refused, unwrapped-name): untouched.
@task_item n=9 part=untouched
Brand expression level, style direction, rejected presets, the self-validation protocol and the web page absence: untouched.
