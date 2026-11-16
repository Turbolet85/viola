@task_prose part=session
The delivery-confirmation claim now matches a driver's send when the submitted prompt equals the typed text exactly, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; leading or inner differences still claim nothing. The trim set is an assumption and the trailing-only side is PROVISIONAL, the founder's word owed.
The relabel still writes origin driver alone and the prompt-submitted line keeps the submitted text; no event kind, refusal word, event field, snapshot field, config key, IPC method, env var, crate or dependency was added.
The readiness-gate turn race is closed by a lock: after the gate returns ready, the human-typing and turn-running reads, the link record, send-issued and the paste run under one flight lock, and the wrapper decides the claim, the session-start claim, turn start and end and the unsent-human wheel move under that same lock; refusal order unchanged, a human key takes no flight lock.
A residual stays: a turn the CLI has started whose UserPromptSubmit has not reached the wrapper before the paste is beyond reach (hook latency unmeasured), carried to the standing live rig with a screen model.
The test-only fake agent gains an argv flag --trim-prompt-end that trims the trailing whitespace run from its UserPromptSubmit prompt; argv only, no env read.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: the Claude hook payload shapes and normalisation in the agent crate, now holding the submitted-form predicate and the trim set the delivery-confirmation claim compares through
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's in-memory send flight and wheel state, where a hook.event's prompt is claimed as the driver's send and the turn and wheel moves are decided
@task_file n=3 path=src/run/send.rs
what: the wrapper's send method behind the channel, running the readiness gate, the refusal reads, the link record, send-issued and the bracketed paste
@task_file n=4 path=src/run/send/relabel.rs
what: the driver relabel of a prompt-submitted line to origin driver and its claim tests
@task_file n=5 path=src/run/send/test_support.rs
what: test support for the send module's race and confirmation cases
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary standing in for the claude CLI, its UserPromptSubmit payload
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parsing tests
@task_file n=8 path=tests/cli_send.rs
what: the end-to-end tests of the send CLI verb against the fake agent
@task_item n=1 part=touches
decision/exact-prompt-match: the prompt match after agent normalisation is no longer exact equality alone; it also takes the typed text with a trailing run of ASCII space, TAB, CR, LF removed, PROVISIONAL.
@task_item n=2 part=touches
contract/hook-event/driver-relabel: the relabel fires when the submitted prompt equals the typed text or its trailing-trimmed form, no longer only when equal to it; it still sets origin alone and keeps the submitted text.
@task_item n=3 part=touches
contract/channel-methods/hook-event-driver-origin: the same widening of the equal-to condition for setting origin driver on a hook.event.
@task_item n=4 part=touches
decision/typed-text-normalisation: its why names the exact match a trimmed prompt failed; the typed text itself is unchanged, the why's history to be re-read against the tolerance.
@task_item n=5 part=touches
decision/every-send-confirmed, decision/send-confirmed-after-the-fact and decision/unconfirmed-delivery-not-delivered: a send whose text ends in whitespace the CLI trims is now confirmed as the driver's own instead of reading unconfirmed; send-confirmed line unchanged.
@task_item n=6 part=touches
contract/event-data/prompt-submitted-origin, contract/event-prompt-submitted/origin and their text rows: origin driver now also lands on a trailing-trimmed prompt; text stays the submitted text.
@task_item n=7 part=touches
decision/send-never-queued, contract/refusal-order/send and contract/send-refusal/order: the post-gate human-typing and turn-running reads now run under the flight lock held across the link, send-issued and the paste; the order and refusal words are unchanged.
@task_item n=8 part=touches
contract/refusal-reason/not-delivered and contract/refusal-reason/human-typing: a turn or human prompt reaching the wrapper after the gate but before the paste is now refused turn-running or human-typing with nothing typed, no link and no send-issued.
@task_item n=9 part=touches
contract/wrapper-event-appends/send-records, contract/event-data/send-issued and contract/channel-methods/send: send-issued and the link record are now written under the flight lock; shapes and text_bytes unchanged; a refusal drops the lock first.
@task_item n=10 part=touches
decision/wheel-in-wrapper-memory, decision/human-prompt-confirms-wheel-take and decision/byte-source-wheel-claim: the claim, the session-start claim, turn start and end and the unsent-human wheel move are decided under one flight guard; a human's unsent prompt and a leading-trimmed prompt still take the wheel human.
@task_item n=11 part=touches
decision/human-always-wins-wheel and decision/human-key-never-held-behind-resize: a human key takes no flight lock and never waits on a send's paste section.
@task_item n=12 part=touches
decision/claude-shapes-in-one-crate, decision/claude-shapes-isolated and decision/agent-claude-pure: the CLI's trim set and the submitted-form predicate live in the Claude agent crate as a pure function, the root bin calling it.
@task_item n=13 part=touches
decision/control-characters-refused, contract/paste-text/control-characters and decision/send-never-types-trailing-newline: nothing is stripped from the typed text; a refused character is still refused; the tolerance applies only on the read-back side.
@task_item n=14 part=touches
decision/capability-ledger, decision/cli-capability-ledger and decision/no-probe-for-unrelied-shapes: no ledger row added; the CLI's trim set rests on an unmeasured assumption from an earlier claude 2.1.296 instance, its live reading owed to the standing live rig.
@task_item n=15 part=touches
decision/fake-agent-test-seams, decision/test-seams-not-configuration and decision/fake-agent-after-double-dash: the fake agent gains an argv-only flag --trim-prompt-end, no env read.
@task_fact n=1 slug=established-decisions
fact: A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes it.
@task_fact n=2 slug=established-decisions
fact: The send race is closed only for turns the wrapper has learned of before the paste; a turn the CLI started whose UserPromptSubmit has not yet reached the wrapper is an accepted residual, hook latency unmeasured.
@task_fact n=3 slug=established-decisions
fact: The delivery-confirmation trailing-whitespace tolerance is PROVISIONAL: trailing-only stands on the delegate's answer, either-end held as the candidate pending the founder's word and a live reading of the CLI's trim.
@task_item n=1 part=untouched
Hook tiers, hook exit codes, hook-to-kind map and dialog handling: no hook output, tier or dialog path changed.
@task_item n=2 part=untouched
MCP tools, results and the mcp server: no tool or result shape changed.
@task_item n=3 part=untouched
revive, verify, list, the board, pause, link and unlink verbs and their exit codes and JSON documents.
@task_item n=4 part=untouched
Stack, dependencies, build system, CI, deployment and crate dependency direction: no crate, dependency, CI step or workflow changed.
@task_item n=5 part=untouched
Occupied names and env vars, sockets, endpoints and test homes: no name taken or freed.
@task_item n=6 part=untouched
Channel frames, fault codes, protocol version, snapshot envelope and config.json: no frame, field or key changed.
@task_item n=7 part=untouched
decision/local-commands-by-ledger and local command post-conditions: a local command's own exact match is unchanged.
@task_item n=8 part=untouched
Event kinds and the refusal-details closed set: no kind or refusal word added.
