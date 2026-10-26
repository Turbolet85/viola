# Classified — architecture · established-decisions.b3

## Answers
established-decisions#17.1 · no · liveness rests on the heartbeat, pid and start time, which the tokio pin does not reach and no source changed
established-decisions#18.1 · no · the report re-reads thiserror 2.0.20 and anyhow 1.0.104 as unmoved on the changed Cargo.lock (the 4-line tokio-only delta) and no error enum or variant changed
established-decisions#19.1 · no · the report re-reads the member list, the fuzz exclude and the tokio feature holders as unmoved on the changed root Cargo.toml and Cargo.lock, so only viola-mcp still enables tokio and the sole-root check exits 0
established-decisions#20.1 · no · the report re-reads the root bin, CARGO_PKG_VERSION and the Deployment and CI decisions as unmoved on the changed Cargo.toml, and no workflow, release check or gate changed
established-decisions#21.1 · no · no plugin file, `.mcp.json` registration or marketplace command changed, and the tokio pin reaches none of that text
established-decisions#22.1 · no · no refusal, exit code or `--json` shape changed, so the CLI conventions still hold
