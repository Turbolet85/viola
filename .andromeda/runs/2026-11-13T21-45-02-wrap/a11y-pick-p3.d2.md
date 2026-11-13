@pick n=1 row=contract/structured-violation-json/suites-failures v=no
@pick n=2 row=contract/structured-violation-json/cli-tui-a11y-rows v=no
@pick n=3 row=decision/tui-screen-reader-test-setup v=no
@pick n=4 row=decision/cli-screen-reader-test-setup v=no
@pick n=5 row=decision/zero-viola-bytes-on-the-terminal v=yes mode=stale
@pick n=6 row=decision/stdin-classifier-non-editing-list v=no
@pick n=7 row=decision/statusline-output-passed-byte-for-byte v=no
@pick n=8 row=decision/live-readings-never-gate v=no
@pick n=9 row=decision/human-key-takes-the-wheel v=no
@pick n=10 row=decision/boundary-tests-sequenced-on-records v=no
@pick n=11 row=decision/boundary-check-never-inspects-claude-content v=no
@pick n=12 row=decision/gate-requires-coverage-doctest v=no
@pick n=13 row=decision/manual-screen-reader-pass v=no
@pick n=14 row=decision/a11y-runs-in-existing-test-suites v=no
@pick n=15 row=decision/a11y-failures-are-nextest-failures v=no
@pick n=16 row=decision/tui-boundary-driver-pin v=no
@pick n=17 row=decision/cli-a11y-by-literal-tests v=no
@pick n=18 row=decision/refusal-ends-with-hint v=yes mode=stale
@pick n=19 row=decision/non-editing-sequences v=no
@pick n=20 row=decision/no-vendor-locked-a11y-tool v=no
@pick n=21 row=decision/no-unwarned-timeouts v=no
@pick n=22 row=decision/no-stack-trace-only-error v=no
@pick n=23 row=decision/no-spinners v=no
@pick n=24 row=decision/no-soft-a11y-slo v=no
@pick n=25 row=decision/no-screen-content-parsing v=no
@pick n=26 row=decision/no-information-by-colour-alone v=no
@pick n=27 row=decision/no-colour-on-failure-words v=no
@pick n=28 row=decision/manual-checks-supplemental-only v=no
@pick n=29 row=decision/machine-output-a11y-review v=no
@pick n=30 row=decision/list-captions-fixed v=no
@pick n=31 row=decision/codes-beside-plain-word v=no
@pick n=32 row=decision/a11y-fail-fast-gate v=no
@pick n=33 row=decision/zero-flakiness v=no
@pick n=34 row=decision/no-a11y-performance-budget v=no
@pick n=35 row=decision/mutation-audit-covers-crate-invariants v=no
@pick n=36 row=decision/cli-invariants-pass-in-ci v=no
@pick n=37 row=decision/a11y-gate-breach-fails-build v=no
@pick n=38 row=decision/no-separate-a11y-driver v=no
@pick n=39 row=decision/no-own-a11y-artifact v=no
@pick n=40 row=decision/a11y-ci-github-actions v=no
@pick n=41 row=decision/a11y-cases-inside-coverage-suite v=no
@pick n=42 row=contract/viola-wait/timed-out v=no
@pick n=43 row=contract/viola-verify/interrupted-pair v=no
@pick n=44 row=contract/viola-link-line/time v=no
@pick n=45 row=contract/viola-hook-statusline/exit v=no
@pick n=46 row=contract/start-json-refusal/document v=no
@pick n=47 row=contract/start-holder-refusal/stale-heartbeat v=no
@pick n=48 row=contract/start-holder-refusal/already-live v=no
@pick n=49 row=contract/send-refusal/no-prompt-submitted v=yes mode=add-to fact=F1
@pick n=50 row=contract/send-refusal/input-not-ready v=yes mode=add-to fact=F1
@pick n=51 row=contract/mcp-wait-tool/timeout-ms v=no
@pick n=52 row=contract/json-fault/internal-error v=no
@pick n=53 row=contract/instance-unreachable-json/document v=no
@pick n=54 row=contract/instance-unreachable-detail/unwrapped-name v=yes mode=contradict
@pick n=55 row=contract/instance-unreachable-detail/strict-modes-failed v=no
@pick n=56 row=contract/instance-unreachable-detail/socket-dir-refused v=no
@pick n=57 row=contract/instance-unreachable-detail/server-verify-failed v=no
@pick n=58 row=contract/instance-unreachable-detail/instance-dead v=no
@pick n=59 row=contract/instance-unreachable-detail/endpoint-lost v=no
@pick n=60 row=contract/human-fault-line/wrapper-fault v=no
@pick n=61 row=contract/fault-body/message v=no
@pick n=62 row=contract/event-line/dialog-expired v=no
@pick n=63 row=contract/config-json/seven-day v=no
@pick n=64 row=contract/config-json/five-hour v=no
@pick n=65 row=contract/config-json/dialog-hold-s v=no
@pick n=66 row=contract/code-word/format v=no
@pick n=67 row=contract/cli-refusal/stderr-layout v=yes mode=add-to fact=F2
@pick n=68 row=contract/cli-fixed-message/form v=no
@pick n=69 row=contract/cli-exit-codes/21 v=no
@pick n=70 row=contract/cli-exit-codes/20 v=no
@pick n=71 row=contract/cli-exit-codes/2 v=no
@pick n=72 row=contract/cli-exit-codes/14 v=no
@pick n=73 row=contract/cli-exit-codes/13 v=yes mode=add-to fact=F1
@pick n=74 row=contract/cli-exit-codes/11 v=no
@pick n=75 row=decision/unstattable-home-is-strict-modes-refusal v=no
@pick n=76 row=decision/unlisted-code-words-stay-visible v=no
@pick n=77 row=decision/unknown-refusal-has-no-hint v=yes mode=add-to fact=F1
@pick n=78 row=decision/time-bounds-fall-on-the-driver v=no
@pick n=79 row=decision/thiserror-messages-interpolate-nothing v=no
@pick n=80 row=decision/strict-modes-refusal-before-any-write v=no
@pick n=81 row=decision/socket-dir-refusal-before-any-frame v=no
@pick n=82 row=decision/printed-names-are-parsed-arguments v=no
@pick n=83 row=decision/peer-replies-screened-to-code-words v=no
@pick n=84 row=decision/one-hint-per-code v=yes mode=stale
@pick n=85 row=decision/no-hint-names-release v=yes mode=stale
@pick n=86 row=decision/link-result-lines-one-write-no-sgr v=no
@pick n=87 row=decision/link-pair-validated-by-wrapper v=no
@pick n=88 row=decision/json-one-document-no-hint v=no
@pick n=89 row=decision/json-failure-documents-are-clean v=yes mode=stale
@pick n=90 row=decision/fixed-cli-strings-pinned-by-literal v=no
@pick n=91 row=decision/faults-carry-no-hint v=no
@pick n=92 row=decision/error-word-detail-hint v=yes mode=add-to fact=F1
@pick n=93 row=decision/error-bodies-have-writer-and-witness v=no
@pick n=94 row=decision/dialog-hold-bounds-the-driver v=no
@pick n=95 row=decision/client-euid-refusal-test-strategy v=no
@pick n=96 row=decision/claude-agents-read-fails-open v=no
@pick n=97 row=decision/budget-gate-only-on-send v=no
@pick n=98 row=decision/budget-gate v=no
