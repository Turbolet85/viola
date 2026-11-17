@pick n=1 row=contract/config-json/claude-env-keep v=no
@pick n=2 row=contract/config-json/diagnostics-level v=no
@pick n=3 row=contract/config-json/dialog-hold-s v=no
@pick n=4 row=contract/hook-stdin/not-one-object v=no
@pick n=5 row=contract/hook-stdin/oversize v=no
@pick n=6 row=contract/mcp-errors/invalid-params v=no
@pick n=7 row=contract/mcp-stdin/oversize-line v=no
@pick n=8 row=contract/mcp-stdin/parse-rejected v=no
@pick n=9 row=contract/mcp-tool-inputs/invalid-params v=no
@pick n=10 row=contract/mcp-tool-inputs/timeout-ms v=no
@pick n=11 row=contract/mcp-tool-inputs/unknown-tool v=no
@pick n=12 row=contract/revive-arguments/id v=no
@pick n=13 row=contract/revive-json/already-live v=no
@pick n=14 row=contract/revive-json/cwd-missing v=no
@pick n=15 row=contract/revive-json/error-document v=no
@pick n=16 row=contract/revive-json/internal-error v=no
@pick n=17 row=contract/revive-json/no-session v=no
@pick n=18 row=contract/revive-json/output-fields v=no
@pick n=19 row=contract/revive-json/pinned-hash-mismatch v=no
@pick n=20 row=contract/revive-json/session-live v=no
@pick n=21 row=contract/revive-json/socket-dir-refused v=no
@pick n=22 row=contract/revive-json/squatted-name v=no
@pick n=23 row=contract/revive-json/stale-heartbeat v=no
@pick n=24 row=contract/revive-json/strict-modes-failed v=no
@pick n=25 row=contract/revive-list-json/document v=no
@pick n=26 row=contract/revive-refusal/no-session v=no
@pick n=27 row=contract/send-refusal/control-character v=no
@pick n=28 row=contract/send-refusal/empty-text v=no
@pick n=29 row=decision/api-error-classes-adapter-only v=no
@pick n=30 row=decision/api-error-turn-end-text-dropped v=no
@pick n=31 row=decision/code-bearing-files-rewritten-each-start v=no
@pick n=32 row=decision/data-protection-minimal-tier v=no
@pick n=33 row=decision/diagnostics-open-fails-to-sink v=no
@pick n=34 row=decision/drift-reports-diagnostics-only v=no
@pick n=35 row=decision/event-log-growth-availability-risk v=no
@pick n=36 row=decision/event-log-growth-risk v=no
@pick n=37 row=decision/events-rotation-keeps-modes-and-offsets v=no
@pick n=38 row=decision/exec-commands-absolute-pinned-path v=no
@pick n=39 row=decision/fixture-recording-scrubbed v=yes mode=stale
@pick n=40 row=decision/fixture-scrub-and-refusal v=yes mode=stale
@pick n=41 row=decision/fixtures-reviewed-before-commit v=no
@pick n=42 row=decision/hyprland-recorder-string-allowlist v=no
@pick n=43 row=decision/hyprland-reply-text-dropped v=no
@pick n=44 row=decision/link-names-kept-out-of-process-log v=no
@pick n=45 row=decision/live-plugin-install-round v=no
@pick n=46 row=decision/live-session-screens-kept-local v=no
@pick n=47 row=decision/log-rotation-keeps-permissions v=no
@pick n=48 row=decision/logs-owner-only v=no
@pick n=49 row=decision/lost-placement-rehash v=no
@pick n=50 row=decision/marketplace-written-only-by-install v=no
@pick n=51 row=decision/mcp-fixtures-via-test-only-tap v=no
@pick n=52 row=decision/no-handle-inheritance v=no
@pick n=53 row=decision/no-keys-at-rest v=no
@pick n=54 row=decision/no-non-cryptographic-hash-fallback v=no
@pick n=55 row=decision/no-tls-local-transport v=no
@pick n=56 row=decision/no-writes-under-claude-home v=yes mode=stale
@pick n=57 row=decision/pinned-binary-rehashed v=no
@pick n=58 row=decision/pinned-copy-content-hash v=no
@pick n=59 row=decision/pinned-copy-no-clobber-placement v=no
@pick n=60 row=decision/plaintext-state-os-permissions v=no
@pick n=61 row=decision/plugin-files-rewritten-each-start v=no
@pick n=62 row=decision/plugin-install-live-round-rig v=yes mode=stale
@pick n=63 row=decision/probe-artifacts-cleanup v=yes mode=add-to fact=F1
@pick n=64 row=decision/probe-prompts-synthetic v=no
@pick n=65 row=decision/same-user-tampering-out-of-scope v=no
@pick n=66 row=decision/scratch-footer-reading-residual v=no
@pick n=67 row=decision/screen-fixture-signature-rows-only v=no
@pick n=68 row=decision/sha2-content-hash v=no
@pick n=69 row=decision/state-files-os-permissions v=no
@pick n=70 row=decision/transcript-path-redacted v=no
@pick n=71 row=decision/two-keys-per-tree-accepted v=no
@pick n=72 row=decision/verify-echo-probe-residuals v=no
@pick n=73 row=decision/verify-never-answers-import v=yes mode=contradict
@pick n=74 row=decision/verify-never-answers-import-dialog v=yes mode=contradict
@pick n=75 row=decision/verify-probe-captures-transient v=no
@pick n=76 row=decision/verify-probe-dirs-cleanup v=yes mode=add-to fact=F1
@pick n=77 row=decision/verify-probes-confined-to-own-dir v=yes mode=contradict
@pick n=78 row=decision/verify-real-cli-transcripts-accepted v=no
@pick n=79 row=decision/verify-trusted-runs-risk v=yes mode=stale
@pick n=80 row=decision/viola-format-fixtures-hygiene v=no
@pick n=81 row=decision/viola-writes-nothing-in-claude-dir v=yes mode=stale
@pick n=82 row=contract/channel-link-method/reply v=no
@pick n=83 row=contract/channel-unlink-method/reply v=no
@pick n=84 row=contract/events-link-lines/data v=no
@pick n=85 row=contract/events-link-lines/no-op-pairs v=no
@pick n=86 row=contract/fixture-refusal-codes/absolute-path v=no
@pick n=87 row=contract/fixture-refusal-codes/email v=no
@pick n=88 row=contract/fixture-refusal-codes/home-path v=no
@pick n=89 row=contract/fixture-refusal-codes/username v=no
@pick n=90 row=contract/fixture-refusal-line/form v=no
@pick n=91 row=contract/verify-refusal-codes/pinned-hash-mismatch v=no
@pick n=92 row=contract/verify-refusal-codes/program-not-found v=no
@pick n=93 row=contract/verify-refusal-codes/strict-modes-failed v=no
@pick n=94 row=contract/verify-refusal-codes/unclean-fixture v=no
@pick n=95 row=contract/verify-refusal-codes/version-unreadable v=no
@pick n=96 row=decision/rmcp-banned-features v=no
@pick n=97 row=decision/rmcp-feature-set v=no
