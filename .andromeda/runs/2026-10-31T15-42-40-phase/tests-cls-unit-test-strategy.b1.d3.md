# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · is_code_word's 1–64 byte, lower-case-first rule binds the new closed codes, since a word that fails it reads as unknown or null (Needs, reply screens)
unit-test-strategy#1.2 · yes · the -32603 internal-error rule (exact message, data null) and sanitised_fault bind the new internal-error code and peer faults in crates/viola-channel/src/client.rs
unit-test-strategy#1.3 · yes · classify's 5 s staleness, server_verdict, live_endpoint and check_instance bind the stale-heartbeat, server-verification and strict-modes causes read by the liveness pre-check, the server check and refuse_untrusted
unit-test-strategy#1.4 · yes · resolve_program's BatchScriptChild and ProgramNotFound refusals and their fixed messages bind the script-child and claude-not-on-PATH causes; the dialog, prompt and R8 strip rules do not
unit-test-strategy#1.5 · yes · of_reply's reply screen (a non-code-word refusal reads unknown, a non-code-word detail reads null, a peer fault reads through sanitised_fault) binds the MCP refusal surface in crates/viola-mcp/src/result.rs
unit-test-strategy#2.1 · yes · the exit-code mapping to the typed set 0/1/2/10–14/20/21, reply_of, the server check's refusal and unreachable_logs_its_cause bind the new codes and causes in src/cmd/client.rs and the verbs it guards
