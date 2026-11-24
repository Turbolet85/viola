@row_set n=1 row=contract/claude-child-process-log/handle-wait-exit cell=promise seen=1 fact=F3 detector=D-obs-instrumentation
value: process-start is logged once each child is spawned, a child a switch respawns included, and process-exit once that child is reaped or the pump returns; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
basis: fct-xvxcbpio, fct-pctfyjeu
because: The pump now owns a respawned child, so a switch writes the old child's process-exit and the new child's process-start before the pump returns, and the row's one-child timing no longer holds.
@row_set n=2 row=contract/run-process-log/process-exit-self cell=promise seen=1 detector=D-obs-instrumentation
value: After the process-exit of the child the pump ends on, run emits process-exit{subject:"self", exit_code, duration_ms}; the process-exit of a child a switch replaces is followed by none.
basis: fct-xvxcbpio
because: A switch ends a child without ending run, so run's own process-exit follows only the last child's exit, not every child's.
@row_set n=3 row=decision/launch-line-untraced cell=rule seen=2 fact=F5 detector=D-obs-instrumentation
value: The launch line written at every run, revive and adopt start and at every switch is log-only product audit data: it wakes no wait and opens no span, writes no process-log line and no terminal byte of its own.
basis: fct-vzga25j6
because: A switch now appends one launch line with no trace of its own, so the row naming only the starts is incomplete.
@row_set n=4 row=decision/window-line-untraced cell=rule seen=1 fact=F5 detector=D-obs-instrumentation
value: The window line a start under --origin writes after its launch line, and a switch writes again after its own launch line, is log-only product audit data: it wakes no wait, opens no span and is never copied into a process-log line.
basis: fct-vzga25j6
because: A switch writes the recorded window line again after its launch line where the snapshot holds one, so the row naming only the start is incomplete.
@row_set n=5 row=contract/send-refused-detail/turn-running cell=promise seen=2 fact=F8 detector=D-obs-instrumentation
value: Covers a running turn the wrapper knows of before the paste, another send in flight, or a switch standing from its request until the new child's session-start, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
basis: fct-xvxcbpio, fct-ionzjh33
because: A send is now refused turn-running while a switch stands (provisional on fork F3), a third cause the row's list of causes lacks.
@row_set n=6 row=decision/instance-check-before-log-init cell=rule seen=4 detector=none
value: Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus, switch) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found.
basis: fct-pmolmu2p
because: viola switch runs the instance strict-modes check first, before its endpoint read and connect, so the list of commands holding the rule is incomplete.
@row_set n=7 row=contract/cli-json-error/strict-modes-failed cell=promise seen=1 detector=none
value: A refused instance check of send, wait, last, answer, pause, release, link, unlink or switch exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
basis: fct-pmolmu2p, fct-xvxcbpio
because: viola switch runs the same instance check before any frame and exits 21 among its codes, so the verb list is incomplete.
@row_add n=1 row=contract/cli-log-file/switch area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
boundary: CLI process-log file
item: viola switch
promise: Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a process-exit inside the cli.switch dispatch span; a refusal ends process-exit with no detail, as send's does.
basis: fct-xvxcbpio, fct-pctfyjeu, fct-np74v5cz
because: The new viola switch verb logs into its instance's cli file under its own dispatch span, a per-verb log-file contract every other verb has a row for.
@row_add n=2 row=decision/switch-method-lines-carry-no-pair area=span-trace-coverage fact=F2 detector=D-obs-pii
name: Switch method lines carry no pair or session id
rule: The switch channel method writes only its channel-request and channel-response pair, labelled switch, and no line of it carries the model, the effort or the session id.
state: provisional
basis: fct-pctfyjeu, fct-hgjb3ikv
because: The new wrapper switch method (provisional on fork F1) joins the closed method-label list and its lines are held free of the pair and the session id, a scrub rule future work on the method must keep.
@row_none n=1 fact=F4 type=4
relied: The four switch refusal words are channel refusal details only; no diag line emits them, so the diag-line schema is unchanged and the standing schema-word rule already covers it.
@row_none n=2 fact=F6 type=0
what: The absence of an exit-1 detail of viola switch's own; its exit 1 is the cli catch site's internal-error, which the standing exit-1 detail row already lists.
@row_none n=3 fact=F7 type=4
relied: After a respawn the wrapper screens one late session-end of the switched id while the new child lives; whether a real CLI fires it on SIGHUP is the route's open reading R6a.
