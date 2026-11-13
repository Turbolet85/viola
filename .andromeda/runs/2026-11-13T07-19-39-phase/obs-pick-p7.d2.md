@pick n=1 row=contract/verify-process-exit/child-exit-status-and-duration v=no
@pick n=2 row=contract/send-refused-detail/refusal-reason-words v=no
@pick n=3 row=contract/send-refused-detail/empty-text v=no
@pick n=4 row=contract/process-exit-detail/verify-refusal v=no
@pick n=5 row=contract/process-exit-detail/server-verify-failed v=no
@pick n=6 row=contract/process-exit-detail/interrupted v=no
@pick n=7 row=contract/process-exit-detail/exit-21 v=no
@pick n=8 row=contract/process-exit-detail/exit-20 v=no
@pick n=9 row=contract/process-exit-detail/exit-1 v=no
@pick n=10 row=contract/parse-rejected/mcp-stdin-oversize v=yes
@pick n=11 row=contract/parse-rejected/claude-agents-json v=no
@pick n=12 row=contract/parse-rejected-detail/unreadable v=yes
@pick n=13 row=contract/parse-rejected-detail/unknown-keys v=no
@pick n=14 row=contract/parse-rejected-detail/symlink-ignored v=no
@pick n=15 row=contract/parse-rejected-detail/strict-modes-failed v=yes
@pick n=16 row=contract/parse-rejected-detail/panicked v=no
@pick n=17 row=contract/parse-rejected-detail/oversize v=yes
@pick n=18 row=contract/parse-rejected-detail/malformed v=no
@pick n=19 row=contract/parse-rejected-detail/control-character v=no
@pick n=20 row=contract/hook-decision-detail/statusline-codes v=yes
@pick n=21 row=contract/hook-decision-detail/server-verify-failed v=yes
@pick n=22 row=contract/hook-decision-detail/fail-open-codes v=yes
@pick n=23 row=contract/hook-decision-detail/channel-unreachable v=yes
@pick n=24 row=contract/diag-subject/version-probe v=no
@pick n=25 row=contract/diag-subject/verify-pty-probe v=no
@pick n=26 row=contract/diag-subject/verify-probe v=no
@pick n=27 row=contract/diag-subject/statusline-shell v=no
@pick n=28 row=contract/diag-subject/self v=no
@pick n=29 row=contract/diag-subject/claude-child v=no
@pick n=30 row=contract/diag-subject/agents-probe v=no
@pick n=31 row=contract/diag-line-fields/state-recovered v=yes
@pick n=32 row=contract/diag-line-fields/send-events v=no
@pick n=33 row=contract/diag-line-fields/release-from-driver v=no
@pick n=34 row=contract/diag-line-fields/process-start v=no
@pick n=35 row=contract/diag-line-fields/process-exit v=no
@pick n=36 row=contract/diag-line-fields/parse-rejected v=yes
@pick n=37 row=contract/diag-line-fields/panic v=no
@pick n=38 row=contract/diag-line-fields/hook-events v=yes
@pick n=39 row=contract/diag-line-fields/dialog-events v=no
@pick n=40 row=contract/diag-line-fields/channel-response v=no
@pick n=41 row=contract/diag-line-fields/channel-request v=no
@pick n=42 row=contract/diag-detail/strict-modes-failed v=yes
@pick n=43 row=contract/diag-detail/socket-dir-refused v=yes
@pick n=44 row=contract/diag-detail/pinned-hash-mismatch v=no
@pick n=45 row=decision/unemittable-schema-words-removed v=yes
@pick n=46 row=decision/symlink-skip-logged-as-count v=no
@pick n=47 row=decision/statusline-command-deadline v=yes
@pick n=48 row=decision/session-live-check-fails-open v=no
@pick n=49 row=decision/revive-preflight-order v=no
@pick n=50 row=contract/pty-seam-log/spawn v=no
@pick n=51 row=contract/pty-seam-log/paste v=no
@pick n=52 row=contract/pty-seam-log/kill-fallback v=no
@pick n=53 row=contract/process-start-line/service-name v=no
@pick n=54 row=contract/log-line/timestamp v=no standing=S2
@pick n=55 row=contract/log-line/target v=no standing=S2
@pick n=56 row=contract/log-line/message v=no standing=S2
@pick n=57 row=contract/log-line/level v=no standing=S2
@pick n=58 row=contract/log-line/instance v=no standing=S2
@pick n=59 row=contract/log-line/event v=no standing=S2
@pick n=60 row=contract/log-line/corr v=no standing=S2
@pick n=61 row=contract/log-levels/warn v=yes
@pick n=62 row=contract/log-levels/trace v=no
@pick n=63 row=contract/log-levels/info v=no
@pick n=64 row=contract/log-levels/error v=no
@pick n=65 row=contract/log-levels/debug v=no
@pick n=66 row=contract/ipc-invocation-log/fields v=no
@pick n=67 row=contract/child-spawn-log/subject v=no
@pick n=68 row=contract/child-spawn-log/claude-agents-json-parse-failure v=no
@pick n=69 row=decision/viola-crates-log-at-info v=no
@pick n=70 row=decision/unlisted-targets-log-off v=no
@pick n=71 row=decision/third-party-crate-logs-off v=no
@pick n=72 row=decision/skipped-symlink-never-logged v=no
@pick n=73 row=decision/no-event-type-field v=no standing=S2
@pick n=74 row=decision/log-schema-additive-only v=no
@pick n=75 row=decision/json-to-file-only-log-mode v=no
@pick n=76 row=decision/ipc-calls-logged-without-arguments v=no
@pick n=77 row=decision/file-only-per-role-log-sinks v=yes
@pick n=78 row=decision/euid-refused-peer-writes-no-line v=no
@pick n=79 row=decision/debug-level-by-config-only v=no
@pick n=80 row=decision/content-detail-in-instance-file v=no
@pick n=81 row=decision/additive-field-naming v=no
