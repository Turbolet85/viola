@task_prose part=chunk
Establishes, for the claude CLI versions viola stamps (2.1.328 and 2.1.330), the MCP client's tool-call timeout: its default, the names that move it, whether a stdio server differs, whether progress notifications reset it, and what the client does at the bound.
Bounds in code the longest block of every tool viola mcp serves (send, wait, last, answer, list) below that client timeout, send's whole path and wait's caller-supplied wait among them.
Where a tool's block can exceed the client bound today, closes the gap by a compiled bound, a cap on a caller's argument, or a documented split, each with a test that can fail.
Leaves to the operator, priced in the P3 block, whether the bound stands as a relied capability-ledger row with a viola verify probe (a 30th row) or as a compiled bound with a test and a drift reading.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server's tool handlers for send, wait, last, answer and list, and their deadlines — edits: each tool handler's longest block and deadline, wait's timeout_ms handling
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp stdio server: tool registration, tools/list and the handlers' shared deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool result and its isError and refusal payloads, the result a bounded call returns
@task_item n=1 part=needs
The MCP client's tool-call timeout for the stamped CLI versions, read from the installed bundles as strings only: its default, the names that move it (names only), stdio versus other transports, progress-notification reset, and the client's action at the bound (error class; the call alone or the server torn down). A value read from strings is recorded as a reading, never as a measurement or verification.
@task_item n=2 part=needs
Every MCP tool (send, wait, last, answer, list) has its longest block bounded by a compiled bound that sits below the client's tool-call timeout; a deadline per tool handler.
@task_item n=3 part=needs
The MCP send tool's whole path stays below the client bound: the client-side checks before any frame (instance check, socket directory, server identity, strict modes), the frame over the wrapper channel, the wrapper's wheel read, the typed text, and the send confirmation window with its expiry and unconfirmable result.
@task_item n=4 part=needs
The MCP wait tool's caller-supplied timeout_ms: the wait bound is capped below the client bound, and a value past the cap is either refused as invalid-params on the timeout_ms field or clamped, the cap advertised in the tool's schema; the wait's timed_out result on expiry.
@task_item n=5 part=needs
The answer, last and list tools' blocks bounded too: answer's frame and dialog decision path, last's and list's reads, each with no unbounded wait on the wrapper or the event log.
@task_item n=6 part=needs
Each gap closed carries a test that can fail: a bounded remove-the-guard witness that turns red when the bound or cap is removed, driven by an injected clock or a forced window, not by real elapsed time.
@task_item n=7 part=needs
The MCP handlers reach the wrapper through viola-channel's Tokio client, shared with the CLI send and wait path; a bound placed there must keep CLI and MCP returning identical payloads and the MCP and CLI parity of refusals and exit-21 causes.
@task_item n=8 part=needs
What a tool returns when its bound expires is a defined MCP tool result (refusal, wrapper-fault, instance-unreachable or a timed_out ok), its refusal and detail words screened, its error payload sanitized with no path or upstream text.
@task_item n=9 part=needs
The wrapper's send handling (wheel read, readiness gate GATE_MAX_WAIT, typing, confirmation window) supplies the values the MCP send bound must sum over; the chunk reads those compiled values and does not lengthen them.
@task_item n=10 part=needs
If the bound becomes a relied capability-ledger row: a 30th relied row with its own viola verify probe, the version gate needing every relied row, stamps refusing a widened ledger, the ledger probe contract suite and the fake agent held to recorded verify fixtures.
@task_item n=11 part=needs
If the bound stays compiled: a test pinning the compiled bound below the reading, and a drift reading that names when a newer CLI's timeout falls below it, without a live start.
@task_item n=12 part=needs
No live start in this phase and nothing of the operator's real ~/.claude* read; tests never read the host's claude sessions; the real claude CLI is tested local-only, never in CI.
@task_item n=13 part=needs
Whatever verify or a rig would newly read, write or spawn is a widening stated whole in the P3 block, with the proof that no read text reaches a file, stamp, fixture or log.
@task_item n=14 part=needs
No documentation leaf gains a figure the code states: the client timeout, the per-tool bounds and the wait cap live as compiled constants with a single source.
@task_item n=15 part=needs
The pinned product (release build of a59da9da) stays untouched; the product re-pin step stays outside this chunk.
@task_item n=16 part=needs
Carry: the intermittent rig rehearsal red (box read empty after x and del) is acted on only if it reds during the chunk: read the printed rig summary and the kept after-del frame, name the cause, then mend the wait; no retry rule and no loosened wait.
@task_item n=17 part=needs
Dialog timeouts outlast the longest dialog hold; a bound the chunk puts on answer or wait must not cut below a hold the dialog path relies on.
@task_standing n=1
convention: each new test is named by the test function naming rule, runs under nextest, uses no retries and no sleep synchronisation, and waits key on logged events
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test oracle is a compiled literal, never a Debug string or a recorded fixture's internals
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: each new time-bound test drives time through the injected clock seam, never test-owned real time
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=4
convention: each new MCP stdio test owns its server child, starts it with inherited env dropped, and runs in a fresh private home of its own
home: 6-e2e-test-strategy · e2e-test-strategy
@task_standing n=5
convention: each new integration test runs on a real endpoint, real files and a fresh home per test, its wrapper held in a drop guard
home: 5-integration-test-strategy · integration-test-strategy
@task_item n=1 part=not_needed
Hook performance budgets and hyperfine timing of the spine hooks: the chunk bounds MCP tool calls, not hook execution deadlines.
@task_item n=2 part=not_needed
revive refusal codes, revive human and JSON output, and run and revive start refusals: the chunk changes no start path.
@task_item n=3 part=not_needed
The fake agent's command-line flags, receipts, screens and turn scripts: the chunk adds no fake-agent behaviour.
@task_item n=4 part=not_needed
Harness pre-push, supervisor, cleanup, status and gate document contracts: the chunk changes no harness command.
@task_item n=5 part=not_needed
viola list human board columns and control-character escaping in list rows: the list tool's output stays as it is.
@task_item n=6 part=not_needed
PTY passthrough, vt100 feed and outer-PTY rendering rows: the chunk touches no screen or passthrough path.
@task_item n=7 part=not_needed
Windows and macOS support, a browser suite, a database and outbound HTTP absences: nothing here reaches them.
@task_item n=8 part=not_needed
Heartbeat staleness classification and instance liveness rows: the chunk changes no liveness reading.
