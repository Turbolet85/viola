@pick n=1 row=decision/ci-homes-kept-until-gates v=no
@pick n=2 row=decision/cleanup-removes-home-record-and-supervisor v=no
@pick n=3 row=decision/control-file-gating v=no
@pick n=4 row=decision/decision-bodies-snapshotted v=no
@pick n=5 row=decision/dirty-payload-refuses-recording v=yes mode=add-to fact=F1
@pick n=6 row=decision/env-per-child v=no
@pick n=7 row=decision/fake-agent-children-own-process-group v=no
@pick n=8 row=decision/fake-agent-exit-ignores-sighup v=no
@pick n=9 row=decision/fake-agent-on-test-path v=no
@pick n=10 row=decision/fixture-hygiene-walk v=yes mode=stale
@pick n=11 row=decision/fixtures-per-cli-version v=no
@pick n=12 row=decision/harness-boot-creates-stamps-and-starts v=no
@pick n=13 row=decision/hook-payloads-recorded-locally v=no
@pick n=14 row=decision/human-output-as-literals v=no
@pick n=15 row=decision/hygiene-reports-class-only v=no
@pick n=16 row=decision/injected-clock v=no
@pick n=17 row=decision/insta-snapshots v=no
@pick n=18 row=decision/mcp-exchange-recorded-by-tap v=no
@pick n=19 row=decision/mcp-tap-refuses-dirty-exchange v=no
@pick n=20 row=decision/per-test-home v=no
@pick n=21 row=decision/planted-hygiene-violations v=no
@pick n=22 row=decision/product-state-from-real-runs v=no
@pick n=23 row=decision/proptest-persisted-seeds v=no
@pick n=24 row=decision/proptest-strategies v=no
@pick n=25 row=decision/record-only-at-zero-failing-rows v=yes mode=add-to fact=F1
@pick n=26 row=decision/recorder-scrub v=yes mode=add-to fact=F1
@pick n=27 row=decision/rstest-fixtures v=no
@pick n=28 row=decision/screen-fixture-scrub-as-detector v=yes mode=contradict
@pick n=29 row=decision/stamps-written-only-by-verify v=yes mode=stale
@pick n=30 row=decision/synthetic-fixture-text v=no
@pick n=31 row=decision/tempfile-test-dirs v=no
@pick n=32 row=decision/test-homes-made-private v=no
@pick n=33 row=decision/tests-seed-own-data v=no
@pick n=34 row=decision/time-as-argument v=no
@pick n=35 row=contract/fake-agent-cli/argv-only-options v=no
@pick n=36 row=contract/fake-agent-cli/cli-version v=no
@pick n=37 row=contract/fake-agent-cli/control v=no
@pick n=38 row=contract/fake-agent-cli/script v=no
@pick n=39 row=contract/fake-agent-cli/settings v=no
@pick n=40 row=contract/fake-agent-cli/transcript v=no
@pick n=41 row=contract/fake-agent-cli/trust-config v=no
@pick n=42 row=contract/fake-agent-cli/trust-record-blank v=no
@pick n=43 row=contract/fake-agent-cli/unknown-arguments v=no
@pick n=44 row=contract/fake-agent-events/session-start v=no
@pick n=45 row=contract/fake-agent-exit/exit-triggers v=no
@pick n=46 row=contract/fake-agent-hooks/command-spawn v=no
@pick n=47 row=contract/fake-agent-hooks/stdin-payload v=no
@pick n=48 row=contract/fake-agent-input/bracketed-paste v=no
@pick n=49 row=contract/fake-agent-input/clear v=no
@pick n=50 row=contract/fake-agent-input/compact v=no
@pick n=51 row=contract/fake-agent-input/echo-command v=no
@pick n=52 row=contract/fake-agent-input/framing-paste v=no
@pick n=53 row=contract/fake-agent-input/keystroke v=no
@pick n=54 row=contract/fake-agent-receipt/env v=no
@pick n=55 row=contract/fake-agent-receipt/kind v=no
@pick n=56 row=contract/fake-agent-receipt/line v=no
@pick n=57 row=contract/fake-agent-screens/trust-dialog v=no
@pick n=58 row=contract/fake-script/format v=no
@pick n=59 row=contract/fake-script/gated-step v=no
@pick n=60 row=contract/fake-script/unreadable-exit v=no
@pick n=61 row=contract/verify-recording-refusal/unable-message v=yes mode=add-to fact=F1
