@task_prose part=chunk
Bounds every tool the viola MCP server serves (send, wait, last, answer, list) in code, each tool's longest block sitting below the claude CLI MCP client's tool-call timeout.
Establishes that client timeout for the versions viola stamps as a reading from the installed bundles' strings: its default, the names that move it, stdio difference, progress-notification reset, and what the client does at the bound.
Covers send's whole path (client-side checks, the frame, the wrapper's wheel read, typing and delivery confirmation) and wait's caller-supplied wait.
Where a tool's block can exceed the client's bound today, closes the gap by a compiled bound, a cap on a caller's argument, or a documented split, with a test that can fail.
How the bound stands (a relied capability-ledger row with a viola verify probe, or a compiled bound with a test and a drift reading) is a fork left to P3.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server's tool handlers (send, wait, last, answer, list) and their deadlines — edits: each tool handler's deadline, the wait tool's timeout_ms argument
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the MCP server component that serves the tools and holds their deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP server's tool results — the fault body and refusal a tool returns, including at its deadline
@task_item n=1 part=needs
The MCP server's tools and their deadlines: every tool's longest block bounded in code and below the MCP client's tool-call timeout.
@task_item n=2 part=needs
The MCP wait tool's caller-supplied timeout_ms: capped, or otherwise held, so the caller's wait cannot carry the call past the client's bound.
@task_item n=3 part=needs
Send's confirmation and its waits: the wrapper's wheel read, typing and delivery confirmation that the send tool shares with the CLI send path through the channel's Tokio client, all inside the tool's bound.
@task_item n=4 part=needs
Send refused while the human types, and the send refusals input-not-ready and no-prompt-submitted: a bound on send's path ends in these refusals, never in waiting on or blocking the human's keystrokes.
@task_item n=5 part=needs
Time bounds fall on the driver only and never on the human: every bound this chunk adds or tightens is the driver's (the MCP caller's), with no time limit put on the person at the terminal.
@task_item n=6 part=needs
Dialog hold bounds the driver, and config.json dialog_hold_s: the answer tool's and wait tool's blocks around a pending dialog must stay below the client's bound alongside the dialog hold.
@task_item n=7 part=needs
The MCP server frame tools' refusals (socket-dir-refused, strict-modes-failed, server-verify-failed) and the socket-directory and strict-modes refusals coming before any frame or write: the client-side checks on send's path stay first and inside the bound.
@task_item n=8 part=needs
The fault body under --json and on MCP: a tool reaching its bound returns a fault or refusal body with its code word, faults carrying no hint, thiserror messages interpolating nothing, peer replies screened to code words.
@task_item n=9 part=needs
Code word format, one hint per code, no hint naming a release, and the unknown refusal carrying no hint: any new code word a bound's expiry adds follows these.
@task_item n=10 part=needs
The CLI wait and send path the MCP tools share: viola wait's waiting line and timed_out outcome, and the CLI exit codes for an unreachable instance, keep their form where the shared client's bounds change.
@task_item n=11 part=needs
The wrapper's send handling under run: zero viola bytes on the terminal and run diagnostics never reaching it, so a bound or deadline added there writes nothing to the TUI passthrough.
@task_item n=12 part=needs
Live readings never gate: the client timeout is read from the installed bundles' strings with no live start, a reading and not a measurement, and no test depends on a live claude.
@task_item n=13 part=needs
If the bound becomes a relied ledger row, viola verify's step line and its interrupted pair stay in form for the new probe; whatever verify newly reads, writes or spawns reaches no file, stamp, fixture or log as text.
@task_item n=14 part=needs
Fixed CLI strings pinned by literal and fixed copy from design: any new refusal, fault or code word text the bound adds is pinned by a literal test.
@task_item n=15 part=needs
Zero flakiness with no retries: the open intermittent rig rehearsal red is mended only after its cause is read, with no retry rule and no loosened wait, and each new bound's test can fail deterministically.
@task_standing n=1
convention: a11y assertions run inside the existing test suites, as nextest failures, with no a11y suite or driver of their own
home: 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: the CI test gate allows no skips, requires coverage and doctest, and a gate breach fails the build
home: 1-a11y-scope-summary · ci-integration
@task_standing n=3
convention: tests flake never and are never retried, and the mutation audit covers the invariants held in crate tests
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: every fixed CLI or tool string a test pins is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=5
convention: boundary tests are sequenced on records and never inspect claude's own content
home: keyboard-test-harness
@task_item n=1 part=not_needed
Colour and SGR rules (bold names, amber dialog word, dim stale rows, colour decision inputs, name truncation): the chunk adds no styled output.
@task_item n=2 part=not_needed
Screen-reader targets and setups and the manual Orca and keyboard passes: the chunk changes no read-aloud surface and runs no manual pass.
@task_item n=3 part=not_needed
viola revive and its --json refusals, revive --list, and start-against-holder refusals: the chunk touches no start or revive path.
@task_item n=4 part=not_needed
viola link and unlink, plugin install, and viola --help output: none of these surfaces is reached by a tool's bound.
@task_item n=5 part=not_needed
Absences of internationalization, a touch surface, a web or graphical frontend or a rendered page, and the not-applicable harness areas (contrast, focus management, WCAG mapping, structured violation JSON): none bears on a time bound.
@task_item n=6 part=not_needed
Keyboard navigation of the CLI and the TUI's focus, mouse and resize sequences: the chunk changes no key or terminal-event handling.
