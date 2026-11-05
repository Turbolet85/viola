# a11y · standing conventions

- S1 · every new or changed test runs inside the existing nextest suites through scripts/agent-run.sh run (in CI, the test job's coverage suite); no new suite enum value such as a11y and no separate a11y driver without a Decisions Log entry · ## 9. CI Integration; ## 11. A11y Anti-Patterns § CI
- S2 · every CI job keeps the gate --require coverage,doctest with no skips allowed; a suite-failed, suite-skipped, suite-missing or artifact-missing breach fails the build · ## 10. SLO Invariants & A11y Budgets
- S3 · every test is zero-flake: retries 0, no #[ignore], no retry-once policy; a flake keeps the chunk red until the root cause is fixed · ## 10. SLO Invariants & A11y Budgets; ## 11. A11y Anti-Patterns § SLO
- S4 · a cli or tui failure lands as a nextest failure in the harness run JSON (suites[].failures[]), and no a11y artifact is uploaded of its own · ## 9. CI Integration
- S5 · no test parses the rendered claude TUI screen or takes screenshots as a11y evidence; assertions stay on text, streams and the outer-PTY boundary · ## 11. A11y Anti-Patterns § Universal
