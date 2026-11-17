@task_prose part=session
The session changed only the test side (the viola-e2e test crate and the CI test profile); no product file, product crate, CLI output, refusal, exit code, IPC method, endpoint or env var moved.
It took the first live readings on the standing live rig: a fired scheduled wake's count and source, the hook-latency window, the external-imports modal of the vendor TUI, and the wrapped session's box text.
It added two wake arms and an imports arm to the rig, a hidden capture verb that records the rig's own UserPromptSubmit hook payload as key names and a closed source word with no value, and a no-key group-kill stop for the imports arm.
It replaced the rig's stamp-only user-config witness with a mark witness whose closed words are unchanged, absent, changed-by-others, changed-by-rig and unread; the old word changed is retired.
It gave the test harness's boot a per-instance readiness deadline (default 20 s) and put three more viola-e2e test binaries on the CI 20 s kill line (override filter 25 to 28 binaries).
It measured that a fired loop wake carries no source key on claude 2.1.325 and 2.1.326, disproving the plan's reading of the vendor bundle; the wrapper read every fired wake as origin human.
@task_file n=1 path=crates/viola-e2e/src/rig/capture.rs
what: new; the standing live rig's capture of its own UserPromptSubmit hook payload, kept as listed key names and a closed source word, no value
@task_file n=2 path=crates/viola-e2e/src/rig/wake.rs
what: new; the standing live rig's scheduled-wake arms, which send through the wrapper and read the wake count and the fired wake's source
@task_file n=3 path=crates/viola-e2e/src/rig/isolate.rs
what: edited; the standing live rig's isolated child scratch and its user-config isolation witness, now the mark witness, plus the capture and imports seeds
@task_file n=4 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the standing live rig's arms, labels, session-id learning and stops, including the no-key group-kill stop of the imports arm
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the standing live rig binary, its live labels, the wake and stand options and the hidden capture verb
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the rig's rehearsal suite against the fake agent, gaining the wake, capture, imports and mark-witness cases
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the test harness's boot of a viola instance, its per-instance readiness deadline
@task_file n=8 path=crates/viola-e2e/src/harness/run/perf.rs
what: edited; the harness's performance run, naming the shared instance deadline at its boot
@task_file n=9 path=crates/viola-e2e/tests/cli.rs
what: edited; the CLI test suite, naming the shared instance deadline at its boot
@task_file n=10 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness lifecycle suite, whose never-ready boot case now ends at the readiness deadline
@task_file n=11 path=.config/nextest.toml
what: edited; the CI test profile's 20 s kill-line override filter, now three more viola-e2e binaries
@task_item n=1 part=touches
Live readings never gate: the session fired five live starts on the standing rig as readings recorded in evidence, one of them red on its isolation witness; any row saying live readings gate nothing or how they are kept applies to these starts.
@task_item n=2 part=touches
Boundary check never inspects claude content: the rig's new capture reads the vendor's UserPromptSubmit payload, keeping key names matching a closed pattern and a closed source word, never a value; a row bounding what the boundary check may read of claude's content is reached.
@task_item n=3 part=touches
No screen-content parsing beyond signatures: the imports arm reads the vendor TUI's external-imports modal and its box state by signature (modal imports, box down) and closes with no key; a row limiting screen reading to signatures rules over this arm.
@task_item n=4 part=touches
The rig keys only its own PTY: wake starts typed no key into any dialog or box text and stopped with Ctrl-C into the rig's own PTY; the imports arm stopped by a group kill typing no byte; any a11y-plan row on the live rig's keys or on human keystrokes never being typed for the human is reached.
@task_item n=5 part=touches
Human key takes the wheel and wheel rules: every fired scheduled wake's wrapper prompt-submitted line read origin human on 2.1.325 and 2.1.326, because the fired wake carries no source key; a row on who holds the wheel or how a prompt's origin is told is reached (the route entry on a fired scheduled wake keeping the wheel is open).
@task_item n=6 part=touches
Time bounds fall on the driver only: the harness boot gained a per-instance readiness deadline (default 20 s) and the CI 20 s kill line now covers three more test binaries; a row on time bounds falling on the driver or test, never on the human, is reached.
@task_item n=7 part=touches
Zero flakiness and no retries: the never-ready boot case fell from about 20.6 s to 2.5 s and no longer risks the 20 s kill line; a row on flakiness or test timeouts in CI is reached.
@task_item n=8 part=touches
A11y cases run inside existing suites and coverage: the CI profile's kill-line filter moved from 25 to 28 binaries; a row naming the CI test profile, its filter or its binaries as where a11y cases run is reached only if it holds that count.
@task_item n=9 part=touches
TUI boundary driver: the rig drives the vendor TUI through its outer PTY for the new wake and imports arms; a row pinning the outer-PTY boundary driver for live or boundary tests is reached.
@task_item n=10 part=touches
Manual screen-reader and keyboard passes: the live rig's readings stand beside them as supplemental measurements; a row saying manual or live checks are supplemental and gate nothing is reached.
@task_item n=1 part=untouched
CLI output discipline: no human or --json output line, caption, board column, colour, SGR use, escaping of control bytes or fixed string moved.
@task_item n=2 part=untouched
Refusals and exit codes: no refusal word, hint, unable line, fault body, code word or CLI exit code was added, changed or retired.
@task_item n=3 part=untouched
viola run TUI passthrough contracts: the product's passthrough of human keystrokes, vendor content, its own terminal bytes and its diagnostics did not change; only test-side drivers read it.
@task_item n=4 part=untouched
viola list, wait, verify, link, revive and plugin contracts: none of these commands or their outputs moved, and the live viola verify --record was re-pinned to a later route entry, not run.
@task_item n=5 part=untouched
Visual design, cognitive accessibility and screen-reader targets: no colour, truncation, reading-order, screen-reader target or copy rule is reached.
@task_item n=6 part=untouched
Absences: no web, graphical or touch surface, no internationalization and no user time limit was added.
@task_item n=7 part=untouched
Structured violation JSON and the a11y tier: no a11y row shape, tier, conformance claim or compliance regime moved.
