# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · its location, retention and volume still hold on the changed files, since the session adds a read check on Windows events.ndjson and an instance check before the wait and last frames, changing neither where the lines sit nor what they hold
threat-model-summary#1.2 · no · the logged and sent flow still holds for an instance that passes the check, since the session adds only a refusal ahead of the hook.dialog frame in hook.rs and dialog free-text validation is untouched
threat-model-summary#1.3 · no · what each file holds and who writes it still hold on the changed statusline.rs, stamps.rs and pin.rs, since the session adds checks ahead of their reads and moves no stored value
threat-model-summary#1.4 · no · the credential stays in process memory and the R8 strip is unchanged, as the session's untouched list records
threat-model-summary#1.5 · no · the operational metadata and the snapshot-only cwd rule still hold on the changed revive.rs and run.rs, whose new refusals carry fixed messages that name no path, pid or cwd
threat-model-summary#2.1 · yes · contradict · every frame-writing client now runs the strict-modes check before its snapshot read and first frame, so the sentence that the snapshot is read without it, owed to Home and code-bearing file integrity, is no longer true
threat-model-summary#3.1 · no · the paste framing, the typed-text rule and the empty-text refusal are unchanged with viola-core not edited, and send.rs adds only the instance check ahead of its frame, which writes nothing on refusal either
threat-model-summary#4.1 · no · the wheel and the stdin classifier are untouched, and the run.rs changes (the home-tree check and the collision-check replay read) do not reach the stdin pump
threat-model-summary#5.1 · yes · add-to · the statusline arm now runs the widened instance check, read rule included, before it reads snapshot.json, and a replayed snapshot carries no statusline_command, a refusal source this integrity claim does not yet name
threat-model-summary#6.1 · yes · add-to · the viola mcp send, wait, last and answer tools now run the instance's strict-modes check before liveness and return instance-unreachable with no frame and no line, a refusal source the MCP boundary does not yet name
threat-model-summary#7.1 · yes · add-to · the six CLI frame verbs and link and unlink now run the instance's strict-modes check before any frame, exiting 21 with the instance pair or instance-unreachable under --json, a refusal source the CLI boundary does not yet name
threat-model-summary#8.1 · yes · contradict · the claim that nothing checks integrity beyond the content hash no longer holds, since the pinned copy and the bin and plugin folders are now mode and DACL checked at run and revive start, and the replay has six product readers rather than the two it names
threat-model-summary#9.1 · no · the spawn argv, cwd, PATH resolution and count still hold on the changed revive.rs, verify.rs, tools.rs and run.rs, since one agents read still precedes the single snapshot read and the new checks sit ahead of the spawns they guard
threat-model-summary#10.1 · no · no Cargo.toml, Cargo.lock, script or workflow changed, so the cargo-deny, tokio-ban and fuzz-lockfile claims hold
threat-model-summary#11.1 · no · no CI workflow or job changed, and the new strict-modes tests witness on the three OSes ci.yml already runs, so the job list still holds
