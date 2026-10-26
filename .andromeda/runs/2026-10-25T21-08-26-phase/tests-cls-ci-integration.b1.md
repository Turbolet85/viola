# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · fmt --check, clippy -D warnings and orphans-check over the Rust the chunk edits and adds, its cfg(windows) code also under the Windows-target clippy gate
ci-integration#1.2 · yes · the chunk's CI runs read supply-chain red on the tokio advisory (the task's Needs), so no ignore may be added to reach green; viola-channel, which the chunk edits, must stay tokio-free under channel-tokio-free
ci-integration#1.3 · yes · the unit run (nextest process-per-test; pwsh on Windows) runs the edited win.rs unit tests and the edited case_4 root unit test, whose fix must hold in parallel without single-threading
ci-integration#1.4 · yes · the parallel integration run carries the reached security_negatives_channel.rs, cli_instance_state.rs and path1_run_start.rs and the Windows DACL read-back driver
ci-integration#1.5 · yes · the E2E run on all three OSes is the witness for the Path 1 squatted-endpoint leg (exit 1) the chunk re-verifies in path1_run_start.rs, on the windows-2025 leg too
ci-integration#1.6 · yes · the per-OS test job's instrumented nextest run measures the chunk's windows-2025 tests and its #[cfg(windows)] coverage; the ConPTY verification step there is not the chunk's
ci-integration#1.7 · no · the chunk's Not-needed list drops the mutation gate: it runs only at the epoch boundary or on demand, never per chunk; the Windows mutants jobs are dispatch-only and report-only
ci-integration#1.8 · yes · the perf job's G2 zero-panics check reads the product code the chunk adds, so new code carries no panic sites; the hyperfine hook gates are not reached (no hook path changes)
ci-integration#1.9 · yes · the msrv job runs cargo check --workspace and the unit suite (case_4 included) on rustc 1.96, so the chunk's code and case_4 fix must compile and pass on the floor
ci-integration#1.10 · no · fuzz-replay replays parser corpora and the chunk changes no parser (the Not-needed list drops the property and fuzz suite)
ci-integration#1.11 · no · release-check judges the release build's own artifacts; the chunk adds no binary and no test-only feature, so it is a re-check every change owes
ci-integration#1.12 · yes · viola-harness gate exits non-zero on any §10 breach, so the per-OS coverage gate (lines 85, functions 95, regions 80) binds the chunk's #[cfg(windows)] code on the windows-2025 job
