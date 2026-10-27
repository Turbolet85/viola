# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · yes · prompt text and last_assistant_message are written verbatim into events.ndjson, so the per-line MAX_FRAME cap on its replay and tail must not tear a line the wrapper itself wrote
threat-model-summary#1.2 · yes · tool input, plan, question and option text cross into the hook.dialog frame and events.ndjson, so the channel frame cap and the option_details string rule decide what a dialog frame may carry
threat-model-summary#1.3 · yes · config.json, the statusline sources, snapshot.json (statusline_command, endpoint) and ledger/stamps.json are read here, so each needs its cap, closed type and strict-modes-first ordering
threat-model-summary#1.4 · yes · the claude agents --json read must spawn under the R8 strip this entity names, and no new refusal or log line may carry a CLAUDE* value
threat-model-summary#1.5 · yes · no revive refusal line may hold the recorded cwd, a pid or a logged id, and budget.json is read only through read_budget under the cap
threat-model-summary#2.1 · yes · the channel frame boundary is the IPC input half the chunk bounds, and the entity states input validation as only JSON-RPC parsing, the v check and the ViolaName nutype
threat-model-summary#3.1 · yes · the paste rule is this chunk's own, refusing C0 bytes such as ESC whole so an embedded ESC[201~ cannot end the bracketed paste early, with the typed-text and empty-text rules at the client and in the wrapper
threat-model-summary#4.1 · yes · the terminal stdin is a boundary the external-reader enumeration must list with its closed non-editing type rule, and no bound or refusal may block or delay a human keystroke
threat-model-summary#5.1 · yes · hook stdin for every event and for hook statusline is capped at MAX_FRAME, mapped into the closed HookEvent, checked against the strict-modes instance first, and fails open with exit 0 and no body
threat-model-summary#6.1 · yes · each MCP handler checks the raw tool arguments (target, text, dialog_id, response, after, timeout_ms) itself, and the strict-modes check runs before any frame
threat-model-summary#7.1 · yes · CLI positionals and link/unlink names pass ViolaName::try_new in the clap value parser, send and answer input is read under MAX_FRAME, and the home that --home or VIOLA_DIR chooses resolves through the chunk's pair rule
threat-model-summary#8.1 · yes · the own state files read here (snapshot.json, events.ndjson replay and tail, ledger/stamps.json, budget.json, heartbeat) take the per-line and per-file caps, the torn-line rule and reads only after the strict-modes check
threat-model-summary#9.1 · no · the pinned copy's no-clobber placement and its ETXTBSY measurement were landed by the previous chunk and are not reopened here
threat-model-summary#10.1 · yes · claude agents --json is read as a bounded direct spawn and the claude --version output under the cap with tolerant parsing, and verify's PTY output reaches the vt100 feed under catch_unwind and is read only against compiled signature literals
threat-model-summary#11.1 · no · standing S4 · a new fuzz or property target stays in the separate fuzz/ workspace, its lockfile audited on its own
threat-model-summary#12.1 · no · standing S5 · a test-only helper, seam or feature stays out of the release build, which release-check.sh refuses for test-support and fake-agent artifacts
