# Classified — obs-plan · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · the session adds or changes no span and no serialisation setting, so the Spans row's claim is untouched
telemetry-strategy#1.2 · no · the closed event enum is unchanged, the session adds only process-exit detail codes and no event value
telemetry-strategy#1.3 · no · the row names distinct detail codes on process-exit generically, and the session only extends that set
telemetry-strategy#1.4 · no · metrics stay counts of event and detail values, and no count or metric the row names moves
telemetry-strategy#1.5 · no · events.ndjson is not in the session and the audit stream is unchanged
telemetry-strategy#2.1 · no · panic records and the lineless refusals (run, revive start, verify home tree, frame verbs) stay as they were
telemetry-strategy#3.1 · no · the channel wire, viola-channel and channel-response are not reached
telemetry-strategy#3.2 · no · the send-refused event and its side field are unchanged, the session moves only send's exit 21 stdout document
telemetry-strategy#3.3 · no · src/cmd/hook.rs is not in the diff and the hook-decision codes are unchanged
telemetry-strategy#3.4 · no · MCP writes no log line, and the isError body's new closed detail word carries no path or pid
telemetry-strategy#3.5 · no · the catch site still logs its chain and line and the human line stands byte for byte, the new --json document is stdout only
telemetry-strategy#3.6 · yes · stale · verify's strict-modes refusal under a parsed VIOLA_NAME now writes a strict-modes-failed code on its process-exit, so the reading that verify's strict-modes check writes no line must be re-read
telemetry-strategy#3.7 · no · process-start subjects and parse-rejected are unchanged, and refused spawns print before spawn_child with no process-start line
telemetry-strategy#3.8 · no · Cargo.toml gains only a test target, and no dependency or feature is added or bumped
telemetry-strategy#3.9 · no · the conn join key and the channel wire are not reached
telemetry-strategy#3.10 · no · no perf row, budget or threshold moved
telemetry-strategy#3.11 · no · the row points to exit_source and during in §6 and §10, which still carry both fields
telemetry-strategy#3.12 · no · endpoint_kind, pty_backend, log modes and per-OS artifacts are unchanged
telemetry-strategy#3.13 · no · zero-unlogged-panics and the three zero counts are untouched
telemetry-strategy#3.14 · no · no rotation, retention or events.ndjson change, and snapshot replay is not in the diff
telemetry-strategy#3.15 · yes · stale · schemas/diag-line.v1.json's process-exit detail list moved at line 78 from 13 to 19 members, so the schema verbatim in §3 is stale
telemetry-strategy#3.16 · yes · contradict · a frame verb's strict-modes refusal now carries a detail cause on its --json document, so "carries no detail code" no longer holds
telemetry-strategy#3.17 · no · the parse-rejected and skipped counters are unchanged and no metric was added
telemetry-strategy#3.18 · no · the vt100 gate outcome and viola-pty are not reached
