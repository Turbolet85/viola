@task_prose part=session
The session added a test-only StopFailure arm (SF, spare SF-2) to the standing live rig and recorded one contained live StopFailure payload on claude 2.1.328, the rig run inside a network-closed bwrap sandbox, refused before any ledger line when the sandbox is absent.
After the turn end the SF arm types one x and one DEL into the rig's own box and reads that box row's cell attributes only (closed words dim, italic, bold, inverse, fg-n, fg-rgb, plain, unread), never a cell's text; the live reading was hint-attr inverse, typed-attr plain, hint-told-apart yes, with the one-cell limit (it may be the cursor's cell) left open as carry C3.
The StopFailure exception is settled as outcome B on the founder's word on SF's green reading (predicates 1-7), no longer provisional; outcome A (a ledger row with a verify probe) is not opened; one live StopFailure of one class on one version, no capability claimed.
The rig's StopFailure capture keeps key names, the error word of the CLI's closed 13 (else other) and two text byte lengths, never a text; the kept record dir is now keyed by CLI version first.
The fake agent gained --turn-fail and --settings-hooks; four sf rehearsals were added to the rig rehearsal suite, the contained one skipping by its printed probe where no sandbox or runtime dir crosses; no product crate, CLI output, terminal byte or CI gate changed.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's isolation of its child: the scratch home, the kept record per CLI version, the network-closed sandbox argv and its runtime-dir precondition, the seeded StopFailure capture hook
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the standing live rig's arms and plan: the new SF arm, the sandbox probe and its refusal, the turn-end and cause words, and the box row's cell-attribute read after the turn end
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: edited — the live rig's closed prompt capture, now also the StopFailure line: key names, the closed error word else other, two text lengths, never text
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: edited — the rig's vt100 screen model of the wrapped TUI, the boundary read by signatures, now also reading a cell's attributes and the box row's first drawn cell, never its text
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the live rig's command line: rehearse sf with --contain and the live SF and SF-2 labels
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the rig rehearsals over the fake agent run in the integration suite in CI: four sf rehearsals added, the contained one skipping by its printed probe
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent standing in for claude in boundary tests and rig rehearsals, gaining --turn-fail and --settings-hooks
@task_item n=1 part=touches
decision/no-screen-content-parsing: its except gains the box row's cell attributes (closed attribute words, never text), read by the rig after the turn end; the row is stale until it says so.
@task_item n=2 part=touches
decision/boundary-check-never-inspects-claude-content: the rig now reads attributes of cells claude drew in its box (never their text) and types x and DEL into that box after the turn end; the row must hold that this is attribute-only and rig-only.
@task_item n=3 part=touches
decision/live-readings-never-gate: the SF reading (2.1.328, 2026-11-19, hint-attr inverse, typed-attr plain, hint-told-apart yes) is recorded as a reading, never verification, and gates nothing.
@task_item n=4 part=touches
decision/ci-test-gate-no-skips: the contained sf rehearsal returns early by its printed sandbox probe (unavailable, no-runtime-dir) where bwrap or a crossing runtime dir is absent; whether CI ran or skipped it is unread; check the row against this probe-skip.
@task_item n=5 part=touches
decision/tui-boundary-driver-pin and decision/tui-boundary-assertions-only (and decision/tui-boundary-only): the rig's screen model, the boundary reader, gains cell-attribute reads beside its signatures; assertions stay at the boundary.
@task_item n=6 part=touches
decision/boundary-tests-sequenced-on-records: the sf rehearsals run the fake agent over the recorded 2.1.328 set with a planted StopFailure payload and a settings.json hook (--turn-fail, --settings-hooks).
@task_item n=7 part=touches
decision/a11y-inside-existing-suites and decision/a11y-cases-inside-coverage-suite: the new sf rehearsals, including the hint-attr, typed-attr and hint-told-apart words, sit in the existing rig rehearsal integration suite, no separate driver.
@task_item n=8 part=touches
decision/non-editing-sequences and decision/stdin-classifier-non-editing-list, indirectly: the rig's x and DEL are real editing keys typed into the rig's own scratch box after the turn end, not terminal sequences; check neither row is read as covering them.
@task_item n=9 part=touches
decision/zero-flakiness: the intermittent rehearsal red C4 (rig_rehearsal_reads_the_box_empty_after_x_and_del) was watched, 4 green implement runs, pre-push and CI green, no recurrence; it stays open with its next reader.
@task_item n=10 part=touches
decision/colour-second-cue-only and decision/no-information-by-colour-alone, indirectly: the placeholder hint is told apart from typed text only by a cell attribute (inverse vs plain) in claude's own box; the reading is of claude's drawing, not viola output, and its one-cell limit is open.
@task_item n=11 part=touches
decision/manual-orca-pass and decision/manual-keyboard-pass, indirectly: the placeholder-hint attribute reading (C3) is a carry whose next reading route-resolve routes; no manual pass was run or changed.
@task_item n=12 part=untouched
CLI refusal, failure and fault contracts (unable and hint lines, exit codes, --json failure documents, code words, instance-unreachable details): no CLI output changed; the instance-dead reply met in a pre-push red was a test-environment cause, the contract unchanged.
@task_item n=13 part=untouched
viola list board, caption, colour and SGR rows (dim, amber, bold, no SGR off a terminal, word beside every SGR span): no viola output or colour changed.
@task_item n=14 part=untouched
viola --help, revive, link, unlink, plugin install, verify and wait output contracts, and the log identity tags: none moved.
@task_item n=15 part=untouched
TUI passthrough contracts for viola's own bytes on the terminal, human keystrokes, focus, mouse and resize, run diagnostics and the vt100 feed panic: the product passthrough was not changed.
@task_item n=16 part=untouched
Screen-reader targets and setups, cognitive time bounds and dialog hold, the a11y tier, compliance and WCAG rows, the absences (web, touch, i18n, rendered page, user time limit) and the CI gate wiring and its thresholds: none reached.
