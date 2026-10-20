group: architecture+security-plan+test-plan+a11y-plan
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:15   class: omission
  leaf-text: «`Agents.recorded-<n>.json` agents variants recorded by `viola verify` (plus the superseded relayed dialog captures, `RELAYED.md`), replayed by the fake agent in CI;»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:113 «Agents.recorded-<n>.json agents variants recorded by `viola verify`, Mcp.<variant>.json MCP exchanges recorded by the test-only viola-mcp-tap, plus superseded relayed dialog fixtures (RELAYED.md)»
  replace-with: «`Agents.recorded-<n>.json` agents variants recorded by `viola verify` (plus the superseded relayed dialog captures, `RELAYED.md`), replayed by the fake agent in CI, and `Mcp.<variant>.json` MCP exchanges recorded by the test-only `viola-mcp-tap`, replayed against `viola mcp` by `tests/mcp_recorded_exchange.rs`;»

- leaf: CLAUDE.md:28   class: omission
  leaf-text: «`viola-harness` behind `scripts/agent-run.*` + the Tokio-based E2E clients»
  source: .andromeda/architecture.md:331 «- Test-only: `viola-e2e` (harness library + bins `viola-harness` and `viola-mcp-tap`, `publish = false`;»
  replace-with: «`viola-harness` behind `scripts/agent-run.*` + `viola-mcp-tap` (`record` / `fixture`: the live MCP exchange behind the `Mcp.` fixtures) + the Tokio-based E2E clients»
