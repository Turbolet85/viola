@row_set n=1 row=decision/wrapped-tui-reads-as-unwrapped cell=except seen=1 fact=F1 detector=none
value: in an untrusted folder a wrapped `claude` opens on its session box, viola's start having written the CLI's trust record (the flag alone) and typed no key; an unwrapped one opens on the trust form
basis: fct-e7fkhweh
because: The wrapped start now writes the trust record before the spawn, so the wrapped first screen is the session box where the unwrapped one is the trust form, and the row's sameness rule must name that exception.
@row_set n=2 row=decision/time-bounds-never-on-human cell=rule seen=3 fact=F2 detector=none
value: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
basis: fct-rntilpcs
because: The start's trust step may pause viola's start on the Claude config lock, a bound that falls on viola's start rather than the driver or the child, so the row's list of bounds and where they fall must follow.
@row_set n=3 row=decision/time-bounds-fall-on-the-driver cell=rule seen=3 detector=none
value: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-rntilpcs
because: The start's new lock wait falls on viola's own start, which the row's driver-or-child rule does not cover, so the rule must name it.
@row_set n=4 row=decision/run-diagnostics-off-terminal cell=rule seen=1 fact=F3 detector=none
value: viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
basis: fct-xro6zomc
because: The start gains a trust step whose outcome is one `trust-record` diagnostics line per start that never refuses the start or moves an exit code or hint, which the off-terminal rule must carry.
