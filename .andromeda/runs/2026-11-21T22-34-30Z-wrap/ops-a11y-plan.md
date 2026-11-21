@row_set n=1 row=decision/time-bounds-fall-on-the-driver cell=rule seen=4 fact=F2 detector=none
value: Every time bound (MCP wait cap, MCP call deadline, dialog hold, readiness gate, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-xhke6zun
because: The chunk adds the MCP call deadline, a time bound on each MCP frame tool call that ends the driver's call, so the row's list of time bounds must name it.
@row_set n=2 row=decision/time-bounds-never-on-human cell=rule seen=4 detector=none
value: Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
basis: fct-xhke6zun
because: The MCP call deadline is a new time bound viola holds and it falls on the driver's MCP call, so the row's list of time bounds must name it.
@row_add n=1 row=contract/instance-unreachable-detail/call-deadline area=cognitive-accessibility fact=F1 detector=none
boundary: instance-unreachable detail
item: call-deadline
promise: An MCP frame tool call (send, wait, last, answer) outlived the MCP call deadline (55 s) and ended; the outcome of a send is unknown. Written by the MCP frame tools alone, never by the CLI, whose exit-21 cause set stays six.
basis: fct-vjs7vhbz
because: The closed unreachable-cause set gains call-deadline as its seventh member on the MCP instance-unreachable result, and the area keeps one contract row a detail member.
