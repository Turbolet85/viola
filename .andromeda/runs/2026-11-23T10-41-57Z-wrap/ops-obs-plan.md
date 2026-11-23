@row_set n=1 row=contract/run-start-events/order cell=promise seen=4 detector=none
value: A run start writes wheel{cause:"start"}, then one lineage line, then one launch line before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
basis: fct-w2kbdeoj
because: Start records per start moved from three to four, a launch line now written after the lineage line, so the promised order must name it.
@row_set n=2 row=decision/product-order-from-events-log cell=rule seen=4 detector=none
value: The start product order, wheel{cause:"start"} then lineage then launch before session-start{source:"hook"} with wheel{human-input} also before it where folder-trust keys pass through the wrapper, is verified from that start's own lines of the instance's events log, not duplicated into process logs.
basis: fct-w2kbdeoj
because: The start's product order now holds a launch line after lineage, so the order the events log is read for must include it.
@row_set n=3 row=decision/config-observed-at-load cell=rule seen=1 detector=none
value: Configuration is observed only at its parse at load time, logged as skipped-key counts and as one malformed line per key whose value is refused.
basis: fct-xb6c4igy
because: A bad default_model or default_effort value now writes one parse-rejected config-json malformed line per key, so configuration is no longer logged as skipped-key counts alone.
@row_add n=1 row=decision/launch-line-untraced area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Launch line has no trace of its own
rule: The launch line written at every run and revive start is log-only product audit data: it wakes no wait and opens no span, writes no process-log line and no terminal byte of its own.
state: provisional
basis: fct-objkygma
because: Every start now writes a launch line after the lineage line, and the chunk gives it no span, process-log line or terminal byte, a choice future work must respect as the lineage line's sibling.
@row_add n=2 row=decision/launch-pair-confined area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Model and effort confined to product surfaces
rule: A start's model and effort, two closed words, appear only in events.ndjson, the snapshot, list --json and the MCP list result, each re-parsed at the board read and dropped otherwise, never in a process-log line.
state: provisional
basis: fct-ydz7awsz
because: The launch pair now crosses into the events log, the snapshot and the list item, and the chunk keeps it out of every process log, a confinement rule beside the lineage names' one.
@row_add n=3 row=contract/config-json/default-model area=otel-sdk-init fact=F3 detector=none
boundary: config.json
item: default_model
promise: Key default_model takes the closed model-word shape, a lowercase letter then up to 63 of a-z, 0-9, dot or hyphen; a bad value takes the model half of DEFAULT_PAIR with one parse-rejected config-json malformed line, and no value disables a control.
basis: fct-xb6c4igy
because: config.json gains the default_model key with its closed shape and fallback, a file format promise the obs plan's config rows must carry.
@row_add n=4 row=contract/config-json/default-effort area=otel-sdk-init fact=F3 detector=none
boundary: config.json
item: default_effort
promise: Key default_effort takes one of the five effort words; a bad value takes the effort half of DEFAULT_PAIR with one parse-rejected config-json malformed line, and no value disables a control.
basis: fct-xb6c4igy
because: config.json gains the default_effort key with its closed set and fallback, a file format promise the obs plan's config rows must carry.
@row_add n=5 row=contract/event-stream/launch area=log-format-json-schema fact=F4 detector=none
boundary: the event stream
item: launch line
promise: A launch line carries data model and effort with source wrapper, is log-only and wakes no wait; the replay takes the last launch line whole, and a line with a non-string member reads no pair.
basis: fct-ydz7awsz
because: The events log gains a launch kind with its data keys and a last-line replay rule, a format other readers rely on.
@row_add n=6 row=contract/run-revive-usage/pair-after-dashes area=3-observability-harness-contract fact=F5 detector=none
boundary: viola run and viola revive usage
item: pair after the dashes
promise: viola run and viola revive refuse --model or --effort among the words after -- as a usage error, exit 2, with one fixed stderr line naming the two flags, echoing none of the caller's words and writing no diagnostics line.
basis: fct-ydz7awsz
because: Both start verbs gain a fixed usage refusal of a pair after the dashes, an output-stream promise of the harness contract.
@row_add n=7 row=decision/revive-launch-pair area=log-coverage fact=F6 detector=none
name: Revive hands the launch pair
rule: viola revive hands the resumed child each half of the launch pair from its override flag, else from the recorded pair while that half re-parses, else from the start's default, and records the pair in the revived start's own launch line.
state: provisional
basis: fct-ydz7awsz
because: Revive now chooses the resumed child's model and effort by a fixed precedence and records them, a decision future starts must respect.
