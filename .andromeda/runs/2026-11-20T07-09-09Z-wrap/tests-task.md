@task_prose part=session
The session tells a CLI-scheduled wake's keyless prompt apart from a typed one under viola run, so a fired wake keeps the driver's wheel, its next send and its dialog (matrix v2-36, now implemented).
A keyless human-filed prompt-submitted is relabelled origin harness with no wheel move only when no stdin key marked the wrapper typed, the last turn end counted at least one scheduled wake, and the CLI is verified; otherwise the human takes the wheel as before.
The stdin typed mark clears only at the next human-filed prompt, never at a turn end or a release: a message queued during a turn reads human; a mouse-only submit is the one named false positive, unmeasured.
The capability ledger gains its 26th relied row stop-session-crons (Stop carries session_crons as an array, checked by is_array only); relied rows 25 to 26, verify step lines [NN/26], stamped summaries 26 pass, the floor row set 21 to 22, every stamped fixture set restamped 26 pass 0 fail.
The fake agent gains argv-only --stop-crons and --wake-after-ms; the standing live rig gains labels WS, HB, R3W, the hb arm and ws, r3w, hb rehearsals, with the wheel-after-fire and stop-session-crons readings.
Live on claude 2.1.328 under scratch homes: WS 26/26, HB hint cells dim and told apart from typed plain cells in a fresh box before any send, R3W one fired interval-form wake read origin harness and wheel kept; the dynamic form was not fired and no other version is claimed.
@task_file n=1 path=src/run/wheel.rs
what: the wrapper's wheel slot: who holds the wheel, human input takes it, now with the stdin typed mark and the counted scheduled wakes
@task_file n=2 path=src/run/send/slot.rs
what: the hook event append into the event log: prompt-submitted origin, turn-ended scheduled_wakes, the human-input wheel move and the told-wake rule
@task_file n=3 path=src/run/send/relabel.rs
what: unit cases for the prompt-submitted origin relabel (harness versus human) and the told-wake rule table
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows, their ids, words and verify probes, the stamps
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: ledger unit cases over the relied row list
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: ledger unit cases for each relied row's verify probe check
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: ledger unit cases for stamps, verified and unverified rows and the version gate's relied-row reading
@task_file n=8 path=src/cmd/verify.rs
what: viola verify: its step lines, summary line and stamped line (unit cases moved to 26 rows)
@task_file n=9 path=src/run/version_gate.rs
what: the unverified CLI version gate that needs every relied row (unit cases moved to 26 rows)
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake agent serving the claude boundary, configured by argv alone; gains --stop-crons and --wake-after-ms
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command's local-live suite and its expected verify pass lines
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the standing committed live rig: its labels, arms, kept and seeded stamps and summary words, including the hint-cell reading
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's scheduled-wake leg: the fired wake, its origin word and the wheel-after-fire reading
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary: its rehearse words and help text
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals over the fake agent (ws, hb, r3w and the refusal before the ledger line)
@task_file n=16 path=tests/cli_wheel.rs
what: CLI integration cases for the wheel: harness-injected turns keep it, human input takes it; gains the Path 5 fired-wake cases
@task_file n=17 path=tests/tui_wheel.rs
what: passthrough cases through an outer PTY for the wheel: a human key before a fired wake, a message queued during a turn
@task_file n=18 path=tests/cli_verify.rs
what: viola verify integration cases: step lines, summary line, stamped line and failing rows
@task_file n=19 path=tests/cli_verify/runs.rs
what: viola verify integration cases over runs with failing rows and their expected step lines
@task_file n=20 path=tests/cli_verify/stamps.rs
what: viola verify integration cases over written stamps and their stamped summaries
@task_file n=21 path=tests/support/verify.rs
what: the shared stamping-verify test helper and its spine fixture set (gains a synthetic Stop with session_crons)
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite: every recorded fixture set stamped against the relied rows
@task_item n=1 part=touches
decision/injected-turn-keeps-wheel and decision/harness-turn-keeps-the-wheel: a fired CLI-scheduled wake now keeps the wheel beside the harness-injected turn, told by the stdin mark, the counted wake and the verified CLI.
@task_item n=2 part=touches
decision/human-input-takes-the-wheel and decision/human-takes-wheel-on-edit: gain an exception, a keyless prompt after a counted wake with no stdin key on a verified CLI does not take the wheel; a draft typed and erased makes the next wake take it once.
@task_item n=3 part=touches
decision/focus-mouse-resize-keep-wheel: a prompt submitted by the mouse alone, after a counted wake, reads as a wake and keeps the wheel, the one named false positive, unmeasured.
@task_item n=4 part=touches
contract/prompt-submitted/origin-harness, contract/events/harness-prompt: the wrapper now relabels a told wake origin harness, not only a hook-filed harness prompt.
@task_item n=5 part=touches
contract/events/human-input-wheel, contract/wheel-record/human-input, contract/event-log/wheel-human-input: no human-input wheel record is written for a told wake; a message queued during a turn still writes one.
@task_item n=6 part=touches
contract/turn-ended/scheduled-wakes, contract/dialog-data/scheduled-wakes, contract/event-data/turn-ended: the scheduled_wakes count is now read by the wheel rule (null reads none; session start and end reset it).
@task_item n=7 part=touches
decision/unverified-cli-send-only, decision/unverified-cli-gate, decision/unverified-cli-gates-answer-only: on an unverified CLI a fired wake takes the human wheel; the told-wake rule needs a verified CLI.
@task_item n=8 part=touches
decision/version-gate-needs-all-relied-rows, decision/ledger-row-verify-probe, decision/ledger-probe-contract-suite: the relied set is 26 rows, the 26th stop-session-crons with its is_array probe; every fixture set stamps 26 pass 0 fail.
@task_item n=9 part=touches
contract/verify-output/step-line, contract/verify-output/summary-line, contract/verify-output/last-line, contract/viola-verify/stamped-line, contract/cli-human-lines/stamped: step lines read [NN/26], summaries 26 pass 0 fail; any cell holding 25 rows is stale.
@task_item n=10 part=touches
a row naming the floor row set or its size (21 rows): it is now 22, stop-session-crons beside stop-message.
@task_item n=11 part=touches
decision/stamps-from-fake-agent-verify, decision/fake-agent-matches-verify-fixtures, decision/replay-every-cli-version-fixture, decision/fake-agent-replay-for-stamps: the stamping spine set gains a synthetic Stop with session_crons; every stamped summary's pass count moved by one.
@task_item n=12 part=touches
contract/fake-agent-cli/argv-only-options, decision/fake-agent-argv-only, decision/fake-agent-as-claude: the fake gains --stop-crons (0 to 4 synthetic session_crons elements) and --wake-after-ms (capped 5 000, one keyless recorded UserPromptSubmit), argv only, no env read, no fixture written.
@task_item n=13 part=touches
decision/standing-live-rig: the rig gains labels WS, HB, HB-2, R3W, R3W-2, the hb arm and the ws, r3w, hb rehearsals; summary words wheel-after-fire and stop-session-crons.
@task_item n=14 part=touches
decision/live-rig-hint-cell-reading: the hint-cell reading also stands for a fresh box before any send (HB), recorded as a reading.
@task_item n=15 part=touches
decision/no-screen-scraping-in-e2e, decision/never-parse-rendered-child-screen, decision/no-screen-parsing, decision/screen-signatures-only: the rig's HB arm reads cell attributes of a fresh box before any send, an exception carried as a live reading only.
@task_item n=16 part=touches
decision/live-failing-reading-not-verification, decision/fake-agent-not-proof-of-real-cli: v2-36 rests on the fake-agent rehearsal and the live R3W reading on one version (2.1.328), one interval-form wake; nothing claimed for another version or the dynamic form.
@task_item n=17 part=touches
contract/viola-send/human-key-during-gate, contract/viola-send/human-typing: the stdin observer's key now also sets the typed mark under the same wheel lock; the send refusals are unchanged.
@task_item n=18 part=touches
decision/tui-outer-pty-driver, decision/tui-verdicts-from-records: new outer-PTY cases read the wheel from records for a human key before a fired wake and a queued message.
@task_fact n=1 slug=6-e2e-test-strategy
fact: A fired CLI-scheduled wake keeps the driver's wheel: a keyless human-filed prompt-submitted is relabelled origin harness with no wheel move when no stdin key set the typed mark, the last turn end counted at least one scheduled wake, and the CLI is verified; otherwise it takes the human wheel.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The stdin typed mark lasts until the next human-filed prompt takes it, surviving a turn end and a release; a message queued during a turn reads human; a mouse-only submit after a counted wake is the named false positive, unmeasured.
@task_fact n=3 slug=7-test-data-fixtures
fact: The fake agent takes --stop-crons n (argv-only, 0 to 4) putting n synthetic elements in every Stop's session_crons, and --wake-after-ms ms (argv-only, capped 5 000) firing once a keyless recorded UserPromptSubmit that long after the first Stop with non-empty session_crons.
@task_fact n=4 slug=6-e2e-test-strategy
fact: The live rig's R3W leg reads viola's own wheel lines and origin word after a fired wake as wheel-after-fire kept, human, other or unread, then sends a plain text; WS keeps a 26-row wake stamp; HB types x and del into a fresh unwrapped box before any send.
@task_fact n=5 slug=6-e2e-test-strategy
fact: A rig rehearsal reads no key typed from the rig's own keys word, never from the fake-agent receipt, since the shared receipt carries viola send's submit Enters as key lines.
@task_fact n=6 slug=6-e2e-test-strategy
fact: The fired-wake capability is claimed for one interval-form wake on claude 2.1.328 under a scratch home; the dynamic form is unfired and no other version is claimed.
@task_item n=1 part=untouched
viola wait and the channel wait method: wake kinds and turn-ended wake are unchanged; no wait wake condition was added for scheduled wakes.
@task_item n=2 part=untouched
MCP server, MCP tools, MCP tool results and invalid-params fields: no tool, schema or reply changed.
@task_item n=3 part=untouched
viola revive, run and revive start refusals, exit 1 causes and strict-modes, socket-directory and server-identity checks.
@task_item n=4 part=untouched
link and unlink methods, the link verb line and implicit link on send.
@task_item n=5 part=untouched
dialog hook decision bodies, dialog hold and expiry, and the dialog event contracts: the dialog answered after a wake uses them unchanged.
@task_item n=6 part=untouched
harness test contract rows: boot, cleanup, status document, session record, supervisor, pre-push, gate breaches and the run document; no agent-run command, status shape or CI step changed.
@task_item n=7 part=untouched
CI integration, coverage floors, mutation and performance gates, fuzzing and supply-chain gates.
@task_item n=8 part=untouched
process log format, log event corr and log line fields: no log line or event kind added.
@task_item n=9 part=untouched
CLI exit codes, CLI --json documents, list board and list --json rows, viola last and viola release.
@task_item n=10 part=untouched
absences (browser suite, Windows and macOS, database, outbound HTTP, deploy stage).
