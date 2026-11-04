@answer n=1 entity=unit-test-strategy#1.1 v=no
why: the viola-core unit claims live in crates/viola-core/src, which no file the session moved, and Section 4 is untouched
@answer n=2 entity=unit-test-strategy#1.2 v=no
why: the viola-channel unit claims are inline cases in crates/viola-channel/src, which the session did not move, and Section 4 is untouched
@answer n=3 entity=unit-test-strategy#1.3 v=no
why: the viola-state unit cases are inline in crates/viola-state/src, which the session did not move, and Section 4 is untouched
@answer n=4 entity=unit-test-strategy#1.4 v=no
why: the viola-agent-claude unit cases are inline in crates/viola-agent-claude/src, which the session did not move, and Section 4 is untouched
@answer n=5 entity=unit-test-strategy#1.5 v=no
why: the viola-mcp stdin and serve cases are inline in crates/viola-mcp/src, which the session did not move
@answer n=6 entity=unit-test-strategy#2.1 v=no
why: the root-bin unit cases are inline in src/ modules, which the session did not move, and the one contract binary it cites, tests/contract_bounded_readers.rs, is not a list the new census member changes
