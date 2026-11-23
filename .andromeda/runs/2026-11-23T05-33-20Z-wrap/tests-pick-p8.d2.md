@pick n=1 row=contract/viola-send/unconfirmable-local-command v=no
@pick n=2 row=contract/viola-send/turn-running v=no
@pick n=3 row=contract/viola-send/trailing-newlines v=no
@pick n=4 row=contract/viola-send/records v=no
@pick n=5 row=contract/viola-send/paste-framing v=no
@pick n=6 row=contract/viola-send/ok-document v=no
@pick n=7 row=contract/viola-send/off-list-slash-text v=no
@pick n=8 row=contract/viola-send/no-prompt-submitted v=no
@pick n=9 row=contract/viola-send/manual-pause v=no
@pick n=10 row=contract/viola-send/inner-cr v=no
@pick n=11 row=contract/viola-send/human-typing v=no
@pick n=12 row=contract/viola-send/human-key-during-gate v=no
@pick n=13 row=contract/viola-send/compact-confirmed v=no
@pick n=14 row=contract/viola-send/clear-confirmed v=no
@pick n=15 row=contract/viola-send-human-mode/unconfirmable v=no
@pick n=16 row=contract/viola-send-human-mode/unable v=no
@pick n=17 row=contract/viola-send-human-mode/read-back v=no
@pick n=18 row=contract/viola-run/child-on-wrapper-kill v=no
@pick n=19 row=contract/viola-run-start/tampered-pinned-exe v=no
@pick n=20 row=contract/viola-run-start/squatted-endpoint v=no
@pick n=21 row=contract/viola-run-start/socket-dir-check v=yes mode=add-to fact=F2
@pick n=22 row=contract/viola-run-start/plugin-hooks v=no
@pick n=23 row=contract/viola-run-start/home-tree-check v=no
@pick n=24 row=contract/viola-run-start/duplicate-run v=no
@pick n=25 row=contract/viola-run-start/child-after-events v=yes mode=stale
@pick n=26 row=contract/viola-run-passthrough/ctrl-z v=no
@pick n=27 row=contract/viola-release/running-turn v=no
@pick n=28 row=contract/viola-release/from-driver v=no
@pick n=29 row=contract/viola-release/document v=no
@pick n=30 row=contract/viola-list-json/live-instance v=no
@pick n=31 row=contract/viola-last/document v=no
@pick n=32 row=contract/viola-answer/human-wheel v=no
@pick n=33 row=contract/viola-answer/allow-suggestion-fields v=no
@pick n=34 row=contract/verify-output/last-line v=no
@pick n=35 row=contract/turn-ended/scheduled-wakes v=no
@pick n=36 row=contract/turn-ended/background-tasks v=no
@pick n=37 row=contract/start-exit-1/strict-modes-failed v=no
@pick n=38 row=contract/start-exit-1/stale-heartbeat v=no
@pick n=39 row=contract/start-exit-1/squatted-name v=no
@pick n=40 row=contract/start-exit-1/socket-dir-refused v=no
@pick n=41 row=contract/start-exit-1/pinned-hash-mismatch v=no
@pick n=42 row=contract/start-exit-1/already-live v=no
@pick n=43 row=contract/revive-json/refusal-document v=yes mode=stale
@pick n=44 row=contract/revive-exit-1/session-live v=yes mode=stale
@pick n=45 row=contract/revive-exit-1/no-session v=yes mode=stale
@pick n=46 row=contract/revive-exit-1/cwd-missing v=no
@pick n=47 row=contract/release-verb/driver-caller v=no
@pick n=48 row=contract/question-event/option-details v=no
@pick n=49 row=contract/process-start-claude-child/env-stripped-known v=no
@pick n=50 row=contract/process-start-claude-child/env-stripped-count v=no
@pick n=51 row=contract/process-start-claude-child/env-kept v=no
@pick n=52 row=contract/mcp-wait/timeout-ms v=no
@pick n=53 row=contract/mcp-tools/result-body v=no
@pick n=54 row=contract/mcp-tools/instance-unreachable v=no
@pick n=55 row=contract/mcp-tool-result/unverified-cli v=no
@pick n=56 row=contract/mcp-tool-input/schema-violation v=no
@pick n=57 row=contract/mcp-tool-input/empty-or-control-text v=no
@pick n=58 row=contract/mcp-send/human-typing v=no
@pick n=59 row=contract/mcp-frame/socket-dir-refused v=no
@pick n=60 row=contract/list-json/skipped-counts v=no
@pick n=61 row=contract/list-json/liveness-stale v=no
@pick n=62 row=contract/list-human/unwrapped-separator v=yes mode=add-to fact=F7
@pick n=63 row=contract/list-human/columns v=yes mode=stale
@pick n=64 row=contract/instance-snapshot/started-at v=yes mode=stale
@pick n=65 row=contract/instance-snapshot/pinned-bin v=yes mode=stale
@pick n=66 row=contract/instance-snapshot/endpoint v=yes mode=stale
@pick n=67 row=contract/instance-snapshot/child-pid v=yes mode=stale
@pick n=68 row=contract/hook-process/exit v=no
@pick n=69 row=contract/hook-decision/socket-dir-refused v=no
@pick n=70 row=contract/hook-decision/server-verify-failed v=no
@pick n=71 row=contract/hook-decision/question v=no
@pick n=72 row=contract/hook-decision/plan-revise v=no
@pick n=73 row=contract/hook-decision/plan-approve v=no
@pick n=74 row=contract/hook-decision/permission v=no
@pick n=75 row=contract/hook-decision/concurrent-dialog v=no
@pick n=76 row=contract/exit-21-detail/unwrapped-name v=no
@pick n=77 row=contract/exit-21-detail/strict-modes-failed v=no
@pick n=78 row=contract/exit-21-detail/socket-dir-refused v=no
@pick n=79 row=contract/exit-21-detail/server-verify-failed v=no
@pick n=80 row=contract/exit-21-detail/instance-dead v=no
@pick n=81 row=contract/exit-21-detail/endpoint-lost v=no
@pick n=82 row=contract/events/human-input-wheel v=no
@pick n=83 row=contract/events/harness-prompt v=no
@pick n=84 row=contract/event-log/start-wheel-record v=yes mode=stale
@pick n=85 row=contract/dialog-events/logged-once v=no
@pick n=86 row=contract/cli-send/input-not-ready v=no
@pick n=87 row=contract/cli-json/internal-error v=no
@pick n=88 row=contract/cli-json/instance-unreachable v=no
@pick n=89 row=contract/cli-json-failure/document v=no
@pick n=90 row=contract/cli-human/internal-error v=no
@pick n=91 row=contract/cli-exit-codes/21 v=no
@pick n=92 row=contract/cli-answer/unverified-cli v=no
@pick n=93 row=contract/channel-unlink/unlink-record v=no
@pick n=94 row=contract/channel-link/already-linked v=no
@pick n=95 row=contract/channel-link-names/invalid-name v=no
@pick n=96 row=contract/channel-error/fault-bodies v=no
@pick n=97 row=contract/channel-error/-32603 v=no
