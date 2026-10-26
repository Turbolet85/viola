# tests · cascade read · group 3 of 3

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:14) The `supply-chain` job must read green on the bumped lock: cargo deny over the root lockfile with no RUSTSEC-2026-0386 finding, the sole-root tokio ban per sync crate, `scripts/channel-tokio-free.sh` ending `channel-tokio-free: ok` and `scripts/deny-probes.sh` ending `deny-probes: 13/13 banned, control clean`, its JSON uploaded; the weekly advisories run then re-audits the same lock.
  > ubuntu job `supply-chain`, fail-closed `shell: bash` steps: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans); `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, run from the repo root (under `fuzz/`, rustup would demand the fuzz nightly); the tokio ban once per crate in `scripts/sync-crates.txt` as sole root, `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans`, then, in the same step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh` (chunk 2026-10-17-mcp-server-for-drivers: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line — exit 0 "nothing to print", or exit 101 "did not match" while tokio is not in the lock; presence read from the printed line, never the exit — and its control with `--features tokio` must print one; verdict `channel-tokio-free: ok`); `bash scripts/deny-probes.sh` (a permanent negative probe per ban + a clean control, last line `deny-probes: 13/13 banned, control clean`: the twelve bans one probe each, plus probe `channel-tokio-edge` — a throwaway workspace with a normal tokio edge must fail the tree form — with its channel control, the same edge behind a `tokio` feature passing); `zizmor --format=json .github/workflows/`. The deny (root and fuzz) and zizmor JSON go to `target/supply-chain/`, uploaded `if: always()` as artifact `supply-chain` (7 days; admissible by content, obs-plan §8 item 6). Weekly: `nightly.yml` job `advisories` runs `cargo deny check advisories` and `cargo deny --manifest-path fuzz/Cargo.toml check advisories`, on `schedule` + `workflow_dispatch`
  (.andromeda/test-plan.md:743)
  > - **Supply-chain:** one fail-closed ubuntu job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate with the channel's tree form, the deny probes and zizmor, uploading their JSON; a weekly nightly job re-runs the advisories.
  (.andromeda/test-plan.md:791)
  > - Supply-chain errors: a cargo deny ban/advisory/licence/source finding, a `fuzz/Cargo.lock` advisory or source finding, a listed sync crate failing the sole-root `deny-sync.toml` tokio ban, `scripts/channel-tokio-free.sh` not ending `channel-tokio-free: ok` (`viola-channel` reaching tokio without its feature, or its control reading none), `scripts/deny-probes.sh` not ending `deny-probes: 13/13 banned, control clean` (one probe per ban, plus the tree form's), a zizmor finding, or the weekly `nightly.yml` advisories run (root and fuzz lockfiles) failing
- (.andromeda/test-plan-detail/ci-integration.md:58) The cargo deny verdict the acceptance reads must come from CI's one pinned cargo-deny version, 0.20.2, run as the pinned CLI, never an action or a host version off that pin.
  > Tools are installed with taiki-e/install-action v2.87.19 (SHA-pinned) as `tool: cargo-nextest@0.9.146,cargo-llvm-cov@0.9.1,cargo-mutants@27.1.0,cargo-deny@0.20.2`;
  (.andromeda/test-plan-detail/ci-integration.md:58)
  > cargo-deny and zizmor run as the pinned CLIs above; cargo-deny-action and zizmor-action are not used, so each tool has exactly one version source.
  (.andromeda/test-plan-detail/ci-integration.md:59)
  > cargo-deny 0.20 takes `--config <path>` as a global option before the subcommand; `check -c` is rejected (as measured on cargo-deny 0.20.2, dev host, 2026-09-24).
- (.andromeda/test-plan-detail/ci-integration.md:9) The lint job must stay green on the new tokio: clippy `-D warnings` turns red on any call in viola's own code to a deprecated safe `AsyncFd` constructor, and the tokio-free `cargo check` of the four listed sync crates must still build without tokio.
  > `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`; `cargo check` with one `-p` per crate listed in `scripts/sync-crates.txt` (no `--features`, so no tokio; target set `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`, four since `viola-channel` left the list at chunk 2026-10-17-mcp-server-for-drivers — its Tokio client sits behind its `tokio` feature and its normal graph is held by the tree form `scripts/channel-tokio-free.sh`, Supply-chain row — each joining the list with its crate; an empty list fails; job `lint`);
  (.andromeda/test-plan.md:741)
  > - **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, per OS; the `--acyclic` review and a dev-host Windows-target clippy are not gates.
  (.andromeda/test-plan.md:790)
  > - Lint errors: fmt, clippy `-D warnings`, a sync crate listed in `scripts/sync-crates.txt` failing to build without tokio (or an empty list), a cargo-modules orphan in any lib/bin target (`scripts/orphans-check.sh` not ending `N/N targets clean`, or its probe not ending `2/2 fired, control clean`), `scripts/rmcp-features.sh` not ending `rmcp-features: server transport-io` (rmcp absent from `viola`'s release graph, or a feature beyond `server` + `transport-io` requested)
- (.andromeda/test-plan-detail/ci-integration.md:9) The chunk re-reads the rmcp release-graph assertion against the new lock, so `scripts/rmcp-features.sh` must still end `rmcp-features: server transport-io` with rmcp present in `viola`'s release graph.
  > The rmcp release-graph assertion is the lint step `rmcp features (exactly server + transport-io, non-vacuous)` (every OS, after `Lint probes`; chunk 2026-10-17-mcp-server-for-drivers): `bash scripts/rmcp-features.sh` runs `cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth`, rmcp must be present and the features a non-rmcp package requests exactly `server transport-io`, last line `rmcp-features: server transport-io`.
- (.andromeda/test-plan-detail/ci-integration.md:50) The release build compiles the bumped tokio through `viola-mcp` under `--locked`, so `scripts/release-check.sh` must still end `release-check: viola only` and its probe `release-check probes: 5/5 refused, control clean`.
  > ci.yml job `release` (`fail-fast: false`): `bash scripts/release-check.sh --probe` (last line `release-check probes: 5/5 refused, control clean`), then `bash scripts/release-check.sh`, which runs `cargo build --release --locked --bin viola --message-format=json` (fake agent excluded: feature off) and judges that build's own `compiler-artifact` records. It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable, and never lists `target/release/`. A plain exit-code job: no upload, no `viola-harness gate` step
  (.andromeda/test-plan.md:757)
  > - **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
  (.andromeda/test-plan.md:792)
  > - Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
- (.andromeda/test-plan-detail/ci-integration.md:18) The workspace tests must pass on the new tokio locally, the unit suite run through the harness.
  > `scripts/agent-run.sh run --unit` (pwsh `scripts/agent-run.ps1 run --unit` on windows)
  (.andromeda/test-plan.md:745)
  > - **Unit / Integration / E2E:** `agent-run run --unit`, `--integration` and `--e2e`, the local equivalents of CI's one instrumented coverage run.
- (.andromeda/test-plan-detail/ci-integration.md:22) The integration suite, home of the MCP handler tests that drive `viola_channel::tokio_client::TokioClient`, must pass on the bumped tokio.
  > `agent-run run --integration`
- (.andromeda/test-plan-detail/ci-integration.md:26) The E2E suite, holding the MCP stdio path driven by the rmcp client over a `viola mcp` child, must pass on the bumped tokio on all three OSes, not on the Linux dev host alone.
  > `agent-run run --e2e` on all 3 OSes
  (.andromeda/test-plan.md:767)
  > - OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal), `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs). `fail-fast: false`.
  (.andromeda/test-plan.md:873)
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/ci-integration.md:30) The per-OS instrumented test job, ending in `viola-harness gate --require coverage,doctest`, is part of the full CI row read on the chunk's push at the wrap, and must read green on every OS.
  > `agent-run run --coverage` (cargo-llvm-cov 0.9.1 under nextest). In CI this is the per-OS test job;
  (.andromeda/test-plan-detail/ci-integration.md:30)
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`
  (.andromeda/test-plan.md:747)
  > - **Coverage report:** `agent-run run --coverage` is the per-OS test job, on `windows-2025` after the ConPTY vendor verification; its one instrumented nextest run replaces the unit, integration and E2E rows, and each job ends in `viola-harness gate --require <suites>`.
  (.andromeda/test-plan.md:787)
  > - Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
  (.andromeda/test-plan.md:854)
  > - Test failure (any level, any OS)
- (.andromeda/test-plan-detail/ci-integration.md:42) The bumped tokio must check and pass the unit suite on the Rust 1.96 floor, the MSRV job read in the full CI row at the wrap.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`; no toolchain action (`RUSTUP_TOOLCHAIN` outranks `rust-toolchain.toml`, as measured at chunk 2026-09-24-quality-gates, research.md M4). 1.96 is the security toolchain floor and the workspace `rust-version` requested from arch (obs-plan D-22; sysinfo 0.39.6 needs 1.95)
  (.andromeda/test-plan.md:753)
  > - **MSRV:** an ubuntu job checks the workspace and runs the unit suite on Rust 1.96, the security toolchain floor.
- (.andromeda/test-plan-detail/ci-integration.md:54) Each harness-gated job's verdict on the chunk's push comes from `viola-harness gate` over its own outputs, and the agent reads job conclusions and the cargo-deny JSON through `gh`, never by hand.
  > `viola-harness gate`, which reads `junit-*.xml`, `run-summary.json`, the llvm-cov JSON summary and the hyperfine JSON, and exits non-zero on any §10 breach
  (.andromeda/test-plan.md:759)
  > - **Quality gates:** `viola-harness gate` reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any §10 breach.
  (.andromeda/test-plan.md:784)
  > The agent reads job conclusions with `gh run view --json jobs` and fetches artifacts with `gh run download`. Locally the same files come from `agent-run run` and `agent-run logs`.
  (.andromeda/test-plan.md:858)
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary

## Bindings
- **ci-integration Supply-chain ↔ quality-gates build failure conditions**: the cargo deny advisory finding that fails the `supply-chain` job (.andromeda/test-plan-detail/ci-integration.md:14) is the same failure §10 names as a build failure (.andromeda/test-plan.md:858); clearing RUSTSEC-2026-0386 by the bump clears both, never by widening an ignore (.andromeda/test-plan.md:931).
- **ci-integration Supply-chain ↔ security plan (Dependencies and CI)**: the cargo deny run over `deny.toml` (.andromeda/test-plan-detail/ci-integration.md:14) must pass with RUSTSEC-2017-0008 still the only ignore, the security plan's no-widening rule binding the same gate.
- **ci-integration tool pins ↔ Supply-chain**: the cargo-deny 0.20.2 pin and its global `--config` form (.andromeda/test-plan-detail/ci-integration.md:58, .andromeda/test-plan-detail/ci-integration.md:59) are what the sole-root `deny-sync.toml` ban (.andromeda/test-plan-detail/ci-integration.md:14) is run at.
- **ci-integration Lint ↔ Release build**: the tokio-free sync-crate check and the rmcp assertion (.andromeda/test-plan-detail/ci-integration.md:9) and the `--locked` release build (.andromeda/test-plan-detail/ci-integration.md:50) all read the same bumped `Cargo.lock`, so one lock move is re-read by both jobs.
- **ci-integration Coverage report ↔ Unit / Integration / E2E and the §11 multi-OS rule**: the local `run --unit` / `--integration` / `--e2e` (.andromeda/test-plan-detail/ci-integration.md:18, .andromeda/test-plan-detail/ci-integration.md:22, .andromeda/test-plan-detail/ci-integration.md:26) are the Linux dev host's equivalents of the per-OS instrumented job (.andromeda/test-plan-detail/ci-integration.md:30); the Windows and macOS Tokio paths are proven only by that job on their own runners (.andromeda/test-plan.md:873).
