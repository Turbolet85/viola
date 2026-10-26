# a11y · standing conventions

- S1 · every new CLI or TUI output-discipline case runs inside `nextest-integration` / `nextest-e2e` locally and inside the per-OS `test` job's single `coverage` suite on all three OS legs; no new `suite` value and no separate a11y driver outside `scripts/agent-run.* run` · §3 A11y Assertion Harness Contract / §9 CI Integration / §11 CI
- S2 · every fixed CLI string a change adds or alters is pinned by trycmd or by a literal, so a copy change fails CI until it is updated on purpose · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S3 · zero flakiness: `retries: 0`, no `#[ignore]`, no retry-once policy; a flake keeps the chunk red until its root cause is fixed · §10 SLO Invariants / §11 SLO
- S4 · the per-OS `gate --require coverage,doctest` never allows skips, and a breach fails the build · §9 CI Integration / §10 SLO Invariants
- S5 · no WCAG conformance claim rests on assert_cmd, trycmd or portable-pty results; they stand in for output discipline only · §11 Strategy
