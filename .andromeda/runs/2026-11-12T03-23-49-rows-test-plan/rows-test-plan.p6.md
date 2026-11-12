@row_add n=1 row=decision/msrv-check area=9-ci-integration rec=1,9
name: MSRV check on the security toolchain floor
rule: An ubuntu CI job checks the workspace and runs the unit suite on Rust 1.96, the minimum supported Rust version.
why: Rust 1.96 is the security toolchain floor.
@row_add n=2 row=decision/fuzz-corpus-replay-in-ci area=9-ci-integration rec=2
name: Fuzz corpus replay in CI
rule: Every push and pull request replays the fuzz corpus, and a nightly run fuzzes each target for a bounded time.
@row_add n=3 row=decision/release-build-viola-only area=9-ci-integration rec=3,24
name: Release build holds viola alone
rule: CI judges a release build of viola alone and fails on any test-only feature or binary in it, with no upload and no gate step.
guard: release-check and its probes
@row_add n=4 row=decision/quality-gate-command area=9-ci-integration rec=4,25
name: One quality gate over the test outputs
rule: The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.
@row_add n=5 row=decision/workflow-least-permissions area=9-ci-integration rec=5
name: Least workflow permissions
rule: Every CI workflow grants no permissions at the top and only read access to contents per job.
@row_add n=6 row=decision/sha-pinned-actions area=9-ci-integration rec=5
name: SHA-pinned actions
rule: Every action a CI workflow uses is pinned by commit SHA.
guard: zizmor unpinned-uses
@row_add n=7 row=decision/single-tool-version-source area=9-ci-integration rec=5
name: One version source for each tool
rule: Each tool has exactly one version source, the CI workflow's pins, and the local pre-push gate checks the host's tools against them and installs nothing.
@row_add n=8 row=decision/no-ci-concurrency-groups area=9-ci-integration rec=6
name: No concurrency groups in CI
rule: No CI workflow has a concurrency block, and zizmor's concurrency-limits finding is declined and left visible.
why: A concurrency group cancels pending runs, which would drop a push's always-run gate and upload chain.
@row_add n=9 row=decision/linux-only-ci-matrix area=9-ci-integration rec=8,7
name: Linux-only CI matrix
rule: CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.
@row_add n=10 row=decision/pinned-rust-toolchain area=9-ci-integration rec=9
name: Exactly pinned Rust toolchain
rule: The Rust toolchain is pinned to one exact stable version at or above the 1.96 floor and installed by rustup on both the CI runner and the host.
@row_add n=11 row=decision/machine-parseable-ci-reports area=9-ci-integration rec=10
name: Machine-parseable CI test reports
rule: CI test reports are machine-parseable and leave CI as uploaded artifacts.
@row_add n=12 row=decision/artifacts-behind-secret-scan area=9-ci-integration rec=11,14
name: Artifacts leave CI behind a secret scan
rule: Every test or perf artifact leaves CI only after its job's secret scan passed; when the scan fails only the scan's hit report is uploaded.
@row_add n=13 row=contract/ci-artifacts/harness area=9-ci-integration rec=11
boundary: CI artifacts
item: harness-${{ matrix.os }}
promise: Uploaded by the test job on failure after a passed secret scan: the agent-run output minus the mutation diff, with per-suite JUnit, run summary, coverage summary and the failure-only logs and status capture; no perf export.
@row_add n=14 row=contract/ci-artifacts/diag area=9-ci-integration rec=11
boundary: CI artifacts
item: diag-${{ matrix.os }}
promise: Holds the E2E homes' diagnostics NDJSON files from the test job, uploaded only behind a passed secret scan.
@row_add n=15 row=contract/ci-artifacts/junit area=9-ci-integration rec=11
boundary: CI artifacts
item: junit-${{ matrix.os }}
promise: Holds the one JUnit report of the test job's single instrumented coverage nextest invocation.
@row_add n=16 row=contract/ci-artifacts/secret-scan area=9-ci-integration rec=11
boundary: CI artifacts
item: secret-scan-${{ matrix.os }}
promise: Holds the test job's secret-scan hit report, uploaded only when the scan failed.
@row_add n=17 row=decision/ci-artifact-retention area=9-ci-integration rec=11
name: Seven-day artifact retention
rule: Every CI test artifact is kept for 7 days.
@row_add n=18 row=decision/per-suite-junit-copies area=9-ci-integration rec=11
name: Per-suite JUnit copies kept
rule: Each suite's JUnit report is kept as its own copy among the run artifacts.
why: Each nextest invocation overwrites the raw JUnit file.
@row_add n=19 row=decision/mutation-output-never-uploaded area=9-ci-integration rec=12
name: Mutation output never uploaded
rule: The mutation-testing output directory is never uploaded from CI.
why: Its outcomes file holds absolute argv paths and its log holds test output.
@row_add n=20 row=contract/ci-artifacts/perf area=9-ci-integration rec=14
boundary: CI artifacts
item: perf-${{ matrix.os }}
promise: Holds the perf job's hyperfine exports, uploaded only when the perf job's secret scan succeeded.
@row_add n=21 row=contract/ci-artifacts/diag-perf area=9-ci-integration rec=14
boundary: CI artifacts
item: diag-perf-${{ matrix.os }}
promise: Holds the perf run homes' diagnostics NDJSON files, uploaded only when the perf job's secret scan succeeded.
@row_add n=22 row=contract/ci-artifacts/secret-scan-perf area=9-ci-integration rec=14
boundary: CI artifacts
item: secret-scan-perf-${{ matrix.os }}
promise: Holds the perf job's secret-scan hit report, uploaded only when that scan failed.
@row_add n=23 row=decision/agent-reads-ci-through-gh area=9-ci-integration rec=17
name: Agent reads CI through gh
rule: The agent reads CI job conclusions and fetches artifacts with the gh CLI, and locally gets the same files from agent-run run and agent-run logs.
@row_add n=24 row=decision/build-fails-on-test-failure area=9-ci-integration rec=19,18
name: Build fails on any test failure
rule: The build fails when any unit, integration, E2E, contract or property test fails.
@row_add n=25 row=decision/build-fails-below-coverage area=9-ci-integration rec=20,18
name: Build fails below coverage threshold
rule: The build fails when coverage falls below the quality-gate threshold.
@row_add n=26 row=decision/build-fails-on-perf-breach area=9-ci-integration rec=21,18
name: Build fails on a performance budget breach
rule: The build fails on any hyperfine performance budget breach.
@row_add n=27 row=decision/build-fails-on-fuzz-crash area=9-ci-integration rec=21,18
name: Build fails on a fuzz replay crash
rule: The build fails when the fuzz corpus replay crashes.
@row_add n=28 row=decision/fmt-and-clippy-deny-warnings area=9-ci-integration rec=22,18
name: Formatting and clippy warnings fail the build
rule: Any rustfmt difference or clippy warning, with warnings denied, fails the build.
@row_add n=29 row=decision/sync-crates-without-tokio area=9-ci-integration rec=22,23,18
name: Sync crates stay free of tokio
rule: Every crate on the listed set of sync crates builds without tokio and passes a tokio ban; a failure, or an empty list, fails the build.
guard: sync-crates build check; deny-sync tokio ban
@row_add n=30 row=decision/no-orphan-modules area=9-ci-integration rec=22,18
name: No orphan modules
rule: Any cargo-modules orphan in any library or binary target fails the build.
guard: orphans-check and its probe
@row_add n=31 row=decision/rmcp-feature-limit area=9-ci-integration rec=22,18
name: rmcp limited to server and transport-io
rule: The build fails when rmcp is absent from viola's release graph or any rmcp feature beyond server and transport-io is requested.
guard: rmcp-features
@row_add n=32 row=decision/cargo-deny-gate area=9-ci-integration rec=23,18
name: cargo-deny supply-chain gate
rule: Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build.
guard: cargo deny; deny-probes, one probe per ban
@row_add n=33 row=decision/channel-tokio-free-by-default area=9-ci-integration rec=23,18
name: viola-channel reaches tokio only by feature
rule: The build fails when viola-channel reaches tokio without its feature enabled.
guard: channel-tokio-free
@row_add n=34 row=decision/zizmor-gate area=9-ci-integration rec=23,18
name: zizmor findings fail the build
rule: Any zizmor finding on the CI workflows fails the build.
@row_add n=35 row=decision/weekly-advisories-run area=9-ci-integration rec=23,18
name: Weekly advisories run
rule: A weekly nightly workflow checks advisories on the root and fuzz lockfiles, and its failure is a build failure.
@row_add n=36 row=decision/comprehensive-coverage-floors area=10-quality-gates-coverage-targets rec=33,28,35
name: Comprehensive-tier coverage floors
rule: viola's coverage must be at least 85% of lines, 80% of branches (enforced as region coverage) and 95% of functions, enforced in the ubuntu test job.
guard: cargo llvm-cov fail-under-lines 85, fail-under-functions 95, fail-under-regions 80
@row_add n=37 row=decision/coverage-workspace-rollup area=10-quality-gates-coverage-targets rec=36
name: Coverage rolled up across the workspace
rule: The coverage number rolls up across all workspace crates.
@row_add n=38 row=decision/coverage-exclusions area=10-quality-gates-coverage-targets rec=37
name: Coverage exclusions
rule: The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness.
why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
@row_add n=39 row=decision/child-process-coverage area=10-quality-gates-coverage-targets rec=38
name: Child viola processes counted in coverage
rule: viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE.
why: LLVM_PROFILE_FILE propagates to child processes.
except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
@row_add n=40 row=decision/doctests-outside-coverage area=10-quality-gates-coverage-targets rec=39
name: Doctests outside the coverage number
rule: Doctests are run but are not part of the coverage number.
@row_add n=41 row=decision/mutation-gate-cadence area=10-quality-gates-coverage-targets rec=40
name: Mutation gate at the epoch boundary only
rule: Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
@row_add n=42 row=decision/mutation-gate-thresholds area=10-quality-gates-coverage-targets rec=40
name: Mutation gate thresholds
rule: A mutation run must show no missed mutants after host-excluded records, no timeouts, and no more unviable mutants than caught ones.
why: A run whose mutant builds mostly failed is no evidence of test strength.
@row_add n=43 row=decision/host-unreachable-mutants-owed area=10-quality-gates-coverage-targets rec=40
name: Host-unreachable mutants recorded as owed
rule: A mutant the host cannot compile or reach is recorded as owed, never as equivalent or caught.
@row_add n=44 row=decision/perf-gate-on-max area=10-quality-gates-coverage-targets rec=42
name: Performance gate on the max sample
rule: The performance gate judges the max sample; mean, median and stddev are reported for trend only, and no percentile is computed or gated.
why: Claude Code hook deadlines are hard cutoffs.
@row_add n=45 row=decision/perf-own-release-build area=10-quality-gates-coverage-targets rec=44,43
name: Perf times its own release build
rule: The perf run times its own non-instrumented release workspace build with the fake agent, never a debug or coverage build, and never with LLVM_PROFILE_FILE set.
@row_add n=46 row=decision/perf-real-hook-path area=10-quality-gates-coverage-targets rec=45
name: Perf times hooks through the real path
rule: The perf run boots one unstamped session with a builder instance and times each hook through the real logging and channel path on synthetic payloads.
@row_add n=47 row=decision/perf-panic-check area=10-quality-gates-coverage-targets rec=45
name: Perf run checks for hook panics
rule: Before cleanup the perf run asserts no panic line in the home's role files.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_add n=48 row=decision/perf-failure-turns-red area=10-quality-gates-coverage-targets rec=45
name: Perf failures turn the suite red
rule: A failed build, boot, timed row, panic check or cleanup turns suite perf red, never a missing or skipped row.
@row_add n=49 row=decision/perf-hyperfine-invocation area=10-quality-gates-coverage-targets rec=46
name: Hyperfine invocation shape
rule: Every hyperfine call runs without a shell, with 3 warm-up runs and 30 timed runs, exporting JSON to the run artifacts; warm-up runs are not part of max.
why: Warm-up runs absorb first-execution costs.
@row_add n=50 row=decision/perf-no-retry area=10-quality-gates-coverage-targets rec=47
name: No retry of a performance breach
rule: The gate evaluates the exported perf JSON once, with no rerun, outlier trimming or second attempt; a breach is fixed in the product or escalated through a Decisions Log entry.
@row_add n=51 row=decision/spine-deadline-single-source area=10-quality-gates-coverage-targets rec=48
name: Spine deadline from the product constant
rule: The perf gate reads the spine deadline from the product's own constant (1.0 s), never from a literal edited into CI or the test plan.
@row_add n=52 row=decision/hook-connect-deadline area=10-quality-gates-coverage-targets rec=48
name: Hook connect deadline below the spine deadline
rule: The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces.
guard: connect_deadline_is_below_the_spine_deadline
state: provisional
@row_add n=53 row=decision/perf-in-own-ci-job area=10-quality-gates-coverage-targets rec=49
name: Perf runs in its own CI job
rule: CI runs perf in its own perf job ending in a gate that requires perf; the test job and the pre-push gate carry no perf step, and no async-tier hook is timed.
@row_add n=54 row=decision/fail-open-spine-bound-at-unit-speed area=10-quality-gates-coverage-targets rec=49
name: Fail-open spine bound asserted at unit speed
rule: Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed.
guard: hook_fails_open_silently_within_the_spine_bound
@row_add n=55 row=contract/perf-budgets/session-end area=10-quality-gates-coverage-targets rec=52
boundary: Performance budgets
item: viola hook session-end
promise: Timed on a synthetic payload through the live wrapper; mean, median and stddev reported for trend, gated on max under 1.0 s.
@row_add n=56 row=contract/perf-budgets/spine-hooks area=10-quality-gates-coverage-targets rec=53
boundary: Performance budgets
item: spine hooks session-start, user-prompt-submit, stop
promise: Timed the same way with VIOLA_NAME and VIOLA_DIR exported to hyperfine; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).
@row_add n=57 row=contract/perf-budgets/pre-tool-use area=10-quality-gates-coverage-targets rec=54
boundary: Performance budgets
item: viola hook pre-tool-use
promise: Timed on an unverified CLI whose dialog answer is null at once, a required perf row; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).
@row_add n=58 row=contract/heartbeat-staleness/age-threshold area=10-quality-gates-coverage-targets rec=55
boundary: Heartbeat staleness classification
item: heartbeat age
promise: For the same process, a heartbeat aged 5.0 s classifies live and one aged 5.1 s classifies stale, held by an exact boundary test.
@row_add n=59 row=contract/heartbeat-staleness/identity-mismatch area=10-quality-gates-coverage-targets rec=55
boundary: Heartbeat staleness classification
item: pid or start-time mismatch
promise: A pid or start-time mismatch classifies the instance gone at any age.
@row_none n=1 rec=13 type=5
@row_none n=2 rec=15 type=5
@row_none n=3 rec=16 type=5
@row_none n=4 rec=26 type=S
@row_none n=5 rec=27 type=S
@row_none n=6 rec=29 type=S
@row_none n=7 rec=30 type=S
@row_none n=8 rec=31 type=0
what: the coverage floors of the Minimal tier, a tier this project does not use
@row_none n=9 rec=32 type=0
what: the coverage floors of the Standard tier, a tier this project does not use
@row_none n=10 rec=34 type=S
@row_none n=11 rec=41 type=S
@row_none n=12 rec=50 type=S
@row_none n=13 rec=51 type=S
