@task_prose part=session
The session bounds every MCP frame tool call (send, wait, last, answer) by a compiled 55 s call deadline, MCP_CALL_DEADLINE, raced against the frame inside viola mcp; list is not under it.
At the deadline the frame's work is dropped and the tool returns the instance-unreachable result with the new cause word call-deadline, the outcome of an in-flight send unknown, as with endpoint-lost.
The closed unreachable-cause set grows from six members to seven (call-deadline), written by the MCP frame tools alone; the CLI exit-21 cause set stays six and the CLI client gained no arm.
The Claude adapter names the client's tool-call bound as a reading of the 2.1.328 and 2.1.330 bundles (120 s); the 50 s wait cap is unchanged, its why re-based off the old 2.1.287 reading.
viola-mcp now enables the tokio time feature for the production deadline timer; no package was added and the lockfile is unchanged; the deadline is an injected seam for tests.
The test-only live rig gained an MB arm that drives a real claude print-mode client against viola mcp through a private 0600 MCP config inside its scratch, keeping no output byte; measured live, the 50 s wait returned whole on 2.1.330 and 2.1.328 and a 5 s per-server timeout cut the call.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core crate's shared types: the closed unreachable-cause set behind the instance-unreachable detail causes, and its code words
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter: what viola reads of the claude CLI, here the client's MCP tool-call bound as read from its bundles
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the viola mcp server's dependency manifest: its tokio features, now rt and time
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the viola mcp stdio server: its tool set construction, the call-deadline seam and its re-export
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP tools send, wait, last, answer and list: the frame path to the wrapper, the wait cap (timeout_ms) and the new call deadline
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP isError structuredContent results: instance-unreachable with its detail cause word, refusals and faults
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the MCP handler integration tests: the frame tools ending at the call deadline and a reply before it
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body contract census: each product error writer's code words and detail lists, here the MCP instance-unreachable detail list
@task_file n=9 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig (test-only): its arms beside the MCP arm, the new MB arm, its scratch with a private MCP config, its isolated claude child and its summary keys
@task_file n=10 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the live rig's MCP call reader (test-only): reads the claude client's output lines into closed words and counts, keeping no byte
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig CLI (rehearse, live, census): its new fake-agent option and the mb and mc rehearsal words
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests: the MB arm's start and teardown, its cut label, its refusal without a fake agent
@task_item n=1 part=touches
contract/mcp-iserror/instance-unreachable: its detail cause list gains call-deadline, written by the MCP frame tools at the call deadline.
@task_item n=2 part=touches
contract/mcp-iserror/detail and contract/mcp-structured-content/codes: where a cell lists the instance-unreachable detail words, the list gains call-deadline (seven, not six).
@task_item n=3 part=touches
contract/mcp-frame-tool-result/unreachable: a frame tool now also returns instance-unreachable when its own 55 s call deadline expires, not only when the wrapper is unreachable.
@task_item n=4 part=touches
contract/instance-unreachable-cause/* (the six per-member rows): each stays true; a new member row for call-deadline is owed beside them, MCP-only.
@task_item n=5 part=touches
contract/instance-unreachable-detail/*: where a row states the set of detail words the MCP result carries, that set is now seven with call-deadline.
@task_item n=6 part=touches
contract/cli-exit-codes/21, contract/cli-instance-unreachable/json-document and contract/cli-stderr-hints/unwrapped-name-endpoint-lost: checked, hold unchanged; the CLI exit-21 cause set stays six and the CLI client maps no call-deadline.
@task_item n=7 part=touches
contract/mcp-tools/send, wait, last and answer: each call is now bounded by the 55 s call deadline; contract/mcp-tools/list is not under it.
@task_item n=8 part=touches
contract/mcp-tool-inputs/timeout-ms: the wait cap of 50000 ms is unchanged, now set below the 55 s call deadline and the client bound read from 2.1.328 and 2.1.330; any cell resting on the 2.1.287 reading is stale.
@task_item n=9 part=touches
decision/mcp-frame-tools-refuse-quietly and decision/mcp-frame-tool-refusals: a call-deadline result is not a pre-frame refusal; the frame may already have reached the wrapper, its outcome unknown, so a claim that every unreachable result sent nothing needs this exception.
@task_item n=10 part=touches
decision/send-text-is-one-bracketed-paste and decision/bridge-send-one-bracketed-paste: the wrapper's paste stays one blocking write with no time bound; the MCP deadline bounds only the driver's wait, and the wrapper's send goes on after it.
@task_item n=11 part=touches
decision/every-error-writer-named: the MCP frame tools are now a writer of instance-unreachable with call-deadline on their own, without the wrapper.
@task_item n=12 part=touches
decision/client-reads-closed-values-only and decision/client-reply-code-words-only: call-deadline is a closed word the MCP server writes itself, never read from the wrapper's reply.
@task_item n=13 part=touches
decision/sync-crate-tokio-ban, decision/sync-crates-tokio-free and decision/no-tokio-in-sync-crates: viola-mcp now runs a tokio timer (time feature); the sync crates stay tokio-free and the channel tokio-free probe is green.
@task_item n=14 part=touches
decision/rmcp-features-exact, decision/rmcp-feature-set and decision/exact-dependency-pins: viola-mcp's tokio feature list widened by time; no package added, the lockfile unchanged, the rmcp feature probe green.
@task_item n=15 part=touches
decision/test-seams-compiled-out and decision/test-seams-fake-agent-only: the call deadline is an injected seam (a deadline provider through a second tool-set constructor); release takes the production timer, and whether that seam is a test seam under these rows is to be checked.
@task_item n=16 part=touches
decision/live-rig-mcp-arm-keeps-no-output: stands unchanged; a new row of its own for the MB arm is owed beside it (P4-F2).
@task_item n=17 part=touches
decision/live-rig-isolated-child and decision/live-rig-credentials-copy: the MB arm runs a real claude client in the rig's isolated scratch, its MCP config private 0600 and removed with the scratch, no product config key.
@task_item n=18 part=touches
decision/live-round-rig-keys-own-pty: the MB arm types no key (keys none on each start).
@task_item n=19 part=touches
decision/bounded-readers-census, decision/bounded-external-readers and decision/child-output-bounded: the rig's new reader of claude output reads through MAX_FRAME into closed words; product readers are unchanged.
@task_item n=20 part=touches
decision/real-cli-local-only: the MB arm drives the real claude CLI on the local machine only, as a live rig leg.
@task_fact n=1 slug=error-handling
fact: Every MCP frame tool call (send, wait, last, answer) ends at a compiled 55 s call deadline, above the 50 s wait cap and below the client's tool-call bound read per version from the claude bundles; at expiry the tool returns instance-unreachable with detail call-deadline and the outcome of an in-flight frame is unknown; list is not bounded by it.
@task_fact n=2 slug=error-handling
fact: call-deadline is the seventh member of the closed instance-unreachable cause set, written by the MCP frame tools alone; the CLI exit-21 cause set stays six and never carries it.
@task_fact n=3 slug=data-protection
fact: The live rig's MB arm keeps no stdout byte of the claude client (read through MAX_FRAME into closed words, every byte dropped), keeps its stderr as a byte count and a SHA-256 only, writes its MCP config 0600 inside the scratch and removes it with the scratch, puts a server timeout on the cut label alone, and counts a long call as whole only when timed-out is true beside call-error false.
@task_fact n=4 slug=threat-model-summary
fact: The wrapper's paste into the pty has no time bound (one blocking write); the MCP call deadline bounds only the driver's wait, so at the deadline the wrapper's send goes on, an accepted open finding until a paste bound lands.
@task_item n=1 part=untouched
Authentication and authorization rows: peer euid checks, server verification before any frame, socket directory and strict-modes checks are unchanged; the deadline sits after them.
@task_item n=2 part=untouched
The wrapper channel's JSON-RPC methods, frames, error codes and the paste and dialog character sets: no method, param, code or refusal word changed.
@task_item n=3 part=untouched
The hook contract: hook stdin, hook-decision details, fail-open and silent exit rows are not reached.
@task_item n=4 part=untouched
Verify, its stamps, probes, fixtures and fixture refusal codes; revive, snapshots and the pinned binary and plugin files: none changed.
@task_item n=5 part=untouched
CI and supply chain: actions pinning, workflow permissions, cargo-deny, zizmor, advisory floors, release build and canary scan rows are untouched.
@task_item n=6 part=untouched
Secret handling of the product's own claude child: the credential strip, hermetic environment and no-stored-secrets rows are unchanged.
@task_item n=7 part=untouched
Logging and diagnostics rows: an MCP instance-unreachable result writes no log line, by rule, so no log or diagnostics row gains a line.
