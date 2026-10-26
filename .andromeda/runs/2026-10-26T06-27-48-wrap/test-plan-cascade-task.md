# Task — test-plan · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The session
The workspace's exact tokio pin moved from `=1.53.1` to `=1.53.3` (`default-features = false` kept), and the lock's tokio entry moved with it (version and checksum, a 4-line lock delta, every other crate unmoved: tokio-macros 2.7.2, tokio-stream 0.1.19, tokio-util 0.7.20).
The move closes the open advisory RUSTSEC-2026-0386 against the locked tokio: `cargo deny check` (all four families, cargo-deny 0.20.2) exits 0 without it, and no ignore was added or widened.
CI job `supply-chain`, red on every run since the 2026-10-20 wrap, read green on ci#40278510123 (sha 948f7f01, checks 15/15), including its fuzz lockfile audit, sole-root tokio ban, ban probes and zizmor steps.
No source, crate manifest, symbol, IPC method, env var, deny config, lint, toolchain, workflow, harness or gate surface changed; tokio 1.53.3 has the same dependency set as 1.53.1 and declares rust-version 1.71.
tokio 1.53.3 deprecates four `AsyncFd` constructors; viola calls none of them, and clippy `-D warnings` on the host and on `x86_64-pc-windows-msvc` read green with no deprecated line.

## Reached files
- Cargo.toml
- Cargo.lock

## Touches
- §8 Mocking & Stubbing Discipline: the two "tokio 1.53.1 `test-util`" mentions (`.andromeda/test-plan-detail/mocking-stubbing-discipline.md` lines 8 and 17) now read 1.53.3, with the "in no manifest yet" qualifier kept unchanged.
- §7 Test Data & Fixtures: the "tokio `=1.53.1`" mention in the `data` group (`.andromeda/test-plan-detail/test-data-fixtures.md` line 21) now reads `=1.53.3`.
- §3 Test Harness Contract: the Bootstrap phases key file `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md` line 24 states the tokio version, which is now 1.53.3.
- Any other test-plan text that names the locked or pinned tokio version as current truth is now stale at 1.53.1 and should read 1.53.3.
- §9 CI Integration, Supply-chain row and supply-chain build failure conditions: re-read and unchanged (cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the channel's tree form, deny probes, zizmor); the job now reads green on CI.
- The `deny-probes: 13/13 banned, control clean` ending and the `channel-tokio-free: ok` ending: re-read and unchanged, as measured this session.
- The four sole-root `deny-sync.toml` bans over `scripts/sync-crates.txt`: each exits 0, list and count unchanged.
- Tokio containment: re-read and unchanged. Tokio is built only in `viola-mcp` (`rt`), behind `viola-channel`'s optional `tokio` feature (`io-util`, dev `rt`), and in the test-only `viola-e2e` (`rt` + `process`); the sync crates and the root package carry no tokio dev-dependency.
- §1 supply-chain coverage trigger and the dependency and boundary policy entity: re-read and unchanged. `cargo deny check` still has RUSTSEC-2017-0008 as its only ignore.
- `scripts/rmcp-features.sh` ending `rmcp-features: server transport-io`: re-read and unchanged.
- The fuzz workspace's own lockfile audit (advisories, sources): re-read and green, rule unchanged.
- §9 MSRV job (Rust 1.96 check): re-read green, since tokio 1.53.3's declared rust-version 1.71 sits below the floor; the floor is unchanged.
- The CI tool pins cargo-deny 0.20.2 and zizmor 1.30.1: re-read and unchanged.
- §11 CI and §8 Mocking bans on tokio in the sync crates (no failpoint crate that pulls tokio, the tokio ban): their subject crate version moved but the rules are unchanged.
- Through the clippy `-D warnings` lint gate: no viola call site reaches the newly deprecated `AsyncFd` constructors, so the lint gate stands green unchanged.

## New facts
- (none)

## Untouched
- §10 coverage thresholds, the `--ignore-filename-regex` exclusion and `LLVM_PROFILE_FILE` handling: no coverage rule or threshold moved.
- §10 mutation gate and `run --mutants` verdict rules: no mutation run or rule was involved.
- §10 performance budgets, hyperfine rows and `SPINE_DEADLINE`: no hook or perf surface changed.
- §6 critical-path scenarios Path 1 to Path 7 and expansion scenarios E1 to E5: no product behaviour changed.
- §6 non-path suites (security sweep, error sanitization, exit-cause matrix, security negatives, list row integrity, chaos, property, contract): no test or product surface changed.
- §7 fake-agent contract and fixture hygiene rules: no fixture, scrub or fake-agent behaviour changed.
- §4 per-crate unit coverage lists and §5 integration drivers (interprocess 2.4.4, jsonschema 0.57.0, schemars 1.2.2, assert_cmd, trycmd, windows-sys 0.61.2): none of those dependency versions moved.
- §2 test pyramid, naming conventions and directory pattern: unchanged.
- §1 untestable zones and the testability of every entity: unchanged.
- §3 harness command grammar, session record, exit codes and usage-error details: no harness surface changed.
- §9 release-build check, ConPTY vendor verification, OS matrix and Rust 1.98.1 toolchain pin: not touched by a lock-only bump.
- Pinned versions of nextest, cargo-llvm-cov, hyperfine, rstest, proptest and insta: no other dev-tool or crate version moved.
