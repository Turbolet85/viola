# tests · CI Integration — category read

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:51) Part B's measurement runs through `scripts/release-check.sh`'s build, so the release build it compares must be exactly this command and any change to the script must keep its probe line, its verdict line and its plain exit-code form.
  > ci.yml job `release` (`fail-fast: false`): `bash scripts/release-check.sh --probe` (last line `release-check probes: 5/5 refused, control clean`), then `bash scripts/release-check.sh`, which runs `cargo build --release --locked --bin viola --message-format=json` (fake agent excluded: feature off) and judges that build's own `compiler-artifact` records. It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable, and never lists `target/release/`. A plain exit-code job: no upload, no `viola-harness gate` step
- (.andromeda/test-plan-detail/ci-integration.md:52) If P4 has the measurement read on CI's three OSes, it runs per OS, as the release job does.
  > **Parallel:** per OS
- (.andromeda/test-plan-detail/ci-integration.md:53) A clean-build comparison in CI must not take a cached target dir. rust-cache stays in ci.yml only and is never used in a release workflow.
  > **Cache:** rust-cache (ci.yml only; never in a release workflow)
- (.andromeda/test-plan-detail/ci-integration.md:13) rust-cache restores build output keyed on OS, toolchain and lockfile. A "clean build, from an empty target dir" read in a ci.yml job must start from an empty target dir that this cache has not restored.
  > **Cache:** Swatinem/rust-cache v2.9.2 (SHA-pinned; key: OS + toolchain + `Cargo.lock`), in `ci.yml` only
- (.andromeda/test-plan-detail/ci-integration.md:9) The pin-race fix in `crates/viola-state/src/pin.rs` and its callers must pass the Lint stage's fmt and clippy gate on every OS.
  > `cargo fmt --check`; `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
- (.andromeda/test-plan-detail/ci-integration.md:10) The exe arm on Windows and `pin_companions`' `cfg(windows)` code are not built by a Linux clippy. The plan may list the check-only Windows-target clippy as a chunk gate entry, but never as a CI step or a pre-push stage.
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
- (.andromeda/test-plan-detail/ci-integration.md:11) The macOS-target clippy is only a host recipe for the Unix arms of the fix. It must never be listed as a plan gate entry.
  > a check-only macOS-target clippy is a recipe on the Linux dev host (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`); it reproduced CI's `lint (macos-latest)` error line for line, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/evidence/operator-pass.md`: neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
- (.andromeda/test-plan-detail/ci-integration.md:31) The race witness and any Windows pin test run inside the per-OS instrumented coverage run, beside `conpty_sideload`. That job's last step requires `coverage,doctest`.
  > the coverage run covers the `conpty_sideload` integration binary (`[[test]]`, required-features `fake-agent`) with the rest. Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`
- (.andromeda/test-plan-detail/ci-integration.md:43) The fix must check and pass the unit suite on the 1.96 floor too. It uses no std API newer than MSRV.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`
- (.andromeda/test-plan-detail/ci-integration.md:59) Any workflow step this chunk adds, such as a CI-side measurement, keeps the workflow permissions and SHA pins. `release-check.sh` keeps the runner-provided `jq` and its `tool-missing: jq` refusal.
  > Every workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`) has `permissions: {}` at the top and `contents: read` per job. Every `uses:` is SHA-pinned; zizmor `unpinned-uses` is the assertion.
- (.andromeda/test-plan-detail/ci-integration.md:59) Any tool the by-section comparison needs has exactly one version source and is never installed through a second action.
  > G2 (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs), `scripts/orphans-check.sh` (`lint`) and `scripts/release-check.sh` (`release`) use the runner-provided `jq`, never installed: the `test` and `perf` jobs presence-check it (`jq --version`) and all three scripts refuse with `tool-missing: jq`.
- (.andromeda/test-plan-detail/ci-integration.md:60) A dev-host measurement or gate added by this chunk installs nothing in the pre-push gate.
  > The local pre-push gate runs natively on the Linux dev host and installs nothing:
- (.andromeda/test-plan-detail/ci-integration.md:61) A CI job added for the measurement gets no `concurrency:` block.
  > No workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`) has a `concurrency:` block: a concurrency group cancels pending runs even with `cancel-in-progress: false`, which would drop a push's `always()` gate and upload chain.
- (.andromeda/test-plan.md:760) The body's broad line for the release build that Part B measures through.
  > - **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
- (.andromeda/test-plan.md:795) Both release-check verdict lines must still end as written after this chunk.
  > - Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
- (.andromeda/test-plan.md:771) The measurement holds the toolchain fixed at the exact pin, so two clean builds differ only in what the build itself produces.
  > - Language version: Rust 1.98.1 (current stable at 2026-09-24, ≥ the 1.96 floor), pinned exactly in `rust-toolchain.toml` and installed by `rustup toolchain install` on every OS, host included.
- (.andromeda/test-plan.md:770) The race fix's tests run on all three runners with `fail-fast: false`.
  > `fail-fast: false`.
- (.andromeda/test-plan.md:790) A race witness that fails on any OS fails the build.
  > - Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
- (.andromeda/test-plan.md:787) CI readings of the chunk's runs and any CI-side measurement artifact are read through `gh`.
  > The agent reads job conclusions with `gh run view --json jobs` and fetches artifacts with `gh run download`.
- (.andromeda/test-plan.md:876) A Linux dev-host reading of the pin race or of build reproducibility is no proof for Windows or macOS. Their OS-branch arms are verified only on their own CI runner.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:922) The concurrent-pin witness gets its concurrency inside the test, never from serialising the suite. Per-test homes stay the isolation.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:925) The race's 6-of-24 failure is a flake source. It is fixed at its root, and no test hides it with an ignore.
  > - NEVER ignore flaky tests; quarantine and fix immediately
- (.andromeda/test-plan.md:927) The clean-build comparison uses no rust-cache in a release workflow.
  > - NEVER use rust-cache in a release workflow; it is ci.yml only.
- (.andromeda/test-plan.md:928) No retry budget absorbs a racing boot.
  > - NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:766) The body's broad line for the concurrency ban that any added job follows.
  > No workflow has a `concurrency:` block: a concurrency group cancels pending runs, which would drop a push's `always()` gate and upload chain; zizmor's `concurrency-limits` finding is declined and stays visible.

## Bindings
- **Release build ↔ scope §B (the measurement) and security.md (release build carries `viola` only)**: the clean-build comparison builds through `release-check.sh`'s exact `cargo build --release --locked --bin viola` and leaves its verdict and probe lines unchanged (.andromeda/test-plan-detail/ci-integration.md:51), (.andromeda/test-plan.md:795)
- **rust-cache scope ↔ scope §B "each from an empty target dir"**: a CI-side clean build never reads a restored cache, and no release workflow takes one (.andromeda/test-plan-detail/ci-integration.md:13), (.andromeda/test-plan-detail/ci-integration.md:53), (.andromeda/test-plan.md:927)
- **Windows-target check-only clippy ↔ scope §A (the exe arm on Windows, `pin_companions`)**: a chunk gate entry on the dev host only, beside CI's `lint (windows-2025)` (.andromeda/test-plan-detail/ci-integration.md:10), (.andromeda/test-plan.md:876)
- **Per-OS coverage run ↔ test-plan §10 (coverage gates, zero-flakiness)**: the race witness lands in the instrumented per-OS run and its `coverage,doctest` gate, with no retry and no ignore (.andromeda/test-plan-detail/ci-integration.md:31), (.andromeda/test-plan.md:925), (.andromeda/test-plan.md:928)
- **CI anti-pattern (no serialisation) ↔ e2e-test-strategy Path 7 (one instance as the test-side workaround, P4's to revisit)**: a two-instance return keeps per-test-home isolation and no suite-wide serialisation (.andromeda/test-plan.md:922)
- **Workflow hygiene ↔ security.md §Dependencies and CI (SHA pins, `permissions: {}`, zizmor)**: any job added for a three-OS reading keeps the pins, the permissions and the no-`concurrency:` rule (.andromeda/test-plan-detail/ci-integration.md:59), (.andromeda/test-plan-detail/ci-integration.md:61)
