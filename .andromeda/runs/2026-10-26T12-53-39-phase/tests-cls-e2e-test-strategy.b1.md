# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the link and unlink verbs, their --json documents and their human result lines are cli surface driven as this row states, with the HELP literal in tests/cli_output_plain.rs
e2e-test-strategy#1.2 · no · standing S5 · Env goes per child through Command::env, and .env_clear() re-adds LLVM_PROFILE_FILE, a convention the task lists
e2e-test-strategy#1.3 · no · the chunk adds no outer-PTY test, and viola run passthrough is used but not changed
e2e-test-strategy#1.4 · yes · tests/channel_links.rs is a wrapper-channel test (sync client, raw-stream name negative, schema-checked link and unlink results) over the real endpoint the scope names
e2e-test-strategy#1.5 · yes · the recorded-exchange replay's tools/list check of tool names in order is the witness that tools/list stays send, wait, last, answer, list
e2e-test-strategy#1.6 · no · the chunk adds no library-crate test (Reached files name no crate), and its unit cases sit in the root bin under §4
e2e-test-strategy#2.1 · no · the chunk changes no run start step, and Path 1 only boots a wrapper the chunk uses
e2e-test-strategy#2.2 · no · the start records, snapshot fields, list item and refusal exits of Path 1 are unchanged by link state
e2e-test-strategy#2.3 · no · the Path 1 one-flow test and its wall-time bound are as landed, and the chunk changes no start step
e2e-test-strategy#2.4 · no · standing S3 · started wrappers sit in a Drop guard, and each test gets a fresh tempfile home
e2e-test-strategy#3.1 · no · an as-landed record of the Path 2 tests, which the chunk does not change
e2e-test-strategy#3.2 · yes · step 2's send from VIOLA_NAME=overseer is the implicit-link trigger that the chunk keeps at exactly one link record
e2e-test-strategy#3.3 · yes · the record order after offset L is what the implicit link's single link record must keep, and its refused sends are the not-issued boundary
e2e-test-strategy#3.4 · no · the agent-run cleanup of a harness session the chunk does not start
e2e-test-strategy#4.1 · no · an as-landed record of the Path 3 wait and last tests, and the chunk changes no wait, last or wake set
e2e-test-strategy#4.2 · no · the wait and last procedure, which the chunk changes neither; its dead-instance expectation sits in the signal entity
e2e-test-strategy#4.3 · yes · step 7's CLI exit 21 instance-unreachable document with detail null is the contract the new verbs' liveness pre-check must also return
e2e-test-strategy#4.4 · no · the harness cleanup of a Path 3 session, which the chunk does not start
e2e-test-strategy#5.1 · no · an as-landed record of the Path 4 answer tests and the MCP answer step, which the chunk does not change
e2e-test-strategy#5.2 · yes · the answer steps are the taken answers the Path 4 implicit link must keep at one link record, with the unknown-id and wheel-held answers as the refused boundary
e2e-test-strategy#5.3 · yes · the unknown-dialog exit 13 and the wheel-held exit 10 are refused answers that an implicit link must not record, while the dialog-once claim is unaffected
e2e-test-strategy#5.4 · no · the harness cleanup of a Path 4 session, which the chunk does not start
e2e-test-strategy#6.1 · no · an as-landed record of the Path 5 wheel tests and the MCP send refusal, which the chunk does not change
e2e-test-strategy#6.2 · no · the Path 5 wheel procedure (take, release, pause, injected turn), and the chunk changes no wheel holder or release
e2e-test-strategy#6.3 · yes · driver-facing hints never contain release, which binds the exit-21 hints of the new verbs as the scope names
