# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surfaces that the chunk neither adds to nor re-reads
obs-scope-summary#1.2 · no · a count of instrumentable crates that the chunk neither changes nor re-reads
obs-scope-summary#1.3 · no · a count of must-trace paths and edge flows that the chunk neither changes nor re-reads
obs-scope-summary#1.4 · no · a count of telemetry triggers that states no rule the chunk's code or tests must honour
obs-scope-summary#2.1 · no · the open diagnostics root, since no chunk need names a diagnostics file and the new test homes sit under target/e2e-home whichever root is ratified
obs-scope-summary#2.2 · no · the mcp stderr fallback, since the chunk does not touch the MCP tools or their Tokio client
obs-scope-summary#2.3 · no · the pending event extensions release-from-driver and the chaos signals, neither of which the chunk adds
obs-scope-summary#2.4 · no · hook-invoked corr timing, since the chunk leaves the hook roles alone
obs-scope-summary#3.1 · no · a provenance line naming the arch excerpt and stack pins, which states no rule of its own
obs-scope-summary#3.2 · yes · the root bin is reached at the run start, refuse_squatted and the budget send refusal, so its new code must be instrumented
obs-scope-summary#3.3 · yes · the exit-1 start line and refuse_squatted must emit fixed messages and closed codes, never the anyhow chain
obs-scope-summary#4.1 · no · a provenance line naming the arch excerpt and the workspace, which states no rule of its own
obs-scope-summary#4.2 · no · viola-core stays uninstrumented and the chunk adds no span or line inside it
obs-scope-summary#4.3 · yes · viola-core supplies the refusal, detail and event kebab values that the chunk's send-refused and exit-1 lines pass as given
obs-scope-summary#5.1 · no · a provenance line naming the arch excerpt and portable-pty, which states no rule of its own
obs-scope-summary#5.2 · no · the PTY seam is outside the chunk, which stops before any child spawn
obs-scope-summary#5.3 · no · the PTY byte stream and the ConPTY boundary are not reached, since the chunk handles no PTY bytes
obs-scope-summary#6.1 · no · a provenance line naming the arch excerpt, the interprocess pin and the IPC contract list, which states no rule of its own
obs-scope-summary#6.2 · yes · the viola-channel listener and bind are reached, so a remote-client or first-instance refusal is captured at the viola seam
obs-scope-summary#6.3 · yes · the listener must stay sync with no Tokio in viola-channel's normal graph, so the new win.rs code adds no async path
obs-scope-summary#7.1 · no · a provenance line naming the arch excerpt and the stack pins, which states no rule of its own
obs-scope-summary#7.2 · no · viola-state is only used, unchanged, and the Not-needed list leaves its torn-line healing out
obs-scope-summary#7.3 · no · the Tokio ban on viola-state is not reached, since the chunk adds no code or dependency there
obs-scope-summary#8.1 · no · a provenance line naming the arch excerpt and serde_path_to_error drift reports, which states no rule of its own
obs-scope-summary#8.2 · no · this crate is not among the chunk's reached files
