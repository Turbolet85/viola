# Classified — obs · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · yes · mcp always opens its home-level `mcp.ndjson`, so the chunk's refused-name rule (no role file opens) must settle whether it reaches mcp
obs-scope-summary#22.4 · yes · mcp's process, instance and the `from` it adds all come from VIOLA_NAME, which the validator must pass before any is set
obs-scope-summary#22.5 · yes · isError carries codes only, and a rejected mcp call's log line is codes-only with no upstream text, path or type name
obs-scope-summary#23.1 · yes · the panic hook's first-statement, one-line, nothing-on-stderr contract (the vt100 panic witness and the run terminal) and the name-then-strict-modes order before a role file opens
obs-scope-summary#24.1 · no · standing S6 · stdout stays reserved on every role; the sink, writer and line format are kept by every new log line alike
obs-scope-summary#25.0 · yes · the events.ndjson Event line the reader parses, the hook trace's swallowed write with nothing on stderr, the R8 floor on process-start, and the closed event enum that `vt100-feed` and `claude-agents-json` must join (S2)
obs-scope-summary#26.1 · yes · the role file names `run-<name>`, `hook-<name>` and `cli-<name>` take only a name that passed `ViolaName::try_new`, and the diagnostics folder is strict-modes checked before any opens
obs-scope-summary#27.1 · yes · run-start surfaces the chunk's reads sit on: the cli run start (version gate, snapshot and budget reads) and the channel frames the wrapper serves
obs-scope-summary#27.2 · yes · run-start spans `state.snapshot_recover`, `run.version_gate` and `run.budget_eval` wrap reads the chunk bounds, and process-start gains the R8 names and counts, so the spans and event order stay
obs-scope-summary#27.3 · yes · run's `cli_version` and `cli_verified` fields: an unreadable, oversize, malformed or strict-modes-refused `ledger/stamps.json` reads `cli_verified:false`, with no path or pid in the detail
obs-scope-summary#27.4 · no · a provenance citation for 27.2 and 27.3, which carry the rules; it states no rule of its own
obs-scope-summary#28.1 · yes · the send path's surfaces where the chunk's paste validation and bounds land: the CLI and MCP send clients, channel frames, the run's paste and readiness gate, and the UserPromptSubmit and SessionStart hook stdin
obs-scope-summary#28.2 · yes · send's spans and event chain: the CLI and MCP clients refuse `control-character` and `empty-text` before any frame, and the readiness gate's `input-not-ready` degrade sits on this chain
obs-scope-summary#28.3 · yes · send's required fields: `control-character`, `empty-text` and `input-not-ready` are the chunk's refusal details, and sent text never appears, only `text_bytes`
obs-scope-summary#28.4 · no · a provenance citation for 28.2 and 28.3, which carry the rules; it states no rule of its own
obs-scope-summary#29.1 · yes · wait's surfaces: the channel frames the chunk bounds on both sides and the events.ndjson tail reader, whose three skip counts the chunk must keep
obs-scope-summary#29.2 · yes · wait's spans and channel lines: over-size or invalid frames answer -32602 or -32600 at WARN, -32603 is exactly `internal error` with `data: null`, and params or result bodies are never recorded
obs-scope-summary#29.3 · yes · wait's required fields: `outcome` (mcp's `instance-unreachable` gets no line), the `after` byte offset, and `last_assistant_message` content never appears
obs-scope-summary#29.4 · no · a provenance citation for 29.2 and 29.3, which carry the rules; it states no rule of its own
obs-scope-summary#30.1 · yes · dialog's surfaces where the chunk's bounds land: the PreToolUse and PermissionRequest hook stdin, the `hook.dialog` channel frames and the mcp `answer` arguments
obs-scope-summary#30.2 · yes · dialog's spans and events: `hook.handle` reads its stdin under `take(MAX_FRAME + 1)`, `state.snapshot_recover` keeps its `state-recovered` line, and an oversize-stdin fail-open is a `hook-decision` with a fail-open detail
obs-scope-summary#30.3 · yes · dialog's required fields: the fail-open `hook-decision` shape (corr null, decision not emitted), answer, question and tool input text never appear, and a refused instance check writes no line before any stdin read
obs-scope-summary#30.4 · no · a provenance citation for 30.2 and 30.3, which carry the rules; it states no rule of its own
obs-scope-summary#31.1 · no · the human-wheel surfaces (key input, wheel events, pause and release) are Not needed: the paste refusal precedes the first wheel read and changes no wheel line
obs-scope-summary#31.2 · no · the human-wheel spans and pause and release lines are Not needed (the paste refusal precedes the first wheel read); `release-from-driver` is a from check, not one of the chunk's name checks
