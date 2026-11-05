@carried n=1 label=test-plan/ci-integration fact=fct-pk32flwi
why: The lint job must run on ubuntu-latest alone, and the dev-host Windows- and macOS-target check-only clippy recipes must leave with the code they checked.
quote: Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
@carried n=1 run=2 label=test-plan fact=fct-346brcpe
quote: **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, per OS; the `--acyclic` review and the dev-host Windows- and macOS-target check-only clippy runs are not gates.
@carried n=2 label=test-plan/ci-integration fact=fct-3gkvok74
why: With the Windows dependencies and their deny.toml entries removed, the supply-chain job must still pass cargo deny, the sole-root tokio ban, the channel tree form and the deny probes with their required last lines.
quote: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans, the bans holding a `[[bans.features]]` table that denies serde_json's `unbounded_depth`)
@carried n=2 run=2 label=test-plan fact=fct-ys3szg34
quote: Supply-chain errors: a cargo deny ban/advisory/licence/source finding, a `fuzz/Cargo.lock` advisory or source finding, a listed sync crate failing the sole-root `deny-sync.toml` tokio ban, `scripts/channel-tokio-free.sh` not ending `channel-tokio-free: ok`
@carried n=3 label=test-plan/ci-integration fact=fct-4otx4vxd
why: The pwsh shim step runs only on Windows CI, so the plan must decide whether agent-run.ps1 leaves with it as a harness step that runs only for that OS.
quote: `scripts/agent-run.sh run --unit` (pwsh `scripts/agent-run.ps1 run --unit` on windows)
@carried n=3 run=2 label=test-plan fact=fct-yqn2bvnu
quote: `scripts/agent-run.sh` (POSIX) and `scripts/agent-run.ps1` (PowerShell) are identical thin shims.
@carried n=4 label=test-plan/ci-integration fact=fct-ijfdjbph
why: The test matrix must shrink from three OSes to ubuntu-latest, and the 3-OS matrix wording is an expected amendment the wrap writes.
quote: `agent-run run --e2e` on all 3 OSes
@carried n=4 run=2 label=test-plan fact=fct-shyrfs6v
quote: OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal), `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs). `fail-fast: false`.
@carried n=4 run=3 label=test-plan fact=fct-q4zg6vp5
quote: Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
@carried n=4 run=4 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=5 label=test-plan/ci-integration fact=fct-sd3lvsrr
why: The windows-2025 ConPTY vendor verification step and the conpty_sideload integration binary must leave with vendor/conpty and the Windows job, while the ubuntu test job keeps its one instrumented run and its gate.
quote: In CI this is the per-OS test job; on `windows-2025` its `ConPTY vendor verification` step runs first (`bash scripts/conpty-vendor.sh --verify`: the committed `vendor/conpty/1.24.260710001/x64/` bytes equal the pinned nupkg and carry the Microsoft signer, last line `conpty-vendor: verified <version>`; then `--probe`: four refusals and a control, last line `conpty-vendor probe: 4/4 refused, control clean`), and the coverage run covers the `conpty_sideload` integration binary (`[[test]]`, required-features `fake-agent`) with the rest.
@carried n=5 run=2 label=test-plan fact=fct-onifqrhg
quote: **Coverage report:** `agent-run run --coverage` is the per-OS test job, on `windows-2025` after the ConPTY vendor verification
@carried n=5 run=3 label=test-plan fact=fct-dk3rsyuy
quote: ConPTY vendor errors (`windows-2025`): `scripts/conpty-vendor.sh --verify` not ending `conpty-vendor: verified <version>`, or its `--probe` not ending `conpty-vendor probe: 4/4 refused, control clean`
@carried n=5 run=4 label=test-plan fact=fct-ryyndnld
quote: Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
@carried n=5 run=5 label=test-plan fact=fct-3mj52n2x
quote: Excluded via the separator-agnostic `--ignore-filename-regex` `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`, the harness const `COVERAGE_IGNORE`.
@carried n=5 run=6 label=test-plan fact=fct-b4avhaux
quote: NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
@carried n=5 run=7 label=test-plan fact=fct-6iojik4n
quote: Coverage falls below threshold (Section 10) on any OS
@carried n=6 label=test-plan/ci-integration fact=fct-ixlw7x3t
why: windows-mutants.yml is removed whole, so the audit's Windows mutation leg must leave the Mutation row while mutation stays at the epoch boundary through run --mutants.
quote: The audit's Windows leg is `windows-mutants.yml`: `workflow_dispatch` only, no inputs, ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item
@carried n=6 run=2 label=test-plan fact=fct-fo2372wn
quote: **Mutation:** no push or pull-request job and no gate — mutation runs at the epoch boundary through `/andromeda-code-audit` and on demand through `run --mutants`; the audit's Windows leg, `windows-mutants.yml`, is dispatch-only and report-only.
@carried n=6 run=3 label=test-plan fact=fct-3x7gvt6f
quote: **Platform:** GitHub Actions (one `ci.yml` on push and PR, a scheduled `nightly.yml` for fuzz and weekly advisories, and the dispatch-only `windows-mutants.yml` for the boundary audit's Windows mutation leg).
@carried n=6 run=4 label=test-plan fact=fct-2ymlhohm
quote: `mutants.out/` is never uploaded (no push or pull-request job runs mutation since 2026-09-28, and the dispatch-only `windows-mutants.yml` uploads nothing)
@carried n=7 label=test-plan/ci-integration fact=fct-bfdvmti3
why: The perf job's Windows and macOS legs must leave, its ubuntu leg still ending in gate --require perf with its scan-gated uploads.
quote: Its own job `perf (${{ matrix.os }})` (`fail-fast: false`, `AGENT_RUN_KEEP_HOMES=1`)
@carried n=7 run=2 label=test-plan fact=fct-3q2uf724
quote: **Perf:** `agent-run run --perf` in its own per-OS job, its verdict from `viola-harness gate`, never from a `jq` binary.
@carried n=7 run=3 label=test-plan fact=fct-ap7wweiw
quote: the `perf` job's uploads, all behind that job's own secret scan: `perf-${{ matrix.os }}` (`target/agent-run/artifacts/perf-*.json`, the hyperfine exports) and `diag-perf-${{ matrix.os }}` (`target/e2e-home/**/diagnostics/*.ndjson`)
@carried n=8 label=test-plan/ci-integration fact=fct-whs3hmpm
why: The msrv job must be green again on Rust 1.96, its unit run passing the two e2e-home cases on a free-space figure the test controls rather than the runner's disk.
quote: ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`
@carried n=8 run=2 label=test-plan fact=fct-bpswkwdk
quote: **MSRV:** an ubuntu job checks the workspace and runs the unit suite on Rust 1.96, the security toolchain floor.
@carried n=8 run=3 label=test-plan fact=fct-zrh4qt5s
quote: If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk.
@carried n=9 label=test-plan/ci-integration fact=fct-2qyicniz
why: The release job's Windows and macOS legs must leave, its ubuntu leg still ending release-check's probe and verdict lines after the manifests change.
quote: ci.yml job `release` (`fail-fast: false`): `bash scripts/release-check.sh --probe` (last line `release-check probes: 5/5 refused, control clean`), then `bash scripts/release-check.sh`
@carried n=9 run=2 label=test-plan fact=fct-thnlptgc
quote: Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
@carried n=10 label=test-plan/ci-integration fact=fct-k7t43itf
why: With windows-mutants.yml and tests/contract_windows_mutation_scope.rs removed, the workflow list, the pin assertion and the one-per-workflow concurrency count must name the two remaining workflows only.
quote: `windows-mutants.yml` installs `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0` with the same action, and `tests/contract_windows_mutation_scope.rs` asserts those two pins equal the `test` job's `tool:` line.
@carried n=10 run=2 label=test-plan/ci-integration fact=fct-k7t43itf
quote: zizmor's pedantic `concurrency-limits` finding is declined and stays visible (3 low, one per workflow; zizmor 1.30.1 on the dev host, 2026-10-04).
@carried n=11 label=test-plan/ci-integration fact=fct-7d3yogoz
why: The ci profile's kill filter names binary(conpty_sideload), so removing tests/conpty_sideload.rs must drop it from the filter line and its recorded count.
quote: the `ci` profile's stamped-home override filter (`.config/nextest.toml:24`) is the 20 s kill (`slow-timeout = { period = "10s", terminate-after = 2 }`) over 27 binaries, the count the `binary(…)` names on that filter line
@carried n=12 label=test-plan/ci-integration fact=fct-fumv4pch
why: The per-OS JUnit reader downloads artifacts only the removed Windows and macOS jobs produce, so the plan must retire or re-point it with the jobs.
quote: it downloads `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` with `gh run download` into a temporary directory removed on exit
@carried n=12 run=2 label=test-plan fact=fct-mos7j36w
quote: `target/nextest/ci/junit.xml`, the report of the job's single instrumented `run --coverage` nextest invocation: `junit-${{ matrix.os }}` (one path);
@carried n=13 label=test-plan/ci-integration fact=fct-5iefujrn
why: The three-OS case-set reading and its macOS carve-out are what the census of OS-gated tests counted, so the plan must re-point or retire it with the census and its gates.
quote: `windows-2025` 3386 · `macos-latest` 3412 · `ubuntu-latest` 3458 cases, 0 failed and 0 skipped; `ubuntu-not-macos` 48 (43 of the macOS no-peer-pid carve-out and 5 Linux-only, each by a named reason)
@carried n=14 label=test-plan/quality-gates-coverage-targets fact=fct-s6rhzqu4
why: The fake agent's per-command process group is a cfg(unix) arm the chunk makes unconditional, and it must keep the zero-refused-profile coverage merge on Linux.
quote: with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).
@carried n=15 label=test-plan/quality-gates-coverage-targets fact=fct-ocdr3xdn
why: The chunk removes the cfg(windows) mutant sites, the HOST_SCRATCH cfg!(windows) const, the windows-mutants.yml dispatch and the macOS compile-out of the two real-cargo-mutants harness tests, while the mutation verdict itself stays unchanged.
quote: A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught
@carried n=15 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-ocdr3xdn
quote: The boundary audit measures the `cfg(windows)` code by dispatching `windows-mutants.yml` (founder ruling C2, 2026-10-04; §9 Mutation row)
@carried n=15 run=3 label=test-plan/quality-gates-coverage-targets fact=fct-ocdr3xdn
quote: The two real-cargo-mutants harness tests (`run_mutants_passes_when_the_change_is_tested`, `run_mutants_reports_survivors_of_an_untested_change`) are compiled out on macOS (`#[cfg(not(target_os = "macos"))]`, never `#[ignore]`)
@carried n=15 run=4 label=test-plan fact=fct-xnmc6vwq
quote: a mutant this host cannot compile or reach is recorded as owed, never equivalent or caught.
@carried n=16 label=test-plan/quality-gates-coverage-targets fact=fct-ftncprbj
why: check_endpoint_dir and peer_admitted lose their Unix-only gate when their sibling arm leaves, and both must stay in the timed hook path under SPINE_DEADLINE on the ubuntu perf job.
quote: Each timed hook's `connect` also runs the socket-directory check (`check_endpoint_dir`, before it connects) and the peer euid decision (`peer_admitted`, after it connects), both before its first frame and each a real check on Unix only
@binding n=1
**ci-integration test matrix ↔ quality-gates-coverage-targets coverage gates**: once the test job runs on ubuntu-latest alone, the coverage thresholds 85/95/80 must hold on that one job (test-plan fct-ryyndnld), its per-OS justification left for the wrap's amendment, and the COVERAGE_IGNORE regex must not be widened while its Windows separator class stays (test-plan fct-b4avhaux; test-plan/ci-integration fct-sd3lvsrr)
@binding n=2
**ci-integration MSRV ↔ quality-gates-coverage-targets zero-flakiness**: the msrv job must go green by root-causing the two e2e-home cases onto a test-controlled free-space figure, never a retry or #[ignore] (test-plan/ci-integration fct-whs3hmpm; test-plan fct-zrh4qt5s)
@binding n=3
**ci-integration Mutation row ↔ quality-gates-coverage-targets mutation gate**: the removal of windows-mutants.yml and of every cfg(windows) arm leaves no host-excluded Windows mutant and no Windows dispatch, so both the §9 Mutation row and the §10 gate-mutation text that name them are amendments the wrap writes, and the plan names them as expected (test-plan/ci-integration fct-ixlw7x3t; test-plan/quality-gates-coverage-targets fct-ocdr3xdn)
@binding n=4
**ci-integration workflow list ↔ the removed contract test**: windows-mutants.yml's tool pins, the contract test asserting them and the three-workflow concurrency count leave together, so the plan must show no test still reading windows-mutants.yml and the count read at two workflows (test-plan/ci-integration fct-k7t43itf)
@binding n=5
**ci-integration kill line ↔ the removed Windows test files**: tests/conpty_sideload.rs leaves the ci profile's stamped-home kill filter, so the plan re-counts the binary(…) names on .config/nextest.toml:24 against the recorded 27 (test-plan/ci-integration fct-7d3yogoz)
@binding n=6
**ci-integration per-OS JUnit reader and reading ↔ the census of OS-gated tests**: the junit-os-sets reader, its three-OS reading and tests/contract_os_gated_tests.rs count the same OS gates, so they are re-pointed or retired together with those gates (test-plan/ci-integration fct-fumv4pch; test-plan/ci-integration fct-5iefujrn)
@binding n=7
**quality-gates-coverage-targets coverage merge and perf session ↔ the cfg(unix) arms made unconditional**: the fake agent's per-command process group and the hooks' socket-directory and peer-euid checks must keep their Linux behaviour exactly once their gate leaves, so the Linux coverage merge reads no refused profile and the five hook rows stay under SPINE_DEADLINE (test-plan/quality-gates-coverage-targets fct-s6rhzqu4; test-plan/quality-gates-coverage-targets fct-ftncprbj)
