# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surface entries over roles and product parts; the chunk adds refusal lines inside the four existing roles and adds no role or part
obs-scope-summary#1.2 · no · a count of instrumentable crates; the chunk adds no crate and its reached files sit in crates already counted
obs-scope-summary#1.3 · yes · a count of must-trace paths and edge flows; the new socket-directory and peer-euid refusals are new edge flows the count must record
obs-scope-summary#1.4 · yes · a count of telemetry triggers; the strict-modes trigger list and the multi-OS compat trigger (macOS 0755 case) are reached by the new refusals
obs-scope-summary#2.1 · no · the diagnostics root layout; the chunk changes no diagnostics root and writes only run, hook and home-level lines
obs-scope-summary#2.2 · no · mcp file sink and stderr fallback; the MCP refusal maps to isError with no line and no stderr, so the fallback is not reached
obs-scope-summary#2.3 · no · release-from-driver and chaos event values; the chunk adds no event value and the wheel and chaos scenarios are not moved
obs-scope-summary#2.4 · no · hook-invoked corr timing and dialog_id lifecycle; the Not-needed list excludes them and only the hook connect is reached
obs-scope-summary#3.1 · no · provenance line for the anyhow crate; carries no rule of its own
obs-scope-summary#3.2 · yes · instrumentable verdict for the crate holding bind_endpoint; its create_private_dir refusal is a new typed refusal the verdict covers
obs-scope-summary#3.3 · yes · anyhow error-capture boundary and fixed-message error; the scope names bind_endpoint's create_private_dir failure replaced by a fixed-message error at the crate seam
obs-scope-summary#4.1 · no · provenance line for viola_core; carries no rule of its own
obs-scope-summary#4.2 · no · not-instrumentable verdict for viola_core; no viola_core file is reached and the chunk adds no instrumentation there
obs-scope-summary#4.3 · yes · the exit-21 cause word set and the rule that a refused strict-modes check writes no line; the home-check strict-modes refusal in scan joins this set and writes no line
obs-scope-summary#5.1 · no · provenance line for the PTY seam; carries no rule of its own
obs-scope-summary#5.2 · no · PTY seam verdict; the chunk touches no PTY pump or screen code
obs-scope-summary#5.3 · no · PTY byte stream never logged; the chunk reads no PTY content and its new lines carry no stream
obs-scope-summary#6.1 · no · provenance line for the IPC crate and its method list; carries no rule of its own
obs-scope-summary#6.2 · yes · instrumentable verdict for the IPC client crate; client::refuse_untrusted and the new euid and socket-directory checks sit in it
obs-scope-summary#6.3 · yes · instrumentation must be sync with Tokio only behind its feature; the new client refusal must be sync and the Tokio client check stays behind that feature
obs-scope-summary#7.1 · no · provenance line for the ndjson and sysinfo crates; carries no rule of its own
obs-scope-summary#7.2 · no · instrumentable verdict for the ndjson writer; the chunk adds no sink or exporter and the existing writer serves the new lines unchanged
obs-scope-summary#7.3 · no · standing S11 · a new dependency stays Tokio-free in the sync crates, adds no C-building crate and passes cargo deny
obs-scope-summary#8.1 · no · provenance line for the serde_path_to_error drift reports; carries no rule of its own
obs-scope-summary#8.2 · no · instrumentable verdict for the drift-report crate; the diag-line schema admission is reached through the Needs entry, not this verdict
