# Classified — obs-plan · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · spans stay declared and unserialized; the session adds no span, and viola-state's new `without_endpoint` is no named seam
telemetry-strategy#1.2 · no · the closed event enum is unmoved (no event value or field added) and refusals are still logged
telemetry-strategy#1.3 · no · distinct detail codes on process-exit still hold; the session only extends that set
telemetry-strategy#1.4 · no · the row names counts of event and detail values, still derived by jq with no instrument added
telemetry-strategy#1.5 · no · events.ndjson is unmoved (replay and appender not in the diff) and the new codes hold no payload
telemetry-strategy#2.1 · no · the panic record and its exemptions are unchanged; the cli catch-site change covers anyhow errors, not panics
telemetry-strategy#3.1 · no · the channel wire, corr and conn are untouched; viola-channel is not in the diff
telemetry-strategy#3.2 · no · send-refused and its side field are unmoved; send.rs changes only the exit 21 document
telemetry-strategy#3.3 · no · hook.rs is not in the diff and the hook-decision codes are unchanged
telemetry-strategy#3.4 · no · MCP's isError stays the only record with no self process-exit; the cause is detail on that same body
telemetry-strategy#3.5 · no · the human fixed lines stand byte for byte, including `error: internal error`
telemetry-strategy#3.6 · no · the strict-modes-before-open exemptions hold; verify's new code is written only after a parsed VIOLA_NAME opens the log, the home-tree refusal stays lineless, and no refused check gained a line
telemetry-strategy#3.7 · no · subject values and parse-rejected are unmoved; the new program-not-found and refused_with codes are process-exit words this entity does not list
telemetry-strategy#3.8 · no · Cargo.toml is reached but no dependency is added or bumped, so the feature set and cargo-deny gates hold
telemetry-strategy#3.9 · no · corr and conn are unchanged and the channel join key is not reached
telemetry-strategy#3.10 · no · no perf row, threshold or budget moved, and no hook is in the diff
telemetry-strategy#3.11 · no · exit_source and during stay as §6 fields with no field added; the moved during value is not one this row names
telemetry-strategy#3.12 · no · endpoint_kind, pty_backend and the concurrent-append check are unmoved; no dependency added, and the macOS server-check gap is the one already recorded
telemetry-strategy#3.13 · no · the panic zero counts and the error budget are untouched
telemetry-strategy#3.14 · no · no rotation changed and events.ndjson stays the audit stream
telemetry-strategy#3.15 · yes · stale · the §3 verbatim schema lists process-exit detail values, and schemas/diag-line.v1.json line 78 grew by six words (stale-heartbeat, unwrapped-name, endpoint-lost, program-not-found, version-unreadable, unclean-fixture)
telemetry-strategy#3.16 · yes · contradict · the frame verbs' exit 21 --json document now carries a closed cause as detail, where this row says a strict-modes refusal carries no detail code
telemetry-strategy#3.17 · no · no counter, parse-rejected or skipped row moved, and the session measures no installed-CLI count
telemetry-strategy#3.18 · no · the vt100 gate outcome is unchanged; viola-pty is not in the diff
