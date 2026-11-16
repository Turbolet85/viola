@task_prose part=session
The driver send's confirmation now matches the CLI's submitted prompt when it equals the typed text, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; any other difference, a leading trim among them, still claims nothing and the wheel goes human.
That tolerance is PROVISIONAL: the CLI's trim set is an assumption on a 2.1.296 reading, not measured live this chunk, and the founder's word on trailing-only versus either end is owed.
The readiness-gate turn race is closed by a lock: after the gate returns ready, the human-typing and turn-running reads, the link record, send-issued and the paste run under one flight lock, and the wrapper decides the hook event's claim, the session-start claim, turn start and end and the unsent-human wheel move under that same lock; a human key never takes it.
A turn the CLI started whose UserPromptSubmit has not yet reached the wrapper before the paste stays beyond reach: hook latency is unmeasured, a named residual.
The fake agent gains an argv-only, off-by-default --trim-prompt-end option that trims the trailing run from a default UserPromptSubmit prompt while its receipt keeps the typed text.
No event kind, refusal word, IPC method, schema, config key, recorded fixture, dependency or CI step changed; local-command confirmation is unchanged.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: edited: the agent-claude hook payload parsing, now holding the send-confirmation match predicate over the submitted prompt and its trim set
@task_file n=2 path=src/run/send/slot.rs
what: edited: the send slot, the in-flight driver send that claims a prompt-submitted hook event as the driver's and moves the wheel on a human's unsent prompt
@task_file n=3 path=src/run/send.rs
what: edited: the driver send path: readiness gate, the post-gate refusals, the implicit link, send-issued and the paste
@task_file n=4 path=src/run/send/relabel.rs
what: edited: the prompt-submitted relabel to origin driver, and its claims-nothing case tables
@task_file n=5 path=src/run/send/test_support.rs
what: edited: the send module's shared unit test helpers
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: edited: the fake agent standing in for the claude CLI, its fired hook events and receipt
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited: the fake agent's command-line option parse cases
@task_file n=8 path=tests/cli_send.rs
what: edited: the viola send CLI integration suite over the fake agent
@task_item n=1 part=touches
Send confirmation by exact normalised match is now stale: the match takes the typed text exactly, or a non-empty submitted form equal to it with a trailing run of ASCII space, TAB, CR and LF removed; a leading trim claims nothing; guarded by the trailing-run tables, a 512-case property, the relabel table and the leading-trim case.
@task_item n=2 part=touches
send refusal order: order unchanged, but the post-gate human-typing and turn-running reads now run under the flight lock held across the link record, send-issued and the paste; a refusal drops the lock first and records no cursor, no link and no send-issued.
@task_item n=3 part=touches
The fake agent command line's modes and options list gains --trim-prompt-end, argv only, off by default.
@task_item n=4 part=touches
send confirmation, prompt-submitted with origin driver: the relabel now fires when the submitted prompt is the typed text or its trailing-trimmed form, no longer only when equal; it still writes origin driver alone and keeps the submitted text.
@task_item n=5 part=touches
driver send confirmation and viola send confirmed send: a send whose text ends in whitespace the CLI trims is now confirmed as the driver's own and keeps the wheel.
@task_item n=6 part=touches
viola send trailing newlines and text ending in or holding CR or LF: a trailing CR or LF the CLI trims no longer fails confirmation.
@task_item n=7 part=touches
viola send unconfirmed send and no-prompt-submitted: a submitted form that differs other than by a trailing whitespace run, a leading trim included, still confirms nothing.
@task_item n=8 part=touches
viola send turn-running refusal: a turn whose prompt-submitted reaches the wrapper after the gate but before the paste now refuses not-delivered turn-running with nothing typed.
@task_item n=9 part=touches
viola send human-typing refusal and its MCP twin: a human prompt submitted after the gate but before the paste now refuses human-typing with nothing typed.
@task_item n=10 part=touches
Harness-injected turns keep the wheel and prompt-submitted origin harness: an unclaimed prompt still reads harness or human, but the widened match moves which submitted prompts the driver claims.
@task_item n=11 part=touches
Human input takes the wheel and human editing takes the wheel: the unsent-human wheel move is now decided under the flight lock; a leading-trimmed prompt reads the wheel human.
@task_item n=12 part=touches
The human is never blocked and viola send human key during the gate wait: a lock was added to the send path, and a human key takes no flight lock and never waits on it.
@task_item n=13 part=touches
Sent text typed in normalised form: typed text is unchanged, but its delivery confirmation is no longer an exact equality.
@task_item n=14 part=touches
Send confirmation window expiry: the confirmation window waits with the flight lock released.
@task_item n=15 part=touches
Control characters in pasted text rejected and sent text validated at both ends: the match tolerates trailing whitespace only after validation; a refused character is still refused.
@task_item n=16 part=touches
Readiness gate tested by its refusals: the refusals read after the gate are now read under the lock that spans the paste, tested by two race cases.
@task_item n=17 part=touches
Fake agent configured by argv alone, fake-agent hook stdin payload and fake-agent receipt prompt: the new option trims the UserPromptSubmit prompt it fires while the receipt's prompt keeps the typed text.
@task_item n=18 part=touches
Fake agent held to recorded fixtures and fixture drift compared byte for byte: the new option is off by default and the drift contract stays byte for byte.
@task_fact n=1 slug=unit-test-strategy
fact: The send-confirmation trailing-whitespace tolerance is PROVISIONAL: the CLI's trim set (ASCII space, TAB, CR, LF, trailing side only) is an assumption, not a live measurement; a live reading of what the CLI trims at submit and the founder's word on trailing-only versus either end are owed.
@task_fact n=2 slug=unit-test-strategy
fact: The wrapper decides a hook event's send claim, the session-start claim, turn start and end and the unsent-human wheel move under one flight lock, the same lock a driver send holds from its post-gate reads through the paste; lock order flight before held.
@task_fact n=3 slug=unit-test-strategy
fact: Known limit: a turn the CLI started whose UserPromptSubmit has not reached the wrapper before a send's paste is not refused; hook latency is unmeasured and owned by the standing live rig.
@task_item n=1 part=untouched
CI integration rows: workflows, actions pinning, cargo-deny, fuzz replay, artifacts and the CI leg; no CI step changed.
@task_item n=2 part=untouched
Quality gates: coverage floors, mutation gate and thresholds, perf budgets and hyperfine; no mutation run and no threshold moved.
@task_item n=3 part=untouched
Test harness contract: agent-run status, boot, cleanup, gate, pre-push, secret scan and their exit codes and documents.
@task_item n=4 part=untouched
Dialog rows: hook decision bodies, dialog hold, answer mapping and viola answer.
@task_item n=5 part=untouched
viola run and revive start refusals, strict modes, socket directory checks and server identity checks.
@task_item n=6 part=untouched
viola list board, liveness, heartbeat staleness, snapshot replay and torn-append healing.
@task_item n=7 part=untouched
MCP tools list, MCP schema rows and the channel JSON-RPC error codes; no IPC method or tool added.
@task_item n=8 part=untouched
Absences: database, outbound HTTP, browser suite, deploy stage, Windows and macOS.
