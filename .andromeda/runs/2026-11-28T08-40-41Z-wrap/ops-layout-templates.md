@row_set n=1 row=contract/viola-help/verb-groups cell=promise seen=4 detector=D-design-cli-wording
value: Five rows `board:    list, focus, console`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink` and the setup row, served verbs only, plain ASCII, then a blank line and `Options:`; no about line and no clap `Commands:` list.
basis: fct-dcllhxcz
because: The help's board row now reads `list, focus, console`, so the verb-group row must state the new wording.
@row_set n=2 row=contract/cli-verbs/help cell=promise seen=6 detector=D-design-cli-wording
value: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list, focus and console, `handoff:` holding link, unlink and `setup:` holding open, run, revive, adopt, up, switch, verify and plugin install.
basis: fct-twjnmpkt
because: The board group gains `console`, so the help verb contract must name it among the board verbs.
@row_set n=3 row=decision/board-vs-strip cell=rule seen=2 detector=none
value: `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify`, `plugin install` and `console` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
basis: fct-xexxpxch
because: The new `viola console` verb takes no argument and names no strip, so the board-versus-strip rule must list it beside the other strip-less verbs.
@row_set n=4 row=decision/no-prompt-or-pager cell=rule seen=1 fact=F8 detector=D-layout-surface
value: No verb has an interactive prompt or a pager, save the terminal fleet console, `viola console`, its own surface, which reads keys in raw mode and redraws.
basis: fct-xexxpxch
because: The terminal fleet console is now the output of the `viola console` verb and reads keys, so the rule must name it as its exception.
@row_set n=5 row=decision/no-persistent-footer cell=except seen=1 fact=F8 detector=D-layout-surface
value: The terminal fleet console, `viola console`, its own surface, keeps `q quit` on the last row of its frame.
basis: fct-xexxpxch
because: The console's frame holds `q quit` on its last row, so the no-footer rule must name the console as its exception.
@row_set n=6 row=decision/human-tty-expression-level cell=except seen=1 fact=F8 detector=D-layout-surface
value: The terminal fleet console, `viola console`, its own surface, which reads keys in raw mode and redraws its frame.
basis: fct-xexxpxch
because: The console redraws its frame on a resize rather than printing static rows, so the static-output rule must name it as its exception.
@row_set n=7 row=decision/only-header-bay-line cell=except seen=1 detector=D-layout-surface
value: The terminal fleet console, `viola console`, its own surface, titles its frame `viola console` on row 1.
basis: fct-xexxpxch
because: The console's frame carries `viola console` on its first row, so the only-header rule must name the console as its exception.
@row_set n=8 row=decision/plain-output-modes cell=except seen=1 detector=D-layout-surface
value: The terminal fleet console, `viola console`, its own surface, enters the alternate screen with the cursor hidden and restores it on every end.
basis: fct-xexxpxch
because: The console writes the alternate-screen enter and restore sequences, cursor control the rule otherwise bans, so the rule must name the console as its exception.
@row_set n=9 row=contract/cli-token-rendering/motion-tokens cell=promise seen=1 detector=D-layout-surface
value: None; nothing redraws in place, save the terminal fleet console's frame, redrawn when the terminal size changes.
basis: fct-xexxpxch
because: The console redraws its frame on a changed size, so the claim that nothing redraws in place must name that exception.
@row_add n=1 row=contract/cli-verbs/console area=surface-cli fact=F6 detector=D-layout-surface
boundary: CLI verbs
item: viola console
promise: Shows the terminal fleet console in the human's terminal; it takes no argument, the global `--home` only and no `--json`, and exits 0 on quit, interrupt or end of input, 1 on a refusal or an internal error.
basis: fct-xexxpxch
because: The chunk adds the `viola console` verb, so the CLI verbs contract must hold its item.
@row_add n=2 row=contract/viola-console-refusals/no-terminal area=surface-cli fact=F7 detector=D-design-cli-wording
boundary: viola console refusals
item: no terminal
promise: With stdin or stdout not a terminal, before any home read, exit 1 with `unable: viola console needs a terminal` / `hint: run viola console in a terminal window`.
basis: fct-xexxpxch
because: The chunk adds this refusal pair to the console's start, so its wording must stand in the master.
@row_add n=3 row=contract/viola-console-refusals/home-refused area=surface-cli fact=F7 detector=D-design-cli-wording
boundary: viola console refusals
item: home not private
promise: A home failing its read rule, before any log opens, is exit 1 with `unable: viola console could not use its home` / `hint: make the viola home readable by you alone, then run viola console again`, no line written, the mode left as found.
basis: fct-xexxpxch
because: The chunk adds this refusal pair to the console's start, so its wording must stand in the master.
@row_add n=4 row=decision/console-own-surface area=new fact=F1 detector=D-layout-surface
section: Surface: console
name: Terminal fleet console as its own surface
rule: The terminal fleet console, `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end, in plain ASCII with no colour.
state: provisional
basis: fct-xexxpxch
because: The chunk adds the console as a new user-facing surface, which no section of the master describes yet.
@row_add n=5 row=decision/console-frame-floor area=new fact=F2 detector=D-layout-surface
section: Surface: console
name: Console frame floor 80x24
rule: The console's frame floor is 80x24: at or above it the whole frame is drawn from a cleared top-left, `viola console` on row 1 and `q quit` on the last row; below it one line, `viola console needs 80x24; this terminal is <cols>x<rows>`; an unread size counts as 0x0.
basis: fct-xexxpxch
because: The chunk sets the console's frame floor and its one-line rule below it, a decision of the new surface the master must hold.
@row_add n=6 row=decision/console-redraw-on-resize area=new fact=F3 detector=D-layout-surface
section: Surface: console
name: Console redraws on a resize
rule: The console redraws its frame when the terminal size changes, the size re-read on each key and every 100 ms, and draws once while the size stands.
state: provisional
basis: fct-xexxpxch
because: The console's frame is the one redraw on the CLI, so the master must state when it redraws.
@row_add n=7 row=decision/console-keys area=new fact=F4 detector=D-layout-surface
section: Surface: console
name: Console keys
rule: In the console `q` quits and Ctrl-C ends it, each with exit 0; every other key is unbound and does nothing; the end of input ends it with exit 0.
state: provisional
basis: fct-xexxpxch
because: The chunk binds the console's keys, a decision of the new surface the master must hold.
@row_none n=1 fact=F5 type=9
what: the fleet view columns and the action keys are owed items, held by the route's Fleet console view and Console actions.
