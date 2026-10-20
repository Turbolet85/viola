group: test-plan-1
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 103 · «- 2026-10-20: A census for a test's leaked children keys on »

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the `Screen.*.json` screens apart against `schemas/claude-screen.v1.json`»
  source: .andromeda/test-plan.md:971 «the one MCP exchange of `2.1.302`, `Mcp.tools-list-1.json`, against `schemas/claude-mcp-fixture.v1.json`»
  replace-with: «the `Mcp.*.json` MCP exchanges (`Mcp.tools-list-1.json` in 2.1.302, recorded by the test-only `viola-mcp-tap`) left out of that walk and walked by `mcp_fixtures` against `schemas/claude-mcp-fixture.v1.json` (`mcp_fixtures_pass_hygiene`), then replayed against `viola mcp` by `mcp_recorded_exchange` (the served `tools/list` result holding the recorded key and type shape and tool names in order), the `Screen.*.json` screens apart against `schemas/claude-screen.v1.json`»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «the 2 missed Windows-only;»
  source: .andromeda/test-plan.md:1096 «`prepare` takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`), so both arms are reached on every host, and both mutants (`Ok(None)` and the deleted `!`) read killed on the Linux dev host»
  replace-with: «the 2 missed in `prepare`, which now takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`) so both arms are reached on every host, both read killed on the Linux dev host by its scratch case at chunk 2026-10-20-epoch-5-cleanup-ii;»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «the 2 `prepare` rows as recorded, and the 16 in `src/cmd/verify/typed.rs` by a case in `typed.rs`'s `mod tests`»
  source: .andromeda/test-plan.md:1096 «the 2 `prepare` rows above stood as recorded (killed on every host since, above); and the 16 in `src/cmd/verify/typed.rs` (whose code now sits under its children in `src/cmd/verify/typed/`;»
  replace-with: «the 2 `prepare` rows as recorded (killed on every host since), and the 16 in `src/cmd/verify/typed.rs` (whose code now sits under its children in `src/cmd/verify/typed/`) by a case in its unit tests»

- leaf: .claude/docs/tests-summary.md:42   class: omission
  leaf-text: «confirms the 45 kills)»
  source: .andromeda/test-plan.md:1096 «The Epoch 5 boundary audit's 16 survivors were each disposed by a nextest case in the package that owns the mutated file, 10 killed and 6 after a behaviour-preserving restatement»
  replace-with: «confirms the 45 kills; the Epoch 5 boundary audit's 16 survivors were each disposed by a nextest case in the package that owns the mutated file, 10 killed and 6 after a behaviour-preserving restatement, none equivalent or recorded, at chunk 2026-10-20-epoch-5-cleanup-ii)»

- leaf: .claude/rules/testing.md:24   class: omission
  leaf-text: «Fixtures `fixtures/claude/<cli-version>/<Event>.<variant>.json` (the CLI's PascalCase hook event name, e.g. `PreToolUse.ask.json`; no mapping table)»
  source: .andromeda/test-plan.md:937 «Checked-in `fixtures/claude/<cli-version>/Mcp.<variant>.json` (`Mcp.tools-list-1.json` in `2.1.302`), recorded by the test-only `viola-mcp-tap` (`crates/viola-e2e`)»
  replace-with: «Fixtures `fixtures/claude/<cli-version>/<Event>.<variant>.json` (the CLI's PascalCase hook event name, e.g. `PreToolUse.ask.json`; no mapping table), and the recorded MCP exchanges `fixtures/claude/<cli-version>/Mcp.<variant>.json` (e.g. `Mcp.tools-list-1.json`, recorded by the test-only `viola-mcp-tap`, schema `schemas/claude-mcp-fixture.v1.json`)»
