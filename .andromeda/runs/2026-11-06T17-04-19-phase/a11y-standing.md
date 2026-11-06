# a11y · standing conventions

- S1 · each new test that asserts CLI output discipline or the TUI boundary runs inside nextest-integration or nextest-e2e locally and, in CI, inside the test job's coverage suite on ubuntu-latest; no new suite enum value and no separate a11y driver outside scripts/agent-run.sh run without a Decisions Log entry · ## 9. CI Integration and ## 11. A11y Anti-Patterns › ### CI
- S2 · each new test meets zero flakiness: retries 0, no #[ignore], no retry-once policy; a flake keeps the chunk red until its root cause is fixed · ## 10. SLO Invariants & A11y Budgets and ## 11. A11y Anti-Patterns › ### SLO
- S3 · each new a11y-bearing check yields machine output in the harness run JSON; a manual screen-reader or keyboard pass is a supplement, never the only evidence, and no a11y artifact of its own is uploaded · ## 9. CI Integration and ## 11. A11y Anti-Patterns › ### Universal (agent-driven specific)
