# Classified — obs-plan · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the entity holds only its arch source and no CI step, gate or artifact changed
obs-scope-summary#17.2 · no · the not-instrumentable label stands, as build and CI infrastructure is untouched and no workflow file was edited
obs-scope-summary#17.3 · no · the session added test code and no fixture, so the not-product-runtime reason still holds
obs-scope-summary#18.1 · no · the logging-stack claims still hold on the changed src/main.rs and src/obs.rs (test captures only; the one global default and tracing-subscriber 0.3.23 are unmoved)
obs-scope-summary#18.2 · no · the CLI has no frontend and the session moved no product behaviour
obs-scope-summary#18.3 · no · the file sink and diagnostics path still hold on the changed src/obs.rs and src/cmd/run.rs (test captures only; the DACL assertions cover the pipe listener, not diagnostics)
obs-scope-summary#18.4 · no · service.name and service.version still hold on the changed Cargo.toml, which adds only a Windows dev-dependency table, and no log field or wire contract moved
obs-scope-summary#18.5 · no · the instance rule, fixed-string refusal lines and exit-code set still hold, as no exit code, refusal word or detail code moved
obs-scope-summary#19.1 · no · the run, pty, channel-handler, readiness-gate and state-write spans still hold (only test captures moved, and the Tokio-free ban stays green; the tokio red is an advisory, not a sync-crate dependency)
obs-scope-summary#19.2 · no · the run has no frontend (the terminal carries the child's screen) and the session moved no product behaviour
obs-scope-summary#19.3 · no · the run-<name> file sink and the no-terminal rule still hold on the changed src/cmd/run.rs (test captures only)
obs-scope-summary#19.4 · no · the service identity is the short-lived one with process="run", and no log field or wire contract moved
obs-scope-summary#19.5 · no · the panic-hook replacement, the 1 s heartbeat and the handle-wait exit detection still hold (heartbeat code untouched, run-start test captures only)
obs-scope-summary#20.1 · no · the hook spans, hook-invoked and hook-decision lines and the panic line still hold (dialog and statusline moved test captures only, no asserted event changed)
obs-scope-summary#20.2 · no · the hook has no frontend and the session added none
obs-scope-summary#20.3 · no · the hook sink, stdout and stderr rules and the hidden capture arms still hold on the changed src/cmd/hook/statusline.rs (test captures only)
obs-scope-summary#20.4 · no · the service identity is as for the short-lived verbs with process="hook", and the VIOLA_NAME no-op is unchanged
obs-scope-summary#20.5 · no · the append-mode writes, the max < 1.0 s hook budget and the 5 s STATUSLINE_DEADLINE still hold (no logger-init, hook or heartbeat code moved; SLO perf budgets untouched)
obs-scope-summary#21.1 · no · the channel spans and the corr, dialog_id and cursor propagation still hold (no channel log line or corr or conn shape changed; the new cases assert listener admission only)
obs-scope-summary#21.2 · no · the channel has no frontend and the session added none
obs-scope-summary#21.3 · no · the per-side log files still hold, as no channel or process sink moved and the new pipe names are test-only
obs-scope-summary#21.4 · no · each side still logs its own process and params.sender (CARGO_PKG_VERSION) still holds, as no wire contract moved
obs-scope-summary#21.5 · no · the corr uniqueness gap, the -32603 shape and the self-reported from still hold, as no channel log line, corr field or wire shape changed
obs-scope-summary#22.1 · no · the MCP tool-call spans still hold, as no viola-mcp or wait/last code moved
obs-scope-summary#22.2 · no · the MCP surface is stdio with no frontend, and the session moved no MCP code
