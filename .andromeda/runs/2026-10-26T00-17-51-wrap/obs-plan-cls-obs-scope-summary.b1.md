# Classified — obs-plan · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · only a test-only witness site is added, and no process role or telemetry surface moved
obs-scope-summary#1.2 · no · the Windows dev-dependency table adds no workspace crate, so the six instrumentable crates stand
obs-scope-summary#1.3 · no · no must-trace path or edge flow moved, and E3, E5 and the verify and plugin install flows are untouched
obs-scope-summary#1.4 · no · the five new Windows cases are CI assertions, not telemetry triggers, and no corr or perf trigger moved
obs-scope-summary#2.1 · no · the DACL assertions concern the pipe listener's descriptor, not diagnostics files, and no product path moved
obs-scope-summary#2.2 · no · the reached src/obs.rs changed only test capture calls, and the MCP surface with its mcp.ndjson sink is untouched
obs-scope-summary#2.3 · no · the session widened no event enum and made no obs decision
obs-scope-summary#2.4 · no · the reached src/run/dialog.rs and src/cmd/hook/statusline.rs changed only test capture calls, and no hook timing or dialog_id assignment moved
obs-scope-summary#3.1 · no · the dev-only Windows table leaves Cargo.lock and the release graph unchanged, so clap 4.6.7, vt100 0.16.2 and thiserror/anyhow hold
obs-scope-summary#3.2 · no · the root bin stays instrumentable, its product code unchanged and the new helper and witness test-only
obs-scope-summary#3.3 · no · the reached src/main.rs changed only a test helper and a witness, so the dispatch error boundary is unmoved
obs-scope-summary#4.1 · no · viola-core is in no reached file, and the report lists the not-instrumentable entities as untouched
obs-scope-summary#4.2 · no · viola-core is not-instrumentable and in no reached file, so its instrumentability is unmoved
obs-scope-summary#4.3 · no · viola-core is in no reached file and still emits nothing, with no kebab-case value or event kind moved
obs-scope-summary#5.1 · no · no pty file is reached and Cargo.lock is unchanged, so the portable-pty stack claim holds
obs-scope-summary#5.2 · no · viola-pty is in no reached file and stays instrumentable
obs-scope-summary#5.3 · no · no pty path is reached, so the unlogged PTY byte stream is unmoved
obs-scope-summary#6.1 · no · Cargo.lock is unchanged, so interprocess 2.4.4 holds, and no IPC method moved
obs-scope-summary#6.2 · no · viola-channel stays instrumentable, with no channel log line, corr or conn field, or channel-request or channel-response shape moved
obs-scope-summary#6.3 · no · the cargo deny bans check and channel-tokio-free script stay green, so the Tokio-free sync graph holds
obs-scope-summary#7.1 · no · Cargo.lock is unchanged, so ndjson, atomic-write-file and sysinfo 0.39.6 hold, and no viola-state file is reached
obs-scope-summary#7.2 · no · viola-state stays instrumentable, since the torn-append rule and its healing emitter are unmoved
obs-scope-summary#7.3 · no · the cargo deny bans check stays green and no obs dependency was added, so the Tokio ban holds
obs-scope-summary#8.1 · no · Cargo.lock is unchanged, so serde_path_to_error holds, and the reached statusline arm changed only a test capture call
obs-scope-summary#8.2 · no · the crate stays instrumentable, since no product parser or drift-report code moved
