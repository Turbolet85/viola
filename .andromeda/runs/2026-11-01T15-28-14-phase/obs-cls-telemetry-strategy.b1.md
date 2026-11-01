# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S5 · span declaration form: `#[instrument(skip_all, name = "<area>.<operation>")]` with a static name and an explicit field allow-list
telemetry-strategy#1.2 · yes · each refusal the chunk adds (socket directory, peer euid mismatch, run start pre-child refusal, hook fail-open) is a refusal line that takes a value of the closed `event` enum
telemetry-strategy#1.3 · yes · typed errors with fixed `Display` and distinct `detail` codes on `process-exit`: the socket-directory and peer euid refusals each take a closed code, and bind_endpoint's typed refusal replaces the anyhow path
telemetry-strategy#1.4 · no · metrics are jq-derived counts and perf gates; the chunk adds no metric (§5 recipes not needed) and its holder_gone and endpoint_gone re-measures are not duration samples
telemetry-strategy#1.5 · no · `instances/<name>/events.ndjson` audit stream and its no-rotation rule; the chunk's new lines sit on process role files, not the audit stream
telemetry-strategy#2.1 · yes · the Section 10 exemption list: a refused home tree on `run` / `verify` and a refused instance check on `link` / `unlink` and the six frame verbs, each made before the role file opens, is where the new refusals must be placed
telemetry-strategy#3.1 · yes · logging-sensitive IPC vector: the euid mismatch and socket-directory refusal lines on the channel use the §6 channel field allow-list with no path or pid, and a verified euid does not authenticate `from`
telemetry-strategy#3.2 · no · paste-text vector (`send-refused` side) past connect; the chunk's refusals fall before the first frame and the send scenario past its connect stays as it stands
telemetry-strategy#3.3 · yes · hook stdin vector: a connect refusal reaches `hook` only as fail-open with `hook-decision{detail}`, so the new codes join the §6 hook-decision detail codes; the macOS gap is recorded here
telemetry-strategy#3.4 · yes · MCP vector: a client-side refusal maps to `instance-unreachable` with a `detail` inside `mcp.tool_call` with no line, so the §6 MCP field rules bind the new detail
telemetry-strategy#3.5 · yes · CLI/env vector: the run start pre-child refusal's two fixed stderr lines with no path or pid are §7 fixed messages
telemetry-strategy#3.6 · yes · filesystem vector: the socket-directory owner, mode and symlink refusal is a Unix strict-modes refusal (`strict-modes-failed`) under the home check's one rule, and a strict-modes check before an open writes no line
telemetry-strategy#3.7 · no · child-spawn vector (`process-start` subject values, `parse-rejected`); the chunk spawns no child and changes no spawn environment, and the run refusals are pre-child
telemetry-strategy#3.8 · no · standing S11 · new dependency rules: the feature set and the cargo-deny gates
telemetry-strategy#3.9 · yes · cross-surface propagation: the wrapper-side euid refusal is keyed by `conn` with no request behind it, and a refused connection is not a channel-request / channel-response pair
telemetry-strategy#3.10 · no · perf-budget instruments name only the §5 perf gates and the §10 budget table, neither of which names a holder_gone probe or endpoint_gone value the chunk re-measures
telemetry-strategy#3.11 · yes · chaos instrumentation: the force-killed cleanup measurement reads `process-exit` `exit_source` and `during`, and the new connect refusals carry `during:"connect"`
telemetry-strategy#3.12 · yes · multi-platform compat: the Unix 0600/0700 strict-modes rule the home check now applies, and the new refusal lines must read the same on Linux and macOS under the OS matrix
telemetry-strategy#3.13 · no · zero-unlogged-panics SLO; the chunk adds no panic site and no SLO value
telemetry-strategy#3.14 · no · audit-log retention (no rotation, log file location); the chunk adds no rotation and changes no log file location
telemetry-strategy#3.15 · yes · creator-explicit schema verbatim: new `detail` values and fields on home-level lines must be admitted in `schemas/diag-line.v1.json`
telemetry-strategy#3.16 · yes · Direction 4: a strict-modes refusal before any log opens writes no line and the `--json` closed `instance-unreachable` document carries `detail` `strict-modes-failed`, which the home check change reaches
telemetry-strategy#3.17 · no · installed-CLI measured counters and `parse-rejected` / `skipped` lines; the chunk adds no counter and touches neither line
telemetry-strategy#3.18 · no · no screen scraping: the vt100 gate outcome (Scenario 2) is not reached, and the chunk's new checks assert exit codes and `--json`, not a screen
