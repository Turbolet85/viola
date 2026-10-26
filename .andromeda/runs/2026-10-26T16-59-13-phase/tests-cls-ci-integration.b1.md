# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the chunk's new Rust (the Windows DACL branch, the pin re-hash, the per-start plugin rewrite) must pass fmt, clippy -D warnings, the viola-state no-tokio cargo check and the orphans gate on every OS leg, and its cfg(windows) code is linted only by that CI leg, not by a Linux check-only clippy
ci-integration#1.2 · no · the chunk names no new dependency and no workflow edit, so cargo-deny, the tokio ban, deny-probes and zizmor judge nothing it changes
ci-integration#1.3 · yes · the chunk's viola-state unit cases (Unix modes, umask-independent owner-only modes, strict-modes, the Windows SDDL stubs at the check function) run in this stage, process-per-test under nextest on each OS, pwsh on Windows
ci-integration#1.4 · yes · the root integration binaries the chunk edits run here: tests/cli_controls_not_disableable.rs (the 0770 --home negative, the completeness case), the tests/hook_statusline.rs re-reads, and the tests/support/home.rs seed_conpty helper
ci-integration#1.5 · yes · the booted-wrapper cases run here on all three OSes: the per-surface strict-modes refusals, the tampered pinned exe exiting run 1, statusline_command not run on a failed home, and the Windows legs
ci-integration#1.6 · yes · the windows-2025 check_instance proof and the conpty_sideload path run in this per-OS coverage job, so the new Windows branches count against its per-OS gate (the scope names that leg)
ci-integration#1.7 · no · no push or pull-request job, and the mutation gate is judged only at the epoch boundary, never per chunk (the task's Not needed list)
ci-integration#1.8 · yes · the strict-modes check added to the spine hooks must keep session-start, user-prompt-submit, stop and pre-tool-use under SPINE_DEADLINE on hyperfine max (the scope's indirect need), and G2's zero-panic check binds the new product code
ci-integration#1.9 · yes · the chunk's new Unix unit cases and its code must compile and pass on the 1.96 floor (ubuntu: cargo check --workspace and agent-run run --unit under RUSTUP_TOOLCHAIN=1.96)
ci-integration#1.10 · no · the chunk changes no fuzzed parser (hook and agents parsers, ndjson framing, validate_paste_text, vt100 feed), and the cargo-fuzz targets are in the task's Not needed list
ci-integration#1.11 · yes · the reproducibility measurement compares two clean builds of this release route (cargo build --release --locked --bin viola, fake agent off), which the scope names as its comparison build
ci-integration#1.12 · yes · viola-harness gate reads the chunk's junit, the per-OS llvm-cov summary and the hyperfine JSON, so the new Windows branches' coverage and the spine-hook deadline both fail the job on a breach
