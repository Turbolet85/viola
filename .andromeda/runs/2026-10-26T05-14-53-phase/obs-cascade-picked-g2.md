# obs · picked accounting · group 2 of 2

## Picked
ci-integration#1.5 · not in scope · the N/A OTel trace-export row; the bump adds no exporter and no exporter crate enters the graph, which is the `cargo deny` ban row's to witness, not this row's
ci-integration#2.1 · carried · the Lint / typecheck row names the three checks the chunk re-runs: clippy `-D warnings` (the `AsyncFd` deprecation), `cargo check` of sync crates without Tokio, and `cargo deny check` with its bans
ci-integration#2.2 · carried · the workspace tests the chunk re-runs include the in-memory schema checks that witness no log line changing
ci-integration#2.4 · carried · the full CI row read at the wrap reads the E2E stage's G2 and G4 gate exit codes on the bumped tokio
ci-integration#3.3 · carried · G4's step and scope are a gate of that CI row the chunk re-reads; G1 and G3 rule over other units of Cargo.toml than the tokio pin row, a re-check every change owes
obs-anti-patterns#1.1 · carried · the print bans are enforced by the clippy step the chunk re-runs, over the `mcp` code path where Tokio lives and where a forced `AsyncFd` edit would land
