# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the reached root-bin modules moved only test captures, no product log line or field moved, and the session's DACL assertions cover the pipe listener and not diagnostics/, so the drift-report and R8-value claims hold
obs-scope-summary#9.1 · no · the rmcp source is unchanged, since the root Cargo.toml gains only a cfg(windows) dev-dependency table that reaches no release build
obs-scope-summary#9.2 · no · no viola-mcp code changed and the Cargo.toml dev-dependency reaches no release build, so the MCP instrumentability holds
obs-scope-summary#9.3 · no · no viola-mcp code changed and the test helper installs no global default, so stdout stays reserved for MCP frames
obs-scope-summary#10.1 · no · the portable-pty source and the unmodified-claude boundary are unchanged in a test-only chunk with no product behaviour moved
obs-scope-summary#10.2 · no · the boundary-only classification stands, since the spawn, exit, paste and readiness-gate lines are unchanged and only test captures moved
obs-scope-summary#10.3 · no · the run-start spawn and exit, the readiness-gate outcome and the send-refused line are unmoved, as only their capture calls changed with no asserted event, field or level
obs-scope-summary#11.1 · no · the hook contract and MCP tool surface are untouched, since the dialog and statusline modules moved only test captures and no viola-mcp or wait/last code changed
obs-scope-summary#11.2 · no · the boundary-only classification stands, since hook-invoked, hook-decision and MCP tool-call handling emit the same lines and only test captures moved
obs-scope-summary#11.3 · no · hook-invoked and hook-decision lines are unchanged, since the dialog and statusline modules moved only their capture calls and no MCP tool-call code changed
obs-scope-summary#12.1 · no · the sysinfo and claude agents --json sources are unchanged, since the reached files do not include the agents read and no release dependency moved
obs-scope-summary#12.2 · no · the boundary-only classification stands, since the agents spawn and exit are unchanged and the run-start module moved only its test capture
obs-scope-summary#12.3 · no · the agents parse outcome and its unknown-plus-counter fallback are unchanged, since no agents code is reached and the run-start asserted fields did not move
obs-scope-summary#13.1 · no · viola hook statusline stays the only shell-out, since the test-only session adds no shell-out and the statusline module moved only its test capture
obs-scope-summary#13.2 · no · the boundary-only classification stands, since the shell-out duration and exit status lines are unchanged and the statusline module moved only its test capture
obs-scope-summary#13.3 · no · the command string stays out of every line, since the statusline module moved only its test capture and no asserted field or level changed
obs-scope-summary#14.1 · no · the Stack OS-primitive source is unchanged, since the new Windows pipe cases are test code that reads pipe properties and changes no call site
obs-scope-summary#14.2 · no · the boundary-only classification stands, since the bind, connect, lock, rename and kill call sites are unchanged and no channel log line or field moved
obs-scope-summary#14.3 · no · the session instruments no OS internal, since the Windows pipe cases only assert pipe admission in test code and the viola-channel call sites are unchanged
obs-scope-summary#15.1 · no · the ndjson, snapshot and heartbeat sources are unchanged, since no wire contract, event value or log field moved
obs-scope-summary#15.2 · no · instrumentability through the viola-state call sites holds, since no viola-state code changed and the torn-append rule is unmoved
obs-scope-summary#15.3 · no · the per-store scrub rules hold, since the budget gate moved only its test capture and the new pipe names are test-only, so no store content changed
obs-scope-summary#16.1 · no · the config sources are unchanged, since no config.json key, value or parse moved in the session
obs-scope-summary#16.2 · no · config.json stays not-instrumentable, since the session's Untouched list names config.json and plugin files and the session changed neither
obs-scope-summary#16.3 · no · the config parse still logs through the root bin, since the config-rejection WARN claim is unmoved and the obs module moved only a test capture
