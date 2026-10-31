# a11y · standing conventions

- S1 · A new CLI assertion uses the existing `assert_cmd` / `trycmd` / portable-pty drivers inside `nextest-integration` / `nextest-e2e` (CI: `coverage`), with no new `suite` value and no separate a11y driver · §1 harness specification, §3 contracts, §9 CI Integration, §11 CI
- S2 · A new CLI output case runs on all three OS legs, each leg's `gate --require coverage,doctest` its own verdict · §9 CI Integration, §10 SLO Invariants
- S3 · Zero flakiness: `retries: 0`, `#[ignore]` banned, no retry-once for a violation · §10 SLO Invariants, §11 SLO
- S4 · Output-discipline results stand in for SCs and carry no WCAG conformance claim · §11 Strategy, .andromeda/a11y-plan-detail/a11y-scope-summary.md
- S5 · Evidence is machine output from an agent-runnable command; a manual pass only supplements it · §11 Universal, §11 CI
