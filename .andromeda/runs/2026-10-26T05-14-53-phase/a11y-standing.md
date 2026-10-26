# a11y · standing conventions

- S1 · Any test case the chunk adds or touches runs inside the existing harness suites (`scripts/agent-run.* run`, CI's `coverage` suite); no new `suite` enum value and no separate a11y driver · body §1 A11y assertion harness specification (CI integration) / §9 CI Integration / §11 CI
- S2 · Zero-flakiness: `retries: 0`, no retry-once policy, `#[ignore]` banned, a flake keeps the chunk red until its root cause is fixed · body §10 SLO Invariants / §11 SLO
- S3 · No WCAG conformance claim rests on cli / tui output-discipline results · body §11 Strategy
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:28) scope-surfaces
