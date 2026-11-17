@task_prose part=session
The session added one standing live rig: a test-only binary, viola-live-rig, that starts the vendor claude CLI (or the fake agent) in a PTY it owns, reads its screen through a vt100 model, and records closed words only.
The rig reads the trust dialog, the input box and modal dialogs, and types keys only under an argv grant: at most 2 down-arrows and Enter on a read trust row, one x, one DEL. Ctrl-C belongs to the stop alone.
Its rehearsals run in CI against the fake agent as the integration test binary rig_rehearsal. Its live starts are by hand under a cap of 2 plus 1 spare, and the live verb refuses under CI.
The fake agent now draws claude 2.1.325's unnumbered trust form over the recorded modal, moves its selector on arrow keys, and erases the prompt's last character on DEL.
Live readings of claude 2.1.325: the trust dialog is unnumbered with the selector first on No, exit; DEL erases the typed character; at submit a leading run of spaces is kept and trailing spaces and TAB are removed.
No product surface changed: the viola binary, its CLI output and its terminal passthrough are as they were. Nothing was retired.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: created — the standing live rig's command line, verbs rehearse, live and census, a test-only harness binary outside the release build
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: created — the live rig's start sequence: refusals before the start, child spawn in a PTY, the R1 and R2 arms, the closed summary words
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: created — the rig's screen model over vt100 that reads the trust dialog, its options and selector, the input box text word and the imports modal, poisoned on a feed panic
@task_file n=4 path=crates/viola-e2e/src/rig/keys.rs
what: created — the rig's key grant and typing site, the only place keys are typed into the vendor CLI's PTY, refusing any key outside the grant or not ready
@task_file n=5 path=crates/viola-e2e/src/rig/isolate.rs
what: created — the rig's isolation: a private scratch config dir, the token-window check, the child's environment
@task_file n=6 path=crates/viola-e2e/src/rig/procs.rs
what: created — the rig's process hygiene: census and sweep of its own leftovers, the watchdog, the stop handlers
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: created — the rig's start ledger and start cap, and the kept screen frames
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: created — the rig's rehearsal integration tests against the fake agent, run in CI
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: edited — the end-to-end test crate's manifest: the new rig binary and the vt100 dependency
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: edited — the end-to-end test crate's root, now exporting the rig module
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent, the test stand-in for claude: its screens, now drawing the trust form with a moving selector, the box rule and prompt row
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: edited — the fake agent's input handling: the typed prompt, with DEL erasing its last character
@task_file n=13 path=Cargo.lock
what: edited — the dependency lockfile, one line for vt100 in the test crate
@task_item n=1 part=touches
decision/live-readings-never-gate: live readings stay by hand and gate nothing, but the standing rig is now committed and its rehearsals against the fake agent run in CI as a test binary that gates. The live verb refuses under CI.
@task_item n=2 part=touches
decision/no-screen-content-parsing: the test-only rig reads the vendor CLI's screen beyond signatures. It reads the trust dialog's option rows and selector place, the input box as empty, holds-text or absent, and the imports modal, all as closed words.
@task_item n=3 part=touches
decision/boundary-check-never-inspects-claude-content: the rig inspects the CLI's drawn screen and records only closed words. The kept frames hold row text and stay in a gitignored local dir.
@task_item n=4 part=touches
decision/tui-boundary-driver-pin and decision/portable-pty-outer-pty-driver: the rig drives the vendor CLI in its own PTY and reads it through vt100, the same boundary driver crate now also in the end-to-end test crate.
@task_item n=5 part=touches
contract/tui-passthrough/vt100-feed-panic and contract/tui-passthrough/after-feed-panic: the rig's screen model contains a vt100 feed panic by catch_unwind and reads poisoned. This is test-only, beside the product's contained-panic rule.
@task_item n=6 part=touches
decision/human-key-takes-the-wheel, decision/stdin-classifier-non-editing-list and decision/non-editing-sequences: the rig types arrow keys, Enter, x, DEL and Ctrl-C straight into the CLI's PTY, not through viola's wheel. The fake agent now treats DEL as an edit of its prompt.
@task_item n=7 part=touches
decision/boundary-tests-sequenced-on-records: the rehearsals wait on screen frames and on the events.ndjson session-start line under bounded settle and deadline values.
@task_item n=8 part=touches
decision/time-bounds-fall-on-the-driver and decision/zero-flakiness: the rehearsal bound is WITHIN 18 s with the rig's --settle 6 --deadline 15, held below the contract lint's 20 s floor and the 30 s end-to-end kill line.
@task_item n=9 part=touches
decision/a11y-runs-in-existing-test-suites and decision/a11y-cases-inside-coverage-suite: a new integration test binary, rig_rehearsal, joins the existing nextest integration suite and the pre-push run, 3484 tests in all.
@task_item n=10 part=touches
decision/ci-test-gate-no-skips: the rig's live verb refuses with the word ci under CI. The rehearsal case that checks this is a test that passes, not a skip.
@task_item n=11 part=touches
decision/paths-not-carried: no rig record holds a credential, home path or user config value. A rehearsal case asserts this, and the kept frames hold only the scratch cwd under /tmp.
@task_item n=12 part=touches
decision/critical-path-send-readback and decision/critical-path-dialog-answer: R2's live reading measured the submit trim, leading run kept and trailing spaces and TAB removed. The trust dialog is the vendor's own modal, read live as unnumbered with the selector first on No, exit.
@task_item n=13 part=touches
decision/wrapped-tui-reads-as-unwrapped and decision/tui-screen-reader-test-setup: the live rig's R2 runs claude under viola run in the rig's PTY. No screen-reader reading was made.
@task_fact n=1 slug=keyboard-test-harness
fact: A standing live rig, a test-only binary outside the release build, is the one site that types keys into the vendor CLI's own PTY. It types only under its argv grant: at most 2 down-arrows and Enter on a read trust row, one x, one DEL, and Ctrl-C for its stop alone. Its rehearsals against the fake agent run in CI. Its live starts are by hand, capped, and refused under CI.
@task_fact n=2 slug=keyboard-test-harness
fact: The live rig reads the vendor CLI's screen beyond signatures as a test-only exception: the trust dialog's selector place, the input box as empty, holds-text or absent, and modal dialogs. It records closed words only and keeps raw frames in a gitignored local dir.
@task_item n=1 part=untouched
The viola CLI's printed output: refusal and hint lines, exit codes, --json failure documents, fault bodies and code words. No product output changed.
@task_item n=2 part=untouched
The viola list board, its caption, colour, SGR, dim and NAME truncation, and the --help text.
@task_item n=3 part=untouched
viola revive, link, unlink, plugin install, verify and wait output contracts, and the MCP frame tool refusals.
@task_item n=4 part=untouched
The product's TUI passthrough contracts on the human's terminal: zero viola bytes, focus, mouse and resize sequences, and run diagnostics off the terminal.
@task_item n=5 part=untouched
Colour, contrast and visual design rows, and the cognitive-accessibility rows on fixed copy and reading level.
@task_item n=6 part=untouched
The screen-reader targets and the manual Orca and keyboard passes. No manual pass was made.
@task_item n=7 part=untouched
The absences of a web or graphical frontend, a touch surface, a rendered page, internationalization and a user time limit.
@task_item n=8 part=untouched
The a11y tier, the compliance regime and the no-WCAG-conformance claim, and the CI workflow and structured violation JSON rows.
