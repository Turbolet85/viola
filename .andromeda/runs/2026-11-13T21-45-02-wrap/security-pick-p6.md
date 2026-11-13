@pick n=1 row=decision/rmcp-advisory-ignores v=no
@pick n=2 row=decision/rmcp-http-server-and-auth-banned v=no
@pick n=3 row=decision/rustix-advisory-floor v=no
@pick n=4 row=decision/scheduled-advisory-run v=no
@pick n=5 row=decision/scheduled-fuzzing v=no
@pick n=6 row=decision/serde-json-bounded-depth v=no
@pick n=7 row=decision/serde-json-unbounded-depth-banned v=no
@pick n=8 row=decision/signed-self-update v=no
@pick n=9 row=decision/test-support-dev-only v=no
@pick n=10 row=decision/toolchains-via-rustup v=no
@pick n=11 row=decision/tracing-subscriber-floor v=no
@pick n=12 row=decision/zizmor-workflow-audit v=no
@pick n=13 row=decision/zizmor-workflow-gate v=no
@pick n=14 row=contract/pre-push-refusals/tool-missing-cc v=no
@pick n=15 row=contract/pre-push-refusals/tool-pin-mismatch v=no
@pick n=16 row=contract/release-check-output/test-only-feature v=no
@pick n=17 row=decision/bounded-external-readers v=no
@pick n=18 row=decision/code-files-rewritten-at-start v=no
@pick n=19 row=decision/dependency-audit-tools v=no
@pick n=20 row=decision/diagnostics-owner-only v=yes mode=stale
@pick n=21 row=decision/error-display-no-upstream-text v=no
@pick n=22 row=decision/human-output-escapes-controls v=no
@pick n=23 row=decision/local-boundary-no-auth-library v=no
@pick n=24 row=decision/never-log-claude-values v=no
@pick n=25 row=decision/no-paths-in-error-output v=no
@pick n=26 row=decision/no-repo-secret-scanner v=no
@pick n=27 row=decision/nutype-input-validation v=no
@pick n=28 row=decision/owner-only-home-and-pinned-binary v=yes mode=stale
@pick n=29 row=decision/scan-gated-ci-uploads v=no
@pick n=30 row=decision/serde-errors-mapped-to-fixed-message v=no
@pick n=31 row=decision/user-content-only-to-diagnostics v=yes mode=stale
@pick n=32 row=contract/mcp-structured-content/codes v=no
@pick n=33 row=decision/ci-artifact-canary-scan v=no
@pick n=34 row=decision/client-checks-before-frame v=yes mode=stale
@pick n=35 row=decision/fuzz-artifacts-upload-unscanned v=no
@pick n=36 row=decision/no-auth-library-local-boundary v=no
@pick n=37 row=decision/no-mutants-out-upload v=no
@pick n=38 row=decision/socket-owner-only-hardening v=no
@pick n=39 row=decision/test-homes-gitignored v=no
@pick n=40 row=decision/artifact-canary-scan v=no
@pick n=41 row=decision/canary-scan-never-echoes-matches v=no
@pick n=42 row=decision/claude-credentials-stripped-from-child v=no
@pick n=43 row=decision/credential-strip-by-name-only v=no
@pick n=44 row=decision/env-not-config-channel v=no
@pick n=45 row=decision/hermetic-child-environment v=no
@pick n=46 row=decision/identity-floor-eleven-names v=no
@pick n=47 row=decision/inherited-credentials-never-serialised v=no
@pick n=48 row=decision/messaging-credentials-are-secret v=no
@pick n=49 row=decision/no-credential-literals v=no
@pick n=50 row=decision/no-stored-secrets v=no
@pick n=51 row=decision/no-uid-0-launch v=no
@pick n=52 row=decision/pre-push-empty-environment-launch v=no
@pick n=53 row=decision/pre-push-linux-only v=no
@pick n=54 row=decision/r8-claude-env-strip v=no
@pick n=55 row=decision/tests-use-own-home v=no
@pick n=56 row=contract/artifact-canary-scan-classes/claude-stripped v=no
@pick n=57 row=contract/artifact-canary-scan-classes/content-canary v=no
@pick n=58 row=contract/artifact-canary-scan-classes/diagnostics-mode v=no
@pick n=59 row=contract/artifact-canary-scan-classes/unreadable v=no
@pick n=60 row=contract/claude-child-process-start/env-kept v=no
@pick n=61 row=contract/claude-child-process-start/env-stripped-known v=no
@pick n=62 row=decision/channel-faults-sanitised v=no
@pick n=63 row=decision/cli-frame-verbs-strict-modes-first v=yes mode=stale
@pick n=64 row=decision/diagnostics-strict-modes-fail-open v=yes mode=stale
@pick n=65 row=decision/error-enum-convention-exceptions v=no
@pick n=66 row=decision/error-messages-interpolate-nothing v=no
@pick n=67 row=decision/every-error-writer-named v=no
@pick n=68 row=decision/fixed-error-display-messages v=no
@pick n=69 row=decision/full-error-detail-to-diagnostics v=no
@pick n=70 row=decision/home-tree-strict-modes-before-log v=yes mode=stale
@pick n=71 row=decision/hook-refusals-write-no-frame v=yes mode=stale
@pick n=72 row=decision/hook-silent-exit-zero v=no
@pick n=73 row=decision/hook-strict-modes-first v=yes mode=stale
@pick n=74 row=decision/internal-error-held-at-unit-layer v=no
@pick n=75 row=decision/logging-bans-before-reporting v=no
@pick n=76 row=decision/mcp-frame-tool-refusals v=yes mode=stale
@pick n=77 row=decision/refused-peer-dropped-before-frame v=no
@pick n=78 row=decision/reply-code-words-validated v=no
@pick n=79 row=decision/run-refuses-socket-dir-before-start v=yes mode=stale
@pick n=80 row=decision/sanitised-external-errors v=no
@pick n=81 row=decision/socket-dir-refusal-names-only-name v=no
@pick n=82 row=decision/stderr-never-log-floor v=no
@pick n=83 row=decision/unusable-response-line-on-stderr v=no
@pick n=84 row=contract/channel-errors/-32600 v=no
@pick n=85 row=contract/channel-errors/-32602 v=no
@pick n=86 row=contract/channel-errors/-32603 v=no
@pick n=87 row=contract/channel-fault/-32603 v=no
@pick n=88 row=contract/channel-fault/unknown-code v=no
@pick n=89 row=contract/channel-refusal/control-character v=yes mode=stale
@pick n=90 row=contract/channel-refusal/empty-text v=yes mode=stale
@pick n=91 row=contract/channel-response-detail/invalid-params v=no
@pick n=92 row=contract/channel-response-detail/release-from-driver v=no
@pick n=93 row=contract/channel-response-detail/unsupported-version v=no
@pick n=94 row=contract/cli-instance-unreachable/json-document v=no
@pick n=95 row=contract/cli-json-error/internal-error v=no
@pick n=96 row=contract/cli-stderr-hints/unwrapped-name-endpoint-lost v=yes mode=stale
@pick n=97 row=contract/collision-check/already-live v=no
@pick n=98 row=contract/collision-check/stale-heartbeat v=no
