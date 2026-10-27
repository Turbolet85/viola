# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the session touches no drift report path, and its one new line holds no byte of the cut line, so R8-stripped CLAUDE* values stay out of every log
obs-scope-summary#9.1 · no · rmcp stays the pinned stdio stack, with no dependency added or bumped, Cargo.lock untouched and features server and transport-io unchanged
obs-scope-summary#9.2 · no · viola-mcp stays instrumentable, since the cut writes one WARN line through obs_event!, and the inbound cap is carried by the caller's entity 11.3
obs-scope-summary#9.3 · no · on a cut stdout still holds only the frames answered before it and stderr stays empty, so the no-telemetry-to-stdout reservation holds on the changed lib.rs
obs-scope-summary#10.1 · no · portable-pty still hosts the unmodified claude, and no hosting or spawn path changed
obs-scope-summary#10.2 · no · the child stays a boundary-only third-party binary, and the chaos test adds a passthrough assertion but no event
obs-scope-summary#10.3 · no · spawn and exit, paste writes and the readiness-gate outcome are unchanged, and the feed-panic capture is re-read and unmoved
obs-scope-summary#11.1 · no · the Standard Contracts for the hook and MCP tools are unedited, and no MCP tool or hook was added
obs-scope-summary#11.2 · no · the caller stays boundary-only, since no frame tool, instance check or server check changed
obs-scope-summary#11.3 · yes · add-to · F2 caps the inbound MCP stdin at MAX_FRAME per line, and an over-long line is refused whole and ends the session as a stdin close does
obs-scope-summary#12.1 · no · the sysinfo and claude agents --json stack is unchanged, and the agents read lives in no reached file
obs-scope-summary#12.2 · no · the agents read stays an external-CLI boundary with its spawn and parse outcome unchanged
obs-scope-summary#12.3 · no · the agents parse outcome and its unknown-plus-counter path are unchanged, since the parser enum gains only mcp-stdin and the derived counter keeps no value list
obs-scope-summary#13.1 · no · viola hook statusline stays the only shell-out, and no reached file holds the statusline arm
obs-scope-summary#13.2 · no · the statusline shell-out stays boundary-only and unchanged
obs-scope-summary#13.3 · no · the session adds no log line on the statusline path, so the command stays out of every log and the fs.rs compare adds none
obs-scope-summary#14.1 · no · the Stack holds, with no dependency added or bumped and Cargo.lock untouched
obs-scope-summary#14.2 · no · the OS boundary stays boundary-only, since the cap is viola's own parser check and no OS call changed
obs-scope-summary#14.3 · no · bind, connect, lock, rename and kill-fallback keep their instrumentation, the fs.rs compare adds no event, and MCP still logs no line for a failed server check
obs-scope-summary#15.1 · no · the ndjson and snapshot stack, the event line, the snapshot envelope and the heartbeat are unchanged, and the diag-line change adds one enum value only
obs-scope-summary#15.2 · no · viola-state stays instrumentable through the same call sites, since the bounded compare adds no event, field or detail code
obs-scope-summary#15.3 · no · the scrub rules per store hold, since the new process-log line carries no payload and the replay and state-recovery paths are unchanged
obs-scope-summary#16.1 · no · no config.json key was added and the config-management source holds, since the deny.toml table is cargo-deny's own config and not viola's
obs-scope-summary#16.2 · no · static configuration stays not-instrumentable, and the session makes no config read observable
obs-scope-summary#16.3 · no · only viola's load-time config parse is observable and it is unchanged, since cargo-deny reads deny.toml in CI and not viola
