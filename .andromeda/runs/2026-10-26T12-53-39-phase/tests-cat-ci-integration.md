# tests · CI Integration — category read

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:9) The new verbs, the two served channel methods and the end of `LinkSet::remove`'s `#[cfg_attr(not(test), allow(dead_code))]` must pass the format gate and clippy with warnings denied, on every OS of the lint matrix.
  > `cargo fmt --check`; `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
- (.andromeda/test-plan-detail/ci-integration.md:9) Any module the chunk adds (a `link` / `unlink` verb file, a channel handler) must be reachable from its target, so the orphans gate and its probe still end clean.
  > `bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh` (cargo-modules 0.27.0: `cargo modules orphans -p <pkg> {--lib | --bin <name>} [--features <required features>] --deny` for every lib/bin target from `cargo metadata --no-deps`, last line `orphans-check: N/N targets clean`;
- (.andromeda/test-plan-detail/ci-integration.md:9) Any `link` / `unlink` param or result type the chunk places in a sync crate (`viola-core`, `viola-state`) or in `viola-channel` must keep that crate building without tokio, and `viola-channel`'s normal graph tokio-free.
  > `cargo check` with one `-p` per crate listed in `scripts/sync-crates.txt` (no `--features`, so no tokio; target set `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`, four since `viola-channel` left the list at chunk 2026-10-17-mcp-server-for-drivers — its Tokio client sits behind its `tokio` feature and its normal graph is held by the tree form `scripts/channel-tokio-free.sh`, Supply-chain row — each joining the list with its crate; an empty list fails; job `lint`)
- (.andromeda/test-plan-detail/ci-integration.md:10) The verbs' client path carries the `cfg(windows)` server check through `client::connect`, which the Linux dev host does not build, so the plan may list the check-only Windows-target clippy as a chunk gate entry, and it stays neither a CI step nor a pre-push stage.
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
- (.andromeda/test-plan-detail/ci-integration.md:11) Test modules for the verbs' server check gated off macOS (the recorded gap) may be read with the macOS-target check-only clippy before a push, but the plan must not list it as a gate entry.
  > Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`); it reproduced CI's `lint (macos-latest)` error line for line, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/evidence/operator-pass.md`: neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
- (.andromeda/test-plan-detail/ci-integration.md:31) The chunk's new root cases (`tests/channel_links.rs`, the `link` / `unlink` cases of `tests/cli_links.rs`, the moved `HELP` literal) run inside each OS's one instrumented coverage run, which must end in a passing `gate --require coverage,doctest`.
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`
- (.andromeda/test-plan-detail/ci-integration.md:43) The verbs and channel methods must compile and their unit cases pass on the 1.96 MSRV toolchain.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`;
- (.andromeda/test-plan.md:767) Every new test must build and pass on all three runners, with the verbs' server-identity cases on Linux and Windows and macOS on its recorded gap.
  > OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal), `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs). `fail-fast: false`.
- (.andromeda/test-plan.md:787) A failing `link` / `unlink` test on any OS fails the build.
  > Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
- (.andromeda/test-plan.md:788) The new verb and method code must hold the per-OS coverage thresholds.
  > Coverage falls below threshold (Section 10) on any OS
- (.andromeda/test-plan.md:784) The chunk's CI verdict is read from the job conclusions and artifacts through `gh`, never the web UI.
  > The agent reads job conclusions with `gh run view --json jobs` and fetches artifacts with `gh run download`.
- (.andromeda/test-plan.md:919) The two-wrapper link cases (`overseer` and `builder`) must take their isolation from per-test homes, never from serialised test threads.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:925) No retry may be added to pass the new cases.
  > NEVER set nextest `retries` above 0.

## Bindings
- **clippy `-D warnings` ↔ scope's `unlink` product path**: once the `unlink` channel method calls `LinkSet::remove`, its `#[cfg_attr(not(test), allow(dead_code))]` must go, and the lint gate must stay green without it on every OS (.andromeda/test-plan-detail/ci-integration.md:9)
- **orphans gate ↔ architecture's module tree**: a new `src/cmd/` verb file or `src/run/` handler must be declared in the bin target's module tree, or `orphans-check` ends below `N/N targets clean` (.andromeda/test-plan-detail/ci-integration.md:9)
- **Coverage build-failure condition ↔ §10 Quality Gates**: the 85 % line, 95 % function and 80 % region floors per OS bind the new code (.andromeda/test-plan.md:788)
- **OS matrix ↔ security-plan server verification and §6 Scenario E3**: E3 steps 1 and 3 and the raw-channel name negative run on all three runners, while the verbs' server-identity case is Linux and Windows only, macOS being the gap owed to "Unix endpoint and home hardening" (.andromeda/test-plan.md:767)
- **Windows-target check-only clippy ↔ `client::connect`'s `cfg(windows)` server check**: an optional host gate entry for the verbs' Windows code path, which the Linux dev host does not build (.andromeda/test-plan-detail/ci-integration.md:10)
- **CI read ↔ the wrap's chunk verdict**: the chunk's push is judged by `gh run view --json jobs` across the 15 checks (.andromeda/test-plan.md:784)
