@apply_row n=1 ref_file=fanout-test-plan--cascade-g6.md ref_n=13 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g6.md ref_n=14 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g6.md ref_n=15 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g6.md ref_n=16 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g6.md ref_n=17 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g6.md ref_n=18 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan--cascade-g6.md ref_n=19 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan--cascade-g6.md ref_n=20 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan--cascade-g6.md ref_n=21 v=refused
why: a hand removal · fct-fumv4pch
@apply_row n=10 ref_file=fanout-test-plan--cascade-g6.md ref_n=22 v=refused
why: a hand removal · fct-5iefujrn
@apply_row n=11 ref_file=fanout-test-plan--cascade-g7.md ref_n=1 v=applied edit=E9
@apply_row n=12 ref_file=fanout-test-plan--cascade-g7.md ref_n=2 v=folded_into to_file=fanout-test-plan--cascade-g7.md to_n=1
@apply_row n=13 ref_file=fanout-test-plan--cascade-g7.md ref_n=3 v=folded_into to_file=fanout-test-plan--cascade-g7.md to_n=1
@apply_row n=14 ref_file=fanout-test-plan--cascade-g7.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g7.md to_n=1
@apply_row n=15 ref_file=fanout-test-plan--cascade-g7.md ref_n=5 v=folded_into to_file=fanout-test-plan--cascade-g7.md to_n=1
@apply_row n=16 ref_file=fanout-test-plan--cascade-g7.md ref_n=6 v=folded_into to_file=fanout-test-plan--cascade-g7.md to_n=1
@apply_row n=17 ref_file=fanout-test-plan--cascade-g7.md ref_n=7 v=applied edit=E10
@apply_row n=18 ref_file=fanout-test-plan.md ref_n=53 v=applied edit=E11
@apply_row n=19 ref_file=fanout-test-plan.md ref_n=55 v=applied edit=E12
@fact id=fct-pk32flwi expect_rev=1
old: clean`). `cargo modules dependencies --acyclic` is an on-demand boundary review, not a gate: at cargo-modules 0.27.0 every type and its own inherent method read as a cycle whatever the filters (measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes, 3 of 4 targets). The rmcp release-graph assertion is the lint step `rmcp features (exactly server + transport-io, non-vacuous)` (every OS, after `Lint probes`; chunk 2026-10-17-mcp-server-for-drivers): `bash scripts/rmcp-features.sh` runs `cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth`, rmcp must be present and the features a non-rmcp package requests exactly `server transport-io`, last line `rmcp-features: server transport-io`.
new: clean`).
old: Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
new: `cargo modules dependencies --acyclic` is an on-demand boundary review, not a gate: at cargo-modules 0.27.0 every type and its own inherent method read as a cycle whatever the filters (measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes, 3 of 4 targets).
old: Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`); it reproduced CI's `lint (macos-latest)` error line for line, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/evidence/operator-pass.md`: neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
new: The rmcp release-graph assertion is the lint step `rmcp features (exactly server + transport-io, non-vacuous)` (after `Lint probes`; chunk 2026-10-17-mcp-server-for-drivers): `bash scripts/rmcp-features.sh` runs `cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth`, rmcp must be present and the features a non-rmcp package requests exactly `server transport-io`, last line `rmcp-features: server transport-io`.
old: **Parallel:** per OS matrix
new: **Parallel:** one-leg matrix `[ubuntu-latest]` (the job name keeps its `${{ matrix.os }}` form)

@fact after=fct-7d3yogoz as=n1
- **The recorded supply-chain red:** the Supply-chain stage's `supply-chain` job reads red on one advisory, RUSTSEC-2026-0441 (`pin-utils` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1`), as measured at ci#42030025050; owner: the CARRY on «Version done-check», its remedy moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`; the ignore list is not widened.

@fact id=fct-4otx4vxd expect_rev=1
old: `scripts/agent-run.sh run --unit` (pwsh `scripts/agent-run.ps1 run --unit` on windows)
new: `scripts/agent-run.sh run --unit`

@fact id=fct-ijfdjbph expect_rev=1
old: `agent-run run --e2e` on all 3 OSes
new: `agent-run run --e2e` on `ubuntu-latest`
old: **Parallel:** matrix per OS
new: **Parallel:** one-leg matrix `[ubuntu-latest]`

@fact id=fct-sd3lvsrr expect_rev=1
old: In CI this is the per-OS test job; on `windows-2025` its `ConPTY vendor verification` step runs first (`bash scripts/conpty-vendor.sh --verify`: the committed `vendor/conpty/1.24.260710001/x64/` bytes equal the pinned nupkg and carry the Microsoft signer, last line `conpty-vendor: verified <version>`; then `--probe`: four refusals and a control, last line `conpty-vendor probe: 4/4 refused, control clean`), and the coverage run covers the `conpty_sideload` integration binary (`[[test]]`, required-features `fake-agent`) with the rest. Its one
new: In CI this is the `test` job (matrix `[ubuntu-latest]`). Its one
old: the per-OS test job uses `coverage,doctest`
new: the `test` job uses `coverage,doctest`
old: **Parallel:** per OS
new: **Parallel:** one-leg matrix `[ubuntu-latest]`

@fact id=fct-ixlw7x3t expect_rev=1
old: §10 Mutation gate). The audit's Windows leg is `windows-mutants.yml`: `workflow_dispatch` only, no inputs, ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item (the count of its `- package:` items; the root package `viola` is five of them, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs` since chunk 2026-10-13-interrupted-verify-cleanup), each running `scripts/agent-run.ps1 run --mutants --package <member> --file …` over that item's Windows-gated files, report-only, no cache or upload
new: §10 Mutation gate).

@fact id=fct-k7t43itf expect_rev=1
old: Every workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`) has
new: Every workflow (`ci.yml`, `nightly.yml`) has
old: cargo-deny@0.20.2`; `windows-mutants.yml` installs `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0` with the same action, and `tests/contract_windows_mutation_scope.rs` asserts those two pins equal the `test` job's `tool:` line.
new: cargo-deny@0.20.2`.
old: No workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`) has
new: No workflow (`ci.yml`, `nightly.yml`) has
old: (3 low, one per workflow; zizmor 1.30.1 on the dev host, 2026-10-04)
new: (2 low, one per workflow; zizmor 1.30.1, as measured at chunk 2026-11-05-linux-only-code-and-ci, implement entry 22)

@fact id=fct-7d3yogoz expect_rev=1
old: over 27 binaries
new: over 26 binaries

@fact id=fct-ocdr3xdn expect_rev=1
old: (the run's own output: under the host mutation scratch on a Windows host, §3 `run` step 4)
new: (the run's own output at cargo-mutants' default place, the harness passing no `--output`; §3 `run` step 4)
old: inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)`
new: inside a `cfg` item or block proved false for the host, or behind a const set by `cfg!`
old: (`crates/viola-e2e/src/harness/run/mutants/scratch.rs`). `prepare` takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`), so both arms are reached on every host, and both mutants
new: (`crates/viola-e2e/src/harness/run/mutants/scratch.rs`, deleted with the Windows scratch wipe and recoverable at the tag `multi-os-final`), and both mutants
old: (killed on every host since, above)
new: (read killed on the Linux dev host, above)
old: The boundary audit measures the `cfg(windows)` code by dispatching `windows-mutants.yml` (founder ruling C2, 2026-10-04; §9 Mutation row): `run --mutants --package <member> --file …` per matrix item on `windows-2025` (§9 Mutation row), verdict `package`, report-only and never a gate.
new: The boundary audit has no Windows leg: `windows-mutants.yml` and the `cfg(windows)` code it measured on `windows-2025` are out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
old: on the founder's own word for it; the ruling stands as written.
new: on the founder's own word for it; no Windows dispatch stands, Viola being Linux-only.
old: (an environment fact; the harness's scratch arm stays Windows-only)
new: (an environment fact; the harness sets no scratch dir of its own)
old: are compiled out on macOS (`#[cfg(not(target_os = "macos"))]`, never `#[ignore]`): there cargo-mutants' baseline build in its copied tree takes ~77 s of an 80.9 s phase against 0.41 s for the same cold build outside it, with no first-launch or jobserver cost, as measured at CI run 36481260151 (the chunk 2026-09-28-mutation-testing-to-the-epoch-boundary's `evidence/macos-mutants-phases.md`); the cause inside that phase stays unnamed, so under the operator's ruling it is runner-side. Both run on ubuntu and windows;
new: run ungated, never `#[ignore]`;

@fact id=fct-ftncprbj expect_rev=1
old: each a real check on Unix only
new: each a real check on every build

@fact id=fct-piiirk6o expect_rev=1
old: wire `scripts/agent-run.{sh,ps1}` as shims over
new: wire `scripts/agent-run.sh` as the shim over
old: above. Binding contract: both shell variants must expose identical semantics.
new: above.

@fact id=fct-2j6rht5l expect_rev=2
old: the twenty-seven verify-driven binaries
new: the twenty-six verify-driven binaries
old: cli_wheel conpty_sideload contract_fake_agent_drift
new: cli_wheel contract_fake_agent_drift
old: in `viola-e2e`: rmcp `>=3.4.1, <3.5` (resolved 3.4.1) with `client` only (no `transport-child-process`: the test owns the `viola mcp` child, chunk 2026-10-17-mcp-server-for-drivers), tokio `=1.53.3` with default features off and `rt` + `process` (no `test-util`); since chunk 2026-10-10-windows-mutation-grade also syn
new: in `viola-e2e` also syn
old: libc `=0.2.189` under `cfg(unix)` (
new: libc `=0.2.189`, a plain dependency (
old: windows-sys 0.61.2 with `Win32_Security_Authorization` (cfg windows)
new: in `viola-e2e`: rmcp `>=3.4.1, <3.5` (resolved 3.4.1) with `client` only (no `transport-child-process`: the test owns the `viola mcp` child, chunk 2026-10-17-mcp-server-for-drivers), tokio `=1.53.3` with default features off and `rt` + `process` (no `test-util`)
@pattern id=tp6-01
regex: check-only (Windows|macOS)-target clippy
@pattern id=tp6-02
fixed: agent-run.ps1 run --unit
@pattern id=tp6-03
fixed: on all 3 OSes
@pattern id=tp6-04
fixed: ConPTY vendor verification
@pattern id=tp6-05
fixed: The audit's Windows leg is
@pattern id=tp6-06
fixed: `nightly.yml`, `windows-mutants.yml`)
@pattern id=tp6-07
fixed: over 27 binaries
@pattern id=tp6-08
fixed: under the host mutation scratch on a Windows host
@pattern id=tp6-09
fixed: HOST_SCRATCH = cfg!(windows)
@pattern id=tp6-10
fixed: so both arms are reached on every host
@pattern id=tp6-11
fixed: The boundary audit measures the `cfg(windows)` code
@pattern id=tp6-12
fixed: scratch arm stays Windows-only
@pattern id=tp6-13
fixed: compiled out on macOS
@pattern id=tp6-14
fixed: a real check on Unix only
@pattern id=tp6-15
fixed: both shell variants must expose identical semantics
@pattern id=tp6-16
fixed: twenty-seven verify-driven binaries
