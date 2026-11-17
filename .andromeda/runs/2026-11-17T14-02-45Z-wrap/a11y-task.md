@task_prose part=session
The session turned the CI supply-chain check green again: the cargo-deny advisory ignore list gains a fourth id, RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade), on a second ground, an unmaintained upstream, beside the first ground for the rmcp advisories.
It ends every booted end-to-end test process with its test: the in-test harness boot's stamping viola verify now gets SIGTERM when its test thread ends (it was SIGKILL), and the supervisor still gets SIGKILL.
It added two integration tests: a killed test leaves no stamping verify behind (checked by process, run-dir and stamp reads), and a census finding no untied boot in the e2e crate's tests. The in-test boots of the harness lifecycle and e2e cli tests are now tied.
The shared source-scan masker's 14-row rstest table is now one test over a labelled case table, so the coverage count fell from 3510 to 3462 with the cases themselves unchanged. No product code, command output, status shape, log format, dependency or spec master changed.
CI on the pre-CI commit read green with 7 of 7 checks, supply-chain among them; pre-push gate breaches none. The longest harness lifecycle case takes 20.6 s against a 20 s CI kill line. That finding goes to route-resolve and is not amended here.
@task_file n=1 path=deny.toml
what: the supply-chain check's cargo-deny configuration, holding the advisory ignore list with each entry's reason, plus the bans, licences and sources rules
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e test harness boot that starts a wrapped session under test with its supervisor and stamping viola verify, and ties each to the test thread by a parent-death signal
@task_file n=3 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the e2e harness lifecycle integration tests that boot a session and check readiness, timeout and stop
@task_file n=4 path=crates/viola-e2e/tests/cli.rs
what: the e2e CLI integration tests that drive status, logs and cleanup against a booted session
@task_file n=5 path=crates/viola-e2e/tests/harness_tied.rs
what: the e2e integration tests that a killed test leaves no harness session or stamping verify behind, and the census of untied in-test boots
@task_file n=6 path=tests/support/source_scan.rs
what: the shared source-scan masker that census and contract tests use to blank comments and strings before scanning code
@task_item n=1 part=touches
decision/a11y-ci-github-actions: the push/PR workflow's supply-chain check went from red to green (7 of 7 checks green). Nothing about the a11y checks in that workflow changed.
@task_item n=2 part=touches
decision/ci-test-gate-no-skips: three integration tests were added, all running and none skipped. The test gate still allows no skips.
@task_item n=3 part=touches
decision/zero-flakiness: the new kill-witness and census tests wait under a fixed bound with no retries. Their wait now needs verify to be mid-run inside its own run dir.
@task_item n=4 part=touches
decision/a11y-cases-inside-coverage-suite: the coverage suite's passed count fell from 3510 to 3462 because one table of shared-helper cases collapsed into one test. No a11y case was added or removed.
@task_item n=5 part=touches
decision/a11y-failures-are-nextest-failures: indirectly, through the nextest CI kill-line finding. The harness lifecycle readiness-timeout case runs 20.6 s against the 20 s ci kill line. That finding is placed at route-resolve, not here.
@task_item n=6 part=touches
decision/gate-requires-coverage-doctest and decision/a11y-gate-breach-fails-build: the pre-push coverage run read green with gate breaches none, at the new passed count.
@task_item n=7 part=touches
contract/interrupted-verify/exit, contract/interrupted-verify/stderr and contract/viola-verify/interrupted-pair: indirectly. The tied test verify now ends by TERM, which takes verify's stopped path, so it removes its own run dirs and writes no stamp. The verify output and exit contracts are unchanged.
@task_item n=8 part=touches
decision/time-bounds-fall-on-the-driver: indirectly. The test harness's new waits are bounded on the test driver, never on a human.
@task_item n=9 part=touches
decision/mutation-audit-covers-crate-invariants: indirectly. The shared masker's own cases now run as one labelled-table test in every test binary that takes it, the e2e crate's included.
@task_item n=1 part=untouched
TUI passthrough contracts (entry, human keystrokes, viola terminal bytes, vendor content, focus/mouse/resize, vt100 feed panic, run diagnostics). No passthrough behaviour changed.
@task_item n=2 part=untouched
CLI refusal, fault and --json failure contracts, and exit codes 2, 10, 11, 13, 14, 20 and 21. No refusal word, hint, code word or exit code changed.
@task_item n=3 part=untouched
viola list board and caption contracts and the colour/SGR decisions (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR). No output styling changed.
@task_item n=4 part=untouched
Screen-reader targets and manual Orca and keyboard passes. No manual pass was run or changed.
@task_item n=5 part=untouched
Cognitive accessibility decisions (fixed CLI strings pinned by literal, one hint per code, errors as word, detail and hint). No fixed CLI copy changed.
@task_item n=6 part=untouched
Scope absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit). No capability was added.
@task_item n=7 part=untouched
viola revive, link/unlink, plugin install, hook call, MCP frame tool and frame verb refusal contracts. No product command changed.
