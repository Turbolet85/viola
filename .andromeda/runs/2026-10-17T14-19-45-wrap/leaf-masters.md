group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:27   class: stale
  leaf-text: «shared by `list`, `revive` and verify's Run B»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:36 «agents.rs: the bounded `claude agents --json` read (`read_logged`, `read_here`) shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`);»
  replace-with: «shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`)»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «wheel, the budget gate (`src/run/budget.rs`»
  source: .andromeda/architecture.md:301 «`mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`»
  replace-with: «the hidden `mcp` verb (`src/cmd/mcp.rs`: run by the plugin, never by a person, in no `viola --help` row; its home and `diagnostics/` through `check_path` before anything opens, exit 1 on a refusal, then `viola_mcp::serve` over stdio until stdin closes, `list::board_document` the `list` tool's provider), wheel, the budget gate (`src/run/budget.rs`»
- leaf: CLAUDE.md:75   class: omission
  leaf-text: «the tokio ban is `deny-sync.toml` per sync crate; `bash scripts/deny-probes.sh` proves every ban fires»
  source: .andromeda/architecture.md:43 «`viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list (since chunk 2026-10-17-mcp-server-for-drivers): its normal graph without the feature is proven tokio-free»
  replace-with: «the tokio ban is `deny-sync.toml` per sync crate of `scripts/sync-crates.txt`, and `viola-channel`'s normal graph without its `tokio` feature is `bash scripts/channel-tokio-free.sh`; `bash scripts/deny-probes.sh` proves every ban fires, the tree form included»
