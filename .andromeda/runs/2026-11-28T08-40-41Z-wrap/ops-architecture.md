@row_add n=1 row=decision/console-own-surface area=established-decisions fact=F1 detector=D-arch-decisions
name: Fleet console's own terminal surface
rule: viola console draws on the alternate screen with the cursor hidden: a whole frame at or above an 80x24 floor, below it one line naming both sizes, a redraw on a resize, and the terminal restored on every end before any error line.
guard: tui_console end-to-end cases
state: provisional
basis: fct-xexxpxch
because: The chunk lands viola console as the fleet console's own surface, which redraws and owns the terminal unlike every other verb, so a decision row must name that surface.
@row_add n=2 row=decision/console-start-checks area=occupied-resources fact=F2 detector=D-arch-resources
name: Console start checks before any log
rule: viola console checks that stdin and stdout are terminals, then holds the home to the strict-modes read rule, both before any log opens; either refusal exits 1 with its own fixed unable and hint pair and writes no line.
state: provisional
basis: fct-xexxpxch
because: The new console verb holds the home to the read rule behind a terminal check, a start order and refusal pair no standing row names for it.
@row_add n=3 row=contract/diagnostics-role-files/console area=occupied-resources fact=F3 detector=D-arch-resources
boundary: home-level diagnostics role files
item: console.ndjson
promise: viola console's codes-only lines, with or without an instance: its own process-start without endpoint_kind, its console-frame and console-key lines, and a process-exit at its end.
basis: fct-xexxpxch
because: The console is a fifth home-level process role with its own role file, which the role-file register must list beside run, hook, mcp and cli.
@row_add n=4 row=decision/console-keys-by-closed-class area=cross-cutting-patterns fact=F4 detector=D-leaf-figures
name: Console keys read bounded and logged by class
rule: The fleet console reads its keys in reads bounded by the compiled read cap (READ_CAP), each decoded to one closed class, quit, interrupt or unbound, and logged by that class alone, never by its bytes.
state: provisional
basis: fct-xexxpxch
because: The console's key reader is a new external byte reader whose bound and closed class keep key bytes out of every line, a rule future console work must keep.
@row_add n=5 row=name/command/console area=occupied-resources detector=D-arch-resources
class: command
name: console
basis: fct-xexxpxch
because: The chunk lands the CLI verb viola console, an occupied command name the register does not yet hold.
@row_set n=1 row=decision/codes-only-process-logs cell=rule seen=1 detector=D-arch-resources
value: Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp, cli or console ndjson).
basis: fct-twjnmpkt
because: The obs process roles moved from four to five with console, so the row's role list is stale without it.
@row_set n=2 row=contract/cli-exit-codes/1 cell=promise seen=4 detector=none
value: Internal error; the start, revive-preflight, open, focus, up, verify and console refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
basis: fct-xexxpxch
because: viola console's two refusals exit 1 with a fixed unable and hint pair, so the exit-1 promise must name the console among its refusals.
@row_set n=3 row=contract/cli-internal-error/output cell=promise seen=1 fact=F5 detector=none
value: A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint, viola console only after it has restored the terminal; under --json before a -- the internal-error document is printed on stdout instead.
basis: fct-elardgcb
because: The console behaves as cli at the catch site but restores its terminal first, which the internal-error promise must state.
@row_set n=4 row=decision/new-capabilities-as-verbs cell=except seen=1 detector=D-arch-decisions
value: viola console, the human's terminal fleet console, takes no --json.
basis: fct-ooj26e7o
because: The new console verb takes no --json, an exception to the rule that capabilities are exposed on the CLI with --json.
@row_set n=5 row=decision/cli-human-text-json-exit-codes cell=except seen=1 detector=D-arch-decisions
value: viola console, the terminal fleet console's own surface, takes no --json and draws its own screen.
basis: fct-ooj26e7o
because: The console verb prints no machine document under --json, so the human-text-and-json rule needs it as its named exception.
@row_set n=6 row=decision/cli-output-conventions cell=except seen=1 detector=D-arch-decisions
value: viola console, the terminal fleet console's own surface, takes no --json and draws its own screen.
basis: fct-ooj26e7o
because: The console verb prints no machine document under --json, so the CLI output convention needs it as its named exception.
