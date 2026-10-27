# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the CI/CD platform is not product runtime, and the chunk adds no CI job or upload
obs-scope-summary#17.2 · no · the platform is not-instrumentable, so the chunk adds no instrumentation for it
obs-scope-summary#17.3 · no · fixtures are scrub data and not a telemetry source, and the chunk adds no fixture
obs-scope-summary#18.1 · yes · the forced vt100 panic witness relies on the per-process panic hook writing one panic line, and the new lines go through this JSON logger
obs-scope-summary#18.2 · no · the frontend is N/A and the chunk adds no output surface or frontend hook
obs-scope-summary#18.3 · yes · the chunk's new CLI-side lines, send-refused and list's parse-rejected, reach a resolved instance's cli log only after its check passes
obs-scope-summary#18.4 · yes · the instance field of each new CLI line is the ViolaName from an argument or VIOLA_NAME, the names validated before any path join
obs-scope-summary#18.5 · yes · the new refusals (control-character and empty-text at exit 13, invalid-params, parse-rejected) are fixed codes and strings with no paths, pids or upstream text
obs-scope-summary#19.1 · yes · the vt100 readiness-gate outcome input-not-ready must be a logged signal that the degrade witness reads
obs-scope-summary#19.2 · yes · the terminal carries the child's screen, the passthrough the chunk re-verifies after a vt100 panic
obs-scope-summary#19.3 · yes · the vt100-feed and config-json parse-rejected lines go to run-<name>.ndjson at WARN, and run writes nothing to the terminal
obs-scope-summary#19.4 · yes · the run-side lines the chunk adds carry process=run and the ViolaName validated before the path join
obs-scope-summary#19.5 · yes · the panic hook is replaced before the child spawns, so a forced vt100 panic degrade prints nothing to the child's terminal
obs-scope-summary#20.1 · yes · the hook's bounded stdin parse and its fail-open hook-decision line sit in the traced path, and a caught panic logs one panic line
obs-scope-summary#20.2 · no · the frontend is N/A and the chunk adds no frontend hook
obs-scope-summary#20.3 · yes · oversize or malformed hook stdin fails open to hook-<name>.ndjson only, with stderr empty and no body, and statusline stdout is never logged
obs-scope-summary#20.4 · yes · the hook reads stdin only after check_instance passes, so a refused or absent instance reads no stdin and writes no line
obs-scope-summary#20.5 · yes · each fail-open line is one append write, and the bounded stdin read sits in the timed path that run --perf measures
obs-scope-summary#21.1 · yes · the channel-request and channel-response lines the chunk adds carry corr as the JSON-RPC id and never record params or result bodies
obs-scope-summary#21.2 · no · the frontend is N/A and the chunk adds no frontend hook
obs-scope-summary#21.3 · yes · the over-size or invalid frame WARN lines go to each side's own process log, and a CLI verb without an instance writes none
obs-scope-summary#21.4 · yes · each side's new lines carry its own process, and the peer's params.sender is never recorded in them
obs-scope-summary#21.5 · yes · the chunk logs from as unauthenticated and answers -32603 with exactly internal error and data null
obs-scope-summary#22.1 · yes · the per-tool-call spans wrap the MCP rejections the chunk codes, codes only, with no line for instance-unreachable
obs-scope-summary#22.2 · no · the frontend is N/A and the chunk adds no frontend hook
