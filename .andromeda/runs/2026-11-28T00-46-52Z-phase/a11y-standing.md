# a11y · standing conventions

- S1 · each new test fails as a nextest failure inside the existing suites and coverage run, no skips, no retries, zero flakiness · 9-ci-integration
  contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
- S2 · each new log line goes to the diagnostics log under its role with its identity tags, never to a terminal · a11y-scope-summary
  contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
  decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
- S3 · each new fixed CLI string is taken from design copy and pinned by a literal test · 8-cognitive-accessibility · cognitive-accessibility
