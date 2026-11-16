@task_prose part=session
The session widened how the wrapper confirms a driver's send: a hook payload's submitted prompt now claims the send when it equals the typed text exactly, or when it is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed.
That trailing-whitespace tolerance is PROVISIONAL on the delegate's answer (founder's word owed); trimming at the start or inside the text still claims nothing, and a refused character is still refused whole.
The relabel still writes origin driver alone and the event line keeps the submitted text; no event kind, refusal word, channel method, env var, config key or crate was added.
The send path's post-gate reads (human typing, turn running), the link record, send-issued and the paste now run under one flight lock, closing the race where a turn or human prompt the wrapper has learned of slips in between the readiness gate and the paste; refusal order is unchanged.
A residual stays open: a turn the CLI started whose UserPromptSubmit has not yet reached the wrapper before the paste (hook latency unmeasured), owned by the standing live rig carry.
The test-only fake agent gained an argv-only flag --trim-prompt-end that trims the trailing whitespace of its submitted prompt; no fixture changed.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook payload module — tolerant hook payload parsing and now the delivery-confirmation match predicate (exact equality or a trailing whitespace run removed) with its unit tables
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's send slot — the in-flight driver send, its delivery-confirmation claim, the session-start claim, turn start and end and the human-typing wheel move, all under one flight lock
@task_file n=3 path=src/run/send.rs
what: the wrapper's send method — the readiness gate, the human-typing and turn-running refusals, the link record, send-issued and the one bracketed paste, now under the flight lock
@task_file n=4 path=src/run/send/relabel.rs
what: the driver relabel of a hook event — origin driver written alone on a confirmed send, the submitted text kept; its unit tables
@task_file n=5 path=src/run/send/test_support.rs
what: unit-test helpers for the send method — the post-gate window and send-then-submit race drivers
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary under the fake-agent feature — its argv-only options and default UserPromptSubmit payload
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parse tests
@task_file n=8 path=tests/cli_send.rs
what: integration tests of viola send against a booted fake agent — delivery confirmation and the wheel
@task_item n=1 part=touches
decision/delivery-confirmation-exact-match: delivery is no longer confirmed by exact equality alone; a submitted prompt equal to the typed text with a trailing run of ASCII space, TAB, CR or LF removed (and non-empty) also confirms. A boundary widening, PROVISIONAL; the row is stale.
@task_item n=2 part=touches
decision/typed-text-line-ending-normalisation: its why says the delivery-confirmation match stays an exact equality; that is now stale, though typed text normalisation itself is unchanged.
@task_item n=3 part=touches
decision/human-typing-takes-wheel and contract/send-refusal/human-typing: the human-typing read after the gate now happens under the flight lock held across the paste, and a human prompt landing in the post-gate window refuses human-typing with nothing typed; a human key still takes no lock and never waits.
@task_item n=4 part=touches
contract/wheel-event/human-input: the unsent-human wheel move is now decided under the same flight lock as the claim; a prompt trimmed at the start still claims nothing and the wheel goes to the human.
@task_item n=5 part=touches
decision/bridge-send-one-bracketed-paste and decision/send-text-is-one-bracketed-paste: the paste is still one bracketed paste, now issued under the flight lock together with the link record and send-issued.
@task_item n=6 part=touches
contract/wrapper-channel-methods/send and contract/mcp-tools/send: send's refusal order and reply are unchanged but its post-gate section is now serialized against hook events; a turn started before the paste refuses not-delivered turn-running.
@task_item n=7 part=touches
contract/wrapper-channel-methods/hook-event and contract/channel-frames/hook-event-revalidation: hook.event handling now decides claim, session-start claim, turn start and end and wheel move under one lock; the frame shape and re-validation are unchanged.
@task_item n=8 part=touches
decision/tolerant-hook-parsing and decision/hook-payload-is-content: the hook prompt is now compared with a trailing whitespace tolerance; it stays content, never a command, and nothing is stripped from what is stored.
@task_item n=9 part=touches
decision/refused-text-never-stripped, decision/reject-dont-strip-paste-text and decision/refuse-control-characters: unchanged in force — the tolerance applies only to the comparison, a refused character is still refused whole — but a picker should check the wording against the new trailing-run comparison.
@task_item n=10 part=touches
decision/driver-permission-policy-outside-viola and decision/upstream-text-is-content: the driver relabel still writes origin driver alone and keeps the submitted text; the claim now accepts a trailing-trimmed form.
@task_item n=11 part=touches
decision/test-seams-fake-agent-only, decision/test-seams-compiled-out and decision/new-env-read-needs-log-entry: the fake agent gained the argv-only flag --trim-prompt-end, test-only, no env read.
@task_item n=12 part=touches
decision/fake-agent-hook-panic-seam and decision/fake-agent-pump-delay-seam: siblings of a new fake agent test option that trims the submitted prompt's trailing whitespace.
@task_item n=13 part=touches
decision/stdin-closed-non-editing-list: the human stdin path is untouched and takes no flight lock; check no row claims the send path and the human key share a lock.
@task_fact n=1 slug=threat-model-summary
fact: The wrapper's send reads human typing and turn running after the readiness gate, then records the link, writes send-issued and types the paste under one flight lock that hook events also take, so a turn or human prompt the wrapper has learned of before the paste refuses the send with nothing typed; a human key never takes that lock.
@task_fact n=2 slug=threat-model-summary
fact: Accepted residual: a turn the claude CLI has started whose UserPromptSubmit has not yet reached the wrapper before a send's paste stays beyond the lock (hook latency unmeasured), owned by the standing live rig carry.
@task_fact n=3 slug=threat-model-summary
fact: The delivery-confirmation trailing-whitespace tolerance (ASCII space, TAB, CR, LF, trailing side only) is PROVISIONAL pending the founder's word; either-end trimming is held as the candidate until a live reading of what the CLI trims at submit.
@task_item n=1 part=untouched
Dependency security: no dependency, pin, floor, feature, audit tool or workflow changed.
@task_item n=2 part=untouched
Secret management and the child environment: no env strip, credential handling or canary scan changed.
@task_item n=3 part=untouched
Socket, peer euid, server verification and strict-modes checks: no channel endpoint, check order or refusal changed.
@task_item n=4 part=untouched
Error handling refusal words and exit codes: no refusal word, error code, detail or CLI exit code was added or changed.
@task_item n=5 part=untouched
Data protection of logs, fixtures, diagnostics and pinned binaries: no event line shape, snapshot field, fixture or log mode changed; send-issued keeps text_bytes.
@task_item n=6 part=untouched
Input validation caps and validators: MAX_FRAME, the paste text character set, JSON depth and hook stdin bounds are unchanged.
@task_item n=7 part=untouched
Verify, revive, list, link, plugin install and MCP stdio server behaviour: none changed.
@task_item n=8 part=untouched
Absences: no new network surface, listener, account, secret or release stage.
