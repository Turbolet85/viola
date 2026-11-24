@pick n=1 row=decision/rmcp-features-server-transport-io v=no
@pick n=2 row=decision/serde-json-unbounded-depth-banned v=no
@pick n=3 row=decision/sync-crates-compile-without-features v=no
@pick n=4 row=decision/tokio-banned-per-sync-crate v=no
@pick n=5 row=decision/viola-channel-tokio-free v=no
@pick n=6 row=decision/weekly-advisory-scan v=no
@pick n=7 row=decision/workflows-audited-by-zizmor v=no
@pick n=8 row=decision/boot-deadline-not-a-budget v=no
@pick n=9 row=decision/child-process-coverage v=yes
@pick n=10 row=decision/comprehensive-coverage-floors v=no standing=S6
@pick n=11 row=decision/coverage-exclusions v=no
@pick n=12 row=decision/coverage-threshold-fails-build v=no standing=S6
@pick n=13 row=decision/coverage-workspace-rollup v=no standing=S6
@pick n=14 row=decision/doctests-outside-coverage v=no
@pick n=15 row=decision/fail-open-spine-bound-at-unit-speed v=no
@pick n=16 row=decision/hook-connect-deadline v=no
@pick n=17 row=decision/hook-deadlines-asserted-at-unit-speed v=no
@pick n=18 row=decision/host-unreachable-mutants-excluded v=no standing=S6
@pick n=19 row=decision/hyperfine-max-perf-gate v=no
@pick n=20 row=decision/lint-gates-fail-build v=no
@pick n=21 row=decision/mutants-verdict-counted-by-harness v=no standing=S6
@pick n=22 row=decision/mutation-gate-cadence v=no standing=S6
@pick n=23 row=decision/mutation-gate-thresholds v=no standing=S6
@pick n=24 row=decision/no-ignore-parking v=no standing=S6
@pick n=25 row=decision/no-test-binary-in-release v=no
@pick n=26 row=decision/perf-failure-turns-red v=no
@pick n=27 row=decision/perf-gate-on-max v=no
@pick n=28 row=decision/perf-hyperfine-invocation v=no
@pick n=29 row=decision/perf-in-own-ci-job v=no
@pick n=30 row=decision/perf-no-retry v=no
@pick n=31 row=decision/perf-own-release-build v=no
@pick n=32 row=decision/perf-panic-check v=no
@pick n=33 row=decision/perf-real-hook-path v=no
@pick n=34 row=decision/send-and-wait-latency-not-timed v=no
@pick n=35 row=decision/spine-deadline-single-source v=no
@pick n=36 row=decision/test-failure-fails-build v=no
@pick n=37 row=decision/zero-flakiness v=yes
@pick n=38 row=contract/heartbeat-staleness/age-threshold v=yes
@pick n=39 row=contract/heartbeat-staleness/identity-mismatch v=yes
@pick n=40 row=contract/perf-budgets/pre-tool-use v=no
@pick n=41 row=contract/perf-budgets/session-end v=no
@pick n=42 row=contract/perf-budgets/spine-hooks v=no
@pick n=43 row=decision/coverage-includes-child-processes v=yes
@pick n=44 row=decision/doctest-only-catch-is-missed v=no standing=S6
@pick n=45 row=decision/equivalent-survivors-known-limit v=no standing=S6
@pick n=46 row=decision/fake-agent-own-process-group v=no
@pick n=47 row=decision/host-unreachable-mutants v=no standing=S6
@pick n=48 row=decision/killed-test-child-uninstrumented v=no
@pick n=49 row=decision/mutation-gate-at-epoch-boundary v=no standing=S6
@pick n=50 row=decision/mutation-harness-tests-ungated v=no standing=S6
@pick n=51 row=decision/peer-admitted-survivor-known-limit v=no standing=S6
@pick n=52 row=decision/perf-failure-turns-suite-red v=no
@pick n=53 row=decision/perf-gate-spine-deadline v=no
@pick n=54 row=decision/perf-hooks-time-the-real-path v=no
@pick n=55 row=decision/perf-panic-check-before-cleanup v=no
@pick n=56 row=decision/profraw-census-dev-host-only v=no
@pick n=57 row=decision/report-files-per-test-process v=no
@pick n=58 row=contract/mutants-verdict/counted v=no
@pick n=59 row=contract/mutants-verdict/no-rust-delta v=no
@pick n=60 row=contract/mutants-verdict/package v=no
@pick n=61 row=contract/mutants-verdict/test-only-rust-delta v=no
@pick n=62 row=contract/mutants-verdict/unviable-exceeds-caught v=no
@pick n=63 row=decision/assert-on-contract-not-fixture v=no standing=S4
@pick n=64 row=decision/chaos-faults-explicit v=yes
@pick n=65 row=decision/child-exit-by-wait v=yes
@pick n=66 row=decision/cleanup-between-integration-tests v=no standing=S4
@pick n=67 row=decision/coverage-ignore-regex-not-widened v=no
@pick n=68 row=decision/coverage-threshold-never-lowered v=no
@pick n=69 row=decision/e2e-asserts-only-cross-process v=yes
@pick n=70 row=decision/echo-oracle-skips-allowed-word v=no
@pick n=71 row=decision/endpoint-from-snapshot-only v=yes
@pick n=72 row=decision/env-clear-keeps-profile-file v=yes
@pick n=73 row=decision/every-surface-asserted v=yes
@pick n=74 row=decision/fake-agent-not-proof-of-real-cli v=yes
@pick n=75 row=decision/harness-output-json-and-exit-code v=no
@pick n=76 row=decision/literal-test-oracles v=yes
@pick n=77 row=decision/no-blanket-approval-tests v=no
@pick n=78 row=decision/no-committed-test-secrets v=no
@pick n=79 row=decision/no-debug-string-comparison v=yes
@pick n=80 row=decision/no-flaky-test-retries v=yes
@pick n=81 row=decision/no-human-reviewed-visual-regression v=no
@pick n=82 row=decision/no-human-screenshot-review v=no
@pick n=83 row=decision/no-in-process-clock-across-processes v=yes
@pick n=84 row=decision/no-interactive-snapshot-review v=no
@pick n=85 row=decision/no-manual-smoke-step v=no
@pick n=86 row=decision/no-mocking-unowned-code v=no standing=S5
@pick n=87 row=decision/no-over-mocking-integration v=no standing=S5
@pick n=88 row=decision/no-over-stubbing v=no standing=S5
@pick n=89 row=decision/no-process-env-mutation-in-tests v=no standing=S4
@pick n=90 row=decision/no-production-data-in-tests v=yes
@pick n=91 row=decision/no-real-cli-in-ci v=yes
@pick n=92 row=decision/no-real-network-in-tests v=no
@pick n=93 row=decision/no-recorder-authored-tests v=no
@pick n=94 row=decision/no-runtime-monkey-patching v=no
@pick n=95 row=decision/no-screen-parsing v=no
@pick n=96 row=decision/no-shared-mutable-state v=no standing=S4
@pick n=97 row=decision/no-skipping-quality-gates v=no
@pick n=98 row=decision/no-sleep-synchronization v=yes
@pick n=99 row=decision/no-test-owned-real-time v=yes
@pick n=100 row=decision/no-test-passing-unstamped-decision-body v=no
