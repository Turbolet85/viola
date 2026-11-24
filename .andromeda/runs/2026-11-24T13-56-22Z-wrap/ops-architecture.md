@row_set n=1 row=decision/send-never-queued cell=rule seen=1 fact=F5 detector=D-arch-decisions
value: A `send` while a turn runs, while another `send` is in flight, or while a switch stands, from its request until the new child's `session-start`, is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
basis: fct-ionzjh33
because: A standing switch now also refuses a send turn-running, so the rule naming when a send is refused must name it.
@row_set n=2 row=decision/launch-pair-handed-not-verified cell=rule seen=2 detector=D-arch-decisions
value: Every start, run, revive and adopt, and every switch, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on.
basis: fct-bs3a4rfe
because: A switch now hands the respawned child a new pair on its command line, still unverified on the real CLI, so the rule's list of who hands a pair must include it.
@row_set n=3 row=contract/hook-event/appended-line cell=promise seen=1 fact=F6 detector=D-arch-decisions
value: A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}; after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive.
basis: fct-xvxcbpio
because: The wrapper now screens one late session-end after a respawn, so not every valid hook event is appended.
@row_set n=4 row=contract/refusal-reason/not-delivered cell=promise seen=2 fact=F11 detector=D-arch-resources
value: Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut, dialog-pending, no-session, child-not-ended, resume-failed.
basis: fct-xvxcbpio
because: The switch method's refusals add four not-delivered details, so the closed detail list must carry them.
@row_set n=5 row=contract/wrapper-event-appends/launch cell=promise seen=1 detector=D-arch-resources
value: The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`, and once at every switch, after the old child is reaped and before the respawn; it is log-only and never wakes `wait`.
basis: fct-vzga25j6
because: Each switch now appends one launch line, so the promise of once at every start is incomplete.
@row_set n=6 row=contract/wrapper-event-appends/window cell=promise seen=1 detector=D-arch-resources
value: The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin, and again after a switch's `launch` line when the snapshot holds a window; it is log-only and never wakes `wait`.
basis: fct-vzga25j6
because: A switch re-appends the recorded window line after its launch line, so the promise of one window line per start is incomplete.
@row_set n=7 row=contract/instance-snapshot/launch cell=promise seen=1 detector=D-arch-resources
value: `launch` is `{model, effort}`, the pair the start or the latest switch handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.
basis: fct-vzga25j6
because: A switch now rewrites the snapshot's launch pair, so it is no longer only the pair the start handed.
@row_set n=8 row=contract/instance-snapshot/child-pid cell=promise seen=1 detector=D-arch-resources
value: `child_pid` is the pid of the current agent child, rewritten when a switch respawns it, and exists only to join `claude agents --json` rows to this instance.
basis: fct-vzga25j6
because: A switch rewrites child_pid with the respawned child's pid, so the field names the current child, not only the one spawned at start.
@row_set n=9 row=contract/child-argv/resume cell=promise seen=2 detector=D-arch-resources
value: A revived or adopted start, and a switch's respawn, launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
basis: fct-bs3a4rfe
because: A switch's respawned child now carries --resume of the same session id, so the list of launches adding --resume is incomplete.
@row_set n=10 row=contract/claude-child-argv/resume cell=promise seen=2 detector=D-arch-resources
value: A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it; an adopted start adds --resume with the operator-typed id, in the session-id shape; a switch's respawn adds --resume with the switched id.
basis: fct-bs3a4rfe
because: A switch's respawned child now carries --resume of the same session id, so the promise naming who adds --resume is incomplete.
@row_add n=1 row=decision/switch-in-place area=established-decisions fact=F1 detector=D-arch-decisions
name: Model and effort switched in place
rule: A running wrapped session switches its model and effort in place: the wrapper ends only the child its own PTY handle holds, reaps it within RESPAWN_REAP_BOUND, appends `launch`, and resumes the same session id on the new pair under the same wrapper.
state: provisional
basis: fct-xvxcbpio
because: The chunk lands a new way to change a live session's pair, built ahead of ratification, which future work must respect.
@row_add n=2 row=decision/switch-refused-never-queued area=established-decisions fact=F2 detector=D-arch-decisions
name: A switch is refused, never queued
rule: A switch is refused while a turn runs, while a dialog is pending and while the human types, never queued.
state: provisional
basis: fct-5xyy4zny
because: The chunk fixes when a switch may not run, built ahead of ratification.
@row_add n=3 row=decision/switch-cli-only area=established-decisions fact=F3 detector=D-arch-decisions
name: Switch is CLI-only
rule: viola switch is offered on the CLI only: no MCP tool switches a session's model or effort.
state: provisional
basis: fct-vkgo67d5
because: The chunk keeps the switch off the driver's MCP surface, a boundary choice built ahead of ratification.
@row_add n=4 row=decision/respawn-reap-bound area=established-decisions fact=F4 detector=D-leaf-figures
name: Respawn reap bound compiled
rule: The respawn reap bound RESPAWN_REAP_BOUND is compiled; a child not exited within it is not replaced, and the switch is refused `not-delivered` with detail `child-not-ended`.
state: provisional
basis: fct-xvxcbpio
because: The chunk adds a new compiled time bound on ending the child, named here by its constant, built ahead of ratification.
@row_add n=5 row=decision/switch-writes-no-user-settings area=established-decisions fact=F7 detector=D-arch-decisions
name: Switch writes nothing into the user's settings
rule: A switch never writes to the user's own config or settings and passes no --settings; the switched pair lives only in viola's own record, which a later revive replays.
basis: fct-bs3a4rfe
because: The chunk keeps the switched pair out of the user's files, a boundary future work must keep.
@row_add n=6 row=contract/channel-methods/switch area=standard-contracts fact=F9 detector=D-arch-resources
boundary: channel methods
item: switch
promise: A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending, no-session, child-not-ended or resume-failed; a bad or missing half is -32602, data null.
basis: fct-xvxcbpio
because: The chunk lands a new wrapper channel method, which must be registered among the channel's methods.
@row_add n=7 row=contract/cli-verbs/switch area=conventions fact=F10 detector=D-arch-resources
boundary: CLI verbs
item: switch
promise: `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort`, else a usage error, exit 2; prints `<name>  switched  <model> <effort>` on stdout and exits 0, 10, 12, 13, 14, 20 or 21.
basis: fct-xvxcbpio
because: The chunk lands a new CLI verb, which must stand among the CLI verbs.
@row_add n=8 row=name/command/switch area=occupied-resources fact=F12 detector=D-arch-resources
class: command
name: switch
basis: fct-xvxcbpio
because: The command name switch is now occupied by viola switch.
@row_none n=1 fact=F8 type=4
relied: The host-input copy and the write queue persist across a respawn, and the panic arm kills the current child.
