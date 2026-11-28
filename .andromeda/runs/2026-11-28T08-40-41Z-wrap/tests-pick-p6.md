@pick n=1 row=contract/event-line/keys v=no
@pick n=2 row=contract/mcp-frame-tools/free-text-refusal v=no
@pick n=3 row=contract/mcp-frame-tools/refusal-result v=no
@pick n=4 row=contract/mcp-unreachable-detail/call-deadline v=no
@pick n=5 row=contract/mcp-unreachable-detail/endpoint-lost v=no
@pick n=6 row=contract/mcp-unreachable-detail/instance-dead v=no
@pick n=7 row=contract/mcp-unreachable-detail/server-verify-failed v=no
@pick n=8 row=contract/mcp-unreachable-detail/socket-dir-refused v=no
@pick n=9 row=contract/mcp-unreachable-detail/strict-modes-failed v=no
@pick n=10 row=contract/strict-modes-refusal/mcp v=no
@pick n=11 row=contract/strict-modes-refusal/other-verbs v=yes mode=contradict
@pick n=12 row=contract/strict-modes-refusal/revive v=no
@pick n=13 row=contract/strict-modes-refusal/run v=no
@pick n=14 row=contract/strict-modes-refusal/verify v=no
@pick n=15 row=contract/verify-exit-detail/interrupted v=no
@pick n=16 row=contract/verify-exit-detail/pinned-hash-mismatch v=no
@pick n=17 row=contract/verify-exit-detail/program-not-found v=no
@pick n=18 row=contract/verify-exit-detail/strict-modes-failed v=no
@pick n=19 row=contract/verify-exit-detail/unclean-fixture v=no
@pick n=20 row=contract/verify-exit-detail/untrusted-folder v=no
@pick n=21 row=contract/verify-exit-detail/version-unreadable v=no
@pick n=22 row=contract/verify-output/step-line v=no
@pick n=23 row=contract/verify-output/summary-line v=no
@pick n=24 row=contract/verify-stop/signal v=no
@pick n=25 row=contract/viola-answer/oversize-stdin v=no
@pick n=26 row=contract/viola-help/verb-table v=yes mode=stale
@pick n=27 row=contract/wrapper-channel/accepted-text v=no
@pick n=28 row=contract/wrapper-channel/empty-text v=no
@pick n=29 row=contract/wrapper-channel/frame-over-max-frame v=no
@pick n=30 row=contract/wrapper-channel/params-v-higher v=no
@pick n=31 row=decision/bounded-readers v=yes mode=stale
@pick n=32 row=decision/child-env-names-viola-instance v=no
@pick n=33 row=decision/child-env-strip v=no
@pick n=34 row=decision/cross-user-peers-untestable v=no
@pick n=35 row=decision/decision-body-gate v=no
@pick n=36 row=decision/diagnostics-files-owner-only v=no
@pick n=37 row=decision/diagnostics-schema-conformance v=yes mode=stale
@pick n=38 row=decision/driver-name-self-reported v=no
@pick n=39 row=decision/endpoint-read-from-snapshot v=no
@pick n=40 row=decision/env-clear-keeps-llvm-profile-file v=no
@pick n=41 row=decision/error-payloads-sanitized v=yes mode=stale
@pick n=42 row=decision/fake-agent-matches-recordings v=no
@pick n=43 row=decision/fired-wake-keeps-wheel v=no
@pick n=44 row=decision/fixtures-match-tolerant-schema v=no
@pick n=45 row=decision/forward-compat-lines-skipped v=no
@pick n=46 row=decision/frame-verbs-reach-only-the-recorded-wrapper v=no
@pick n=47 row=decision/frame-verbs-refuse-widened-instance v=no
@pick n=48 row=decision/fuzz-separate-workspace v=no
@pick n=49 row=decision/hint-names-its-cause v=yes mode=add-to fact=F3
@pick n=50 row=decision/home-readable-by-others-refused v=yes mode=add-to fact=F3
@pick n=51 row=decision/hook-deadline-driver v=no
@pick n=52 row=decision/hook-fails-open v=no
@pick n=53 row=decision/hook-suites-in-integration-layer v=no
@pick n=54 row=decision/implicit-link-on-send v=no
@pick n=55 row=decision/injected-turn-keeps-wheel v=no
@pick n=56 row=decision/jsonschema-validator v=yes mode=stale
@pick n=57 row=decision/kr-after-key-settle v=no
@pick n=58 row=decision/kr-late-redraw-not-dirty-box v=no
@pick n=59 row=decision/kr-seams-rehearse-only v=no
@pick n=60 row=decision/link-is-idempotent v=no
@pick n=61 row=decision/link-name-validation v=no
@pick n=62 row=decision/list-board-refusal-opens-no-log v=no
@pick n=63 row=decision/list-names-escaped v=no
@pick n=64 row=decision/live-failing-reading-not-verification v=no
@pick n=65 row=decision/live-rig-hint-cell-reading v=no
@pick n=66 row=decision/mcp-stdio-test-owns-child v=no
@pick n=67 row=decision/no-release-in-driver-hints v=no
@pick n=68 row=decision/no-unbounded-json-depth v=no
@pick n=69 row=decision/non-json-lines-skipped v=no
@pick n=70 row=decision/panic-seam-test-only v=yes mode=add-to fact=F6
@pick n=71 row=decision/pinned-exe-hash-vectors v=no
@pick n=72 row=decision/plugin-install-refusal-creates-nothing v=no
@pick n=73 row=decision/property-tests-on-parsers v=no
@pick n=74 row=decision/refusal-rehearsal-reads-own-effects v=no
@pick n=75 row=decision/rig-no-key-read-from-keys-word v=no
@pick n=76 row=decision/scan-hits-never-echo-bytes v=yes mode=stale
@pick n=77 row=decision/schema-failure-no-content v=no
@pick n=78 row=decision/single-writer-state-files v=no
@pick n=79 row=decision/snapshot-replay-from-log v=no
@pick n=80 row=decision/socket-dir-refusal v=no
@pick n=81 row=decision/standing-live-rig v=no
@pick n=82 row=decision/start-collision-by-heartbeat v=no
@pick n=83 row=decision/start-hands-wheel-to-driver v=no
@pick n=84 row=decision/start-lines-read-from-start-offset v=no
@pick n=85 row=decision/torn-append-healed v=no
@pick n=86 row=decision/tty-asserted-only-on-verb-sgr v=yes mode=add-to fact=F1
@pick n=87 row=decision/tui-passthrough-never-parses-screen v=yes mode=add-to fact=F2
@pick n=88 row=decision/umask-independent-modes v=no
@pick n=89 row=decision/unlink-survives-restart v=no
@pick n=90 row=decision/unverified-cli-send-only v=no
@pick n=91 row=decision/unwrapped-name-never-a-target v=no
@pick n=92 row=decision/window-reopen-proven-live v=no
@pick n=93 row=contract/dialog-event/logged-once v=no
@pick n=94 row=contract/event-log/unlink v=no
@pick n=95 row=contract/exit-causes/socket-dir-refused v=no
@pick n=96 row=contract/generated-plugin/hook-commands v=no
@pick n=97 row=contract/hook-stdout/concurrent-dialog v=no
@pick n=98 row=contract/hook-stdout/decision-body v=no
@pick n=99 row=contract/json-rpc-errors/-32603 v=no
@pick n=100 row=contract/list-json/name v=no
