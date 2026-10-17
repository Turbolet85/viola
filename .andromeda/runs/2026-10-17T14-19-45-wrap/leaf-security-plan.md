group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:17   class: omission
  leaf-text: «and `viola pause` / `viola release`'s frames after the same liveness-only pre-check (the seventh gap, F-W1), the founder's 2026-10-04 rulings)»
  source: .andromeda/security-plan.md:182 «until the same two entries land, `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the same liveness-only pre-check»
  replace-with: «and `viola pause` / `viola release`'s frames after the same liveness-only pre-check (the seventh gap, F-W1), the founder's 2026-10-04 rulings, and `viola mcp`'s `send` / `wait` / `last` / `answer` tool frames after the same liveness-only pre-check (the eighth gap, the founder's own answer to P4 fork Q1 of chunk 2026-10-17-mcp-server-for-drivers, 2026-10-17))»

- leaf: .claude/docs/security-summary.md:64   class: omission
  leaf-text: «Every CLI verb, `list`, `run` and `revive` included (`hook` keeps its own shape check), resolves its home through `resolved_home`»
  source: .andromeda/security-plan.md:203 «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers), resolves the home through `resolved_home`»
  replace-with: «Every CLI verb, `list`, `run` and `revive` included, and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers; its use written as built and NOT ratified either, a delegate's answer) (`hook` keeps its own shape check), resolves its home through `resolved_home`»

- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «they borrow none of the six.»
  source: .andromeda/security-plan.md:182 «until the same two entries land, `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the same liveness-only pre-check»
  replace-with: «they borrow none of the six. An eighth, the founder's own answer (P4 fork Q1, live, 2026-10-17, relayed by the overseer, `inputs#I2` of chunk 2026-10-17-mcp-server-for-drivers): `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the same liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `instance-unreachable`), until the same Epoch 6 entries; they borrow none of the seven.»

- leaf: .claude/rules/security.md:23   class: omission
  leaf-text: «Home resolution (every CLI verb, `run` and `revive` included; `hook` keeps its own shape check; chunk 2026-10-17-cli-machine-contract):»
  source: .andromeda/security-plan.md:203 «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers), resolves the home through `resolved_home`»
  replace-with: «Home resolution (every CLI verb, `run` and `revive` included, and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers; its use written as built and NOT ratified either, a delegate's answer); `hook` keeps its own shape check; chunk 2026-10-17-cli-machine-contract):»

- leaf: .claude/rules/security.md:31   class: omission
  leaf-text: «the tokio ban is `deny-sync.toml`, run per sync crate as sole root;»
  source: .andromeda/security-plan.md:295 «left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh`»
  replace-with: «the tokio ban is `deny-sync.toml`, run per sync crate as sole root over the four crates of `scripts/sync-crates.txt`, and for `viola-channel` (its Tokio client behind its `tokio` feature) the `cargo tree` form `scripts/channel-tokio-free.sh` (no `tokio v` line without the feature);»

- leaf: .claude/rules/security.md:32   class: stale
  leaf-text: «rmcp features exactly `server` + `transport-io`.»
  source: .andromeda/security-plan.md:273 «rmcp features exactly `server` and `transport-io` in `viola`'s release graph, checked by `scripts/rmcp-features.sh` in CI's lint job»
  replace-with: «rmcp features exactly `server` + `transport-io` in `viola`'s release graph (`scripts/rmcp-features.sh`, CI's lint job, rmcp present); the test-only `viola-e2e` takes rmcp `client` only, never `transport-child-process`.»

- leaf: .claude/rules/security.md:40   class: omission
  leaf-text: «channel `-32603` is exactly `"internal error"` / `data: null`.»
  source: .andromeda/security-plan.md:451 «Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null`»
  replace-with: «channel `-32603` is exactly `"internal error"` / `data: null`; MCP `isError` results carry only the closed `structuredContent` codes (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault | invalid-params`) with text `refused: <code>`, and any other MCP handler fault, unknown reply or panicked call is `-32603` `internal error` with `data: null`.»
