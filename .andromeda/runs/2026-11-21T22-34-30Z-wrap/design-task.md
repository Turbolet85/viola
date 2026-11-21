@task_prose part=session
The session bounded every MCP frame tool call (send, wait, last, answer) by a compiled MCP call deadline of 55 s, below the claude client's tool-call bound; list is no frame tool and is not under it.
At the deadline the call's work is dropped and the tool returns the instance-unreachable result with a new cause word call-deadline, written by the MCP frame tools alone; the outcome of a send cut there is unknown, as with endpoint-lost.
The closed unreachable-cause set went from six members to seven; the CLI exit-21 cause set is unchanged at six, and the CLI gained no arm, hint or line for the new word.
The wait cap stays 50 s; its why is re-based from the retired 2.1.287 reading onto the 2.1.328 and 2.1.330 bundle readings (120 s auto-background default, a reading per version, never measured), and live starts measured a 50 s wait returning whole and a 5 s per-server timeout cutting the call.
The deadline is an injected seam in the MCP server; the server now runs a Tokio timer in production. The live rig gained a test-only MB arm beside AV that keeps no output.
The paste into the wrapped terminal stays one blocking write with no time bound: a finding carried forward, not fixed.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the closed unreachable-cause set, the cause words an instance-unreachable result or exit 21 carries
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude CLI adapter, its readings of the claude client's MCP tool-call bound per version
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the manifest of the mcp verb's server, its Tokio features
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the mcp verb's server, its tool set and the injected call-deadline seam
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP frame tools send, wait, last and answer, the wait cap, and the MCP call deadline over them
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP tool results, the instance-unreachable result and its one cause word
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the mcp verb's handler tests, the frame tools ending at the call deadline
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body contract census, the MCP instance-unreachable detail list and the CLI exit-21 list
@task_file n=9 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the live rig's reader of the claude client's MCP call lines into closed words, test-only
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig, its arms and labels, the MB arm beside AV that keeps no output, test-only
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command line, its rehearse and live words and the fake-agent option, test-only
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests, the MB arm cases
@task_item n=1 part=touches
The exit-21 cause rows (exit-21-cause and exit-21-json-cause, six members each): the shared unreachable-cause set gained a seventh member call-deadline that the CLI never writes; the CLI set stays six, so these rows stay true but now describe a subset of the closed set.
@task_item n=2 part=touches
decision/instance-unreachable-hint-per-cause: one hint per cause; call-deadline is a cause the CLI never prints, so it takes no hint, and the rule must not read as owing one.
@task_item n=3 part=touches
The refusal-hints rows for instance-unreachable causes, notably refusal-hints/endpoint-lost: call-deadline shares endpoint-lost's outcome-unknown meaning for a send but reaches only the MCP driver, never a hint line.
@task_item n=4 part=touches
contract/mcp-verb/stdout and decision/mcp-hidden-verb: the mcp verb's tool results now include a call cut at the 55 s deadline returning instance-unreachable with cause call-deadline.
@task_item n=5 part=touches
contract/cli-colour-decision-order/viola-mcp: the mcp verb's output path changed (a new result word), its colourless output unchanged.
@task_item n=6 part=touches
decision/no-success-before-confirmation and decision/no-optimistic-send-confirmation: a send cut at the call deadline reports call-deadline, never success; the wrapper's paste may still go on, so the outcome is unknown.
@task_item n=7 part=touches
The send readback rows (send-readback/unconfirmable, viola-send-mirror/unconfirmable, readback-box/unconfirmable): an MCP send cut at the deadline is a further case whose delivery is unconfirmed, reported to the driver as call-deadline rather than as a readback.
@task_item n=8 part=touches
contract/viola-wait-output/timed-out and the wait rows: the wait cap stays 50 s, now below the 55 s MCP call deadline; its why is re-based on the 2.1.328 and 2.1.330 readings, the 2.1.287 reading retired.
@task_item n=9 part=touches
decision/no-internals-in-errors and decision/no-spinner: the deadline expiry surfaces as one closed cause word, no internals, no elapsed-time counter.
@task_item n=10 part=touches
contract/peer-reply-screening rows on code words: call-deadline is a new code word of the instance-unreachable detail, written locally by the MCP server, never read from a peer reply.
@task_fact n=1 slug=surface
fact: The cause word call-deadline (an MCP frame tool call cut at the 55 s MCP call deadline) belongs to the instance-unreachable result of the mcp verb alone; the CLI never prints it, takes no exit-21 cause and no hint for it.
@task_item n=1 part=untouched
Brand identity, the strip board, the flight-progress strip and its fields: no strip, slot, cocked state or vocabulary changed.
@task_item n=2 part=untouched
Colour palette, colour tokens and colour depth: no colour, accent or SGR span added or moved.
@task_item n=3 part=untouched
Typography, spacing, iconography, glyphs and separators: no glyph, column, width or separator changed.
@task_item n=4 part=untouched
The viola list board and table rows, revive, run start, verify, link and unlink output and refusal rows: none of those verbs changed.
@task_item n=5 part=untouched
viola --help verb groups and the command names: no verb added, renamed or retired; the test-only rig words are no product name.
@task_item n=6 part=untouched
The wrapper fault detail rows and the strict-modes refusal rows: no wrapper fault code or strict-modes check changed.
@task_item n=7 part=untouched
The surface-web-spa and rendered-page absences, the anti-pattern rows on colour and status dots, and the self-validation protocol: nothing visual changed.
