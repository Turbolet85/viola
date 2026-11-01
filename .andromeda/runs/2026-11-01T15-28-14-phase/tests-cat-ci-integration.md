# tests · CI Integration — category read

## Carried
- (.andromeda/test-plan.md:775) The chunk's Unix legs must hold on both Unix runners, and macOS's server verification must stand as euid plus directory only, with no peer pid comparison added.
  > `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs). `fail-fast: false`.
- (.andromeda/test-plan.md:254) The chunk's OS-branch tests must cover Unix modes and macOS's euid and directory verification on their own runners.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:610) The macOS case owed to this entry must land as a case that runs on the `macos-latest` job.
  > beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening")
- (.andromeda/test-plan.md:486) The chunk's macOS legs, the 0755 socket-directory case among them, must be proven on CI, because no local macOS run exists.
  > - macOS local runs, which are CI-only.
- (.andromeda/test-plan.md:881) The chunk's green on the Linux dev host must not be claimed as proof of its macOS or Windows behaviour.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/ci-integration.md:60) The pre-push gate must not be read as the chunk's macOS proof, since it runs on Linux only.
  > The local pre-push gate runs natively on the Linux dev host and installs nothing
- (.andromeda/test-plan.md:46) The peer euid mismatch must be asserted at the peer-credential check boundary, never as a CI case that needs a second account on a hosted runner.
  > only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:484) The chunk's euid checks on both sides of the socket must be covered by the boundary unit test, since cross-user peers stay untestable end to end.
  > - Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan-detail/ci-integration.md:51) Any seam built to drive a refusal must stay out of the release build, which this check fails on a test-only feature or binary.
  > It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable
- (.andromeda/test-plan-detail/ci-integration.md:9) The chunk's code must pass fmt and clippy `-D warnings` on every target and keep each listed sync crate building without tokio, so any check it adds in `viola-state` or `viola-core` must take no tokio edge.
  > **Runs:** `cargo fmt --check`; `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`; `cargo check` with one `-p` per crate listed in `scripts/sync-crates.txt` (no `--features`, so no tokio; target set `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`,
- (.andromeda/test-plan-detail/ci-integration.md:9) Any module the chunk adds must be reachable from its target, or the orphans gate fails.
  > `bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh` (cargo-modules 0.27.0: `cargo modules orphans -p <pkg> {--lib | --bin <name>} [--features <required features>] --deny` for every lib/bin target from `cargo metadata --no-deps`, last line `orphans-check: N/N targets clean`;
- (.andromeda/test-plan.md:749) Lint must be clean per OS, so code that goes unused on macOS once its comparison compiles out must not break `lint (macos-latest)`.
  > **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, per OS;
- (.andromeda/test-plan-detail/ci-integration.md:11) The macOS-target clippy may serve the chunk as a dev-host recipe for its macOS branches, but must not be listed as a plan gate entry.
  > Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`); it reproduced CI's `lint (macos-latest)` error line for line, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/evidence/operator-pass.md`: neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
- (.andromeda/test-plan-detail/ci-integration.md:10) The plan may list the Windows-target clippy as a chunk gate entry when the chunk touches shared code, since Windows behaviour must not move and the dev host does not build `cfg(windows)`.
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
- (.andromeda/test-plan-detail/ci-integration.md:15) The euid check on the Tokio client must sit behind `viola-channel`'s `tokio` feature, so that the crate's normal graph still prints no tokio line.
  > then, in the same step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh` (chunk 2026-10-17-mcp-server-for-drivers: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line
- (.andromeda/test-plan-detail/ci-integration.md:15) Any dependency the chunk adds to read peer credentials or directory ownership must pass the deny check's bans, licences and sources.
  > ubuntu job `supply-chain`, fail-closed `shell: bash` steps: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans,
- (.andromeda/test-plan-detail/ci-integration.md:27) The chunk's end-to-end cases must pass on all three OSes through this stage.
  > `agent-run run --e2e` on all 3 OSes
- (.andromeda/test-plan-detail/ci-integration.md:31) The chunk's cases must pass the per-OS test job's one instrumented run and its closing gate on `coverage,doctest`.
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`
- (.andromeda/test-plan.md:821) The chunk's `cfg(unix)` code must meet these thresholds on each Unix runner's own job.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:929) A coverage shortfall on the chunk's hard-to-drive refusal paths must not be met by widening the ignore regex.
  > - NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:795) Any failure of the chunk's tests or coverage on any one of the three OSes must fail the build, a macOS-only red included.
  > - Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
  > - Coverage falls below threshold (Section 10) on any OS
- (.andromeda/test-plan-detail/ci-integration.md:39) The client euid check and directory verification the chunk adds to the hook's connect path must hold the perf gate on every OS.
  > Its own job `perf (${{ matrix.os }})` (`fail-fast: false`, `AGENT_RUN_KEEP_HOMES=1`): checkout, toolchain, rust-cache, `cargo install --locked hyperfine@1.20.0`, `jq --version`, `run --perf`, G2 (probe then check), `schema-check`, `secret-scan`, the scan-gated uploads, then `gate --require perf`.
- (.andromeda/test-plan-detail/ci-integration.md:43) Whatever API the chunk uses for peer credentials and directory checks must compile and pass the unit suite on Rust 1.96.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`;
- (.andromeda/test-plan.md:927) A test that loosens or replaces the per-user socket directory must isolate itself per test rather than serialise the suite, because that directory is per user and not per home.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:933) The chunk's new cases must pass with no retries, on the shared-directory and socket paths too.
  > - NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:879) `v1-53`'s one live Linux run and any live measurement this chunk takes must stay local, never a CI step.
  > The suites reach the real CLI only through `agent-run run --local-live`, which refuses when `CI` is set; a route entry's own live measurement of a real session is local too and never a CI step.

## Bindings
- **macOS runner ↔ §6 exit-cause matrix**: the owed macOS case and the 0755 socket-directory case are §6's to define and `macos-latest`'s to prove, (.andromeda/test-plan.md:610) with (.andromeda/test-plan.md:775).
- **Hosted-runner limit ↔ §4 unit strategy**: the runners cannot drive a peer of another euid, so the mismatch is held by the unit layer's injected euids, (.andromeda/test-plan.md:46) with (.andromeda/test-plan.md:380).
- **Per-OS coverage gate ↔ §10 thresholds**: the test job's gate reads the thresholds §10 sets per OS, (.andromeda/test-plan-detail/ci-integration.md:31) with (.andromeda/test-plan.md:821).
- **Perf job ↔ §10 spine deadline**: the hook's added checks run inside the connect deadline that §10 holds below the spine gate, (.andromeda/test-plan-detail/ci-integration.md:39) with (.andromeda/test-plan.md:842).
- **Channel tokio tree form ↔ §5 MCP handlers**: the MCP handlers use the Tokio client that "every Unix client" reaches, and the supply-chain step holds it behind its feature, (.andromeda/test-plan-detail/ci-integration.md:15) with (.andromeda/test-plan.md:457).
- **Parallel suite ↔ §7 per-test isolation**: the CI ban on serialising rests on §7's isolated per-test state, which the shared per-user socket directory does not get from the home alone, (.andromeda/test-plan.md:927) with (.andromeda/test-plan.md:697).
