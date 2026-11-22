@task_prose part=session
The wrapper's paste into the child's PTY now ends within a compiled bound of 5 s (PASTE_BOUND), set by no config key, env var or flag.
A paste that cannot finish in time is cut: the writer stops after its in-flight chunk, closes the bracket with the end marker and types no Enter, and the send is refused not-delivered with the new closed detail paste-cut, the wheel unmoved.
After a cut, an in-memory cut mark refuses every next driver send paste-cut with nothing typed, until a human key, a prompt-submitted, a session-start or a session-end clears it.
The shared input-writer lock is retired: one writer thread under catch_unwind takes human key chunks and pastes from one bounded queue of 8 in arrival order, so a human key waits behind at most one 4096-byte chunk and the closing marker, never dropped.
The fake agent gains an argv-only --stop-reading test option; no dependency, env var, channel method, socket or config key was added; send's longest delivered total moves from 48.5 s to 53.5 s, still under the MCP call deadline.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: edited — the PTY pump: the child output copy threads and the writer to the child's PTY, now one writer thread under catch_unwind fed by a bounded job queue, the bounded paste write with its cut, and the run teardown that waits for the writer at most 2 s
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: edited — the PTY crate's public surface: the paste handle whose paste now takes a bound, and the PTY error enum gaining PasteCut with its fixed display message and no source
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: edited — the Claude adapter's compiled screen and timing constants, now holding the compiled paste bound PASTE_BOUND of 5 s
@task_file n=4 path=crates/viola-core/src/lib.rs
what: edited — the core crate's shared closed types: the closed not-delivered detail set, gaining the code word paste-cut
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: edited — the MCP frame tools and their call deadline; its test of the call-deadline arithmetic now counts the paste bound (53.5 s under the client read)
@task_file n=6 path=src/human/hints.rs
what: edited — the CLI stderr refusal hints, gaining the fixed paste-cut send hint literal (provisional wording)
@task_file n=7 path=src/human/mirror.rs
what: edited — the human-mode mirror of a refused send, its struck line pinned for paste-cut
@task_file n=8 path=src/run/send.rs
what: edited — the wrapper's send handler: the send refusal order, the bracketed-paste delivery and its refusals, now mapping a cut paste to not-delivered paste-cut with the send's cursor and adding the cut-mark rung
@task_file n=9 path=src/run/send/slot.rs
what: edited — the send slot's in-flight guard, now holding the in-memory cut mark and clearing it on prompt-submitted, session-start and session-end
@task_file n=10 path=src/run/wheel.rs
what: edited — the wheel (who holds the session: human or driver), now counting each stdin read holding an editing key
@task_file n=11 path=src/cmd/run.rs
what: edited — viola run, the wrapper process that spawns the child and wires its pump; its attached paste now passes the compiled bound
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: edited — viola verify's typed run against the child, its paste of text now passing the compiled bound
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent test binary (under the fake-agent feature), gaining the argv-only stop-reading option
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: edited — the fake agent's stdin reader, which can now park forever and read nothing
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited — the fake agent's argv option parse tests
@task_file n=16 path=tests/cli_send.rs
what: edited — the viola send cross-process integration tests, gaining the paste-cut cases against a child that stops reading
@task_item n=1 part=touches
decision/send-text-is-one-bracketed-paste and decision/bridge-send-one-bracketed-paste: send text stays one bracketed paste, except a cut paste — the bracket is closed with the end marker and no Enter is typed; the paste is now written in chunks of at most 4096 bytes.
@task_item n=2 part=touches
contract/channel-refusal/empty-text and contract/channel-refusal/control-character, and any row listing the closed not-delivered detail words: the closed set gains not-delivered/paste-cut beside them.
@task_item n=3 part=touches
contract/send-refusal/empty-text, contract/send-refusal/control-character and contract/send-refusal/human-typing: the send refusal order gains two paste-cut rungs — the cut mark after the human-typing and turn-running reads, and the cut itself after the paste, with the send's cursor.
@task_item n=4 part=touches
contract/mcp-tools/send and contract/wrapper-channel-methods/send: the send method may now refuse not-delivered paste-cut, and a send during a blocked paste waits on the flight guard up to the paste bound before reading turn-running.
@task_item n=5 part=touches
contract/mcp-structured-content/codes, contract/mcp-iserror/refusal and contract/mcp-iserror/detail: the refusal and detail code words the MCP send tool returns gain paste-cut.
@task_item n=6 part=touches
decision/human-typing-takes-wheel and contract/wheel-event/human-input: the wheel now counts each stdin read holding an editing key; a moved count clears the cut mark; a hook's human_input move counts nothing.
@task_item n=7 part=touches
decision/refusals-never-block-human: the paste bound and the cut mark fall on the driver's send only; human keys are never refused, only queued behind at most one chunk and the closing marker.
@task_item n=8 part=touches
decision/vt100-inside-catch-unwind and any row counting the run's threads under catch_unwind: the run's PTY threads under catch_unwind move from two copy threads to three with the writer thread; a writer panic kills the child.
@task_item n=9 part=touches
decision/fixed-error-display-messages and decision/error-messages-interpolate-nothing: the new PTY error PasteCut displays the fixed message pty paste cut at its bound with no source.
@task_item n=10 part=touches
decision/controls-not-switchable, decision/config-cannot-disable-controls and decision/no-control-switch-off: the new paste bound is a compiled constant no config key, env var or flag sets.
@task_item n=11 part=touches
decision/test-seams-fake-agent-only, decision/test-seams-compiled-out and decision/fake-agent-pump-delay-seam: the fake agent gains a new argv-only stop-reading test option, no env seam.
@task_item n=12 part=touches
contract/instance-unreachable-cause/call-deadline and decision/mcp-frame-tool-refusals (indirect, through the MCP call deadline): send's longest delivered total moves from 48.5 s to 53.5 s (a cut send 13.5 s), still under the MCP call deadline and the client read.
@task_item n=13 part=touches
decision/client-reply-code-words-only, decision/reply-code-words-validated and decision/client-reads-closed-values-only: a client built before the word reads exit 13 with paste-cut shown as the detail and no hint line, not unknown.
@task_item n=14 part=touches
contract/cli-stderr-hints/unwrapped-name-endpoint-lost and decision/human-output-escapes-controls (indirect, through the CLI stderr hints): a new fixed send hint for paste-cut joins the refusal hints, holding no upstream text.
@task_item n=15 part=touches
decision/paste-validation-on-both-sides and decision/wrapper-checks-authoritatively (indirect): text validation is unchanged and still upstream of the paste; only the write after it is now bounded and chunked.
@task_item n=16 part=touches
decision/hook-fails-open and contract/wrapper-channel-methods/hook-event (indirect): a hook event arriving during a blocked paste still lands within the paste bound; prompt-submitted, session-start and session-end clear the cut mark.
@task_item n=17 part=touches
decision/sync-crates-tokio-free and decision/no-tokio-in-sync-crates (indirect, confirming): the PTY crate's new writer thread uses std threads, a bounded std channel and atomics only, no Tokio.
@task_fact n=1 slug=threat-model-summary
fact: The wrapper's paste write into the child's PTY is bounded by a compiled 5 s bound (PASTE_BOUND) that no config key, env var or flag sets; a paste not finished by then, or still queued at its deadline, is cut — the bracket closed with no Enter — and the send refused not-delivered paste-cut with its cursor.
@task_fact n=2 slug=threat-model-summary
fact: After a cut paste the wrapper holds a cut mark in memory only; every next driver send is refused paste-cut with nothing typed until a human key moves the wheel's key count or a prompt-submitted, session-start or session-end clears it; a release with no key or a turn-ended leaves it.
@task_fact n=3 slug=threat-model-summary
fact: One PTY writer thread takes human key chunks and paste jobs from one bounded queue of 8 in arrival order; a full queue makes the human's keys wait in the terminal's own buffer, never dropped, and a human key waits behind at most one 4096-byte chunk and the closing marker.
@task_fact n=4 slug=input-validation
fact: The PTY writer thread runs under catch_unwind like the copy threads; a writer panic ends the run with the child killed, and teardown waits for the writer at most 2 s, leaving it detached past that.
@task_fact n=5 slug=error-handling
fact: The closed not-delivered detail set holds paste-cut, appended last; the channel refusal reads not-delivered/paste-cut and the CLI exits 13 with it.
@task_fact n=6 slug=input-validation
fact: The fake agent's stop-reading option is an argv-only test seam under the fake-agent feature with no env seam; it parks the child's stdin reader forever with the terminal left raw.
@task_item n=1 part=untouched
Authentication and authorization: peer euid checks, socket directory checks, socket modes, strict-modes checks and server verification are unchanged; no channel method, socket or endpoint was added.
@task_item n=2 part=untouched
Dependency security: no crate added or bumped, no feature changed, no CI workflow, action pin, advisory ignore, licence list or toolchain pin moved.
@task_item n=3 part=untouched
Secret management: no env var read, credential strip, canary scan or child environment rule changed.
@task_item n=4 part=untouched
Data protection: no file mode, log, rotation, fixture scrub, pinned binary, trust record or plugin file rule changed; the paste span logs no text.
@task_item n=5 part=untouched
API security: the rmcp feature set and its bans are unchanged.
@task_item n=6 part=untouched
Input validation of text and frames: the paste text character set, the control-character and empty-text rules, frame caps, MAX_FRAME, hook stdin and MCP stdin line caps are unchanged.
@task_item n=7 part=untouched
Answer and dialog rows: the answer path, its refusals and its dialog free text are unchanged; only send's paste is bounded.
@task_item n=8 part=untouched
The absences (network, hosting, release stage, secrets, accounts, database, webhook): no capability added that any of them rules out.
