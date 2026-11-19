@row_add n=1 row=decision/live-rig-stopfailure-capture-not-telemetry area=span-trace-coverage fact=F1 detector=none
name: Live-rig StopFailure capture is not telemetry
rule: The live rig's StopFailure capture is a test-only record, never telemetry: it keeps the payload's key names, the error word only from the CLI's closed thirteen (any other value as other) and the byte lengths of error_details and last_assistant_message, never a text.
why: Ratified as the one boundary widening of the live StopFailure reading, held to exactly that width.
basis: fct-usi3wfxj
because: The chunk adds the rig's StopFailure capture line and the founder's answer fixes its width, so obs-plan must state that the record is no telemetry and carries no text.
