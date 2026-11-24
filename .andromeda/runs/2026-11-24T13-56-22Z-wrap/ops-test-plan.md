@row_set n=1 row=contract/viola-send/turn-running cell=promise seen=2 fact=F2 detector=none
value: While a turn runs, or from a switch's request to the new child's session-start, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended, an API-error turn end included, a send is accepted with no pause or release.
basis: fct-xvxcbpio, fct-ionzjh33
because: A send is now also refused turn-running while a switch stands, so the row's "while a turn runs" no longer names every case of the refusal.
@row_set n=2 row=contract/json-rpc-errors/-32602 cell=promise seen=1 detector=none
value: Invalid params is reported as -32602, its cause telling the three invalid-params faults apart; data holds supported and wrapper on a too-new v, and is null on a switch's invalid params.
basis: fct-xvxcbpio
because: The new switch method answers its invalid params -32602 with data null, so data no longer always holds supported and wrapper.
@row_set n=3 row=decision/frame-verbs-check-instance-first cell=rule seen=1 detector=D-tests-count-literals
value: All nine CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
basis: fct-pmolmu2p, fct-xvxcbpio
because: viola switch is a new frame verb that runs the strict-modes check, then the live endpoint, then the connect, so the frame-verb count moves from eight to nine.
@row_set n=4 row=contract/exit-21-detail/socket-dir-refused cell=promise seen=1 detector=D-tests-count-literals
value: The socket directory is refused, on the nine frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
basis: fct-pmolmu2p, fct-xvxcbpio
because: viola switch connects through the same client connect as the other frame verbs, so the frame-verb count moves from eight to nine.
@row_retire n=1 row=decision/unverified-cli-gates-answer-only seen=1 heir=decision/unverified-cli-gates-answer-and-switch detector=none
why: Under an unverified CLI version switch is refused unverified-cli as well, so answer is no longer the only verb the version gates.
basis: fct-xvxcbpio, fct-kwsxyu52
because: The switch method's refusals include unverified-cli, read from the start's cli_verified, which makes the row's "answer only" false.
@row_add n=1 row=decision/unverified-cli-gates-answer-and-switch area=e2e-test-strategy detector=none
name: Unverified CLI version gates answer and switch
rule: Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer and switch are refused unverified-cli.
basis: fct-xvxcbpio, fct-kwsxyu52
because: The heir of the retired answer-only row, naming switch beside answer as refused unverified-cli.
@row_add n=2 row=decision/switch-refusal-order area=unit-test-strategy fact=F1 detector=none
name: switch refusal order
rule: switch refuses in the order human-typing, turn-running, dialog-pending, unverified-cli, then no-session; the first rung wins, and no refusal appends a line or ends the child.
guard: switch_refusal_order_cases
state: provisional
basis: fct-xvxcbpio, fct-bs3a4rfe, fct-5xyy4zny
because: The chunk adds the switch method with a fixed refusal order, built ahead of the answer to fork F3, which future verbs touching the wrapper must respect.
@row_add n=3 row=decision/respawn-reap-bound area=unit-test-strategy fact=F4 detector=D-leaf-figures
name: Respawn reap bound
rule: A child not exited within RESPAWN_REAP_BOUND is not replaced, and the switch is refused not-delivered with detail child-not-ended.
guard: respawn_of_a_child_ignoring_the_signal_reads_not_ended
state: provisional
basis: fct-xvxcbpio, fct-bs3a4rfe, fct-5xyy4zny
because: The chunk adds a compiled respawn reap bound, built ahead of the answer to fork F6, named here by its constant rather than its figure.
@row_add n=4 row=contract/switch-method/params area=integration-test-strategy fact=F6 detector=none
boundary: the wrapper channel switch method
item: params
promise: switch takes at least one of model and effort, re-checks each half in the wrapper and reads no from; neither half, a half of another type or a half failing its check is answered -32602 with data null.
basis: fct-xvxcbpio
because: The chunk adds the switch channel method, whose params are a promise at the wrapper channel boundary.
@row_add n=5 row=contract/cli-human-lines/switched area=integration-test-strategy fact=F11 detector=none
boundary: CLI human lines
item: switched
promise: A switch prints the single plain line <name>  switched  <model> <effort> on stdout.
basis: fct-xvxcbpio, fct-pctfyjeu
because: The chunk adds the switched human line of viola switch, a fixed output pinned by literal.
@row_add n=6 row=contract/viola-switch/exit-codes area=e2e-test-strategy fact=F5 detector=none
boundary: viola switch
item: exit codes
promise: viola switch exits 0, 10 human-typing, 12 unverified-cli, 13 not-delivered (turn-running, dialog-pending, no-session, child-not-ended, resume-failed), 14, 20 and 21; neither --model nor --effort, or a bad value, is a usage error exiting 2.
basis: fct-xvxcbpio, fct-vocxyji6
because: The chunk adds the viola switch verb, whose exit codes are a promise at the CLI boundary.
@row_add n=7 row=contract/event-log/switch-launch-line area=e2e-test-strategy fact=F7 detector=none
boundary: instance event log
item: launch line of a switch
promise: A switch appends one launch line with the new pair after the old child is reaped and before the respawn; a later revive with no pair replays the switched pair.
basis: fct-vzga25j6, fct-bs3a4rfe
because: The events log gains one launch line per switch, which revive reads to replay the pair.
@row_add n=8 row=contract/viola-switch/settings area=e2e-test-strategy fact=F8 detector=none
boundary: viola switch
item: settings
promise: A switch passes the child no --settings and leaves the user's config and settings byte- and mtime-equal.
basis: fct-ylurzsjb, fct-bs3a4rfe
because: The chunk promises a switch writes nothing to the user's own settings, a promise outward like the start's.
@row_add n=9 row=decision/switch-ends-only-its-own-child area=e2e-test-strategy fact=F9 detector=none
name: Switch ends only its own child
rule: A switch ends only the child its own PTY handle holds, with no pid lookup, and resumes the same session id on a new pseudo-terminal pair under the same host terminal, the wrapper pid unchanged.
guard: switch_of_an_idle_session_resumes_it_on_the_new_pair
state: provisional
basis: fct-xvxcbpio, fct-przrtlcy, fct-bs3a4rfe, fct-5xyy4zny
because: The chunk has the wrapper end and replace its own child, built ahead of the answer to fork F1, which bounds what any later respawn may kill.
@row_add n=10 row=decision/switch-proven-live area=e2e-test-strategy fact=F10 detector=none
name: Real switch proven only by live readings R6a and R6b
rule: A real switch, an idle one read on the next turn with the user's settings byte-equal and whether SessionEnd fires on SIGHUP, and one refused mid-turn, is proven only by the gated live readings R6a and R6b; a switch over the fake agent claims no capability.
basis: fct-5xyy4zny, fct-vsonkq72
because: Every switch reading of the chunk is over the fake agent, so the real behaviour rests on the carried live readings alone.
@row_add n=11 row=decision/switch-cli-only area=test-scope-summary fact=F12 detector=none
name: switch is CLI-only
rule: switch is a CLI-only control verb with no MCP tool.
state: provisional
basis: fct-5xyy4zny, fct-vkgo67d5
because: The chunk adds switch to the CLI alone, built ahead of the answer to fork F5, so MCP tests must not expect a switch tool.
@row_none n=1 fact=F3 type=4
relied: A send's flight after a respawn is not ended by the first late session-end of the switched id while the new child's pid is alive; the screen's known limit is the route's, owed to the live reading R6a.
