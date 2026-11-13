@pick n=1 row=contract/verify-recording-refusal/unable-message v=yes
@pick n=2 row=contract/fake-script/unreadable-exit v=no
@pick n=3 row=contract/fake-script/gated-step v=yes
@pick n=4 row=contract/fake-script/format v=yes
@pick n=5 row=contract/fake-agent-receipt/line v=no
@pick n=6 row=contract/fake-agent-receipt/kind v=no
@pick n=7 row=contract/fake-agent-receipt/env v=no
@pick n=8 row=contract/fake-agent-input/keystroke v=yes
@pick n=9 row=contract/fake-agent-input/framing-paste v=yes
@pick n=10 row=contract/fake-agent-input/compact v=yes
@pick n=11 row=contract/fake-agent-input/clear v=no
@pick n=12 row=contract/fake-agent-input/bracketed-paste v=yes
@pick n=13 row=contract/fake-agent-hooks/stdin-payload v=yes
@pick n=14 row=contract/fake-agent-hooks/status-line-command v=no
@pick n=15 row=contract/fake-agent-hooks/command-spawn v=no
@pick n=16 row=contract/fake-agent-exit/exit-triggers v=no
@pick n=17 row=contract/fake-agent-events/session-start v=no
@pick n=18 row=contract/fake-agent-cli/unknown-arguments v=no
@pick n=19 row=contract/fake-agent-cli/script v=yes
@pick n=20 row=contract/fake-agent-cli/control v=yes
@pick n=21 row=contract/fake-agent-cli/cli-version v=yes
@pick n=22 row=contract/fake-agent-cli/argv-only-options v=no
@pick n=23 row=decision/time-as-argument v=yes
@pick n=24 row=decision/tests-seed-own-data v=no standing=S5
@pick n=25 row=decision/test-homes-made-private v=no standing=S5
@pick n=26 row=decision/tempfile-test-dirs v=no standing=S5
@pick n=27 row=decision/synthetic-fixture-text v=no standing=S2
@pick n=28 row=decision/statusline-source-read-only v=no
@pick n=29 row=decision/stamps-written-only-by-verify v=yes
@pick n=30 row=decision/screen-fixture-scrub-as-detector v=yes
@pick n=31 row=decision/rstest-fixtures v=no standing=S1
@pick n=32 row=decision/recorder-scrub v=no standing=S2
@pick n=33 row=decision/record-only-at-zero-failing-rows v=yes
@pick n=34 row=decision/proptest-strategies v=no
@pick n=35 row=decision/proptest-persisted-seeds v=no
@pick n=36 row=decision/product-state-from-real-runs v=no standing=S5
@pick n=37 row=decision/planted-hygiene-violations v=no
@pick n=38 row=decision/per-test-home v=no standing=S5
@pick n=39 row=decision/mcp-tap-refuses-dirty-exchange v=no
@pick n=40 row=decision/mcp-exchange-recorded-by-tap v=no
@pick n=41 row=decision/insta-snapshots v=no
@pick n=42 row=decision/injected-clock v=yes
@pick n=43 row=decision/hygiene-reports-class-only v=no
@pick n=44 row=decision/human-output-as-literals v=yes
@pick n=45 row=decision/hook-payloads-recorded-locally v=yes
@pick n=46 row=decision/harness-boot-creates-stamps-and-starts v=no
@pick n=47 row=decision/fixtures-per-cli-version v=no standing=S2
@pick n=48 row=decision/fixture-hygiene-walk v=no standing=S2
@pick n=49 row=decision/fake-agent-on-test-path v=yes
@pick n=50 row=decision/fake-agent-exit-ignores-sighup v=no
@pick n=51 row=decision/fake-agent-children-own-process-group v=no
@pick n=52 row=decision/env-per-child v=no standing=S4
@pick n=53 row=decision/dirty-payload-refuses-recording v=yes
@pick n=54 row=decision/decision-bodies-snapshotted v=no
@pick n=55 row=decision/control-file-gating v=yes
@pick n=56 row=decision/cleanup-removes-home-record-and-supervisor v=no
@pick n=57 row=decision/ci-homes-kept-until-gates v=no
@pick n=58 row=decision/budget-file-from-statusline-hook v=no
@pick n=59 row=contract/viola-wait/turn-ended v=no
@pick n=60 row=contract/viola-wait/timeout v=no
@pick n=61 row=contract/viola-wait/non-waking-records v=no
@pick n=62 row=contract/viola-wait/already-past v=no
@pick n=63 row=contract/viola-send/unconfirmable-local-command v=no
@pick n=64 row=contract/viola-send/turn-running v=no
@pick n=65 row=contract/viola-send/trailing-newlines v=no
@pick n=66 row=contract/viola-send/records v=no
@pick n=67 row=contract/viola-send/paste-framing v=no
@pick n=68 row=contract/viola-send/ok-document v=no
@pick n=69 row=contract/viola-send/off-list-slash-text v=no
@pick n=70 row=contract/viola-send/no-prompt-submitted v=no
@pick n=71 row=contract/viola-send/manual-pause v=no
@pick n=72 row=contract/viola-send/inner-cr v=no
@pick n=73 row=contract/viola-send/human-typing v=no
@pick n=74 row=contract/viola-send/human-key-during-gate v=no
@pick n=75 row=contract/viola-send/compact-confirmed v=no
@pick n=76 row=contract/viola-send/clear-confirmed v=no
@pick n=77 row=contract/viola-send-human-mode/unconfirmable v=no
@pick n=78 row=contract/viola-send-human-mode/unable v=no
@pick n=79 row=contract/viola-send-human-mode/read-back v=no
@pick n=80 row=contract/viola-run/child-on-wrapper-kill v=no
@pick n=81 row=contract/viola-run-start/tampered-pinned-exe v=no
@pick n=82 row=contract/viola-run-start/squatted-endpoint v=no
@pick n=83 row=contract/viola-run-start/socket-dir-check v=no
@pick n=84 row=contract/viola-run-start/settings-override v=no
@pick n=85 row=contract/viola-run-start/plugin-hooks v=no
@pick n=86 row=contract/viola-run-start/home-tree-check v=no
@pick n=87 row=contract/viola-run-start/event-order v=no
@pick n=88 row=contract/viola-run-start/duplicate-run v=no
@pick n=89 row=contract/viola-run-start/child-after-events v=no
