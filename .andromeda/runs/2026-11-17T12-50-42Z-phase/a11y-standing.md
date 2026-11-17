# a11y · standing conventions

- S1 · each new test lives inside an existing test suite and runs in the existing nextest run, its failure a nextest failure · bootstrap-phases-derive-for-route-setup-project
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
  decision/no-a11y-performance-budget · No performance budget per a11y CI run — A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
- S2 · each new test is deterministic with no retries, and a gate breach fails the build · 10-slo-invariants-a11y-budgets
- S3 · each new test's time bound falls on the driver, never on a human · cognitive-accessibility
