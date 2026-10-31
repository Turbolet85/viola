# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the session logs no sensitive value, since the new codes and the UnreachableCause words hold no path, pid or upstream text, so drift to diagnostics and the never-logged R8 values still hold
obs-scope-summary#9.1 · no · the rmcp and Workspace source still holds, Cargo.toml is reached but no dependency is added or bumped
obs-scope-summary#9.2 · no · viola-mcp stays instrumentable, the change adds a cause mapping and no new site
obs-scope-summary#9.3 · yes · add-to · viola-mcp maps each refused frame to a cause (F10) in its isError detail, which this entity does not yet carry, and stdout still carries only MCP frames
obs-scope-summary#10.1 · no · viola-pty source unchanged, no pty file is in the diff
obs-scope-summary#10.2 · no · boundary-only instrumentability unchanged, no pty file is in the diff
obs-scope-summary#10.3 · no · spawn and exit are detected on the process handle as before, and the process-exit codes are written by the cmd layer, not at the pty boundary
obs-scope-summary#11.1 · no · the Standard Contracts source (Hook contract, MCP tools) still holds, the isError body changes its detail only
obs-scope-summary#11.2 · no · boundary-only instrumentability unchanged
obs-scope-summary#11.3 · no · MCP tool-call handling is still the instrumented site, isError stays the only record, and the stdin cut and absent trace context are untouched
obs-scope-summary#12.1 · no · the sysinfo and claude agents source is unchanged, no file of this scope is in the diff
obs-scope-summary#12.2 · no · boundary-only instrumentability unchanged
obs-scope-summary#12.3 · no · the parse outcome and spawn and exit are unchanged, no file of this scope is in the diff
obs-scope-summary#13.1 · no · the statusline shell-out source is unchanged and src/cmd/hook.rs is not in the diff
obs-scope-summary#13.2 · no · boundary-only instrumentability unchanged
obs-scope-summary#13.3 · no · shell-out duration and exit status are unchanged, and the hook is untouched
obs-scope-summary#14.1 · no · the Stack source is unchanged
obs-scope-summary#14.2 · no · boundary-only instrumentability unchanged
obs-scope-summary#14.3 · no · server-verify-failed keeps its word on the CLI, MCP still logs no line, and the report measures the strict-modes refusals unmoved
obs-scope-summary#15.1 · no · the ndjson and snapshot source is unchanged and no store format or reader is added
obs-scope-summary#15.2 · no · instrumentable through the viola-state call sites still holds, since without_endpoint writes no line
obs-scope-summary#15.3 · yes · add-to · viola-state gains without_endpoint (F5), which a no-endpoint instance now reads beside live_endpoint's None, a reading this entity does not yet carry, and the scrub rules stand
obs-scope-summary#16.1 · no · the Observability Hints and security config-values source names no moved file
obs-scope-summary#16.2 · no · not-instrumentable stands for static configuration
obs-scope-summary#16.3 · yes · stale · src/main.rs is reached whole and the report does not show the root bin's config-parse skipped-key log unmoved
