@pick n=1 row=contract/verify-exit-detail/version-unreadable v=no
@pick n=2 row=contract/verify-exit-detail/unclean-fixture v=no
@pick n=3 row=contract/verify-exit-detail/strict-modes-failed v=no
@pick n=4 row=contract/verify-exit-detail/program-not-found v=no
@pick n=5 row=contract/verify-exit-detail/pinned-hash-mismatch v=no
@pick n=6 row=contract/verify-exit-detail/interrupted v=no
@pick n=7 row=contract/unclean-fixture-refusal/username v=no
@pick n=8 row=contract/unclean-fixture-refusal/message v=no
@pick n=9 row=contract/unclean-fixture-refusal/home-path v=no
@pick n=10 row=contract/unclean-fixture-refusal/email v=no
@pick n=11 row=contract/unclean-fixture-refusal/absolute-path v=no
@pick n=12 row=contract/session-id/shape v=yes
@pick n=13 row=contract/profraw-census-exit-codes/2 v=no
@pick n=14 row=contract/profraw-census-exit-codes/1 v=no
@pick n=15 row=contract/profraw-census-exit-codes/0 v=no
@pick n=16 row=contract/process-start-claude-child/env-stripped-known v=no
@pick n=17 row=contract/process-start-claude-child/env-kept v=no
@pick n=18 row=contract/process-exit-detail/window-not-found v=no
@pick n=19 row=contract/process-exit-detail/untrusted-folder v=no
@pick n=20 row=contract/process-exit-detail/unsupported v=yes
@pick n=21 row=contract/process-exit-detail/unreachable v=yes
@pick n=22 row=contract/process-exit-detail/terminal-not-found v=yes
@pick n=23 row=contract/process-exit-detail/stale-heartbeat v=yes
@pick n=24 row=contract/process-exit-detail/refused v=no
@pick n=25 row=contract/process-exit-detail/oversize v=no
@pick n=26 row=contract/process-exit-detail/not-live v=yes
@pick n=27 row=contract/process-exit-detail/malformed v=no
@pick n=28 row=contract/process-exit-detail/boot-time-unread v=yes
@pick n=29 row=contract/instance-unreachable-detail/strict-modes-failed v=no
@pick n=30 row=contract/instance-unreachable-detail/socket-dir-refused v=no
@pick n=31 row=contract/instance-unreachable-detail/server-verify-failed v=no
@pick n=32 row=contract/human-output-colour-env/term v=no
@pick n=33 row=contract/human-output-colour-env/no-color v=no
@pick n=34 row=contract/human-output-colour-env/colorterm v=no
@pick n=35 row=contract/hook-decision-detail/socket-dir-refused v=no
@pick n=36 row=contract/hook-capture-arm/payload-file v=no
@pick n=37 row=contract/hook-capture-arm/names-only v=no
@pick n=38 row=contract/hook-capture-arm/exit v=no
@pick n=39 row=contract/hook-capture-arm/env-names-file v=no
@pick n=40 row=contract/harness-environment/agent-run-keep-homes v=no standing=S2
@pick n=41 row=contract/harness-environment/agent-run-keep-failed v=no standing=S2
@pick n=42 row=contract/harness-environment/agent-run-chunk-base v=no
@pick n=43 row=contract/frame-verb-exit-detail/wrapper-fault v=no
@pick n=44 row=contract/frame-verb-exit-detail/unwrapped-name v=no
@pick n=45 row=contract/frame-verb-exit-detail/socket-dir-refused v=no
@pick n=46 row=contract/frame-verb-exit-detail/server-verify-failed v=no
@pick n=47 row=contract/frame-verb-exit-detail/instance-dead v=no
@pick n=48 row=contract/frame-verb-exit-detail/endpoint-lost v=no
@pick n=49 row=contract/frame-verb-exit-codes/21 v=no
@pick n=50 row=contract/events-ndjson/byte-offsets v=yes
@pick n=51 row=contract/e2e-home-refusal/headroom-message v=no standing=S2
@pick n=52 row=contract/diagnostics-role-files/run-name v=yes
@pick n=53 row=contract/diagnostics-role-files/mcp v=no
@pick n=54 row=contract/diagnostics-role-files/hook-name v=no
@pick n=55 row=contract/diagnostics-role-files/cli-name v=yes
@pick n=56 row=contract/diagnostics-lines/process-log-line v=yes
@pick n=57 row=contract/diagnostics-lines/detail-line v=no
@pick n=58 row=contract/diag-line/state-recovered v=yes
@pick n=59 row=contract/diag-line/format v=no
@pick n=60 row=contract/config-json/verify-new-cli v=no
@pick n=61 row=contract/config-json/v v=no
@pick n=62 row=contract/config-json/dialog-hold-s v=no
@pick n=63 row=contract/config-json/diagnostics-level v=no
@pick n=64 row=contract/config-json/default-model v=no
@pick n=65 row=contract/config-json/default-effort v=no
@pick n=66 row=contract/config-json/claude-env-keep v=yes
@pick n=67 row=contract/claude-child-argv/resume v=yes
@pick n=68 row=contract/claude-child-argv/plugin-dir v=no
@pick n=69 row=contract/claude-child-argv/fork-session v=yes
@pick n=70 row=contract/child-environment/viola-name v=no
@pick n=71 row=contract/child-environment/viola-dir v=no
@pick n=72 row=contract/child-environment/viola-bin v=yes
@pick n=73 row=contract/child-environment/path v=yes
@pick n=74 row=contract/child-argv/resume v=yes
@pick n=75 row=contract/child-argv/plugin-dir v=no
@pick n=76 row=contract/child-argv/launch-pair v=yes
@pick n=77 row=contract/child-argv/fork-session v=yes
@pick n=78 row=contract/child-argv/caller-arguments v=no
@pick n=79 row=contract/channel-response-detail/unsupported-version v=no
@pick n=80 row=contract/channel-response-detail/release-from-driver v=no
@pick n=81 row=contract/channel-response-detail/invalid-params v=no
@pick n=82 row=decision/wait-report-kept-on-failure v=no standing=S2
@pick n=83 row=decision/viola-env-namespace-product-only v=yes
@pick n=84 row=decision/viola-channel-tokio-free-by-default v=no
@pick n=85 row=decision/version-gate-strict-stamps-read v=no
@pick n=86 row=decision/verify-sole-stamps-writer v=no
@pick n=87 row=decision/verify-run-e-resumes-run-b-first-session v=no
@pick n=88 row=decision/verify-removes-only-own-dirs v=no
@pick n=89 row=decision/verify-probe-dirs-drop-guarded v=no
@pick n=90 row=decision/test-support-never-in-product v=no
@pick n=91 row=decision/test-sockets-under-short-tmp-root v=no standing=S2
@pick n=92 row=decision/test-side-removal-outside-workdir v=no standing=S2
@pick n=93 row=decision/test-seams-not-configuration v=yes
@pick n=94 row=decision/test-homes-cleaned-by-owner v=no standing=S2
@pick n=95 row=decision/test-home-owner-sweep v=no standing=S2
@pick n=96 row=decision/test-evidence-survives-kill v=no standing=S2
