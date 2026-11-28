@row_set n=1 row=contract/diag-line-fields/console-frame cell=promise seen=1 detector=D-obs-instrumentation
value: Logged on the first draw and then only when one of its fields changes, with cols, rows, shown and hidden (counts) and fits (boolean), all required; an unread terminal size logs 0x0, not fitting.
basis: fct-vg7fsgme, fct-tol6uebc
because: The console-frame arm gains the required shown and hidden counts and is logged on the first draw then only on a change of its five fields, so the once-per-draw, three-field promise is false.
@row_set n=2 row=contract/diag-subject/agents-probe cell=promise seen=2 detector=D-obs-instrumentation
value: Subject of the process-start and process-exit pair of the claude agents read, written by viola list, the session-live reading of viola revive and viola adopt, verify's Run B, the list tool of viola mcp and viola console's agents thread into their role files.
basis: fct-tol6uebc, fct-o33yqef7
because: viola console now runs the claude agents read on its own thread and writes the agents-probe pair into console.ndjson, so the row's list of writers must name it.
@row_set n=3 row=decision/symlink-skip-logged-as-count cell=except seen=2 detector=D-obs-instrumentation
value: viola up, which never takes the strict board read, writes no parse-rejected state-entry line; viola console writes the line only when the scan's counts change.
basis: fct-tol6uebc, fct-o33yqef7
because: The console writes parse-rejected state-entry on a change of the scan's counts rather than once per scan, an exception the row must carry.
@row_set n=4 row=contract/state-recovered/scanned cell=promise seen=1 detector=D-obs-instrumentation
value: From the board read, the line carries instance (the caller) and scanned (the scanned instance's name when it parses, absent otherwise); with no VIOLA_NAME no log is open and the line goes nowhere, bar viola console, which writes it into console.ndjson with no instance.
basis: fct-tol6uebc
because: The console now writes state-recovered per replay into its home-level console.ndjson with no VIOLA_NAME, so the line no longer always goes nowhere without a name.
@row_set n=5 row=contract/process-exit-detail/strict-modes-failed cell=promise seen=2 detector=D-obs-instrumentation
value: Carried by `verify`'s `process-exit` for a refused stamp write, by `viola up`'s `process-exit` for a home tree refused after its board scan, and by `viola console`'s `process-exit` for a board scan refused mid-run, each with exit 1.
basis: fct-tol6uebc, fct-aspdcuf3
because: A home widened mid-run now ends the console with one process-exit exit 1 detail strict-modes-failed, a third carrier of the detail.
@row_set n=6 row=contract/strict-modes-refusal-exit/1 cell=promise seen=4 detector=none
value: run, revive's start, adopt's start, verify, open, up's home-tree check and viola console's mid-run board scan exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.
basis: fct-aspdcuf3, fct-vecccir6
because: The console's board scan refused mid-run, a widened home, now exits 1 with the home pair, so the exit-1 list must name it.
@row_add n=1 row=decision/console-mid-run-strict-modes-refusal-logged area=1-obs-scope-summary detector=D-obs-instrumentation
name: Console mid-run strict-modes refusal logged
rule: A board scan of viola console refused mid-run, its home widened after the console's log opened, restores the terminal and ends console.ndjson with one process-exit exit 1 detail strict-modes-failed and the home pair on stderr.
basis: fct-tol6uebc, fct-aspdcuf3
because: The rows that list where a strict-modes failure leaves a line (decision/strict-modes-failure-lines, decision/strict-modes-before-diagnostics) cannot take the console's new case within their bounds, so it stands as a row of its own.
@row_add n=2 row=decision/console-agents-probe-volume area=log-coverage fact=F1 detector=D-obs-instrumentation
name: Console agents-probe log volume accepted
rule: viola console reads claude agents on its own thread at most every 5 s and writes an agents-probe pair on each read, up to 720 pairs an hour into console.ndjson; its other lines are written only on a real change.
why: The console's log volume is narrowed to these pairs and the lines of real changes, and is accepted as built.
state: provisional
basis: fct-qezvmgl2, fct-tol6uebc
because: The console's own log volume into a never-rotated home-level file is a decided trade, accepted on the delegate's technical word, that future work on the console must respect.
@row_add n=3 row=decision/console-agents-read-fails-empty area=obs-scope-summary fact=F2 detector=none
name: Console agents read fails to an empty list
rule: A failed or unparsable claude agents read in viola console hands the console an empty list, so STATUS reads unknown and the unwrapped rows drop; the last good list is never kept.
why: Keeping the last good list would show a stale busy.
basis: fct-klybeaot, fct-o33yqef7
because: The console's agents read fails open to an empty list by decision, which no row of the area says.
@row_none n=1 fact=F3 type=4
relied: A board read that errs mid-run in viola console restores the terminal and reaches the cli catch site as error: internal error, the dispatch-error outcome contract/panic-outcome-by-role/console already holds; the code holds how it is built.
