@row_set n=1 row=decision/console-frame-floor cell=rule seen=1 detector=D-design-cli-wording
value: The console's frame floor is 80x24: at or above it the whole frame is drawn from a cleared top-left, `viola console` on row 1 and the key line on the last row; below it one line, `viola console needs 80x24; this terminal is <cols>x<rows>`; an unread size counts as 0x0.
basis: fct-htib4wp5, fct-v633oldo, fct-pabjcb3k
because: The key line `q quit` alone is retired and the last row now holds the whole key line, so the row's `q quit` on the last row no longer states the frame.
@row_set n=2 row=decision/console-own-surface cell=except seen=1 fact=F2 detector=none
value: The editing line of send, switch and open echoes its text as typed through row escaping, each scalar 1, 2 or 0 columns wide.
basis: fct-pabjcb3k, fct-zcv5bkss
because: The editing line is the named exception to the console's plain ASCII, and the row states plain ASCII with no exception.
@row_set n=3 row=decision/board-vs-strip cell=rule seen=3 detector=none
value: `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify`, `plugin install` and `console` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`, the console running those verbs on its selected row.
basis: fct-pabjcb3k, fct-zcv5bkss
because: The console now acts on its selected strip by running the strip verbs as CLI children, which the row does not say.
@row_add n=1 row=decision/console-key-line area=surface-console fact=F1 detector=D-design-cli-wording
name: Console key line
rule: The console's key line reads `q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open`, `q quit` first, and at the 80-column floor it is cut after `m swi`, so `a answer` and `o open` are not shown there.
guard: console_run_on_q_draws_the_frame_then_quits_restored
state: provisional
basis: fct-zcv5bkss, fct-htib4wp5, fct-ucszbfnd
because: The key line now holds every binding and its cut at the floor stands as built while the operator's fork is open, a wording no row states.
@row_add n=2 row=decision/console-editing-line area=surface-console detector=D-layout-surface
name: Console editing line in the key line's place
rule: The editing line of send, switch and open takes the key line's place with no second cut: the prompt, then the text; past the edge the prompt, then `...`, then the longest fitting tail, a straddling wide scalar and a width-0 head left out.
guard: editing_width_follows_the_one_rule
basis: fct-pabjcb3k, fct-zcv5bkss, fct-dxzom25q
because: The chunk adds the editing line as a new region of the console frame with its own width rule, which no row describes.
@row_add n=3 row=decision/console-selection area=surface-console fact=F3 detector=D-layout-surface
name: Console selection and scroll
rule: The console's selection is a two-column gutter on the selected wrapped row alone; the selection keeps its name when the fleet changes, a gone name takes the row at its place clamped, and the body scrolls to follow it with a `... <n> above` line and the more line.
guard: selection_keeps_its_name_when_the_fleet_changes
basis: fct-zcv5bkss, fct-odksttxp, fct-akj5dhl7
because: The chunk adds the selection and its scroll window to the console body, and the body-lines row cannot take them within its bound.
@row_add n=4 row=decision/console-result-rows area=surface-console fact=F4 detector=D-layout-surface
name: Console action results in the lowest body rows
rule: An action's output lines in the console are laid in the lowest rows of the body with the selection kept above them, and a running child shows one running line until it finishes.
guard: scroll_keeps_the_selection_above_the_result_rows
basis: fct-zcv5bkss, fct-outvcy4j
because: The chunk adds the result rows and the running line as a new region of the console frame, which no row describes.
@row_add n=5 row=decision/console-no-key-trap area=surface-console fact=F5 detector=none
name: No key trap in the console
rule: Every console mode is left by one key (`n` or escape from a confirm, escape from the editing line and the answer prompt), and Ctrl-C ends the console from every mode.
guard: console_ctrl_c_ends_from_every_mode
basis: fct-outvcy4j, fct-c6y5k4zm
because: The chunk adds the confirm, editing and answer modes, and the way out of each is a rule future modes must keep.
@row_add n=6 row=contract/console-prompts/editing area=surface-console fact=F6 detector=D-design-cli-wording
boundary: the console's prompts
item: send, switch and open editing prompts
promise: The editing prompts are fixed: `send <name>: `, `switch <name>: ` and `open child of <name>: `.
basis: fct-zcv5bkss
because: The chunk adds fixed prompt wordings to the console that no row states.
@row_add n=7 row=contract/console-prompts/answer area=surface-console detector=D-design-cli-wording
boundary: the console's prompts
item: answer prompt
promise: The answer prompt is fixed: `answer <name> permission? y allow  n deny` for a permission and `answer <name> plan? y approve  n revise` for a plan; `a` with nothing pending is unbound.
basis: fct-zcv5bkss, fct-pabjcb3k
because: The chunk adds the answer prompt's fixed wordings to the console, which no row states.
@row_add n=8 row=absence/console-adopt area=surface-console fact=F7 detector=none
capability: Adopting a session from the terminal fleet console
basis: fct-t6d232vl
because: The console gains every other verb but adopt, and its absence is a whole capability future work must not assume.
