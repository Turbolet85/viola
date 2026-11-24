@row_set n=1 row=decision/strict-modes-before-any-log cell=rule seen=2 detector=none
value: Frame verbs, link verbs, starts, viola focus and viola up's board scan take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found.
basis: fct-ykd4mawd, fct-vdfhytdg
because: viola up takes the board's strict scan before the caller's cli log opens, while its later home-tree check runs after that log and writes a process-exit, so only the scan joins the row.
@row_set n=2 row=contract/strict-modes-refusal/no-line cell=promise seen=2 detector=none
value: A refused check at run, viola revive, viola verify, viola focus, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.
basis: fct-m4vsf7nm, fct-vdfhytdg
because: viola up's board-scan refusal exits 21 with no line, while its later home-tree refusal writes a process-exit, so only the scan joins the no-line list.
@row_add n=1 row=decision/stand-in-terminal-tree-teardown area=quality-gates-coverage-targets fact=F7 detector=none
name: One shared teardown ends a stand-in terminal's tree
rule: A stand-in terminal's process tree is ended by one shared test teardown: it kills first only a leaf whose recorded parent still runs, waits for every other member under one deadline, and at that deadline kills only a pid whose start time still matches.
why: Killing a fake agent already in its own exit left cut-short coverage profiles that turned pre-push red.
state: provisional
basis: fct-ykd4mawd, fct-z7bnkfyd, fct-z4vd2t3l
because: The chunk replaces each terminal guard's own kill loop with one shared teardown, a fold the overseer decided as delegate, so the decision lands provisional.
@row_add n=2 row=decision/child-fd-table-read-after-launch-settles area=e2e-test-strategy fact=F8 detector=none
name: Child fd guard reads the table once the launch settles
rule: The child fd guard reads the child's fd table again, inside its existing deadline, while a whole table holds a non-fixture path under the home, and asserts the last whole table unchanged.
why: The case read the fake agent's launch-time open of the plugin hooks file as a hold.
guard: tui_channel_fds_the_child_holds_nothing_of_violas
state: provisional
basis: fct-ykd4mawd, fct-q3azldqy, fct-sh2mcdex
because: The chunk closes the fd case's red by a settle wait, a fold the overseer decided as delegate, so the decision lands provisional.
@row_add n=3 row=decision/live-rig-root-outside-repositories area=e2e-test-strategy detector=none
name: Live rig root outside every repository
rule: A live rig's live run refuses a root inside a repository, and runs its stand-in agent from a target dir of its own.
state: provisional
basis: fct-vdfhytdg
because: The plan's amendment to decision/live-rig-clean-start is carried on the overseer's P5 review, a delegate's word, and the row cannot take it within its bound.
