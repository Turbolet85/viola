# a11y · standing conventions

- S1 · A new CLI or boundary case runs inside `nextest-integration` / `nextest-e2e`, in CI inside the per-OS `test` job's `coverage` suite, with no new `suite` value and no second driver · body §9 CI Integration and §11 CI
- S2 · A new case is held by `gate --require coverage,doctest` on each OS leg and never skips · body §9 CI Integration and §10
- S3 · Zero flakiness for a new case: `retries: 0`, no retry-once, `#[ignore]` banned · body §10 and §11 SLO
- S4 · A new fixed CLI string is pinned by trycmd or by literal, so a copy change fails CI until the pin moves on purpose · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S5 · A new output case uses `assert_cmd`, `trycmd` or the portable-pty outer PTY, and never parses the `claude` screen or takes a screenshot · body §1 harness specification and §11 Universal
- S6 · Results of these cases are output-discipline stand-ins, and no WCAG conformance claim rests on them · body §11 Strategy
