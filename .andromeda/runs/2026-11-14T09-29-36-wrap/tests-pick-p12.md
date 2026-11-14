@pick n=1 row=decision/no-debug-string-comparison v=no
@pick n=2 row=decision/no-flaky-test-retries v=no
@pick n=3 row=decision/no-human-reviewed-visual-regression v=no
@pick n=4 row=decision/no-human-screenshot-review v=no
@pick n=5 row=decision/no-in-process-clock-across-processes v=yes mode=stale
@pick n=6 row=decision/no-interactive-snapshot-review v=no
@pick n=7 row=decision/no-manual-smoke-step v=no
@pick n=8 row=decision/no-mocking-unowned-code v=no
@pick n=9 row=decision/no-over-mocking-integration v=no
@pick n=10 row=decision/no-over-stubbing v=no
@pick n=11 row=decision/no-process-env-mutation-in-tests v=yes mode=stale
@pick n=12 row=decision/no-production-data-in-tests v=yes mode=contradict
@pick n=13 row=decision/no-real-cli-in-ci v=no
@pick n=14 row=decision/no-real-network-in-tests v=no
@pick n=15 row=decision/no-recorder-authored-tests v=no
@pick n=16 row=decision/no-runtime-monkey-patching v=no
@pick n=17 row=decision/no-screen-parsing v=no
@pick n=18 row=decision/no-shared-mutable-state v=no
@pick n=19 row=decision/no-skipping-quality-gates v=no
@pick n=20 row=decision/no-sleep-synchronization v=yes mode=stale
@pick n=21 row=decision/no-test-owned-real-time v=yes mode=stale
@pick n=22 row=decision/no-test-passing-unstamped-decision-body v=no
@pick n=23 row=decision/no-tokio-failpoint-crate v=no
@pick n=24 row=decision/proptest-seeds-committed v=yes mode=stale
@pick n=25 row=decision/real-hook-in-e2e v=no
@pick n=26 row=decision/real-on-disk-state v=no
@pick n=27 row=decision/rust-cache-only-in-ci v=no
@pick n=28 row=decision/seeded-generators v=yes mode=stale
@pick n=29 row=decision/state-files-never-hand-written v=no
@pick n=30 row=decision/test-public-behaviour-only v=no
@pick n=31 row=decision/tests-run-in-parallel v=no
@pick n=32 row=decision/wrapper-spawned-with-guard v=no
@pick n=33 row=decision/boot-readiness-file-probe v=no
@pick n=34 row=decision/canary-list-single-sourced v=yes mode=stale
@pick n=35 row=decision/chunk-diff-pins-prefixes v=no
@pick n=36 row=decision/ci-gate-last-step v=no
@pick n=37 row=decision/ci-keeps-test-homes v=no
@pick n=38 row=decision/ci-uploads-only-after-secret-scan v=no
@pick n=39 row=decision/cleanup-ctrl-c-repeated-until-exit v=no
@pick n=40 row=decision/cleanup-is-idempotent v=no
@pick n=41 row=decision/closed-enums-need-decisions-log v=yes mode=stale
@pick n=42 row=decision/coverage-floors v=no
@pick n=43 row=decision/dev-host-e2e-home-on-tmpfs v=no
@pick n=44 row=decision/diagnostics-schema-validation v=yes mode=stale
@pick n=45 row=decision/doc-examples-never-ignore v=no
@pick n=46 row=decision/e2e-boot-model-by-cleanup v=no
@pick n=47 row=decision/e2e-home-deletion-through-link v=no
@pick n=48 row=decision/e2e-home-link-base v=no
@pick n=49 row=decision/e2e-homes-under-one-root v=no
@pick n=50 row=decision/e2e-homes-under-target-e2e-home v=no
@pick n=51 row=decision/fixture-chain-exists-once v=yes mode=stale
@pick n=52 row=decision/fuzz-corpus-never-vacuous v=yes mode=stale
@pick n=53 row=decision/fuzz-replay-linux-only v=no
@pick n=54 row=decision/fuzz-reproducers-committed v=yes mode=stale
@pick n=55 row=decision/fuzz-toolchain-single-source v=no
@pick n=56 row=decision/harness-builds-in-its-own-target v=no
@pick n=57 row=decision/harness-never-reads-cargo-bin-exe v=no
@pick n=58 row=decision/harness-scoped-path v=no
@pick n=59 row=decision/host-excluded-mutants v=no
@pick n=60 row=decision/internal-harness-subcommands-off-agent-surface v=no
@pick n=61 row=decision/junit-copied-after-each-invocation v=no
@pick n=62 row=decision/killed-wrapper-endpoint-wait v=no
@pick n=63 row=decision/live-verify-never-in-ci v=yes mode=stale
@pick n=64 row=decision/local-live-runner-seam v=no
@pick n=65 row=decision/log-assertions-via-jq-or-jaq v=no
@pick n=66 row=decision/members-declare-fake-agent-feature v=yes mode=stale
@pick n=67 row=decision/missing-artifact-is-breach v=no
@pick n=68 row=decision/mutant-build-timeout v=no
@pick n=69 row=decision/mutants-output-live v=no
@pick n=70 row=decision/mutants-survived-count v=no
@pick n=71 row=decision/mutants-tested-by-owning-package v=yes mode=stale
@pick n=72 row=decision/mutation-base-whole-chunk v=no
@pick n=73 row=decision/mutation-diff-classification v=no
@pick n=74 row=decision/mutation-keeps-no-test-home v=no
@pick n=75 row=decision/mutation-own-target-dir v=no
@pick n=76 row=decision/mutation-verdict-by-counts v=no
@pick n=77 row=decision/nightly-fuzz-fails-on-crash v=no
@pick n=78 row=decision/no-ci-mutants-gate v=no
@pick n=79 row=decision/no-nested-harness-in-nextest v=no
@pick n=80 row=decision/panic-counts-read-role-files-only v=no
@pick n=81 row=decision/perf-verdict-belongs-to-gate v=no
@pick n=82 row=decision/pre-push-clean-environment v=no
@pick n=83 row=decision/pre-push-local-linux-gate v=no
@pick n=84 row=decision/pre-push-runs-no-mutation v=no
@pick n=85 row=decision/root-wait-bound v=yes mode=stale
@pick n=86 row=decision/scoped-mutants-not-the-verdict v=no
@pick n=87 row=decision/secret-scan-scope v=no
@pick n=88 row=decision/session-home-retention v=no
@pick n=89 row=decision/skipped-counts-real-skips-only v=no
@pick n=90 row=decision/status-bounded-read v=no
@pick n=91 row=decision/status-strips-bridge-env v=no
@pick n=92 row=decision/supervisor-ordinary-child v=no
@pick n=93 row=decision/supervisor-spawned-only-by-boot v=no
@pick n=94 row=decision/test-layer-filtersets v=no
@pick n=95 row=decision/verify-sole-stamp-writer v=no
@pick n=96 row=decision/workspace-root-absolute v=no
@pick n=97 row=contract/agent-run-logs-stream/detail-line v=no
@pick n=98 row=contract/agent-run-logs-stream/diag-line v=no
@pick n=99 row=contract/agent-run-logs-stream/events-line v=no
@pick n=100 row=contract/agent-run-logs-stream/torn-line v=no
