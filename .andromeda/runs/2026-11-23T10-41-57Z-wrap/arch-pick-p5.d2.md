@pick n=1 row=contract/agents-listing/unwrapped-sessions v=no
@pick n=2 row=contract/agents-listing/status v=no
@pick n=3 row=decision/wrapper-events-log-only v=yes mode=stale
@pick n=4 row=decision/wrapper-channel-json-rpc v=no
@pick n=5 row=decision/version-verified-all-relied-rows v=no
@pick n=6 row=decision/upstream-text-is-content v=no
@pick n=7 row=decision/stamps-writer-checks-before-and-under-lock v=no
@pick n=8 row=decision/stamps-merge-own-version-only v=no
@pick n=9 row=decision/spawn-directory-confined-to-snapshot v=no
@pick n=10 row=decision/snapshot-replay-log-fields-only v=yes mode=contradict
@pick n=11 row=decision/server-identity-check-before-frame v=no
@pick n=12 row=decision/revive-refused-without-cwd v=no
@pick n=13 row=decision/replayed-snapshot-liveness v=no
@pick n=14 row=decision/replayed-snapshot-gives-no-live-endpoint v=no
@pick n=15 row=decision/replay-writes-no-file v=no
@pick n=16 row=decision/permission-answer-without-suggestions v=no
@pick n=17 row=decision/option-text-never-in-human-line v=no
@pick n=18 row=decision/no-shell-out v=no
@pick n=19 row=decision/no-document-after-child-held-terminal v=no
@pick n=20 row=decision/mcp-over-stdio v=no
@pick n=21 row=decision/mcp-list-reads-agents v=no
@pick n=22 row=decision/malformed-stamps-unverified v=no
@pick n=23 row=decision/list-without-pagination v=no
@pick n=24 row=decision/list-reads-state-directly v=yes mode=add-to fact=F10
@pick n=25 row=decision/list-needs-no-wrapper v=no
@pick n=26 row=decision/link-set-follows-log v=no
@pick n=27 row=decision/link-set-changes-only-on-link-calls v=no
@pick n=28 row=decision/hyprctl-direct-spawn v=no
@pick n=29 row=decision/hook-output-one-crate v=no
@pick n=30 row=decision/hook-fails-open-on-replayed-snapshot v=no
@pick n=31 row=decision/frame-client-check-order v=no
@pick n=32 row=decision/dialog-logged-exactly-once v=no
@pick n=33 row=decision/client-strict-modes-before-frame v=no
@pick n=34 row=decision/api-error-turn-end-clears-turn v=no
@pick n=35 row=decision/agents-listing-never-errors v=no
@pick n=36 row=contract/viola-hook-exit-codes/0 v=no
@pick n=37 row=contract/verify/no-json v=no
@pick n=38 row=contract/verify-refusals/home-tree v=no
@pick n=39 row=contract/verify-process-exit/version-unreadable v=no
@pick n=40 row=contract/verify-process-exit/unclean-fixture v=no
@pick n=41 row=contract/verify-process-exit/strict-modes-failed v=no
@pick n=42 row=contract/verify-process-exit/program-not-found v=no
@pick n=43 row=contract/verify-process-exit/pinned-hash-mismatch v=no
@pick n=44 row=contract/verify-process-exit/interrupted v=no
@pick n=45 row=contract/unreachable-cause/cause-set v=no
@pick n=46 row=contract/turn-end-cause/cause-set v=no
@pick n=47 row=contract/snapshot/writer v=no
@pick n=48 row=contract/send/paste-failure-after-issue v=no
@pick n=49 row=contract/send-ok-payload/unconfirmable v=no
@pick n=50 row=contract/run-start-refusals/socket-dir-refused v=yes mode=add-to fact=F9
@pick n=51 row=contract/run-start-refusals/home-tree v=yes mode=add-to fact=F9
@pick n=52 row=contract/refusal-reason/unverified-cli v=no
@pick n=53 row=contract/refusal-reason/unknown-catch-all v=no
@pick n=54 row=contract/refusal-reason/unknown v=no
@pick n=55 row=contract/refusal-reason/not-delivered v=no
@pick n=56 row=contract/refusal-reason/human-typing v=no
@pick n=57 row=contract/refusal-reason/detail v=no
@pick n=58 row=contract/refusal-order/send v=no
@pick n=59 row=contract/refusal-order/first-only v=no
@pick n=60 row=contract/refusal-order/answer v=no
@pick n=61 row=contract/protocol-version/v v=no
@pick n=62 row=contract/peer-reply/refusal v=no
@pick n=63 row=contract/peer-reply/detail v=no
@pick n=64 row=contract/paste-text/empty v=no
@pick n=65 row=contract/paste-text/control-characters v=no
@pick n=66 row=contract/ok-payload/unconfirmable v=no
@pick n=67 row=contract/normalised-event-kinds/wheel v=yes mode=add-to fact=F3
@pick n=68 row=contract/normalised-event-kinds/unlink v=yes mode=add-to fact=F3
@pick n=69 row=contract/normalised-event-kinds/turn-ended v=yes mode=add-to fact=F3
@pick n=70 row=contract/normalised-event-kinds/session-start v=yes mode=add-to fact=F3
@pick n=71 row=contract/normalised-event-kinds/session-end v=yes mode=add-to fact=F3
@pick n=72 row=contract/normalised-event-kinds/send-refused v=yes mode=add-to fact=F3
@pick n=73 row=contract/normalised-event-kinds/send-issued v=yes mode=add-to fact=F3
@pick n=74 row=contract/normalised-event-kinds/send-confirmed v=yes mode=add-to fact=F3
@pick n=75 row=contract/normalised-event-kinds/question v=yes mode=add-to fact=F3
@pick n=76 row=contract/normalised-event-kinds/prompt-submitted v=yes mode=add-to fact=F3
@pick n=77 row=contract/normalised-event-kinds/plan v=yes mode=add-to fact=F3
@pick n=78 row=contract/normalised-event-kinds/permission v=yes mode=add-to fact=F3
@pick n=79 row=contract/normalised-event-kinds/link v=yes mode=add-to fact=F3
@pick n=80 row=contract/normalised-event-kinds/lineage v=yes mode=add-to fact=F3
@pick n=81 row=contract/normalised-event-kinds/dialog-expired v=yes mode=add-to fact=F3
@pick n=82 row=contract/normalised-event-kinds/activity v=yes mode=add-to fact=F3
@pick n=83 row=contract/mcp-tool-results/wrapper-fault v=no
@pick n=84 row=contract/mcp-tool-results/success v=no
@pick n=85 row=contract/mcp-tool-results/refusal v=no
@pick n=86 row=contract/mcp-tool-results/is-error-true v=no
@pick n=87 row=contract/mcp-tool-results/is-error-text v=no
@pick n=88 row=contract/mcp-tool-results/invalid-params v=no
@pick n=89 row=contract/mcp-tool-results/instance-unreachable v=no
@pick n=90 row=contract/mcp-list-results/ttlms v=no
@pick n=91 row=contract/mcp-list-results/cachescope v=no
@pick n=92 row=contract/mcp-json-rpc-errors/32603 v=no
@pick n=93 row=contract/mcp-json-rpc-errors/32602 v=no
@pick n=94 row=contract/mcp-frame-tools/strict-modes-failed v=no
@pick n=95 row=contract/mcp-frame-tools/socket-dir-refused v=no
@pick n=96 row=contract/mcp-frame-tools/server-verify v=no
@pick n=97 row=contract/hook-to-kind-map/userpromptsubmit v=no
@pick n=98 row=contract/hook-to-kind-map/stopfailure v=no
@pick n=99 row=contract/hook-to-kind-map/stop v=no
@pick n=100 row=contract/hook-to-kind-map/sessionstart v=no
