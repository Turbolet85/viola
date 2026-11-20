@task_prose part=session
Under viola run, a prompt fired by a CLI-scheduled wake is now told apart from a typed one, so the wheel stays with the driver and its next viola send and viola answer still go through.
The rule: no key typed into the wrapped box since the last human-filed prompt, a wake counted at the last turn end, and a verified CLI; then the prompt reads origin harness with no wheel move, otherwise it takes the wheel for the human as before.
The typed mark clears only at the next human-filed prompt, never at a turn end or a release, so a message queued during a turn still reads human.
viola verify gains a 26th relied ledger row, stop-session-crons (words: Stop carries session_crons as an array); the relied rows went 25 to 26, the step lines [NN/25] to [NN/26], the stamped summary 25 pass to 26 pass, the floor rows 21 to 22.
The test-only fake agent gained two argv options for synthetic crons and a fired wake; the live rig gained the WS, HB and R3W labels and their rehearsals; live readings on claude 2.1.328 measured one fired interval wake kept on the driver's wheel.
No CLI verb, flag, exit code, refusal, hint, JSON field, env var, config key or event kind was added or retired.
@task_file n=1 path=src/run/wheel.rs
what: the wheel: who holds it, the human move and the stdin observer's typed-key mark, the counted scheduled wakes
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's hook-event appender that files prompt-submitted with its origin and decides the wheel move on a human-filed prompt
@task_file n=3 path=src/run/send/relabel.rs
what: the tests of the prompt-origin relabel (a keyless prompt after a counted wake told by the rule)
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows viola verify checks, their ids, words and checks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit cases
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger's per-row check cases
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamp cases (verified and unverified row ids, row counts)
@task_file n=8 path=src/cmd/verify.rs
what: the viola verify verb: its step lines, summary and stamp (only its tests moved here, to 26 rows)
@task_file n=9 path=src/run/version_gate.rs
what: the version gate over the stamped CLI version at viola run (only its tests moved, to 26 rows)
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent standing in for claude: its argv options, hook events and synthetic Stop crons and fired wake
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness's run cases and the live verify output fixture at 26 step lines
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its labels, arms, stamp shapes and summary words
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's wake leg: the fired wake and its wheel-after-fire reading
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's binary: its rehearse words and help text
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals against the fake agent
@task_file n=16 path=tests/cli_wheel.rs
what: the CLI wheel integration cases: a fired wake keeps the driver's wheel, its next send and its dialog
@task_file n=17 path=tests/tui_wheel.rs
what: the outer-PTY wheel cases: a human key before a fired wake, a message queued during a turn
@task_file n=18 path=tests/cli_verify.rs
what: the viola verify integration cases: step lines, summary, exit codes at 26 rows
@task_file n=19 path=tests/cli_verify/runs.rs
what: the viola verify run cases: failing steps and summaries
@task_file n=20 path=tests/cli_verify/stamps.rs
what: the viola verify stamp cases: stamped summaries for a set without a Stop
@task_file n=21 path=tests/support/verify.rs
what: the shared verify test support: the spine set of recorded hook events, now with a synthetic Stop carrying session_crons
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the contract test stamping every recorded fixture set through the ledger rows
@task_item n=1 part=touches
contract/viola-verify-output/step-line: the step line's counter moved from [NN/25] to [NN/26], with a new stop-session-crons step line.
@task_item n=2 part=touches
contract/viola-verify-output/summary and contract/viola-verify/summary: a full pass now reads 26 pass 0 fail, not 25.
@task_item n=3 part=touches
contract/viola-verify-output/exit-0, exit-1 and failing-row: a Stop without a session_crons array now fails the new row (a missing key reads fail), moving any failing-set count by one.
@task_item n=4 part=touches
contract/viola-verify-output/record and contract/viola-verify-output/help-paragraph: the stamp gains the stop-session-crons row key; any row stating 25 relied rows is stale.
@task_item n=5 part=touches
contract/cli-verbs/run: under viola run a scheduled wake's fired prompt no longer takes the wheel for the human on a verified CLI; a typed key since the last human-filed prompt or an unverified CLI still takes it.
@task_item n=6 part=touches
contract/cli-verbs/send and contract/viola-send-output: after a fired wake the driver's next viola send is no longer refused for a human-held wheel.
@task_item n=7 part=touches
contract/viola-answer-output/answered and contract/mcp-tools/answer: a dialog raised after a fired wake stays answerable by the driver.
@task_item n=8 part=touches
contract/wheel-verb-output/pause and contract/wheel-verb-output/release: a release no longer clears whether a key was typed; the typed mark lasts until the next human-filed prompt.
@task_item n=9 part=touches
contract/viola-list-board/wheel-unknown and the board's WHEEL column: the wheel cell no longer flips to human when a scheduled wake fires on a verified CLI.
@task_item n=10 part=touches
decision/turn-ended-not-send-outcome and contract/viola-wait-output/turn-ended: the turn-ended event's scheduled wakes count is now read by the wheel rule; viola wait still has no wake condition.
@task_item n=11 part=touches
decision/spoken-phraseology-verb-output: the wheel's human move on a told wake is replaced by a harness origin with no wheel line.
@task_fact n=1 slug=surface-cli
fact: Under viola run, a prompt with no key typed since the last human-filed prompt, after a turn end that counted a scheduled wake, on a verified CLI, is filed origin harness and leaves the wheel with the driver; otherwise it takes the wheel for the human; a prompt submitted by the mouse alone is the one named false positive, unmeasured.
@task_fact n=2 slug=surface-cli
fact: viola verify checks 26 relied rows; the 26th, stop-session-crons, reads Stop carries session_crons as an array and checks only that the array is present, reading no element.
@task_item n=1 part=untouched
The web-spa surface and its absence: no page or web surface is reached.
@task_item n=2 part=untouched
The CLI's styling and token rendering rows (colours, SGR, spacing, captions, banner, spinner, footer): no output styling changed.
@task_item n=3 part=untouched
viola list board layout rows (column widths, truncation, ordering, escaping, empty board, stale rows): no column or layout moved.
@task_item n=4 part=untouched
viola revive, viola link and unlink, and viola plugin install contracts: none of their outputs, refusals or JSON codes changed.
@task_item n=5 part=untouched
CLI exit codes, global flags, the refusal line form, hints and the instance-unreachable details: no code, flag, hint or detail was added or moved.
@task_item n=6 part=untouched
viola verify refusals (CLI not found, fixture not clean, home not private, stopped, version unreadable): unchanged.
@task_item n=7 part=untouched
The MCP tool list and its result shapes, and the deep-links absence: no tool or result shape was added.
