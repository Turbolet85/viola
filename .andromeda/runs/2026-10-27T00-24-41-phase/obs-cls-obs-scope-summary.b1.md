# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surfaces the chunk does not add or remove
obs-scope-summary#1.2 · no · a count of instrumentable crates the chunk does not add or remove
obs-scope-summary#1.3 · no · a count of must-trace paths and edge flows the chunk does not add or remove
obs-scope-summary#1.4 · no · a count of telemetry triggers the chunk does not add or remove
obs-scope-summary#2.1 · yes · the open diagnostics root decides where the chunk's pin-write detail-<process>.ndjson lines go
obs-scope-summary#2.2 · no · the viola mcp log and its stderr fallback are named Not needed, so no mcp unit binds
obs-scope-summary#2.3 · no · the chunk adds no event value, only a detail code on process-exit
obs-scope-summary#2.4 · no · hook-invoked corr timing and dialog_id are Not needed, as no dialog path changes
obs-scope-summary#3.1 · no · a source citation for the root crate that states no rule the run.rs or verify.rs edits must honour
obs-scope-summary#3.2 · yes · the root crate's instrumentable status rules over the run.rs start pin call and the verify.rs pin_exe call the chunk edits, whose new process-exit detail must be instrumented
obs-scope-summary#3.3 · yes · the anyhow dispatch boundary binds the pin-write error reaching run and verify, whose full chain goes only to diagnostics
obs-scope-summary#4.1 · no · a source citation for viola-core, which the chunk does not reach
obs-scope-summary#4.2 · no · viola-core is not a reached file and the chunk adds no instrumentation there
obs-scope-summary#4.3 · yes · viola-core supplies the closed kebab-case log-field values that the chunk's new closed detail codes must join, so its home binds
obs-scope-summary#5.1 · no · a source citation for viola-pty, which the chunk does not reach
obs-scope-summary#5.2 · no · viola-pty is not a reached file, and its instrumentable status rules over code the chunk leaves unchanged
obs-scope-summary#5.3 · no · a re-check every change owes: the PTY byte stream is never logged, and the chunk logs no stream byte
obs-scope-summary#6.1 · no · a source and IPC-method citation, and the chunk adds or changes no channel method
obs-scope-summary#6.2 · no · viola-channel is not a reached file and the chunk adds no channel instrumentation
obs-scope-summary#6.3 · no · viola-channel is not a reached file, so its sync rule governs no code the chunk adds
obs-scope-summary#7.1 · no · a source citation for viola-state that states no rule the pin.rs or fs.rs edits must honour
obs-scope-summary#7.2 · yes · viola-state is instrumentable, and the chunk's pin write adds a WARN or ERROR line for its typed io error inside it
obs-scope-summary#7.3 · yes · the sync and Tokio-free rule binds the chunk's new viola-state instrumentation, under the sync-crate ban over its graph
obs-scope-summary#8.1 · no · a source citation (serde_path_to_error drift reports) that states no rule over a unit the chunk edits
obs-scope-summary#8.2 · no · the chunk edits no unit of the crate this instrumentable status describes
