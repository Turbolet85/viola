# a11y · standing conventions

- S1 · each new a11y assertion sits inside an existing nextest suite and its failure is a nextest failure, run in the GitHub Actions push/PR gate with no skips · bootstrap-phases-derive-for-route-setup-project · 9-ci-integration
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
  decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
  decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
  decision/a11y-gate-breach-fails-build · Gate breach fails the build — A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
  decision/a11y-inside-existing-suites · A11y assertions inside existing suites — A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
  decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
  decision/ci-test-gate-no-skips · CI test gate allows no skips — The CI test job's gate requires coverage and doctest and never allows skips.
  decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.
  decision/no-a11y-performance-budget · No performance budget per a11y CI run — A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
  decision/no-own-a11y-artifact · No a11y artifact of its own — A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
  decision/no-separate-a11y-driver · No separate a11y driver or suite value — A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
- S2 · each new TUI boundary test asserts at the boundary only, sequenced on records, never inspecting claude content · keyboard-test-harness
  decision/tui-boundary-only · Tui accessibility is boundary-only — The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.
- S3 · each new fixed CLI string a fix adds is pinned by its literal in a test and comes from design copy · 8-cognitive-accessibility · cognitive-accessibility
  decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
  decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
  decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
