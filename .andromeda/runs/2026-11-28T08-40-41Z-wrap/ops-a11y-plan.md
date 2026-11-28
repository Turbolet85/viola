@row_set n=1 row=decision/static-linear-output cell=except seen=1 detector=D-a11y-surface
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes
basis: fct-pz7mv7c6
because: The console verb added by the chunk redraws its frame on a resize, so the static, no-redraw rule now holds for viola's output except on that named surface.
@row_set n=2 row=decision/cli-not-keyboard-interactive cell=except seen=1 detector=D-a11y-surface
value: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode
basis: fct-pz7mv7c6
because: The console verb added by the chunk reads keys in raw mode, so the no-interactive-mode rule now holds except on that named surface.
@row_set n=3 row=decision/cli-no-interactive-input cell=except seen=1 detector=D-a11y-surface
value: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode and redraws on a resize
basis: fct-pz7mv7c6
because: The console verb added by the chunk reads keys interactively and redraws, so the no-interactive-input rule now holds except on that named surface.
@row_set n=4 row=decision/linear-static-output cell=except seen=1 detector=D-a11y-surface
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes
basis: fct-xexxpxch
because: The console verb draws a frame from a cleared top-left and redraws it at a changed size, so the linear, static rule holds except on that surface.
@row_set n=5 row=decision/linear-static-cli-output cell=except seen=1 detector=D-a11y-surface
value: the terminal fleet console (`viola console`), its own surface, which enters the alternate screen, hides the cursor and redraws its frame from a cleared top-left on a resize
basis: fct-xexxpxch
because: The console verb writes cursor control and redraws its frame on a resize, so the no-redraw, no-cursor-movement rule holds except on that surface.
@row_set n=6 row=decision/no-spinners cell=except seen=1 detector=D-a11y-surface
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes
basis: fct-xexxpxch
because: The console verb redraws its frame on a resize, so the no-redraw ban holds except on that surface.
@row_set n=7 row=contract/log-identity/process cell=promise seen=2 detector=D-a11y-obs-schema
value: `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run` and a `viola adopt` start; `process:"console"` with no instance for `viola console`.
basis: fct-u62b25hk
because: The diagnostics schemas' process enum gains `console`, the role `viola console` logs under with no instance, so the process tag contract must name it.
@row_set n=8 row=contract/cli-fixed-message/form cell=promise seen=4 detector=none
value: viola revive's exit-1 refusals, viola adopt's strict-modes-failed, session-live and not-resumed refusals, viola run's socket-directory refusal, viola verify's untrusted-folder refusal and viola console's two refusals print one unable: <text> line, then one hint: line last.
basis: fct-efbgxzuj
because: The console's two refusals are fixed unable/hint pairs with no document, so the fixed-message form must name them.
@row_set n=9 row=decision/strict-modes-refusal-before-any-write cell=rule seen=3 fact=F6 detector=none
value: A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs, link/unlink and focus, exit 1 for run, revive's and adopt's start, verify and console.
basis: fct-efbgxzuj
because: `viola console` refuses a home failing its read rule with exit 1 before any log opens and with no line written, so the rule's exit list gains the console.
@row_add n=1 row=contract/console-refusal/no-terminal area=cognitive-accessibility fact=F6 detector=D-a11y-surface
boundary: viola console refusals
item: no-terminal
promise: When stdin or stdout is not a terminal, `viola console` refuses before any home read or log line: exit 1, one `unable:` line and one `hint:` line last on stderr.
basis: fct-xexxpxch
because: The new console verb refuses off a terminal with its own unable/hint pair and exit 1, a promise at the CLI boundary no standing row holds.
@row_add n=2 row=decision/console-keys-quit-and-interrupt-only area=5-keyboard-navigation fact=F1 detector=D-a11y-surface
name: Console keys quit and interrupt only
rule: The terminal fleet console's only keys are q to quit and Ctrl-C to interrupt, both exiting 0; every other key is read as unbound and does nothing, and the end of its input ends the console.
state: provisional
basis: fct-xexxpxch
because: The chunk adds the console, a new keyboard-driven surface, and its closed key set is the keyboard coverage §5 owes it, resting on the delegate's provisional word.
@row_add n=3 row=decision/console-size-floor area=visual-design-verification fact=F2 detector=D-a11y-surface
name: Console 80x24 floor
rule: The terminal fleet console needs at least 80x24: below that floor it draws one line naming the floor and the terminal's own size instead of its frame, and it redraws when the terminal's size changes.
state: provisional
basis: fct-xexxpxch
because: The chunk adds the console's size floor and its one-line rule below it, a visual decision of the new surface no standing row holds.
@row_add n=4 row=decision/console-plain-ascii-no-sgr area=6-visual-design-verification fact=F3 detector=D-a11y-surface
name: Console plain ASCII with no SGR
rule: The terminal fleet console draws plain ASCII with no colour and no SGR, its title `viola console` on the first row and its key line `q quit` on the last row.
state: provisional
basis: fct-xexxpxch
because: The chunk adds the console's frame, drawn with no colour at all, a visual decision of the new surface no standing row holds.
@row_add n=5 row=decision/console-restores-terminal-first area=cognitive-accessibility fact=F5 detector=D-a11y-surface
name: Console restores the terminal first
rule: The terminal fleet console restores the terminal (leaves the alternate screen, shows the cursor, returns the line mode) before it returns, before any error line is printed and during a panic's unwind.
state: provisional
basis: fct-elardgcb
because: The chunk's console guarantees a usable terminal before any error text reaches the human, a rule of the new surface no standing row holds.
@row_none n=1 fact=F4 type=4
relied: the console's `console-key` line carries only the class word quit, interrupt or unbound, never the key's bytes, which the keystroke test driver reads
@row_none n=2 fact=F7 type=4
relied: the console's own `console-frame` and `console-key` diagnostics lines are what the keystroke test driver sequences on, as the standing record-sequenced boundary rule already says
