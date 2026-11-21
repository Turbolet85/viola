@task_prose part=chunk
Bounds every tool the hidden viola mcp server serves (send, wait, last, answer, list) so its longest block ends below the claude CLI's MCP tool-call timeout.
Establishes that client timeout for the CLI versions viola stamps, as a reading from strings of the installed bundles: its default, the names that move it, whether a stdio server differs, whether progress resets it, and what the client does at the bound.
Where a tool's block can exceed the bound today (send's whole path, wait's caller-supplied wait among them), closes the gap by a compiled bound, a cap on a caller's argument or a documented split, each with a test that can fail.
Prices at P3 how the bound stands: a relied capability-ledger row with a viola verify probe, or a compiled bound with a test and a drift reading.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola mcp tool handlers (send, wait, last, answer, list) and their deadlines — edits: each tool's handler and its deadline
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp server, which serves the tools over stdio and holds their deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the viola mcp tool results (success, refusal, invalid-params) a tool returns when its call ends or hits its bound
@task_item n=1 part=needs
The viola mcp tool set (send, wait, last, answer, list): each tool's longest block is bounded in code, and that bound sits below the client's tool-call timeout.
@task_item n=2 part=needs
The send tool's whole path is bounded end to end: the client-side checks before any request (empty text, control character), the frame, the wrapper's wheel read, the typing and the delivery confirmation with its waits.
@task_item n=3 part=needs
The wait tool's caller-supplied wait: capped or bounded so the whole call ends below the client's bound; its outcomes (turn-ended, dialog, dialog-expired, timed out) stay the ones the wait verb already reports, and turn-ended stays no send outcome.
@task_item n=4 part=needs
The last, answer and list tools: each one's longest block is read and bounded, the same as send and wait.
@task_item n=5 part=needs
A call that reaches its bound, or an argument over its cap, returns through the existing tool result shapes (refusal or invalid-params), never a new shape and never a silent hang.
@task_item n=6 part=needs
The CLI send and wait path the MCP tools share through the channel crate's Tokio client: the deadlines its calls carry, and who waits on what, are what the bound is measured against.
@task_item n=7 part=needs
The wrapper's send handling (wheel read, typing, delivery confirmation and its waits): its time bounds are counted inside the send tool's bound.
@task_item n=8 part=needs
The client's MCP tool-call timeout for the versions viola stamps (2.1.328, and 2.1.330 as the bare claude on the host): its default, the names that move it (names only), whether a stdio server differs, whether progress notifications reset it, and what happens at the bound (the error class; the call alone or the server torn down).
@task_item n=9 part=needs
That timeout is read from strings of the installed bundles only: a READING, not a measurement; no live start, and nothing of the operator's real claude home is read.
@task_item n=10 part=needs
Each gap closed (a compiled bound, a cap on a caller's argument, or a documented split) carries a test that can fail when the bound is breached.
@task_item n=11 part=needs
How the bound stands is a fork priced at P3: a relied capability-ledger row with a viola verify probe (a 30th row beside the 29), or a compiled bound with a test and a drift reading; the ledger's rows, verify's child set and the residual rows are reached only on the first branch.
@task_item n=12 part=needs
Whatever viola verify or a rig would newly read, write or spawn is a widening said whole at P3, with what is kept and where, and a proof that no text reaches a file, stamp, fixture or log.
@task_item n=13 part=needs
The MCP server keeps no human surface and keeps hiding the wheel and handoff verbs: bounding the tools adds no tool and no human output.
@task_item n=14 part=needs
No documentation leaf gains a figure the code states: a bound's value lives in the code, not in prose.
@task_item n=15 part=needs
The pinned product build stays untouched; the product re-pin stays outside this chunk.
@task_item n=16 part=needs
The carried intermittent rig-rehearsal red (box read empty after x and del): acted on only if it reds during the chunk, by reading the printed rig summary and the kept after-del frame and naming the cause before mending the wait; no retry rule and no loosened entry.
@task_standing n=1
convention: each new test proves a stated bound or cap and can fail when it is breached; it reaches no live claude start and writes no text to a file, stamp, fixture or log
home: surface-cli
@task_standing n=2
convention: a new MCP tool result or refusal uses the existing result shapes and the existing refusal cause words, with no new human output
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The viola list board and its BAY line: column widths, truncation, stale rows, rack separator and colour depth; the chunk changes no board rendering.
@task_item n=3 part=not_needed
The CLI token rendering rows (colour, spacing, radius, motion, callsign and label roles) and SGR styling: the chunk adds no styled output.
@task_item n=4 part=not_needed
The viola run and viola revive starts, their preflight, refusals and --json error codes: the chunk makes no start and changes no start path.
@task_item n=5 part=not_needed
The viola plugin install output and refusals, and the viola link and unlink output: neither is on a tool's call path.
@task_item n=6 part=not_needed
The viola --help and per-verb usage output: the chunk adds no verb and no flag to the human CLI.
@task_item n=7 part=not_needed
The wheel verb output for pause and release: those verbs are hidden from the MCP server and the chunk does not touch them.
