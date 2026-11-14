@pick n=1 row=decision/test-home-owner-record-sweep v=no standing=S2
@pick n=2 row=decision/rstest-fixture-chain v=no standing=S2
@pick n=3 row=decision/proptest-committed-seeds v=yes
@pick n=4 row=decision/owner-record-removed-last v=no
@pick n=5 row=decision/keep-failed-home-opt-in v=no
@pick n=6 row=decision/fake-agent-replay-for-stamps v=no
@pick n=7 row=decision/e2e-home-headroom-floor v=no
@pick n=8 row=decision/e2e-home-base-checks v=no
@pick n=9 row=decision/chaos-home-outside-scan v=no
@pick n=10 row=contract/status-state/ready v=no
@pick n=11 row=contract/status-state/down v=no
@pick n=12 row=contract/status-state/degraded v=no
@pick n=13 row=contract/status-document/uptime-ms v=no
@pick n=14 row=contract/status-document/pid v=no
@pick n=15 row=contract/status-document/ok v=no
@pick n=16 row=contract/status-document/list v=no
@pick n=17 row=contract/status-document/instances v=no
@pick n=18 row=contract/session-json/wrapper-pid v=no
@pick n=19 row=contract/session-json/supervisor-pid v=no
@pick n=20 row=contract/session-json/lifecycle v=no
@pick n=21 row=decision/pid-start-time-verified-kill v=yes
@pick n=22 row=decision/no-product-pid-file v=no
@pick n=23 row=contract/log-streams/event-stream v=no
@pick n=24 row=contract/log-line/timestamp v=no standing=S4
@pick n=25 row=contract/log-line/target v=no standing=S4
@pick n=26 row=contract/log-line/process v=no standing=S4
@pick n=27 row=contract/log-line/null-encoding v=no standing=S4
@pick n=28 row=contract/log-line/message v=no standing=S4
@pick n=29 row=contract/log-line/level v=no standing=S4
@pick n=30 row=contract/log-line/instance v=no standing=S4
@pick n=31 row=contract/log-line/hook-panic v=no
@pick n=32 row=contract/log-line/event v=yes
@pick n=33 row=contract/log-line/corr v=no standing=S4
@pick n=34 row=contract/log-line/conn-and-rpc-id v=no
@pick n=35 row=contract/log-format/json-per-line v=no standing=S4
@pick n=36 row=contract/log-files/modes v=no
@pick n=37 row=contract/log-event-corr/state-recovered v=no
@pick n=38 row=contract/log-event-corr/send-refused v=no
@pick n=39 row=contract/log-event-corr/send-issued v=no
@pick n=40 row=contract/log-event-corr/send-confirmed v=no
@pick n=41 row=contract/log-event-corr/release-from-driver v=no
@pick n=42 row=contract/log-event-corr/process-start v=no
@pick n=43 row=contract/log-event-corr/process-exit v=no
@pick n=44 row=contract/log-event-corr/parse-rejected v=no
@pick n=45 row=contract/log-event-corr/panic v=no
@pick n=46 row=contract/log-event-corr/hook-invoked v=no
@pick n=47 row=contract/log-event-corr/hook-decision v=no
@pick n=48 row=contract/log-event-corr/dialog-raised v=no
@pick n=49 row=contract/log-event-corr/dialog-answered v=no
@pick n=50 row=contract/log-event-corr/channel-response v=no
@pick n=51 row=contract/log-event-corr/channel-request v=no
@pick n=52 row=decision/process-logs-via-tracing-subscriber v=no standing=S4
@pick n=53 row=decision/hook-trace-write-failure-swallowed v=no
@pick n=54 row=decision/diag-line-schema-requires-corr v=yes
@pick n=55 row=decision/codes-only-role-logs v=yes
@pick n=56 row=decision/within-7s-wait-bound v=yes
@pick n=57 row=decision/viola-e2e-30s-mutants-kill v=no
@pick n=58 row=decision/verify-window-60s-kill v=no
@pick n=59 row=decision/verify-driven-tests-20s-kill v=no
@pick n=60 row=decision/syn-cfg-reader-for-host-exclusion v=no
@pick n=61 row=decision/status-shape-ownership v=no
@pick n=62 row=decision/send-window-paste-hint-30s-kill v=no
@pick n=63 row=decision/runner-provided-jq v=no
@pick n=64 row=decision/rmcp-client-only v=no
@pick n=65 row=decision/ripgrep-verified-install v=no
@pick n=66 row=decision/mutants-kill-below-cargo-mutants-timeout v=yes
@pick n=67 row=decision/mutants-fail-fast-wait v=no
@pick n=68 row=decision/harness-log-fields-owned-by-test-plan v=no
@pick n=69 row=decision/g2-zero-panics v=yes
@pick n=70 row=decision/fixture-git-no-auto-maintenance v=no
@pick n=71 row=decision/ci-tools-version-pinned v=no
@pick n=72 row=decision/ci-profile-no-retries v=no standing=S5
@pick n=73 row=decision/ci-exit-code-gates v=yes
@pick n=74 row=decision/ci-actions-sha-pinned v=no
@pick n=75 row=decision/cargo-llvm-cov-coverage v=no
@pick n=76 row=decision/agent-run-shim-over-harness v=no
@pick n=77 row=contract/supervisor-exit-file/stdout v=no
@pick n=78 row=contract/supervisor-exit-file/exits v=no
@pick n=79 row=contract/supervise-exit-codes/2 v=no
@pick n=80 row=contract/supervise-exit-codes/1 v=no
@pick n=81 row=contract/supervise-exit-codes/0 v=no
@pick n=82 row=contract/status-last-error/null v=no
@pick n=83 row=contract/status-last-error/list-exit-none v=no
@pick n=84 row=contract/status-last-error/list-exit-code v=no
@pick n=85 row=contract/secret-scan-reason/empty-scope v=no
@pick n=86 row=contract/secret-scan-output/report v=no
@pick n=87 row=contract/secret-scan-output/hits v=no standing=S3
@pick n=88 row=contract/secret-scan-hits-file/hits-json v=no
@pick n=89 row=contract/secret-scan-exit-codes/2 v=no
@pick n=90 row=contract/secret-scan-exit-codes/1 v=yes
@pick n=91 row=contract/secret-scan-class/unreadable v=no
@pick n=92 row=contract/secret-scan-class/mode v=no
@pick n=93 row=contract/secret-scan-class/content-canary v=no
@pick n=94 row=contract/secret-scan-class/claude-stripped v=no
@pick n=95 row=contract/schema-check-reason/schema-unreadable v=no
@pick n=96 row=contract/schema-check-reason/empty-scope v=no
@pick n=97 row=contract/schema-check-output/torn v=no
@pick n=98 row=contract/schema-check-output/skipped-non-file v=no
@pick n=99 row=contract/schema-check-output/failures v=no
@pick n=100 row=contract/schema-check-exit-codes/2 v=no
