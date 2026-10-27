# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · the ndjson frame codec at MAX_FRAME − 1, MAX_FRAME and MAX_FRAME + 1 and the -32600 error for an over-size frame are the IPC coverage trigger's named witnesses
unit-test-strategy#1.2 · yes · the events.ndjson reader's over-long line counted in torn_lines and its never-panic rule, and the classified snapshot read's failure rule for a home file over its cap, are the filesystem coverage trigger and the file size caps
unit-test-strategy#1.3 · yes · the question option_details and Stop background_tasks bounds, the claude agents --json oversize and malformed reads as unknown, the statusline rate_limits parse, the typed_text rule, and the vt100 feed under catch_unwind giving input-not-ready on a forced panic (the re-read of screen.rs and gate.rs) all bind
unit-test-strategy#2.1 · yes · the RefusalReason unknown variant to exit 14, the send and answer refusal ordering with control-character first, the config.json Percent and DialogHold bounds with their defaults, and the typed_text confirmation matcher all bind
