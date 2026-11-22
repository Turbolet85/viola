@task_prose part=chunk
Answers whether real claude on Linux loses a key typed right after a window resize, and fixes it if it does.
Reads where the claim came from and what viola does around a resize today: who sends the window-size change to the child's PTY, what the pump and wheel write after it, and every wait or settle bound on that path.
Reads the installed claude 2.1.328 and 2.1.330 bundles as strings only (a redraw, an input-mode reset, a debounce); this is a reading, never a measurement.
Makes no live start. It prices the cheapest live measuring start and the fix options (a settle wait after a resize, a read-back and one re-type, a refusal) in ONE operator block at the end of P3, and picks a fix only after a reading.
Re-reads the carries: the fake agent's DEL erase (read as met), the CHILD_WITHIN comment and the paste's missing time bound (each taken only if the change opens viola-pty or the paste write path).
@task_file n=1 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY seam's pump and paste handle, which write typed keys and pasted text to the child's PTY — edits: PasteHandle::paste and its blocking write_all, only if this chunk's change opens that write path
@task_file n=2 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam crate's root and its test module — edits: the test-module comment over CHILD_WITHIN that names the mutants profile's kill, only if this chunk edits the PTY seam
@task_file n=3 path=src/bin/viola-fake-agent/input.rs mark=re-reads
what: the fake agent's prompt input handling, read back to confirm that plain() erases the prompt's last character on a typed DEL, as the live CLI's input box does
@task_item n=1 part=needs
The PTY seam's resize path: how the window-size change reaches the claude child's PTY, the pump's writes after it, and any settle wait a fix adds between a resize and the next written key.
@task_item n=2 part=needs
The run pump and the wheel: what is typed into the child right after a resize, and how the send path's readiness gate and confirmation window stand when a resize lands mid-send.
@task_item n=3 part=needs
Typed keys and the PTY byte stream are never logged, and sent text never reaches telemetry: any line or span the chunk adds on the resize path or a re-type carries no key bytes and no text.
@task_item n=4 part=needs
No telemetry from screen content: a read-back of the input box that tells a kept key from a lost one decides on a closed word, and nothing read off the screen goes into a log line or span.
@task_item n=5 part=needs
Retry policies and retry-once are forbidden: the read-back-and-one-re-type option must be priced against that ban, and the intermittent rig rehearsal red gets no retry rule and no loosened wait.
@task_item n=6 part=needs
A refusal as the fix uses a closed send-refused detail word (such as input-not-ready or human-typing) and keeps the send log and span schema additive only; a new detail word is an addition to that closed list.
@task_item n=7 part=needs
PTY seam log events (spawn, paste, kill-fallback): a resize or settle event, if the chunk adds one, joins that closed set additively and carries no content.
@task_item n=8 part=needs
The pump and the resize path are hot: no hot-path instrumentation and no info logging inside the pump loop or per written key.
@task_item n=9 part=needs
The paste's missing time bound carry: one blocking write in the paste handle, taken only as a fork of the P3 block if this change opens that write path; its paste log event keeps its fields.
@task_item n=10 part=needs
The wrapped claude CLI's internals and transcripts are not a contract: what the installed bundles' strings say about a resize is a reading that may inform a fix, never a relied behaviour on its own.
@task_item n=11 part=needs
The capability ledger, only if the answer becomes a relied row: its path and stamps are never logged, an unreadable ledger reads as an unverified CLI, and only verify writes stamps.
@task_item n=12 part=needs
The live rig as a measuring arm, only if added: a real-CLI start never runs in CI, CI verifies against the fake agent only, and the live start, its cap and what verified rests on wait for the operator's word.
@task_item n=13 part=needs
The fake agent's resize and input behaviour, only if a fix or a rehearsal needs it: it must mirror the live CLI's input box, including erasing on a typed DEL.
@task_item n=14 part=needs
No read of the operator's real claude home or config: every test start runs under a test-owned user home, and environment variables are not a configuration channel for any new settle bound.
@task_item n=15 part=needs
A waiting test takes a bound under the 20 s lint and waits on an observed condition, never a sleep; no mutation run is made in this chunk.
@task_item n=16 part=needs
Recorded claude fixtures are scrub-required and are no telemetry source, if a resize fixture is captured or added.
@task_standing n=1
convention: every new integration or E2E test checks each emitted diagnostics line against the line schema, on real process output
home: ci-integration
@task_standing n=2
convention: every new test home lives under the one scanned integration-test root and is kept until the gate steps run
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: every new log line goes through the single obs_event macro with a closed event value, never a raw tracing macro or a print macro
home: 11-obs-anti-patterns-never-do-these
@task_standing n=4
convention: every new span skips all arguments and declares an explicit field allow-list with a static low-cardinality name
home: 4-span-trace-coverage · span-trace-coverage
@task_standing n=5
convention: every new test or CI step leaves the zero-panics and schema-conformance gates failing closed, with no sleep-based log waits
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The MCP stdio server and its frame tools, their call deadline, MAX_FRAME and isError results: the resize path is inside run's pump, not an MCP tool.
@task_item n=2 part=not_needed
The dialog hook and hook output streams, fail-open, hook latency and session-end/spine perf budgets: the chunk touches no hook.
@task_item n=3 part=not_needed
The viola-channel JSON-RPC protocol, peer-credential checks, socket-directory and strict-modes refusals with their exit codes: the resize path opens no socket.
@task_item n=4 part=not_needed
Revive, link and unlink, list liveness and heartbeat staleness: no lifecycle or listing verb changes.
@task_item n=5 part=not_needed
State recovery, snapshot replay and the events log's link and dialog records: the chunk writes no state.
@task_item n=6 part=not_needed
Error-reporter, Sentry, OTel exporter and network-egress absences and their future-reporter rules: nothing here reports or exports.
@task_item n=7 part=not_needed
Secret-scan hit reports, artifact retention and SHA-pinned actions: the chunk adds no upload or workflow change.
@task_item n=8 part=not_needed
The agents-probe and hyprctl-probe parsers and their parse-rejected lines: a resize reading parses neither.
