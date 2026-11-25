# a11y · standing conventions

- S1 · each new test case runs inside the existing test suites as a nextest case, and its failure fails the build with no skip or retry · bootstrap-phases-derive-for-route-setup-project
  decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
  decision/cli-invariants-pass-in-ci · CLI output-discipline invariants pass in CI — The CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside the `coverage` suite.
  decision/no-separate-a11y-driver · No separate a11y driver or suite value — A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
- S2 · each new or changed fixed CLI string is pinned by its literal in a test · cognitive-accessibility
- S3 · each live reading is an operator-fired leg that never gates CI · keyboard-test-harness
