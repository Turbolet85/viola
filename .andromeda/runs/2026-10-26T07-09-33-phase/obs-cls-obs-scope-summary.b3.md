# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · a provenance line for the CI/CD Platform scope entity, stating no rule the chunk's code or tests honour; the chunk adds no CI job
obs-scope-summary#17.2 · no · the CI/CD Platform's not-instrumentable classification; the chunk adds no CI telemetry and no CI job
obs-scope-summary#17.3 · no · the not-product-runtime reason for CI and fixtures; the chunk adds no fixture and no telemetry from one
obs-scope-summary#18.1 · yes · the CLI verbs' dispatch spans and the client spans the verification adds (send, wait, last, answer, pause, release, hook, channel) run through this tracing facade and JSON logger
obs-scope-summary#18.2 · no · frontend N/A; the CLI has no frontend hook to instrument, and its exit-21 stderr string is bound by 18.5
obs-scope-summary#18.3 · yes · the exit-21 process-exit line and its detail go to the file sink (cli-<VIOLA_NAME>.ndjson when an instance resolves), never to stdout, which carries the --json result
obs-scope-summary#18.4 · no · the service identity every line already carries; the chunk adds no process role or identity source
obs-scope-summary#18.5 · yes · each exit-21 cause gets a distinct closed detail code (Founder Direction 4), the exit code is a log field, and the refusal stays a fixed string with no path, pid or start time
obs-scope-summary#19.1 · yes · the instrumentation the chunk adds in viola-channel's sync client and in viola-state must stay sync (std threads, no Tokio); the run start-sequence, seam and vt100 spans are not reached
obs-scope-summary#19.2 · no · frontend N/A for run; the terminal carries the child's screen, which the chunk does not touch
obs-scope-summary#19.3 · no · standing S7 · no telemetry to stdout on any role, a convention the task lists
obs-scope-summary#19.4 · no · the service identity as for the short-lived verbs with process run; the chunk adds no run line and no identity change
obs-scope-summary#19.5 · no · the panic-hook replacement and the child's handle-wait exit concern run's child, which the chunk does not reach; the heartbeat writer is unchanged
obs-scope-summary#20.1 · yes · the hook.event and hook.dialog clients gain the verification before their channel call, inside the stdin-parse to decision-emit spans; hook-invoked and the fail-open hook-decision line are the lines it requires
obs-scope-summary#20.2 · no · frontend N/A for the hook; its fail-open silence on stdout and stderr is carried by 20.3 and S7
obs-scope-summary#20.3 · yes · the fail-open hook-decision and hook-invoked lines go only to hook-<name>.ndjson, one append write each, with stdout and stderr empty on the exit 0
obs-scope-summary#20.4 · yes · process=hook and the missing-VIOLA_NAME silent no-op bind the verification: with no instance the hook reads no endpoint and writes no line
obs-scope-summary#20.5 · yes · the fail-open line is one append write, the added peer-credential and start-time reads stay inside the hook's max under 1.0 s, and no instance means no log and no stderr
obs-scope-summary#21.1 · yes · the client-side spans on the channel call and the corr field on channel-request and channel-response bind the sync and Tokio client connects; channel params gain no trace-context field
obs-scope-summary#21.2 · no · frontend N/A for the channel
obs-scope-summary#21.3 · yes · the client's channel lines land in the caller's own process log (hook-<name>, mcp or cli-<name>), and none without an instance
obs-scope-summary#21.4 · no · each side logs its own process, the identity every new line keeps; the peer version params.sender is read by no chunk code
obs-scope-summary#21.5 · yes · corr is not unique across connections, so the conn discriminator the chunk adds to every channel call it makes is the additive field the tests contract allows; params are never recorded, so from is never logged
obs-scope-summary#22.1 · yes · the per-tool spans around the viola-channel Tokio client bind the send, wait, last and answer pre-check and its codes-only mcp.ndjson line
obs-scope-summary#22.2 · no · frontend N/A; stdio has no frontend hook and no HTTP auto-instrumentation to add
