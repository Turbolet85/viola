group: architecture+test-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:53   class: stale
  leaf-text: «the probe's `orphans-check probes: 1/1 fired, control clean`»
  source: .andromeda/test-plan.md:1028 «the probe fires on each of its two planted orphans, last line `orphans-check probes: 2/2 fired, control clean`»
  replace-with: «the probe fires on each of its two planted orphans, last line `orphans-check probes: 2/2 fired, control clean`»

- leaf: .claude/docs/commands.md:23   class: omission
  leaf-text: «CI runs it only against the fake agent's `-p/--print` mode and its `--screens --turn-stop --trusted-root --dialogs --framing` interactive modes»
  source: .andromeda/architecture.md:315 «`viola-mcp-tap` (a `[[bin]]` of `viola-e2e` that records a live MCP exchange for the `Mcp.` fixtures, Repository below: `record <capture> <viola exe> <args…>` is a direct spawn, no shell and no PATH lookup»
  replace-with: «CI runs it only against the fake agent's `-p/--print` mode and its `--screens --turn-stop --trusted-root --dialogs --framing` interactive modes. The `Mcp.<variant>.json` MCP exchanges (`Mcp.tools-list-1.json` in `2.1.302`) are recorded not by `--record` but by the test-only `viola-mcp-tap` (a `[[bin]]` of `viola-e2e`): `record <capture> <viola exe> <args…>` sits between one live `claude -p` start and a direct spawn of `viola mcp` (no shell, no PATH lookup), appending each direction-tagged line through `take(MAX_FRAME)` to a 0600 capture that is never committed; `fixture <capture> <out>` keeps the client frames up to and including the first `tools/list` request and each request's result keyed by method, through the `--record` scrub, refusing a dirty exchange whole with the file name and one closed code (`home-path` · `absolute-path` · `username` · `email`); the fixture is validated against `schemas/claude-mcp-fixture.v1.json` and replayed against `viola mcp` by `tests/mcp_recorded_exchange.rs`»
