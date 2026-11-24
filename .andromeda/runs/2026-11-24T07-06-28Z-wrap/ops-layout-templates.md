@row_set n=1 row=contract/cli-verbs/help cell=promise seen=3 detector=D-design-cli-wording
value: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, up, verify and plugin install.
basis: fct-wivrj76k
because: The help's `setup:` row now lists `up` between revive and verify, so the row's setup list must name it.
@row_set n=2 row=contract/viola-revive/no-window cell=promise seen=1 fact=F8 detector=none
value: viola revive places no window unless given its hidden `--origin <cli|agent>`, under which the revived window moves to the workspace a real snapshot recorded, or reads unplaced when none was recorded.
basis: fct-ykd4mawd
because: Revive's hidden `--origin` now replays the recorded workspace, so a revive no longer never places a window.
@row_set n=3 row=contract/viola-revive/usage cell=promise seen=2 detector=none
value: `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments.
basis: fct-ykd4mawd
because: The new hidden `--origin` conflicts with `--list`, so the row's list of what `--list` conflicts with is incomplete.
@row_set n=4 row=contract/cli-exit-codes/21 cell=promise seen=2 detector=none
value: `unable  instance-unreachable`; for `viola list`, `viola plugin install`, the board refusal of `viola up` and the strict-modes refusal of the frame verbs and `viola focus` the fixed state pair instead.
basis: fct-ykd4mawd
because: viola up now exits 21 for the board's strict refusal, so the exit-21 row must name it beside the other strict-modes refusals.
@row_set n=5 row=decision/board-vs-strip cell=rule seen=1 detector=none
value: `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
basis: fct-ykd4mawd
because: viola up acts on the fleet with optional names, so the rule that every other verb acts on one strip no longer holds without it.
@row_add n=1 row=contract/viola-help/setup-row area=surface-cli detector=D-design-cli-wording
boundary: viola --help output
item: setup row
promise: The `setup:` verb group row of `viola --help` reads `setup:    open, run, revive, up, verify, plugin install`.
basis: fct-wivrj76k
because: The `setup:` row gained `up`, and the verb-group row states the old list while its set would pass the cell's bound.
@row_add n=2 row=contract/cli-verbs/up area=surface-cli fact=F1 detector=D-layout-surface
boundary: CLI verbs
item: viola up [<NAME>...] [--dry-run] [--json]
promise: Re-opens the fleet after a reboot: each candidate instance is revived in its own terminal at its recorded working directory, parents before children, and instances already live are skipped; it is CLI-only, with no MCP tool and no channel method.
basis: fct-ykd4mawd
because: The chunk adds the viola up verb as a new CLI surface, CLI-only, which no row describes.
@row_add n=3 row=contract/viola-up-output/line area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola up output
item: instance line
promise: One line per instance, `<name>  <word>` or `<name>  live  <placement>`, the word one of the closed set `live`, `already-live`, `stale-heartbeat`, `would-open`, `no-session`, `cwd-missing`, `session-live`, `not-live`, `not-taken`.
basis: fct-ykd4mawd
because: viola up adds a new printed line form and a closed set of outcome words that the master does not state.
@row_add n=4 row=contract/viola-up-output/exit-codes area=surface-cli fact=F3 detector=none
boundary: viola up output
item: exit codes
promise: Exit 0 when every line reads `live`, `already-live`, `stale-heartbeat` or `would-open`, else 1; exit 21 for the board's strict refusal; exit 1 with a document for `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch` and `terminal-not-found`.
basis: fct-ykd4mawd
because: viola up adds its own exit contract, which no exit row states.
@row_add n=5 row=contract/viola-up-output/line-order area=surface-cli fact=F4 detector=none
boundary: viola up output
item: line order
promise: Every line that is not a spawn comes first, in name order, then the spawned or `would-open` lines in spawn order.
basis: fct-tpkehh2e
because: viola up's line order is a new output contract the master does not state.
@row_add n=6 row=contract/viola-up/named-instances area=surface-cli detector=none
boundary: viola up selection
item: named instances
promise: With names given, the fleet narrows to them and a name not scanned reads `not-taken`; with none, the whole fleet is read.
basis: fct-urp643y2
because: viola up's names narrow the fleet rather than add to it, a contract no row states.
@row_add n=7 row=contract/viola-up-output/not-live-hint area=surface-cli fact=F5 detector=D-design-cli-wording
boundary: viola up output
item: not-live hint
promise: After its lines, viola up writes one stderr `hint:` line when any line reads `not-live`, and none under `--json`.
basis: fct-ykd4mawd
because: viola up adds a hint written after its result lines, which no row states.
@row_add n=8 row=contract/viola-up-output/dry-run area=surface-cli detector=none
boundary: viola up output
item: --dry-run
promise: `--dry-run` prints the plan as `would-open` lines and spawns nothing.
basis: fct-m4vsf7nm
because: viola up adds a dry-run form whose output no row states.
@row_add n=9 row=contract/viola-up-output/json-document area=surface-cli fact=F6 detector=none
boundary: viola up output
item: --json document
promise: Under `--json` viola up prints one document, `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.
basis: fct-ykd4mawd
because: viola up adds a new `--json` document shape that no row states.
@row_add n=10 row=contract/viola-up-output/no-address area=surface-cli detector=none
boundary: viola up output
item: withheld fields
promise: No line and no document of viola up carries an address, a pid or a workspace.
basis: fct-m4vsf7nm
because: viola up's output withholds addresses, pids and workspaces, a promise no row states.
@row_add n=11 row=contract/viola-up-refusals/boot-time-unread area=surface-cli fact=F7 detector=D-design-cli-wording
boundary: viola up refusals
item: boot-time-unread
promise: When the host's boot time cannot be read, viola up refuses with exit 1 and its own `unable:` / `hint:` pair, or under `--json` the `boot-time-unread` document.
basis: fct-ykd4mawd
because: The chunk adds the `boot-time-unread` refusal word with its own pair and hint, which no row states.
@row_add n=12 row=name/command/up area=surface-cli fact=F9 detector=none
class: command
name: up
basis: fct-ykd4mawd
because: The command name up is now occupied by the viola up verb.
