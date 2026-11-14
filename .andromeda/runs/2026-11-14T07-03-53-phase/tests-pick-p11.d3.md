@pick n=1 row=decision/perf-in-own-ci-job v=no
@pick n=2 row=decision/perf-hyperfine-invocation v=no
@pick n=3 row=decision/fake-agent-not-proof-of-real-cli v=no
@pick n=4 row=decision/endpoint-from-snapshot-only v=no standing=S5
@pick n=5 row=decision/mutation-gate-cadence v=yes
@pick n=6 row=decision/deny-bans-probed v=yes
@pick n=7 row=decision/host-unreachable-mutants-excluded v=no
@pick n=8 row=decision/test-failure-fails-build v=no standing=S6
@pick n=9 row=decision/no-ignored-advisories v=no
@pick n=10 row=decision/module-acyclicity-not-gated v=no
@pick n=11 row=contract/heartbeat-staleness/age-threshold v=no
@pick n=12 row=decision/actions-sha-pinned v=no
@pick n=13 row=decision/profraw-census-dev-host-only v=no
@pick n=14 row=decision/lint-gates-fail-build v=yes
@pick n=15 row=decision/coverage-exclusions v=no standing=S6
@pick n=16 row=decision/child-process-coverage v=yes
@pick n=17 row=decision/serde-json-unbounded-depth-banned v=yes
@pick n=18 row=decision/hook-deadlines-asserted-at-unit-speed v=no
@pick n=19 row=decision/nightly-fuzzing v=yes
@pick n=20 row=decision/perf-gate-spine-deadline v=no
@pick n=21 row=contract/heartbeat-staleness/identity-mismatch v=no
@pick n=22 row=decision/coverage-threshold-never-lowered v=no standing=S6
@pick n=23 row=contract/mutants-verdict/no-rust-delta v=no
@pick n=24 row=decision/ci-single-ubuntu-leg v=yes
@pick n=25 row=decision/assert-on-contract-not-fixture v=no standing=S5
@pick n=26 row=decision/coverage-workspace-rollup v=no standing=S6
@pick n=27 row=decision/child-exit-by-wait v=yes
@pick n=28 row=decision/fuzz-corpus-replay-on-push v=yes
@pick n=29 row=decision/no-cache-in-release-workflow v=no
@pick n=30 row=decision/local-pre-push-gate-installs-nothing v=no
@pick n=31 row=decision/no-workflow-concurrency v=no
@pick n=32 row=decision/mutation-gate-thresholds v=no
@pick n=33 row=decision/cargo-deny-supply-chain-gate v=yes
@pick n=34 row=contract/mutants-verdict/unviable-exceeds-caught v=no
@pick n=35 row=decision/doctests-outside-coverage v=no
@pick n=36 row=decision/literal-test-oracles v=no standing=S5
@pick n=37 row=decision/send-and-wait-latency-not-timed v=no
@pick n=38 row=decision/perf-own-release-build v=no
@pick n=39 row=decision/ci-stamped-home-kill-line v=no
@pick n=40 row=decision/no-ignore-parking v=no standing=S5
@pick n=41 row=decision/comprehensive-coverage-floors v=no standing=S6
@pick n=42 row=decision/hyperfine-measures-perf v=no
@pick n=43 row=contract/perf-budgets/spine-hooks v=no
@pick n=44 row=contract/mutants-verdict/package v=no
@pick n=45 row=decision/viola-channel-tokio-free v=no
@pick n=46 row=decision/cleanup-between-integration-tests v=no standing=S5
@pick n=47 row=decision/chaos-faults-explicit v=no standing=S5
@pick n=48 row=contract/mutants-verdict/counted v=no
@pick n=49 row=decision/hyperfine-max-perf-gate v=no
@pick n=50 row=contract/mutants-verdict/test-only-rust-delta v=no
@pick n=51 row=decision/peer-admitted-survivor-known-limit v=no
@pick n=52 row=decision/harness-gate-fails-on-breach v=no
@pick n=53 row=contract/perf-budgets/session-end v=no
@pick n=54 row=decision/perf-panic-check v=no
@pick n=55 row=decision/mutants-verdict-counted-by-harness v=no
@pick n=56 row=decision/killed-test-child-uninstrumented v=yes
@pick n=57 row=decision/harness-output-json-and-exit-code v=no
@pick n=58 row=decision/no-committed-test-secrets v=yes
@pick n=59 row=decision/rmcp-features-server-transport-io v=no
@pick n=60 row=decision/perf-verdict-from-harness-gate v=no
@pick n=61 row=decision/coverage-threshold-fails-build v=no standing=S6
@pick n=62 row=decision/mutation-gate-at-epoch-boundary v=yes
@pick n=63 row=decision/workflows-audited-by-zizmor v=no
@pick n=64 row=decision/perf-failure-turns-red v=no
@pick n=65 row=decision/perf-gate-on-max v=no
@pick n=66 row=decision/spine-deadline-single-source v=no
@pick n=67 row=decision/jq-runner-provided v=no
@pick n=68 row=decision/weekly-advisory-scan v=no
@pick n=69 row=decision/env-clear-keeps-profile-file v=yes
@pick n=70 row=decision/host-unreachable-mutants v=no
@pick n=71 row=decision/msrv-1-96 v=no
@pick n=72 row=decision/no-test-binary-in-release v=yes
@pick n=73 row=decision/perf-panic-check-before-cleanup v=no
@pick n=74 row=decision/no-blanket-approval-tests v=no
@pick n=75 row=decision/report-files-per-test-process v=no
@pick n=76 row=decision/hook-connect-deadline v=no
@pick n=77 row=decision/every-surface-asserted v=yes
@pick n=78 row=decision/perf-hooks-time-the-real-path v=no
@pick n=79 row=decision/tokio-banned-per-sync-crate v=yes
@pick n=80 row=decision/zero-flakiness v=yes
@pick n=81 row=decision/fake-agent-own-process-group v=no
@pick n=82 row=decision/ci-jobs-end-in-harness-gate v=no
@pick n=83 row=decision/one-instrumented-ci-test-run v=no
@pick n=84 row=decision/one-version-source-per-tool v=no
@pick n=85 row=decision/lint-fmt-and-clippy-deny-warnings v=no
@pick n=86 row=decision/ripgrep-pinned-checksum-verified v=no
@pick n=87 row=decision/fail-open-spine-bound-at-unit-speed v=no
@pick n=88 row=decision/perf-failure-turns-suite-red v=no
@pick n=89 row=decision/perf-no-retry v=no
@pick n=90 row=decision/boot-deadline-not-a-budget v=no
@pick n=91 row=decision/coverage-includes-child-processes v=yes
@pick n=92 row=decision/mutation-testing-off-ci v=yes
@pick n=93 row=decision/least-workflow-permissions v=no
@pick n=94 row=decision/perf-real-hook-path v=no
@pick n=95 row=decision/coverage-ignore-regex-not-widened v=no standing=S6
@pick n=96 row=contract/perf-budgets/pre-tool-use v=no
@pick n=97 row=decision/sync-crates-compile-without-features v=yes
@pick n=98 row=decision/e2e-asserts-only-cross-process v=no standing=S5
@pick n=99 row=decision/mutation-harness-tests-ungated v=no
@pick n=100 row=decision/doctest-only-catch-is-missed v=no
