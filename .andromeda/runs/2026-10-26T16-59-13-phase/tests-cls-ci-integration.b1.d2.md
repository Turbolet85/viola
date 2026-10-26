# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the per-OS lint leg lints the chunk's cfg(windows) owner and DACL code on windows-2025 under -D warnings, where S9 says that OS's code is verified
ci-integration#1.2 · no · the reached files name no manifest, deny config or workflow, so the advisory, licence, source and tokio-ban gates read a dependency graph the chunk does not change
ci-integration#1.3 · yes · the chunk's viola-state unit cases for Unix modes and strict-modes run in this tier, on Linux and on Windows through pwsh
ci-integration#1.4 · yes · the chunk's integration cases (the 0770 and completeness cases of cli_controls_not_disableable.rs, hook_statusline.rs, and the seed_conpty home in tests/support/home.rs) run in this tier
ci-integration#1.5 · yes · the booted-wrapper and per-verb strict-modes cases run in the E2E tier on all three OSes, the Windows legs of the chunk's multi-OS proof
ci-integration#1.6 · yes · the per-OS test job on windows-2025 runs the chunk's Windows strict-modes and DACL proof and its three hook_statusline re-reads, and the conpty_sideload binary covers the Windows companion load
ci-integration#1.7 · no · the mutation gate is judged only at the epoch boundary, never per chunk (the task's Not needed list)
ci-integration#1.8 · yes · the hook's added strict-modes check is measured on hyperfine max against SPINE_DEADLINE by this job's gate --require perf
ci-integration#1.9 · no · the 1.96 MSRV floor is a toolchain check every Rust change owes equally, and the chunk's unit cases run on the main toolchain through the Unit tier
ci-integration#1.10 · no · the task's Not needed list excludes the cargo-fuzz targets and the parser properties, and the chunk edits no parser
ci-integration#1.11 · yes · the reproducibility measurement compares two clean release builds, and this job's --release route is one of the builds it compares against
ci-integration#1.12 · no · standing S8 · coverage gated per OS and zero flakiness
