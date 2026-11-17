# a11y · standing conventions

- S1 · each new a11y assertion lives inside the existing test suites, not a separate a11y suite or driver · 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
  decision/no-a11y-performance-budget · No performance budget per a11y CI run — A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
  decision/no-separate-a11y-driver · No separate a11y driver or suite value — A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
- S2 · each new fixed CLI string is pinned by its literal in a test and its copy comes from design · 8-cognitive-accessibility · cognitive-accessibility
- S3 · each new test runs under the CI gate with no skips, no retries and zero flakiness, its failures nextest failures · 10-slo-invariants-a11y-budgets · 9-ci-integration
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
  decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
  decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
  decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
