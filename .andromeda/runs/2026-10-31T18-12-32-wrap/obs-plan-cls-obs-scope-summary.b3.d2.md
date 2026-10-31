# Classified — obs-plan · obs-scope-summary.b3

## Answers

obs-scope-summary#17.1 · no · the entity's source is the arch excerpt, which the session does not touch, and it names no reached file
obs-scope-summary#17.2 · no · not-instrumentable is unchanged, since the session adds no telemetry source for CI/CD
obs-scope-summary#17.3 · no · fixtures stay scrub-required data and not a telemetry source; the unclean-fixture code names a cause only and no fixture name enters a code
obs-scope-summary#18.1 · no · the tracing facade, spans and JSON logger are unchanged; Cargo.toml is reached but the report shows no dependency added or bumped, so tracing-subscriber 0.3.23 still holds
obs-scope-summary#18.2 · no · the cli surface is still only terminal stdout and stderr; the new stdout document is terminal output, not a frontend hook
obs-scope-summary#18.3 · yes · contradict · exit 21's `--json` document now carries one of the five UnreachableCause words as `detail` (F11), so the `detail` null on send, wait, last, answer, pause, release, link and unlink is no longer true
obs-scope-summary#18.4 · no · the hardcoded service name, the CARGO_PKG_VERSION mechanism, the process field and the VIOLA_NAME instance are unchanged; the claim names the macro, not a version value, and no dependency is added or bumped in Cargo.toml
obs-scope-summary#18.5 · yes · add-to · the Founder Direction 4 carve-outs gain F12: the socket-directory cause has no code and macOS has no server check, beside the strict-modes carve-out
obs-scope-summary#19.1 · no · the start-sequence and viola-pty, channel and state span hooks are unchanged; the run.collision_check span outcome is unmoved
obs-scope-summary#19.2 · no · the frontend stays N/A, and run writes no terminal output
obs-scope-summary#19.3 · no · run still writes to its file sink only, passes json false, and adds no terminal write while the child runs
obs-scope-summary#19.4 · no · the service identity for run (process="run", instance=ViolaName) is unchanged
obs-scope-summary#19.5 · no · the panic hook, the 1 s heartbeat tick and child exit detection on the process handle are unchanged
obs-scope-summary#20.1 · no · the hook spans and hook-invoked and hook-decision lines are unchanged; src/cmd/hook.rs is not in the diff
obs-scope-summary#20.2 · no · the frontend stays N/A
obs-scope-summary#20.3 · no · the hook file sink, stderr ban and stdout decision body are unchanged; src/cmd/hook.rs is not in the diff
obs-scope-summary#20.4 · no · the hook service identity and the silent no-op for a refused or absent instance are unchanged; src/cmd/hook.rs is not in the diff
obs-scope-summary#20.5 · no · the append-mode single-write rule and the 1.0 s hook latency budget are unchanged, and no perf row or threshold moved
obs-scope-summary#21.1 · no · channel spans and corr propagation are unchanged; viola-channel is not in the diff
obs-scope-summary#21.2 · no · the frontend stays N/A
obs-scope-summary#21.3 · no · each side still logs to its own file sink, and the channel and MCP sinks are unchanged
obs-scope-summary#21.4 · no · each side still logs its own process and the peer version as params.sender; channel is untouched
obs-scope-summary#21.5 · no · channel ids, corr, the -32603 fixed body and the from field are unreached; viola-channel is untouched
obs-scope-summary#22.1 · no · the per-tool-call spans and the channel client wrap are unchanged; the session changes only the refusal-to-cause mapping and the isError detail, not the spans
obs-scope-summary#22.2 · no · the frontend stays N/A
