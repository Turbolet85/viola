
## 2026-10-17-mcp-server-for-drivers — the CI gates: the channel tree form, rmcp features, deny-probes 13/13
**Section:** §9 CI Integration (Pipeline structure, Lint and Supply-chain rows; Pipeline fails on, Lint and Supply-chain errors)
**Change:**
- Lint row: the sync-crate `cargo check` target set is four (`viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`) — was five with `viola-channel`; was "the rmcp release-graph assertion … has no subject until `viola-mcp` exists"; now the per-OS step `rmcp features (exactly server + transport-io, non-vacuous)` runs `bash scripts/rmcp-features.sh` after `Lint probes`, last line `rmcp-features: server transport-io`.
- Supply-chain row: after the sole-root loop, `bash scripts/channel-tokio-free.sh` in the same step (no `tokio v` line without the feature, read from the printed line never the exit; its control one; `channel-tokio-free: ok`); `deny-probes.sh` ends `13/13 banned, control clean` — was `12/12`: the twelve bans plus `channel-tokio-edge` with its channel control.
- Fails on: a lint error adds a red `rmcp-features.sh`; a supply-chain error adds a red `channel-tokio-free.sh`, and the deny-probes line reads 13/13.
**Why:** the chunk landed both scripts and the probe; `viola-channel` left the sole-root list because no listed crate depends on it.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
