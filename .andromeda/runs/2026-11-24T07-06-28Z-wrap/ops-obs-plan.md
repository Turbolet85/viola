@row_set n=1 row=contract/diag-subject/terminal cell=promise seen=1 detector=D-obs-instrumentation
value: Subject of the one process-start, with no process-exit, of each foot terminal window that viola open or viola up spawns, written into that verb's cli role file; the terminal outlives the verb.
basis: fct-vk27pydd
because: The claimed process-start / process-exit pair is disproved by measurement and ruled corrected to the code: open and up write the terminal's process-start alone, so the promise must drop the exit and name up.
@row_set n=2 row=contract/process-log/child-and-probe-lifecycle cell=promise seen=3 detector=D-obs-instrumentation
value: process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes and for verify's print-mode sender; the terminal viola open or viola up spawns gets a process-start alone.
basis: fct-ywkv2jdx
because: The terminal pair is corrected to a process-start alone (the terminal outlives the verb), so the row's claim of a process-exit for the terminal is false.
@row_set n=3 row=decision/strict-modes-failure-lines cell=rule seen=1 detector=D-obs-instrumentation
value: A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate, on `verify`'s own `process-exit` for a refused stamp write, and on `viola up`'s own `process-exit` for a home tree refused after its board scan.
basis: fct-uiucoyys
because: viola up's home-tree refusal after its scan writes its own strict-modes-failed process-exit word, a third place the row's «only» leaves out.
@row_set n=4 row=contract/diag-detail/strict-modes-failed cell=promise seen=1 detector=D-obs-instrumentation
value: Written on the process-exit exit 1 of viola verify and of viola up under a parsed VIOLA_NAME, and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
basis: fct-uiucoyys
because: viola up now writes strict-modes-failed on its exit-1 process-exit, which the promise's closed list of writers must name.
@row_set n=5 row=contract/process-exit-detail/strict-modes-failed cell=promise seen=1 detector=D-obs-instrumentation
value: Carried by `verify`'s `process-exit` for a refused stamp write, and by `viola up`'s `process-exit` for a home tree refused after its board scan, each with exit 1.
basis: fct-uiucoyys
because: viola up's home-tree refusal is a second carrier of the strict-modes-failed process-exit detail.
@row_set n=6 row=contract/strict-modes-refusal-exit/21 cell=promise seen=2 detector=none
value: The six frame verbs, link / unlink, focus and up's board scan exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.
basis: fct-ykd4mawd
because: viola up exits 21 with the instance-unreachable / strict-modes-failed document on its board's strict refusal, a verb the closed list lacks.
@row_set n=7 row=contract/strict-modes-refusal-exit/1 cell=promise seen=2 detector=none
value: run, revive's start, verify, open and up's home-tree check exit 1 on a strict-modes refusal; revive, open and up under --json print the strict-modes-failed document.
basis: fct-ykd4mawd
because: viola up exits 1 with a strict-modes-failed document on its home-tree check before the first spawn, a verb the closed list lacks.
@row_set n=8 row=decision/strict-modes-before-diagnostics cell=except seen=1 detector=D-obs-instrumentation
value: viola up's home-tree check before its first spawn runs after its board scan has opened its log, and a refusal there ends that log with process-exit detail strict-modes-failed.
basis: fct-uiucoyys
because: The rule says a refused home gets no diagnostics line, but viola up's later home-tree refusal writes one, so the exception must be named.
@row_set n=9 row=decision/symlink-skip-logged-as-count cell=except seen=1 fact=F4 detector=D-obs-instrumentation
value: viola up, which never takes the strict board read, writes no parse-rejected state-entry line.
basis: fct-sn4h5u7b
because: viola up scans the board without the strict board read and so writes no parse-rejected state-entry line, an exception to the once-per-scan count.
@row_add n=1 row=contract/cli-log-file/up area=span-trace-coverage fact=F3 detector=D-obs-instrumentation
boundary: CLI process-log file
item: viola up
promise: Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson only under a parsed VIOLA_NAME, after its strict board scan, inside the cli.up dispatch span: its own process-start and process-exit, one process-start{subject:"terminal"} per spawn and state-recovered; a refused scan writes no line.
basis: fct-ykd4mawd
because: The new verb viola up has a log file contract of its own, as each other cli verb does, and the decision row listing the verbs cannot take it within its bound.
@row_add n=2 row=contract/process-exit-detail/exit-1-up area=6-log-coverage fact=F2 detector=D-obs-instrumentation
boundary: process-exit detail
item: exit 1 from viola up
promise: viola up ends exit 1 with process-exit detail not-live when any instance reads not-live, else with no detail (an exit 1 from no-session, cwd-missing, session-live or not-taken alone); its refusals each write their own detail word.
basis: fct-uiucoyys
because: viola up brings an exit 1 whose detail follows its per-instance outcomes, which neither the exit-1 detail row nor the one-code-per-cause rule says.
@row_add n=3 row=contract/process-exit-detail/boot-time-unread area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: process-exit detail
item: boot-time-unread
promise: Written by viola up on its process-exit with exit 1 when the host boot time cannot be read; the refusal has its own unable and hint pair.
basis: fct-xgdyo5mg
because: The process-exit detail enum gains boot-time-unread, and the exit-1 detail row cannot take it within its bound.
@row_none n=1 fact=F5 type=5
what: the cli.up span name is one verb's entry in the code; decision/cli-manual-dispatch-span already rules one dispatch span per cli verb, and contract/cli-log-file/up names it.
@row_none n=2 fact=F6 type=4
relied: a revive start under the hidden --origin writes the existing run.window span and window line; without the flag there is no window step and no window line.
@row_none n=3 fact=F7 type=4
relied: the host boot-time read, the beat-before-boot check and the ended-after-last-start read write no line and open no span of their own.
@row_none n=4 fact=F8 type=4
relied: viola up writes no snapshot, event line or stamp; its lines stand in contract/cli-log-file/up.
