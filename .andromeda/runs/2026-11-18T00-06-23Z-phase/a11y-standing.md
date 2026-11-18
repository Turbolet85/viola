# a11y · standing conventions

- S1 · every new check lives inside the existing test suites as an ordinary nextest case; no separate a11y suite, driver or artifact · 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
  decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
  decision/a11y-inside-existing-suites · A11y assertions inside existing suites — A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
  decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
  decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
  decision/no-separate-a11y-driver · No separate a11y driver or suite value — A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
- S2 · every new test is deterministic — zero flakiness, no retries, no skips in the CI gate · 10-slo-invariants-a11y-budgets
- S3 · every new fixed CLI string or printed word is pinned in its test by literal · 8-cognitive-accessibility · cognitive-accessibility
  decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
- S4 · every new test against the fake agent's screens is sequenced on records, never on screen content beyond the signatures · keyboard-test-harness
  decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
