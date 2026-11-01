# Classified — obs-plan · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · the session adds no telemetry surface entry, so the roles and product-part counts hold
obs-scope-summary#1.2 · no · no crate was added or removed, so the instrumentable-crate count holds
obs-scope-summary#1.3 · no · the must-trace path count holds; the session reorders steps and adds a start step, but no path
obs-scope-summary#1.4 · no · no trigger was added or removed; the session changes a trigger's scope, not the count
obs-scope-summary#2.1 · no · the diagnostics/ directory root and its layout are untouched
obs-scope-summary#2.2 · no · MCP's stderr-when-unopenable logging is untouched; the new frame check writes no line
obs-scope-summary#2.3 · no · the event enum is not extended; the session adds detail codes, not events
obs-scope-summary#2.4 · no · the refused hook path follows the existing hook-invoked-then-decision order and settles no corr-timing question
obs-scope-summary#3.1 · no · no dependency was added, so the CLI crate's stack pins hold
obs-scope-summary#3.2 · no · the CLI crate stays instrumentable; the new refusals are logged lines
obs-scope-summary#3.3 · no · it stays the only anyhow crate, and the new Refused variant is a fixed message naming no path, as Vector 6 requires
obs-scope-summary#4.1 · no · viola-core's stack pins and workspace placement hold; no dependency moved
obs-scope-summary#4.2 · no · viola-core stays a pure type crate with no I/O; the new variant is a type, not behaviour
obs-scope-summary#4.3 · yes · stale · the new socket-dir-refused word makes exit 21's closed cause set six, not five
obs-scope-summary#5.1 · no · viola-pty was not edited and its stack pins hold
obs-scope-summary#5.2 · no · viola-pty was not edited and stays instrumentable
obs-scope-summary#5.3 · no · viola-pty was not edited; PTY bytes stay unlogged
obs-scope-summary#6.1 · no · the interprocess pin and the IPC method list are unchanged; the new euid read adds no method
obs-scope-summary#6.2 · no · the seam stays instrumentable; a dropped peer writes no line by design, which this verdict does not claim against
obs-scope-summary#6.3 · no · the seam stays sync and Tokio-free; TokioClient::server_euid sits behind the existing feature and no dependency was added
obs-scope-summary#7.1 · no · no dependency moved; the ndjson, atomic-write-file and sysinfo pins hold, and no store format changed
obs-scope-summary#7.2 · no · its crate stays instrumentable on the changed files; no store format changed
obs-scope-summary#7.3 · no · its crate stays sync and Tokio-free on the changed files; no dependency was added
obs-scope-summary#8.1 · no · viola-agent-claude and its drift reports were not edited
obs-scope-summary#8.2 · no · viola-agent-claude was not edited and stays instrumentable
