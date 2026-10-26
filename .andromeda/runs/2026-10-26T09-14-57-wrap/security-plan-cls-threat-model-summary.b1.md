# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the wait and last results still carry the same event lines verbatim on the changed files once the server check passes
threat-model-summary#1.2 · no · the dialog payloads still go to events.ndjson and to hook.dialog on the changed hook files once the server is verified
threat-model-summary#1.3 · no · the snapshot's stored fields and the code-bearing and stamp files still hold, since the check only reads pid and started_at
threat-model-summary#1.4 · no · the R8 strip and secret management are not reached, and no env var was added or changed
threat-model-summary#1.5 · no · the check writes and logs no pid or start time, and the new refusal lines carry no pid, start time, path or endpoint
threat-model-summary#2.1 · yes · contradict · the clients now verify the server's pid and start time before any frame on Linux and Windows, so acting on whatever answers the endpoint no longer holds there (macOS keeps the gap)
threat-model-summary#3.1 · no · the exit-13 refusals stay first (F3), so empty text is still refused and nothing is written on the changed send file
threat-model-summary#4.1 · no · the stdin classifier and the wheel are outside the reached files, and pause and release gain only a pre-check before their frame
threat-model-summary#5.1 · no · a failed hook server check still fails open with exit 0 and no body (F4), and the statusline arm is outside the reached files
threat-model-summary#6.1 · no · a failed frame-tool check returns the existing instance-unreachable word (F5), so the tool list and input checks still hold
threat-model-summary#7.1 · no · the server check only adds an exit-21 refusal before the frame, leaving the subcommand inputs and home flags as described, and the entity names no client-side impersonation
threat-model-summary#8.1 · no · the client readers stay on read_snapshot (F8), so the two replay readers and the home trust boundary still hold
threat-model-summary#9.1 · no · the start-time read is a process-table read on spawn_blocking, not a child spawn, so the spawn and PATH claims hold on the changed tools file
threat-model-summary#10.1 · no · no dependency was added or bumped, so the cargo-deny, tokio-ban and Cargo.lock boundary holds
threat-model-summary#11.1 · no · no CI or toolchain pin changed and the new negatives are cargo tests that compile out on macOS
