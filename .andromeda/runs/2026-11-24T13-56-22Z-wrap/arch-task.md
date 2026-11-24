@task_prose part=session
The session added a way to switch a running wrapped session to a given model and effort: a new CLI verb viola switch and a new wrapper channel method switch.
On a switch the wrapper ends its own live child, reaps it within a new compiled bound RESPAWN_REAP_BOUND of 10 s, appends a launch line with the new pair, and respawns claude --resume of the same session id on the new pair, in a new pseudo-terminal pair under the same host terminal and the same wrapper pid.
A switch is refused, first refusal winning, while the human types, mid-turn, with a dialog pending, on an unverified CLI, with no session id, when the old child does not end within the bound, or when the resume fails; a send during a switch is refused turn-running.
Nothing is written to the user's own settings and no --settings flag is passed; a later revive replays the switched pair from the record.
The PTY pump now owns a respawnable child; the host-input copy and the write queue persist across a respawn. Forks F1, F3, F5 and F6 stand PROVISIONAL on a delegate overseer's word; proven over the fake agent alone, no capability claimed.
@task_file n=1 path=src/cmd/switch.rs
what: the new viola switch CLI verb: its arguments, its client checks before the frame, its refusal and human output, its exit codes
@task_file n=2 path=src/run/switch.rs
what: the wrapper's switch channel method: param checks of the model and effort halves, the refusal order, the reap and respawn of the child, the launch append
@task_file n=3 path=tests/cli_switch.rs
what: the integration tests of viola switch over the fake agent
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: the PTY pump of viola's pty seam: copies between the host terminal and the child, now with a respawn handle that swaps the child under a bounded reap
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: the viola-pty crate's public surface, now exporting the respawn handle, its outcomes and the reap bound
@task_file n=6 path=crates/viola-channel/src/server.rs
what: the wrapper channel server and its closed list of method labels
@task_file n=7 path=src/cmd/run.rs
what: the wrapper's run path: the child pump and the channel method dispatch
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: viola verify's typed run that spawns the probed child through the pump
@task_file n=9 path=src/cmd/mod.rs
what: the CLI's subcommand table and the viola --help verb groups
@task_file n=10 path=src/run/mod.rs
what: the wrapper's run module list
@task_file n=11 path=src/run/send.rs
what: the wrapper's send method and its refusal checks
@task_file n=12 path=src/run/send/slot.rs
what: the send flight slot under the wrapper flight lock, now carrying the switch mark and the late session-end screen
@task_file n=13 path=src/human.rs
what: the one human writer of the root bin
@task_file n=14 path=src/human/verbs.rs
what: the human result lines of the CLI verbs
@task_file n=15 path=src/human/hints.rs
what: the human hint lines for refusal causes
@task_file n=16 path=tests/cli_output_plain.rs
what: the tests of plain human CLI output and the served verbs in help
@task_file n=17 path=tests/contract_error_bodies.rs
what: the census of CLI error documents and of the channel's served methods
@task_file n=18 path=tests/cross_exit_causes.rs
what: the cross-check table of refusal causes to exit codes
@task_file n=19 path=tests/support/home.rs
what: the root test support for test homes and wrappers, adopt among them
@task_item n=1 part=touches
contract/channel-methods: a new served wrapper channel method switch with params v, sender, model and effort and an ok result of the pair; no switch row exists yet.
@task_item n=2 part=touches
contract/cli-verbs: a new CLI verb switch with --model, --effort and --json, at least one half required, exits 0, 10, 12, 13, 14, 20, 21; no switch row exists yet.
@task_item n=3 part=touches
contract/wrapper-event-appends/launch and contract/event-data/launch: a launch line is now appended at every switch as well as at every start, after the reap and before the respawn, with the window line again after it when the snapshot holds one.
@task_item n=4 part=touches
contract/instance-snapshot/launch and contract/instance-snapshot/child-pid: a switch rewrites the recorded launch pair and the child pid.
@task_item n=5 part=touches
decision/revive-pair-from-record, decision/recorded-pair-on-child-argv and decision/launch-pair-handed-not-verified: a revive with no pair now replays the switched pair; the switched pair is handed on the child's argv and never verified.
@task_item n=6 part=touches
contract/child-argv/resume, contract/claude-child-argv/resume and contract/child-argv/launch-pair: the respawned child carries --resume of the recorded session id with --model and --effort of the new pair.
@task_item n=7 part=touches
decision/revive-in-place, decision/revive-resumes-in-place and decision/run-start-order: a second in-place resume path now exists inside a live wrapper, not through the start path; the child is spawned again after the wrapper is up.
@task_item n=8 part=touches
decision/portable-pty-seam, decision/portable-pty-behind-pty-seam and decision/pty-knows-no-agent: the pty seam now owns a respawnable child; a respawn opens a new pseudo-terminal pair under the same host terminal, the old pair closed, and the seam still knows no agent.
@task_item n=9 part=touches
decision/child-exit-on-process-handle and contract/child-exit-record/exit-source: the old child is ended by SIGHUP through its own PTY handle and reaped within RESPAWN_REAP_BOUND, kill-fallback where it does not exit; only the child its own handle holds is ended, no pid lookup.
@task_item n=10 part=touches
decision/gate-timings-compiled, decision/confirm-window-compiled and decision/send-time-bound: a new compiled time bound, RESPAWN_REAP_BOUND at 10 s (F6, PROVISIONAL), distinct from the send time bound.
@task_item n=11 part=touches
decision/send-never-queued, decision/send-flight-lock, contract/send-refusal/order and contract/refusal-order/send: a send is refused turn-running from a switch's request until the new child's session-start, through a switch mark on the flight.
@task_item n=12 part=touches
contract/refusal-order/first-only and decision/first-refusal-only: switch has its own refusal order, human-typing, then turn-running, then dialog-pending, then unverified-cli, then no-session, the first winning, none appending or ending the child.
@task_item n=13 part=touches
decision/refusal-details-closed-set, contract/refusal-reason/not-delivered and contract/not-delivered-details: four new not-delivered detail words, dialog-pending, no-session, child-not-ended and resume-failed.
@task_item n=14 part=touches
contract/refusal-reason/human-typing and contract/refusal-reason/unverified-cli: both now also refuse a switch; unverified-cli reads the start's cli_verified.
@task_item n=15 part=touches
contract/channel-fault-codes/-32602 and contract/channel-32602-cause/invalid-params: a switch with neither half, a half of another type or a half failing its check is -32602 with data null.
@task_item n=16 part=touches
decision/human-controls-cli-only, decision/pause-link-unlink-cli-only and decision/release-cli-only: switch joins the CLI-only verbs, no MCP tool (F5, PROVISIONAL).
@task_item n=17 part=touches
decision/global-statusline-untouched and absence/status-line-interception: a switch writes nothing to the user's config or settings and passes no --settings, both byte- and mtime-equal.
@task_item n=18 part=touches
decision/frame-client-check-order, decision/client-strict-modes-before-frame and decision/verify-server-before-first-frame: the switch verb runs the untrusted-home refusal, then the live endpoint read, then the verified connect before its frame.
@task_item n=19 part=touches
contract/viola-cli-arguments/run-model and contract/viola-cli-arguments/run-effort: switch's --model and --effort go through the same model and effort parsers, and the wrapper re-checks each half.
@task_item n=20 part=touches
contract/diagnostics-role-files/cli-name and decision/codes-only-process-logs: the switch verb logs as cli into cli-<name>.ndjson with a cli.switch dispatch span; no pair, model or session id in any line.
@task_item n=21 part=touches
contract/frame-verb-exit-codes/21 and contract/cli-exit-codes: switch is a new frame verb exiting 21 on an unreachable instance and 13 on its not-delivered refusals.
@task_item n=22 part=touches
contract/hook-tiers/session-end, contract/event-data/session-end and contract/normalised-event-kinds/session-end: after a respawn the old child's late session-end of the switched id is screened once while the new child's pid is alive; the screen cannot tell it from the new child's own.
@task_item n=23 part=touches
decision/human-always-wins-wheel, decision/one-pending-dialog and decision/dialog-hold: a switch is refused while the human types and while a dialog is pending, read from the wrapper's own snapshot.
@task_item n=24 part=touches
decision/run-terminal-child-only: viola writes no byte to the terminal at a respawn; the new child spawns at the host's current size.
@task_item n=25 part=touches
name/command/*: the command name switch is now occupied.
@task_item n=26 part=touches
decision/new-capabilities-as-verbs and decision/wrap-unmodified-claude: the capability lands as a CLI subcommand and a channel verb over the unmodified claude CLI.
@task_item n=27 part=touches
contract/list-item/model and contract/list-item/effort: the board's model and effort of a wrapped instance follow the switched launch pair.
@task_fact n=1 slug=established-decisions
fact: A running wrapped session switches its model and effort in place: the wrapper ends only the child its own PTY handle holds, reaps it within RESPAWN_REAP_BOUND, appends launch, and resumes the same session id on the new pair under the same wrapper; PROVISIONAL, fork F1.
@task_fact n=2 slug=established-decisions
fact: A switch is refused mid-turn, with a dialog pending and while the human types, never queued; PROVISIONAL, fork F3.
@task_fact n=3 slug=established-decisions
fact: Switch is CLI-only, with no MCP tool; PROVISIONAL, fork F5.
@task_fact n=4 slug=established-decisions
fact: RESPAWN_REAP_BOUND is a compiled 10 s bound; a child not exited within it is not replaced and the switch is refused not-delivered child-not-ended; PROVISIONAL, fork F6.
@task_fact n=5 slug=established-decisions
fact: A send is refused turn-running while a switch stands, from the switch's request until the new child's session-start.
@task_fact n=6 slug=established-decisions
fact: After a respawn the old child's late session-end of the switched id is screened once while the new child's pid is alive; whether the real CLI fires SessionEnd on SIGHUP stays unproven, carried to the real-CLI reading R6a.
@task_fact n=7 slug=established-decisions
fact: A switch never writes to the user's own config or settings and passes no --settings; the switched pair lives only in viola's own record, which a later revive replays.
@task_fact n=8 slug=established-decisions
fact: A respawn opens a new pseudo-terminal pair under the same host terminal; the host-input copy and the write queue persist across it, the writer and output copies restart on the new child, and the panic arm kills the current child.
@task_fact n=9 slug=standard-contracts
fact: Channel method switch takes v, sender and at least one of model and effort, reads no from, and answers ok with the resulting model and effort, or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending, no-session, child-not-ended or resume-failed; a bad or missing half is -32602 with data null.
@task_fact n=10 slug=conventions
fact: CLI verb viola switch <name> takes --model, --effort (at least one, else usage exit 2) and --json, prints <name> switched <model> <effort> on stdout, and exits 0, 10, 12, 13, 14, 20 or 21.
@task_fact n=11 slug=conventions
fact: The not-delivered refusal details gain dialog-pending, no-session, child-not-ended and resume-failed, each exit 13, emitted on no diagnostics line.
@task_fact n=12 slug=occupied-resources
fact: The command name switch is occupied by viola switch.
@task_item n=1 part=untouched
The MCP tool contracts and MCP results: no MCP tool was added or changed.
@task_item n=2 part=untouched
The hook to kind map, hook exit and body contracts and the dialog answer path: no hook payload or dialog-slot behaviour changed.
@task_item n=3 part=untouched
The capability ledger rows, ledger stamps and the probe ledger rows: no capability claimed and no ledger row added.
@task_item n=4 part=untouched
config.json keys and configuration precedence: no config key added.
@task_item n=5 part=untouched
The diagnostics line schema and the process-exit detail enum: unchanged, no line emits the new detail words.
@task_item n=6 part=untouched
Build system, CI, dependency policy, crate dependency direction and toolchain rows: no crate or dependency added or removed.
@task_item n=7 part=untouched
The Hyprland seam, window placement and focus rows: untouched.
@task_item n=8 part=untouched
Trust writer code words, pinned copy and plugin install rows: untouched.
@task_item n=9 part=untouched
Links, the wheel event and release rows: the switch moves no wheel and records no link.
