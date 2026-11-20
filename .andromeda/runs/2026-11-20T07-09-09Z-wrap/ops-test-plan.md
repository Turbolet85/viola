@row_add n=1 row=decision/fired-wake-keeps-wheel area=6-e2e-test-strategy fact=F1 detector=none
name: A fired scheduled wake keeps the driver's wheel
rule: A keyless human-filed prompt-submitted is relabelled origin harness with no wheel move when no stdin key set the typed mark, the last turn end counted at least one scheduled wake and the CLI is verified; otherwise it takes the human wheel.
guard: path5_a_fired_wake_keeps_the_driver_wheel_its_next_send_and_its_dialog
basis: fct-yqdgc3b5
because: The chunk adds the told-wake rule beside the harness-injected turn, so a CLI-scheduled wake no longer takes the wheel and test-plan must hold the rule its Path 5 cases test.
@row_add n=2 row=decision/typed-mark-lasts-until-taken area=6-e2e-test-strategy fact=F2 detector=none
name: Stdin typed mark lasts until a human-filed prompt takes it
rule: The stdin typed mark lasts until the next human-filed prompt takes it, surviving a turn end and a release, so a message queued during a turn reads human.
except: A prompt submitted by the mouse alone after a counted wake is the named false positive, unmeasured.
guard: tui_a_message_queued_during_a_turn_reads_human_after_that_turn_ends
basis: fct-wncy4gmx
because: The wheel rule as built clears the mark only at a human-filed prompt and names its one false positive, a decision the outer-PTY cases hold.
@row_add n=3 row=contract/fake-agent-cli/stop-crons area=7-test-data-fixtures fact=F3 detector=none
boundary: viola-fake-agent command line
item: --stop-crons
promise: The argv-only --stop-crons <n>, 0 to 4, puts n synthetic elements in every Stop's session_crons.
basis: fct-yqdgc3b5
because: The fake agent gains an argv option tests rely on to count scheduled wakes, and no row of its command line names it.
@row_add n=4 row=contract/fake-agent-cli/wake-after-ms area=7-test-data-fixtures fact=F3 detector=none
boundary: viola-fake-agent command line
item: --wake-after-ms
promise: The argv-only --wake-after-ms <ms>, capped at 5 000, fires once, that long after the first Stop carrying a non-empty session_crons, a keyless recorded UserPromptSubmit, and under --turn-stop its turn end.
basis: fct-yqdgc3b5
because: The fake agent gains an argv option that fires a scheduled wake for the wheel cases, and no row of its command line names it.
@row_add n=5 row=decision/rig-no-key-read-from-keys-word area=6-e2e-test-strategy fact=F5 detector=none
name: Rig rehearsal reads no key typed from its own keys word
rule: A live rig rehearsal reads no key typed from the rig's own keys word, never from the fake-agent receipt.
why: The shared receipt carries viola send's submit Enters as key lines.
basis: fct-6vnzowdq
because: The chunk settles where a rehearsal reads the no-key witness, a rule later rig rehearsals must follow.
@row_set n=1 row=decision/live-rig-hint-cell-reading cell=rule seen=1 fact=F4 detector=none
value: The live rig's hint-cell reading records the attribute words of every drawn hint cell past the cursor cell, after the second send or in a fresh box before any send, as a reading and never as verification; no drawn cell reads unread (no-cell).
basis: fct-o2xu5jdq
because: The HB arm reads the hint cells in a fresh unwrapped box before any send, so the row naming only the after-second-send reading is incomplete.
@row_set n=2 row=decision/fake-agent-replays-recorded-bytes cell=rule seen=2 detector=none
value: The fake agent offers each hook the recorded fixture bytes, building no payload, save the fields it sets: a typed prompt, the launch SessionStart's source and session_id under --resume, transcript_path under --transcript, the compact replay's session_id, Stop's session_crons under --stop-crons.
basis: fct-yqdgc3b5
because: Under --stop-crons the fake agent now sets synthetic elements in every Stop's session_crons, a field the row's closed list of set fields leaves out.
@row_set n=3 row=contract/fake-agent-hooks/stdin-payload cell=promise seen=2 detector=none
value: The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume, a compact replay, the launch SessionStart under --transcript or a Stop under --stop-crons sets.
basis: fct-yqdgc3b5
because: Under --stop-crons every Stop's session_crons is no longer the recorded bytes, so the promise's closed list of exceptions must name it.
@row_none n=1 fact=F6 type=8
what: The scope of one live measurement (one interval-form wake on claude 2.1.328 under a scratch home); the route and the sidecar hold it.
