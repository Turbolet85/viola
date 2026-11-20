@task_prose part=session
Under viola run, a prompt that a CLI-scheduled wake fires is now told from a typed one: when no key was typed on the wrapper's stdin since the last human-filed prompt, at least one scheduled wake was counted at the last turn end, and the CLI is verified, the wrapper relabels that keyless prompt origin harness and makes no wheel move, so the driver keeps its wheel, its next send and its dialogs.
The typed mark is set by the stdin observer and clears only at the next human-filed prompt, never at a turn end or a release; a message queued during a turn still reads human, and an unverified CLI or a prompt with no wake counted still takes the wheel for the human as before.
viola verify gains a 26th relied capability row, stop-session-crons (Stop carries session_crons as an array), so its step lines count to 26 and a clean stamp reads 26 pass 0 fail; the floor set grows 21 to 22.
The test-only fake agent gains argv options for synthetic session crons and one fired wake; the live rig gains the HB arm (an unwrapped fresh box read for hint-cell attributes before any send) and the WS and R3W labels.
Live readings on claude 2.1.328 under the rig's scratch homes: one interval wake fired, origin harness, wheel kept, the next send answered; nothing is claimed for another version, and no CLI flag, verb, output word, exit code or colour was added to the product.
@task_file n=1 path=src/run/wheel.rs
what: the wheel slot that holds who has the wheel (driver or human), now with the stdin typed mark and the counted scheduled wakes
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's hook-event intake that records prompt-submitted and turn-ended events and moves the wheel on a human-filed prompt, now with the told-wake rule
@task_file n=3 path=src/run/send/relabel.rs
what: the tests of the prompt-submitted origin relabel (human versus harness) for a send
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger, the list of relied rows viola verify stamps, now 26 with stop-session-crons
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's row-list tests
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger's per-row check tests, now covering stop-session-crons pass and fail
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamp and verified-state tests over the 26 rows
@task_file n=8 path=src/cmd/verify.rs
what: the viola verify command, its step lines and summary over the relied rows (tests only moved here)
@task_file n=9 path=src/run/version_gate.rs
what: the version gate that decides a verified or unverified CLI from the stamp (tests only moved here)
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, now with synthetic session crons on Stop and one fired keyless wake
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the end-to-end harness's expected viola verify transcript, step lines numbered to 26
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig, its arms and labels, now the HB fresh-box hint-cell arm and the WS and R3W stamp labels
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's scheduled-wake leg, now reading the wheel after a fired wake and a second send
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig command, its rehearse words and help text
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals against the fake agent, now ws, hb and r3w
@task_file n=16 path=tests/cli_wheel.rs
what: the wheel integration tests, now the fired-wake cases that keep or take the wheel
@task_file n=17 path=tests/tui_wheel.rs
what: the wheel tests through an outer terminal, now a key before a wake and a message queued during a turn
@task_file n=18 path=tests/cli_verify.rs
what: the viola verify integration tests, step lines and summary at 26 rows
@task_file n=19 path=tests/cli_verify/runs.rs
what: the viola verify run tests, failing-step summaries at 26 rows
@task_file n=20 path=tests/cli_verify/stamps.rs
what: the viola verify stamps.json tests, the summary of a set without a Stop
@task_file n=21 path=tests/support/verify.rs
what: the shared verify test support, the spine event set now carrying a synthetic Stop with session_crons
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger contract probes that stamp every recorded fixture set, now 26 pass 0 fail
@task_item n=1 part=touches
The viola verify step line contract: the step counter now reads [NN/26] where it read [NN/25], with a new stop-session-crons step; any row holding the 25 total or the step list is stale.
@task_item n=2 part=touches
The viola verify summary and summary line: a clean stamp now reads 26 pass 0 fail; any row holding 25 pass as the clean figure is stale.
@task_item n=3 part=touches
The viola verify fail word and the stopped-or-refused run line: the new row reads fail when Stop carries no session_crons array, so a set without a Stop now counts one more fail.
@task_item n=4 part=touches
Wheel shown as words and the flight-progress strip WHEEL field: after a told scheduled wake the WHEEL word stays on the driver; the wake makes no wheel move, so no human word appears for it.
@task_item n=5 part=touches
Pause and release as control transfer: a release does not clear the typed mark; only the next human-filed prompt does, so a draft typed and erased makes the next wake take the wheel once.
@task_item n=6 part=touches
The handoff transfer marker and the CLI transfer arrow: a told wake makes no wheel transfer, so no transfer is shown for it; a typed key, an unverified CLI or a prompt with no counted wake still transfers to the human as before.
@task_item n=7 part=touches
Dialog-pending cocked strip and the strip DIALOG field: a dialog raised in the turn a told wake fires stays the driver's to answer, since the wheel did not move.
@task_item n=8 part=touches
CLI verification state verified versus unverified-cli: the told-wake rule holds only on a verified CLI; on an unverified CLI a fired wake still takes the wheel for the human.
@task_item n=9 part=touches
viola wait event form: a told wake's prompt-submitted event now carries origin harness where it read human.
@task_item n=10 part=touches
viola send readback mirror and the send readback prompt send: the driver's next send after a told wake is delivered and read back instead of refused for human typing.
@task_item n=11 part=touches
Refusal hints human-typing: no longer shown for a send after a told scheduled wake; still shown after a typed key, an unverified CLI wake or a wake-less keyless prompt.
@task_item n=12 part=touches
viola run silent while wrapped: the stdin observer now also marks typed input; still nothing printed while wrapped.
@task_fact n=1 slug=surface-cli
fact: A keyless prompt fired by a CLI-scheduled wake on a verified CLI, with no key typed since the last human-filed prompt and at least one wake counted at the last turn end, reads origin harness and leaves the wheel with the driver; the typed mark clears only at the next human-filed prompt, and a prompt submitted by the mouse alone is the one named false positive, unmeasured.
@task_fact n=2 slug=surface
fact: viola verify stamps 26 relied rows, the 26th stop-session-crons (Stop carries session_crons as an array, no element read), so a clean step counter ends at [26/26] and a clean summary reads 26 pass 0 fail.
@task_item n=1 part=untouched
Colour palette, colour tokens and colour depth: no colour, attribute or SGR use was added; the HB rig reading of dim hint cells is a reading of claude's own box, not of viola's output.
@task_item n=2 part=untouched
Typography, spacing, iconography, glyphs and separators: no new glyph, column, separator or width.
@task_item n=3 part=untouched
viola list board and table rows, columns and skip counts, beyond the WHEEL word staying driver after a told wake.
@task_item n=4 part=untouched
Exit codes, exit 21 causes, revive and run start refusals, link and unlink output, strict-modes refusals and wrapper fault details: no code, cause or refusal added or moved.
@task_item n=5 part=untouched
viola --help verb groups and the command names: no verb, flag or hidden verb added; the fake agent options and rig words are test-only and no product name.
@task_item n=6 part=untouched
The web surface and rendered-page absences, the self-validation protocol and the brand style direction: untouched.
