@task_prose part=session
The wrapper's paste of sent text into the child's PTY is now bounded by a compiled 5 s PASTE_BOUND (no config key, env var or flag sets it); a paste that cannot finish in time is cut, the bracket closed with no Enter, and the send refused not-delivered with the new closed detail word paste-cut, exit 13, with the send's cursor.
One PTY writer thread now takes human key chunks and paste jobs from one bounded queue (8) in arrival order; the paste is written in chunks of at most 4096 bytes; a human key waits behind at most the in-flight chunk and the closing marker, never dropped; the shared input-writer lock and its one-write paste are retired. The run's PTY threads under catch_unwind go from two to three.
A cut leaves a cut mark in memory: every next send is refused paste-cut with no cursor until a human key read moves the wheel's key count, or a prompt-submitted, session-start or session-end record clears it; the send refusal order gains two paste-cut rungs.
The fake agent gains an argv-only --stop-reading option (it never reads stdin); four cross-process send cases, new unit and table cases, and a literal-pinned paste-cut hint and struck line were added; send's longest delivered total moved 48.5 s to 53.5 s, still under the MCP call deadline.
Measured: a cut lands at 5.3 s; a send during a blocked paste waits on the flight guard up to the bound then reads turn-running (4.0 s); a hook record during a blocked paste lands within the bound. No capability is claimed; this chunk closes Epoch 4; CI green 7 of 7.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: the PTY pump — the child's PTY input writer, now one writer thread over a bounded job queue carrying human input and the bracketed paste, its bounded paste and cut, and its teardown join
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: the PTY crate's surface — the paste handle's paste call now taking a bound, the PTY error set gaining the paste-cut error, and its real-openpty tests' child kill-line comment
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: the claude agent's compiled constants — the new compiled PASTE_BOUND of 5 s for the driver's paste write
@task_file n=4 path=crates/viola-core/src/lib.rs
what: the shared code words — the closed not-delivered detail set, gaining paste-cut
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP frame tools — their call deadline and the test holding it between the wrapper's send bounds and the client read (send total 53.5 s)
@task_file n=6 path=src/human/hints.rs
what: the human-mode refusal hints for send and answer — a new hint for not-delivered paste-cut
@task_file n=7 path=src/human/mirror.rs
what: the human-mode mirror of driver sends — the struck line for a refused send, pinned by literal for paste-cut
@task_file n=8 path=src/run/send.rs
what: the wrapper's send handler — the send refusal order, the bracketed paste of the typed text, send-issued and send-refused records, and the paste-failure mapping to input-not-ready or paste-cut
@task_file n=9 path=src/run/send/slot.rs
what: the send slot's in-flight state under the flight guard — now holding the cut mark and its clears on prompt-submitted, session-start and session-end
@task_file n=10 path=src/run/wheel.rs
what: the wheel — who holds it, human editing taking it, and now a count of stdin reads holding an editing key
@task_file n=11 path=src/cmd/run.rs
what: viola run's wrapper start and child pump — its attached paste closure passing the paste bound
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: viola verify's typed-run driver against the agent — its paste of text passing the paste bound
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent binary standing in for claude — its argv options, gaining --stop-reading
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input reader — keystrokes and bracketed paste, and now a read-nothing park for --stop-reading
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's command-line parse cases — a case for the --stop-reading flag
@task_file n=16 path=tests/cli_send.rs
what: the viola send cross-process integration suite over the fake agent — four new paste-cut and blocked-paste cases
@task_item n=1 part=touches
decision/send-refusal-order: the send refusal order gains two paste-cut rungs — the cut mark's rung after the second human-typing and turn-running reads and before the link (no cursor), and the cut after the paste (with the send's cursor); six table rows were added.
@task_item n=2 part=touches
contract/viola-send/not-delivered and contract/cli-exit-codes/13 (and contract/cli-exit-codes/10-14, contract/cli-exit-codes/13-text-refused where they list details): the closed not-delivered detail set and exit 13 gain paste-cut.
@task_item n=3 part=touches
contract/driver-send/typed-text, contract/viola-send/paste-framing and contract/send-event-log/send-issued: except a cut — a cut paste closes the bracket with no CR, after send-issued, and is refused paste-cut rather than confirmed.
@task_item n=4 part=touches
contract/fake-agent-cli/argv-only-options: the fake agent's argv option count moves by one (+ --stop-reading); decision/fake-agent-argv-only holds (argv only, no env seam).
@task_item n=5 part=touches
contract/viola-send-human-mode/unable and contract/cli-send-json not-delivered rows: a new paste-cut refusal hint line and struck mirror line, pinned by literal (PROVISIONAL wording).
@task_item n=6 part=touches
decision/human-never-blocked, decision/human-input-takes-the-wheel, decision/human-takes-wheel-on-edit: human keys are never refused during a paste, only queued behind at most one chunk and the closing marker; the wheel now counts stdin reads holding an editing key, and a hook's human-input move counts nothing.
@task_item n=7 part=touches
decision/multi-line-send-one-prompt, decision/sent-text-typed-form, contract/fake-agent-input/bracketed-paste: the paste is now written in chunks of at most 4096 bytes inside one bracket — still one prompt, except a cut, which ends the bracket with no Enter.
@task_item n=8 part=touches
contract/instance-unreachable-cause/call-deadline and contract/mcp-unreachable-detail/call-deadline: send's longest delivered total moved 48.5 s to 53.5 s (fallback window 18.5 to 23.5 s; a cut send 13.5 s), still below the MCP call deadline.
@task_item n=9 part=touches
decision/no-sleep-synchronization and decision/no-test-retries-or-sleeps: two new cross-process cases place a second send or a hook about 1 s into a blocked paste by a sleep after the first send's send-issued record — a placement, not a synchronization; a picker should judge whether that needs an exception.
@task_item n=10 part=touches
decision/unknown-refusal-word-stays-visible: a client built before the word reads exit 13 with paste-cut shown as the detail and no hint line.
@task_item n=11 part=touches
decision/closed-enums-need-decisions-log: the closed not-delivered enum gains paste-cut, appended last.
@task_item n=12 part=touches
decision/vt100-panic-contained, decision/vt100-panic-keeps-passthrough, decision/outer-pty-reader-drained: the run's PTY threads under catch_unwind go from two copy threads to three (the writer thread); a writer panic kills the child.
@task_item n=13 part=touches
decision/pty-tested-on-real-openpty and decision/pty-tests-real-openpty: five new PTY cases (bound cut, cut with no Enter, human key behind one chunk, pump returning with the writer blocked) run on real openpty with a child that stops reading.
@task_item n=14 part=touches
decision/paste-injection-test-layers, decision/paste-validated-at-both-ends, decision/paste-control-characters-rejected: the paste write is newly bounded; text validation is unchanged upstream.
@task_item n=15 part=touches
decision/mutants-kill-below-cargo-mutants-timeout, decision/send-window-paste-hint-30s-kill, decision/within-7s-wait-bound, decision/root-wait-bound: the PTY tests' CHILD_WITHIN comment now names the mutants profile's 15 s kill (5 s x 3), not 10 s; and a send can now wait up to the 5 s paste bound — read against any kill line or wait bound a row's cell holds.
@task_item n=16 part=touches
contract/viola-send/turn-running and contract/viola-send/human-key-during-gate: a second send during a blocked paste waits on the flight guard up to the paste bound, then is refused turn-running (measured 4.0 s); refused, never queued.
@task_item n=17 part=touches
contract/cli-send/input-not-ready: any paste error other than a cut stays input-not-ready with its cursor.
@task_item n=18 part=touches
decision/hints-never-suggest-release and decision/no-release-in-driver-hints: the new paste-cut hint says a human must look at the session before the next send — read it against the no-release rule.
@task_item n=19 part=touches
contract/mcp-cli-parity/refusal and contract/mcp-reply/screened-words: the paste-cut refusal word reaches MCP send as it does the CLI.
@task_item n=20 part=touches
decision/sync-crates-without-tokio and decision/tokio-banned-per-sync-crate: the PTY crate's new writer thread uses std mpsc, thread and atomic only — no Tokio, no dependency added.
@task_fact n=1 slug=unit-test-strategy
fact: After a paste-cut, the send slot keeps a cut mark in memory set to the wheel's key count; every next send is refused not-delivered / paste-cut with no cursor until a human key read moves that count or a prompt-submitted (any origin), session-start or session-end record clears it; a release with no key and a turn-ended leave it.
@task_fact n=2 slug=7-test-data-fixtures
fact: viola-fake-agent --stop-reading (argv only): the fake never reads stdin, parked forever with its terminal left raw, so a paste into it blocks.
@task_fact n=3 slug=6-e2e-test-strategy
fact: viola send into a child that stops reading ends exit 13 not-delivered / paste-cut within the compiled 5 s paste bound (measured 5.3 s, under 12 s), with the send's cursor, no Enter typed, and the next send refused paste-cut with nothing issued.
@task_fact n=4 slug=1-test-scope-summary
fact: During a driver paste, a human key is never refused and never dropped: it waits behind at most the in-flight paste chunk (at most 4096 bytes) and the closing marker.
@task_fact n=5 slug=6-e2e-test-strategy
fact: A hook record during a blocked paste lands within the paste bound, and a send during a blocked paste is refused turn-running after waiting at most the bound.
@task_fact n=6 slug=unit-test-strategy
fact: A paste still queued at its deadline is a cut with nothing written: refused paste-cut with the send's cursor, and it sets the cut mark like a mid-write cut.
@task_item n=1 part=untouched
The harness contract — agent-run boot, status, cleanup, run, logs, session record, supervisor and their exit codes and reasons; no harness command, status shape or CI step changed.
@task_item n=2 part=untouched
CI integration and supply chain — workflows, SHA pins, cargo-deny, zizmor, MSRV, toolchain pins, artifacts and their retention; no dependency or tool version moved.
@task_item n=3 part=untouched
Coverage floors, the mutation gate's thresholds and cadence, and performance budgets of the hooks; no gate threshold moved.
@task_item n=4 part=untouched
Dialog and answer contracts, hook decision bodies, viola hook exit and stderr, and the answer refusal order; answer hints are unchanged.
@task_item n=5 part=untouched
list, revive, link and unlink, wait and last, release contracts; strict-modes, socket-directory, home-tree and server-identity checks.
@task_item n=6 part=untouched
Recorded claude fixtures, verify stamps and fixture hygiene; the process log line format and its corr fields — no new log event or field (the cut shows as the existing send-refused record).
@task_item n=7 part=untouched
Diagnostics schema: the send-refused detail stays an open string; no config key, env var or flag added.
