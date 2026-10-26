# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the drift-report routing and the R8 strip are code rules the session did not change, since no source changed and the reached Cargo files carry no log rule
obs-scope-summary#9.1 · no · the rmcp 3.4.1 pin and the workspace members hold in the changed Cargo.toml, since only line 242 (the tokio pin) moved
obs-scope-summary#9.2 · no · instrumentability still holds on the changed lock, since the claim names no tokio version and rmcp-features still reads server transport-io
obs-scope-summary#9.3 · no · the stdout-reserved rule is a convention in code the session did not change, since no source changed
obs-scope-summary#10.1 · no · the portable-pty =0.8.1 pin holds in the changed Cargo.toml, since only line 242 (the tokio pin) moved
obs-scope-summary#10.2 · no · the boundary-only rule holds, since the session changed no pty code and no crate gained or lost tokio
obs-scope-summary#10.3 · no · the spawn/exit, paste and readiness-outcome boundary in viola-pty is unchanged, since the session changed no pty code and viola-pty gained no tokio
obs-scope-summary#11.1 · no · the hook and MCP tool contracts are unmoved, since no schema, symbol or rmcp pin changed
obs-scope-summary#11.2 · no · the boundary-only rule holds, since the session changed no hook or MCP tool code
obs-scope-summary#11.3 · no · the hook-invoked, hook-decision and MCP tool-call instrumentation is untouched, since no event, field or source changed
obs-scope-summary#12.1 · no · the sysinfo and claude agents --json citation holds on the changed Cargo.lock, since the delta is the tokio entry alone
obs-scope-summary#12.2 · no · the boundary-only rule holds, since the agents read and its parse are unchanged and no source changed
obs-scope-summary#12.3 · no · the spawn/exit and parse-outcome instrumentation is untouched, since no source changed and the agents read, parse and counter are unchanged
obs-scope-summary#13.1 · no · the viola hook statusline shell-out contract is unchanged, since no statusline source changed
obs-scope-summary#13.2 · no · the boundary-only rule holds, since the user-command shell-out is unchanged and no source changed
obs-scope-summary#13.3 · no · the shell-out duration, exit status and never-logged command rule are untouched, since no statusline source changed
obs-scope-summary#14.1 · no · the Stack citation holds on the changed Cargo.lock, since the delta is the tokio entry alone and no other crate moved
obs-scope-summary#14.2 · no · the boundary-only rule holds, since the four AsyncFd constructors tokio 1.53.3 deprecates reach no viola call site
obs-scope-summary#14.3 · no · the bind, connect, lock, rename and kill-fallback call sites are unchanged, since the session moved no call site
obs-scope-summary#15.1 · no · the ndjson and snapshot Stack citation holds on the changed Cargo.lock, since no crate gained or lost tokio and the delta is the tokio entry alone
obs-scope-summary#15.2 · no · the instrumentability through viola-state holds, since its containment was re-measured unmoved and it gained no tokio
obs-scope-summary#15.3 · no · the per-store scrub rules and the events.ndjson audit stream are unchanged, since no store or scrub code changed
obs-scope-summary#16.1 · no · the config source citation holds, since config.json and its data classes are not in the reached Cargo files and no source changed
obs-scope-summary#16.2 · no · the not-instrumentable rule holds, since the static config read is code the session did not change
obs-scope-summary#16.3 · no · the load-time parse and its skipped-key logging are unchanged, since no config read changed
