# a11y · standing conventions

- S1 · each new test's a11y-relevant assertions live inside the existing test suites, with no separate a11y driver or suite · 1-a11y-scope-summary · 9-ci-integration
  decision/a11y-cases-inside-coverage-suite · A11y cases run inside the coverage suite — The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict. · why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
  decision/a11y-inside-existing-suites · A11y assertions inside existing suites — A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
  decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
  decision/no-a11y-tool-output-discipline-assertions · Output-discipline assertions instead of an a11y tool — cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement. · why: no automated a11y tool reaches a terminal
  decision/no-separate-a11y-driver · No separate a11y driver or suite value — A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
- S2 · each new test fails as a nextest failure; a test may not be skipped and the CI test gate allows no skips · bootstrap-phases-derive-for-route-setup-project
  contract/structured-violation-json/cli-tui-a11y-rows · Structured violation JSON · a11y rows for cli and tui — The cli and tui surfaces emit no a11y rows.
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
  decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
  decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
- S3 · each new test is deterministic, never retried, and a breach fails the build · 10-slo-invariants-a11y-budgets
  decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
  decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
  decision/no-soft-a11y-slo · No soft a11y SLO — An a11y SLO is never soft: it triggers a build or deploy failure on its threshold.
- S4 · each new boundary test runs on recorded fixtures, sequenced on records, never on a live claude start · keyboard-test-harness
