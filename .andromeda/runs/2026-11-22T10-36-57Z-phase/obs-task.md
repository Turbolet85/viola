@task_prose part=chunk
Bounds the wrapper's paste write into the child's PTY by a compiled time bound, so a send never waits without an end on a child that stops reading.
At the bound the send ends with a closed word the driver reads, on the CLI send and on the MCP send tool, never upstream text, and the wheel is not moved.
First measures what the code does today: a fake agent that stops reading its PTY, its buffer full, blocking the wrapper's send past the readiness gate's and the confirm window's bounds.
Reads the bound's value against send's own compiled total, the readiness gate, the confirm window and the MCP call deadline, the arithmetic shown, and disposes what a cut paste leaves in the child's input box and what the human's keys do while a paste is blocked and after it is cut.
@task_file n=1 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY seam's pump, the wrapper's paste write into the child and its write contract — edits: the one blocking paste write with no time bound
@task_file n=2 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam crate and its tests — edits: the test-module comment over the child-exit bound naming the mutants profile's kill
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's send path in the run pump: readiness gate, flight lock, paste and confirmation window, and its outcome when the paste is cut
@task_file n=4 path=src/run/wheel.rs
what: the wheel, which a cut paste leaves unmoved, and what a human key may be held behind while a paste is blocked
@task_file n=5 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola-mcp stdio server's frame tools — edits: the send tool's result at the paste bound
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP tool result shapes, the isError result and its closed words
@task_file n=7 path=crates/viola-core/src/lib.rs
what: the closed refusal, detail and cause word sets, reached only if a new closed word is added
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent, the test child — edits: a behaviour that stops reading its PTY
@task_item n=1 part=needs
The PTY seam's paste log event and the seam's write contract: the paste now ends at a compiled bound, and how a cut paste is recorded on the seam.
@task_item n=2 part=needs
The wrapper-side send outcome at the bound: a closed detail word on the send line and the process-exit detail, never upstream text, in the send-refused or unconfirmable send-line shape the plan holds; the send never presumed delivered.
@task_item n=3 part=needs
The CLI send's exit code and its stderr pair at the bound, and the wrapper-fault versus refusal classification of a cut paste.
@task_item n=4 part=needs
The MCP send tool's result at the bound: an isError result carrying one closed word, codes only, distinct from the call-deadline expiry the driver reads today.
@task_item n=5 part=needs
Any new closed word added to the detail, refusal or cause sets is a closed enum value, additive to the diagnostic line schema and its event and detail enums, with no word in the schema no build can emit.
@task_item n=6 part=needs
The confirmed send span chain and its span attributes: the readiness gate, the paste and the confirm window, their window and byte-count attributes, and what the chain shows when the paste is cut.
@task_item n=7 part=needs
The send path log events' order and their corr: send-issued, then send-confirmed or the bound's outcome, correlated with the channel lines by corr, conn and rpc id.
@task_item n=8 part=needs
The wheel's log line and product event: at the bound the wheel is not moved, so no wheel transition, pause or release line is written for a cut paste; the human-typing refusal and what a human key is held behind while a paste blocks.
@task_item n=9 part=needs
Time bounds and who waits: the paste bound plus the rest of send's compiled total stays under the MCP call deadline, read against the readiness gate's and confirm window's bounds.
@task_item n=10 part=needs
Sent text and the PTY byte stream are never logged, nor typed keys: the cut paste, its bytes and what it leaves in the child's input box reach no log line, span field or detail code beyond a byte count the plan already allows.
@task_item n=11 part=needs
Detail codes carry no path, pid or text, and error messages interpolate nothing: the bound's word is a fixed closed code with a fixed message.
@task_item n=12 part=needs
No retry: a cut paste is not retried by the wrapper or the client, and a test's red is never re-run for green.
@task_item n=13 part=needs
CI verify runs against the fake agent only and a real-CLI verify never runs in CI: the measuring child is the fake agent that stops reading, with no live or rig start of a real claude.
@task_item n=14 part=needs
Third-party PTY crate log targets stay off: the bound is observed at viola's own call site on the seam, not through the PTY library's internals.
@task_item n=15 part=needs
Run worker threads stay inside catch_unwind and a run worker panic ends the session: a thread or timer the bound adds to the pump runs under the same catch site.
@task_item n=16 part=needs
The send refusal is logged by each deciding side: the wrapper's line and the client's or MCP server's process-exit line each carry the bound's word once, the mid-call disconnect logged once by the client.
@task_standing n=1
convention: each new integration or E2E test that starts a viola process runs under a test-owned home under the scanned test-home root, kept until the gates run
home: ci-integration
@task_standing n=2
convention: each new test waits on a condition with a bound under the lint, never a sleep-based log wait, and asserts budgets by exit code
home: 10-slo-invariants-telemetry-budgets
@task_standing n=3
convention: each line a new test's process emits is schema-checked against the diagnostic line schema in the integration and E2E tests
home: 9-ci-integration
@task_standing n=4
convention: each new log line goes through the obs event macro with a closed event value and corr where correlated, no raw tracing or print macro
home: 11-obs-anti-patterns-never-do-these
@task_standing n=5
convention: each new span skips all arguments with an explicit field allow-list and a static low-cardinality name, never err or ret options
home: 4-span-trace-coverage
@task_standing n=6
convention: each new OS-gated test joins the OS-gated test census
home: 9-ci-integration
@task_item n=1 part=not_needed
Dialog hook rows: hook invocation, hook decision, fail-open codes, hook stdout and stderr streams and hook perf budgets; the chunk touches no hook.
@task_item n=2 part=not_needed
Dialog raising and answering, the answer path and its refusals, the dialog hold and expiry; the chunk changes only the send's paste.
@task_item n=3 part=not_needed
Link and unlink methods, implicit link records and link names; the chunk adds no link behaviour.
@task_item n=4 part=not_needed
Revive start and revive list, their preflight order and detail codes, and the session-live check; the chunk changes no start.
@task_item n=5 part=not_needed
The run start's collision check, pinned copy, version gate, trust record and start span attributes; no live start and no start path change.
@task_item n=6 part=not_needed
viola verify's spawn subjects, stdout step and summary lines and stamps writing, and the claude agents probe and its parse failures; the chunk measures with the fake agent only.
@task_item n=7 part=not_needed
Heartbeat ticks, liveness and staleness thresholds, list output and agent-run status; the chunk does not change liveness.
@task_item n=8 part=not_needed
Snapshot replay, the state-recovered line, torn-line healing and the event stream's rotation; the chunk writes no state record of its own.
@task_item n=9 part=not_needed
The wait and last tools and their wait-path log events and spans; the bound is on send alone.
@task_item n=10 part=not_needed
Future error reporter rows, Sentry, OTel exporters, network egress and hosted deployment absences; the chunk adds no exporter or reporter.
@task_item n=11 part=not_needed
Metric derivation, gauges, histograms and quantile reports; the chunk adds no metric.
@task_item n=12 part=not_needed
Service identity, home resolution order and the diagnostics file paths and modes; the chunk opens no new log file.
