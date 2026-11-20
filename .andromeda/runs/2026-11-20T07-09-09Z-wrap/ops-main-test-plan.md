@row_set n=1 row=decision/fake-agent-replays-recorded-bytes cell=rule seen=2 detector=none
value: The fake agent gives each hook the recorded fixture bytes, no payload built, save what it sets: a default UserPromptSubmit's typed prompt, the launch SessionStart's source and session_id (--resume) and transcript_path (--transcript), compact replay's session_id, Stop's session_crons (--stop-crons).
basis: fct-yqdgc3b5
because: Under --stop-crons the fake agent now sets every Stop's session_crons, a field the closed list leaves out; every standing member is kept, the cell shortened to its bound.
@row_add n=1 row=decision/typed-mark-lasts-until-taken area=6-e2e-test-strategy detector=none
name: Stdin typed mark lasts until a human-filed prompt takes it
rule: The stdin typed mark a human key sets lasts until the next human-filed prompt takes it, never cleared at a turn end or a release, so a message queued during a turn reads human and a draft typed then erased makes the next counted wake take the wheel once.
except: A prompt submitted by the mouse alone after a counted wake is the one named false positive, unmeasured.
guard: tui_a_message_queued_during_a_turn_reads_human_after_that_turn_ends
basis: fct-wncy4gmx
because: The wheel rule as built clears the mark only at a human-filed prompt, so a queued message reads human, an erased draft costs one wake its keep, and the mouse-only submit is its one named false positive.
