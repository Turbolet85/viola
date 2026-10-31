# New groups — test-plan · unit-test-strategy

## Verdicts
- fanout-test-plan--cascade-g1.md 6 · distinct · no group names `sanitised_fault` or any of its four cases
- fanout-test-plan--cascade-g1.md 7 · distinct · line 10 maps the five known codes and states nothing of a code outside them or of the `wrapper fault` reading
- fanout-test-plan--cascade-g1.md 8 · distinct · line 10 gives the `-32602` mapping's data, but no group names `ProtocolError::cause` or its cases, or tells the three `-32602` faults apart
- fanout-test-plan--cascade-g1.md 10 · distinct · no group names `AgentError`, its two variants or `refusal_messages_are_fixed`
- fanout-test-plan--cascade-g1.md 11 · distinct · line 52 holds a line past the cap ending the session and a closed stdin as the initialize error, but not a line past the cap before `initialize`
- fanout-test-plan--cascade-g1.md 16 · distinct · line 59 holds an unknown refusal string at exit 14 with a `--json` document, but not the word staying visible in it and in `refused: <word>`, nor the closed list's screen
- fanout-test-plan--cascade-g1.md 17 · distinct · line 73 reads `read_hold` through `take(MAX_FRAME)` under other cases, and no group names these six cap-driving cases
- fanout-test-plan--cascade-new-unit-test-strategy.md 1 · distinct · the file holds no viola-core group and names neither `is_code_word` nor `code_word_literal_table`
