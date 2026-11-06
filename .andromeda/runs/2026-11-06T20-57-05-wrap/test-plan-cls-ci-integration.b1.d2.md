@answer n=1 entity=ci-integration#1.1 v=no
why: the lint crate list and the orphans and rmcp-features commands are unchanged, and green ci#42288148289 on 849aad18 covers them on the changed Cargo.toml and Cargo.lock
@answer n=2 entity=ci-integration#1.2 v=no
why: the deny.toml check sections still hold and deny-probes re-measured 14/14 clean, the emptied ignore list is not named by this entity
@answer n=3 entity=ci-integration#1.3 v=no
why: the unit command and its nextest process-per-test parallelism are unchanged, and the entity names no test the session added or renamed
@answer n=4 entity=ci-integration#1.4 v=no
why: the integration command is unchanged, and the new contract test and the renamed harness test are not named by this entity
@answer n=5 entity=ci-integration#1.5 v=no
why: the E2E command and its one ubuntu-latest leg are unchanged, and the entity names no test the session edited
@answer n=6 entity=ci-integration#1.6 v=no
why: the coverage command, the test job gate set and the 85/95/80 floors hold, and coverage.rs changed only comments with its regex trees byte-identical
@answer n=7 entity=ci-integration#1.7 v=no
why: mutation testing stays on demand and at the epoch boundary, and the session changes neither the mutation command nor its schedule
@answer n=8 entity=ci-integration#1.8 v=no
why: the perf command, the hyperfine pin, the artifact path and the one ubuntu-latest matrix leg are unchanged, and no workflow was edited
@answer n=9 entity=ci-integration#1.9 v=no
why: the MSRV 1.96 pin and the msrv job are stated unchanged, and no workflow was edited
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz-replay job, the cargo-fuzz pin and the fuzz toolchain are not reached by the session's changes
@answer n=11 entity=ci-integration#1.11 v=no
why: release-check probes re-measured 5/5, and the release build command and its test-feature refusal are unchanged
@answer n=12 entity=ci-integration#1.12 v=no
why: the gate's readers, the coverage threshold table and the section 10 breach rule are unchanged per the report
@answer n=13 entity=ci-integration#1.13 v=yes mode=stale
why: .config/nextest.toml was edited with re-worded comments, so the cited line .config/nextest.toml:24 may have moved
@answer n=14 entity=ci-integration#1.14 v=yes mode=contradict
why: the supply-chain job no longer reads red on RUSTSEC-2026-0441, since the advisory is closed, portable-pty is 0.9.0 and the ignore list is empty on green ci#42288148289
