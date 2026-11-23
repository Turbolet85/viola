@row_set n=1 row=contract/run-start-events/order cell=promise seen=3 detector=none
value: A run start writes wheel{cause:"start"} and then one lineage line before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
basis: fct-sdz3kc3h
because: Every run and revive start now writes three records before the hook's session-start (wheel, lineage, session-start), so the start order the contract promises gains the lineage line.
@row_set n=2 row=decision/product-order-from-events-log cell=rule seen=3 detector=none
value: The start product order, wheel{cause:"start"} then lineage before session-start{source:"hook"} with wheel{human-input} also before it where folder-trust keys pass through the wrapper, is verified from that start's own lines of the instance's events log, not duplicated into process logs.
basis: fct-yurmbmhi
because: The start now writes the snapshot, the wheel start line and then one lineage line before the spawn, with no process-log line, so the product order read from the events log gains the lineage line.
@row_set n=3 row=decision/link-names-never-in-process-log cell=rule seen=1 detector=D-obs-pii
value: link and unlink driver and driven names appear only in events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, each pair kept only when both names parse, never in a process-log line; the channel pair logs the method, never the params.
basis: fct-yurmbmhi
because: The board read model now carries every listed instance's link pairs to the viola list board, list --json and the MCP list result, name-checked, so the row's «only in the link and unlink event lines» no longer holds.
@row_add n=1 row=decision/lineage-line-untraced area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Lineage line has no trace of its own
rule: The lineage line written at every run and revive start is log-only product audit data: it wakes no wait and writes no process-log line, span or terminal byte of its own.
state: provisional
basis: fct-yurmbmhi
because: The chunk adds a lineage events line at every start, log-only and outside the wait wake kinds, instrumented by rule with no line or span, and no row of the span coverage says so.
@row_add n=2 row=decision/lineage-names-confined area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Parent and role confined to product surfaces
rule: A start's parent and role names appear only in events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, each parent re-parsed as a viola name and each role as a code word and dropped otherwise, never in a process-log or detail line.
state: provisional
basis: fct-i5eugfgb
because: The chunk records a self-reported parent and role at every start and shows them on list and the MCP list tool, so the PII section must say where those names may reach and that no log line carries them.
@row_add n=3 row=decision/inherited-name-default-parent area=otel-sdk-init fact=F3 detector=none
name: Inherited VIOLA_NAME as default parent only
rule: viola run takes an inherited VIOLA_NAME only as its default parent, a self-reported claim that is no identity, overridden by --parent and dropped when it equals the instance's own name; the instance itself never comes from it.
basis: fct-yurmbmhi
because: lineage_of now reads the inherited VIOLA_NAME as the parent when no --parent is given, a second use beside the per-role instance source, whose row is already at its bound.
