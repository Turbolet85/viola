# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · add-to · F18 joins the viola-core code-word cases: `UnreachableCause` and `unreachable_cause_is_each_its_own_code_word` sit in the reached `crates/viola-core/src/lib.rs`
unit-test-strategy#1.2 · no · viola-channel is not in the change ("nothing in viola-channel moved"), so its codec, -32602 mapping, sanitised_fault and peer checks hold
unit-test-strategy#1.3 · yes · add-to · F19 joins the viola-state liveness readings: `without_endpoint` in the reached `crates/viola-state/src/liveness.rs` opens nothing and writes no line
unit-test-strategy#1.4 · no · viola-agent-claude is not in the reached set and the change moves none of its rules (AgentError messages, resolve_program, hook, version gate); the send and verify files hold only its callers and outputs
unit-test-strategy#1.5 · yes · stale · result.rs is reached whole and `unreachable` now writes each cause word into MCP's isError body; `of_reply`'s reply-screen readings sit in that file and the report does not show them unmoved
unit-test-strategy#2.1 · yes · add-to · F20 and F21 join the root bin unit cases (`json_asked` in `src/main.rs`, the unwrapped-name and endpoint-lost hints in `src/human/hints.rs`), and the closed Unreachable causes it names have grown
