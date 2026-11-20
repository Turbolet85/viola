@row_set n=1 row=decision/human-prompt-confirms-wheel-take cell=except seen=1 fact=F1 detector=D-arch-decisions
value: On a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end is a fired wake: relabelled harness, wheel kept.
basis: fct-yqdgc3b5
because: The wrapper now relabels a CLI-fired scheduled wake `harness` with no wheel move under the told-wake rule, so a human-filed unsent prompt no longer always takes the wheel.
@row_set n=2 row=decision/byte-source-wheel-claim cell=rule seen=1 fact=F2 detector=D-arch-decisions
value: Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`; the key's typed mark clears only when a human-filed prompt takes it, never at a turn end or a release.
basis: fct-yqdgc3b5
because: The stdin key now also sets a typed mark that only a human-filed prompt takes, and the told-wake rule reads it, so the claim must state the mark's life.
@row_set n=3 row=decision/wheel-byte-source-claim cell=rule seen=1 detector=D-arch-decisions
value: Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts; the key's typed mark clears only when a human-filed prompt takes it, never at a turn end or a release.
basis: fct-yqdgc3b5
because: The stdin key now also sets a typed mark that only a human-filed prompt takes, and the told-wake rule reads it, so the claim must state the mark's life.
@row_set n=4 row=contract/event-data/prompt-submitted-origin cell=promise seen=1 detector=none
value: `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`; the wrapper sets `driver` only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake `harness`.
basis: fct-yqdgc3b5
because: The wrapper now relabels a told wake `origin:"harness"`, so the origin contract no longer holds that the wrapper's only relabel is the driver one.
@row_set n=5 row=contract/event-prompt-submitted/origin cell=promise seen=1 detector=none
value: driver, human or harness; the hook files only human or harness; the wrapper sets driver only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake harness.
basis: fct-yqdgc3b5
because: The wrapper now relabels a told wake `origin:"harness"`, so the origin contract no longer holds that the wrapper's only relabel is the driver one.
@row_set n=6 row=contract/event-data/turn-ended-scheduled-wakes cell=promise seen=1 detector=none
value: `scheduled_wakes` is the Stop payload's `session_crons` array length, no element read or carried, `null` when that key is absent, `null` or not an array and on an API-error turn end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`, and the wrapper's wheel rule reads it.
basis: fct-yqdgc3b5
because: A turn end now records `scheduled_wakes` for the told-wake rule, so the count no longer crosses into the log and `wait` only.
@row_add n=1 row=decision/stop-session-crons-ledger-row area=design-philosophy fact=F3 detector=D-arch-resources
name: Stop session crons ledger row
rule: The stop-session-crons relied ledger row reads pass only when the Stop payload carries `session_crons` as an array; no element is read, and a missing key reads fail.
basis: fct-yqdgc3b5
because: The chunk adds the relied ledger row stop-session-crons, checked by `is_array()` alone, and the told-wake rule rests on it.
@row_add n=2 row=decision/fake-agent-wake-seams-argv-only area=occupied-resources fact=F4 detector=none
name: Fake agent wake seams are argv-only
rule: The fake agent's scheduled-wake test seams are the argv-only options --stop-crons and --wake-after-ms, each capped and writing only synthetic elements, never an environment variable or a fixture.
basis: fct-yqdgc3b5
because: The chunk adds two fake-agent seams as capped argv options with no env read and no fixture written, a seam form beside the env test seams.
