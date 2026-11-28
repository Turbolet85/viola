@row_set n=1 row=decision/truncate-only-unwrapped-name cell=except seen=1 detector=D-design-cli-wording
value: the fleet console cuts a whole line wider than the terminal at its columns, ending in `...`, and cuts no unwrapped NAME separately
basis: fct-auq57yfv
because: The fleet text no longer applies the board's width-capped NAME cut and cuts the whole line instead, so the rule that only an unwrapped NAME is truncated must name the console as its exception.
@row_set n=2 row=contract/human-tables/name-truncation cell=promise seen=1 detector=D-design-cli-wording
value: Where the width is detectable, human tables outside the fleet console truncate only the NAME of unwrapped rows, ending it in `...`.
basis: fct-auq57yfv
because: The fleet console cuts its whole line at the terminal's columns rather than an unwrapped NAME, so the NAME-only truncation promise holds for the other human tables alone.
@row_set n=3 row=decision/escape-controls-in-human-output cell=rule seen=1 detector=none
value: Human output of `last`, `list` and the fleet console escapes C0/C1 control characters.
basis: fct-mhjm6wnn
because: Every cell of the fleet console's frame is now row-escaped, under its control `fleet-escape`, so the rule naming the human outputs that escape controls must name the console too.
@row_add n=1 row=contract/fleet-console-view/body-rows area=surface-console fact=F1 detector=none
boundary: Fleet console view
item: body rows
promise: The console draws the board's rows in the board's forest order, in its state and wheel words, as body lines between the title and the key line.
basis: fct-aspdcuf3
because: The console now draws the fleet, and its rows promise the board's order and words, which no row of the console surface states.
@row_add n=2 row=contract/fleet-console-view/captions area=surface-console fact=F2 detector=D-design-cli-wording
boundary: Fleet console view
item: caption row
promise: The console's caption row is the board's six captions, then a seventh, `LAST`.
basis: fct-tol6uebc
because: The console adds a seventh caption after the board's six, a printed wording no master states.
@row_add n=3 row=contract/fleet-console-view/last area=surface-console fact=F3 detector=D-design-cli-wording
boundary: Fleet console view
item: LAST
promise: `LAST` is the age of the instance's newest log line in one unit (`12s`, `4m`, `2h`, `3d`); `unknown` on a wrapped row with no log line, `n/a` on an unwrapped row.
basis: fct-aspdcuf3
because: The new `LAST` cell prints an age form and two fixed words that no row states.
@row_add n=4 row=contract/fleet-console-view/line-cut area=surface-console fact=F4 detector=D-design-cli-wording
boundary: Fleet console view
item: line cut
promise: A console line wider than the terminal is cut whole at its columns ending in `...`, never inside an escape; an unwrapped NAME is not cut separately in the console.
basis: fct-auq57yfv
because: The console cuts its lines by its own rule rather than the board's NAME cut, a decision taken on his F4 that no row states.
@row_add n=5 row=contract/fleet-console-view/more-line area=surface-console fact=F5 detector=D-design-cli-wording
boundary: Fleet console view
item: more line
promise: A fleet taller than the console's body rows ends in a line of the form `... <n> more`.
basis: fct-aspdcuf3
because: The console prints a new more line for a fleet past its rows, a wording no master states.
@row_add n=6 row=decision/console-reread-once-a-second area=surface-console fact=F6 detector=none
name: Fleet console re-read once a second
rule: The console re-reads the board once a second and redraws only when its view or the terminal size changes.
guard: console_run_redraws_when_the_view_changes
basis: fct-gcaco7hy
because: The console now keeps the fleet current on a fixed interval and redraws on a change alone, a chosen cadence future console work must respect.
@row_add n=7 row=decision/console-agents-read-off-the-loop area=surface-console fact=F7 detector=none
name: Fleet console agents read on its own thread
rule: The console reads the claude agents list on its own thread at most every 5 s; a failed read gives an empty list, so STATUS reads `unknown`.
why: Keeping the last good list would show a stale `busy`.
guard: console_reads_the_agents_list_off_the_loop
basis: fct-klybeaot
because: The console gains an agents read with a chosen spacing and a fail-open empty list whose reason the report states, which no row holds.
@row_add n=8 row=contract/viola-console-refusals/mid-run-home-widened area=surface-console fact=F8 detector=D-design-cli-wording
boundary: viola console refusals
item: home widened mid-run
promise: A home widened while the console runs ends it with exit 1 and the home-state `unable:` / `hint:` pair after the terminal is restored; a board directory that cannot be listed ends it as `error: internal error`.
basis: fct-vecccir6
because: The console gains a mid-run refusal with its exit code and printed pair, an outward promise no console refusal row states.
@row_add n=9 row=decision/console-fleet-text-uncoloured area=surface-console fact=F9 detector=none
name: Uncoloured fleet console text
rule: The fleet console's fleet text is plain ASCII with no colour.
basis: fct-mhjm6wnn
because: The console's fleet text carries no SGR colour, unlike the board's amber and dim spans, a choice the console surface does not yet state.
@row_add n=10 row=decision/last-activity-console-only area=surface-console fact=F10 detector=none
name: Last activity is the console's alone
rule: The last-activity value is the console's alone; `viola list`, `list --json` and the MCP list item never carry it.
guard: board_read_item_never_serialises_last_activity
basis: fct-xjbaemp6
because: The board item gains a last-activity value kept out of every list output, a boundary future list work must respect.
