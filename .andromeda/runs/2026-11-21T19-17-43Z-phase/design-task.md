@task_prose part=chunk
Establishes, as a reading from the installed 2.1.328 and 2.1.330 claude bundles read as strings only, the MCP client's tool-call timeout: its default, the names that move it, whether a stdio server differs, whether progress resets it, and what the client does at the bound.
Bounds in code the longest block of every tool viola mcp serves (send, wait, last, answer, list) below that timeout, send's whole path and wait's caller-supplied wait among them.
Where a tool's block can exceed the bound today, closes the gap by a compiled bound, a cap on a caller's argument, or a documented split, each with a test that can fail.
Leaves to the P3 fork whether the bound stands as a relied capability-ledger row with a viola verify probe (a 30th row) or as a compiled bound with a test and a drift reading.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server's tool handlers for send, wait, last, answer and list, and their deadlines — edits: the tool handlers and their deadlines
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the MCP server behind the hidden mcp verb, the tool handlers' shared wiring and deadlines — edits: the tool handlers' deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results the handlers return, the result or error a tool gives when its deadline is reached — edits: the tool handlers' results at their deadlines
@task_item n=1 part=needs
The MCP server's five tools (send, wait, last, answer, list): each tool's longest block gets a bound in code that sits below the client's tool-call timeout.
@task_item n=2 part=needs
send's whole path as one bounded block: the client-side checks, the frame, the wrapper's wheel read, the typing and the delivery confirmation; a send cut by the bound must not read as delivered, nothing announces success before confirmation, and a send whose delivery could not be confirmed in time takes the unconfirmable readback, never an optimistic one.
@task_item n=3 part=needs
wait's caller-supplied wait: capped, or its block otherwise bounded, below the client's tool-call timeout; when the wait runs out it ends in the wait's timed-out outcome, not in a client-side tool timeout.
@task_item n=4 part=needs
A caller argument refused for exceeding a new cap: the refusal takes the existing fixed refusal and wrapper fault --json detail forms (the -32602 invalid-params class and its other-invalid-params detail), carries no internals, and quotes no caller text.
@task_item n=5 part=needs
The client's behaviour at its tool-call timeout (the call's error class; the call alone or the server torn down) as read from the installed bundles: a reading, not a measurement, and the reason every bound sits below it.
@task_item n=6 part=needs
The hidden mcp verb's stdout carries the protocol only: no new bound, timeout or cap may print a human line, hint or colour onto it; colour stays off for viola mcp in the colour decision order.
@task_item n=7 part=needs
The CLI send and wait path the MCP tools share, through viola-channel's Tokio client and the wrapper's send handling: the CLI viola send readback mirror (open, read back, unable, unconfirmable) and the viola wait output (waiting line, event form, dialog form, timed out) keep their forms when a bound is added under them.
@task_item n=8 part=needs
The fork the P3 block prices: a relied capability-ledger row with a viola verify probe (a 30th row), which would add a row to the verify output's step lines, summary and trust record, or a compiled bound with a test and a drift reading.
@task_item n=9 part=needs
Any widening of what verify or a rig would newly read, write or spawn (verify's child set) is said whole in the P3 block, with what is kept and where and the proof that no text reaches a file, stamp, fixture or log; the stamps.json update and unclean-fixture refusals stay as they are.
@task_item n=10 part=needs
The CLI verification state (verified, unverified-cli) the tools run under: a bound read from a bundle version is tied to the versions viola stamps, 2.1.328 and 2.1.330.
@task_item n=11 part=needs
No documentation leaf gains a figure the code states: the bound's value lives in code, not in a document.
@task_item n=12 part=needs
Each new test of a bound must be able to fail: it reaches the bounded block and proves the tool returns before the bound, not that a constant has a value.
@task_item n=13 part=needs
No live start of the claude CLI and no read of the operator's real home: the bundles are read as strings only, and no test or rig the chunk adds starts a live session.
@task_item n=14 part=needs
The carried intermittent rig rehearsal red (the box read empty after x and del): acted on only at its next red met during the chunk, by reading the printed rig summary and the kept after-del frame and naming the cause before the wait is mended; no retry rule, no loosened entry.
@task_standing n=1
convention: every new error, refusal or hint line is ASCII only, escapes control characters, and keeps data on stdout and messages on stderr
home: anti-patterns-never-do-these
@task_standing n=2
convention: every new refusal or error names no internals: no path, pid, stack trace or caller text in an error or hint
home: anti-patterns-never-do-these
@task_standing n=3
convention: every new failure under --json stays one clean stdout document with its fixed code, and no hint line under --json
home: surface · surface-cli
@task_item n=1 part=not_needed
The brand identity's strip-board rows (flight-progress strip fields, stable slots, cocked dialog strip, stale dimming): the chunk adds no board or strip display.
@task_item n=2 part=not_needed
The colour palette, typography, spacing, border radius, depth and iconography areas: the chunk adds no colour, weight, width, glyph or column.
@task_item n=3 part=not_needed
The web-spa surface and the rendered-page absence: viola renders no page and the chunk adds none.
@task_item n=4 part=not_needed
The viola list board and list table rows (columns, NAME cut, skip counts, row escaping, strict-modes scan): the list tool's block is bounded, its board's form is untouched.
@task_item n=5 part=not_needed
The viola run and revive start refusals, revive --json exit-1 codes and revive --list output: the chunk starts no session and changes no start path.
@task_item n=6 part=not_needed
The viola link and unlink output, pause and release output, and the handoff transfer marker: the chunk changes no control transfer or link.
@task_item n=7 part=not_needed
The strict-modes refusals and the home-under-regular-file and socket-directory refusals: the chunk adds no home or socket check.
@task_item n=8 part=not_needed
The viola --help verb groups and the plugin install output: the chunk adds no verb and no install step.
@task_item n=9 part=not_needed
The self-validation protocol's design tests (squint, swap, domain, readback box signature check): the chunk adds no visual element to judge.
