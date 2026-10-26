# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the pinned 1.98.1 toolchain runs the chunk's fmt and clippy gates, and the 1.96 floor must still build the bumped tokio, the pin itself stays
stack-and-technologies#1.2 · yes · names Tokio 1.53.1 built only inside mcp, the version the chunk moves to >=1.53.3, and the containment rule the bump must keep
stack-and-technologies#1.3 · yes · clap 4.6.7 is named in the scope as a version that stays, so its pin and feature set with no colour crate must stay as locked
stack-and-technologies#1.4 · yes · portable-pty =0.8.1 is the source of the one advisory ignore the scope keeps, so its pin stays
stack-and-technologies#1.5 · yes · vt100 0.16.2 is named in the scope as a version that stays, so the bump must not move it
stack-and-technologies#1.6 · yes · interprocess 2.4.4 is named in the scope as a version that does not move, and its Tokio backend behind viola-channel's feature must resolve under the new tokio
stack-and-technologies#1.7 · no · no database, the chunk writes or reads no state file, so no store is reached
stack-and-technologies#1.8 · no · N/A ORM and migrations, no record or snapshot format changes
stack-and-technologies#1.9 · yes · tempfile 3.27.0 is named in the scope as a version that stays, so the bump must not move it
stack-and-technologies#1.10 · no · sha2 =0.11.0 is no version the scope names, and its pin is another Cargo.toml and Cargo.lock unit that must only keep holding, a re-check every change owes
stack-and-technologies#1.11 · yes · serde_json 1.0.151 is named in the scope as a version that stays, with its preserve_order feature
stack-and-technologies#1.12 · no · tracing, tracing-subscriber and veil are no version the scope names and the chunk adds no log line or tracing feature, a re-check every change owes
stack-and-technologies#1.13 · yes · nutype 0.8.0 is named in the scope as a version that stays, so the bump must not move it
stack-and-technologies#1.14 · yes · chrono 0.4.45 is named in the scope as a version that stays, so the bump must not move it
stack-and-technologies#1.15 · yes · thiserror 2.0.20 and anyhow 1.0.104 are named in the scope as versions that stay, so the bump must not move them
stack-and-technologies#1.16 · no · no broker, the per-wrapper JSON-RPC over ndjson transport is unchanged, and the tokio client's pin is bound through #1.2 and #1.6
stack-and-technologies#1.17 · no · sysinfo 0.39.6 is no version the scope names and liveness is untouched, a re-check every change owes
stack-and-technologies#1.18 · yes · rmcp 3.4.1 is named in the scope as a version that stays, and the new tokio must resolve under rmcp's pin with its server and transport-io features unchanged
stack-and-technologies#1.19 · no · N/A, viola calls no model API, so the bump reaches no AI serving
stack-and-technologies#1.20 · no · N/A, no mobile framework in v1
stack-and-technologies#1.21 · yes · the scope names deployment by cargo install --path . against the committed Cargo.lock, the shipped dependency set the bump changes
stack-and-technologies#1.22 · yes · the supply-chain job the chunk re-reads, whose green closes the red, runs under these CI pins (SHA-pinned actions and installs), and the three-OS matrix stays
stack-and-technologies#1.23 · yes · cargo-deny 0.20.2 is the tool the chunk re-runs with the tokio ban and the ignore list unchanged, clippy -D warnings is the gate that reads the AsyncFd deprecation, and the fuzz lockfile stays outside the root graph
stack-and-technologies#1.24 · no · v1.x distribution (dist, cargo-auditable, self_update) is out of scope under the Not needed section, and release-check judges artifact records, not dist
