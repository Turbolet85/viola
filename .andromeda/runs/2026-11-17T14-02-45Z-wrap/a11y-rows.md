# a11y-plan · the rows 2026-11-17-supply-chain-green-and-booted-tests-end touches — picked 5 of 235 · rule any · pin 2351

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal. · r1
decision/tui-boundary-driver-pin · TUI boundary driver — The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins. · r1

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own. · r1 · stale
decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own. · r1
decision/manual-screen-reader-pass · Manual screen reader pass — The screen reader check is a manual Orca pass owned by the founder, with no automated test spec. · r1
absence/rendered-page — A rendered page, and with it any browser a11y tooling · r1

## ci-integration · key of ## 3. A11y Assertion Harness Contract
decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed. · r1 · stale

## contrast-verification-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## focus-management-test-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content. · r1
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case. · r1
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins · r1
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating. · r1
decision/stdin-classifier-non-editing-list · Closed non-editing stdin list — Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel. · why: ratified, the exact grammars chosen after a narrower option was shown · r1
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run. · r1

## screen-reader-test-pattern · key of ## 3. A11y Assertion Harness Contract
decision/cli-screen-reader-test-setup · CLI screen-reader test setup — The cli surface is screen-reader tested with Orca on GNOME Terminal. · r1
decision/tui-screen-reader-test-setup · TUI screen-reader test setup — The tui surface is screen-reader tested with Orca on foot under Hyprland. · r1

## structured-violation-json-schema · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.
contract/structured-violation-json/cli-tui-a11y-rows · Structured violation JSON · a11y rows for cli and tui — The cli and tui surfaces emit no a11y rows. · r1
contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[]. · r1

## wcag-criteria-mapping · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## 10-slo-invariants-a11y-budgets · ## 10. SLO Invariants & A11y Budgets
decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy. · r1 · stale
decision/cli-invariants-pass-in-ci · CLI output-discipline invariants pass in CI — The CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside the `coverage` suite. · r1
decision/mutation-audit-covers-crate-invariants · Mutation audit covers invariants held in crate tests — Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them. · r1 · stale
decision/no-a11y-performance-budget · No performance budget per a11y CI run — A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite. · r1
decision/zero-flakiness · Zero flakiness, no retries — Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed. · why: retry-once policies mask real signals (the agent-driven principle) · r1 · stale
