# Instrument controls — scripts/rmcp-features.sh and scripts/channel-tokio-free.sh

Read 2026-10-17T11:47:31Z on the Linux dev host, before either script's first graded use (plan §Test Commands).
Each input is a scratch copy of the working tree (`git ls-files -co --exclude-standard`, without `.andromeda/` and
`viola-0.1.0/`) under the session scratchpad, `$TMPDIR/{scratchpad}/ctl-<input>/`, the script run from the copy's own
`scripts/` with the copy as cwd. One planted change per must-fail input, nothing else.

## rmcp-features.sh

| Input | Planted | Exit | Verdict line (verbatim) | Known verdict |
|---|---|---|---|---|
| pass | none (the tree as built) | 0 | `rmcp-features: server transport-io` | pass ✓ |
| extra | root `Cargo.toml` rmcp pin: `features = ["server", "transport-io", "client"]` | 1 | `rmcp-features: FAILED — rmcp features requested: client server transport-io; expected: server transport-io` | fail ✓ |
| absent | root `[dependencies]` line `viola-mcp = { path = "crates/viola-mcp" }` deleted | 1 | `rmcp-features: FAILED — rmcp is absent from viola's release graph` | fail, vacuity guard ✓ |

## channel-tokio-free.sh

| Input | Planted | Exit | Verdict line (verbatim) | Known verdict |
|---|---|---|---|---|
| pass | none (the tree as built) | 0 | `channel-tokio-free: ok` (reading: `tokio absent; with --features tokio: tokio present`) | pass ✓ |
| tokio-edge | `crates/viola-channel/Cargo.toml`: `tokio` a normal dependency, the feature `tokio = ["interprocess/tokio"]` | 1 | `channel-tokio-free: FAILED — viola-channel reaches tokio without its feature` (reading: `tokio present; with --features tokio: tokio present`) | fail ✓ |

`scripts/deny-probes.sh` carries the same pair for the tree form in throwaway workspaces (`channel-tokio-edge`
banned, `channel control` clean): its first run read `deny-probes: 13/13 banned, control clean`, exit 0.
