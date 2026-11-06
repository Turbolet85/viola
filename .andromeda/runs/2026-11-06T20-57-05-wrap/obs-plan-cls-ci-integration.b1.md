@answer n=1 entity=ci-integration#1.1 v=no
why: the matrix is now ubuntu-latest only and diag-${{ matrix.os }} still resolves on it, with CI section 9 re-read and no rule moved
@answer n=2 entity=ci-integration#1.2 v=no
why: the harness-${{ matrix.os }} upload and its failure() condition hold on the ubuntu-only matrix, with section 9 re-read and no rule moved
@answer n=3 entity=ci-integration#1.3 v=no
why: the junit-${{ matrix.os }} artifact and the nextest junit path hold, with only the timing comments changed and no rule moved
@answer n=4 entity=ci-integration#1.4 v=no
why: the perf-${{ matrix.os }} and diag-perf artifacts are confirmed on ubuntu-latest by the green perf job, with no rule moved
@answer n=5 entity=ci-integration#1.5 v=no
why: the row is N/A because the session adds no OTel exporter
@answer n=6 entity=ci-integration#2.1 v=no
why: the lint and cargo deny gates are green on the run, and the emptied advisories ignore is not named here while the listed bans still hold
@answer n=7 entity=ci-integration#2.2 v=no
why: the in-memory writer unit tests are not the session's change and the nextest job is green on the run
@answer n=8 entity=ci-integration#2.3 v=no
why: the target/e2e-home integration homes and the chaos outside_scan carve-out still hold because G2, G4 and the secret scan are green on the run
@answer n=9 entity=ci-integration#2.4 v=no
why: the E2E zero-panic gate G2 and schema gate G4 are green on the run and no E2E rule moved
@answer n=10 entity=ci-integration#2.5 v=no
why: the OS-gated census rule is unchanged by removing the Windows and macOS remnant files and the e2e census is green on the run
@answer n=11 entity=ci-integration#2.6 v=no
why: the green seven-job CI run carries no mutation job, so run --mutants stays at the epoch boundary and on demand
@answer n=12 entity=ci-integration#3.1 v=no
why: the G1 bare-#[instrument] check is green on ci#42288148289 and no instrument rule moved
@answer n=13 entity=ci-integration#3.2 v=no
why: the G2 probe and the seam exemption are green on the run and src/cmd/hook/seam.rs is not in the change set
@answer n=14 entity=ci-integration#3.3 v=no
why: G4 schema conformance is green on the run and no schema or diagnostic line shape changed
@answer n=15 entity=ci-integration#4.1 v=no
why: the test-job order, the if: always() on G2 and G4 and the chaos carve-out hold on the green run, with no perf step changed
@answer n=16 entity=ci-integration#5.1 v=no
why: the secret-scan reading and AGENT_RUN_KEEP_HOMES=1 hold on the green run and the session does not move v1-47
