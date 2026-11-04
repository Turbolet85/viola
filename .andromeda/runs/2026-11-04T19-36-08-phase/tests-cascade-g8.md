@carried n=1 label=test-plan/ci-integration fact=fct-pk32flwi
why: Every cfg(unix) or cfg(target_os) test code the chunk adds or un-gates must pass the per-OS lint job on ubuntu-latest and macos-latest, and the dev-host macOS-target clippy must not be listed as a gate entry.
quote: `cargo fmt --check`; `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
@carried n=1 run=2 label=test-plan/ci-integration fact=fct-pk32flwi
quote: Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`); it reproduced CI's `lint (macos-latest)` error line for line, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/evidence/operator-pass.md`: neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
@carried n=1 run=3 label=test-plan fact=fct-346brcpe
quote: fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, per OS; the `--acyclic` review and the dev-host Windows- and macOS-target check-only clippy runs are not gates.
@carried n=1 run=4 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=2 label=test-plan/ci-integration fact=fct-ijfdjbph
why: The full fake-agent suite must run as the same E2E suite on all three runners, every Unix-shared case on ubuntu-latest and macos-latest.
quote: `agent-run run --e2e` on all 3 OSes
@carried n=2 run=2 label=test-plan fact=fct-shyrfs6v
quote: `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs). `fail-fast: false`.
@carried n=2 run=3 label=test-plan fact=fct-q4zg6vp5
quote: Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
@carried n=3 label=test-plan/ci-integration fact=fct-sd3lvsrr
why: The per-OS test job the chunk re-reads green is one instrumented nextest run ending in gate --require coverage,doctest, so every test moved onto Unix runners is judged there.
quote: Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`
@carried n=3 run=2 label=test-plan fact=fct-ryyndnld
quote: Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
@carried n=3 run=3 label=test-plan fact=fct-6iojik4n
quote: Coverage falls below threshold (Section 10) on any OS
@carried n=3 run=4 label=test-plan fact=fct-b4avhaux
quote: NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
@carried n=3 run=5 label=test-plan fact=fct-r52fahdc
quote: NEVER lower coverage threshold to pass build (raise quality, not bar)
@carried n=4 label=test-plan/ci-integration fact=fct-bfdvmti3
why: The perf job the chunk re-reads green on ubuntu-latest and macos-latest must keep its G2, schema-check, secret-scan and gate --require perf steps.
quote: Its own job `perf (${{ matrix.os }})` (`fail-fast: false`, `AGENT_RUN_KEEP_HOMES=1`): checkout, toolchain, rust-cache, `cargo install --locked hyperfine@1.20.0`, `jq --version`, `run --perf`, G2 (probe then check), `schema-check`, `secret-scan`, the scan-gated uploads, then `gate --require perf`.
@carried n=4 run=2 label=test-plan fact=fct-3q2uf724
quote: `agent-run run --perf` in its own per-OS job, its verdict from `viola-harness gate`, never from a `jq` binary.
@carried n=5 label=test-plan/ci-integration fact=fct-k7t43itf
why: The obs gates the chunk re-reads on both Unix runners live in named jobs: G2 in the test and perf jobs, the G1/G3 ripgrep in the lint job.
quote: G2 (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs), `scripts/orphans-check.sh` (`lint`) and `scripts/release-check.sh` (`release`) use the runner-provided `jq`, never installed: the `test` and `perf` jobs presence-check it (`jq --version`) and all three scripts refuse with `tool-missing: jq`. ripgrep 15.2.0, the G1/G3 tool, comes from `scripts/install-ripgrep.sh` (pinned, checksum-verified official release) in the `lint` job.
@carried n=5 run=2 label=test-plan fact=fct-bzvycnyj
quote: `viola-harness gate` reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any §10 breach.
@carried n=6 label=test-plan/quality-gates-coverage-targets fact=fct-s6rhzqu4
why: Kill-shaped tests the chunk reads on Unix runners (Wrapper::kill, kill-and-revive) must leave no refused profile in the ubuntu and macOS coverage merge, which rests on the fake agent's own process groups.
quote: A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).
@carried n=6 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-s6rhzqu4
quote: One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
@carried n=7 label=test-plan/quality-gates-coverage-targets fact=fct-ocdr3xdn
why: The per-OS gating the chunk states must keep the two real-cargo-mutants harness tests compiled out on macOS with their named reason, never #[ignore], running on ubuntu and windows.
quote: The two real-cargo-mutants harness tests (`run_mutants_passes_when_the_change_is_tested`, `run_mutants_reports_survivors_of_an_untested_change`) are compiled out on macOS (`#[cfg(not(target_os = "macos"))]`, never `#[ignore]`): there cargo-mutants' baseline build in its copied tree takes ~77 s of an 80.9 s phase against 0.41 s for the same cold build outside it, with no first-launch or jobserver cost, as measured at CI run 36481260151 (the chunk 2026-09-28-mutation-testing-to-the-epoch-boundary's `evidence/macos-mutants-phases.md`); the cause inside that phase stays unnamed, so under the operator's ruling it is runner-side. Both run on ubuntu and windows; `[profile.ci]`'s 120 s kill and `retries = 0` are unchanged.
@carried n=8 label=test-plan/quality-gates-coverage-targets fact=fct-ftncprbj
why: The perf job re-measured on ubuntu-latest and macos-latest times the hook rows with the Unix-only socket-directory and peer-euid checks in the path, and every row's max must stay under SPINE_DEADLINE.
quote: Each timed hook's `connect` also runs the socket-directory check (`check_endpoint_dir`, before it connects) and the peer euid decision (`peer_admitted`, after it connects), both before its first frame and each a real check on Unix only; with those two in the timed path the five hook rows read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms and a highest `max` of 2.8 ms, under `SPINE_DEADLINE`, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening (report, Outcome, `run --perf`). The `max` reading above is that of the path without these two, and the gate stays `SPINE_DEADLINE`.
@carried n=8 run=2 label=test-plan/quality-gates-coverage-targets fact=fct-ftncprbj
quote: After the last row and **before** `cleanup`, `--perf` asserts the home's role files (`diagnostics/*.ndjson` without `detail-*`) hold no `event:"panic"` line, with no seam exemption, because a hook panic exits 0 and hyperfine cannot see it; a panic line fails `panic`.
@carried n=8 run=3 label=test-plan fact=fct-ggkcj6j7
quote: **Spine deadline:** the gate reads the product constant `viola_core::SPINE_DEADLINE` (1.0 s), imported by `viola-harness` (`gate.rs` `perf()`), never an edited literal in `ci.yml` or this table.
@carried n=8 run=4 label=test-plan fact=fct-5j2njdi5
quote: There is no rerun, no outlier trimming and no second attempt. A breach is fixed in the product or escalated through a Decisions Log entry, never retried away.
@binding n=1
**ci-integration (per-OS test job) ↔ quality-gates-coverage-targets (per-OS coverage gate)**: each Unix test job's one instrumented run must end in `gate --require coverage,doctest` at lines 85, functions 95 and regions 80 on its own OS; a test moved onto ubuntu-latest or macos-latest changes what that job counts, and the gate is never met by widening the ignore regex or lowering a threshold (test-plan/ci-integration fct-sd3lvsrr) (test-plan fct-ryyndnld) (test-plan fct-b4avhaux)
@binding n=2
**ci-integration (perf job) ↔ quality-gates-coverage-targets (perf session)**: `perf (ubuntu-latest)` and `perf (macos-latest)` must read green with every hook row's `max` under `SPINE_DEADLINE`, the Unix-only `check_endpoint_dir` and `peer_admitted` in the timed path, no panic line before `cleanup`, and no rerun of a breach (test-plan/ci-integration fct-bfdvmti3) (test-plan/quality-gates-coverage-targets fct-ftncprbj) (test-plan fct-5j2njdi5)
@binding n=3
**ci-integration (G-check placement) ↔ the obs gates the scope re-reads**: G1 and G3 must read green through the lint job's ripgrep, G2 through the test and perf jobs, and the G4 `schema-check` step in the perf job, each on ubuntu-latest and macos-latest beside windows-2025 (test-plan/ci-integration fct-k7t43itf) (test-plan/ci-integration fct-bfdvmti3)
@binding n=4
**quality-gates-coverage-targets (profraw mechanisms) ↔ the fake agent (test data)**: every kill-shaped case the chunk reads on Unix runners, the wrapper kill and the kill-and-revive case among them, must leave no refused profile in the coverage merge, which holds only while every command the fake agent spawns runs in a process group of its own and its exit ignores SIGHUP (test-plan/quality-gates-coverage-targets fct-s6rhzqu4) (test-plan fct-7o4nbra2)
@binding n=5
**quality-gates-coverage-targets (mutation harness tests) ↔ per-OS test gating**: the two real-cargo-mutants harness tests must stay gated `#[cfg(not(target_os = "macos"))]` with their measured reason, never `#[ignore]`, and run on ubuntu and windows under `[profile.ci]`'s 120 s kill and `retries = 0` (test-plan/quality-gates-coverage-targets fct-ocdr3xdn)
@binding n=6
**ci-integration (lint per OS) ↔ Test Strategy anti-pattern (own-runner proof)**: OS-branch test code must be read green by `lint (ubuntu-latest)` and `lint (macos-latest)` on CI; the dev-host macOS-target clippy is a recipe, never a plan gate entry and never proof for macOS (test-plan/ci-integration fct-pk32flwi) (test-plan fct-lktzdsoq)
