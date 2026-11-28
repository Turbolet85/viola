@row_set n=1 row=contract/help-output/verb-groups cell=promise seen=6 detector=D-design-cli-wording
value: `viola --help` lists the served verbs as `board: list, focus, console` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, switch, verify, plugin install`, then `Options:`.
basis: fct-twjnmpkt
because: The help's board row moved from `list, focus` to `list, focus, console`, so the row's stated help wording is stale.
@row_set n=2 row=contract/viola-help/verb-groups cell=promise seen=6 detector=D-design-cli-wording
value: `viola --help` lists the served verbs only, grouped `board: list, focus, console` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, switch, verify, plugin install`, then `Options:`.
basis: fct-dcllhxcz
because: The help's board row now reads `list, focus, console`, so the row's stated grouping is stale.
@row_set n=3 row=decision/list-is-the-board cell=rule seen=2 detector=none
value: `viola list` is the board, `verify`, `plugin install` and `console` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
basis: fct-xexxpxch
because: The new verb `viola console` is run by a person and is dispatched with no instance, so the row's claim that every other verb acts on one strip is false.
@row_set n=4 row=contract/cli-refusal-line/fixed-unable-form cell=promise seen=7 detector=D-design-cli-wording
value: The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up`, `verify` and `console` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
basis: fct-xexxpxch
because: `viola console`'s two exit-1 refusals print the fixed `unable:` form, so the row's list of verbs using it is incomplete.
@row_add n=1 row=decision/fleet-console-own-surface area=new fact=F1 detector=none
section: Surface: console
name: Fleet console as its own alternate-screen surface
rule: The terminal fleet console, `viola console`, is its own surface: it enters the alternate screen with the cursor hidden and restores both on every end, an error or a panic included.
state: provisional
basis: fct-xexxpxch
because: The chunk built the console's screen guard, and the new surface's entry and restore rule must stand for the work to come.
@row_add n=2 row=contract/fleet-console-frame/at-floor area=new fact=F2 detector=D-design-cli-wording
section: Surface: console
boundary: Fleet console frame
item: at or above the floor
promise: At or above the 80x24 floor the frame is drawn whole from a cleared top-left, the title `viola console` on row 1 and `q quit` on the last row.
basis: fct-xexxpxch
because: The console's frame wording is new output no master states.
@row_add n=3 row=contract/fleet-console-frame/below-floor area=new fact=F3 detector=D-design-cli-wording
section: Surface: console
boundary: Fleet console frame
item: below the floor
promise: Below the 80x24 floor the console draws one line, `viola console needs 80x24; this terminal is <cols>x<rows>`; a size that cannot be read counts as 0x0, not fitting.
basis: fct-xexxpxch
because: The console's below-floor line is new output no master states.
@row_add n=4 row=decision/fleet-console-redraws-on-resize area=new fact=F4 detector=none
section: Surface: console
name: Fleet console redraws on a size change
rule: The fleet console redraws its frame whenever the terminal size changes.
state: provisional
basis: fct-xexxpxch
because: The console is the one human surface that redraws, and that choice binds its later views.
@row_add n=5 row=contract/fleet-console-keys/q area=new fact=F5 detector=none
section: Surface: console
boundary: Fleet console keys
item: q
promise: `q` quits the fleet console with exit 0.
basis: fct-xexxpxch
because: The console's key set is a new closed contract with the human.
@row_add n=6 row=contract/fleet-console-keys/ctrl-c area=new fact=F5 detector=none
section: Surface: console
boundary: Fleet console keys
item: Ctrl-C
promise: Ctrl-C ends the fleet console with exit 0.
basis: fct-xexxpxch
because: The console's key set is a new closed contract with the human.
@row_add n=7 row=contract/fleet-console-keys/unbound area=new fact=F5 detector=none
section: Surface: console
boundary: Fleet console keys
item: any other key
promise: Every key other than `q` and Ctrl-C is unbound and ignored; the end of the console's input also ends it with exit 0.
basis: fct-xexxpxch
because: The console's key set is a new closed contract with the human.
@row_add n=8 row=decision/fleet-console-plain-ascii area=new fact=F6 detector=D-design-tokens
section: Surface: console
name: Fleet console in plain ASCII without colour
rule: The fleet console uses no colour and no SGR: its text is plain ASCII only.
state: provisional
basis: fct-pns455gq
because: The console's coverage reads tokens n/a with no colour, a choice its later views must keep.
@row_add n=9 row=name/command/console area=surface-cli fact=F7 detector=none
class: command
name: console
basis: fct-xexxpxch
because: The CLI verb `viola console` now occupies the name console.
@row_add n=10 row=contract/viola-console-refusals/no-terminal area=surface-cli fact=F8 detector=D-design-cli-wording
boundary: viola console refusals
item: no terminal
promise: With stdin or stdout not a terminal, `viola console` exits 1 before any home read with `unable: viola console needs a terminal` then `hint: run viola console in a terminal window`.
basis: fct-xexxpxch
because: The off-terminal refusal pair is a new fixed wording no master states.
@row_add n=11 row=contract/viola-console-refusals/home-refused area=surface-cli fact=F9 detector=D-design-cli-wording
boundary: viola console refusals
item: home failing its read rule
promise: A home failing its read rule refuses `viola console` with exit 1: `unable: viola console could not use its home` then `hint: make the viola home readable by you alone, then run viola console again`; no line is written and the mode is left as found.
basis: fct-xexxpxch
because: The home refusal pair is a new fixed wording no master states.
@row_add n=12 row=contract/cli-catch-site/console area=surface fact=F10 detector=D-design-cli-wording
boundary: CLI catch site
item: viola console
promise: At the catch site `viola console` restores the terminal first, then prints `error: internal error` once on stderr and exits 1, as the cli role.
basis: fct-elardgcb
because: The console's catch-site order, restore before the error line, is a new outward promise beside the standing catch-site items.
@row_add n=13 row=contract/cli-exit-codes/1-console area=surface-cli detector=D-design-cli-wording
boundary: CLI exit codes
item: 1 of viola console
promise: A refusal of `viola console`, off a terminal or for a home failing its read rule, exits 1 with its fixed `unable:` / `hint:` pair on stderr; `viola console` takes no `--json`.
basis: fct-xexxpxch
because: `viola console` adds two fixed exit-1 refusals, and the standing exit-1 row cannot name them within its bound.
