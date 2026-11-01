# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S5 · the instrumented-span convention (`skip_all`, static name); the chunk adds no span, and the seams it reaches keep their existing declared spans
telemetry-strategy#1.2 · yes · each refusal the chunk adds (socket directory, peer euid mismatch, connect-time refusals) is a boundary refusal that must be logged as a closed `event`
telemetry-strategy#1.3 · yes · the new refusals take distinct `detail` codes on `process-exit`, the rule that every cause behind exit 21 and exit 1 has its own code
telemetry-strategy#1.4 · no · the chunk adds no in-process instrument or metric; jq-derived counts of existing `detail` values are a re-check every change owes
telemetry-strategy#1.5 · yes · the new refusal lines in `run-<name>.ndjson` and `hook-<name>.ndjson` must not copy `events.ndjson` payloads into process logs
telemetry-strategy#2.1 · yes · the new refusals before the role file opens (frame verbs, `link`/`unlink`, `list`, `run` start, `hook`) must be recorded in the Section 10 exemption list
telemetry-strategy#3.1 · yes · the peer euid check and the channel-level refusal lines sit on the IPC surface and must use the §6 channel field allow-list
telemetry-strategy#3.2 · no · paste text and `send-refused` side apply past the connect; the chunk's connect refusal precedes any frame and carries no paste text
telemetry-strategy#3.3 · yes · the `hook` connect refusal is fail-open with a `hook-decision` fail-open `detail`, and the macOS server-verify gap (no peer pid compared) is recorded here
telemetry-strategy#3.4 · yes · the MCP frame tools' client-side refusal maps to `instance-unreachable` with a `detail` and no line, inside `mcp.tool_call`, covering Tokio and sync clients
telemetry-strategy#3.5 · yes · the two fixed stderr lines for a refused socket directory at `run` start are CLI fixed messages, with no path or pid
telemetry-strategy#3.6 · yes · the socket directory's owner/mode/symlink check and the home check under Unix strict modes are filesystem refusals in the `strict-modes-failed` catalogue, with the §10 no-line rule before any open
telemetry-strategy#3.7 · no · the chunk spawns no child and changes no child environment; `process-start` subject values and `parse-rejected` are not reached
telemetry-strategy#3.8 · no · standing S11 · a new dependency stays Tokio-free in the sync crates, adds no C-building crate and passes `cargo deny`; the chunk names no dependency
telemetry-strategy#3.9 · yes · the wrapper-side refusal is keyed by `conn` with no request behind it, so it is not a `channel-request`/`channel-response` pair (corr + conn propagation)
telemetry-strategy#3.10 · no · the chunk adds no hyperfine-gated or budget-table measurement; its measurements are the `holder_gone` probe and the cleanup reading
telemetry-strategy#3.11 · yes · the connect-time `process-exit` shape (`during:"connect"`, `exit_source`) and the force-kill cleanup measurement on Unix are chaos-instrumented lines
telemetry-strategy#3.12 · yes · the Unix 0600/0700 socket-directory and home strict-modes checks must read the same on Linux and macOS, with the OS matrix leg kept in the live Linux run
telemetry-strategy#3.13 · no · the chunk adds no panic path; zero-unlogged-panics is a re-check every change owes
telemetry-strategy#3.14 · no · no log rotation or retention is changed; `events.ndjson` is not touched
telemetry-strategy#3.15 · yes · a new `detail` value or home-level line field must be admitted by `schemas/diag-line.v1.json` for the conformance gate
telemetry-strategy#3.16 · yes · the socket-directory refusal and the home-tree refusal before any process log opens write no line, and under `--json` carry the closed `instance-unreachable` or `strict-modes-failed` document
telemetry-strategy#3.17 · no · no installed-CLI counter or `parse-rejected`/`skipped` line is measured by the chunk
telemetry-strategy#3.18 · no · no screen scraping or vt100 gate outcome is reached; the chunk touches no screen output
