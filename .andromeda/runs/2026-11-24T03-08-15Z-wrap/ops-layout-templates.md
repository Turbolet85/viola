@row_set n=1 row=contract/cli-verbs/help cell=promise seen=2 fact=F6 detector=D-design-cli-wording
value: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, verify and plugin install.
basis: fct-ud37d4a7
because: The board group of viola --help now holds focus beside list, so the help verb row must say what the board row holds.
@row_set n=2 row=contract/cli-exit-codes/21 cell=promise seen=1 fact=F4 detector=none
value: `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the strict-modes refusal of the frame verbs and `viola focus` the fixed state pair instead.
basis: fct-yz4mggmo
because: viola focus now exits 21 for a name not live and for a home or instance failing strict modes, so the exit-21 row must name focus among the verbs whose strict-modes refusal prints the fixed state pair.
@row_add n=1 row=contract/cli-verbs/focus area=surface-cli fact=F1 detector=D-layout-surface
boundary: CLI verbs
item: viola focus <name>
promise: Brings the named session's window forward and shows its workspace on Hyprland; it takes one ViolaName and `--json` only, and it is a CLI-only human control.
basis: fct-yz4mggmo
because: The chunk adds the viola focus verb, a new user-facing CLI surface the verb list must describe.
@row_add n=2 row=contract/viola-focus-output/focused area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola focus output
item: focused
promise: A success prints `<name>  focused` on stdout, or `{"v":1,"ok":{"name":"<name>"}}` under `--json`; no window address, pid or workspace is printed.
basis: fct-yz4mggmo
because: The chunk adds a printed success wording for viola focus that the masters must state as built.
@row_add n=3 row=contract/viola-focus-json/refusal-document area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus --json refusals
item: refusal document
promise: Each exit-1 refusal (`window-not-found`, `unsupported`, `unreachable`, `oversize`, `malformed`, `refused`) is one document `{"v":1,"error":"<code>","detail":null}` under `--json`.
basis: fct-yz4mggmo
because: The chunk adds six exit-1 refusal codes for viola focus, a contract at the CLI boundary the masters must state.
@row_add n=4 row=decision/focus-pairs-planning-forms area=surface-cli detector=D-design-cli-wording
name: viola focus refusal pairs as planning forms
rule: Each viola focus refusal prints a fixed unable/hint pair, and the wording of those pairs is a planning form, not yet the ratified wording.
state: provisional
basis: fct-tfjv3ute
because: The chunk adds six fixed unable/hint pairs for focus built on the provisional arm of fork F-C, so the masters must hold their wording as provisional.
@row_add n=5 row=contract/viola-focus/exit-21-order area=surface-cli detector=none
boundary: viola focus
item: exit-21 order
promise: A name not live exits 21 (`instance-dead` or `unwrapped-name`) before any compositor read, and a home or instance failing strict modes exits 21 `strict-modes-failed` before any log opens.
basis: fct-yz4mggmo
because: The chunk adds viola focus's exit-21 refusals and the order in which they stand before any compositor read or log, a contract other verbs' rows do not hold.
@row_add n=6 row=contract/viola-focus-refusals/unsupported area=surface-cli fact=F5 detector=none
boundary: viola focus refusals
item: unsupported
promise: Off Hyprland, viola focus refuses `unsupported`, exit 1, and spawns nothing.
basis: fct-gsppqa74
because: The chunk adds the off-Hyprland refusal of viola focus, a promise at the CLI boundary.
@row_add n=7 row=contract/viola-focus-refusals/window-not-found area=surface-cli detector=none
boundary: viola focus refusals
item: window-not-found
promise: With no window of its session, viola focus refuses `window-not-found`, exit 1, and dispatches nothing.
basis: fct-gsppqa74
because: The chunk adds the no-window refusal of viola focus, a promise at the CLI boundary.
@row_add n=8 row=absence/mcp-window-focus area=surface-cli fact=F7 detector=none
capability: An MCP tool or other driver-facing form that focuses a session's window
basis: fct-tfjv3ute
because: The chunk leaves a driver-facing focus out until the operator's word, a whole capability the project does not have.
@row_add n=9 row=contract/viola-help/board-row area=surface-cli detector=D-design-cli-wording
boundary: viola --help output
item: board row
promise: The board verb group row of `viola --help` reads `board:    list, focus`, plain ASCII.
basis: fct-ud37d4a7
because: The help board row moved from `board:    list` to `board:    list, focus`, and the standing verb-group row cannot take it within its bound.
