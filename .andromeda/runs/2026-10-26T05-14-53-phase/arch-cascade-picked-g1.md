# arch · picked accounting · group 1 of 2

## Picked
stack-and-technologies#1.1 · carried · the 1.96 floor the `msrv` job checks is a bound the bumped tokio's own rust-version must stay inside
stack-and-technologies#1.2 · carried · the Stack row names the Tokio version the chunk moves and the containment it must keep
stack-and-technologies#1.3 · not in scope · clap sits outside tokio's resolved graph, so its pin is another `Cargo.toml` row that only keeps holding, a re-check every change owes
stack-and-technologies#1.4 · carried · portable-pty `=0.8.1` is the RUSTSEC-2017-0008 source that stays the only ignore, and libc `=0.2.189` / windows-sys 0.61.2 are pins tokio's own graph shares, which the new lock must resolve under unmoved
stack-and-technologies#1.5 · not in scope · vt100 sits outside tokio's graph and no readiness behaviour changes; its pin only keeps holding
stack-and-technologies#1.6 · carried · interprocess's Tokio flavour depends on tokio, its version must not move, and its feature-gated use by `viola-mcp` alone is the containment the bump keeps
stack-and-technologies#1.9 · not in scope · tempfile and the state-file primitives are untouched and outside tokio's graph; the shared windows-sys pin is held by the PTY row's item
stack-and-technologies#1.11 · not in scope · tokio is built with `default-features = false` and pulls no serde; the serialization pins only keep holding
stack-and-technologies#1.13 · not in scope · nutype newtypes are outside tokio's graph and untouched
stack-and-technologies#1.14 · not in scope · chrono reaches the tree through rmcp, whose pin does not move; no timestamp behaviour changes
stack-and-technologies#1.15 · not in scope · thiserror and anyhow are outside tokio's graph and no error type changes
stack-and-technologies#1.18 · carried · rmcp requires Tokio, so the new tokio must resolve under rmcp 3.4.1 unchanged, with `scripts/rmcp-features.sh` re-read
stack-and-technologies#1.21 · not in scope · the row rules the install route (no container, `cargo install --path .`, plugin embedded), which the chunk neither edits nor re-reads
stack-and-technologies#1.22 · carried · the chunk's acceptance is the three-OS CI run, the verdict of record, with the pre-push gate a filter only
stack-and-technologies#1.23 · carried · clippy's `-D warnings` reads the advisory's deprecations, and cargo-deny 0.20.2 with the per-sync-crate tokio ban is the gate the chunk re-runs
established-decisions#1.1 · carried · the Tokio-only-in-`mcp` keystone and its CI enforcement (sole-root ban, `cargo check` on three OSes, `scripts/channel-tokio-free.sh`) are what the bump must leave unchanged and re-run
established-decisions#2.1 · carried · the portable-pty `=0.8.1` pin is the source of the one advisory ignore, which must stay the only one
established-decisions#3.1 · not in scope · the screen model's readiness gate is untouched and vt100 is outside tokio's graph
established-decisions#6.1 · carried · its rationale records `deny.toml`'s licence allow list refusing a BSD-3-Clause crate, the allow list the bumped crates must stay inside when `cargo deny check` is re-run
established-decisions#7.1 · carried · interprocess 2.4.4 with its sync and Tokio flavours stays at its version while the tokio its Tokio flavour builds on moves
established-decisions#8.1 · carried · rmcp's pinned minor is the constraint the new tokio must resolve under, rmcp unchanged
established-decisions#19.1 · carried · only `viola-mcp` enabling tokio among the product crates and `fuzz/` being its own workspace are the graph the bump must keep
conventions#3.1 · not in scope · the chunk changes no MCP result shape, error word or `tools/list` field
occupied-resources#2.1 · carried · test-only binaries never in a release build is what the re-run `scripts/release-check.sh` re-asserts after the lock moves
occupied-resources#11.1 · not in scope · the two fake-agent test seams are untouched and the bullet states nothing about a dependency's environment read
