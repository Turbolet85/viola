# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · no · a re-check every change owes: fmt, clippy -D warnings, the per-crate cargo check and orphans-check hold for any change; the chunk adds no lint target or sync crate
ci-integration#1.2 · no · a re-check every change owes: the chunk names no dependency change, so cargo deny, the tokio ban and zizmor hold as for any change
ci-integration#1.3 · no · a re-check every change owes: each new unit test keeps its suite and process-per-test isolation as every test does; the scope names no unit-level instance
ci-integration#1.4 · no · standing S2 · integration binaries are named with their layer prefix so the filterset selects them; the scope names no integration instance this row must record
ci-integration#1.5 · yes · the two-instance harness-boot witness and its single-instance control are E2E-rig tests, so the per-OS E2E matrix is where they run and the scope's Windows exe-arm reading (S8) is read on the windows-2025 leg
ci-integration#1.6 · yes · the per-OS instrumented nextest run is where the witness runs on each OS, including the windows-2025 leg that reads the exe arm; its ConPTY vendor step is not needed here
ci-integration#1.7 · no · mutation is judged at the epoch boundary through the code audit, not per chunk (the scope's Not needed)
ci-integration#1.8 · no · perf measures the per-hook spawns, and the chunk changes no hook, statusline or budget path (the scope's Not needed)
ci-integration#1.9 · no · a re-check every change owes: the 1.96 floor and the unit run under it hold for any change, and the scope names no newer toolchain or API
ci-integration#1.10 · no · the chunk adds or changes no parser, so no fuzz target or corpus is reached (the scope's Not needed)
ci-integration#1.11 · yes · Measurement B re-reads release-check's build (--release, --locked, bin viola, fake-agent feature off, artifacts located by compiler-artifact record, never target/release/), and its two builds must match those
ci-integration#1.12 · no · standing S7 · per-OS coverage thresholds hold with the ignore regex unwidened; the gate is unchanged, and the witness verdict comes from logged events, receipts and exit codes, not junit
