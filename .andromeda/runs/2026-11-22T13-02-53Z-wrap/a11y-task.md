@task_prose part=session
The wrapper's bracketed paste into the child's PTY now ends within a compiled bound, PASTE_BOUND 5 s, with no config key, env var or flag to set it; it falls on the driver's send only.
A paste that cannot finish is cut: the closing bracket marker is written with no Enter, and the send is refused not-delivered with the new closed detail word paste-cut (exit 13), its cursor kept, the wheel unmoved; a new hint line for paste-cut (wording PROVISIONAL) and a struck mirror line are pinned by literal.
After a cut, a cut mark refuses every later send paste-cut with nothing typed until a human key moves the wheel or a prompt-submitted, session-start or session-end record clears it; a release with no key or a turn end leaves it.
The shared input-writer lock is retired: one PTY writer thread under catch_unwind (three run PTY threads, was two) takes human keystrokes and pastes from one bounded queue in arrival order; a human key waits behind at most the in-flight chunk and the closing marker, never dropped, never refused.
send's longest delivered total moved from 48.5 s to 53.5 s, still under the MCP call deadline; the fake agent gained an argv-only --stop-reading option; no capability is claimed, the chunk closes Epoch 4.
Measured: a cut ends at about 5.3 s, a next send refused paste-cut in 9 ms, a send during a blocked paste refused turn-running at about 4 s; all suites and CI green.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: the outer-PTY pump of viola run's TUI passthrough: the human keystrokes copied into the child and the bracketed paste write, now one writer thread with a bounded queue; edited
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: the portable-pty outer-PTY boundary driver's surface: the paste handle, its time bound and its errors (the new paste-cut error); edited
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: the claude agent's compiled send time bounds that fall on the driver (the paste bound beside them); edited
@task_file n=4 path=crates/viola-core/src/lib.rs
what: the closed refusal reasons and their detail code words (the not-delivered detail list); edited
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP server frame tools and their call deadline, with the test holding the send-total arithmetic under it; edited
@task_file n=6 path=src/human/hints.rs
what: the refusal hint lines, one per code word, pinned by literal; edited
@task_file n=7 path=src/human/mirror.rs
what: the human mirror's struck line for a refused send, pinned by literal; edited
@task_file n=8 path=src/run/send.rs
what: viola send's delivery path and its refusal order (human typing, turn running, input not ready, not delivered); edited
@task_file n=9 path=src/run/send/slot.rs
what: the send slot's in-flight state, now holding the cut mark and its clears; edited
@task_file n=10 path=src/run/wheel.rs
what: the wheel, who holds the session (a human key takes it), now counting the human's editing-key reads; edited
@task_file n=11 path=src/cmd/run.rs
what: viola run's TUI passthrough entry and pump_child, which drives the paste; edited
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: viola verify's typed run step that pastes text into the child; edited
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent test double and its argv-only options; edited
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's stdin reader (a child that stops reading); edited
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parse tests; edited
@task_file n=16 path=tests/cli_send.rs
what: viola send's cross-process integration suite inside the existing test suites; edited
@task_item n=1 part=touches
decision/time-bounds-fall-on-the-driver: its list of driver bounds gains the paste write, PASTE_BOUND 5 s compiled; the cell was measured at about 297 of its 300 limit before the amendment, read it after.
@task_item n=2 part=touches
decision/time-bounds-never-on-human: gains the paste write, a bound that falls on the driver's send; the human's keys are never refused, only queued behind at most one chunk and the closing marker; the cell was near its 300 limit, read it after.
@task_item n=3 part=touches
contract/cli-exit-codes/13: the not-delivered details shown at exit 13 gain paste-cut.
@task_item n=4 part=touches
decision/human-keystrokes-never-blocked: human keystrokes now pass through one bounded write queue shared with the paste; a full queue holds keys in the terminal's own buffer, never dropped or refused; a key waits behind at most the in-flight paste chunk.
@task_item n=5 part=touches
contract/tui-passthrough/human-keystrokes: the human-keystroke path into the child now runs through the writer thread in arrival order with the paste.
@task_item n=6 part=touches
decision/send-refused-while-human-typing: the send refusal order gains two paste-cut rungs after the human-typing and turn-running reads (the cut mark), and after the paste (the cut, with its cursor).
@task_item n=7 part=touches
contract/send-refusal/input-not-ready: a paste that is cut no longer reads input-not-ready; any other paste failure stays input-not-ready with its cursor.
@task_item n=8 part=touches
decision/one-hint-per-code and contract/cli-refusal/hint-line and decision/refusal-ends-with-hint: the new paste-cut code word carries its own hint line, a human must look at the session before the next send.
@task_item n=9 part=touches
decision/fixed-copy-from-design: the paste-cut hint wording is the plan's PROVISIONAL text, not yet from design.
@task_item n=10 part=touches
decision/fixed-cli-strings-pinned and decision/fixed-cli-strings-pinned-by-literal and decision/refusal-layout-literal-pins: the paste-cut hint and the mirror's struck line for it are pinned by literal.
@task_item n=11 part=touches
decision/unlisted-code-words-stay-visible: a client built before the word reads exit 13 with paste-cut shown as the detail and no hint line, not unknown.
@task_item n=12 part=touches
decision/codes-beside-plain-word and decision/reason-code-beside-unable: paste-cut is a new kebab-case detail beside the not-delivered reason.
@task_item n=13 part=touches
decision/no-unwarned-timeouts and absence/user-time-limit: a new timeout exists, but on the driver's send only, ending with a closed word and a hint; no limit falls on the user.
@task_item n=14 part=touches
decision/human-key-takes-the-wheel and decision/non-editing-sequences: the wheel now counts each stdin read holding an editing key; that count is what clears the cut mark, a hook's human_input move counts nothing.
@task_item n=15 part=touches
decision/critical-path-send-readback: the confirmed send path gains a refused outcome, not-delivered paste-cut, and a sticky refusal until a human acts.
@task_item n=16 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: a third run PTY thread, the writer, runs under catch_unwind; its panic ends the run with the child killed; its teardown waits at most 2 s.
@task_item n=17 part=touches
contract/instance-unreachable-detail/call-deadline: send's longest delivered total under the call deadline moves from 48.5 s to 53.5 s, still below it.
@task_item n=18 part=touches
decision/portable-pty-outer-pty-driver: the boundary driver's paste call now takes a bound and can end cut.
@task_fact n=1 slug=cognitive-accessibility
fact: After a cut paste every later send is refused not-delivered paste-cut with nothing typed until a human editing key moves the wheel or a prompt-submitted, session-start or session-end record clears the mark; a release with no key or a turn end leaves it; the mark is held in memory only.
@task_fact n=2 slug=2-a11y-strategy
fact: A paste and the human's keystrokes share one write queue in arrival order; a human key typed during a paste waits behind at most the in-flight chunk of 4096 bytes and the closing marker, and is never dropped.
@task_fact n=3 slug=cognitive-accessibility
fact: A cut paste closes its bracket with no Enter, so a partial text is never submitted for the human; the paste bound is compiled and no config key, env var or flag sets it.
@task_item n=1 part=untouched
The viola list board rows (caption, order, colour, dim stale rows, NAME truncation, the unwrapped separator): no board output changed.
@task_item n=2 part=untouched
The screen-reader targets and manual Orca and keyboard passes: no reading surface or manual pass changed.
@task_item n=3 part=untouched
The revive, link and unlink, plugin install and viola --help contracts: none of their output or exit codes moved.
@task_item n=4 part=untouched
The socket-directory, strict-modes and server-verify refusals across frame verbs, hook calls and start: unchanged.
@task_item n=5 part=untouched
The CI integration, SLO and gate rows (no skips, zero flakiness, coverage and doctest, mutation audit): no gate, step or threshold changed; the added tests run inside the existing suites.
@task_item n=6 part=untouched
The absences of a web or graphical frontend, a touch surface and internationalization, and the N/A harness areas (contrast, focus, structured violation JSON, WCAG mapping): unchanged.
@task_item n=7 part=untouched
The dialog hold, wait timed_out and dialog-expired rows: the dialog and wait bounds did not move.
