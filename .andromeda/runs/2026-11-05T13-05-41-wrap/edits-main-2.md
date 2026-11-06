@fact id=fct-vf6cfvel expect_rev=1
old: GitHub Actions, three workflows. `ci.yml`, triggered on push and pull request, with matrix `os: [windows-2025, macos-latest, ubuntu-latest]` on native runners, is the only push/PR pipeline.
new: GitHub Actions, two workflows. `ci.yml`, triggered on push and pull request, with matrix `os: [ubuntu-latest]`, is the only push/PR pipeline.
old: `windows-mutants.yml` (founder ruling C2, 2026-10-04) is triggered by `workflow_dispatch` only, with no `inputs:`, and is dispatched only during the epoch-boundary audit. It is never a gate, never a required check and never a dependency of `ci.yml`, and it adds no check to a push run. One dispatch was made outside that audit, on 2026-10-10, for chunk 2026-10-10-windows-mutation-grade alone and on the founder's own word for it (run 38036448183); the ruling stands as written. One job definition, named `mutants (<label>)`, runs on `windows-2025` with `contents: read`, `timeout-minutes: 120` and `fail-fast: false` over a ten-item `matrix.include` (counted as its `- package:` items; each item is a `package`, a `label` and Windows-gated `files`). The root package `viola` is five of the ten, split by file (`src/cmd/run.rs`; `src/main.rs` with `src/conpty.rs`; `src/run/env.rs`; `src/panic_frames.rs`; `src/cmd/verify/stop.rs`, label `viola-cmd-verify-stop`, added at chunk 2026-10-13-interrupted-verify-cleanup because the contract test below requires every product file with a `cfg(windows)` gate to be listed; nothing was dispatched), so that each job ends inside the ceiling. `tests/contract_windows_mutation_scope.rs` keeps the file lists equal to the gated sources, the workflow's tool pins equal to the `test` job's, and the labels distinct, one per item. Its steps are checkout, `rustup toolchain install` and install-action with `cargo-nextest@0.9.146,cargo-mutants@27.1.0`, then one `shell: pwsh` step that reads `PACKAGE` / `FILES` from step `env:` and runs `scripts/agent-run.ps1 run --mutants --package <member> --file …`. It has no cache, upload, secret, `concurrency:` or `needs:`. Its job log, the harness document plus cargo-mutants' outcome lines, is the audit's read, and the harness document judges a job. The harness leaves a missed mutant the Windows build never compiled (a `#[cfg(unix)]` twin in a scoped file) out of the count and names it in the document's `mutants.host_excluded` (test-plan §3 `run` step 4), while cargo-mutants' own lines still say MISSED for it. Run 38036448183 read all nine jobs green with 30 mutants left out, as measured at `viola-0.1.0/chunks/2026-10-10-windows-mutation-grade/evidence/windows-dispatch.md`.
new: The boundary audit has no Windows leg: `windows-mutants.yml` left with the Windows code, recoverable at the tag `multi-os-final`.
old: — mutation testing moved to the epoch-boundary code audit on 2026-09-28, and `windows-mutants.yml` uploads nothing — so
new: — mutation testing moved to the epoch-boundary code audit on 2026-09-28 — so

@fact id=fct-3nc6guvb expect_rev=1
old: Jobs wired today in `ci.yml` (7; 15 check-runs per push, as measured at ci#36483042659 on `17b93c7`): `test` (per OS; on `windows-2025` only, before the coverage run, the `ConPTY vendor verification` step runs `bash scripts/conpty-vendor.sh --verify` then `--probe` (verdicts `conpty-vendor: verified <version>` and `conpty-vendor probe: 4/4 refused, control clean`); then one harness `run --coverage` through the OS's shim, i.e. the instrumented nextest run as suite `coverage` with the per-OS floors 85/95/80,
new: Jobs wired today in `ci.yml` (7, every one green on ci#42030025050 but `supply-chain`, red on RUSTSEC-2026-0441 alone): `test` (on `ubuntu-latest`; one harness `run --coverage` through the `sh` shim (`scripts/agent-run.sh`), i.e. the instrumented nextest run as suite `coverage` with the floors 85/95/80,
old: `perf` (per OS, `fail-fast: false`,
new: `perf` (on `ubuntu-latest`, `fail-fast: false`,
old: (test-plan §10 Mutation gate), whose Windows leg is the dispatch-only `windows-mutants.yml` running `run --mutants --package <member> --file …` through the pwsh shim. `msrv`
new: (test-plan §10 Mutation gate). `msrv`
old: `lint` (per OS: target job 3,
new: `lint` (on `ubuntu-latest`: target job 3,
old: `scripts/lint-probes.sh` on Linux, then `rmcp features (exactly server + transport-io, non-vacuous)` on every OS (`bash scripts/rmcp-features.sh`,
new: `scripts/lint-probes.sh`, then `rmcp features (exactly server + transport-io, non-vacuous)` (`bash scripts/rmcp-features.sh`,
old: `release` (per OS, `fail-fast: false`,
new: `release` (on `ubuntu-latest`, `fail-fast: false`,

@fact id=fct-jykzrth4 expect_rev=1
old: Target jobs per OS, each wired
new: Target jobs, on `ubuntu-latest`, each wired
old: compile on each OS with no tokio;
new: compile with no tokio;

@fact id=fct-oqzfmod4 expect_rev=1
old: (`LedgerRow::relied()`: all twenty-four on the Unix runners, twenty-one on `windows-2025`, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`; `tests/cli_verify.rs`, harness `boot` step 4)
new: (`LedgerRow::relied()`: all twenty-four on every build; `tests/cli_verify.rs`, harness `boot` step 4)

@fact id=fct-ni3gbuvj expect_rev=1
old: and no mutation stage (test-plan §3 Internal harness subcommands). The other OSes are CI's alone. `run --mutants`
new: and no mutation stage (test-plan §3 Internal harness subcommands). `run --mutants`
