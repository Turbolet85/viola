@row_set n=1 row=decision/shared-captions-fixed cell=rule seen=1 detector=D-design-cli-wording
value: The six captions, words and slot order are fixed, and the terminal fleet console reuses them and adds `LAST` after them.
basis: fct-tol6uebc
because: The console's caption row is now the board's six captions then a seventh, LAST, so a row saying the console only reuses the six is incomplete.
@row_set n=2 row=decision/truncate-only-unwrapped-names cell=except seen=1 fact=F3 detector=D-design-cli-wording
value: The terminal fleet console cuts a whole line wider than the terminal at its columns, ending in `...`, never inside an escape, and cuts no unwrapped NAME on its own.
basis: fct-auq57yfv
because: The console cuts the whole line at the terminal's columns instead of the board's unwrapped NAME, an exception the row does not yet state.
@row_set n=3 row=contract/cli-token-rendering/motion-tokens cell=promise seen=1 detector=none
value: None; nothing redraws in place, except the terminal fleet console's frame, redrawn when its view changes.
basis: fct-aspdcuf3
because: The console now redraws its frame when its view changes, so a promise that nothing redraws in place is false for that surface.
@row_add n=1 row=decision/console-fleet-body-lines area=surface-console fact=F1 detector=D-layout-surface
name: Console fleet drawn as body lines
rule: The terminal fleet console draws the fleet as body lines between its title line and its key line, the board's rows in the board's forest order and indent.
basis: fct-aspdcuf3
because: The chunk adds the fleet body region to the console surface, which no row of the section describes.
@row_add n=2 row=contract/console-fleet/last-cell area=surface-console fact=F2 detector=D-design-cli-wording
boundary: the terminal fleet console's fleet lines
item: LAST
promise: The seventh caption, LAST, after the board's six; its cell is the age of the instance's newest log line in one unit (12s, 4m, 2h, 3d), unknown where there is no line, n/a on an unwrapped row.
basis: fct-aspdcuf3
because: The console adds a LAST column and its age wording, a printed form no row states.
@row_add n=3 row=contract/console-fleet/more-line area=surface-console fact=F4 detector=D-design-cli-wording
boundary: the terminal fleet console's fleet lines
item: ... <n> more
promise: A fleet taller than the body rows ends in a `... <n> more` line, the shown and hidden rows counted.
basis: fct-aspdcuf3
because: The console adds the overflow line as a new printed wording of its surface, which no row states.
@row_add n=4 row=decision/console-redraw-on-change area=surface-console fact=F5 detector=D-layout-surface
name: Console re-read and redraw on a change
rule: The terminal fleet console re-reads the board once a second and redraws only when its view changes, with no redraw while the terminal size and the view stand.
basis: fct-aspdcuf3
because: The chunk makes the console keep its fleet current by a timed re-read and a change-only redraw, a behaviour of the surface no row holds.
@row_add n=5 row=decision/console-agents-read-off-loop area=surface-console fact=F6 detector=none
name: Console agents read off the key loop
rule: The terminal fleet console reads the agents list on its own thread at most every 5 s, off the key loop, and a failed read leaves STATUS unknown.
why: Keeping the last good list after a failed read would show a stale busy.
basis: fct-klybeaot
because: The console now joins the agents list on its own thread, and the row records that choice and why a failed read empties STATUS.
@row_add n=6 row=contract/viola-console/mid-run-refusal area=surface-console fact=F7 detector=D-design-cli-wording
boundary: viola console
item: mid-run refusal
promise: A home widened while the console runs ends it with exit 1, the home refusal pair and one process-exit strict-modes-failed line; a board directory that cannot be listed ends it with `error: internal error`.
basis: fct-vecccir6
because: The console now re-reads the board while it runs, and its end on a refused or failed mid-run scan is a promised outcome no row states.
