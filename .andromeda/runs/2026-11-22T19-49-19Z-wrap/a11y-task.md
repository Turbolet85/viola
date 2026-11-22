@task_prose part=session
The session mended the Epoch 4 audit's ranked defects and re-measured the chrono advisory; no capability was claimed and no row flips.
A send whose claimed session-start line fails its snapshot write now settles back to waiting and ends unconfirmed through the existing confirm deadline, answering not-delivered with no-prompt-submitted; no new closed word and no new time bound.
The claude agents read now has its own named kill bound at the same 5 s the version probe uses, not measured against the real CLI; the stalled-host false red of verify's agents read is not mended and stays a carry.
The test harness's abort wait after a failed readiness moved from 20 s to 15 s, a test-only bound; the readiness bound stays 20 s.
Two drift replay tests joined the verify-window kill class in both the ci and mutants nextest profiles, no kill value moved; eight killable mutation survivors got killing cases and two equivalents were named.
@task_file n=1 path=src/run/send/slot.rs
what: the confirmed send's confirm window: the slot a session-start hook line claims, its settle, and the no-prompt-submitted refusal when the send ends unconfirmed
@task_file n=2 path=src/run/agents.rs
what: the claude agents read, which fails open, and its kill bound — a time bound that falls on the driver, now its own named constant at 5 s
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: crate tests of the claude ledger's checks: the env-names fault reading the user word as a name part — invariants held in crate tests under the mutation audit
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: crate tests of the claude ledger's stamped documents: a v1 document of a failing shape refused
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen shapes over the claude screen signatures (dialog option rows); tests only — a dot-led option row reads unnumbered
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the outer-PTY pump that carries the child's output to the terminal and the human's and the driver's input to the child; tests only — output drained after an exit, enqueue waits for room until its deadline, the writer's end bounded
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the agent harness lifecycle tests (boot, status, cleanup), the stuck-boot supervisor case now hard-linking its binaries
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness's boot: the readiness bound and the abort wait after a failed readiness, a test-only bound now 15 s
@task_file n=9 path=.config/nextest.toml
what: the nextest profiles (ci, mutants) and their per-test kill overrides — the verify-window class now also holding the two drift replay tests
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rule file; its frontmatter paths list only
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the contract test that every bounded reader passes its own kill bound — its needle for the agents read now names the agents read's own constant
@task_file n=12 path=tests/contract_lints.rs
what: the contract lint test pinning the nextest verify-window override by its whole filter line
@task_item n=1 part=touches
decision/time-bounds-fall-on-the-driver names the agents read among the driver's time bounds: that bound is now its own named constant, still 5 s, unmeasured against the real CLI; the value is unmoved, its owner named.
@task_item n=2 part=touches
decision/claude-agents-read-fails-open: the read still fails open; its kill bound is now separate from the version probe's while equal to it, and the stalled-host false red of verify's agents read stays unmended, a carry.
@task_item n=3 part=touches
contract/send-refusal/no-prompt-submitted: a send whose claimed session-start line fails its snapshot write now ends here, through the confirm window's existing deadline, instead of returning an error with the slot held.
@task_item n=4 part=touches
decision/critical-path-send-readback: the confirmed send's path gains a settled end for a claimed line whose snapshot write fails; no new closed word, no new time bound.
@task_item n=5 part=touches
decision/time-bounds-never-on-human and absence/user-time-limit, indirect: the confirm deadline, the agents read bound and the harness abort wait all fall on the driver or the test harness, none on the human; read whether any row lists the harness abort wait.
@task_item n=6 part=touches
decision/mutation-audit-covers-crate-invariants: eight killable survivors now have killing crate cases (ledger env-names fault and stamp shape, screen dot-led option row, pump enqueue wait, writer end, exit drain) and two equivalents are named and argued.
@task_item n=7 part=touches
decision/compiled-shapes-over-signatures and decision/no-screen-content-parsing, indirect: a test pins that a dot-led option row of the compiled dialog shapes reads unnumbered; no shape changed.
@task_item n=8 part=touches
decision/human-keystrokes-never-blocked and contract/tui-passthrough/human-keystrokes, indirect: the pump's input enqueue waits for room only until its deadline and the writer's end is bounded, now pinned by tests; no pump behaviour changed.
@task_item n=9 part=touches
decision/zero-flakiness and decision/a11y-failures-are-nextest-failures, indirect: the nextest ci and mutants profiles' verify-window kill class now holds two drift replay tests by name; no kill value moved and no retry was added.
@task_item n=10 part=touches
decision/a11y-cases-inside-coverage-suite and decision/a11y-runs-in-existing-test-suites, indirect: every new case sits in the existing crate and binary suites, none in an a11y suite of its own.
@task_item n=11 part=touches
decision/live-readings-never-gate, indirect: the host IO-stall reads stopped or released the gate layers but are not a gate; the agents read's stalled-host red stays a carry, not a gated reading.
@task_item n=12 part=touches
contract/structured-violation-json/suites-failures and decision/gate-requires-coverage-doctest, indirect: the coverage suite read 4023 passed and 0 failed with the carried rehearsal red not recurring; no gate shape or threshold moved.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR span): no output styling changed.
@task_item n=2 part=untouched
CLI help, exit codes, refusal layouts, unable and hint lines, one hint per code, code-word format: no printed string, code word or exit code was added or changed.
@task_item n=3 part=untouched
viola list board, captions, the unwrapped separator and NAME truncation: the board's output is not reached.
@task_item n=4 part=untouched
viola link and unlink, plugin install, revive refusals and revive --list documents: none of these verbs' outputs moved.
@task_item n=5 part=untouched
Frame-verb, hook-call and MCP frame-tool refusals for socket directory, strict modes and server verify: no refusal path of these changed.
@task_item n=6 part=untouched
Screen-reader targets and setups (Orca with GNOME Terminal, Orca with foot), the manual keyboard and Orca passes: no manual pass ran or changed.
@task_item n=7 part=untouched
The absences of internationalization, a touch surface, a web or graphical frontend and a rendered page: no capability was added or removed.
@task_item n=8 part=untouched
Dialog hold bound and config dialog_hold_s, the dialog-expired event line and viola wait's timed_out: no dialog or wait bound moved.
@task_item n=9 part=untouched
Control-character escaping and run diagnostics off the terminal: no byte the terminal receives from viola changed.
