@answer n=1 entity=obs-scope-summary#22.2 v=no
why: frontend instrumentation hooks are N/A, and the 0.2.0 frontend is out of this chunk
@answer n=2 entity=obs-scope-summary#22.3 v=no
why: the mcp process's file sink and stdout rules cover a code path this chunk does not touch
@answer n=3 entity=obs-scope-summary#22.4 v=no
why: service identity of the mcp process; no mcp code is reached by the chunk
@answer n=4 entity=obs-scope-summary#22.5 v=yes
why: its closed instance-unreachable cause list names socket-dir-refused, the code item 14 ratifies or re-words
@answer n=5 entity=obs-scope-summary#23.1 v=yes
why: the cargo deny ban on Tokio and C-building crates binds the portable-pty bump, and the init order emits the process-start the live run checks
@answer n=6 entity=obs-scope-summary#24.1 v=yes
why: the tracing-subscriber 0.3.20 version floor binds the Cargo.lock the portable-pty bump moves, and the one-write-per-line sink rule covers the live run's log
@answer n=7 entity=obs-scope-summary#25.0 v=yes
why: the required fields and the closed event enum are what the live run and the dialog read check their evidence against
@answer n=8 entity=obs-scope-summary#26.1 v=yes
why: the live run logs to diagnostics/run-<name>.ndjson, so the file location and the home-level rule bound where that evidence is read
@answer n=9 entity=obs-scope-summary#27.1 v=yes
why: the surfaces of the run path that the live viola run (item 10) exercises
@answer n=10 entity=obs-scope-summary#27.2 v=yes
why: the run path's must-trace spans and the wheel, budget-gate, session-start order are what the live run's evidence must show
@answer n=11 entity=obs-scope-summary#27.3 v=yes
why: run's required log fields and exit-1 detail codes, socket-dir-refused included, are read from the live run and bound by item 14's wording
@answer n=12 entity=obs-scope-summary#27.4 v=no
why: source citation only; the rules it cites are entities 27.1 to 27.3
@answer n=13 entity=obs-scope-summary#28.1 v=yes
why: the overseer driving a builder (item 10) runs this send path
@answer n=14 entity=obs-scope-summary#28.2 v=yes
why: the send's must-trace spans and log events are the overseer's live-session evidence
@answer n=15 entity=obs-scope-summary#28.3 v=yes
why: the send's required fields and refusal details are read from the overseer's live session
@answer n=16 entity=obs-scope-summary#28.4 v=no
why: source citation only; the rules it cites are entities 28.1 to 28.3
@answer n=17 entity=obs-scope-summary#29.1 v=no
why: no wait call is named by the chunk's live sessions or its edits, so the wait surfaces are not reached
@answer n=18 entity=obs-scope-summary#29.2 v=no
why: the wait path's spans and channel events are not exercised by any named live session or edit
@answer n=19 entity=obs-scope-summary#29.3 v=no
why: wait's required fields and exit 21 are not named by the chunk's live sessions; no wait call is in scope
@answer n=20 entity=obs-scope-summary#29.4 v=no
why: source citation only; the rules it cites are entities 29.1 to 29.3
@answer n=21 entity=obs-scope-summary#30.1 v=yes
why: the dialog path is the one item 12 reads live in the dialog to answer scenario
@answer n=22 entity=obs-scope-summary#30.2 v=yes
why: the dialog spans and events (hook-invoked, dialog-raised, dialog-answered, hook-decision) are the item 12 read's evidence
@answer n=23 entity=obs-scope-summary#30.3 v=yes
why: the dialog's required fields, refusal codes and no question, answer or input text rule are what the item 12 live read checks
@answer n=24 entity=obs-scope-summary#30.4 v=no
why: source citation only; the rules it cites are entities 30.1 to 30.3
@answer n=25 entity=obs-scope-summary#31.1 v=no
why: the wheel and release surfaces are named by no live session or edit of the chunk
