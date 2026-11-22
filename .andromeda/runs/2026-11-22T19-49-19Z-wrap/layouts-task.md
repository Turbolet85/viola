@task_prose part=session
The session mended the ranked defects of the Epoch 4 code audit; no verb, flag, output line, word, hint or exit code was added, moved or retired.
A send whose claimed session-start line then fails its snapshot write now settles back to waiting, and the confirm window's existing deadline closes it as not-delivered with the cause no-prompt-submitted, instead of the error escaping mid-send.
The claude agents --json read that viola list, viola revive and viola verify use now has its own named kill bound, still 5 s, the version probe's value kept and not measured against the real CLI; the stalled-host false red of verify's agents read is not mended and stays a carry.
The test harness's abort wait after a failed readiness dropped from 20 s to 15 s, two drift replay tests joined the verify-window kill class in the ci and mutants profiles, and new unit cases kill the audit's mutation survivors; the testing rule's path list was widened.
The RUSTSEC advisory was re-read with no safe upgrade, and a SIGKILLed verify was measured to leave only its own one-key trust record; neither changed code.
@task_file n=1 path=src/run/send/slot.rs
what: the send's confirm slot — the hook-event append that claims a session-start line and settles the send's outcome (read back for /clear and /compact, unconfirmable, not-delivered)
@task_file n=2 path=src/run/agents.rs
what: the claude agents --json read behind the board's STATUS join, the revive list and verify's agents step — its kill bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger's environment-name fault check (test-only)
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the ledger's stamped-document shape check (test-only)
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the screen reader of the claude CLI's dialog options — a test case only, for a dot-led option row read unnumbered
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the PTY pump — test cases only, for the output drained after an exit, the input queue's wait for room and the writer's end wait
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the agent harness's boot, status and cleanup lifecycle tests, the stuck-boot supervisor case
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the agent harness's boot — its abort wait after a failed readiness (test-only bound)
@task_file n=9 path=.config/nextest.toml
what: the test runner's profiles — the verify-window kill class of the ci and mutants profiles
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rule — its path list
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the contract test that every external read is bounded — its pin for the agents read's bound
@task_file n=12 path=tests/contract_lints.rs
what: the contract lint tests — the pin on the verify-window kill class filter
@task_item n=1 part=touches
The viola send read back for /clear and for /compact: a claimed session-start whose snapshot write fails now ends the send not-delivered with no-prompt-submitted rather than an error; read whether those rows name that end.
@task_item n=2 part=touches
The viola send unable and unconfirmable output rows and the send --json typed document: the not-delivered / no-prompt-submitted end now also covers the failed-snapshot case; no new word or field.
@task_item n=3 part=touches
The readback mirror's issue and outcome lines and the readback vocabulary's state words, through the send outcome above; the set of words is unchanged.
@task_item n=4 part=touches
The viola list board's failed agents read row and its STATUS join: the agents read is now killed at its own named bound, still 5 s; any row holding that 5 s keeps its value, its owner named apart from the version probe.
@task_item n=5 part=touches
The viola verify failing row, stop and summary rows where they name the agents step: the agents read's false red on a stalled host is not mended and stays a carry.
@task_item n=6 part=touches
The viola revive --list document and row, which read the same agents read and so share its renamed bound.
@task_item n=7 part=touches
The hint for every refusal cause and the hint naming the next verb: no new cause was added, so any row listing the causes of not-delivered stays whole.
@task_item n=8 part=untouched
The web-spa surface's absence of a web page.
@task_item n=9 part=untouched
The CLI token rendering rows (colours, roles, spacing, motion, radius) and the styling, banner, spinner, footer and glyph decisions.
@task_item n=10 part=untouched
The CLI exit codes, the global flags, the --help and per-verb usage rows, and the flat-verb and argument-order decisions.
@task_item n=11 part=untouched
viola link / unlink output, viola plugin install, the wheel verbs (pause, release), viola answer, viola wait and viola last output rows.
@task_item n=12 part=untouched
The MCP server's tools and results rows and the hidden mcp verb.
@task_item n=13 part=untouched
The run and revive start refusals and their --json error codes, the frame verbs' exit-21 hints and the instance-unreachable detail.
