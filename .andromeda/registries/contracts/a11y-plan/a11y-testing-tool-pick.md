### A11y testing tool pick

- **Primary tool per surface:**
  - **cli:** no a11y tool exists for terminal output. Output discipline is checked with assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1, running in the `nextest-integration` / `nextest-e2e` suites locally, and in the CI `test` job's `coverage` suite (one instrumented nextest run), on all three OS legs.
  - **tui:** portable-pty outer-PTY boundary driver, using the inherited `=0.8.1` pin (Decisions Log D-A11Y-13).
    - This is an inherited risk: architecture and tests pin portable-pty `=0.8.1` (2023-03-13) while 0.9.0 is the maintained line. a11y inherits that driver as-is and never re-pins it.
