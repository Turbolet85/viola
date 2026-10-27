# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · the frame codec at MAX_FRAME − 1, MAX_FRAME and MAX_FRAME + 1 and the -32600 mapping of an over-size frame (the IPC coverage trigger)
unit-test-strategy#1.2 · yes · the events.ndjson reader's torn_lines count of an over-long line and its never-panic skip counts (the filesystem coverage trigger), and the classified snapshot.json read's failure rule that an oversize file takes under the size cap
unit-test-strategy#1.3 · yes · the hook payload's unknown-field count, the question option_details and Stop background_tasks bounds, the agents parser's oversize-or-malformed reads as unknown, the typed_text rule (paste-injection trigger), and the vt100 feed under catch_unwind degrading to input-not-ready
unit-test-strategy#2.1 · yes · the root bin's send and answer refusal ordering with control-character first and empty-text next, the RefusalReason #[serde(other)] unknown to exit 14 mapping, and read_hold's bounded dialog_hold_s
