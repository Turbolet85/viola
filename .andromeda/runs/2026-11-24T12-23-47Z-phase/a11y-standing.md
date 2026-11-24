# a11y · standing conventions

- S1 · each new a11y assertion runs inside the existing test suites under nextest in the GitHub Actions push/PR workflow, as a nextest failure, with no skips, no retries and no a11y artifact of its own · 9-ci-integration
  contract/structured-violation-json/cli-tui-a11y-rows · Structured violation JSON · a11y rows for cli and tui — The cli and tui surfaces emit no a11y rows.
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
  decision/a11y-cases-inside-coverage-suite · A11y cases run inside the coverage suite — The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict. · why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
  decision/a11y-ci-github-actions · A11y checks in the GitHub Actions push/PR workflow — A11y checks run in the one GitHub Actions push/PR workflow on an `ubuntu-latest` matrix; the scheduled nightly workflow carries no a11y step.
  decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
  decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
  decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
  decision/a11y-inside-existing-suites · A11y assertions inside existing suites — A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
  decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
  decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips.
  decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
  decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.
  decision/no-a11y-performance-budget · No performance budget per a11y CI run — A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
  decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
  decision/no-separate-a11y-driver · No separate a11y driver or suite value — A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
- S2 · each new fixed CLI string, caption, refusal and hint is pinned by its literal in a test, and refusal layout is held by literal pins · 8-cognitive-accessibility · cognitive-accessibility
- S3 · each new refusal is a word, a detail and a hint, its reason code beside the unable word, one hint per code, faults with no hint · cognitive-accessibility
- S4 · each new CLI output line is linear and static, escapes control bytes, carries no SGR off a terminal and none beside a failure word · aria-patterns-roles · 4-aria-patterns-roles
  contract/link-unlink/result-line · viola link and viola unlink · result line — `viola link` and `viola unlink` each print one plain-ASCII result line in one write, with no SGR in any mode.
- S5 · each new log line carries the log identity tags and never reaches the human's terminal · a11y-scope-summary
- S6 · each new boundary test is sequenced on records, never inspects claude content, and no live reading gates it · keyboard-test-harness
  contract/tui-passthrough/vendor-content · viola run TUI passthrough · vendor content — The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.
  decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content.
  decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
