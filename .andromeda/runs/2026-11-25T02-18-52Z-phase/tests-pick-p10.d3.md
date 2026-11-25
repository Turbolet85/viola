@pick n=1 row=contract/fake-foot/spawn-record v=yes
@pick n=2 row=decision/no-http-mocking v=no standing=S4
@pick n=3 row=contract/ci-artifacts/secret-scan v=no
@pick n=4 row=decision/deny-bans-probed v=no standing=S6
@pick n=5 row=decision/fakes-for-external-services v=yes
@pick n=6 row=decision/jobs-end-in-harness-gate v=no standing=S6
@pick n=7 row=decision/perf-verdict-from-harness v=no standing=S6
@pick n=8 row=decision/build-fails-on-test-failure v=no standing=S6
@pick n=9 row=decision/msrv-check v=no standing=S6
@pick n=10 row=contract/ci-artifacts/diag v=no
@pick n=11 row=decision/linux-only-ci-matrix v=no
@pick n=12 row=decision/channel-tokio-free-by-default v=no
@pick n=13 row=decision/per-suite-junit-copies v=no
@pick n=14 row=decision/supply-chain-fail-closed v=no standing=S6
@pick n=15 row=decision/mutation-testing-off-ci v=yes
@pick n=16 row=decision/test-clocks-behind-clock-seam v=yes
@pick n=17 row=contract/process-exit-detail/unclean-fixture v=no
@pick n=18 row=decision/ci-lint-gate v=no standing=S6
@pick n=19 row=contract/fake-agent-receipt/start v=no
@pick n=20 row=decision/build-fails-below-coverage v=no standing=S6
@pick n=21 row=decision/workflow-least-permissions v=no
@pick n=22 row=decision/peer-euid-admission v=no
@pick n=23 row=decision/filesystem-not-mocked v=yes
@pick n=24 row=decision/module-acyclicity-not-gated v=no
@pick n=25 row=decision/least-workflow-permissions v=no
@pick n=26 row=contract/ci-artifacts/harness v=no
@pick n=27 row=decision/zizmor-gate v=no standing=S6
@pick n=28 row=contract/mcp-fixture-scrub-refusal/username v=yes
@pick n=29 row=decision/no-ci-concurrency-groups v=no
@pick n=30 row=decision/stamps-only-from-verify v=yes
@pick n=31 row=decision/peer-admission-by-injection v=no
@pick n=32 row=decision/fuzz-corpus-replay-on-push v=no
@pick n=33 row=decision/lint-fmt-and-clippy-deny-warnings v=no standing=S6
@pick n=34 row=decision/machine-parseable-ci-reports v=no
@pick n=35 row=decision/rust-cache-only-cache v=no
@pick n=36 row=decision/nightly-fuzzing v=no
@pick n=37 row=contract/ci-artifacts/junit v=no
@pick n=38 row=contract/mcp-fixture-scrub-refusal/email v=no
@pick n=39 row=decision/ci-single-ubuntu-leg v=no
@pick n=40 row=contract/claude-fixtures/payload-path v=no
@pick n=41 row=contract/ci-artifacts/perf v=no
@pick n=42 row=decision/single-tool-version-source v=no
@pick n=43 row=contract/fake-agent-receipt/line-format v=no
@pick n=44 row=contract/fake-agent-receipt/prompt v=no
@pick n=45 row=decision/harness-gate-fails-on-breach v=no standing=S6
@pick n=46 row=decision/actions-sha-pinned v=no
@pick n=47 row=decision/fmt-and-clippy-deny-warnings v=no standing=S6
@pick n=48 row=decision/impostor-server-real-listener v=no
@pick n=49 row=contract/fake-agent-receipt/key v=no
@pick n=50 row=decision/cargo-deny-gate v=no standing=S6
@pick n=51 row=decision/build-fails-on-fuzz-crash v=no
@pick n=52 row=decision/utc-now-read-directly v=yes
@pick n=53 row=decision/wrapper-deadline-injected-clock v=no
@pick n=54 row=decision/forced-window-hook-is-test-parameter v=no
@pick n=55 row=decision/no-mocks-in-e2e v=yes
@pick n=56 row=contract/ci-artifacts/diag-perf v=no
@pick n=57 row=decision/no-orphan-modules v=no standing=S6
@pick n=58 row=decision/no-http-fake-servers v=no
@pick n=59 row=decision/pinned-rust-toolchain v=no
@pick n=60 row=decision/ci-artifact-retention v=no
@pick n=61 row=contract/mcp-fixture-scrub-refusal/home-path v=yes
@pick n=62 row=contract/mcp-wait-tool/timeout-bound v=no
@pick n=63 row=decision/impostor-holder-real-process v=no
@pick n=64 row=decision/wrapper-boots-pass-trusted-screens v=yes
@pick n=65 row=decision/agent-reads-ci-through-gh v=no
@pick n=66 row=decision/quality-gate-command v=no standing=S6
@pick n=67 row=decision/mutation-not-a-ci-gate v=yes
@pick n=68 row=decision/local-pre-push-gate-installs-nothing v=no
@pick n=69 row=decision/hyperfine-measures-perf v=no
@pick n=70 row=decision/ci-jobs-end-in-harness-gate v=no standing=S6
@pick n=71 row=decision/time-faked-through-seams v=yes
@pick n=72 row=decision/build-fails-on-perf-breach v=no
@pick n=73 row=decision/jq-runner-provided v=no
@pick n=74 row=decision/weekly-advisories-run v=no
@pick n=75 row=decision/msrv-1-96 v=no standing=S6
@pick n=76 row=decision/ci-stamped-home-kill-line v=no standing=S1
@pick n=77 row=decision/artifacts-behind-secret-scan v=no
@pick n=78 row=decision/no-cache-in-release-workflow v=no
@pick n=79 row=contract/fake-agent-receipt/hold v=no
@pick n=80 row=decision/real-process-faults v=yes
@pick n=81 row=contract/fake-agent-hooks/session-start-env v=no
@pick n=82 row=decision/fuzz-corpus-replay-in-ci v=no
@pick n=83 row=decision/external-processes-real-stubs v=yes
@pick n=84 row=decision/github-actions-ci v=no
@pick n=85 row=contract/fake-agent-receipt/cwd v=no
@pick n=86 row=decision/pure-functions-tested-directly v=yes
@pick n=87 row=contract/fake-agent-receipt/hook v=no
@pick n=88 row=decision/test-endpoint-dirs-made-private v=yes
@pick n=89 row=decision/mutation-output-never-uploaded v=no
@pick n=90 row=contract/mcp-fixture-scrub-refusal/absolute-path v=no
@pick n=91 row=contract/fake-agent-receipt/fds v=no
@pick n=92 row=contract/fake-agent-receipt/size v=no
@pick n=93 row=decision/single-instrumented-test-run v=no standing=S6
@pick n=94 row=decision/cargo-deny-supply-chain-gate v=no standing=S6
@pick n=95 row=decision/real-filesystem-faults v=yes
@pick n=96 row=decision/mockall-on-seam-traits-only v=yes
@pick n=97 row=decision/rmcp-feature-limit v=no
@pick n=98 row=contract/fake-agent-receipt/step v=no
@pick n=99 row=decision/hook-unit-impostor-varies-start-time v=no
@pick n=100 row=contract/ci-artifacts/secret-scan-perf v=no
