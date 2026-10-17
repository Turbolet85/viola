
## 2026-10-17-mcp-server-for-drivers — the tokio ban's tree form and the rmcp release-graph gate
**Section:** Dependency Security (the arch bans bullet); Threat Model Summary (Trust boundary; CI jobs); API Security (MCP transport row)
**Change:**
- Dependency Security: the sole-root `deny-sync.toml` tokio ban runs over the four crates of `scripts/sync-crates.txt`; `viola-channel` (its Tokio client, tokio `=1.53.1` `io-util`, behind its `tokio` feature) left the list and is covered by `scripts/channel-tokio-free.sh` (no `tokio v` line without the feature, one with its control), proven by `deny-probes.sh`'s `channel-tokio-edge` probe and control (13/13).
- Trust boundary: names both tokio-ban forms.
- CI jobs: lint adds the per-OS `rmcp features` step (`scripts/rmcp-features.sh`); supply-chain runs the tree form after the sole-root loop.
- MCP transport: was an unscoped "rmcp features exactly `server` and `transport-io`"; now scoped to `viola`'s release graph and checked by `scripts/rmcp-features.sh` (rmcp present, exactly those two); the test-only `viola-e2e` takes rmcp `client` only, never `transport-child-process`.
**Why:** the ban's invariant holds in a new form: no listed sync crate depends on `viola-channel`, so the sole-root loop could not cover it (premise corrected by measurement). New dependencies tokio `=1.53.1` and schemars `=1.2.2` are exact pins; rmcp's range and features match the bans.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
