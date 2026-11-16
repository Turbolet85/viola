@row_add n=1 row=decision/wait-wakes-on-every-turn-end area=surface-cli fact=F1 detector=none
name: viola wait wakes on every turn end
rule: `viola wait` wakes on every turn end whatever its background-task count, and never on an `activity` record or a harness-origin `prompt-submitted`; viola carries no scheduled-wake count.
why: Telling a finished turn from a paused one is the driver's, not viola's.
basis: fct-xueqgrc6, fct-j7joxnr6
because: The chunk pins that the wake rule reads only the event kind, so a held turn end with background tasks wakes a parked wait and activity or a harness turn never does, and no row of the cli surface states when `wait` returns.
