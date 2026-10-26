# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · yes · the listener and dispatch spans follow the skip_all #[instrument] form with a static channel.<op> name, never err or ret (Needs)
telemetry-strategy#1.2 · yes · the remote-client and first-instance bind refusals are captured as viola events, and the refusal and lifecycle lines use the closed event enum (Needs)
telemetry-strategy#1.3 · yes · new or changed ChannelError variants keep a fixed thiserror Display, and the exit-1 start failure carries a distinct process-exit detail code (Needs)
telemetry-strategy#1.4 · yes · the chunk adds no in-process instrument and no exporter, and its Windows-only assertions add no OS-specific exporter (Needs)
telemetry-strategy#1.5 · yes · obs never copies event payloads into process logs, so the new refusal and process-exit lines carry no event payload, path or pid (Needs)
telemetry-strategy#2.1 · yes · a panic in the new listener tests or in run's start leaves its one event:"panic" line in the home-level role file (Needs: G2)
telemetry-strategy#3.1 · yes · channel-request and channel-response lines never record params or result bodies, and a self-reported from is never logged as an authenticated identity (Needs)
telemetry-strategy#3.2 · yes · the send-refused line carries the refusal code and window word and no sent text and no cursor, which case_4 reads (Needs)
telemetry-strategy#3.3 · no · the hook roles are excluded (Not needed), and the chunk touches no hook stdin, hook sink or statusline instance check
telemetry-strategy#3.4 · no · the MCP surface, mcp.ndjson and isError codes are excluded (Not needed), and the chunk does not touch the MCP tools or their Tokio client
telemetry-strategy#3.5 · yes · the endpoint name and env values never reach a span or line, the squatted refusal writes exactly two fixed stderr lines, and no RUST_LOG or env var is read (Needs)
telemetry-strategy#3.6 · yes · each new integration test home lives under target/e2e-home so the §9 secret scan, G2, G4 and the diag upload cover it (Needs)
telemetry-strategy#3.7 · yes · the child-spawn subject rule binds the start sequence up to channel bind, whose span and lines never carry the pinned bin path (Needs)
telemetry-strategy#3.8 · yes · the §3 feature set and §9 cargo-deny gates keep the sync viola-channel crate free of Tokio in its normal graph (Needs); the advisories red stays excluded
telemetry-strategy#3.9 · no · the chunk adds no cross-surface call and sets no corr or conn requirement; the send-refused line it reads is only read, not emitted
telemetry-strategy#3.10 · no · the chunk adds and changes no derived metric and no hyperfine budget; the hook latency budgets are excluded (Not needed)
telemetry-strategy#3.11 · yes · the exit-1 start failure is a process-exit line, which must carry the exit_source and during fields that §6 names for process-exit
telemetry-strategy#3.12 · yes · the same file sink and line schema on every OS, with the Windows-only assertions adding no OS-specific exporter, field or event (Needs)
telemetry-strategy#3.13 · yes · zero unlogged panics: a panic in the new listener tests or in run's start must leave its one event:"panic" line (Needs: G2)
telemetry-strategy#3.14 · no · the chunk rotates, retains and moves no log, and the operational retention of events.ndjson and diagnostics is excluded (Not needed)
telemetry-strategy#3.15 · yes · every line the chunk's code or tests emit conforms to the §3 schema verbatim and gate G4, with no field outside the catalog (Needs)
telemetry-strategy#3.16 · yes · the exit-1 start failure takes a closed, distinct detail code, the squatted-name cause keeps its existing code, and the catalog grows only through the plan (Needs)
telemetry-strategy#3.17 · no · the installed-CLI counters and the parse-rejected and skipped lines are not reached, and the capability ledger read in run.version_gate is excluded (Not needed)
telemetry-strategy#3.18 · no · the chunk reads no screen, since vt100 is excluded (Not needed), and case_4 reads a log line only
