@task_prose part=session
The session bounds every viola mcp frame tool call (send, wait, last, answer) by a compiled 55 s call deadline; at the deadline the call ends with the instance-unreachable result, its detail the new cause word call-deadline.
The closed unreachable-cause set grows from six members to seven (call-deadline), written by the MCP frame tools alone; the CLI exit-21 cause set and its hints stay six, unchanged.
The wait cap stays 50 000 ms; its why is re-based off the old claude 2.1.287 reading onto what the 2.1.328 and 2.1.330 bundles read (a 120 s auto-background default) and what was measured live.
Measured live: on 2.1.330 and 2.1.328 a wait with no timeout_ms returned whole at the 50 s cap; under a 5000 ms per-server timeout the client cut the call at 5 s with its hard-limit fragment.
The test-only live rig gains an MB arm (labels MB, MB8, MC and spares) that keeps no stdout byte, plus a fake-agent option and the mb and mc rehearsal words.
Found, not fixed: the wrapper's paste write has no time bound; at the MCP deadline the wrapper's send goes on and the driver reads call-deadline with the outcome unknown.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the closed unreachable-cause set behind instance-unreachable and its detail words (edited: call-deadline added)
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude CLI adapter and its readings of the claude client (edited: the client's tool-call bound reading added)
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the viola mcp server's manifest (edited: the async runtime's timer feature turned on)
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the viola mcp server, the hidden MCP verb with no human surface, and its tool set (edited: the call deadline and its injected seam exported)
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the viola mcp tools send, wait, last, answer and list and the wait cap (edited: the frame tools raced against the call deadline)
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the viola mcp tool results, success, refusal and invalid-params, and the instance-unreachable result's detail (edited)
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the check of the viola mcp tool handlers (edited: the call-deadline cases added)
@task_file n=8 path=tests/contract_error_bodies.rs
what: the check of the error bodies, the CLI refusals and the MCP instance-unreachable result (edited: the MCP detail list gains call-deadline)
@task_file n=9 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the test-only live rig's reader of the claude client's MCP call lines into closed words (new)
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig and its arms (edited: the MB arm added)
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig's command line (edited: the fake option and the mc rehearsal word added)
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the check of the live rig's rehearsals (edited: three MB rehearsal cases added)
@task_item n=1 part=touches
contract/mcp-tool-results/refusal: the instance-unreachable refusal result's detail word list gains call-deadline, the frame tool's call ended at the 55 s deadline.
@task_item n=2 part=touches
contract/mcp-tools/send: a send now ends at the call deadline with instance-unreachable call-deadline; the wrapper's paste goes on, the outcome unknown to the driver, as with endpoint-lost.
@task_item n=3 part=touches
contract/mcp-tools/wait: the wait cap stays 50 000 ms, now below the 55 s call deadline; any cell whose why cites the 2.1.287 client default or a 60 s smallest default is re-based to the 2.1.328 and 2.1.330 reading.
@task_item n=4 part=touches
contract/mcp-tools/last and contract/mcp-tools/answer: each is a frame tool now bounded by the call deadline, its expiry the call-deadline result.
@task_item n=5 part=touches
contract/mcp-tools/list: list is not a frame tool and is not under the call deadline; a row that says every tool call is bounded would be touched.
@task_item n=6 part=touches
decision/mcp-server-without-human-surface and decision/mcp-verb-hidden: rules over the viola mcp server, which now runs a timer and ends frame calls at a compiled deadline; the call-deadline result writes no log line.
@task_item n=7 part=touches
contract/cli-exit-codes/21 and contract/cli-refusals/instance-unreachable: the shared unreachable-cause set grew to seven, but the CLI exit-21 set stays six; a cell that says the CLI and MCP detail sets are the same closed set is touched.
@task_item n=8 part=touches
contract/instance-unreachable-detail/* (the exit-21 --json detail rows): no CLI verb writes call-deadline, so no new exit-21 detail row; a cell that counts the closed set at six as the whole set is touched.
@task_item n=9 part=touches
contract/frame-verb-hints/*: no CLI hint is owed for call-deadline, the CLI never writes it; decision/hint-for-every-cause holds for the CLI set alone, a cell claiming a hint for every member of the shared set is touched.
@task_item n=10 part=touches
decision/mcp-hides-wheel-and-handoff-verbs: the MCP tool set is unchanged (send, wait, last, answer, list); no tool added or removed.
@task_fact n=1 slug=surface-cli
fact: Every viola mcp frame tool call (send, wait, last, answer) ends at a compiled 55 s call deadline with the instance-unreachable result, detail call-deadline; list is not under it, and the deadline sits above the 50 s wait cap and below the client's 120 s auto-background reading.
@task_fact n=2 slug=surface-cli
fact: call-deadline is a seventh member of the closed unreachable-cause set written by the MCP frame tools alone; the CLI exit-21 detail set stays six and gains no hint.
@task_fact n=3 slug=surface-cli
fact: At the MCP call deadline the wrapper's send is not cancelled: the driver reads call-deadline with the send outcome unknown, as with endpoint-lost; the paste write itself stays unbounded.
@task_item n=1 part=untouched
absence/web-surface and absence/cli-deep-links: no page and no link surface came or went.
@task_item n=2 part=untouched
The viola list board and BAY line rows, their columns, widths, truncation, styling, colour depth and token rendering: no board or styling change.
@task_item n=3 part=untouched
The human send output rows, the readback mirror, its vocabulary and word column, and the send refusals before any request: the human CLI send is unchanged.
@task_item n=4 part=untouched
viola run and viola revive start output, refusals and --json documents: no start path changed.
@task_item n=5 part=untouched
viola verify, viola link and unlink, viola plugin install, wheel verb output and viola --help rows: none of these verbs changed.
@task_item n=6 part=untouched
The human-TTY presentation decisions (no banner, spinner, pager, footer, progress glyphs; amber means dialog; plain output modes; stdout results, the rest on stderr): no human output changed.
@task_item n=7 part=untouched
viola wait output rows on the CLI (waiting line, timed out, turn-ended, dialog): the CLI wait verb is unchanged; only the MCP tool call is bounded.
