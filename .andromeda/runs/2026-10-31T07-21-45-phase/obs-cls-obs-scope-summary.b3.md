# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · source attribution only (arch excerpt, CI/CD Platform); states no rule the chunk's code or tests must honour
obs-scope-summary#17.2 · no · CI/CD Platform not-instrumentable; the chunk adds no CI telemetry and the secret-scan inputs are named by the scope itself
obs-scope-summary#17.3 · yes · fixtures are scrub-required data; the witnesses' planted CLAUDE*, path and upstream values are such fixtures and must stay out of scanned or uploaded inputs (secret-scan leg of v1-47)
obs-scope-summary#18.1 · no · generic tracing hooks and panic-hook line for the process; the chunk adds no span or panic path on the three error surfaces
obs-scope-summary#18.2 · no · frontend instrumentation N/A; terminal output is not a telemetry hook the chunk changes
obs-scope-summary#18.3 · yes · full error goes only to diagnostics/ when an instance resolves and human stderr carries only fixed-message lines, which the fault path and the internal-error stderr line must honour
obs-scope-summary#18.4 · no · service identity values (name, version, process, instance) are unchanged by the chunk and bind no error body
obs-scope-summary#18.5 · yes · refusal and fault stderr lines are fixed strings with no paths, pids, chains or upstream text, and exit codes are a logged outcome field the error documents must carry
obs-scope-summary#19.1 · no · run-process span hooks on the start sequence and the PTY seam; none produces a body on the three surfaces
obs-scope-summary#19.2 · no · frontend instrumentation N/A; the terminal carries the child's screen
obs-scope-summary#19.3 · no · run writes to diagnostics/run-<name>.ndjson only; the sink is unchanged and no error body is added there by this chunk
obs-scope-summary#19.4 · no · service identity as for short-lived verbs with process="run"; unchanged by the chunk
obs-scope-summary#19.5 · no · run panic-hook replacement, heartbeat tick and child exit detection are unchanged and bind no error body on the three surfaces
obs-scope-summary#20.1 · no · hook spans and catch_unwind panic line for the hook process; the chunk changes no hook output on the three surfaces
obs-scope-summary#20.2 · no · frontend instrumentation N/A for hooks
obs-scope-summary#20.3 · no · hook sink and stdout decision body are unchanged; the statusline stdout pass-through is governed by the scope's bounded status-line read, not this line
obs-scope-summary#20.4 · no · hook service identity and silent no-op on absent instance are unchanged by the chunk
obs-scope-summary#20.5 · no · append-mode single-write discipline and latency budget for hooks are unchanged; the chunk adds no hook I/O
obs-scope-summary#21.1 · yes · channel JSON-RPC correlation through the id-based corr field is the channel-response line the chunk must read against error_code and the causes it cannot tell apart
obs-scope-summary#21.2 · no · frontend instrumentation N/A for channel
obs-scope-summary#21.3 · yes · the channel error body reaches the peer's log file; that file must carry only codes and fixed text on the -32603 and -32602 paths the chunk edits
obs-scope-summary#21.4 · no · peer version as params.sender is unchanged by the chunk
obs-scope-summary#21.5 · yes · -32603 must be "internal error" with data null and a self-reported from stays unauthenticated, both bound on the channel error body the chunk edits
obs-scope-summary#22.1 · yes · MCP tool-call spans wrap the channel client; the span must stay allow-list and must not carry upstream text into the isError body
obs-scope-summary#22.2 · no · frontend instrumentation N/A for MCP
