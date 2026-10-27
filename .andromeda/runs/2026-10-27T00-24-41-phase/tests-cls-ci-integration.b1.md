# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the lint job runs on every OS, so any cfg(windows) line the fix adds to the reached pin.rs or fs.rs is clippy-checked only on the windows-2025 runner (S8: a Linux-only run is no proof)
ci-integration#1.2 · no · a re-check every change owes (cargo deny, the tokio ban, deny-probes, zizmor); the scope edits no Cargo manifest or workflow and names no new dependency
ci-integration#1.3 · no · the unit stage's suite and nextest process-per-test isolation are the convention every new unit test keeps; the per-OS run of #1.6 carries the same tests in CI
ci-integration#1.4 · no · standing S2 · integration binaries named with the layer prefix so nextest filtersets select them
ci-integration#1.5 · yes · the harness-boot race witness and the Path 1 and Path 7 start scenarios run on each OS of this matrix, and the Windows arm's reading is taken on the windows runner (S8's scope obligation)
ci-integration#1.6 · yes · the one instrumented per-OS run carries the witness and the Windows arm's CI-runner reading on windows-2025 (S8's scope obligation), and replaces the local Unit, Integration and E2E rows
ci-integration#1.7 · no · Not needed: the mutation gate is judged at the epoch boundary through the code audit, not per chunk
ci-integration#1.8 · no · a re-check every change owes (G2, schema and secret scans); perf gates hook latency and the chunk changes no hook path
ci-integration#1.9 · no · the 1.96 floor compile is a re-check every change owes; the scope names no toolchain-sensitive API
ci-integration#1.10 · no · Not needed: the chunk adds or changes no parser target, so the corpus replay has nothing new to hold
ci-integration#1.11 · yes · the scope re-reads this build's profile and CARGO_TARGET_DIR for Measurement B, and the entity rules over `cargo build --release --locked --bin viola` (feature off) judged on its own artifact records, never target/release/
ci-integration#1.12 · no · standing S7 · per-OS coverage thresholds hold, with the ignore regex unwidened
