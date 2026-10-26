# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · §8 is untouched (no field, schema or scrub shape added) and the session's new lines land in events.ndjson and cli-<driven>.ndjson, none in diagnostics/
pii-scrubbing-compliance#1.2 · no · §8 is untouched (no scrub shape added) and the session reads no env and spawns nothing, so the R8 name-only log and the no-env-to-event rule hold
pii-scrubbing-compliance#1.3 · no · the session adds no signing or CI secret path, and the v1 runtime still holds no signing credential
pii-scrubbing-compliance#1.4 · no · §8 is untouched (no scrub shape added) and the new cli.link and cli.unlink spans are skip_all with names only in their frames
pii-scrubbing-compliance#1.5 · no · §8 is untouched (no scrub shape added) and the session adds no dialog, tool input or answer field
pii-scrubbing-compliance#1.6 · no · §8 is untouched (no scrub shape added) and the session writes no drift report
pii-scrubbing-compliance#1.7 · no · §4 is untouched (no statusline or budget code changed) and §8 is untouched, so the shell-out span and its detail-only rule stand
pii-scrubbing-compliance#1.8 · no · §8 is untouched and the new refusals print the fixed instance-state pair while the new lines carry names and codes only, so no absolute path reaches a home-level line
pii-scrubbing-compliance#1.9 · no · §8 is untouched and the new server-verify-failed and instance-unreachable outputs carry no endpoint or pinned_bin, which are never logged
pii-scrubbing-compliance#1.10 · no · §8 is untouched and the new refusal and exit lines carry no pid, child_pid or start time
pii-scrubbing-compliance#1.11 · no · §8 is untouched and the session reads and renders no claude agents --json row; its one human addition is the fixed instance-state pair
pii-scrubbing-compliance#1.12 · no · §8 is untouched and the new -32602 causes are closed codes with data null, so no new refusal-detail value reaches a log
pii-scrubbing-compliance#1.13 · no · §8 is untouched, the session adds no fixture content, and link and unlink are not MCP tools, so no MCP fixture changes
pii-scrubbing-compliance#2.1 · yes · stale · tests/security_negatives_server_identity.rs grows from 19 to 25 cases (eight verbs with two modes each, plus controls) and only its two request_refuses_an_unverified_server_inside_its_dispatch_span cases compile out on macOS, so the file-level macOS reading this entity states moves
