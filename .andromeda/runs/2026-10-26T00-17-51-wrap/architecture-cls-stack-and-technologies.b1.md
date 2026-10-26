# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the toolchain pin, rust-version 1.96 and edition 2024 hold: Cargo.toml was edited only by the new Windows dev-dependency table, and rust-toolchain.toml is not a reached file
stack-and-technologies#1.2 · yes · add-to · F2: the pinned tokio 1.53.1 (its version and mcp-only containment unchanged) carries the unsound advisory RUSTSEC-2026-0386 and fails CI supply-chain, owned by the route entry "tokio RUSTSEC-2026-0386 lock bump"
stack-and-technologies#1.3 · no · clap 4.6.7 and its feature set hold: Cargo.toml's only edit is the new Windows dev-dependency table, and src/main.rs gains only a #[cfg(test)] module, so no product dispatch changed
stack-and-technologies#1.4 · no · portable-pty =0.8.1 and the windows-sys 0.61.2 pin and product features hold: the pin is unchanged, and the two new dev-only features (Win32_System_IO, Wdk_Storage_FileSystem) are named nowhere in this row
stack-and-technologies#1.5 · no · vt100 0.16.2 and run's feed-thread readiness and modal detection hold: the reached files change only test code (the nine captures behind a #[cfg(test)] helper) and no product behaviour moved
stack-and-technologies#1.6 · no · the Windows client open flags, the SDDL-converted pipe DACL and the identify-never-impersonate claim are unchanged, and the session's witnesses (remote-client rejection, anonymous-token denial) are not stated by this row
stack-and-technologies#1.7 · no · no database exists or is added, and viola-state, where the ndjson and snapshot stores live, is unchanged
stack-and-technologies#1.8 · no · the v-field, unknown-field and snapshot-rebuild rules are viola-state formats, and viola-state is unchanged
stack-and-technologies#1.9 · no · the state-file primitives and the home DACL creation are viola-state code, which is unchanged, and the session's DACL witnesses cover only the endpoint pipe
stack-and-technologies#1.10 · no · sha2 =0.11.0 and the root dev-dependency its known-answer test uses both hold: Cargo.toml's only edit is the new Windows dev-dependency table
stack-and-technologies#1.11 · no · serde 1.0.229, serde_json 1.0.151 and the preserve_order feature hold: no version moved and Cargo.toml's only edit is the new Windows dev-dependency table
stack-and-technologies#1.12 · no · tracing, tracing-subscriber's dependents and the one-line JSON diagnostics hold: no product log line moved, src/run/gate.rs's one set_global_default is untouched, and every captured test line still goes through obs_event!
stack-and-technologies#1.13 · no · the nutype newtypes live in viola-core, which the session did not edit
stack-and-technologies#1.14 · no · chrono 0.4.45 comes into the tree through rmcp, and no version or use changed
stack-and-technologies#1.15 · no · thiserror per crate and anyhow at the bin edge hold: the root bin's only change is a #[cfg(test)] module in src/main.rs and no dependency version was added or bumped
stack-and-technologies#1.16 · no · the row names per-wrapper local sockets and no broker, and the session adds no broker, socket or wire change (its new pipes are test-owned control pipes)
stack-and-technologies#1.17 · no · sysinfo 0.39.6 and the claude agents join sit in the liveness contract and the board read model, both unchanged, and in no reached file
stack-and-technologies#1.18 · no · rmcp 3.4.1 and its release-graph features hold: viola-mcp is unchanged and Cargo.toml's only edit is the new Windows dev-dependency table
stack-and-technologies#1.19 · no · N/A row, and the session adds no model API call
stack-and-technologies#1.20 · no · N/A row, and the session adds no phone view
stack-and-technologies#1.21 · no · cargo install --path . and the include_str! plugin embedding hold: plugin/ is unchanged and the new Windows table is dev-only
stack-and-technologies#1.22 · no · the matrix, toolchain installs and SHA-pinned actions hold: no CI step, script, harness command or status shape changed, and the new cases run inside the existing test (windows-2025) step
stack-and-technologies#1.23 · no · the tool roles hold: cargo-deny's advisory red belongs to the Tokio row, and every captured test line still goes through obs_event!, so the tracing-macro ban is unaffected
stack-and-technologies#1.24 · no · dist and cargo-auditable are for a future release, and no release build or distribution artifact changed
