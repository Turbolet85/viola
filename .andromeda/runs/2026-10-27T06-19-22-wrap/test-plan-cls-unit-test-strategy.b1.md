# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · no · no code under test in viola-channel changed and no reached file is in viola-channel, so the codec, error mapping, endpoint vectors and peer checks hold as written
unit-test-strategy#1.2 · yes · add-to · the session bounded replace_private_shared's fallback compare in fs.rs, so the viola-state fs-helper list gains that case (F5) beside the place_private_new cases
unit-test-strategy#1.3 · no · the vt100 feed's catch_unwind and forced-panic-to-input-not-ready claim still holds on the changed chaos file, which only adds a passthrough assertion beside it, and no viola-agent-claude code under test changed
unit-test-strategy#2.1 · no · the root-bin unit cases are unchanged, the mcp stdin cut exits on the existing codes 0 and 11 with no new code or variant, and the reached chaos and contract files are integration tests outside this list
