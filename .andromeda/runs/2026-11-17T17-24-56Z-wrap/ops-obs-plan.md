@row_add n=1 row=decision/live-rig-prompt-capture-not-telemetry area=obs-scope-summary fact=F1 detector=D-obs-pii
name: Live rig prompt capture is no telemetry
rule: The standing live rig's own UserPromptSubmit capture is a test-only witness, not telemetry: one line per payload with key names matching a closed pattern, a closed source word (schedule_wakeup, loop_wakeup, other, absent) and a millisecond stamp, never a value.
why: Ratified as a widening of the live rig; no payload value is kept.
guard: rig::capture unit tests
basis: fct-wy25afcs, fct-pmfixyq6, fct-rfaae7sn
because: The chunk adds a test-only capture of hook payloads whose closed, value-free shape no obs-plan row states, and the scope must say it is a witness rather than a telemetry source.
