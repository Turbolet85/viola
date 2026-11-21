@task_prose part=chunk
Reads, as strings only, the client tool-call timeout of the claude CLI versions viola stamps (2.1.328 and 2.1.330): its default, the names that move it, whether a stdio server differs, whether progress notifications reset it, and what the client does at the bound.
Bounds in code the longest block of every tool the viola mcp stdio server serves (send, wait, last, answer, list) below that client timeout, send's whole path and wait's caller-supplied wait among them.
Where a tool's block can exceed the client's bound today, closes the gap by a compiled bound, a cap on a caller's argument or a documented split, each with a test that can fail.
Leaves to a priced P3 fork whether the bound stands as a 30th relied capability-ledger row with a viola verify probe, or as a compiled bound with a test and a drift reading.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola-mcp stdio server tools (send, wait, last, answer, list) — the MCP tool handlers and their deadlines — edits: each tool handler's longest block and its deadline, wait's caller-supplied wait
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp stdio server (rmcp on stdio, Tokio allowed) that serves the tool handlers and holds their deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results — isError results with their code words (invalid-params, refusal, instance-unreachable, wrapper-fault) that a tool returns when a deadline or a cap stops it
@task_item n=1 part=needs
The client's MCP tool-call timeout for the stamped claude versions, read from the installed bundles as strings: a READING, not a measurement; the environment names that move it named as names only, never values; whether progress resets it; the call's error class at the bound and whether the call alone or the whole server is torn down.
@task_item n=2 part=needs
Every viola mcp tool (send, wait, last, answer, list) has its longest block bounded by a compiled deadline that sits below the client's tool-call timeout.
@task_item n=3 part=needs
send's whole path is inside that bound: the client-side send validation (empty-text, control-character), the frame over the channel, the wrapper's wheel read (human-typing), the readiness gate, typing and the confirmation window that closes a confirmed send.
@task_item n=4 part=needs
wait's caller-supplied wait gets a cap or a compiled bound below the client timeout; a caller argument past the cap is refused as an invalid-params result carrying a closed detail code, never the value.
@task_item n=5 part=needs
A tool that stops at its own bound returns an MCP isError result carrying one closed code word; the result names no path, no text, no endpoint, and MCP rejections stay codes only.
@task_item n=6 part=needs
Each gap closed (compiled bound, argument cap or documented split) carries a test that can fail when the bound is exceeded or removed.
@task_item n=7 part=needs
The CLI send and wait path the MCP tools share through the viola-channel Tokio client and the wrapper's send handling in viola run: the wrapper-side send-refused details, the send-confirmed line, the wait.block and run.wait_dispatch span attributes and the confirm window's close.
@task_item n=8 part=needs
A new deadline-hit outcome, if logged, goes to the mcp role's file sink as a codes-only line under the closed event enum with corr set explicitly; its fields are additive to the diag-line schema; sent text, typed keys and the reply body never reach it.
@task_item n=9 part=needs
viola mcp's stdout carries MCP frames only: no telemetry, no log line and no print on stdout while a tool waits or hits its bound.
@task_item n=10 part=needs
The mcp.tool_call span around each tool call stays the span the bound lives under; spans skip all arguments and carry no text, endpoint or path.
@task_item n=11 part=needs
If the client tears a call or the server down at its bound, the mid-call disconnect is logged once, by the client side, and the channel lines still carry no params or result bodies.
@task_item n=12 part=needs
The fork's ledger arm: a 30th relied capability-ledger row stamped by a viola verify probe; the ledger's path and stamps are never logged, an unreadable ledger reads as an unverified CLI, verify stays the only stamps writer, and a drift report goes to the detail file only.
@task_item n=13 part=needs
Any probe or child verify would newly spawn joins verify's child spawn subjects and the diag-line subject enum, is said whole in the P3 block with what it keeps and where, and proves no text reaches a file, stamp, fixture or log line; real-CLI verify never runs in CI.
@task_item n=14 part=needs
A bound is a deadline, never a retry: no retry-once and no retry policy around a tool call, and tests wait on no sleep.
@task_item n=15 part=needs
The phase makes no live start and reads nothing of the operator's real claude home; the pinned product build and its re-pin step stay untouched.
@task_item n=16 part=needs
No documentation leaf gains a figure the code states: the bound's value lives in code, and a doc names it without the number.
@task_item n=17 part=needs
The carried rig rehearsal red (box read empty after x and del): acted on only if it reds during the chunk, by reading the printed rig summary and the kept after-del frame and naming the cause before mending the wait; no retry rule and no loosened entry.
@task_standing n=1
convention: each new test that runs a process writes its home under the scanned test root, its emitted diagnostics lines are schema-checked, and the zero-panics, schema and secret-scan gates run over it and fail closed
home: 9-ci-integration · ci-integration
@task_standing n=2
convention: each new log line goes through the obs_event macro with a closed event value, a closed detail code with no path or pid, and field names added to the schema, never renamed
home: 6-log-coverage · log-coverage
@task_standing n=3
convention: each new span is declared manually with a static name, skip-all arguments and an explicit field allow-list, never err or ret
home: 4-span-trace-coverage · span-trace-coverage
@task_standing n=4
convention: each new timing assertion is asserted by exit code on the maximum sample, with no sleep-based wait for a log line
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_standing n=5
convention: each new or re-stamped claude fixture is scrub-required and synthetic, and is never a telemetry source
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
The dialog hook's paths: hook invocation and decision lines, hook fail-open, hook stdout and stderr, and the hook latency and session-end budgets; the chunk changes no hook.
@task_item n=2 part=not_needed
Heartbeat ticks and liveness: the heartbeat file, the stale threshold and the list liveness reading; no tool's bound reads them.
@task_item n=3 part=not_needed
viola revive: its preflight order, its preflight and list exit details and the agents probe it runs; the chunk does not revive a session.
@task_item n=4 part=not_needed
State recovery: snapshot replay, the torn-line healing appender and the state-recovered line; the chunk writes no state.
@task_item n=5 part=not_needed
Link and unlink: the explicit link methods, their events and the implicit link record; the bound changes no link.
@task_item n=6 part=not_needed
The start of viola run: its start spans and attributes, start-failure detail codes, the pinned copy, the PTY spawn and the start's trust record; no live start is made and the product re-pin stays outside the chunk.
@task_item n=7 part=not_needed
The agent-run and harness logs commands: their merged sources, filters, records and torn-line output; the chunk adds no reader.
@task_item n=8 part=not_needed
The hyprctl probe and the claude agents probe with their parse-rejected parsers; the chunk spawns neither.
@task_item n=9 part=not_needed
Derived metrics: gauges from harness snapshots, histograms as raw samples and counters from home-level role files; the chunk adds no metric.
@task_item n=10 part=not_needed
A future error reporter: its transport, its egress entry and its opt-in switch; the chunk adds no reporter.
