@row_set n=1 row=contract/channel-methods/wait-wake-kinds cell=promise seen=1 fact=F1 detector=none
value: Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks`; telling a finished turn from a paused one is the driver's, and a parked `wait` never polls the file.
basis: fct-xueqgrc6 · fct-j7joxnr6
because: The chunk pins that the wake reads only the event kind, so a turn end holding background tasks wakes a parked wait, and the wake-kinds contract must say so outward.
@row_set n=2 row=contract/channel-wait/wake-kinds cell=promise seen=1 fact=F2 detector=none
value: Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records, an activity record and a harness-origin prompt-submitted never wake, also after a task-free turn end.
basis: fct-xueqgrc6
because: The chunk pins that an activity record and a harness-origin prompt-submitted never wake a parked wait after a task-free turn end, and the non-waking list must name them.
@row_set n=3 row=contract/event-data/turn-ended-last-assistant-message cell=promise seen=2 fact=F3 detector=none
value: `turn-ended` data is `{last_assistant_message, background_tasks, cause?}` and carries no scheduled-wake count, so a driver cannot learn from the turn end that the CLI has a wake scheduled; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
basis: fct-j7joxnr6
because: The chunk records that viola carries no scheduled-wake count on turn-ended, the count left a founder candidate, so the turn-ended data contract must state the absence.
@row_set n=4 row=decision/test-home-owner-sweep cell=except seen=1 fact=F4 detector=none
value: A viola verify stamp dir under the e2e-home base, one per built test binary, holds no owner record and is removed only when its lock on the .lock sibling is taken without waiting.
basis: fct-xueqgrc6
because: Test homes are now copies of one stamp per built binary whose dir holds no owner record and another build's stamp is swept by its lock, so the owner-record-only sweep rule needs that exception.
@row_set n=5 row=decision/test-homes-cleaned-by-owner cell=except seen=1 detector=none
value: A viola verify stamp dir under the e2e-home base holds no owner record and is removed only when its lock on the .lock sibling is taken without waiting.
basis: fct-xueqgrc6
because: Another build's stamp dir, which holds no owner record, is now removed when its lock is taken without waiting, so the owner-process-only cleanup rule needs that exception.
