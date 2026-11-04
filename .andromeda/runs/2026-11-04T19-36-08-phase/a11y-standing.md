# a11y · standing conventions

- S1 · each new cli output-discipline or tui boundary test runs inside the existing nextest-integration / nextest-e2e suites (CI: the per-OS coverage suite) on all three OS legs; no new suite enum value such as a11y without a Decisions Log entry, and no separate a11y driver outside scripts/agent-run.* run · ## 9. CI Integration
- S2 · each new test obeys zero flakiness: retries 0, no retry-once policy for a11y violations, #[ignore] banned, a flake keeps the chunk red until the root cause is fixed · ## 10. SLO Invariants & A11y Budgets
- S3 · no new cli or tui test result is cited as WCAG conformance evidence; assert_cmd / trycmd / portable-pty results are output-discipline stand-ins for SC 1.3.2, 1.4.1 and 3.3.1 only · ## 11. A11y Anti-Patterns
- S4 · no new a11y artifact upload and no stage or performance budget of its own; a cli or tui failure is a nextest failure in the harness run JSON · ## 9. CI Integration
- S5 · an output-discipline invariant that lives in crate tests falls under the epoch-boundary mutation audit (run --mutants) · ## 10. SLO Invariants & A11y Budgets
