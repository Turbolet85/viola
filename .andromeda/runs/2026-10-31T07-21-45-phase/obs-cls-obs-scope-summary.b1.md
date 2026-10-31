# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surfaces, not a rule the chunk's code or tests act on; the chunk changes no tier
obs-scope-summary#1.2 · no · a count of instrumentable crates, not a rule the chunk's code or tests act on; the chunk changes no tier
obs-scope-summary#1.3 · no · a count of must-trace paths and edge flows, not a rule the chunk's code or tests act on; the chunk changes no tier
obs-scope-summary#1.4 · no · a count of telemetry triggers, not a rule the chunk's code or tests act on; the chunk changes no tier or correlation scheme
obs-scope-summary#2.1 · yes · the detail-file leg and the secret-scan input (`target/e2e-home/**/diagnostics/*.ndjson`) name the root, so the fork decides where `detail-<process>.ndjson` lands and what the scan reads
obs-scope-summary#2.2 · yes · mcp's log target decides where the codes-only rejected-call line lands (`mcp.ndjson`, stderr only when that file cannot be opened), and the never-log floor covers stderr as a sink
obs-scope-summary#2.3 · yes · names `release-from-driver`, a `-32602` cause the chunk must tell apart, so the §6 event enum must record it; the chaos signals are outside the chunk
obs-scope-summary#2.4 · no · `hook-invoked` `corr` timing is Not needed; the chunk changes no correlation scheme
obs-scope-summary#3.1 · yes · names `thiserror` and `anyhow`, the mechanisms the fixed Display impls and this crate's chain capture act on
obs-scope-summary#3.2 · yes · the crate is instrumentable and is where the CLI fault path sits (`viola::obs::report_internal_error`, the internal-error stderr line, `process-exit` with `detail:"internal-error"`)
obs-scope-summary#3.3 · yes · the error-capture rule: a serde-sourced anyhow chain becomes the fixed `internal error` before stderr, `--json`, `isError` or `error.data`, with the full chain only in `detail-<process>.ndjson`
obs-scope-summary#4.1 · no · provenance only (arch excerpt, Workspace / Modules), with no upstream or mechanism named; the binding rule is in 4.3
obs-scope-summary#4.2 · no · not-instrumentable; a re-check every change owes, since the pure type crate emits nothing and the scope names no emission in it
obs-scope-summary#4.3 · yes · the closed kebab values `internal-error`, `instance-unreachable`, `parse-rejected` and `release-from-driver` are ones this crate supplies, and the §6 catalog and closed `detail` list must record them, the S3 instance clause
obs-scope-summary#5.1 · yes · names `portable-pty`, whose upstream text must stay off all three surfaces through the viola seam
obs-scope-summary#5.2 · yes · the seam is instrumentable and the upstream failures are captured there, so its events must carry the fixed `PtyError` message
obs-scope-summary#5.3 · yes · the OS APIs are boundary-only, so capture happens at the seam alone, and the never-logged PTY byte stream is a payload no `PtyError` message may carry
obs-scope-summary#6.1 · yes · names `interprocess` (its upstream text stays off the surfaces) and the verbs `link`, `unlink` and `release` whose `-32602` causes the chunk must tell apart
obs-scope-summary#6.2 · yes · the channel is instrumentable and holds the `channel-response` line and its `Drop` guard, `ProtocolError::InvalidParams` and `fault_of`, which the chunk edits
obs-scope-summary#6.3 · yes · the graph bans Tokio unless its feature is on, so the `channel-response` `Drop` guard and any new channel line emit synchronously
obs-scope-summary#7.1 · yes · names `ndjson` with `atomic-write-file`, the writer of the detail-file leg (0600 in 0700), and `sysinfo`, whose upstream text the chunk keeps off the surfaces
obs-scope-summary#7.2 · yes · the sink is instrumentable, and the detail-file writes the chunk adds (chain and drift records) go through it under the never-log floor across every sink
obs-scope-summary#7.3 · yes · Tokio is banned here, so the chunk's detail-file and sink writes must stay synchronous
obs-scope-summary#8.1 · yes · names `serde_path_to_error` drift reports, which the detail-file leg (`drift_report`) and the serde-chain rule act on
obs-scope-summary#8.2 · yes · instrumentable; the drift-report and `config.json` reader behaviour the chunk re-measures (load-time parse only, values kept out of external output) runs in it
