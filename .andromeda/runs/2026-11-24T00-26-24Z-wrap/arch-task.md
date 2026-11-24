@task_prose part=session
Added the CLI verb viola open: it starts a named wrapped claude session in a new foot terminal window beside its parent, under a closed flag allowlist, with only claude accepted after --, spawning the terminal directly with a closed argv and no shell, then waiting up to 30 s for the session to go live.
Added two hidden run flags, --origin (cli or agent) and --place N. Under --origin the wrapper runs a window step after the trust step and before the slots: it finds its terminal's window through the Hyprland seam, moves it to the target workspace, and records the result.
Added a log-only event kind window (address, workspace, placement, origin), appended once after launch. The snapshot gains window, the replay recovers it (the last window line after the last launch wins, and a later launch clears it), and list --json and MCP list items gain window and origin.
Moved the crate edges: viola now depends on viola-hyprland as a normal dependency, and viola-state depends on viola-hyprland for its one window-address validator. Added the diag subject terminal and two exit-1 details, terminal-not-found and not-live. EventKind went from 17 to 18 kinds.
Measured: on the fakes, v2-08 is met and v2-07 is met. The live R2 reading ran once and came back red on the rig (the stand-in parent refused the wrapper's arguments), so v2-07 stays claimed and gated. After implement, only the operator's R2 script changed, no product file.
@task_file n=1 path=src/cmd/open.rs
what: the new viola open verb: its argument allowlist, the parent-window target, the closed terminal argv, the direct terminal spawn, the live wait and the ok and error documents
@task_file n=2 path=src/run/window.rs
what: the wrapper's window step: it finds the window of its own terminal through the Hyprland seam and moves it to the place, with a 5 s bound
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: the test-only fake agent's stand-in for the foot terminal, which records its argv and pid and runs its words as a direct child
@task_file n=4 path=tests/cli_open.rs
what: the integration tests of viola open over the fake agent and the fake hyprctl
@task_file n=5 path=scripts/r2-open.sh
what: edited; the operator's one live R2 rig script for viola open on real Hyprland, which no gate runs
@task_file n=6 path=Cargo.toml
what: the root crate manifest: viola-hyprland moved from dev-dependencies to dependencies
@task_file n=7 path=crates/viola-core/src/lib.rs
what: viola-core's normalised event kinds (window added) and the closed placement and origin word lists
@task_file n=8 path=crates/viola-state/Cargo.toml
what: the viola-state crate manifest: a new dependency edge on viola-hyprland
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot shape: the optional window record
@task_file n=10 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the event log: it recovers window, and its recovered state is boxed
@task_file n=11 path=crates/viola-state/src/board.rs
what: the board read model behind CLI list and MCP list: the item's window and origin
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: process liveness reads: the new bounded parent-process ancestor chain
@task_file n=13 path=src/cmd/mod.rs
what: the CLI subcommand table and dispatch: the open command and its cli log sink
@task_file n=14 path=src/cmd/run.rs
what: the viola run verb and the wrapper start path: the hidden --origin and --place flags, the window step in the start order, and the window line and snapshot field
@task_file n=15 path=src/run/mod.rs
what: the wrapper runtime module root: the window module registered
@task_file n=16 path=src/human/hints.rs
what: the human hint lines: the pairs for terminal-not-found and not-live
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary: dispatch to the foot stand-in
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test-only fake hyprctl: a terminal mode that lists the stand-in terminal's window
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: the Hyprland tap recording tests: move-refused, move-unseen and window-left over a planted stub
@task_file n=20 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the subject terminal and the process-exit details terminal-not-found and not-live
@task_file n=21 path=tests/cli_output_plain.rs
what: the plain CLI output tests: the help's setup row
@task_file n=22 path=tests/cross_exit_causes.rs
what: the cross-verb exit-cause census
@task_file n=23 path=tests/contract_error_bodies.rs
what: the census of CLI error documents and the bodies that are not error bodies
@task_file n=24 path=tests/contract_diag_schema.rs
what: the diagnostics schema contract tests: the open's exit details
@task_file n=25 path=crates/viola-state/src/strict.rs
what: the strict-modes read rule, edited here only in a test's snapshot constructor for window
@task_file n=26 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler tests, edited here only in a snapshot constructor for window
@task_file n=27 path=src/cmd/hook.rs
what: the hook verb, edited here only in its tests' snapshot constructors for window
@task_file n=28 path=src/run/dialog.rs
what: the wrapper's dialog hold, edited here only in a test's snapshot constructor for window
@task_file n=29 path=src/run/wheel.rs
what: the wrapper's wheel, edited here only in a test's snapshot constructor for window
@task_file n=30 path=src/run/send/slot.rs
what: the wrapper's send slot, edited here only in a test's snapshot constructor for window
@task_file n=31 path=tests/hook_fail_open.rs
what: the hook fail-open tests, edited here only in a snapshot constructor for window
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: the server-identity security negatives, edited here only in a snapshot constructor for window
@task_file n=33 path=crates/viola-state/tests/state_replay.rs
what: the state replay tests: the boxed replayed state at its constructors
@task_file n=34 path=src/cmd/revive.rs
what: the viola revive verb: it hands no window ask to the start path
@task_file n=35 path=src/cmd/list.rs
what: the viola list verb, adjusted for the board item's window and origin
@task_file n=36 path=src/human/board/test_support.rs
what: the human board's test support: item constructors with window and origin
@task_file n=37 path=src/human.rs
what: the single human writer module, adjusted for the open hint pairs
@task_file n=38 path=Cargo.lock
what: the workspace lockfile: the viola-state to viola-hyprland edge
@task_file n=39 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests: the hyprctl terminal-mode case
@task_item n=1 part=touches
decision/run-start-order: a window step now runs under --origin, after the trust step and before the slots and the first snapshot. The child is still spawned last.
@task_item n=2 part=touches
decision/snapshot-replay-log-fields-only and contract/snapshot-envelope/unreadable-snapshot: the replay now also recovers window from the log.
@task_item n=3 part=touches
contract/instance-snapshot/data and the instance-snapshot rows: the snapshot gains an optional window record (address, workspace, placement, origin). It is omitted when absent (decision/absent-optional-fields-omitted).
@task_item n=4 part=touches
contract/normalised-event-kinds rows and decision/event-kinds-agent-neutral: the set of 17 kinds gains an 18th, window, added in viola-core.
@task_item n=5 part=touches
contract/wrapper-event-appends rows and contract/event-line/source: the wrapper now appends one window line (source wrapper) after launch, only under --origin.
@task_item n=6 part=touches
decision/wrapper-events-log-only, contract/channel-methods/wait-wake-kinds and contract/channel-wait/wake-kinds: the window line is log-only and is not a wait wake kind.
@task_item n=7 part=touches
contract/event-data rows: a new per-kind data shape for window.
@task_item n=8 part=touches
name/command rows: the command name open is now taken.
@task_item n=9 part=touches
contract/cli-verbs rows, contract/viola-cli-arguments rows and decision/new-capabilities-as-verbs: a new verb, open, with its own flags; run gains the hidden flags --origin and --place, and --place without --origin is a usage error.
@task_item n=10 part=touches
contract/cli-exit-codes/1 and contract/cli-exit-codes/2: exit 1 gains terminal-not-found and not-live. Exit 2 now also covers any word after -- other than claude alone, and a self parent.
@task_item n=11 part=touches
contract/process-exit-detail rows and contract/diag-line/format: the process-exit detail enum goes from 19 to 21, and the diag subject enum from 9 to 10 with terminal.
@task_item n=12 part=touches
decision/hyprland-seam-crate and the crate-dependency-direction rows: viola now takes viola-hyprland as a normal dependency, and viola-state takes a new edge to viola-hyprland, so the seam enters the release graph.
@task_item n=13 part=touches
decision/board-read-model-shape, decision/sync-crates-tokio-free and decision/no-tokio-in-sync-crates: the board read model's crate now depends on the compositor seam crate, which is sync.
@task_item n=14 part=touches
decision/hyprctl-direct-spawn and contract/hyprland-seam-errors rows: the seam now has two product callers (open's parent read and the wrapper's window step), and its error classes surface as placement words.
@task_item n=15 part=touches
decision/window-features-target-hyprland: this is the first product window feature built on Hyprland.
@task_item n=16 part=touches
decision/no-shell-out and decision/no-tmux-or-shell-dependency: open spawns the terminal directly with a closed argv, one element per value, and no shell.
@task_item n=17 part=touches
decision/child-program-resolution and decision/hooks-and-mcp-never-search-path: open resolves the terminal program foot itself, and the wrapper resolves hyprctl on PATH.
@task_item n=18 part=touches
contract/list-item rows, contract/list-payload/items and contract/mcp-tools/list: list items gain an optional window (address and workspace, kept only when the address re-parses) and an optional origin.
@task_item n=19 part=touches
decision/process-liveness-signals and the session-liveness rows: a new bounded ancestor-chain read of parent pids (depth 8), held in memory.
@task_item n=20 part=touches
decision/child-env-claude-strip, decision/env-strip-names-only and decision/r8-identity-floor: each hyprctl read of the window step removes the R8 strip's names from its environment.
@task_item n=21 part=touches
contract/child-environment/viola-name and decision/viola-env-namespace-product-only: open reads VIOLA_NAME to default the parent and to self-report origin agent.
@task_item n=22 part=touches
decision/lineage-label-only and contract/viola-cli-arguments/run-parent: open hands the parent on to the started run as lineage.
@task_item n=23 part=touches
decision/no-start-over-live-instance and decision/run-refuses-held-name: open refuses already-live and stale-heartbeat before any spawn.
@task_item n=24 part=touches
decision/strict-check-before-log-open, decision/strict-modes-check-before-log and decision/start-pins-the-bytes-it-checked: open checks strict modes before any log and pins the bytes it checked, and it can refuse with pinned-hash-mismatch.
@task_item n=25 part=touches
contract/diagnostics-role-files/cli-name: open writes its process log to cli-<name>.ndjson.
@task_item n=26 part=touches
decision/viola-home-resolution: open re-canonicalises its home after its log creates it, as run does.
@task_item n=27 part=touches
decision/fake-agent-test-seams, name/binary/viola-fake-agent and decision/test-binaries-never-released: the fake agent gains a foot stand-in under the file name foot and a hyprctl terminal mode, both test-only.
@task_item n=28 part=touches
decision/four-interfaces-one-binary and decision/cli-output-conventions: open prints human text by default and one ok document or a named error document under --json.
@task_fact n=1 slug=occupied-resources
fact: The command name open is occupied by viola open, the verb that starts a named wrapped claude session in a new terminal window.
@task_fact n=2 slug=established-decisions
fact: viola open starts a session in a foot terminal it spawns directly with a closed argv, no shell. It accepts only its allowlisted flags (parent, role overseer, builder or worker, cwd, workspace, model, effort, json), takes claude alone after --, and waits up to 30 s for the session to go live.
@task_fact n=3 slug=established-decisions
fact: open targets the workspace of the parent's recorded window first, and otherwise the window of the nearest process ancestor up to 8 levels; with neither, the window is left unplaced.
@task_fact n=4 slug=established-decisions
fact: The wrapper's window step waits up to 5 s for its terminal's window, polling every 100 ms, and makes at most one move to the asked workspace.
@task_fact n=5 slug=established-decisions
fact: A window's origin is self-reported through the hidden run flag --origin and is never verified.
@task_fact n=6 slug=conventions
fact: The normalised event kinds include window, a log-only wrapper kind that does not wake a wait.
@task_fact n=7 slug=conventions
fact: Placement words are a closed set of eight (placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused), and origin words a closed set of two (cli, agent), both defined in viola-core.
@task_fact n=8 slug=standard-contracts
fact: The wrapper appends one window event line, with data address, workspace, placement and origin, after the launch line, and only under --origin.
@task_fact n=9 slug=standard-contracts
fact: The instance snapshot carries an optional window record of address, workspace, placement and origin.
@task_fact n=10 slug=standard-contracts
fact: In the snapshot replay, the last window line after the last launch line wins, and a later launch line clears the window.
@task_fact n=11 slug=standard-contracts
fact: A list item carries an optional window of address and workspace (only an address that re-parses) and an optional origin (only an origin word), on both CLI list and MCP list.
@task_fact n=12 slug=standard-contracts
fact: viola open exits 0 when live, 2 on usage, and 1 on strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error. Its --json ok document holds name, parent, window and placement.
@task_fact n=13 slug=occupied-resources
fact: The process-exit details terminal-not-found and not-live and the diag subject terminal are occupied by viola open.
@task_fact n=14 slug=occupied-resources
fact: viola run's hidden flags --origin and --place are occupied, and --place requires --origin.
@task_fact n=15 slug=crate-dependency-direction
fact: viola-state depends on viola-hyprland, whose address check is the one window-address validator, and viola depends on viola-hyprland in its release graph.
@task_item n=1 part=untouched
The capability ledger, its stamps and the verify verb's probes, refusals and exit details: nothing in them moved.
@task_item n=2 part=untouched
The wrapper channel's JSON-RPC methods, frames, fault codes and protocol version: no channel method or frame was added or changed.
@task_item n=3 part=untouched
config.json keys and configuration precedence: no config key was added.
@task_item n=4 part=untouched
The trust writer's code words and the folder-trust step's rules: the step itself is unchanged, and only the step that follows it is new.
@task_item n=5 part=untouched
The PTY seam, the vt100 readiness gate and the screen signatures: nothing in screen handling moved.
@task_item n=6 part=untouched
CI workflows, coverage floors, fuzzing, mutation, release tooling and cargo-deny policy: no workflow, gate shape or outside dependency changed.
@task_item n=7 part=untouched
Link and unlink and the link set: no rule of the link set moved.
@task_item n=8 part=untouched
The absences of a daemon, a listening network port and a graphical frontend: open drives a terminal emulator and adds no listener or GUI.
