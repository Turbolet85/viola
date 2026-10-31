# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · yes · stale · viola-core/lib.rs and src/cmd/send.rs are reached whole (UnreachableCause added there, send's frame refusal path changed) and the report does not clear the paste-text rule on them
input-validation#1.2 · yes · stale · viola-core/lib.rs and src/cmd/answer.rs are reached whole (answer's frame refusal path changed) and the report does not clear the free-text rule on them
input-validation#1.3 · no · the reply code-word rule holds on the changed client.rs and result.rs: every new word passes viola_core::is_code_word, and viola-channel and the -32602 log refusals did not move
input-validation#1.4 · no · src/cmd/hook.rs is not in the diff, so the hook stdin cap, the event mapping and the instance check stand (Untouched: hook fail-open)
input-validation#1.5 · yes · stale · tools.rs and result.rs are reached whole, the MCP frame tools' refusal path in them changed to cause-bearing bodies, and the report does not clear the invalid-params shape or the argument checks there
input-validation#1.6 · no · the line cap lives in the unreached viola-mcp stdin.rs, and src/main.rs exit_code gains no exit or exit code (the report says none was added)
input-validation#1.7 · yes · contradict · the exit-21 document's detail is now a cause word, not null, and revive --json no longer leaves the post-preflight gap; F10 (closed revive --json codes, one document for every start refusal) and F11 (internal-error for an anyhow error from the start) join this list
input-validation#1.8 · yes · stale · viola-core/lib.rs, which holds Percent and DialogHold, is reached whole and the report names no change that clears those types
input-validation#1.9 · no · the Windows registry name read lives in the unreached src/run/env.rs and nothing in the session names it
input-validation#1.10 · yes · stale · src/cmd/run.rs, where hold_pump_start lives, is reached whole and the report does not clear the seam's parse and cap
input-validation#1.11 · no · the panic seam lives in the unreached src/cmd/hook/seam.rs; the new Cargo.toml [[test]] gates a test binary and moves no seam claim
input-validation#1.12 · no · the colour decision lives in the unreached src/human/style.rs, and src/cmd/list.rs only calls it
input-validation#1.13 · no · the snapshot, events, stamps, budget and heartbeat readers are unreached, and liveness.rs's without_endpoint opens nothing (F7 is an authentication fact), so the six callers and the read bounds hold
input-validation#1.14 · no · the notify tailing code is in the unreached viola-state tailing and no change names it
input-validation#1.15 · no · the agents parse and the 5 s kill live in unreached viola-agent-claude and run; revive's fail-open on a failed agents read is neither a refusal nor an anyhow error (F11), so it holds
input-validation#1.16 · no · the vt100 feed and the gate tee live in unreached src/run/gate.rs and viola-agent-claude, and nothing in the session names them
input-validation#1.17 · yes · add-to · under revive --json batch-script-child and cwd-missing now print their document (F10) and a program not found prints internal-error (F11); the human pairs stand
input-validation#1.18 · yes · add-to · no-session stands in revive's closed --json code list (F10), so this refusal before any spawn now prints its document under --json
input-validation#1.19 · yes · stale · src/cmd/run.rs and src/cmd/mod.rs are reached whole, the statusline-source read is in them, and the report does not clear it
