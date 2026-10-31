# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the paste set and empty-text rule live in viola-core and src/cmd/send.rs, which the session changes only for UnreachableCause and exit-21 reporting; CI green on 8a4c1a2c holds the claim
input-validation#1.2 · no · the free-text set and the exit-13 check for answer live in viola-core and src/cmd/answer.rs, changed only for exit-21 reporting; CI green on 8a4c1a2c holds the claim
input-validation#1.3 · no · reply_of and of_reply still take refusal and detail only as code words, every new word passes is_code_word, and the channel frame rules are wrapper-side and unreached
input-validation#1.4 · no · hook.rs is not in the diff and the hook stdin rules live there and in unreached viola-state, so exit 0, empty stderr and the size caps stand
input-validation#1.5 · no · MCP input failures (invalid-params with its field word, no frame) are unchanged, the new frame mappings act only after a passed input, and JSON-RPC errors did not move
input-validation#1.6 · no · the line cap lives in the unreached viola-mcp stdin.rs, and mcp stays silent at the catch site with no exit code added, so the exit-1 and empty-stream claim holds
input-validation#1.7 · yes · contradict · revive --json now prints a document for every start refusal and exit 21's --json and MCP detail carry a cause word, so the null detail and the no-document sentence are false (F10 extends the code list)
input-validation#1.8 · no · the config reads and thresholds live in unreached src/run/budget.rs, src/run/dialog.rs and obs, and the report moves no config rule
input-validation#1.9 · no · the registry name reads live in unreached src/run/env.rs, and the report moves no R8 rule
input-validation#1.10 · no · the fake-agent seam's u64 parse, 5 000 cap and feature gate are not among the report's changes, and CI green holds on the changed run.rs
input-validation#1.11 · no · the seam's panic and fail-open rules live in unreached src/cmd/hook/seam.rs, and hook.rs is not in the diff
input-validation#1.12 · no · the colour decision lives in unreached src/human/style.rs, and the report moves no list output rule
input-validation#1.13 · yes · stale · the stale-holder verdict moved to stale-heartbeat (F4) and without_endpoint is a new reading where live_endpoint gives no endpoint, so the replayed-snapshot liveness reading in liveness.rs and run's collision check moved
input-validation#1.14 · no · notify tailing lives in unreached viola-state tail code, and no events.ndjson rule moved
input-validation#1.15 · no · the agents parse, the --version probe and revive's preflight fail-open live in unreached viola-agent-claude, and the report's verify change adds recorded codes without moving the probe read
input-validation#1.16 · no · the vt100 feed and gate rules live in unreached src/run/gate.rs and viola-agent-claude's screen model
input-validation#1.17 · yes · add-to · F10 and F11 belong here: under revive --json batch-script-child, pinned-hash-mismatch and squatted-name print a document, cwd-missing stands in the code list, and an anyhow error from checked_exe prints internal-error
input-validation#1.18 · yes · add-to · F10 belongs here: no-session stands in revive --json's closed code list and now prints a document for its refusal
input-validation#1.19 · no · the statusline source read and the default-home rule live in src/cmd/run.rs and src/cmd/mod.rs functions the report does not change, and CI green holds on the changed tree
