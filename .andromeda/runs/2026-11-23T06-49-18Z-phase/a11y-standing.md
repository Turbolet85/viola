# a11y · standing conventions

- S1 · each new fixed CLI string (help line, refusal word, hint, caption) is pinned by a literal test and comes from design copy · cognitive-accessibility · 8-cognitive-accessibility
- S2 · each new refusal names a kebab-case code word beside unable, carries exactly one hint (none for a fault), and under --json is one clean document · cognitive-accessibility
  contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.
  decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
- S3 · each new a11y-bearing test runs inside the existing test suites and fails as a nextest failure · bootstrap-phases-derive-for-route-setup-project
- S4 · each new test is deterministic, never skipped and never retried · 10-slo-invariants-a11y-budgets
- S5 · each new test case runs inside the coverage suite of the GitHub Actions push/PR workflow, with no artifact or driver of its own · 9-ci-integration
- S6 · each new log line of a run goes to the run diagnostics, never the terminal, and carries the log identity tags · a11y-scope-summary
