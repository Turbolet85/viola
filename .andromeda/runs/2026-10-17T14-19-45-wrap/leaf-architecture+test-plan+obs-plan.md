group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:55   class: stale
  leaf-text: «`cargo tree -e features -p viola --edges normal` — the rmcp `server` + `transport-io` assertion; it lands with the "MCP server for drivers" chunk (no rmcp in the graph before `viola-mcp`)»
  source: .andromeda/test-plan.md:1021 «The rmcp release-graph assertion is the lint step `rmcp features (exactly server + transport-io, non-vacuous)` (every OS, after `Lint probes`; chunk 2026-10-17-mcp-server-for-drivers)»
  replace-with: «`bash scripts/rmcp-features.sh` — the rmcp release-graph assertion (CI lint step `rmcp features (exactly server + transport-io, non-vacuous)`, every OS, after `Lint probes`): runs `cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth`; rmcp must be present and the features a non-rmcp package requests exactly `server transport-io` (last line `rmcp-features: server transport-io`)»

- leaf: .claude/docs/commands.md:60   class: stale
  leaf-text: «(last line `deny-probes: 12/12 banned, control clean`; needs the network)»
  source: .andromeda/test-plan.md:1022 «last line `deny-probes: 13/13 banned, control clean`: the twelve bans one probe each, plus probe `channel-tokio-edge` — a throwaway workspace with a normal tokio edge must fail the tree form»
  replace-with: «(last line `deny-probes: 13/13 banned, control clean`: the twelve bans one probe each, plus probe `channel-tokio-edge` for the `channel-tokio-free.sh` tree form with its channel control; needs the network)»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «the tokio ban, once per crate in `scripts/sync-crates.txt` as sole root (cargo-deny 0.20: `--config` goes before the subcommand; `check -c` is rejected)»
  source: .andromeda/test-plan.md:1022 «then, in the same step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh` (chunk 2026-10-17-mcp-server-for-drivers: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line»
  replace-with: «the tokio ban, once per crate in `scripts/sync-crates.txt` as sole root (cargo-deny 0.20: `--config` goes before the subcommand; `check -c` is rejected); then, in the same CI step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh`: `viola-channel` (off the list since chunk 2026-10-17-mcp-server-for-drivers, its Tokio client behind its `tokio` feature) must print no `tokio v` line from `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none`, its `--features tokio` control must print one (verdict `channel-tokio-free: ok`)»

- leaf: .claude/docs/commands.md:22   class: omission
  leaf-text: «no `viola ui` subcommand: the frontend is deferred to 0.2.0; its technology is not chosen»
  source: .andromeda/architecture.md:301 «`mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`»
  replace-with: «no `viola ui` subcommand: the frontend is deferred to 0.2.0; its technology is not chosen. `viola mcp [--home <dir>]` — the stdio MCP server (tools `send · wait · last · answer · list`), hidden from `viola --help` and run by the plugin, never by a person: no argument but the global `--home`, it serves until stdin closes, and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` fails `viola_state::strict::check_path`»
