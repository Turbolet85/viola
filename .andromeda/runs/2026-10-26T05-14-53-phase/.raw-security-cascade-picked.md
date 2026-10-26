# security · picked accounting

## Picked
threat-model-summary#10.1 · carried · the supply-chain trust boundary it names (`cargo deny check` over `deny.toml`, the sole-root tokio ban, `scripts/channel-tokio-free.sh`, `Cargo.lock`) is what the chunk moves and re-runs
threat-model-summary#11.1 · carried · it states the ubuntu supply-chain job's steps, the rmcp features step, MSRV 1.96 and the release build through `scripts/release-check.sh`, all of which the chunk re-reads and re-runs
input-validation#1.9 · not in scope · rules over the `FAKE_AGENT_PUMP_DELAY_MS` seam's own parse and cap in `src/cmd/run.rs`, a unit the chunk neither edits nor re-reads
input-validation#1.10 · not in scope · rules over the `FAKE_AGENT_HOOK_PANIC` seam's own closed value and panic in `src/cmd/hook/seam.rs`, a unit the chunk neither edits nor re-reads
data-protection#1.1 · not in scope · rules over home file modes and the code-bearing artefacts (the `sha2` pin among them), none of which a tokio pin and lock move edits or re-reads
dependency-security#2.1 · carried · it states the tokio containment bans (`deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh`, `scripts/deny-probes.sh`) the chunk re-runs, and names the tokio pin the chunk moves
dependency-security#3.1 · carried · it states the fuzz lockfile's own audit step in the supply-chain job that must read green on the chunk's push, and the chunk re-reads `fuzz/Cargo.lock` as holding no tokio
dependency-security#8.1 · carried · the open-advisory line is the chunk's own subject: a tokio bump to `>=1.53.3` with no ignore-list widening
error-handling#1.1 · not in scope · rules over `viola hook`'s fail-open and stderr discipline, a path the chunk neither edits nor re-reads
security-anti-patterns#5.1 · not in scope · rules over `config.json`, `VIOLA_*` and flag switches and the env seams, none of which a dependency bump adds, reads or changes
