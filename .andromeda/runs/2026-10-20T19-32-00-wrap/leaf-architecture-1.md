group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: retired
  leaf-text: «the last three relied on every OS) and `relied_here`;»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «the twenty-four rows (`LedgerRow::ALL`) and the per-OS relied set (`relied()`, filtering on `relied_on(WRAPPED_HERE)`)»
  replace-with: «the last three relied on every OS);»

- leaf: .claude/docs/services/viola-agent-claude.md:39   class: omission
  leaf-text: «and every other hook payload against `claude-fixture.v1.json`»
  source: .andromeda/architecture.md:381 «`tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/Mcp.*.json` against it with the path, username and email walk, and its hook-payload walk leaves `Mcp.` files out.»
  replace-with: «every `Mcp.*.json` against `schemas/claude-mcp-fixture.v1.json` (`mcp_fixtures_pass_hygiene`; the hook-payload walk leaves `Mcp.` files out) and every other hook payload against `claude-fixture.v1.json`»

- leaf: .claude/docs/services/viola-agent-claude.md:14   class: omission
  leaf-text: «`viola verify` (probes, fixture recorder).»
  source: .andromeda/architecture.md:315 «through `ledger::scrub` (the home → `~`, the user word → `<user>`) and then `ledger::screen_fault` over every string and key»
  replace-with: «`viola verify` (probes, fixture recorder), and the test-only `viola-mcp-tap` (its `fixture` command runs a recorded MCP exchange through `ledger::scrub` and then `ledger::screen_fault` for the `Mcp.` fixtures).»

- leaf: .claude/docs/conventions.md:13   class: omission
  leaf-text: «(the founder's own word, chunk 2026-10-17-cli-output-discipline; arch §Occupied Resources → Environment variables).»
  source: .andromeda/architecture.md:351 «Test-only, read by no `viola` build: `MCP_SERVE_TEST_CHILD`, a marker `viola-mcp`'s unit tests set by `Command::env` on the child they spawn»
  replace-with: «(the founder's own word, chunk 2026-10-17-cli-output-discipline; arch §Occupied Resources → Environment variables). One test-only marker sits outside both prefixes and is read by no `viola` build: `MCP_SERVE_TEST_CHILD`, set by `viola-mcp`'s unit tests through `Command::env` on the child they spawn and read only by that crate's `#[cfg(test)]` child entry `serve_child_entry`.»
