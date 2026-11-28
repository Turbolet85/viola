@row_add n=1 row=decision/console-own-surface area=surface-console new_area=Surface:_console detector=D-layout-surface
name: Terminal fleet console as its own surface
rule: The terminal fleet console, `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end, in plain ASCII with no colour.
state: provisional
basis: fct-xexxpxch
because: The chunk adds the console as a new user-facing surface, which no section of the master describes yet; the plan names a new Surface: console area for it.
@row_add n=2 row=decision/console-frame-floor area=surface-console new_area=Surface:_console detector=D-layout-surface
name: Console frame floor 80x24
rule: The console's frame floor is 80x24: at or above it the whole frame is drawn from a cleared top-left, `viola console` on row 1 and `q quit` on the last row; below it one line, `viola console needs 80x24; this terminal is <cols>x<rows>`; an unread size counts as 0x0.
basis: fct-xexxpxch
because: The chunk sets the console's frame floor and its one-line rule below it, a decision of the new surface the master must hold.
@row_add n=3 row=decision/console-redraw-on-resize area=surface-console new_area=Surface:_console detector=D-layout-surface
name: Console redraws on a resize
rule: The console redraws its frame when the terminal size changes, the size re-read on each key and every 100 ms, and draws once while the size stands.
state: provisional
basis: fct-xexxpxch
because: The console's frame is the one redraw on the CLI, so the master must state when it redraws.
