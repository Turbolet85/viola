@row_set n=1 row=decision/time-bounds-never-on-human cell=rule seen=1 detector=none
value: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, hyprctl read, budget pause) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it.
basis: fct-woso52s6
because: The Hyprland seam adds a new time bound, the hyprctl read deadline that kills and reaps viola's own hyprctl child, so the row's list of the time bounds viola holds must name it.
@row_set n=2 row=decision/time-bounds-fall-on-the-driver cell=rule seen=1 detector=none
value: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, budget pause, statusline deadline, agents read, hyprctl read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
basis: fct-o2wtfen7
because: The hyprctl read deadline is a new time bound that falls on viola's own hyprctl child, so the row's enumeration of time bounds must include it to stay complete.
