@row_set n=1 row=contract/help-output/verb-groups cell=promise seen=3 detector=D-design-cli-wording
value: `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, up, verify, plugin install`, then `Options:`.
basis: fct-wivrj76k (Counts / qualifiers moved: the help's `setup:` row)
because: The help's `setup:` row now lists `up` between `revive` and `verify`, so the row stating the old group is stale.
@row_set n=2 row=contract/viola-help/verb-groups cell=promise seen=3 detector=D-design-cli-wording
value: `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, up, verify, plugin install`, then `Options:`.
basis: fct-wivrj76k (Counts / qualifiers moved: the help's `setup:` row)
because: The help's `setup:` row now lists `up` between `revive` and `verify`, so the row stating the old group is stale.
@row_set n=3 row=decision/list-is-the-board cell=rule seen=1 detector=none
value: `viola list` is the board, `verify` and `plugin install` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
basis: fct-urp643y2 (Deviations 1: named instances narrow the fleet; with none, the whole fleet)
because: `viola up` is a verb a person runs that acts on many strips at once, so the rule that every other verb acts on one strip no longer holds as written.
@row_set n=4 row=contract/exit-21-json-cause/strict-modes-failed cell=promise seen=2 detector=none
value: The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
basis: fct-ykd4mawd (Symbols / APIs, viola up exits: 21 for the board's strict refusal, `instance-unreachable` / `strict-modes-failed`)
because: `viola up` exits 21 with `strict-modes-failed` on the board's strict refusal, so the list of verbs reading that cause must name it.
@row_set n=5 row=contract/cli-refusal-line/fixed-unable-form cell=promise seen=4 detector=D-design-cli-wording
value: The exit-1 refusals of `run`, `revive`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
basis: fct-ykd4mawd (Symbols / APIs, Human output: `human::boot_time_unread_pair`; exits 1 for `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch`, `terminal-not-found`)
because: `viola up` adds exit-1 refusals printed as fixed `unable:` / `hint:` pairs, so the verbs using the fixed form must include it.
@row_set n=6 row=contract/cli-stderr/fixed-message-refusal cell=promise seen=4 detector=D-design-cli-wording
value: The exit-1 refusals of `viola run`, `revive`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
basis: fct-ykd4mawd (Symbols / APIs, Human output: `human::boot_time_unread_pair`; exits 1 for `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch`, `terminal-not-found`)
because: `viola up` adds exit-1 refusals printed as fixed `unable:` / `hint:` pairs, so the verbs using the fixed form must include it.
@row_add n=1 row=decision/up-cli-only area=surface-cli fact=F1 detector=none
name: viola up is a CLI-only verb
rule: `viola up [NAME...] [--dry-run] [--json]` is a CLI-only verb of the `setup:` group, with no MCP tool and no channel method.
basis: fct-ykd4mawd (Symbols / APIs: the CLI verb `viola up`, CLI-only; no MCP tool, no channel method)
because: A new verb is served on the CLI alone, a choice future surfaces must respect.
@row_add n=2 row=decision/up-names-narrow-the-fleet area=surface-cli detector=none
name: Names given to viola up narrow the fleet
rule: With names given, `viola up` reads only the instances named and re-opens no other; a name not on the board reads `not-taken`; with no name it reads the whole fleet.
why: Narrowing makes `viola up <name>` re-open no other instance.
basis: fct-urp643y2 (Deviations 1: named instances narrow the fleet)
because: The chunk chose narrowing over adding for named instances, with its reason stated.
@row_add n=3 row=name/command/up area=surface-cli detector=none
class: command
name: up
basis: fct-ykd4mawd (Symbols / APIs: the CLI verb `viola up`)
because: `up` is a newly occupied verb name beside the other command names.
@row_add n=4 row=contract/cli-verb-output/up area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: CLI verb output
item: viola up
promise: `viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, or under `--json` one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.
basis: fct-ykd4mawd (Symbols / APIs, viola up Output)
because: The new verb's printed lines and its `--json` document are output wordings the master must state.
@row_add n=5 row=contract/viola-up-outcome/live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: live
promise: `live` is one of the nine closed outcome words of `viola up`: the spawned instance came up live, and its line carries the placement word.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words; Output)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=6 row=contract/viola-up-outcome/already-live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: already-live
promise: `already-live` is one of the nine closed outcome words of `viola up`: the instance is live and is not re-opened.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=7 row=contract/viola-up-outcome/stale-heartbeat area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: stale-heartbeat
promise: `stale-heartbeat` is one of the nine closed outcome words of `viola up`: the instance reads stale and is not re-opened.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=8 row=contract/viola-up-outcome/would-open area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: would-open
promise: `would-open` is one of the nine closed outcome words of `viola up`: a `--dry-run` plan line for an instance that would be spawned.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=9 row=contract/viola-up-outcome/no-session area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: no-session
promise: `no-session` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words; Exits)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=10 row=contract/viola-up-outcome/cwd-missing area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: cwd-missing
promise: `cwd-missing` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words; Exits)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=11 row=contract/viola-up-outcome/session-live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: session-live
promise: `session-live` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words; Exits)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=12 row=contract/viola-up-outcome/not-live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: not-live
promise: `not-live` is one of the nine closed outcome words of `viola up`: a spawned instance that did not come up live; a line reading it makes `up` exit 1.
basis: fct-ykd4mawd (Symbols / APIs, viola up closed outcome words; Exits)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=13 row=contract/viola-up-outcome/not-taken area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola up outcome words
item: not-taken
promise: `not-taken` is one of the nine closed outcome words of `viola up`: a name given that is not on the board; a line reading it makes `up` exit 1.
basis: fct-urp643y2 (Deviations 1: a name not scanned reads `not-taken`)
because: The closed outcome word set is a new printed contract, one row an item.
@row_add n=14 row=contract/viola-up-output/line-order area=surface-cli fact=F4 detector=none
boundary: viola up output
item: line order
promise: Every `viola up` line that is not a spawn comes first, in name order, then the spawned or `would-open` lines in spawn order, parents before children.
basis: fct-tpkehh2e (Deviations 4: line order)
because: The order of the new verb's lines is a promise its readers rely on.
@row_add n=15 row=contract/viola-up-exit-codes/0 area=surface-cli fact=F5 detector=none
boundary: viola up exit codes
item: 0
promise: `viola up` exits 0 when every line reads `live`, `already-live`, `stale-heartbeat` or `would-open`.
basis: fct-ykd4mawd (Symbols / APIs, viola up Exits)
because: The new verb's exit codes are a contract the CLI exit rows do not yet state, and the exit-1 row is at its bound.
@row_add n=16 row=contract/viola-up-exit-codes/1 area=surface-cli detector=none
boundary: viola up exit codes
item: 1
promise: `viola up` exits 1 when any line reads another word, and with a document under `--json` for its refusals `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch` and `terminal-not-found`.
basis: fct-ykd4mawd (Symbols / APIs, viola up Exits)
because: The new verb's exit 1 is a contract the CLI exit-1 row cannot hold within its bound.
@row_add n=17 row=contract/viola-up-exit-codes/21 area=surface-cli detector=none
boundary: viola up exit codes
item: 21
promise: `viola up` exits 21 with no line on the board's strict-modes refusal, `instance-unreachable` with `strict-modes-failed`.
basis: fct-ykd4mawd (Symbols / APIs, viola up Exits); fct-m4vsf7nm (exit 21 with no line)
because: The new verb's exit 21 is a contract its readers rely on.
@row_add n=18 row=contract/viola-up-output/dry-run area=surface-cli fact=F8 detector=none
boundary: viola up output
item: --dry-run
promise: `viola up --dry-run` prints the plan as `would-open` lines and spawns nothing.
basis: fct-m4vsf7nm (Acceptance: `--dry-run` spawns nothing); fct-ykd4mawd (closed outcome words)
because: The dry run is a new printed form of the verb.
@row_add n=19 row=decision/up-writes-no-instance-state area=surface-cli detector=none
name: viola up writes no instance state
rule: `viola up` writes no snapshot, event line or stamp of its own; each instance it re-opens records its own start.
basis: fct-ykd4mawd (Symbols / APIs: it writes no snapshot, event line or stamp); fct-lwvzzjno
because: The new verb is bound to leave instance state to the instances it re-opens.
@row_add n=20 row=decision/revive-origin-hidden-flag area=surface-cli fact=F9 detector=none
name: Hidden --origin of viola revive places its window
rule: `viola revive` takes a hidden `--origin <cli|agent>` of closed values, conflicting with `--list` and listed in no help; only under it does a revive place a window, at the workspace a real snapshot recorded, else `unplaced`.
basis: fct-ykd4mawd (Symbols / APIs: `viola revive`'s hidden `--origin`); fct-mf7rueph
because: A revive now places its window under a hidden flag, a choice of surface the master must hold.
@row_add n=21 row=contract/fixed-refusal-pairs/boot-time-unread area=surface fact=F6 detector=D-design-cli-wording
boundary: fixed refusal pairs
item: boot-time-unread pair
promise: When the host boot time cannot be read, `viola up` selects nothing and prints its own fixed `unable:` / `hint:` pair, or under `--json` an exit-1 document with the code `boot-time-unread`.
basis: fct-ykd4mawd (Human output: `human::boot_time_unread_pair`; Exits); fct-uiucoyys
because: A new refusal word with its own fixed pair is an output wording the master must state.
@row_add n=22 row=contract/viola-up-output/not-live-hint area=surface fact=F7 detector=D-design-cli-wording
boundary: viola up output
item: not-live hint
promise: After its lines, `viola up` writes one fixed stderr `hint:` line when any line reads `not-live`, and none under `--json`.
basis: fct-ykd4mawd (Symbols / APIs: after the lines, one stderr `hint:` line; `human::UP_NOT_LIVE_HINT`)
because: A new fixed hint line is an output wording the master must state.
@row_none n=1 fact=F10 type=4
relied: each instance `viola up` spawns waits on `viola open`'s live bound, and `up` mints none of its own; the code holds it, and the a11y time-bound lists wait on their own fork.
