# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · Cargo.toml gains only a `[[test]]` entry and the nextest change adds one operand to the ci kill line; cargo-nextest and rstest 0.27.0 are unmoved
integration-test-strategy#1.2 · no · the concurrent-append check and its shared diagnostics files keep their append behaviour; the session changes codes and documents, not appends
integration-test-strategy#1.3 · no · viola-fake-agent and its binding contract are unmoved, with no mode, argument, receipt or script format changed
integration-test-strategy#1.4 · no · viola-channel moved nothing, so the endpoint, the socket modes and the pipe DACL stand
integration-test-strategy#1.5 · no · the PTY spawn, resize and sideload paths are unchanged; run only passes json false to Launch for its human pairs
integration-test-strategy#1.6 · no · hook fail-open is untouched: src/cmd/hook.rs is not in the change and hooks still exit 0 with empty stderr
integration-test-strategy#2.1 · no · the placement witness in viola-state fs.rs is not reached; the crate's change is liveness.rs only
integration-test-strategy#3.0 · yes · add-to · F15 the unserved handlers row binds no endpoint and a Windows Serving has no Drop, F16 the empty_home is now viola-made, F17 Frozen is a SIGSTOP guard on Unix as well as Windows
integration-test-strategy#4.1 · yes · contradict · the impostor and widened-instance MCP frame tools in handlers.rs no longer read detail null: they carry server-verify-failed and strict-modes-failed (F14 holds the mapping)
integration-test-strategy#4.2 · yes · contradict · the MCP frame tools' detail null and the link and unlink --json documents with detail null on a widened instance now read strict-modes-failed
integration-test-strategy#5.1 · yes · add-to · F11 verify logs its refusal's code on process-exit beside interrupted, F12 a refused home tree writes no line, a failing row and the fixture's file name add no code, and verify takes no --json, F13 tests/cli_verify.rs gains the tampered-pin, window and no-claude cases
