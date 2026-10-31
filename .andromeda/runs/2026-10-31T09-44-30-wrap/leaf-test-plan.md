group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 106 · «- 2026-10-27: Every test or control that spawns `viola run`,»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»
- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «the `events.ndjson` reader, its wait and the session-start boot in `events.rs`»
  source: .andromeda/test-plan-detail/test-strategy.md:42 «and the one error-body check, `error_body_hit` and `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes)»
  replace-with: «the `events.ndjson` reader, its wait, the session-start boot and the one error-body check `error_body_hit` and `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes) in `events.rs`»
- leaf: .claude/rules/testing.md:40   class: omission
  leaf-text: «a lower-layer case (parser, refusal order, threshold, clock) belongs in unit tests, not E2E.»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:264 «No `-32603` is driven in a scanned home, and no failpoint or counted panic stands for one; the unit layer holds it»
  replace-with: «a lower-layer case (parser, refusal order, threshold, clock) belongs in unit tests, not E2E; no `-32603` is driven in a scanned home, and no failpoint or counted panic stands for one: the unit layer holds it.»
- leaf: .claude/docs/tests-summary.md:33   class: omission
  leaf-text: «secret scan + canary,»
  source: .andromeda/test-plan.md:606 «no helper runs after each E2E test: none is built, and the three surface witnesses (channel, CLI `--json`, MCP), the enumeration and the census of writers (`tests/contract_error_bodies.rs`) replace it»
  replace-with: «error sanitization (no helper runs after each E2E test: the three surface witnesses, `tests/channel_error_bodies.rs`, `tests/cli_error_bodies.rs` and the MCP cases of `crates/viola-e2e/tests/mcp_tools.rs`, and the enumeration and census of writers `tests/contract_error_bodies.rs` replace it, reading the check in `tests/support/events.rs`; every `-32603` held at the unit layer), secret scan + canary (after each job's end-to-end run, PROVISIONAL),»
