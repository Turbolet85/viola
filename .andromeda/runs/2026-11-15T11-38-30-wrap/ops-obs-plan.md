@row_set n=1 row=contract/run-start-events/order cell=promise seen=2 fact=F1 detector=none
value: A run start writes wheel{cause:"start"} before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
basis: fct-k3ry5p3p, fct-rdvnj325
because: The live round on 2.1.325 measured the trust keys writing wheel{human-input} before session-start and the later key adding no wheel line, so the start-order promise read alone no longer covers such a start.
@row_set n=2 row=decision/product-order-from-events-log cell=rule seen=2 detector=none
value: The start product order, wheel{cause:"start"} before session-start{source:"hook"} with wheel{human-input} also before it where folder-trust keys pass through the wrapper, is verified from that start's own lines of the instance's events log, not duplicated into process logs.
basis: fct-k3ry5p3p, fct-jyydc3j4, fct-rdvnj325
because: The report disproved reading the start order as the log's first two lines, both for a start whose trust keys pass through the wrapper and for a reused instance name, so the rule must name the order as it stands and read it from that start's own lines.
@row_set n=3 row=decision/events-stream-never-rotated cell=rule seen=1 fact=F2 detector=none
value: The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins.
basis: fct-jyydc3j4, fct-rdvnj325
because: The report states that a reused instance name keeps the earlier start's lines at the head of events.ndjson since cursors are byte offsets never rewritten, which the row's never-rotated rule says less.
