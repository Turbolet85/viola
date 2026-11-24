@row_add n=1 row=decision/switch-ends-only-its-own-child area=authentication-authorization fact=F1 detector=none
name: Switch ends only the wrapper's own child
rule: A switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place on the new pair.
state: provisional
basis: fct-pmolmu2p; fct-vkgo67d5
because: The chunk adds a wrapper that ends its own live child and resumes in place (F1, on a delegate's word), a process-ending boundary no row states.
@row_add n=2 row=contract/wrapper-channel-methods/switch area=threat-model-summary fact=F2 detector=D-security-input
boundary: wrapper channel methods
item: switch
promise: Switches the driven session to a given model and effort: params {v, sender, model?, effort?}; replies ok with model and effort, or a refusal: human-typing, not-delivered turn-running, dialog-pending, no-session, child-not-ended or resume-failed, or unverified-cli; from is not read.
basis: fct-xvxcbpio
because: The wrapper channel serves a new switch method and the closed list of wrapper channel methods holds one row a method.
@row_add n=3 row=contract/channel-frames/switch-params area=input-validation fact=F3 detector=D-security-input
boundary: channel frames
item: switch params
promise: model and effort are each absent or a string passing its check, at least one given; neither half, a half of another type or one failing its check is -32602 with data null.
basis: fct-xvxcbpio
because: The new switch method's params are re-checked in the wrapper and no channel-frames row states their validation.
@row_add n=4 row=decision/switch-refusal-order area=authentication-authorization fact=F4 detector=none
name: Switch refusal order
rule: A switch is refused in a fixed order, the first rung winning: human typing, a running turn, a pending dialog, an unverified CLI, then no session; no refusal appends a line or ends the child.
state: provisional
basis: fct-bs3a4rfe; fct-sqt5v7r2; fct-vkgo67d5
because: The chunk fixes when a switch may end the child, held provisional on F3, and no row states it.
@row_set n=1 row=decision/launch-pair-kept-to-snapshot-and-event-log cell=rule seen=1 fact=F5 detector=none
value: An instance's launch pair is kept only in the snapshot's launch field and the events log's launch lines, one at start and one per switch, never in the user's Claude settings; log-only with no span, process-log line or terminal byte, it leaves only through list --json and the MCP list item.
basis: fct-vzga25j6; fct-bs3a4rfe
because: A switch appends one launch line per switch and writes nothing to the user's settings, so the row's single launch line no longer holds.
@row_add n=5 row=decision/switch-cli-only area=threat-model-summary fact=F6 detector=none
name: Switch is CLI-only
rule: Switching a session's model and effort is CLI-only; no MCP tool switches a session.
state: provisional
basis: fct-vkgo67d5; fct-5xyy4zny
because: The chunk keeps the switch off the MCP tool list on F5, a decision no row states.
@row_add n=6 row=decision/respawn-reap-bound area=new fact=F7 detector=none
section: ## Security Decisions Log
name: Respawn reap bound
rule: A switch waits a compiled 10 s bound for the old child's exit; a child not ended within it is not replaced and the switch is refused child-not-ended.
state: provisional
basis: fct-xvxcbpio; fct-vocxyji6
because: The chunk adds RESPAWN_REAP_BOUND, a new compiled bound held provisional on F6, and no area of security-plan holds it.
@row_add n=7 row=contract/send-refusal/turn-running area=new fact=F8 detector=none
section: ## Threat Model Summary
boundary: send refusal
item: turn-running
promise: A send is refused not-delivered turn-running while a turn runs and while a switch stands, from the switch's request until the new child's session-start.
basis: fct-ionzjh33; fct-xvxcbpio
because: A standing switch now refuses a send turn-running, a refusal no send-refusal row of security-plan states.
@row_add n=8 row=decision/switch-method-behind-ipc-controls area=threat-model-summary detector=D-security-input
name: Switch method mapped to the IPC controls
rule: The wrapper's switch method, which ends and resumes the driven child, is served only behind the IPC controls (socket-directory, peer euid and server checks), re-checks both halves in the wrapper and reads no from.
why: A new channel method needs its own entry mapping it to the IPC controls.
state: provisional
basis: fct-pmolmu2p; fct-xvxcbpio
because: The chunk adds a channel method, and the standing rule requires each new method to be mapped to the IPC controls.
@row_set n=2 row=decision/launch-flags-checked cell=rule seen=2 detector=D-security-input
value: viola run's, revive's, adopt's and switch's --model must be a model word, 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes, and --effort one of the five effort words of claude --help; a bad value is a usage error, exit 2, before anything is read.
basis: fct-pmolmu2p; fct-xvxcbpio
because: viola switch takes --model and --effort through the same checks, so the row's list of verbs is incomplete.
@row_set n=3 row=decision/cli-frame-verbs-strict-modes-first cell=rule seen=2 detector=none
value: CLI send, wait, last, answer, pause, release, link, unlink, focus and switch run the instance strict-modes check before any frame or compositor read; a refusal exits 21, opens nothing under the home and writes no line.
basis: fct-pmolmu2p; fct-vyyrsacb
because: viola switch is a new frame verb that runs the instance strict-modes check first, so the row's verb list is incomplete.
@row_set n=4 row=contract/cli-exit-codes/12 cell=promise seen=2 detector=none
value: answer or switch refused unverified-cli: no verify stamp for the child's CLI version on which every relied ledger row counts (pass, or absent on the browser-tools-prompt row alone).
basis: fct-xvxcbpio; fct-vocxyji6; fct-kwsxyu52
because: viola switch now exits 12 on unverified-cli, so a promise naming answer alone is incomplete.
