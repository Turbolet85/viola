@row_set n=1 row=contract/strict-modes-refusal/console cell=promise seen=1 detector=none
value: Exit 1 with the fixed pair unable: viola console could not use its home, then hint: make the viola home readable by you alone, then run viola console again, on stderr; at start with no line written and the mode left as found, mid-run with one process-exit line of detail strict-modes-failed.
basis: fct-tol6uebc
because: A home widened mid-run now ends viola console with exit 1, the home pair and one process-exit strict-modes-failed line, so the contract's «no line written» holds only for the refusal at start.
@row_set n=2 row=decision/claude-agents-resolved-from-path cell=rule seen=2 detector=D-security-input
value: list, revive's preflight, adopt's live-elsewhere check, the MCP list tool and viola console resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME.
basis: fct-o33yqef7
because: viola console now runs the claude agents read on its own thread, resolving claude on PATH and bounded by MAX_FRAME, so the row's list of callers must name the console.
@row_set n=3 row=decision/claude-agents-read-fail-open cell=rule seen=1 detector=D-security-input
value: The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged, passes revive and hands viola console an empty list, its status unknown.
basis: fct-klybeaot
because: In viola console a failed agents read hands an empty list so STATUS reads unknown rather than a stale busy, a fail-open outcome the row does not yet state.
@row_set n=4 row=decision/list-scan-checks-before-open cell=rule seen=1 fact=F1 detector=D-security-input
value: viola list, and viola console at each board re-read, check the home, instances and every instance, never following a link, before any file under the home is opened; viola console replays only a log whose stamp, taken without following a link, changed.
basis: fct-mhjm6wnn
because: The console's board source re-reads the board once a second behind the board scan's strict checks and replays only a changed log by a stamp that follows no link, so the row must name the console's re-reads.
@row_set n=5 row=decision/tests-use-own-home cell=rule seen=1 fact=F5 detector=none
value: Development handles secrets as production does, and tests use their own --home directory; viola console's tests run it with PATH set to an empty test-scoped bin dir, so none reaches a host claude.
basis: fct-ezqgpxkh
because: Every console test start now sets PATH to an empty test-scoped bin dir so the console's agents thread cannot read the host's real claude, a test rule the row must carry.
@row_set n=6 row=decision/escape-terminal-controls cell=rule seen=1 fact=F6 detector=none
value: wait/last human-mode output, list rows taken from claude agents --json and viola console's fleet cells are never printed to a terminal without escaping C0/C1 controls, and a fleet line too wide for the terminal is cut ending in dots, never inside an escape sequence.
basis: fct-aspdcuf3
because: viola console now prints the fleet to a terminal with every cell control-escaped and a too-wide line cut ending in dots never inside an escape, so the ban must cover that surface.
@row_add n=1 row=decision/board-last-activity-in-memory area=data-protection fact=F3 detector=none
name: Board last-activity time held in memory only
rule: The board item's last-activity time is held in memory only and never serialised: no list --json item and no MCP list item carries it.
guard: board_read_item_never_serialises_last_activity
basis: fct-xjbaemp6
because: The board item gains a last-activity time kept out of every serialised list item, a boundary promise on what list and MCP may carry.
@row_add n=2 row=decision/console-agents-child-outlives-quit area=data-protection fact=F4 detector=none
name: Console agents child may outlive a quit
rule: An agents child of viola console may outlive a quit of the console by up to its 5 s bound.
why: Accepted as built.
basis: fct-qezvmgl2
because: The delegate's word accepts as built that a claude agents child may run on after the console quits, an accepted residual future work must respect.
@row_add n=3 row=contract/console-board-read/scan-error area=error-handling fact=F2 detector=none
boundary: viola console's board re-read
item: a scan that errs
promise: A board read whose scan errs, not a refusal (an unlistable instances directory or a state error), restores the terminal and ends viola console at the cli catch site as an internal error.
basis: fct-fv2cyr6d
because: The console now ends on an erring board scan through the cli catch site as internal error after restoring the terminal, a promise at the console's exit no row states.
