@task_prose part=session
The wrapper's bracketed paste into the child's PTY now ends within a compiled bound, PASTE_BOUND of 5 s, counted from the call; no config key, env var or flag sets it.
A paste that cannot finish in time is cut: the writer stops after its in-flight chunk of at most 4096 bytes, writes the closing paste marker and no Enter, and the send is refused not-delivered with the new closed detail paste-cut, with the send's cursor; the wheel does not move.
The shared input-writer lock and its one-write paste are retired: one std writer thread under catch_unwind takes human key chunks and paste jobs in arrival order from a bounded queue of 8; human keys wait behind at most one chunk and the closing marker, never dropped; a paste still queued at its deadline is a cut with nothing written.
A cut leaves an in-memory cut mark on the send flight: the next send is refused paste-cut with no cursor until a human key read, a prompt-submitted, a session-start or a session-end clears it.
The longest delivered send total moves from 48.5 s to 53.5 s, still under the 55 s MCP call deadline; a new human hint line for paste-cut (provisional wording) and a fake-agent argv option that stops reading stdin come with it.
No capability is claimed (a dated ledger note only), no dependency, IPC method, socket, schema or CI step changed; this chunk closes Epoch 4.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: viola's PTY seam pump: the stdin passthrough to the child, now the one PTY writer thread with its bounded job queue, the chunked bracketed paste with its cut, and the writer's bounded teardown; edited
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: viola's PTY seam surface: the paste handle whose paste now takes a bound, and the hand-written PTY error type gaining the PasteCut variant; edited
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: the Claude agent crate's compiled screen and timing literals, now holding the compiled PASTE_BOUND of 5 s; edited
@task_file n=4 path=crates/viola-core/src/lib.rs
what: viola-core's shared vocabulary, the closed not-delivered refusal detail set, gaining paste-cut; edited
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP frame tools and their call deadline, whose test arithmetic of the wrapper bounds against the 55 s deadline now counts the paste bound; edited
@task_file n=6 path=src/human/hints.rs
what: the human hint lines for send refusals, gaining the paste-cut hint; edited
@task_file n=7 path=src/human/mirror.rs
what: the human mirror of driver sends with its struck refusal line, gaining the paste-cut line pin; edited
@task_file n=8 path=src/run/send.rs
what: the wrapper's send path and its refusal order under the flight lock, now mapping a cut paste to not-delivered paste-cut and holding the cut-mark rung; edited
@task_file n=9 path=src/run/send/slot.rs
what: the send slot and its in-flight state, gaining the in-memory cut mark and its clears on hook events; edited
@task_file n=10 path=src/run/wheel.rs
what: the wrapper's in-memory wheel and its byte-source classifier, now counting human editing-key reads; edited
@task_file n=11 path=src/cmd/run.rs
what: the run command that starts the wrapped child and its pump, passing the paste bound to the attached paste; edited
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: verify's typed probe run that pastes probe text into the child, passing the paste bound; edited
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary standing in for claude, gaining an argv option that stops reading its input; edited
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input reader, gaining a read that parks forever; edited
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing unit cases, gaining the stop-reading case; edited
@task_file n=16 path=tests/cli_send.rs
what: the cross-process send integration cases, gaining the paste-cut, after-cut, during-blocked-paste and hook-during-blocked-paste cases; edited
@task_file n=17 path=.andromeda/runs/2026-11-22T12-03-19Z-implement/evidence/harness.py
what: a measuring harness kept as gitignored run evidence, outside the product; one line rewritten path-free
@task_item n=1 part=touches
decision/send-time-bound: the send's time bounds gain the paste write bound PASTE_BOUND of 5 s, compiled, set by no config key, env var or flag.
@task_item n=2 part=touches
decision/atomic-bracketed-send: the send stays one bracketed paste except a cut; a cut paste closes the bracket with no Enter after its in-flight chunk, and a human key waits behind at most one chunk and the closing marker.
@task_item n=3 part=touches
decision/mcp-call-deadline: its arithmetic of the wrapper bounds moves from 48.5 s to 53.5 s for the longest delivered send (fallback window 18.5 s to 23.5 s, a cut send 13.5 s), still under the 55 s deadline.
@task_item n=4 part=touches
contract/refusal-reason/not-delivered and decision/refusal-details-closed-set: the closed not-delivered detail set gains paste-cut, appended last.
@task_item n=5 part=touches
contract/not-delivered-details/empty-text, input-not-ready and no-prompt-submitted: a new sibling detail paste-cut stands beside them; the existing rows keep their words.
@task_item n=6 part=touches
contract/refusal-order/send and contract/send-refusal/order: two paste-cut rungs join the send refusal order, the cut mark after the second human-typing and turn-running reads and before the link, and the cut itself after the paste.
@task_item n=7 part=touches
contract/send/paste-failure-after-issue: a paste failing after send-issued stays input-not-ready with its cursor except a cut, which is refused not-delivered paste-cut with the send's cursor.
@task_item n=8 part=touches
contract/cli-exit-codes/13: exit 13 now also carries the paste-cut detail; an older client reads exit 13 with paste-cut shown as the detail and no hint line, not unknown.
@task_item n=9 part=touches
decision/send-never-queued: read against the build, a second send during a blocked paste waits on the flight guard up to the paste bound, then is refused turn-running, nothing typed for it; its words stand, the wait is the bound's.
@task_item n=10 part=touches
decision/send-flight-lock: the flight guard is held through the bounded paste, and the cut mark is set and read under it.
@task_item n=11 part=touches
decision/human-always-wins-wheel, decision/human-key-never-held-behind-resize and decision/passthrough-never-waits-on-model: human stdin chunks now go through the one writer queue of 8, waiting behind at most one paste chunk and never dropped; stdin reading blocks on a full queue.
@task_item n=12 part=touches
decision/portable-pty-seam and decision/pty-knows-no-agent: the pty seam's paste now takes a bound from its caller; the bound's value lives in the agent crate, viola-pty still knows no agent.
@task_item n=13 part=touches
decision/pty-error-hand-written and decision/thiserror-messages-interpolate-nothing: the PTY error type gains PasteCut with a fixed message and no source.
@task_item n=14 part=touches
decision/hybrid-concurrency, decision/sync-core-tokio-only-in-mcp and decision/sync-crates-tokio-free: the new PTY writer is a std thread with a std bounded channel, no Tokio; the run's PTY threads under catch_unwind go from two to three.
@task_item n=15 part=touches
decision/gate-timings-compiled, decision/confirm-window-compiled and decision/screen-signatures-compiled: the agent crate's compiled literals gain PASTE_BOUND beside them.
@task_item n=16 part=touches
decision/wheel-in-wrapper-memory and decision/wheel-classifier-only-observes: the wheel now counts each stdin read holding an editing key; a hook's human-input move counts nothing; the count only observes.
@task_item n=17 part=touches
decision/unconfirmed-delivery-not-delivered, decision/every-send-confirmed and decision/send-confirmed-after-the-fact: a cut send is issued but never confirmed and is reported not-delivered paste-cut.
@task_item n=18 part=touches
contract/human-hint-lines/input-not-ready and contract/human-stderr/refusal: a paste-cut hint line joins the send hints (provisional wording); the answer hint is unchanged.
@task_item n=19 part=touches
contract/event-data/send-refused and contract/wrapper-event-appends/send-records: a cut shows as a send-refused record with detail paste-cut; the cut mark writes no line of its own and its clears are their own records.
@task_item n=20 part=touches
contract/mcp-tools/send and contract/mcp-tool-result/refusal: the MCP send relays the paste-cut refusal through the existing refusal result; no tool or field added.
@task_item n=21 part=touches
decision/fake-agent-test-seams and decision/test-seams-not-configuration: the fake agent gains an argv-only option that stops reading its input, with no env seam.
@task_item n=22 part=touches
decision/bounded-readers: the PTY paste write is now bounded beside the bounded readers, and the writer's teardown waits at most 2 s, leaving the thread detached past that.
@task_fact n=1 slug=established-decisions
fact: A paste not finished within the compiled paste bound, counted from the call, is cut: the writer stops after its in-flight chunk of at most 4096 bytes, writes the closing paste marker with no Enter, and the send is refused not-delivered paste-cut with its cursor and no wheel move; a paste still queued at the deadline is a cut with nothing written.
@task_fact n=2 slug=established-decisions
fact: The cut mark: a cut sets an in-memory mark at the wheel's human-key count; while the count is unchanged the next send is refused not-delivered paste-cut with no cursor; a human key read, a prompt-submitted of any origin, a session-start or a session-end clears it; a release with no key or a turn-ended leaves it; it writes no line of its own.
@task_fact n=3 slug=established-decisions
fact: All writes into the child's PTY go through one writer thread under catch_unwind taking human key chunks and paste jobs in arrival order from a bounded queue of 8; human keys block on a full queue and are never dropped; a writer panic ends the run with the child killed.
@task_fact n=4 slug=established-decisions
fact: contract not-delivered-details paste-cut: the closed detail for a send whose paste was cut at its bound or refused by a standing cut mark, exit 13, with its human hint line that a human must look at the session before the next send (provisional wording).
@task_item n=1 part=untouched
Dialog hooks, the dialog hold, answer and its refusal order: no hook tier, dialog rule or answer path changed.
@task_item n=2 part=untouched
list, the board, session liveness, snapshots and their replay: nothing read or written on disk changed shape.
@task_item n=3 part=untouched
revive, the pinned copy, plugin install and the Hyprland seam: untouched.
@task_item n=4 part=untouched
link, unlink, pause, release, wait and last channel methods and their CLI verbs: no method, param or result changed.
@task_item n=5 part=untouched
Occupied resources and names: no env var, socket, port, config key, binary, command, crate or channel method added or freed.
@task_item n=6 part=untouched
Build system, dependencies and CI: no dependency added or bumped, no lint, gate, coverage floor or workflow step changed.
@task_item n=7 part=untouched
The capability ledger and its rows: no capability claimed; only a dated ledger note outside architecture.
@task_item n=8 part=untouched
The diagnostics line and log formats: the send-refused detail stays an open string in its schema; no event kind added.
