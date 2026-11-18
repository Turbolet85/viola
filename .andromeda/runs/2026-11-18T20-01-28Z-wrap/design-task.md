@task_prose part=session
The session landed the recorded fixture set for claude 2.1.325, 28 files, copied from two live recorder starts under the test rig's private config.
The test-only live rig gained two recording arms: one keeps the clean record of a live viola verify run, one records the viola mcp exchange through the test-only tap.
The rig refuses a recording start whose scratch base names a home component, and reports new summary words for the record and the mcp child.
The ledger probes' list of stamped recorded sets grew from two to three, so the recorded sets replayed grew from four to five.
No product crate, CLI verb, output line, exit code, colour or format of viola changed; CI on the pushed head read green.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's arms, plan and start, which drive viola verify and viola mcp against a live CLI and summarise each start
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: the test-only live rig's private scratch and its record keep, which copies a recorder's output and checks a path for a home component
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only viola-live-rig binary's command line, its rehearse arms and live labels
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rehearsal tests of the test-only live rig
@task_file n=5 path=tests/contract_ledger_probes.rs
what: the contract ledger probe suite and its list of stamped recorded fixture sets
@task_file n=6 path=fixtures/claude/2.1.325/
what: the recorded fixture set for claude 2.1.325, hook payloads, screens and the recorded MCP exchange that the drift and replay suites read
@task_item n=1 part=touches
A row on the CLI verification state, verified or unverified-cli, where it names which CLI versions hold a recorded set: 2.1.325 now has one, stamped in the test probes only, with no stamps.json written.
@task_item n=2 part=touches
A row on viola verify refusals for an unclean fixture, indirectly: the new 2.1.325 set passed the fixture hygiene walk.
@task_item n=3 part=touches
A row on viola verify output, its step line or summary, indirectly: the rig's recording arm read a live verify run at 24 of 24 on 2.1.325, with no output form changed.
@task_item n=4 part=touches
A row on the mcp verb's stdout or the hidden mcp verb, indirectly: a live viola mcp exchange on 2.1.325 was recorded through the test-only tap, the verb itself unchanged.
@task_item n=5 part=touches
A row on the browser-tools prompt or the trust-record row of viola verify output, indirectly: the recording start ran with the browser-tools key seeded off and read the prompt absent.
@task_item n=6 part=touches
Any row whose cell holds a count of recorded fixture sets or stamped sets: recorded sets 4 to 5, stamped sets 2 to 3.
@task_item n=1 part=untouched
Brand identity, the flight-progress strip, the readback box and the handoff display: no strip, readback or transfer rendering changed.
@task_item n=2 part=untouched
Colour palette, colour tokens, colour decision order and colour depth: no colour or SGR choice changed.
@task_item n=3 part=untouched
Typography, spacing, iconography, glyphs and separators: no CLI character, column or separator changed.
@task_item n=4 part=untouched
The web surface and rendered-page absences: nothing renders a page.
@task_item n=5 part=untouched
viola list board and table, viola last, viola wait, viola send mirror and send readback rows: none of those verbs changed.
@task_item n=6 part=untouched
run, revive, link and unlink output, their refusals and --json codes, exit 21 causes, strict-modes refusals and refusal hints: no refusal or exit code of viola moved; the rig's own refusal is no viola refusal.
@task_item n=7 part=untouched
CLI exit codes, streams, catch sites and wrapper fault details: no product crate was reached.
@task_item n=8 part=untouched
The anti-patterns and self-validation protocol: no human-facing output was added.
