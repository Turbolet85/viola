@task_prose part=session
The session added a human CLI verb, viola focus, that raises a named session's terminal window and shows its workspace through the Hyprland seam, by the recorded window address only while that window's pid is in the wrapper's ancestry, else by the nearest ancestor pid's window.
Focus checks the home and instance for strict modes before any log opens, refuses a dead or unknown name with exit 21 before any hyprctl read, and makes at most three seam calls, each under the hyprctl deadline: one clients read, a workspace dispatch for a workspace id of 1 or more, then a window focus dispatch.
Its refusals are six closed exit-1 codes (window-not-found, unsupported, unreachable, oversize, malformed, refused), added to the process-exit detail enum; output and log lines carry no window address, pid or workspace, and focus writes no snapshot or event line.
The ancestry walk is held in memory and never written; no MCP tool, config key, env var, crate or dependency was added; nothing was retired.
The fake hyprctl (test-only) gained focus answers and a receipt of each dispatch it is asked, and a live reading script R3 was added beside R2, rehearsed over the fakes only; the live raise is unproven and gated.
Every focus decision rests PROVISIONAL on the operator's open forks F-A to F-E, answered on the conservative arm by the delegate overseer.
@task_file n=1 path=src/cmd/focus.rs
what: the new viola focus CLI verb — the client that checks strict modes, reads liveness and the snapshot's recorded window, walks the ancestry in memory, and asks the Hyprland seam to show the workspace and focus the window
@task_file n=2 path=src/cmd/mod.rs
what: the CLI command set and dispatch, with the help template's board row — the human verb list gains focus
@task_file n=3 path=src/human/hints.rs
what: the fixed human refusal pairs (unable line and hint) the CLI writes, now with focus's six pairs
@task_file n=4 path=src/human.rs
what: the human output module re-exporting the fixed refusal pairs
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test-only fake hyprctl under the fake-agent feature, answering the focus dispatches and writing a receipt of each dispatch argv it is asked
@task_file n=6 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: unit tests of the test-only fake agent's modes, including the fake hyprctl's focus answers and dispatch receipt
@task_file n=7 path=schemas/diag-line.v1.json
what: the diagnostics line schema, whose process-exit detail enum is the closed list of exit causes
@task_file n=8 path=tests/cli_focus.rs
what: integration tests of viola focus — the raise by record and by ancestry, each refusal, strict-modes refusal, no snapshot or event write, clean --json documents
@task_file n=9 path=tests/cli_output_plain.rs
what: the plain human output check, holding the help text's board row
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-check table of every exit cause against its exit code and log detail
@task_file n=11 path=tests/contract_error_bodies.rs
what: the census of CLI error documents and their fixed bodies
@task_file n=12 path=tests/cli_error_bodies.rs
what: CLI error-body tests, including the loosened-socket-directory table with a focus row
@task_file n=13 path=tests/contract_diag_schema.rs
what: the diagnostics schema contract test, now taking focus's exit details
@task_file n=14 path=scripts/r3-focus.sh
what: the live reading R3 for focus — a gated rig opening a hand-started and an opened session in foot and focusing each, with a rehearse form under a scratch HOME and runtime dir over the fakes
@task_item n=1 part=touches
decision/open-ancestor-walk-in-memory: focus walks the wrapper's ancestry (bound 8) in memory and never writes it — a sibling of open's walk.
@task_item n=2 part=touches
decision/state-tree-strict-modes-before-use and its kin (strict-modes-at-start, home-strict-modes-first, home-tree-strict-modes-before-log, start-checks-home-tree-before-log-opens): focus now checks home and instance for strict modes before any log, exit 21 strict-modes-failed.
@task_item n=3 part=touches
decision/hyprctl-read-deadline: a human verb now makes up to three hyprctl calls, each under the deadline, and the first product dispatches (window focus, workspace) beyond reads — the row stands on fork F-E arm 1.
@task_item n=4 part=touches
decision/window-record-kept-to-snapshot-and-event-log: focus reads the recorded window address from the snapshot and prints or logs no address, pid or workspace.
@task_item n=5 part=touches
decision/window-origin-authenticates-nothing, decision/hyprland-recorder-moves-own-window and decision/window-step-moves-own-terminal-window: focus trusts a recorded address only while its window's pid is in the wrapper's ancestry and raises another user-visible window — a new window-moving capability beside the wrapper's own.
@task_item n=6 part=touches
contract/cli-exit-codes/21: focus exits 21 for instance-dead, unwrapped-name and strict-modes-failed.
@task_item n=7 part=touches
contract/instance-unreachable-detail/instance-dead and /unwrapped-name, contract/instance-unreachable-cause rows: focus is a new verb producing these causes, from a liveness read with no socket connect.
@task_item n=8 part=touches
contract/strict-modes-refusal rows (cli-frame-verbs, list-human, list-json): focus is a new strict-modes refusal site that is not a frame verb.
@task_item n=9 part=touches
decision/refusals-hold-no-paths-pids-ids, decision/no-identity-values-in-refusals, decision/no-paths-in-error-output: focus's six refusals and success line carry no address, pid or workspace.
@task_item n=10 part=touches
decision/every-error-writer-named, decision/fixed-error-display-messages and decision/error-messages-interpolate-nothing: a new error writer with six fixed refusal pairs that carry the session name.
@task_item n=11 part=touches
decision/new-env-read-needs-log-entry: a new seam use (the Hyprland dispatches) — focus logs under span cli.focus with process-start and process-exit and the seam's hyprctl-probe pairs.
@task_item n=12 part=touches
decision/unwrapped-name-never-target and decision/unwrapped-session-name-not-target: focus refuses an unwrapped name with exit 21 before any hyprctl read.
@task_item n=13 part=touches
decision/cli-names-through-name-check, decision/names-through-validated-type, decision/cli-input-trust-boundary: focus takes one validated name positional and --json, nothing else.
@task_item n=14 part=touches
decision/hyprland-reply-text-dropped and decision/client-reply-code-words-only: focus maps each seam outcome to a closed code (unsupported, unreachable, oversize, malformed, refused), a reply of another kind to malformed.
@task_item n=15 part=touches
decision/snapshot-strict-checked-before-trust, decision/replayed-snapshot-inert and decision/replayed-snapshot-yields-nothing: focus reads the snapshot or its replay for the recorded window only after strict modes.
@task_item n=16 part=touches
decision/snapshot-written-only-by-wrapper and decision/state-readers-never-write: focus writes no snapshot or event line.
@task_item n=17 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake hyprctl's new focus answers and dispatch receipt live under the fake-agent feature.
@task_item n=18 part=touches
decision/client-seam-check-order and decision/frame-verbs-instance-check-first: focus orders its checks strict modes, then liveness, then the seam, and opens no socket.
@task_item n=19 part=touches
Live rig rows (live-rig-isolated-child, live-round-rig-keys-own-pty, real-cli-local-only): a new gated live reading R3 opens foot windows and focuses them; its rehearsal runs under a scratch HOME and runtime dir; R3 has not run.
@task_item n=20 part=touches
decision/refusals-never-block-human: focus's refusals affect only the window raise; nothing of the session is changed.
@task_fact n=1 slug=data-protection
fact: viola focus's ancestor walk (bound 8) is held in memory and never written, as open's.
@task_fact n=2 slug=authentication-authorization
fact: viola focus raises a recorded window only while that window's pid is in the wrapper's ancestry, else the nearest ancestor pid's window; a recorded address outside the ancestry is never focused (PROVISIONAL, fork F-A).
@task_fact n=3 slug=authentication-authorization
fact: viola focus checks the home and instance for strict modes before any log opens, refusing exit 21 strict-modes-failed, and refuses a dead or unknown name exit 21 before any hyprctl read.
@task_fact n=4 slug=error-handling
fact: viola focus refuses with six closed exit-1 codes (window-not-found, unsupported, unreachable, oversize, malformed, refused) as fixed pairs, and no output or log line of it carries a window address, pid or workspace.
@task_fact n=5 slug=data-protection
fact: viola focus writes no snapshot or event line; it only reads the recorded window.
@task_fact n=6 slug=authentication-authorization
fact: focus is a human CLI control only; no driver-facing focus (MCP tool) exists, a further widening only on the operator's word (PROVISIONAL, fork F-A).
@task_fact n=7 slug=input-validation
fact: viola focus makes at most three Hyprland seam calls, each under the hyprctl deadline — one clients read, a workspace dispatch only for a workspace id of 1 or more, then the window focus dispatch.
@task_item n=1 part=untouched
Dependency security: no crate, dependency, feature, pin, toolchain or CI workflow changed.
@task_item n=2 part=untouched
Secret management and the child's environment: no env var, credential handling or claude child environment changed.
@task_item n=3 part=untouched
MCP server rows: no MCP tool, input schema or stdin cap changed.
@task_item n=4 part=untouched
Wrapper channel methods, frames and socket rows (bind, socket directory, peer euid, server verification): focus sends no frame and connects to no socket.
@task_item n=5 part=untouched
The hook contract and its fail-open rules: untouched.
@task_item n=6 part=untouched
Config keys, the pinned binary and plugin files, release and deploy absences: untouched.
@task_item n=7 part=untouched
viola verify, its stamps, fixtures and probe residuals: untouched.
@task_item n=8 part=untouched
Paste and send text validation, answer behaviour and the wheel: untouched.
