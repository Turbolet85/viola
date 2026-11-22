@row_set n=1 row=decision/time-bounds-never-on-human cell=rule seen=5 detector=none
value: Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
basis: fct-shrbvdzl
because: The paste write now ends within the compiled PASTE_BOUND, a time bound on the driver's send, so the list names it beside every member it held, each word kept.
@row_set n=2 row=decision/time-bounds-fall-on-the-driver cell=rule seen=5 detector=none
value: Every time bound (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-shrbvdzl
because: The paste write now ends within the compiled PASTE_BOUND, a time bound on the driver's send, so the list names it beside every member it held, each word kept.
@row_set n=3 row=decision/human-keystrokes-never-blocked cell=rule seen=2 detector=none
value: A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current paste's in-flight chunk and closing marker.
basis: fct-z6owtgsx
because: The standing «delayed past the current atomic paste» is false since the paste became chunks on one writer thread; the bound now named is the in-flight chunk and the closing marker, nothing longer.
