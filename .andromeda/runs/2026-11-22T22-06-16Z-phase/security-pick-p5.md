@pick n=1 row=contract/events-link-lines/data v=no
@pick n=2 row=contract/events-link-lines/no-op-pairs v=no
@pick n=3 row=contract/fixture-refusal-codes/absolute-path v=no standing=S3
@pick n=4 row=contract/fixture-refusal-codes/email v=no standing=S3
@pick n=5 row=contract/fixture-refusal-codes/home-path v=no standing=S3
@pick n=6 row=contract/fixture-refusal-codes/username v=no standing=S3
@pick n=7 row=contract/fixture-refusal-line/form v=no standing=S3
@pick n=8 row=contract/verify-refusal-codes/pinned-hash-mismatch v=no
@pick n=9 row=contract/verify-refusal-codes/program-not-found v=no
@pick n=10 row=contract/verify-refusal-codes/strict-modes-failed v=no
@pick n=11 row=contract/verify-refusal-codes/unclean-fixture v=no standing=S3
@pick n=12 row=contract/verify-refusal-codes/version-unreadable v=no
@pick n=13 row=decision/rmcp-banned-features v=no
@pick n=14 row=decision/rmcp-feature-set v=no
@pick n=15 row=decision/actions-pinned-by-sha v=no
@pick n=16 row=decision/advisory-ignore-grounds v=no
@pick n=17 row=decision/advisory-ignore-list v=no
@pick n=18 row=decision/bytes-advisory-floor v=no
@pick n=19 row=decision/c-building-crates-banned v=no
@pick n=20 row=decision/cargo-audit-not-gated v=no
@pick n=21 row=decision/cargo-deny-audit v=no
@pick n=22 row=decision/checkout-no-persisted-credentials v=no
@pick n=23 row=decision/ci-cargo-deny-gate v=no
@pick n=24 row=decision/cli-tool-exact-pins v=no
@pick n=25 row=decision/crates-io-only v=no
@pick n=26 row=decision/cve-sla-disabled-feature v=no
@pick n=27 row=decision/cve-sla-enabled-feature v=no
@pick n=28 row=decision/event-values-through-env v=no
@pick n=29 row=decision/exact-dependency-pins v=no
@pick n=30 row=decision/exact-third-party-pins v=no
@pick n=31 row=decision/fuzz-lockfile-audited-separately v=no
@pick n=32 row=decision/fuzz-nightly-pin v=no
@pick n=33 row=decision/fuzz-workspace-exempt-from-root-deny v=no
@pick n=34 row=decision/fuzz-workspace-exemption v=no
@pick n=35 row=decision/g2-single-panic-exemption v=no
@pick n=36 row=decision/g2-zero-panics-single-exemption v=no
@pick n=37 row=decision/least-workflow-permissions v=no
@pick n=38 row=decision/licence-allow-list v=no
@pick n=39 row=decision/manual-dependency-updates v=no
@pick n=40 row=decision/msrv-floor v=no
@pick n=41 row=decision/no-c-building-crates v=no
@pick n=42 row=decision/no-rust-cache-in-release v=no
@pick n=43 row=decision/no-stale-license-exceptions v=no
@pick n=44 row=decision/no-tokio-in-sync-crates v=no
@pick n=45 row=decision/no-workflow-concurrency v=no
@pick n=46 row=decision/nothing-runs-as-root v=no
@pick n=47 row=decision/perf-uploads-scan-gated v=no
@pick n=48 row=decision/pre-push-checks-pins-installs-nothing v=no
@pick n=49 row=decision/product-toolchain-pin v=no
@pick n=50 row=decision/release-artifact-attestations v=no
@pick n=51 row=decision/release-binary-audit v=no
@pick n=52 row=decision/release-build-without-test-features v=no standing=S2
@pick n=53 row=decision/release-cache-poisoning-addressed v=no
@pick n=54 row=decision/rmcp-http-server-and-auth-banned v=no
@pick n=55 row=decision/rustix-advisory-floor v=no
@pick n=56 row=decision/scheduled-advisory-run v=no
@pick n=57 row=decision/scheduled-fuzzing v=no standing=S4
@pick n=58 row=decision/serde-json-bounded-depth v=no
@pick n=59 row=decision/serde-json-unbounded-depth-banned v=no
@pick n=60 row=decision/signed-self-update v=no
@pick n=61 row=decision/test-support-dev-only v=no
@pick n=62 row=decision/toolchains-via-rustup v=no
@pick n=63 row=decision/tracing-subscriber-floor v=no
@pick n=64 row=decision/zizmor-workflow-audit v=no
@pick n=65 row=decision/zizmor-workflow-gate v=no
@pick n=66 row=contract/pre-push-refusals/tool-missing-cc v=no
@pick n=67 row=contract/pre-push-refusals/tool-pin-mismatch v=no
@pick n=68 row=contract/release-check-output/test-only-feature v=no
@pick n=69 row=decision/bounded-external-readers v=yes
@pick n=70 row=decision/code-files-rewritten-at-start v=no
@pick n=71 row=decision/dependency-audit-tools v=no
@pick n=72 row=decision/diagnostics-owner-only v=yes
@pick n=73 row=decision/error-display-no-upstream-text v=yes
@pick n=74 row=decision/human-output-escapes-controls v=yes
@pick n=75 row=decision/local-boundary-no-auth-library v=no
@pick n=76 row=decision/never-log-claude-values v=yes
@pick n=77 row=decision/no-paths-in-error-output v=yes
@pick n=78 row=decision/no-repo-secret-scanner v=no
@pick n=79 row=decision/nutype-input-validation v=yes
@pick n=80 row=decision/owner-only-home-and-pinned-binary v=yes
@pick n=81 row=decision/scan-gated-ci-uploads v=no
@pick n=82 row=decision/serde-errors-mapped-to-fixed-message v=yes
@pick n=83 row=decision/user-content-only-to-diagnostics v=yes
@pick n=84 row=contract/mcp-structured-content/codes v=no
@pick n=85 row=decision/ci-artifact-canary-scan v=no
@pick n=86 row=decision/client-checks-before-frame v=yes
@pick n=87 row=decision/fuzz-artifacts-upload-unscanned v=no
@pick n=88 row=decision/no-auth-library-local-boundary v=no
@pick n=89 row=decision/no-mutants-out-upload v=no
@pick n=90 row=decision/socket-owner-only-hardening v=no
@pick n=91 row=decision/test-homes-gitignored v=no standing=S1
