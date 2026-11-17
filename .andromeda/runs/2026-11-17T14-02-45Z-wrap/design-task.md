@task_prose part=session
The supply-chain gate reads green again: the dependency advisory ignore list grows from three ids to four, adding RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade) on a second ground, an unmaintained upstream with no safe upgrade, beside the rmcp ground.
Test-only harness change: the stamping viola verify started by the in-test boot is now tied to its test thread by SIGTERM rather than SIGKILL, so a stopped verify ends its children and removes its own run dirs; the supervisor stays on SIGKILL.
Two new integration tests: a killed test leaves no stamping verify behind (a stopped verify writes no stamps.json), and a census finds no untied boot in the e2e tests.
One shared source-scan masker test became a single test over a labelled table of the same 14 cases, so the test count fell from 3510 to 3462 with no case lost.
No product code, CLI output, command, exit code, status shape, log format, dependency or crate changed; CI read green 7 of 7.
@task_file n=1 path=deny.toml
what: the supply-chain policy: the dependency advisory ignore list with each ignore's reason, plus bans, licences and sources
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: the test-only harness boot that starts a booted session, its supervisor and its stamping viola verify, tied to the test thread by a parent-death signal
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the harness lifecycle tests that drive booted sessions through the in-test boot
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: the e2e CLI tests that drive a booted session's status, logs and cleanup
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: the tied-harness tests: a killed test leaves no harness session and no stamping verify, and the census of untied boots
@task_file n=6 path=tests/support/source_scan.rs
what: the shared source-scan masker that census tests use to blank comments and strings before they scan source
@task_item n=1 part=touches
Indirect, no change: the viola verify stopped or interrupted run. The tied verify now ends on TERM in tests, and a stopped verify returns interrupted before it updates stamps.json. The new test reads that the stamp is absent. The stopped-or-refused output and the interrupted refusal rows are unchanged, only exercised.
@task_item n=2 part=touches
Indirect, no change: the viola verify stamps.json update. A stopped verify never reaches the stamp update, and the new witness relies on that. The stamps-update refusal row still holds.
@task_item n=3 part=touches
Indirect, no change: the silent catch sites and the stdout stream. The measurement found that verify drops its result writes to a dead stdout pipe and runs to its end. Behaviour is unchanged; a row on silent writes or the stdout stream still holds.
@task_item n=4 part=touches
Indirect, no change: the verify run dirs in the OS temp dir. They are removed by verify's own guard when it passes, fails or is stopped by Ctrl-C or TERM, and left to be removed by hand after SIGKILL. The tied test verify now goes the TERM way. A verify output or refusal row naming that removal still holds.
@task_item n=5 part=touches
Indirect, no change: name/command/verify. The verb and its help are unchanged; only the test harness's signal to it moved.
@task_item n=6 part=touches
The supply-chain advisory ignore list now holds four ids under two grounds. No design-system row holds that list, and a picker should flag only a row whose cell names the ignore count or the rule that admits an ignore.
@task_item n=1 part=untouched
Brand identity, flight-progress strip, readback box, handoff and send readback rows: no displayed element or wording changed.
@task_item n=2 part=untouched
Colour palette, colour tokens, colour decision order, colour depth, typography, spacing, iconography and glyph rows: no styling changed.
@task_item n=3 part=untouched
viola list board and table rows, NAME truncation, the WHEEL column and skip counts: the board is unchanged.
@task_item n=4 part=untouched
CLI exit codes, exit 21 causes, --json refusal codes, refusal hints, strict-modes refusals and wrapper fault detail: no code, cause or hint moved.
@task_item n=5 part=untouched
run, revive, link, unlink, send, wait, last, answer, pause, release, mcp, hook and plugin install output and refusal rows: none of those verbs changed.
@task_item n=6 part=untouched
viola --help verb groups, the flat verb set, the hidden mcp verb and the command names: no verb was added, renamed or removed.
@task_item n=7 part=untouched
The Self-Validation Protocol, anti-pattern rows and the absences of a rendered page, a web surface and a CLI tape: nothing rendered was added.
