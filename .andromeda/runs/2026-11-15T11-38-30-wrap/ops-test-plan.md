@row_set n=1 row=contract/event-log/start-wheel-record cell=promise seen=2 fact=F2 detector=none
value: Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
basis: fct-k3ry5p3p, fct-rdvnj325
because: The live start measured trust keys passing through the wrapper writing a human-input wheel line before session-start, so session-start is not always the start's second record and the row must say so.
@row_add n=1 row=decision/start-lines-read-from-start-offset area=6-e2e-test-strategy fact=F3 detector=none
name: A start's lines read from its own offset
rule: A reader of a start's first event lines reads that start's own lines, from the offset where the start began, never line 1 of events.ndjson, since a reused instance name keeps the earlier starts' lines at the head of the log.
why: Cursors are byte offsets and the event log is never rewritten.
basis: fct-jyydc3j4
because: A reader of a reused instance name's log read the earlier start's lines from line 1, so the convention for reading a start's own lines must stand as a row.
@row_none n=1 fact=F1 type=9
what: The fake agent keeping a typed DEL where the real CLI erases the box is left unchanged and owned by a carry on the route.
