# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surface entries; states no rule the chunk's code must honour
obs-scope-summary#1.2 · no · a count of instrumentable crates; states no rule the chunk's code must honour
obs-scope-summary#1.3 · no · a count of must-trace paths and edge flows; states no rule the chunk's code must honour
obs-scope-summary#1.4 · no · a count of telemetry triggers; the strict-modes trigger list is not stated here
obs-scope-summary#2.1 · yes · the open fork on the diagnostics root (`<home>/diagnostics/` vs `instances/<name>/diagnostics/`) decides where the new home-level refusal lines land
obs-scope-summary#2.2 · no · the mcp file sink and stderr fallback; the chunk's MCP refusal is isError only and touches no sink
obs-scope-summary#2.3 · no · the event enum extensions for release-from-driver and chaos signals; the chunk adds no event value
obs-scope-summary#2.4 · no · hook-invoked corr timing against dialog_id is Not needed (Dialog scenario excluded); the hook refusal is a fail-open via hook-decision
obs-scope-summary#3.1 · no · source attribution for the root bin's crate
obs-scope-summary#3.2 · no · instrumentability label only
obs-scope-summary#3.3 · no · standing S8 · anyhow stays at the root bin's dispatch edge, with context chains only at dispatch
obs-scope-summary#4.1 · no · source attribution for viola-core
obs-scope-summary#4.2 · no · not-instrumentable label only
obs-scope-summary#4.3 · yes · viola_core::UnreachableCause's closed exit-21 cause words and the no-line rule for a refused strict-modes check bind the new socket-directory refusal and its closed code
obs-scope-summary#5.1 · no · source attribution for the PTY seam
obs-scope-summary#5.2 · no · instrumentable label only
obs-scope-summary#5.3 · no · the PTY seam and user content; the chunk does not touch the PTY
obs-scope-summary#6.1 · no · source attribution for the IPC crate
obs-scope-summary#6.2 · no · instrumentable label only
obs-scope-summary#6.3 · yes · the scope names the rule that viola-channel instrumentation stays sync, Tokio only behind its feature, and the new euid check lands in both the sync and Tokio clients
obs-scope-summary#7.1 · no · source attribution for the store crate
obs-scope-summary#7.2 · no · instrumentability label only
obs-scope-summary#7.3 · yes · the sync-only instrumentation rule for the store crate (viola-state) binds the new refusal in board.rs's scan home check
obs-scope-summary#8.1 · no · source attribution; serde_path_to_error drift reports are not reached
obs-scope-summary#8.2 · no · instrumentability label only
