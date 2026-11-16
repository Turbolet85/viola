@task_prose part=session
The session widened the driver's send confirmation: a submitted prompt now matches the driver's typed text when it is equal, or when it is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed (the whitespace the CLI trims at submit); that trailing-only set is PROVISIONAL, awaiting the founder's word.
A send whose text ends in such whitespace is now confirmed and read back as the driver's own, the wheel stays with the driver, and a refused character is still refused; a prompt trimmed at the start claims nothing and the wheel goes to the human.
It closed the readiness-gate turn race: after the gate reads ready, the human-typing and turn-running checks, the link record, the send-issued line and the paste run under one lock, and hook events decide the claim, the session start, the turn start and end and the unsent-human wheel move under that same lock; a turn the wrapper learns of before the paste refuses turn-running with nothing typed.
A human key never takes that lock: the stdin path is untouched and human keystrokes never wait on a send.
The test-only fake agent gained an argv flag that trims the end of the prompt it submits.
No refusal word, event kind, exit code, CLI output line, config key, IPC method or crate was added or retired; the residual hook-latency window (a turn the CLI starts whose submit has not reached the wrapper before the paste) stays open, owned by the standing live rig.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload reading of the internal claude agent crate, now holding the submitted-form predicate the send confirmation's exact match compares through
@task_file n=2 path=src/run/send/slot.rs
what: the send slot, the in-flight send's claim of a submitted prompt and the wheel move to the human on an unsent human prompt, under the flight lock
@task_file n=3 path=src/run/send.rs
what: viola send, the confirmed send after the readiness gate — the human-typing and turn-running refusals, the link record, send-issued and the paste
@task_file n=4 path=src/run/send/relabel.rs
what: the driver relabel of a prompt-submitted event as the driver's own on the send readback path, and its tables
@task_file n=5 path=src/run/send/test_support.rs
what: the send module's test helpers, driving the post-gate window race cases
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that stands in for claude in the boundary tests, its argv-only options
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option parse tests
@task_file n=8 path=tests/cli_send.rs
what: the viola send CLI integration suite, inside the existing nextest suites, covering confirmed send and readback
@task_item n=1 part=touches
The critical path confirmed send and readback: a driver's send whose text ends in trailing whitespace the CLI trims is now confirmed and read back rather than failing the exact match; the match is exact equality or the typed text less a trailing ASCII space, TAB, CR or LF run.
@task_item n=2 part=touches
The critical path human takes the wheel: a prompt trimmed at the start, or differing in any other way, still claims nothing and the wheel goes to the human; the unsent-human wheel move is now decided under the send's flight lock.
@task_item n=3 part=touches
Human keystrokes never blocked and human key takes the wheel: the new flight lock is never taken by a human key; the stdin path is unchanged, so these hold, re-asserted by the gate-wait human-key cases.
@task_item n=4 part=touches
Send refused while the human types: the post-gate human-typing check now runs under the flight lock held across the link, send-issued and the paste; the refusal order and words are unchanged and nothing is typed on a refusal.
@task_item n=5 part=touches
Any row stating the send confirmation or the readback is an exact match of the typed text is now stale: the claim tolerates the trailing trimmed run.
@task_item n=6 part=touches
The viola send refusals (input-not-ready, no-prompt-submitted, turn-running, human-typing): no word, exit code or layout changed, but a turn the wrapper knows of before the paste now always refuses turn-running with no link and no send-issued.
@task_item n=7 part=touches
Boundary tests sequenced on records and the boundary check never inspecting claude content: the fake agent that stands in for claude gains the argv-only flag --trim-prompt-end; it reads no env and the drift contract and recorded fixtures stay byte for byte.
@task_item n=8 part=touches
Time bounds fall on the driver only: the send's confirmation window still waits with the lock released, so no new bound falls on the human.
@task_item n=9 part=touches
Live readings never gate: no live claude start this session; the CLI's trimmed whitespace set stays an assumption, its live reading carried to the standing live rig.
@task_item n=10 part=touches
A11y assertions inside existing suites and a11y failures are nextest failures: the new send, relabel, hook and CLI send cases run inside the existing nextest suites with no separate a11y driver.
@task_item n=11 part=untouched
CLI output rows — the list board, its captions, colour and SGR rules, truncation, the --help rows, wait and verify lines: no printed line or escape sequence changed.
@task_item n=12 part=untouched
Refusal and fault formats — unable and hint lines, code words, --json failure and refusal documents, exit codes, the unknown refusal: no refusal word, hint or exit code added or moved.
@task_item n=13 part=untouched
TUI passthrough terminal-byte rows — zero viola bytes on the terminal, focus, mouse and resize sequences, vendor content, the vt100 feed panic, run diagnostics: the paste and stdin paths emit nothing new to the terminal.
@task_item n=14 part=untouched
Revive, link and unlink, plugin install, frame verb, MCP tool and socket-directory or strict-modes refusal rows: the session reached none of those commands.
@task_item n=15 part=untouched
CI integration, SLO and budget rows, the screen-reader targets, manual passes and the absences (web frontend, touch, i18n, user time limit): no CI step, gate, threshold or tool changed and no capability was added or removed.
