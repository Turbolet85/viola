@row_set n=1 row=decision/static-linear-output cell=except seen=2 detector=none
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes or the fleet it shows changes
basis: fct-o33yqef7
because: The console now redraws when its view of the fleet changes, so the console exception must name that redraw as well as the resize one.
@row_set n=2 row=decision/cli-no-interactive-input cell=except seen=2 detector=none
value: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode and redraws on a resize or a change in the fleet it shows
basis: fct-o33yqef7
because: The console now redraws on a change in the fleet it shows, so the exception must say so beside the resize redraw.
@row_set n=3 row=decision/claude-agents-read-fails-open cell=rule seen=2 detector=none
value: A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's and adopt's preflight; in the fleet console it hands an empty list, so STATUS reads unknown.
basis: fct-klybeaot
because: The console now runs the claude agents read and a failed read there hands an empty list, so the fail-open rule must name the console.
@row_set n=4 row=decision/state-carried-by-printed-word cell=rule seen=2 fact=F3 detector=none
value: Each board state is carried by its printed word (stale, DIALOG <kind>, expired, unverified-cli, idle, busy, n/a), never by colour alone; the fleet console's LAST cell carries the last activity as a word in one unit (12s, 4m, 2h, 3d), unknown or n/a.
basis: fct-aspdcuf3
because: The console adds a LAST cell whose value is carried by a printed word in one unit, so the row on state words must cover it.
@row_set n=5 row=decision/strict-modes-refusal-before-any-write cell=except seen=4 fact=F5 detector=none
value: the fleet console's mid-run refusal, after frames were drawn: the terminal is restored first, then exit 1 with the home pair and one process-exit strict-modes-failed line
basis: fct-vecccir6
because: A home widened while the console runs is now refused after frames were drawn and with one process-exit line, which the before-any-write rule does not hold.
@row_set n=6 row=decision/no-spinners cell=except seen=1 detector=none
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes or the fleet it shows changes
basis: fct-o33yqef7
because: The console now redraws when the fleet it shows changes, so the no-redraw ban must carry the console as its exception.
@row_set n=7 row=decision/linear-static-output cell=except seen=1 detector=none
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes or the fleet it shows changes
basis: fct-o33yqef7
because: The console now redraws when the fleet it shows changes, so the static-output rule must carry the console as its exception.
@row_set n=8 row=decision/linear-static-cli-output cell=except seen=1 detector=none
value: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes or the fleet it shows changes
basis: fct-o33yqef7
because: The console now redraws when the fleet it shows changes, so the no-redraw rule must carry the console as its exception.
@row_set n=9 row=contract/viola-list-board/caption cell=promise seen=1 detector=none
value: The caption order is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, reused by the terminal fleet console, which adds LAST after them.
basis: fct-tol6uebc
because: The console's caption row is now the board's six captions then a seventh LAST, so the reuse clause must say so.
@row_add n=1 row=decision/fleet-console-line-cut area=visual-design-verification fact=F1 detector=none
name: Fleet console lines cut at the columns
rule: In the fleet console an unwrapped NAME is not width-capped as on the board; the whole line is cut at the terminal's columns, ending `...`, never inside an escape.
basis: fct-auq57yfv
because: The chunk decided the console cuts whole lines instead of the board's NAME cap, a decision no standing row holds.
@row_add n=2 row=decision/fleet-console-more-line area=6-visual-design-verification fact=F2 detector=none
name: Hidden fleet rows named by a more line
rule: A fleet taller than the fleet console's body rows ends in a `... <n> more` line, so no row is hidden without a word saying so.
basis: fct-aspdcuf3
because: The console now shows a tall fleet only in part, and the word that names the hidden rows is a convention no standing row holds.
@row_add n=3 row=decision/console-agents-read-off-the-loop area=cognitive-accessibility fact=F4 detector=none
name: Fleet console agents read off the loop
rule: The fleet console's claude agents read runs on its own thread at most every 5 s, never on the console's loop: a key never waits on it, and an agents child may outlive a quit by up to its 5 s bound.
why: an agents child outliving a quit is accepted as built
state: provisional
basis: fct-qezvmgl2
because: The console gains a periodic claude agents read whose bound never falls on a key, and the time-bound rows cannot take it within their bound.
