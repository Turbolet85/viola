@task_prose part=session
The session mended the Epoch 4 audit's ranked defects and re-read the chrono advisory, with no capability claimed and no verb, status, refusal word or output shape changed.
A send whose claimed session-start line then fails its snapshot write now settles back to waiting, and the existing confirm deadline closes it as not-delivered with no-prompt-submitted; before, the error returned with the slot still claimed.
The claude agents --json read now has its own named kill bound, still 5 s, the version probe's value kept and not measured against the real CLI; the stalled-host false red of verify's agents read is not mended and stays a carry.
The test harness's abort wait after a failed readiness moved from 20 s to 15 s; the readiness bound stays 20 s.
Two drift replay tests joined the verify-window test class in the ci and mutants test profiles, no kill value moved.
Unit cases were added to kill eight surviving mutants (env-name fault reading, a v1 stamps document of a failing shape, a dot-led option row on screen, the PTY pump's exit drain, enqueue wait and writer-end wait).
@task_file n=1 path=src/run/send/slot.rs
what: the send's confirm slot: claims a hook line for the send in flight and settles the send's readback outcome (delivered, not-delivered, unconfirmable) in the confirm window
@task_file n=2 path=src/run/agents.rs
what: the claude agents --json read behind the list board's agents and verify's agents step, and its kill bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: tests of the verify ledger's checks, here the env-names fault reading
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: tests of the verify stamps record, here refusing a v1 stamps document of a failing shape
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the agent screen reader that reads option rows of a dialog off the terminal screen; tests only changed
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the PTY pump that drains the child's output and writes input to it; tests only changed
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the test harness's lifecycle tests, including the stuck-boot supervisor case
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness's boot: readiness deadline and the abort wait after a failed readiness
@task_file n=9 path=.config/nextest.toml
what: the test runner's ci and mutants profiles and their per-class kill overrides, the verify-window class among them
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rules (its frontmatter path scope only)
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the contract check that every external read runs under a kill bound
@task_file n=12 path=tests/contract_lints.rs
what: the contract lints, including the pin on the verify-window test filter
@task_item n=1 part=touches
The send readback for /clear and /compact: their confirmation rides a claimed session-start line; when the snapshot write after that claim fails, the send now ends not-delivered with no-prompt-submitted rather than erroring with the slot held.
@task_item n=2 part=touches
The refusal hint for not-delivered with no-prompt-submitted: a new path now reaches it (a claimed session-start whose snapshot write failed); the word and hint are unchanged.
@task_item n=3 part=touches
Nothing announces success before confirmation, and no optimistic send confirmation: the failed-snapshot path now closes as not-delivered, never a read back; a rule over the confirm slot.
@task_item n=4 part=touches
The readback box and the send readback mirror's unable and unconfirmable lines: the outcome a failed-snapshot claimed send prints moved to the not-delivered unable form.
@task_item n=5 part=touches
The list board's failed agents read: the read is killed at its own named bound, still 5 s; any row stating the agents read's bound or its fail-open on timeout holds, its value unmoved.
@task_item n=6 part=touches
viola verify output's agents step: verify's agents read uses the same newly named 5 s bound; the false red under a stalled host is not mended, so no row may claim it is.
@task_item n=7 part=touches
Any row listing the time bounds the driver sees (agents read, version read): the agents read's bound is now separate from the version probe's, both 5 s.
@task_item n=8 part=touches
The send readback for /context and the prompt send: their confirmation is not claimed by a session-start line, so they are unchanged; a picker should read them only to confirm that.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography: no glyph, colour token, column, density or style moved.
@task_item n=2 part=untouched
The web-spa surface and the rendered-page absence: no page or web surface was added.
@task_item n=3 part=untouched
CLI exit codes and the exit 21 causes, human and --json: no code or cause added, renamed or moved.
@task_item n=4 part=untouched
viola run and revive start refusals, revive --json refusal codes, revive --list output and the automatic-verify line: none changed.
@task_item n=5 part=untouched
viola link and unlink output, viola wait output, viola last output, the help verb groups and the command names: no verb, flag or name taken or freed.
@task_item n=6 part=untouched
Peer reply screening, the wrapper fault --json detail and the wrapper socket refused peer: unchanged.
@task_item n=7 part=untouched
CLI colour decision order, colour depth, streams, escaping and strict-modes refusals: unchanged.
@task_item n=8 part=untouched
The self-validation protocol and anti-patterns: no new exception; the session adds no output.
