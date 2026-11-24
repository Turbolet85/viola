@row_set n=1 row=contract/help-output/verb-groups cell=promise seen=4 fact=F1 detector=D-design-cli-wording
value: `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, verify, plugin install`, then `Options:`.
basis: fct-zdl3zlbi, fct-xpheltqv
because: The help's `setup:` row now reads `open, run, revive, adopt, up, verify, plugin install`, so the stated verb groups must name `adopt` after `revive`.
@row_set n=2 row=contract/viola-help/verb-groups cell=promise seen=4 fact=F1 detector=D-design-cli-wording
value: `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, verify, plugin install`, then `Options:`.
basis: fct-zdl3zlbi, fct-xpheltqv
because: The help's `setup:` row gains `adopt` after `revive`, so the twin verb-groups contract must state it.
@row_set n=3 row=decision/run-prints-nothing-while-wrapped cell=rule seen=1 detector=none
value: `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` or `viola adopt` that passed its preflight.
basis: fct-zdl3zlbi
because: `viola adopt` starts through `run`'s start and pumps its child as a silent role, so the silence while wrapped now covers an adopted start too.
@row_set n=4 row=contract/viola-run-output/child-started cell=promise seen=1 detector=none
value: `run`, `revive` and `adopt` print nothing once the child starts.
basis: fct-zdl3zlbi
because: `viola adopt` hands its child to `run`'s start and pump as a silent role, so the print-nothing promise now names it.
@row_set n=5 row=decision/start-refusals-show-no-path-or-pid cell=rule seen=1 fact=F3 detector=D-design-cli-wording
value: The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
basis: fct-zdl3zlbi, fct-slqf7dkf
because: `viola adopt` adds exit-1 start refusals in the fixed pair form with no id, path or pid in any pair, so the rule must name its start.
@row_set n=6 row=contract/cli-refusal-line/fixed-unable-form cell=promise seen=5 fact=F3 detector=D-design-cli-wording
value: The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
basis: fct-zdl3zlbi
because: `viola adopt`'s exit-1 refusals print the fixed `unable:` / `hint:` pair, so the list of verbs using the fixed form must include it.
@row_set n=7 row=contract/cli-stderr/fixed-message-refusal cell=promise seen=5 fact=F3 detector=D-design-cli-wording
value: The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
basis: fct-zdl3zlbi
because: `viola adopt`'s exit-1 refusals use the fixed `unable:` form, so the twin stderr contract must name it.
@row_set n=8 row=contract/cli-start-refusal/fixed-message cell=promise seen=1 fact=F4 detector=none
value: A start refusal of `run`, `revive` or `adopt` prints a fixed-message `unable:` line and one hint per cause; `revive`'s and `adopt`'s causes are read in a fixed order that stops at the first.
basis: fct-zdl3zlbi
because: `viola adopt` adds a start whose refusals print fixed pairs read in a fixed order where the first refusal wins.
@row_set n=9 row=decision/silent-catch-sites cell=rule seen=1 fact=F7 detector=none
value: At the catch site, `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent.
basis: fct-zdl3zlbi
because: The word `adopt` is now a named role silent at the catch site, so the list of silent catch sites must include it.
@row_set n=10 row=contract/cli-catch-site/silent-verbs cell=promise seen=1 fact=F7 detector=none
value: `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent at the catch site, under `--json` too.
basis: fct-zdl3zlbi
because: `adopt` joins the silent roles at the catch site, so the twin contract must name it.
@row_set n=11 row=decision/no-json-after-child-held-terminal cell=rule seen=1 fact=F7 detector=none
value: An error while pumping the child under `viola revive --json` or `viola adopt --json` prints no document.
basis: fct-zdl3zlbi
because: `viola adopt --json` prints the internal-error document only for an error before `pump_child`, its role silent once the child holds the terminal, as revive's start is.
@row_add n=1 row=name/command/adopt area=surface-cli fact=F1 detector=none
class: command
name: adopt
basis: fct-zdl3zlbi
because: `adopt` is a new served CLI verb, an occupied command name.
@row_add n=2 row=contract/adopt-json-refusal-code/strict-modes-failed area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: strict-modes-failed
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow, fct-zdl3zlbi
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, a contract the surface states item by item as it does revive's.
@row_add n=3 row=contract/adopt-json-refusal-code/session-live area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: session-live
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow, fct-zdl3zlbi
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, `session-live` among them.
@row_add n=4 row=contract/adopt-json-refusal-code/already-live area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: already-live
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, run's shared `already-live` among them.
@row_add n=5 row=contract/adopt-json-refusal-code/stale-heartbeat area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: stale-heartbeat
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, run's shared `stale-heartbeat` among them.
@row_add n=6 row=contract/adopt-json-refusal-code/pinned-hash-mismatch area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: pinned-hash-mismatch
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, run's shared `pinned-hash-mismatch` among them.
@row_add n=7 row=contract/adopt-json-refusal-code/squatted-name area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: squatted-name
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, run's shared `squatted-name` among them.
@row_add n=8 row=contract/adopt-json-refusal-code/socket-dir-refused area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: socket-dir-refused
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries.
basis: fct-spvlmkow
because: `viola adopt --json` adds its own closed list of exit-1 refusal codes, run's shared `socket-dir-refused` among them.
@row_add n=9 row=contract/adopt-json-refusal-code/internal-error area=surface-cli fact=F2 detector=none
boundary: adopt --json refusal code
item: internal-error
promise: One of the closed set of exit-1 codes a `viola adopt --json` start refusal document carries; an error before the child holds the terminal prints the same document.
basis: fct-spvlmkow, fct-zdl3zlbi
because: `viola adopt --json` prints the `internal-error` document for an error before `pump_child`, one item of its closed code list.
@row_add n=10 row=contract/cli-exit-codes/1-adopt-start area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: CLI exit codes
item: 1 of viola adopt
promise: A start refusal of `viola adopt` exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty.
basis: fct-zdl3zlbi, fct-gneiqmbq
because: `cli-exit-codes/1` lists the verbs whose exit-1 refusals print a `--json` document and has no room left under its bound, so adopt's exit-1 refusals stand as a row of their own.
@row_add n=11 row=contract/viola-adopt-refusals/session-live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt refusals
item: session-live
promise: A session open in another `claude` is refused with exit 1 and no child started: `unable: that session is open in another claude` then `hint: close it there first, then adopt it again`.
basis: fct-zdl3zlbi, fct-k2nhjvap
because: `viola adopt` adds a new fixed refusal pair for its live-elsewhere check, a wording held provisional on the delegate's word (W4) that no master states yet.
@row_add n=12 row=contract/viola-adopt-refusals/order area=surface-cli fact=F4 detector=none
boundary: viola adopt refusals
item: order
promise: `viola adopt` reads its refusal causes in a fixed order, the home-tree strict-modes check, the pinned executable, the wrapper log, the name holder, then the live-elsewhere check, and the first refusal wins; a refused holder takes no agents read, and a failed agents read still starts.
basis: fct-zdl3zlbi, fct-k2nhjvap
because: `viola adopt` adds a start whose refusal order is a promise its fixed pairs and documents follow.
@row_add n=13 row=contract/viola-adopt-usage/exit-2 area=surface-cli fact=F5 detector=none
boundary: viola adopt usage
item: exit 2
promise: A session id outside the closed session-id shape, ASCII hex of either case, or a `--parent` equal to `--name` with the fixed line `run` uses, is a usage exit 2 before anything is read.
basis: fct-zdl3zlbi, fct-jtbdy5p2, fct-tc34su7x
because: `viola adopt` adds a usage boundary on its id and names that no row of the surface states.
@row_none n=1 fact=F6 type=4
relied: an adopted start's lineage parent and its folder are recorded in the instance snapshot as every start's are; how the parent is chosen and that it logs as process `run` is how adopt is built, which the code and the architecture rows hold, not a design-system rule.
