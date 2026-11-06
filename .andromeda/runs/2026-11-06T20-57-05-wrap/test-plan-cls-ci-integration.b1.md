@answer n=1 entity=ci-integration#1.1 v=no
why: the lint commands, the sync-crates cargo check list, the orphans probes and the rmcp feature assertion name no changed crate, count or dependency
@answer n=2 entity=ci-integration#1.2 v=no
why: deny.toml's advisories ignore list is emptied, but the advisories, licences, sources and bans sections, the serde_json feature ban and deny-probes 14/14 that the row names still stand
@answer n=3 entity=ci-integration#1.3 v=no
why: the nextest.toml change is comment-only on three-runner timing, so the unit run line and the nextest process-per-test parallel mode stand
@answer n=4 entity=ci-integration#1.4 v=no
why: the integration run line and its parallel mode are unchanged, and the session edits no workflow step
@answer n=5 entity=ci-integration#1.5 v=no
why: the e2e run on the ubuntu-latest one-leg matrix is unchanged, since no workflow was edited and the single leg stands
@answer n=6 entity=ci-integration#1.6 v=no
why: coverage.rs got comments only and the regex rule stands, while the instrumented run and the coverage,doctest gate suites are unchanged
@answer n=7 entity=ci-integration#1.7 v=no
why: the session names no change to the mutation run, its gate or its epoch-boundary reading, so the stage's claim holds
@answer n=8 entity=ci-integration#1.8 v=yes mode=stale
why: the perf row's per-OS parallel reading is a Windows remnant the session retires, since the perf job runs as the ubuntu one-leg matrix
@answer n=9 entity=ci-integration#1.9 v=no
why: the MSRV job keeps its 1.96 toolchain and nextest-unit gate, which the session leaves unchanged
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz replay and nightly fuzz jobs name no touched file or dependency, and the cargo-fuzz pin and fuzz channel are unchanged
@answer n=11 entity=ci-integration#1.11 v=yes mode=stale
why: the release build row's per-OS parallel wording is a Windows remnant the session retires, while release-check probes 5/5 and its viola-only verdict stand
@answer n=12 entity=ci-integration#1.12 v=no
why: the session moves no §10 threshold and the gate's report inputs are unchanged, so the gate's exit rule still holds
@answer n=13 entity=ci-integration#1.13 v=yes mode=stale
why: the nextest.toml comments were re-worded on the ubuntu reading, so the cited filter line 24 and its 26-binary count must be re-read on the changed file
@answer n=14 entity=ci-integration#1.14 v=yes mode=contradict
why: RUSTSEC-2026-0441 is closed by portable-pty =0.9.0 with pin-utils gone and the ignore list emptied, and ci#42288148289 is green, so the recorded red no longer holds
