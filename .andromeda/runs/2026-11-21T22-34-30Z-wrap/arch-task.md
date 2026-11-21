@task_prose part=session
The session bounded every MCP frame tool call (send, wait, last, answer) by a compiled 55 s call deadline, so each call ends below the claude client's tool-call bound; list is no frame tool and is not under it.
At the deadline the call's work is dropped and the tool returns instance-unreachable with a new seventh unreachable cause, call-deadline, written by the MCP frame tools alone; the CLI exit-21 cause set stays six.
The deadline sits above the 50 s wait cap (unchanged) and below the client bound read as 120 s from the 2.1.328 and 2.1.330 bundles; the wait cap's reason is re-based off the old 2.1.287 reading.
viola-mcp now runs a Tokio timer (the time feature) for the production deadline; tests fire the deadline through an injected seam, never a paused Tokio clock.
It measured the real client live through a new test-only live-rig arm MB: a 50 s wait returned whole on 2.1.330 and 2.1.328, and a per-server 5 s timeout cut the call at its bound on 2.1.330 with the server outliving the cut call (one start).
It found and did not fix that the deadline bounds the driver's wait, never the wrapper's paste: the paste write stays one blocking write with no time bound, pinned as a carry, no route entry minted.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core's shared vocabulary: the closed unreachable-cause set and its kebab-case wire words (gained call-deadline)
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent crate's Claude-specific constants: the CLI MCP client's tool-call bound as read from the CLI bundles per version
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the viola-mcp crate manifest: its dependencies and the Tokio features it takes (rt plus time)
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the MCP server crate's entry: Tools construction and the injected call-deadline seam, as the stdio serve path builds it
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP tools send, wait, last, answer and list: the frame tools' shared frame path, the 50 s wait cap and the 55 s call deadline
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP tool results: the instance-unreachable result and the cause word its detail carries
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the MCP handler integration tests: each frame tool ending at the call deadline, a reply before the deadline kept
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body contract census: each error body's detail list, the MCP instance-unreachable row gaining call-deadline
@task_file n=9 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig: its arms, labels, plans and summary keys, the new MB arm that runs a real claude client against viola mcp
@task_file n=10 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the test-only live rig's MCP-call reader: reads the claude client's lines into closed words, keeping no output byte
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test binary's CLI: rehearse, live and census, gaining a fake-agent path option and the mb and mc rehearsal words
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests against the fake agent, gaining three MB arm cases
@task_item n=1 part=touches
decision/mcp-wait-cap: the cap stays 50 s, but its reason rested on the claude 2.1.287 reading (below the client's smallest default); it is re-based to the 2.1.328 and 2.1.330 reading (the 60 s figure floors HTTP transports only, the stdio bound read as 120 s) and now sits below the 55 s MCP call deadline.
@task_item n=2 part=touches
decision/send-time-bound: the MCP call deadline bounds only the driver's wait on send; at the deadline the wrapper's send goes on and the paste write itself stays unbounded (one blocking write), a finding the row must name.
@task_item n=3 part=touches
contract/unreachable-cause/cause-set: the closed set grows from six members to seven with call-deadline, written by the MCP frame tools alone; the CLI client maps no variant to it.
@task_item n=4 part=touches
contract/mcp-tool-results/instance-unreachable and contract/mcp-refusal-result/instance-unreachable: the detail's cause list gains call-deadline.
@task_item n=5 part=touches
decision/mcp-unreachable-result-for-every-cause: the new call-deadline cause reaches the driver through the same instance-unreachable result as every other cause.
@task_item n=6 part=touches
contract/mcp-tools/send, contract/mcp-tools/wait, contract/mcp-tools/last and contract/mcp-tools/answer: each frame tool now ends at the 55 s call deadline with instance-unreachable call-deadline; contract/mcp-tools/list is no frame tool and stays outside the deadline.
@task_item n=7 part=touches
contract/channel-wait/timeout-ms: a wait with no timeout_ms returns at viola's 50 s cap, measured live to return whole on two client versions, below the new call deadline.
@task_item n=8 part=touches
decision/hybrid-concurrency, decision/sync-core-tokio-only-in-mcp, decision/tokio-only-in-mcp-crate and decision/tokio-containment: Tokio stays inside the MCP crate, but that crate now takes the time feature and runs a Tokio timer for the production deadline.
@task_item n=9 part=touches
decision/mcp-dependencies: viola-mcp's Tokio features change from rt to rt plus time; no package added, the lockfile unchanged.
@task_item n=10 part=touches
decision/lost-endpoint-loses-event and decision/connect-failure-dead-closed-stream-lost: call-deadline joins endpoint-lost as a cause whose send outcome is unknown to the driver.
@task_item n=11 part=touches
decision/every-send-confirmed and decision/send-confirmed-after-the-fact: a driver reading call-deadline on send does not learn whether the send landed; confirmation still comes from the log after the fact.
@task_item n=12 part=touches
decision/spine-deadline and decision/hook-connect-deadline: a third compiled deadline now stands beside them, the MCP call deadline, between the wrapper's own bounds and the client's bound.
@task_item n=13 part=touches
decision/test-seams-not-configuration: the deadline is injected through a seam for tests only; it is compiled, with no config key and no environment knob.
@task_item n=14 part=touches
decision/claude-shapes-isolated, decision/claude-shapes-in-one-crate and decision/agent-claude-pure: the client's tool-call bound reading is a Claude-specific constant kept in the agent crate, a pure value with no file I/O.
@task_item n=15 part=touches
decision/e2e-test-only, decision/e2e-no-rmcp-child-transport and name/binary/viola-live-rig: the test-only live rig gains the MB arm, which runs a real claude print-mode client that spawns viola mcp from a private MCP config inside the rig's scratch, plus a fake-agent path option and new rehearsal words.
@task_item n=16 part=touches
decision/real-claude-local-only, decision/real-cli-probe-local-only and decision/no-live-claude-in-ci: the MB arm's live starts ran locally against claude 2.1.330 and 2.1.328; CI exercises only its rehearsal against the fake agent.
@task_item n=17 part=touches
decision/wire-enum-values-kebab-case and decision/refusal-details-closed-set: the new cause word call-deadline is one more closed kebab-case wire value.
@task_fact n=1 slug=established-decisions
fact: Every MCP frame tool call (send, wait, last, answer) is bounded by a compiled 55 s call deadline, above the 50 s wait cap and below the claude client's tool-call bound; at expiry the call's work is dropped and the tool returns instance-unreachable with cause call-deadline; list is not under it.
@task_fact n=2 slug=established-decisions
fact: The MCP call deadline bounds the driver's wait, never the wrapper's work: at the deadline the wrapper's send goes on and its outcome is unknown to the driver, and the paste write stays unbounded, an open finding carried without a route entry.
@task_fact n=3 slug=established-decisions
fact: The claude client's default MCP tool-call bound is a reading of the CLI bundle strings per version (read as 120 s on 2.1.328 and 2.1.330), never a measured value; live, a 50 s call returned whole on both versions and a per-server timeout cut a call at its bound.
@task_fact n=4 slug=conventions
fact: The unreachable cause call-deadline is written by the MCP frame tools alone; the CLI frame verbs never write it and the CLI exit-21 cause set stays six.
@task_fact n=5 slug=cross-cutting-patterns
fact: viola-mcp runs a Tokio timer for its production call deadline; tests fire the deadline through an injected seam, never through a paused Tokio clock.
@task_item n=1 part=untouched
Hook rows (hook tiers, hook exits, dialog hold, hook diagnostics, hook-to-kind map): no hook path or hook output changed.
@task_item n=2 part=untouched
CLI exit codes, the exit-21 documents and the CLI frame-verb process-exit detail rows: the CLI cause set and its exit codes are unchanged at six.
@task_item n=3 part=untouched
Wheel, link, pause, release, revive and verify rows, and the trust writer code words: none of those paths moved.
@task_item n=4 part=untouched
Event kinds, event data, the events log, instance snapshots and ledger stamps: no event, snapshot or stamp shape changed, and no new capability ledger row was added.
@task_item n=5 part=untouched
The plugin's MCP config and config.json keys: no product config key was added; the per-server timeout lives only in the test rig's private config.
@task_item n=6 part=untouched
Paste text contracts (control characters, empty text), the PTY seam, the readiness gate and screen signatures: the paste and screen paths were not edited.
@task_item n=7 part=untouched
CI/CD approach rows (workflows, pinned actions, coverage floors, fuzz, release): no workflow or gate threshold changed.
@task_item n=8 part=untouched
Design philosophy, project intent and deployment model rows: no product boundary or deployment shape moved.
