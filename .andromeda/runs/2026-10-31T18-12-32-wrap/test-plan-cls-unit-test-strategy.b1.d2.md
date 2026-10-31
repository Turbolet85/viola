# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · add-to · viola-core gains `UnreachableCause` and its five code words, which stand beside the code-word table (F18)
unit-test-strategy#1.2 · no · viola-channel is not reached, `sanitised_fault` and the JSON-RPC mapping are unmoved, and `src/human/mirror.rs` is not reached
unit-test-strategy#1.3 · yes · add-to · `viola_state::liveness` gains `without_endpoint`, reading a directory as `instance-dead` and all else as `unwrapped-name` (F19), beside the liveness rules
unit-test-strategy#1.4 · yes · stale · `tests/cli_verify.rs` is reached and holds these AgentError outputs; `verify` now logs `batch-script-child` and `program-not-found` on its process-exit line and the file gains cases
unit-test-strategy#1.5 · yes · stale · `crates/viola-mcp/src/result.rs` is reached (its `unreachable` writer now emits each cause word) and `of_reply` lives there, with no report line showing its claim holds on the changed file
unit-test-strategy#2.1 · yes · add-to · the root bin unit list gains `json_asked`'s argv-before-separator rule (F20) and the `unwrapped-name` and `endpoint-lost` hints in `src/human/hints.rs` (F21)
