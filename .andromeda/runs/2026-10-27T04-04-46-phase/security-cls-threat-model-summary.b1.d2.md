# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · yes · the events.ndjson replay caps each line at MAX_FRAME and counts an over-long line as torn, so the prompt and reply text it holds must keep the same MAX_FRAME rule at send input and at replay
threat-model-summary#1.2 · yes · the hook payload's permission tool input, plan text and question option_details (description and preview kept only as strings, else null) are read through the capped hook stdin parser
threat-model-summary#1.3 · yes · config.json, the statusline source and command, ledger/stamps.json and the pinned bin/plugin key before its join are read by the chunk under take(MAX_FRAME) or a closed shape
threat-model-summary#1.4 · yes · the claude agents --json child spawn runs under the R8 strip, and the new refusal and parse-rejected lines must carry no R8-stripped CLAUDE* value
threat-model-summary#1.5 · yes · revive's refusal lines must hold no recorded cwd, pid or logged session id, and the snapshot and budget.json readers the chunk bounds carry these fields
threat-model-summary#2.1 · yes · its stated input validation (JSON-RPC parse, v check, ViolaName) is what the chunk widens on channel frames: the MAX_FRAME cap before read_line, -32600 on an oversize frame, -32602 on bad params
threat-model-summary#3.1 · yes · the paste rule over decoded chars (LF, CR and TAB allowed, other C0, DEL and C1 refused) and the typed-text rewrite bind CLI send, MCP send and the wrapper before its first wheel read, with empty-text refused
threat-model-summary#4.1 · yes · the terminal is named in the boundary set the reader enumeration is checked against (Needs), so its stdin row carries the closed non-editing list, and no new bound may delay a human keystroke
threat-model-summary#5.1 · yes · hook stdin for every event and for hook statusline is capped at MAX_FRAME, mapped to the closed HookEvent, and fails open with exit 0 and no body
threat-model-summary#6.1 · yes · each MCP handler checks its raw arguments (target, text, dialog_id, response, after, timeout_ms) because schemars is advisory, and a failure is invalid-params with no frame
threat-model-summary#7.1 · yes · CLI positionals and link/unlink names pass ViolaName::try_new in the clap value_parser, send and answer read stdin or --file under MAX_FRAME, and config.json is read tolerantly
threat-model-summary#8.1 · yes · the own-state readers under the home (snapshot.json, events.ndjson, budget.json, ledger/stamps.json, heartbeat) take the per-line cap and torn rule and read only after the strict-modes check
threat-model-summary#9.1 · no · the pinned copy's no-clobber placement (viola_state::fs::place_private_new) is landed by the previous chunk, its file is not reached, and the scope lists it as not reopened
threat-model-summary#10.1 · yes · the claude agents --json read is bounded at MAX_FRAME + 1 from a direct R8-stripped spawn, claude --version output is parsed tolerantly, and verify's screen reads stay on the compiled signature literals
threat-model-summary#11.1 · no · standing S4 · a new fuzz or property target stays in the separate fuzz/ workspace, never linked into viola or in the root [workspace], its lockfile audited on its own
threat-model-summary#12.1 · no · standing S5 · a test-only helper, seam or feature stays out of the release build (scripts/release-check.sh refuses test-support and fake-agent artifacts)
