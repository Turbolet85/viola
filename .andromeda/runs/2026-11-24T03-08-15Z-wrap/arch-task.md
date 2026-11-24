@task_prose part=session
The session added a new CLI verb, viola focus <name> [--json], that brings a wrapped session's window forward and shows its workspace on Hyprland.
It finds the window by the snapshot's recorded window address only while that window's pid is in the wrapper's ancestry, else by the nearest ancestor pid's window (an in-memory walk of at most 8), and refuses otherwise with a named code.
It makes at most three Hyprland seam calls, each under the hyprctl deadline: one clients read, a workspace dispatch only for a workspace id of 1 or more, then the window focus dispatch. It is the first product caller of the seam's FocusWindow and Workspace dispatches.
Focus connects to no wrapper socket: it reads liveness and the snapshot only, writes no snapshot or event line, and prints or logs no window address, pid or workspace.
Six exit-1 refusal codes joined the process-exit detail enum (21 words to 27); the help board row now reads list, focus; the fake hyprctl answers the focus dispatches and keeps a receipt of each dispatch argv; a gated live reading R3 script was added.
Nothing was retired. The verb rests on the delegate overseer's PROVISIONAL arms of forks F-A to F-E, with the operator's word still owed.
@task_file n=1 path=src/cmd/focus.rs
what: added — the focus verb's command module: the strict-modes check, the liveness read, the window choice by recorded address or ancestry, and the Hyprland seam dispatches with their refusal codes
@task_file n=2 path=src/cmd/mod.rs
what: edited — the CLI command set and its dispatch: the Focus subcommand, and the help template's board row
@task_file n=3 path=src/human/hints.rs
what: edited — the fixed human unable and hint line pairs, now with the focus refusals' pairs
@task_file n=4 path=src/human.rs
what: edited — the human output module's re-exports of the hint pairs
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs
what: edited — the fake agent's test-only hyprctl stand-in: its terminal mode now answers the window and workspace focus dispatches, and every mode keeps a receipt of each dispatch argv
@task_file n=6 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited — the fake agent's own unit cases for its modes, now covering the focus answers and the dispatch receipt
@task_file n=7 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema, its process-exit detail enum gaining six focus refusal words
@task_file n=8 path=tests/cli_focus.rs
what: added — the integration tests of the focus verb over the fake agent and the fake hyprctl
@task_file n=9 path=tests/cli_output_plain.rs
what: edited — the plain human output check, holding the expected help text with its board row
@task_file n=10 path=tests/cross_exit_causes.rs
what: edited — the cross-check of every exit cause and its document, six focus exit-1 rows added
@task_file n=11 path=tests/contract_error_bodies.rs
what: edited — the census of CLI error documents, six focus bodies added
@task_file n=12 path=tests/cli_error_bodies.rs
what: edited — the CLI error body tables, including the loosened socket-directory table, with focus rows
@task_file n=13 path=tests/contract_diag_schema.rs
what: edited — the diagnostics schema contract test, now taking the focus exit details
@task_file n=14 path=scripts/r3-focus.sh
what: added — the gated live reading R3 of focus on the operator's Hyprland screen, with a rehearse form over the fakes
@task_item n=1 part=touches
The name/command set: a new subcommand name, focus, is taken beside list, open and the others, and no name/command row names it yet.
@task_item n=2 part=touches
decision/human-controls-cli-only: focus is a new human control that is CLI-only, with no MCP tool for a driver. PROVISIONAL on fork F-A, the Boundary-widening escalation answered by the delegate on the conservative arm.
@task_item n=3 part=touches
contract/cli-exit-codes/1: exit 1 now also carries focus's six refusals, window-not-found, unsupported, unreachable, oversize, malformed and refused, each a {"v":1,"error":code,"detail":null} document under --json.
@task_item n=4 part=touches
contract/cli-exit-codes/21 and the exit-21 CLI document rows for instance-dead, unwrapped-name and strict-modes-failed: focus now exits 21 for a name that is not live and for a home or instance failing strict modes, before any hyprctl read.
@task_item n=5 part=touches
The CLI verbs contract set (contract/cli-verbs/*) and the viola CLI arguments set: a new verb, focus, with one ViolaName positional and --json only, and no row for it yet.
@task_item n=6 part=touches
Process-exit detail enum rows (contract/process-exit-detail/* and the other rows holding that closed enum): the enum grows from 21 words to 27 with the six focus exit-1 words.
@task_item n=7 part=touches
decision/refusal-details-closed-set and contract/code-word/format: six new closed refusal words join the set for focus.
@task_item n=8 part=touches
decision/window-features-target-hyprland: focus is a new window feature, Hyprland only; off Hyprland it refuses unsupported and spawns nothing.
@task_item n=9 part=touches
decision/hyprland-seam-crate and decision/hyprctl-direct-spawn: focus is the first product caller of the seam's FocusWindow and Workspace dispatches; the seam itself is unchanged, its other callers open and run keep their reads.
@task_item n=10 part=touches
contract/hyprland-seam-errors/* (unsupported, unreachable, oversize, malformed, refused): each seam word now maps to a focus exit-1 code of the same name, and a reply of another kind maps to malformed.
@task_item n=11 part=touches
The hyprctl read deadline: a human verb now bears it for up to three seam calls; what unreachable verifies stands on fork F-E arm 1, PROVISIONAL.
@task_item n=12 part=touches
contract/instance-snapshot/window and decision rows on the recorded window: focus reads the recorded window address from the snapshot but trusts it only while its pid is in the wrapper's ancestry; it writes nothing back.
@task_item n=13 part=touches
decision/identity-by-instance: focus picks a window by the session's own process ancestry, never by a window title or location.
@task_item n=14 part=touches
Strict-modes ordering rows (decision/strict-check-before-log-open, decision/strict-modes-check-before-log, decision/strict-read-rule): focus runs the untrusted-home check before any log opens.
@task_item n=15 part=touches
decision/no-live-endpoint-dead-or-unwrapped, decision/replayed-snapshot-gives-no-live-endpoint and contract/unreachable-cause/cause-set: focus reads the live endpoint, then the snapshot, and reports not live through the same unreachable cause as the frame verbs.
@task_item n=16 part=touches
decision/list-is-caller-side-read, decision/list-needs-no-wrapper and decision/list-reads-state-directly: focus is a second caller-side verb that sends no frame and connects to no socket; a row saying list is the only such verb would be stale.
@task_item n=17 part=touches
contract/diagnostics-role-files/cli-name and the codes-only process log rows: focus logs as the cli role into cli-<name>.ndjson under span cli.focus, with process-start and a closed process-exit detail.
@task_item n=18 part=touches
decision/sanitised-error-surfaces and the content-out-of-logs rows: focus prints and logs no window address, pid or workspace, only the name and a code.
@task_item n=19 part=touches
contract/cli-json-error-document/shape and decision/cli-output-conventions: focus success prints <name>  focused on stdout or {"v":1,"ok":{"name":...}} under --json, one document each.
@task_item n=20 part=touches
contract/human-hint-lines/* and decision/single-human-writer: six new fixed unable and hint pairs for focus, as planning forms on fork F-C, PROVISIONAL.
@task_item n=21 part=touches
The viola --help board row: list becomes list, focus, on fork F-B, PROVISIONAL; any row holding that help line is now stale.
@task_item n=22 part=touches
decision/fake-agent-test-seams and decision/tests-against-fake-agent: the fake hyprctl gains focus answers in terminal mode and a hyprctl-asked receipt beside its exe, test-only.
@task_item n=23 part=touches
decision/new-capabilities-as-verbs: the new capability came as a subcommand, as that rule says.
@task_item n=24 part=touches
decision/real-cli-probe-local-only and decision/no-live-claude-in-ci: a new gated live reading R3 runs only on the operator's host and never in CI; matrix v2-11 stays claimed GATED on it.
@task_fact n=1 slug=occupied-resources
fact: The subcommand name focus is occupied: viola focus <name> [--json], a human verb on the board that brings the named session's window forward on Hyprland.
@task_fact n=2 slug=conventions
fact: CLI verb focus takes one ViolaName and --json only; on success it prints <name>  focused, or {"v":1,"ok":{"name":"<name>"}} under --json, exit 0.
@task_fact n=3 slug=conventions
fact: viola focus exit 1 refusal codes are window-not-found, unsupported, unreachable, oversize, malformed and refused, each with detail null; exit 21 covers instance-dead, unwrapped-name and strict-modes-failed (PROVISIONAL on fork F-C for the wording, F-E for what unreachable verifies).
@task_fact n=4 slug=established-decisions
fact: focus trusts the snapshot's recorded window address only while that window's pid is in the wrapper's process ancestry, else takes the nearest ancestor pid's window; the walk is bounded to 8 ancestors and held in memory, never written.
@task_fact n=5 slug=established-decisions
fact: focus is a caller-side verb: it reads liveness and the snapshot, sends no frame, connects to no socket, writes no snapshot or event line, and makes at most three Hyprland seam calls in order (clients, a workspace dispatch only for a workspace id of 1 or more, then the window focus).
@task_fact n=6 slug=occupied-resources
fact: viola focus logs as the cli role into cli-<name>.ndjson under span cli.focus, with no window address, pid or workspace in any output or line.
@task_fact n=7 slug=established-decisions
fact: Focus has no driver-facing form: no MCP tool raises a window; adding one is a further boundary widening, only on the operator's word (PROVISIONAL, fork F-A).
@task_fact n=8 slug=occupied-resources
fact: The fake hyprctl writes a hyprctl-asked receipt beside its exe, one JSON array line per dispatch argv it was asked, test-only.
@task_item n=1 part=untouched
The wheel, its take, release and byte-source claim: focus moves no wheel and types nothing into the session.
@task_item n=2 part=untouched
Sends, paste, readiness and confirmation rows: focus sends no text and reads no screen.
@task_item n=3 part=untouched
Hook tiers, dialog hold and answer rows: no hook or dialog path changed.
@task_item n=4 part=untouched
The wrapper channel's methods, frames and fault codes: focus sends no frame and no method was added or changed.
@task_item n=5 part=untouched
The MCP server, its tools, stdin cap and exit codes: no MCP tool was added.
@task_item n=6 part=untouched
The capability ledger, stamps and viola verify: no ledger row, stamp or verify step moved.
@task_item n=7 part=untouched
Run and revive start order, pinned copies and the child's argv and environment: no start path changed.
@task_item n=8 part=untouched
Links and the event log's kinds and data: focus appends no event and no kind was added.
@task_item n=9 part=untouched
config.json keys, environment variables and dependency policy: no key, variable, flag beyond --json, or crate was added.
