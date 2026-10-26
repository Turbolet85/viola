# Classified — obs · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · no · the mcp file sink and its explicit writer are not reached by the pin or lock moves
obs-scope-summary#22.4 · no · service identity (process mcp, instance from VIOLA_NAME) is unchanged by a dependency move
obs-scope-summary#22.5 · no · the isError codes-only rule sits in viola-mcp's handler, a rule the pin and lock moves do not reach
obs-scope-summary#23.1 · yes · the scope names cargo deny's Tokio-in-sync-crates and C-building-crate bans (deny-sync.toml, channel-tokio-free.sh), which the pin and lock change must keep passing
obs-scope-summary#24.1 · yes · tracing-subscriber 0.3.23 must keep the security Vector 9 floor of >=0.3.20 (RUSTSEC-2025-0055), an obs dependency the Vector 9 need requires to pass cargo deny
obs-scope-summary#25.0 · no · standing S1 · the span and log discipline a forced source line keeps: one JSON line per event, the closed field catalog, no multi-line stack traces
obs-scope-summary#26.1 · no · the diagnostics locations and modes are not reached, and the secret-scan that enforces them is a standing gate every change runs
obs-scope-summary#27.1 · no · a surfaces list; the sync channel bind is not reached by the Tokio move
obs-scope-summary#27.2 · no · the run start's must-trace spans and events, which the chunk does not change (Not needed: the six must-trace path scenarios)
obs-scope-summary#27.3 · no · run's required log fields and exit-1 detail codes are unchanged (Not needed: the log-field and detail code catalogs)
obs-scope-summary#27.4 · no · a source citation, not a rule the chunk must honour
obs-scope-summary#28.1 · no · a surfaces list for the confirmed send; no hop of it is changed
obs-scope-summary#28.2 · no · the confirmed send's must-trace spans, which the chunk does not change (Not needed: the six must-trace path scenarios)
obs-scope-summary#28.3 · no · the send's required fields and refusal detail codes are unchanged (Not needed: the log-field and detail code catalogs)
obs-scope-summary#28.4 · no · a source citation, not a rule the chunk must honour
obs-scope-summary#29.1 · no · a surfaces list for wait and last; no hop of it is changed
obs-scope-summary#29.2 · no · wait and last's must-trace spans, which the chunk does not change (Not needed: the six must-trace path scenarios)
obs-scope-summary#29.3 · no · wait's required fields and outcome codes are unchanged (Not needed: the log-field and detail code catalogs)
obs-scope-summary#29.4 · no · a source citation, not a rule the chunk must honour
obs-scope-summary#30.1 · no · a surfaces list for the dialog path; no hop of it is changed
obs-scope-summary#30.2 · no · the dialog path's must-trace spans, which the chunk does not change (Not needed: the six must-trace path scenarios)
obs-scope-summary#30.3 · no · the dialog path's required fields and refusal codes are unchanged (Not needed: the log-field and detail code catalogs)
obs-scope-summary#30.4 · no · a source citation, not a rule the chunk must honour
obs-scope-summary#31.1 · no · a surfaces list for the wheel; no hop of it is changed
obs-scope-summary#31.2 · no · the wheel's must-trace spans and the release-from-driver event admitted by schemas/diag-line.v1.json are unchanged (Not needed: the six must-trace path scenarios)
