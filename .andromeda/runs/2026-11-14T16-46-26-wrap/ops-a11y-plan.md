@row_set n=1 row=contract/cli-exit-codes/11 cell=promise seen=1 fact=F1 detector=none
value: Retired (was budget-paused) and never reused; an older wrapper's budget-paused refusal reads as unknown, exit 14.
basis: fct-xxks7iqj
because: The exit map now sends nothing to 11 and an older wrapper's budget-paused reads unknown (14), so the row naming 11 as budget-paused must state it retired and never reused.
@row_set n=2 row=contract/cli-usage-error/exit-2 cell=promise seen=1 fact=F2 detector=none
value: A clap usage error exits 2; `viola plugin` alone is such an error, and so is `viola release --budget`.
basis: fct-tjydxlay
because: The release verb's --budget flag is removed and now refused by clap with exit 2, a usage error this row's list of such errors must name.
@row_set n=3 row=decision/state-carried-by-printed-word cell=rule seen=1 detector=none
value: Each board state is carried by its printed word (stale, DIALOG <kind>, expired, unverified-cli, idle, busy, n/a), never by colour alone.
basis: fct-xxks7iqj
because: The WHEEL cell has no budget-paused form any more, so the board's printed state words no longer include budget-paused.
@row_set n=4 row=decision/time-bounds-never-on-human cell=rule seen=1 detector=none
value: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it.
basis: fct-vwne6ov4
because: The budget gate is removed whole, so the budget pause is no longer one of the time bounds viola holds.
@row_set n=5 row=decision/time-bounds-fall-on-the-driver cell=rule seen=1 detector=none
value: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, statusline deadline, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-vwne6ov4
because: The budget gate is removed whole, so the budget pause is no longer one of the time bounds the row lists.
@row_retire n=1 row=decision/budget-gate seen=1 detector=none
why: The budget gate on automated sends is removed whole, with its thresholds, its budget-paused refusal, release --budget and its Path 6 guard.
basis: fct-vwne6ov4
because: The chunk retires the budget gate and governor entirely, so no row may stand stating the gate refuses a send.
@row_retire n=2 row=decision/budget-gate-only-on-send seen=1 detector=none
why: No budget gate remains for send or any other verb to read, and its Path 6 guard is deleted.
basis: fct-vwne6ov4
because: With the gate removed the rule about which verbs read it is true of nothing.
@row_retire n=3 row=contract/config-json/five-hour seen=1 detector=none
why: config.json no longer has a five_hour key; the schema drops it and the budget thresholds are retired.
basis: fct-xalfd6ij
because: The config schema and the read keys drop five_hour, so its contract row describes a key that no longer exists.
@row_retire n=4 row=contract/config-json/seven-day seen=1 detector=none
why: config.json no longer has a seven_day key; the schema drops it and the budget thresholds are retired.
basis: fct-xalfd6ij
because: The config schema and the read keys drop seven_day, so its contract row describes a key that no longer exists.
@row_add n=1 row=contract/config-json/unknown-keys area=cognitive-accessibility fact=F3 detector=none
boundary: config.json
item: unknown keys
promise: A key outside v, diagnostics_level, claude_env_keep and dialog_hold_s, five_hour and seven_day among them, is logged once as unknown keys at its process's start and never refused.
basis: fct-xalfd6ij
because: The threshold keys left the schema, and a config.json still carrying them is logged once as unknown keys and never refused, a promise to the file's writer no row states.
