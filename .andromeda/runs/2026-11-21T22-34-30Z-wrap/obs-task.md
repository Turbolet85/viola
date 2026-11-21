@task_prose part=session
The session bounded every viola mcp frame tool call (send, wait, last, answer) by a compiled MCP call deadline of 55 s; at expiry the call returns the instance-unreachable result with the new cause word call-deadline, and the list tool stays outside the deadline.
The closed unreachable-cause set grew from six members to seven (call-deadline); only the MCP frame tools write the new word, and the CLI exit 21 cause set stays at six.
The deadline is an injected seam (a deadline provider), so tests fire it with a signal and need no paused Tokio clock; viola mcp now runs a Tokio timer in production (the tokio time feature, no package added, the lock file unchanged).
The claude CLI MCP client's tool-call bound is recorded as a reading of the 2.1.328 and 2.1.330 bundle strings (a 120 s auto-background default), never as a contract; the wait cap of 50 s stays unchanged, and its rationale is re-based below the deadline and that reading.
Measured live through a new test-only live-rig arm (MB): on 2.1.330 and 2.1.328 a wait with no timeout returned whole at the 50 s cap; under a 5000 ms per-server timeout the client cut the call at 5 s and the viola mcp process outlived the cut call.
Finding, not fixed: the wrapper's paste into the PTY stays one blocking write with no time bound; at the deadline the wrapper's send goes on, and the driver cannot know the outcome, as with endpoint-lost.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the closed unreachable-cause set and its cause words, shared by the CLI exit 21 detail and the MCP instance-unreachable result
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent adapter's readings of the wrapped claude CLI, now including the MCP client's tool-call bound as read from the bundle
@task_file n=3 path=crates/viola-mcp/Cargo.toml
what: the viola mcp stdio server's dependencies, its Tokio runtime features
@task_file n=4 path=crates/viola-mcp/src/lib.rs
what: the rmcp stdio MCP server's entry, the tools constructor and the injected call-deadline seam
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the viola-mcp stdio server tools (send, wait, last, answer, list), the frame call to the instance, the wait cap and the MCP call deadline
@task_file n=6 path=crates/viola-mcp/src/result.rs
what: the MCP isError result bodies, the instance-unreachable result and its cause detail
@task_file n=7 path=crates/viola-mcp/tests/handlers.rs
what: the viola mcp handler integration tests, the frame tools ending at the call deadline
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body contract check over the CLI and MCP refusal bodies and their detail lists
@task_file n=9 path=crates/viola-e2e/src/rig/mcpcall.rs
what: the test-only live rig's reader of the claude client's MCP call lines, closed into words with every byte dropped
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig and its arms, the new MB arm that runs a real claude MCP client against viola mcp
@task_file n=11 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig command line, its rehearse and live verbs and the fake agent option
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal integration tests
@task_item n=1 part=touches
contract/mcp-iserror/instance-unreachable: its cause list gains call-deadline, which only the MCP frame tools write; the set is now seven words.
@task_item n=2 part=touches
decision/mcp-unreachable-carries-one-cause-word: call-deadline becomes one of the cause words an MCP unreachable result carries.
@task_item n=3 part=touches
Rows listing the MCP frame tool refusal causes (contract/mcp-frame-tool-refusal/*, contract/mcp-instance-unreachable/*, contract/mcp-frame-tools/*, contract/mcp-wait-last-connect/*): call-deadline belongs beside them as a new member; the existing per-cause rows stay true.
@task_item n=4 part=touches
contract/viola-mcp-tools/send, wait, last and answer: each now ends at the 55 s MCP call deadline with instance-unreachable call-deadline; contract/viola-mcp-tools/list is not under the deadline.
@task_item n=5 part=touches
Rows about the CLI exit 21 cause words (contract/unreachable-cause/*, contract/exit-21-detail/*, contract/cli-exit-21/*, contract/exit-21-document/detail, contract/cli-exit-codes/21): the shared cause set grew, but the CLI set stays six and call-deadline is never a CLI detail, so a picker should confirm they still hold.
@task_item n=6 part=touches
decision/instance-unreachable-not-logged, decision/mcp-log-file-sink and decision/home-level-mcp-log-no-user-content: the new call-deadline result writes no diagnostics line, like every MCP instance-unreachable result.
@task_item n=7 part=touches
decision/mcp-results-codes-only, decision/mcp-rejects-codes-only and decision/mcp-rejections-codes-only: the deadline's result carries only a closed code word.
@task_item n=8 part=touches
decision/mcp-stdio-server-tokio-allowed, decision/sync-crates-without-tokio and decision/obs-deps-tokio-free: viola mcp now runs a Tokio timer (the time feature) for its production deadline; the channel crate stays Tokio-free and no package was added.
@task_item n=9 part=touches
decision/obs-deps-pass-cargo-deny and decision/banned-telemetry-dependencies: indirect, through the viola mcp dependency features; no new package, the lock file is unchanged.
@task_item n=10 part=touches
decision/claude-internals-not-a-contract: the claude MCP client's tool-call bound is held as a per-version reading of the bundle strings, never a contract and never measured as a value.
@task_item n=11 part=touches
contract/mcp-spans/mcp-tool-call: at the deadline, the frame call inside a tool call is dropped and the tool returns the call-deadline result.
@task_item n=12 part=touches
contract/send-line/unconfirmable, contract/send-log-fields/send-issued and decision/mid-call-disconnect-logged-once: at the deadline, the driver's send stops waiting while the wrapper's send goes on, so the outcome is unknown, as with endpoint-lost.
@task_item n=13 part=touches
contract/pty-seam-log/paste: the paste stays one blocking write with no time bound; this is a finding carried to the route, not a change made here.
@task_item n=14 part=touches
contract/mcp-refusal-text/instance-unreachable: the instance-unreachable body now has a call-deadline cause beside strict-modes-failed and the other causes.
@task_item n=15 part=touches
decision/deterministic-event-emission, decision/deterministic-emission and decision/no-sleep-based-log-waits: indirect, through the deadline seam; tests fire the deadline by an injected signal, with no sleep and no paused Tokio clock.
@task_item n=16 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: indirect, through the new live rig arm, which runs the real claude CLI only in an operator live leg, never in CI.
@task_fact n=1 slug=error-capture-reporting
fact: Every viola mcp frame tool call (send, wait, last, answer) ends at a compiled 55 s MCP call deadline. At expiry it returns instance-unreachable with the cause call-deadline, and a send's outcome is unknown to the driver, as with endpoint-lost. The list tool is not under the deadline.
@task_fact n=2 slug=obs-scope-summary
fact: Only the MCP frame tools write the cause word call-deadline; the unreachable-cause set has seven members, while the CLI exit 21 cause set stays at six and never carries call-deadline.
@task_fact n=3 slug=1-obs-scope-summary
fact: The claude CLI MCP client's tool-call bound is a per-version reading of its bundle strings (120 s auto-background on 2.1.328 and 2.1.330), never a contract and never a measured value. The MCP call deadline sits below that reading and above the 50 s wait cap.
@task_fact n=4 slug=1-obs-scope-summary
fact: viola mcp runs a Tokio timer in production for its call deadline. The deadline is an injected seam, so tests fire it with a signal and use no paused Tokio clock.
@task_item n=1 part=untouched
The hook path: hook log lines, fail-open, hook budgets and hook stdout and stderr.
@task_item n=2 part=untouched
The viola-channel JSON-RPC protocol, its error codes, channel lines and the conn and corr correlation.
@task_item n=3 part=untouched
The dialog raise path, dialog log fields and dialog spans on the wrapper side.
@task_item n=4 part=untouched
The viola run process log, run start spans and attributes, the wheel, the heartbeat and liveness.
@task_item n=5 part=untouched
Panic capture, panic outcome by role, the detail-file line and its fields.
@task_item n=6 part=untouched
PII scrubbing, veil redaction, the secret scan and CI artifact uploads.
@task_item n=7 part=untouched
CI gates, perf budgets, hyperfine gates and the SLO rows.
@task_item n=8 part=untouched
The diagnostics line schema, field names, log levels, file locations and modes, and service identity.
@task_item n=9 part=untouched
The events.ndjson product stream, state recovery and snapshot replay.
@task_item n=10 part=untouched
viola verify, its probes, the capability ledger and stamps, and the revive preflight.
