# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · provenance line naming the CI/CD source, with no rule the chunk's code or tests must honour
obs-scope-summary#17.2 · no · the CI/CD platform is not instrumentable and the chunk adds no CI instrumentation (it edits no workflow)
obs-scope-summary#17.3 · no · the exclusion reason (not product runtime, scrub-required fixtures) names no fixture or product surface the chunk changes
obs-scope-summary#18.1 · yes · the chunk's spans and the one-line panic hook that the start-panic test re-asserts rest on this tracing facade and JSON logger
obs-scope-summary#18.2 · yes · the squatted-name refusal writes two fixed stderr lines through human.rs refuse, which is the static terminal text this states
obs-scope-summary#18.3 · yes · the refusal, the process-exit line and the captured listener events go to the file sink diagnostics/run-<name>.ndjson and never to stdout
obs-scope-summary#18.4 · no · the identity of short-lived verbs, which the chunk does not touch (its run-start lines carry the run identity, see 19.4)
obs-scope-summary#18.5 · yes · the refusal lines carry no path or pid, and the exit-1 process-exit line keeps its closed distinct detail code with the exit code as a field
obs-scope-summary#19.1 · yes · spans on the run start sequence and the viola-channel sync server handlers, which must stay sync with no Tokio
obs-scope-summary#19.2 · no · frontend N/A, and the child-screen terminal is the run pump's, which the chunk leaves unchanged (the refusal exits before any child spawns)
obs-scope-summary#19.3 · yes · the new run-start and listener events land in diagnostics/run-<name>.ndjson and never on the terminal
obs-scope-summary#19.4 · yes · the process-exit, panic and listener lines are run-role lines (process run, instance ViolaName), and the endpoint name must never be the instance
obs-scope-summary#19.5 · yes · the start-sequence panic test re-asserts that run's panic hook is replaced before any child spawns
obs-scope-summary#20.1 · no · the hook role's spans, hook-invoked and hook-decision lines and catch_unwind, which the chunk does not reach
obs-scope-summary#20.2 · no · frontend N/A for the hook role, which the chunk does not reach
obs-scope-summary#20.3 · no · the hook-<name>.ndjson sink and the hook stdout rules, with no hook output added or changed
obs-scope-summary#20.4 · no · the hook process identity, and the chunk emits no hook line
obs-scope-summary#20.5 · no · the append rule for concurrent hook processes, and the chunk adds no hook write
obs-scope-summary#21.1 · yes · the channel server dispatch spans and the corr field of channel-request and channel-response lines
obs-scope-summary#21.2 · no · frontend N/A, and the channel events never reach the terminal
obs-scope-summary#21.3 · yes · the server side's listener events and channel lines land in the run-side log file run-<name>.ndjson
obs-scope-summary#21.4 · yes · the server side logs its own process (run) on the new listener events, and the chunk never records params.sender
obs-scope-summary#21.5 · yes · from is never logged as an authenticated identity, channel lines never record params or results, and -32603 stays internal error with data null
obs-scope-summary#22.1 · no · the MCP tool spans wrap the viola-channel Tokio client, which the chunk does not touch
obs-scope-summary#22.2 · no · frontend N/A, and the MCP surface is not reached
