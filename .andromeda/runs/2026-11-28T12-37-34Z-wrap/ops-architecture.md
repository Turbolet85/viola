@row_set n=1 row=decision/agents-join-read-ungated cell=rule seen=1 detector=none
value: viola list, viola revive and viola console do not gate their claude agents --json read on a ledger stamp.
basis: fct-o33yqef7
because: viola console now runs the same agents read on its own thread, ungated, so the row that names the ungated readers must name the console too.
@row_set n=2 row=decision/refused-check-writes-no-line cell=except seen=1 detector=none
value: A home that fails the strict-modes check while viola console runs, after its log is open, ends the console with one process-exit line of detail strict-modes-failed.
basis: fct-aspdcuf3
because: the console's mid-run refusal of a widened home writes one process-exit strict-modes-failed line, so the no-line rule now has a named exception.
@row_set n=3 row=decision/strict-modes-check-before-log cell=except seen=1 detector=none
value: A home that fails the strict-modes check while viola console runs, after its log is open, ends the console with exit 1, the home pair and one process-exit line of detail strict-modes-failed.
basis: fct-aspdcuf3
because: the console re-checks the home on every board read after its log is open, and its mid-run refusal writes one line, which the rule's no-line clause must name as its exception.
@row_add n=1 row=decision/console-rereads-board-from-disk area=standard-contracts fact=F1 detector=none
name: Fleet console re-reads the board from disk
rule: The fleet console re-reads the board from disk once a second through the board's strict-modes scan, contacting no wrapper, replays no log whose file stamp is unchanged, and redraws and logs its console-frame line only when its view or size changes.
basis: fct-vecccir6
because: the chunk lands the console's board source and its redraw-on-change cadence, a contract future readers of the console's lines and disk reads must respect.
@row_add n=2 row=decision/console-agents-read-off-loop area=standard-contracts fact=F2 detector=none
name: Fleet console reads claude agents off its key loop
rule: The fleet console reads claude agents --json on its own thread at most every 5 s, off the key loop; a failed read hands an empty list, so every STATUS reads unknown and no unwrapped row is shown.
why: Keeping the last good list would show a stale busy.
basis: fct-klybeaot
because: the chunk adds the console's agents thread and chooses an empty list over the last good one on a failed read, a decision with its stated reason.
@row_add n=3 row=decision/board-last-activity-never-serialised area=project-directory-structure fact=F3 detector=none
name: Board last activity held in the read model only
rule: The board item carries a last-activity time, the newest whole log line time that parses as RFC 3339, absent for an unwrapped row or an empty log, and never serialised, so no list --json item and no MCP list item carries it.
guard: board_read_item_never_serialises_last_activity
basis: fct-xjbaemp6
because: the board read model gains a field the console draws while the list payload stays unchanged, a boundary future work must keep.
@row_add n=4 row=contract/console-end/home-widened-mid-run area=standard-contracts fact=F4 detector=none
boundary: viola console end
item: home widened mid-run
promise: A home widened while viola console runs ends it, after the terminal is restored, with exit 1, the home unable and hint pair and one process-exit line, exit_code 1, detail strict-modes-failed.
basis: fct-aspdcuf3
because: the chunk lands the console's mid-run refusal, an exit and a line other parts read.
@row_add n=5 row=contract/console-end/scan-error area=standard-contracts fact=F4 detector=none
boundary: viola console end
item: scan error
promise: A board scan that errs while viola console runs, not a refusal, ends it after the terminal is restored as an internal error.
basis: fct-fv2cyr6d
because: the chunk routes an erring board scan to the internal-error catch site after the restore, an outcome distinct from the refusal.
@row_none n=1 fact=F5 type=4
relied: the console view's shape, the board's rows in forest order with the six captions and a seventh LAST cell, its cut and its more line, is the layout and design masters' surface; the code holds how it is drawn.
