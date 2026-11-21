@task_prose part=session
Every MCP frame tool call (send, wait, last, answer) now ends at a compiled 55 s MCP call deadline; at the deadline the call returns the instance-unreachable result with the new cause word call-deadline, and list stays outside the deadline.
The closed unreachable-cause set grew from six members to seven (call-deadline), written by the MCP frame tools alone; the CLI exit-21 cause set is unchanged at six.
The MCP wait cap stays 50 000 ms; its reason is re-based from the claude 2.1.287 reading to the 2.1.328 and 2.1.330 bundles' client bound (read as 120 s auto-background, never measured), so the order is wait cap below call deadline below client bound.
Measured live on a new test-only rig arm MB: on claude 2.1.330 and 2.1.328 a wait with no timeout_ms returned whole at the 50 s cap; under a 5000 ms per-server timeout the client cut the call at 5 s and viola mcp outlived the cut call.
Finding, not fixed: the deadline bounds only the driver's wait; the wrapper's paste into the PTY stays one blocking write with no time bound, and at the deadline the wrapper's send goes on with its outcome unknown, as with endpoint-lost.
The viola-mcp adapter now runs a Tokio timer for the production deadline, with the deadline injected as a seam in tests (no paused Tokio clock).
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the closed unreachable-cause set (instance-unreachable detail words), edited to add the call-deadline member and its case row
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent adapter, edited to name the claude MCP client's tool-call bound as a reading of the 2.1.328 and 2.1.330 bundles
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the MCP server crate's manifest, edited to enable the Tokio time feature
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the MCP server's public surface, edited to export the MCP call deadline and the injected deadline seam
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP server frame tools (send, wait, last, answer) and the MCP wait tool's timeout_ms cap, edited so each frame tool call ends at the MCP call deadline with instance-unreachable call-deadline
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP tool results, the instance-unreachable error body and its detail, edited to carry call-deadline
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the MCP server's handler tests, edited with the cases that each frame tool ends at the call deadline and a reply before it is the reply
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body census, the witness of each error body's writer, edited so the MCP instance-unreachable row's detail list names call-deadline
@task_file n=9 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the live rig's reader of the claude MCP client's call lines into closed words, a new test-only module
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig and its arms, edited to add the MCP bound arm MB beside the AV arm
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig CLI, edited with the fake option and the mb and mc rehearsal words
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests, edited with three MB arm rehearsal cases
@task_item n=1 part=touches
decision/time-bounds-fall-on-the-driver: gains the MCP call deadline (55 s) as one more time bound, one that falls on the driver alone.
@task_item n=2 part=touches
decision/time-bounds-never-on-human: gains the MCP call deadline as a bound that never falls on the human; the human at the wrapped TUI is never timed by it.
@task_item n=3 part=touches
contract/mcp-wait-tool/timeout-ms: the 50 000 ms cap is unchanged in value, but its reason is re-based off the claude 2.1.287 reading to the 2.1.328 and 2.1.330 client bound and the 55 s call deadline above it; any cell naming 2.1.287 or a 60 s smallest default is stale.
@task_item n=4 part=touches
the instance-unreachable detail rows (contract/instance-unreachable-detail/*): the MCP instance-unreachable detail list gains a seventh word, call-deadline; any row stating the whole MCP list or its count of six is stale.
@task_item n=5 part=touches
contract/mcp-frame-tool/*: the MCP server frame tools now also end in instance-unreachable call-deadline when the call outlives the deadline, beside the socket-dir, strict-modes and server-verify refusals.
@task_item n=6 part=touches
decision/error-bodies-have-writer-and-witness: the new call-deadline detail has its writer (the MCP frame tools) and its witness (the error-body census row and the handler deadline cases).
@task_item n=7 part=touches
contract/code-word/format and decision/codes-beside-plain-word: call-deadline is a new kebab-case code word carried beside instance-unreachable.
@task_item n=8 part=touches
decision/no-unwarned-timeouts and absence/user-time-limit: indirect, a new timeout on the MCP call exists, ending in a closed result word to the driver, with no time limit placed on the user.
@task_item n=9 part=touches
decision/dialog-hold-bounds-the-driver: indirect, a second driver-side bound now sits beside the dialog hold; an answer call is under the MCP call deadline.
@task_item n=10 part=touches
decision/send-refused-while-human-typing and decision/human-keystrokes-never-blocked: indirect, at the deadline the driver reads call-deadline while the wrapper's paste goes on unbounded, its outcome unknown; the paste's missing time bound is a finding carried, not fixed.
@task_item n=11 part=touches
decision/live-readings-never-gate: the new live rig MB arm's readings (client bound behaviour on 2.1.330 and 2.1.328) are live readings; the default client bound stays a reading per version, not a gate.
@task_item n=12 part=touches
decision/unlisted-code-words-stay-visible and decision/peer-replies-screened-to-code-words: indirect, the closed cause-word set they screen against grew by call-deadline.
@task_fact n=1 slug=cognitive-accessibility
fact: The MCP instance-unreachable detail call-deadline: a frame tool call (send, wait, last, answer) that outlives the 55 s MCP call deadline ends with instance-unreachable and detail call-deadline, the outcome of a send unknown; written by the MCP frame tools alone, never by the CLI (its exit-21 set stays six).
@task_fact n=2 slug=cognitive-accessibility
fact: The MCP call deadline orders the bounds: the wait cap (50 s) sits below the call deadline (55 s), which sits below the claude client's tool-call bound as read from its bundles, so the longest wait returns whole to the driver.
@task_item n=1 part=untouched
Colour, SGR, bold, dim and amber rows and the visual design area: no printed output or styling changed.
@task_item n=2 part=untouched
The viola list board, its captions, NAME truncation and the unwrapped separator: no board output changed.
@task_item n=3 part=untouched
The TUI passthrough contracts, the keyboard boundary and the wheel rules: no terminal byte, keystroke classification or wheel behaviour changed.
@task_item n=4 part=untouched
The CLI exit-21 refusal and its human and --json documents, and the CLI exit-code rows: the CLI writes the same six causes and gained no arm.
@task_item n=5 part=untouched
Revive, link, unlink, plugin install, verify and help contracts: none of these verbs changed.
@task_item n=6 part=untouched
Screen-reader targets and manual Orca and keyboard passes: nothing in them changed.
@task_item n=7 part=untouched
A11y CI integration, the gate rows, flakiness and the a11y suite placement: no CI workflow or gate threshold changed.
@task_item n=8 part=untouched
The scope absences (web or graphical frontend, touch surface, internationalization, rendered page): no capability added there.
