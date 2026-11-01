# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the arch source of the scope-entity classification states no behaviour the chunk's code or tests must honour
obs-scope-summary#17.2 · no · the not-instrumentable label of the scope entities is a classification, not a rule over the chunk's lines
obs-scope-summary#17.3 · no · the fixture scrub reason excludes fixtures from the sections and does not reach the chunk's rows or code
obs-scope-summary#18.1 · yes · the new refusal lines are emitted through this tracing-subscriber JSON logger, so its flattened shape with no current span is what the chunk's lines carry
obs-scope-summary#18.2 · yes · the run-start and list refusal pairs are the cli stderr surface, which must be static fixed-column text with no SGR
obs-scope-summary#18.3 · yes · the refusal ordering binds: a refused instance or home check opens no log and writes no line, and client refusals log as cli once their check passes
obs-scope-summary#18.4 · no · service identity is the field set every line carries, and the chunk adds no service, version or instance source
obs-scope-summary#18.5 · yes · refusal and fault stderr lines carry no paths or pids, and the exit-code field must record the new refusal exits
obs-scope-summary#19.1 · yes · the new viola-channel server-side euid check and its instrumentation must stay sync with Tokio only behind its feature
obs-scope-summary#19.2 · no · frontend N/A for the terminal carrying the child's screen, and the chunk adds no frontend code
obs-scope-summary#19.3 · yes · the wrapper-side euid refusal writes nothing to the terminal and its trace, if any, goes to run-<name>.ndjson
obs-scope-summary#19.4 · no · process=run and instance identity on run lines is the per-line field set the chunk does not change
obs-scope-summary#19.5 · yes · the new pre-child refusal in run's start must sit after the panic hook replacement and before the child spawns
obs-scope-summary#20.1 · yes · a hook connect refusal reaches hook only as a fail-open hook-decision detail under the hook-invoked and hook-decision lines
obs-scope-summary#20.2 · no · frontend N/A, and the chunk adds no frontend code
obs-scope-summary#20.3 · yes · a hook refusal is a fail-open exit 0 with no body and no stderr, and the trace goes only to hook-<name>.ndjson
obs-scope-summary#20.4 · yes · the hook's new connect-time checks fall after the instance trusted check and must write no line when that check refuses
obs-scope-summary#20.5 · yes · the new socket-directory and peer euid checks on the hook connect sit in the timed path under the per-hook latency budget
obs-scope-summary#21.1 · yes · a refused connection is not a channel-request and channel-response pair, and the corr rule over channel lines binds the new refusals
obs-scope-summary#21.2 · no · frontend N/A, and the chunk adds no frontend code
obs-scope-summary#21.3 · yes · each side's client refusal trace goes to its own process log file, and the wait, last and MCP refusal placement follows it
obs-scope-summary#21.4 · no · peer version as params.sender is untouched by the euid check
obs-scope-summary#21.5 · yes · the from field stays self-reported, so a verified euid does not make it an authenticated identity in any line
obs-scope-summary#22.1 · yes · the MCP client refusal maps to instance-unreachable inside mcp.tool_call over the Tokio client, which checks every Unix connect
obs-scope-summary#22.2 · no · frontend N/A, and the chunk adds no frontend code
