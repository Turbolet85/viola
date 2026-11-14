# a11y · standing conventions

- S1 · every new test runs in the existing nextest suites inside the coverage run, and its failure fails the build · bootstrap-phases-derive-for-route-setup-project
  decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
- S2 · every new test is deterministic, never retried and never skipped in CI · 10-slo-invariants-a11y-budgets
- S3 · every new test passes the gate's coverage and doctest requirement · ci-integration
