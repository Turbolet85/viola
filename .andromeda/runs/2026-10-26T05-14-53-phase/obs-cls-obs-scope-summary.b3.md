# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · a source line naming the CI/CD Platform entity's arch origin, stating no rule the bump's code or tests must honour
obs-scope-summary#17.2 · no · CI/CD is not instrumentable and the chunk's supply-chain work is a job-log gate read that adds no telemetry to CI
obs-scope-summary#17.3 · no · the fixture and not-product-runtime reason; the chunk touches no fixture and no runtime code
obs-scope-summary#18.1 · no · standing S1 · span and log discipline for a source line the bump forces: #[instrument(skip_all, fields(..))], events only through obs_event!, no field outside the Section 6 catalog
obs-scope-summary#18.2 · no · frontend hooks are N/A for the CLI, so no frontend hook is reached
obs-scope-summary#18.3 · no · the file-only exporter and its OTLP and stdout exclusions stay as they are, and the bump adds no exporter
obs-scope-summary#18.4 · no · the service identity (name, CARGO_PKG_VERSION, process, instance) is unchanged and the chunk moves no package version
obs-scope-summary#18.5 · no · instance resolution, fixed refusal strings and the exit-code and detail-code notes are the CLI's sync outcome code and the Not-needed catalogs, which the bump does not reach
obs-scope-summary#19.1 · yes · the sync instrumentation of the viola-pty seam, the channel sync server, viola-state writes and the vt100 gate must stay Tokio-free, so deny-sync and channel-tokio-free.sh must stay green on the bump
obs-scope-summary#19.2 · no · frontend hooks are N/A for run, so no frontend hook is reached
obs-scope-summary#19.3 · no · the run's file-only sink and silent terminal are sync run code the bump does not touch, the run path being std threads with no Tokio
obs-scope-summary#19.4 · no · the service identity with process=run is unchanged
obs-scope-summary#19.5 · no · the default panic-hook replacement, the 1 s heartbeat and the handle-based child exit are the run's own code, and panic hooks and the heartbeat are Not needed
obs-scope-summary#20.1 · no · the hook's spans, required hook-invoked and hook-decision lines and catch_unwind panic line are the hook's own code, which the chunk leaves unchanged, and panic capture is Not needed
obs-scope-summary#20.2 · no · frontend hooks are N/A for the hook, so no frontend hook is reached
obs-scope-summary#20.3 · no · the hook's file-only sink, stdout decision body and stderr ban are the hook's own code, and the chunk edits no hook or output module
obs-scope-summary#20.4 · no · the hook's service identity, including the silent no-op without VIOLA_NAME, is unchanged
obs-scope-summary#20.5 · no · the append-mode single-write discipline and the hook latency budget belong to the hook's writer, which the chunk does not change, and the performance budgets are Not needed
obs-scope-summary#21.1 · no · the channel's client-call and server-dispatch spans and corr propagation are unchanged channel code, and the must-trace channel paths are Not needed
obs-scope-summary#21.2 · no · frontend hooks are N/A for the channel, so no frontend hook is reached
obs-scope-summary#21.3 · no · the exporter is each side's own log file, and the chunk adds no sink
obs-scope-summary#21.4 · no · the peer version in params.sender is a frame field the bump does not touch
obs-scope-summary#21.5 · no · the -32603 body, the corr-join gap and the self-reported from are wire and security Vector 1 rules, Vector 1 is Not needed and the bump changes no error path
obs-scope-summary#22.1 · no · the MCP tool-call spans are viola-mcp's own tool code, which the bump leaves unchanged, and no scope Need names them
obs-scope-summary#22.2 · no · frontend hooks are N/A for the MCP surface, so no frontend hook is reached
