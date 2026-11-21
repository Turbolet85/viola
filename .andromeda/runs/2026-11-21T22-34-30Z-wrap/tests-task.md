@task_prose part=session
The session bounds every MCP frame tool call (send, wait, last, answer) by a compiled MCP call deadline of 55 s; at the deadline the frame is dropped and the result is instance-unreachable with the new cause word call-deadline; list is not under it.
The closed unreachable-cause set grew from six members to seven (call-deadline), written by the MCP frame tools alone; the CLI exit-21 cause set is unchanged at six.
The deadline is an injected seam (a deadline provider handed to the MCP tools); production uses a Tokio sleep, so viola-mcp now runs a Tokio timer, while tests fire the deadline through an injected signal with no paused Tokio clock.
A client bound read of 120 s is recorded as a reading of the claude 2.1.328 and 2.1.330 bundles; the wait cap of 50 000 ms is unchanged, its reasoning re-based off the old 2.1.287 reading to sit below the 55 s deadline and the 120 s read.
The live rig gained an MB arm beside AV (labels MB, MB8, MC and spares) that runs the real claude print-mode MCP client against a wrapped fake agent; live it measured a 50 s wait returning whole on 2.1.330 and 2.1.328 and a per-server 5000 ms timeout cutting a call at 5 s on 2.1.330.
The paste write into the child stays one blocking write with no time bound; that is a finding carried forward, not changed here.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core types, holding the closed instance-unreachable cause set and each cause's word; edited to add the call-deadline member
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent adapter, holding readings of the claude CLI; edited to add the MCP client bound read (120 s) from the 2.1.328 and 2.1.330 bundles
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the MCP server crate's manifest and its tokio features; edited so tokio takes the time feature beside rt
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the MCP adapter's public surface, the tools constructor and serve; edited to add the injected deadline provider seam and a constructor that takes it, and to re-export the MCP call deadline
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP frame tools send, wait, last and answer and the wait cap; edited so every frame call races the MCP call deadline and ends as instance-unreachable call-deadline, with the wait cap's reasoning re-based
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP tool result bodies, including instance-unreachable and its cause detail; edited for the call-deadline detail
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the MCP handler integration suite; edited to add a case per frame tool ending at the call deadline and a case where a reply before the deadline is the reply, driven by an injected deadline signal
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body census of every surface's error bodies; edited so the MCP instance-unreachable row's detail list names call-deadline, the CLI row unchanged
@task_file n=9 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the live rig's reader of the claude client's MCP call lines into closed words, test-only; new
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig, its arms, labels, plans and summaries, test-only; edited to add the MB arm beside AV with its private MCP config and summary keys
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command line; edited to add the fake option on rehearse and live and the mb and mc rehearsal words
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal integration suite over the fake agent; edited to add three MB rehearsal cases
@task_item n=1 part=touches
The instance-unreachable cause rows (contract/instance-unreachable-cause/* and contract/unreachable-cause/socket-dir-refused): the closed set moved from six to seven with call-deadline; each per-member row stays true, and a call-deadline member row is owed beside them; any cell holding the count six for the whole set is stale.
@task_item n=2 part=touches
The MCP instance-unreachable result rows (contract/mcp-tool-result/instance-unreachable, contract/mcp-tools/instance-unreachable, contract/mcp-iserror/payload): the detail's cause list gains call-deadline, written by the MCP frame tools alone.
@task_item n=3 part=touches
The MCP frame tools instance-unreachable detail rows (contract/mcp-unreachable-detail/*): a call-deadline member is owed beside the five standing members.
@task_item n=4 part=touches
decision/time-faked-through-seams: its qualifier that viola-mcp runs no tokio timer no longer holds; the MCP call deadline is now an injected seam (a deadline provider), production a Tokio sleep, tests an injected signal, no paused Tokio clock.
@task_item n=5 part=touches
The injected-clock rows (decision/injected-clock, decision/test-clocks-behind-clock-seam, decision/wrapper-deadline-injected-clock, decision/time-as-argument): the MCP adapter's deadline is one more time seam injected the same way.
@task_item n=6 part=touches
The tokio placement rows (decision/tokio-only-in-e2e-crate, decision/sync-crates-without-tokio, decision/tokio-banned-per-sync-crate, decision/channel-tokio-free-by-default, decision/viola-channel-tokio-free): viola-mcp's tokio gained the time feature; no package added, the channel tokio-free probe stays green.
@task_item n=7 part=touches
The rmcp feature rows (decision/rmcp-feature-limit, decision/rmcp-features-server-transport-io): the MCP crate's manifest changed tokio features only; rmcp features unchanged and the probe green.
@task_item n=8 part=touches
The MCP wait bound rows (contract/mcp-wait-tool/timeout-bound, contract/mcp-wait/timeout-ms): the wait cap stays 50 000 ms; any cell justifying it by the claude 2.1.287 reading or a smallest client default of 60 s is stale, re-based to the 2.1.328 and 2.1.330 reading (120 s auto-background read) and to sitting below the 55 s MCP call deadline.
@task_item n=9 part=touches
The instance-first rows for MCP frame tools (decision/mcp-frame-tools-check-instance-first, decision/frame-verbs-check-instance-first, decision/server-checked-before-any-frame): the frame call is now raced against the deadline; the check order is unchanged and list stays outside the deadline.
@task_item n=10 part=touches
The CLI and MCP parity rows (decision/cli-mcp-identical-payloads, contract/mcp-cli-parity/exit-21): call-deadline is an MCP-only cause with no CLI exit-21 counterpart, an exception to identical cause sets.
@task_item n=11 part=touches
decision/closed-enums-need-decisions-log: a closed enum (the unreachable-cause set) gained a member, call-deadline, ratified provisionally on the overseer's delegate confirm.
@task_item n=12 part=touches
The MCP test layer rows (decision/mcp-test-layer, decision/mcp-handlers-use-tokio-client, decision/integration-suite-per-contract): the handler suite gained a call-deadline case per frame tool and a reply-before-deadline case, plus a unit case that the deadline sits between the wrapper bounds and the client read.
@task_item n=13 part=touches
The no-sleep and owned-time rows (decision/no-sleep-synchronization, decision/no-test-owned-real-time, decision/no-test-retries-or-sleeps): the deadline expiry cases fire an injected signal rather than waiting 55 s real time.
@task_item n=14 part=touches
The error-body witness rows (decision/error-bodies-held-by-surface-witnesses, decision/contract-tables-are-literal, decision/error-writers-each-named): the census's MCP instance-unreachable row now names call-deadline.
@task_item n=15 part=touches
The unreachable log cause rows (contract/unreachable-log-cause/*): an MCP instance-unreachable writes no log line, so call-deadline adds no log cause; a picker should confirm no row claims the log causes mirror the whole MCP set.
@task_item n=16 part=touches
The standing live rig rows (decision/standing-live-rig, decision/live-rig-hint-cell-reading, decision/live-failing-reading-not-verification): the rig gained an MB arm beside AV with labels MB, MB8, MC and spares, a fake option and mb and mc rehearsal words, and summary keys read as closed words.
@task_item n=17 part=touches
The real-CLI rows (decision/real-claude-local-only, decision/real-claude-never-in-ci, decision/no-real-cli-in-ci, decision/ci-never-real-claude): the MB live starts run the real claude CLI locally only, by path on 2.1.330 and 2.1.328.
@task_item n=18 part=touches
The multi-version rows (decision/compat-tests-for-cli-versions, decision/multi-version-trigger-layers): the MCP client bound was measured live on two CLI versions and read from both bundles.
@task_item n=19 part=touches
The bounded reader rows (decision/bounded-readers, decision/bounded-readers-keep-the-cap): the MB arm reads the claude client's stdout through the frame cap into closed words and drops every byte.
@task_item n=20 part=touches
The test-only crate rows (decision/single-test-only-crate, decision/test-only-e2e-crate): a new test-only module for reading MCP call lines inside the test-only crate; no crate added.
@task_item n=21 part=touches
decision/fake-agent-not-proof-of-real-cli: the MB rehearsals over the fake agent stand beside live MB starts on the real client, the live starts alone carrying the client-bound claim.
@task_fact n=1 slug=test-scope-summary
fact: The instance-unreachable cause call-deadline is written only by the MCP frame tools send, wait, last and answer when a call reaches the 55 s MCP call deadline; the frame is dropped and the outcome of an in-flight send is unknown, as with endpoint-lost.
@task_fact n=2 slug=integration-test-strategy
fact: MCP frame tools instance-unreachable detail call-deadline: each of send, wait, last and answer ends with detail call-deadline at the call deadline, and a reply arriving before the deadline is returned as the reply.
@task_fact n=3 slug=1-test-scope-summary
fact: The MCP instance-unreachable cause set has seven members while the CLI exit-21 cause set stays six; call-deadline is the one MCP-only cause and has no CLI counterpart.
@task_fact n=4 slug=test-strategy
fact: The MCP call deadline is held by a unit case that it sits above the wait cap and below the client bound read, and by integration cases that fire it through an injected deadline signal, never by waiting out 55 s.
@task_fact n=5 slug=e2e-test-strategy
fact: The live rig's MB arm judges a long call green only on timed-out true beside call-error false, so a client result with no is_error key cannot turn a cut call green; it keeps no stdout byte and its MCP config is owner-only inside the scratch.
@task_fact n=6 slug=6-e2e-test-strategy
fact: The claude CLI's default MCP tool-call bound is a reading of each version's bundle strings, never a measurement; live MB starts measure only that a 50 s call returns whole and that a per-server timeout cuts the call while the server outlives it.
@task_item n=1 part=untouched
The CLI exit-21 cause rows (contract/exit-21-detail/*, contract/cli-exit-codes/21, contract/cli-wait/endpoint-lost, contract/viola-wait/endpoint-lost, contract/cli-json/instance-unreachable, the hint rows): the CLI set stays six and the CLI client gained no arm.
@task_item n=2 part=untouched
The wrapper's send path rows (contract/viola-send/*, contract/driver-send/*, send confirmation, paste framing and paste validation): the paste stays one blocking write per line, unchanged; the deadline bounds only the driver's wait.
@task_item n=3 part=untouched
Hook rows, dialog decision bodies and hook performance budgets: no hook surface changed.
@task_item n=4 part=untouched
run and revive start refusals, snapshots, heartbeats, the event log and the list board: none moved.
@task_item n=5 part=untouched
The 5-command test harness, agent-run status, the gate, pre-push and their exit codes and reasons: unchanged.
@task_item n=6 part=untouched
CI workflow, coverage floors, mutation gates, fuzzing and supply-chain rows: no gate, threshold or job changed.
@task_item n=7 part=untouched
The fake agent's command line, receipts, screens and turn scripts: the fake agent gained nothing.
@task_item n=8 part=untouched
The MCP tools list, invalid-params fields, schema-violation and stdin line-cap rows: no tool, argument or input rule changed.
