@task_prose part=chunk
An API-error turn end names its cause from a closed list of nine words: the turn-ended event's closed cause set grows from the one word api-error to rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit and api-error.
The Claude adapter keeps classifying a StopFailure's error word as it does today; only the resulting word now crosses into the turn-ended cause, and so into the event log, viola wait --json and the MCP wait tool.
api-error stands for the CLI's unknown, an absent error and any value outside the map; the CLI's error text still reaches no event, snapshot, log line, reply or diagnostics line.
Every consumer that accepts exactly api-error today accepts the nine and still refuses any other text; the plan says how a new CLI error value is noticed later (a count or a closed log word, never its text) or that it is not.
P3 re-measures where the word is produced, every consumer that matches on it, and each known CLI error value as measured (a recorded payload) or read from source; no live claude start and no mutation run.
@task_file n=1 path=src/cmd/run.rs mark=edits
what: the wrapper's appended-line check that admits a turn-ended event only with its cause absent or a closed turn-end cause word — edits: the appended-line check's admitted cause set
@task_item n=1 part=needs
The turn-ended event data's cause field: a closed value set owned by the core crate, growing from one word to nine; the event-data keys of turn-ended and the event line that carries it.
@task_item n=2 part=needs
The Stop hook path for an API-error turn end: the StopFailure payload's error word is classified inside the Claude adapter alone, the nine-way map with unknown, an absent value and any unmapped value reading api-error; no Claude shape leaves the adapter.
@task_item n=3 part=needs
Error text never crosses a surface: the CLI's last assistant message, error details and raw error value reach no event, snapshot, process log line, diagnostics line, CLI reply or MCP reply other than as one of the nine closed words (errors sanitized at every surface, no upstream text in error bodies, logs carry no secrets).
@task_item n=4 part=needs
The wrapper's check of an appended event line: a turn-ended line is admitted with cause absent or one of the nine words and any other cause text is refused.
@task_item n=5 part=needs
Readers of the event log: the tolerant, forward-compatible event log reader accepts the nine cause words and still refuses or counts any other text, never panicking.
@task_item n=6 part=needs
viola wait and the MCP wait tool: the wake on turn-ended carries the cause word in the --json document and the MCP result, the two payloads identical between CLI and MCP.
@task_item n=7 part=needs
The board and viola list (human board and --json): wherever the turn-end cause is shown or carried, the nine words are accepted, other text refused, and human output keeps escaping control characters.
@task_item n=8 part=needs
The contract tests that pin the one word api-error: their literal tables grow to the nine words, each word asserted as a literal, and a word outside the nine asserted refused.
@task_item n=9 part=needs
A closed enum changes only by a recorded decision: the growth of the cause set is a founder's ruling, no word added, renamed or dropped beyond the nine, and no new event kind minted.
@task_item n=10 part=needs
Notice of an unmapped CLI error value: if the plan marks it, it is a count or a closed codes-only log word, never the value's text; otherwise the plan states plainly it is not noticed.
@task_item n=11 part=needs
Proof rests on the fake agent and the recorded hook payloads: the fake agent fires StopFailure with each error value, its hook stdin payloads match the recorded fixtures per CLI version, and recorded payloads are walked by the one run-time test; no real claude CLI in CI and no live reading as proof.
@task_item n=12 part=needs
Hook behaviour on a StopFailure payload: the hook fails open, tolerates unknown payload fields, and an absent or unparseable error value reads api-error rather than failing the hook.
@task_item n=13 part=needs
The classification is a pure function tested directly with table-driven unit cases, one case per known CLI error value and the fall-back cases, oracles written as literals.
@task_item n=14 part=needs
Bounded tests: waits key on logged events with bounded waits under the wait bound, no sleeps or retries, controls named with a timeout and ids of at most 16 characters.
@task_item n=15 part=needs
Fixtures for the new cases are synthetic, scrubbed and carry no real prompt, error text or user data; a fixture holding CLI error text never surfaces that text in an assertion failure or log.
@task_standing n=1
convention: each new test function follows the test function naming rule and unit tests stay inline in the module they test
home: 2-test-strategy
@task_standing n=2
convention: each new integration or E2E test binary is prefixed by the surface it tests and runs under nextest without retries or sleep synchronisation
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: each new test creates its own private home and seeds its own data; product state is produced by viola, never hand-written
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=4
convention: each new fixture or synthetic input is free of real data, scrubbed and carries the canary
home: e2e-test-strategy
@task_standing n=5
convention: each new test asserts on the contract with literal oracles, never on a Debug string or fixture internals, and never parses the rendered child screen
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: each new log line is a codes-only structured JSON line, its detail kept apart, carrying no upstream text
home: log-format
@task_item n=1 part=not_needed
Dialog decision bodies, dialog hold and expiry, and answer mapping: the chunk changes no dialog.
@task_item n=2 part=not_needed
Send confirmation, send refusal order, typed framing, local-command post-conditions and the wheel: the chunk changes no send path and no wheel rule.
@task_item n=3 part=not_needed
Link and unlink methods, their records and names: the chunk touches no link.
@task_item n=4 part=not_needed
Revive and run start refusals, start exit-1 causes, pinned-exe hash and squatted endpoints: the chunk changes no start path.
@task_item n=5 part=not_needed
Socket-directory, strict-modes and server-identity refusals and the instance-unreachable causes: a different closed word set, untouched by this chunk.
@task_item n=6 part=not_needed
Performance budgets and hyperfine timing of hooks: the chunk adds no hook work that a budget times.
@task_item n=7 part=not_needed
Harness boot, cleanup, status, supervisor and pre-push contracts, and the agent-run status shape: the chunk changes no harness command.
@task_item n=8 part=not_needed
Mutation run verdicts, thresholds and mutants kill lines: no mutation run in this chunk.
@task_item n=9 part=not_needed
Fuzz corpus replay, nightly fuzzing and fuzz toolchain: the chunk adds no parser surface to fuzz.
@task_item n=10 part=not_needed
Heartbeat staleness, liveness classification and the instance snapshot's pid, endpoint and start fields: unchanged.
@task_item n=11 part=not_needed
CI artifacts, workflow permissions, SHA-pinned actions and the release build: the chunk changes no workflow.
@task_item n=12 part=not_needed
Terminal passthrough, outer PTY, resize and vt100 panic containment: no screen or terminal change.
@task_item n=13 part=not_needed
Windows and macOS support, a database, outbound HTTP and a browser suite: absences the chunk does not reach.
