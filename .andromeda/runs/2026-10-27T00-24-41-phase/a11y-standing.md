# a11y · standing conventions

- S1 · Each new test runs inside `nextest-integration` / `nextest-e2e` locally and inside the per-OS `test` job's single instrumented `coverage` suite in CI, gated by `gate --require coverage,doctest`; no new `suite` value and no separate driver outside `scripts/agent-run.* run` · §3 A11y Assertion Harness Contract and §9 CI Integration
- S2 · Zero flakiness: `retries: 0`, no `#[ignore]`, no retry-once policy; a flake keeps the chunk red until its root cause is fixed · §10 SLO Invariants & A11y Budgets and §11 SLO
- S3 · No WCAG conformance claim rests on any assert_cmd / trycmd / portable-pty result this chunk adds · §11 Strategy
