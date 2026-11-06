@carried n=1 label=test-plan/ci-integration fact=fct-zwsm4wxk
why: The chunk must clear the recorded supply-chain red by moving portable-pty off =0.8.1 to a release whose graph drops pin-utils, never by widening the ignore list.
quote: the Supply-chain stage's `supply-chain` job reads red on one advisory, RUSTSEC-2026-0441 (`pin-utils` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1`), as measured at ci#42030025050; owner: the CARRY on «Version done-check», its remedy moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`; the ignore list is not widened.
@carried n=2 label=test-plan/ci-integration fact=fct-3gkvok74
why: After the portable-pty move, the supply-chain job's root and fuzz deny checks, its sole-root tokio ban over viola-pty and the other sync crates, channel-tokio-free and the deny probes must all read green.
quote: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans, the bans holding a `[[bans.features]]` table that denies serde_json's `unbounded_depth`); `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, run from the repo root (under `fuzz/`, rustup would demand the fuzz nightly); the tokio ban once per crate in `scripts/sync-crates.txt` as sole root, `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans`, then, in the same step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh` (chunk 2026-10-17-mcp-server-for-drivers: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line — exit 0 "nothing to print", or exit 101 "did not match" while tokio is not in the lock; presence read from the printed line, never the exit — and its control with `--features tokio` must print one; verdict `channel-tokio-free: ok`); `bash scripts/deny-probes.sh` (a permanent negative probe per ban + a clean control, last line `deny-probes: 14/14 banned, control clean`
@carried n=2 run=2 label=test-plan fact=fct-ys3szg34
quote: Supply-chain errors: a cargo deny ban/advisory/licence/source finding, a `fuzz/Cargo.lock` advisory or source finding, a listed sync crate failing the sole-root `deny-sync.toml` tokio ban, `scripts/channel-tokio-free.sh` not ending `channel-tokio-free: ok` (`viola-channel` reaching tokio without its feature, or its control reading none), `scripts/deny-probes.sh` not ending `deny-probes: 14/14 banned, control clean` (one probe per ban, plus the tree form's), a zizmor finding, or the weekly `nightly.yml` advisories run (root and fuzz lockfiles) failing
@carried n=3 label=test-plan/ci-integration fact=fct-pk32flwi
why: The lint gates must read green at the wrap, including the tokio-free cargo check of viola-pty once its portable-pty dependency moves.
quote: `cargo fmt --check`; `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`; `cargo check` with one `-p` per crate listed in `scripts/sync-crates.txt` (no `--features`, so no tokio; target set `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`, four since `viola-channel` left the list at chunk 2026-10-17-mcp-server-for-drivers — its Tokio client sits behind its `tokio` feature and its normal graph is held by the tree form `scripts/channel-tokio-free.sh`, Supply-chain row — each joining the list with its crate; an empty list fails; job `lint`)
@carried n=3 run=2 label=test-plan fact=fct-346brcpe
quote: **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, on `ubuntu-latest`; the `--acyclic` review is not a gate.
@carried n=3 run=3 label=test-plan fact=fct-7h3hdkfi
quote: Lint errors: fmt, clippy `-D warnings`, a sync crate listed in `scripts/sync-crates.txt` failing to build without tokio (or an empty list)
@carried n=4 label=test-plan/mocking-stubbing-discipline fact=fct-sik7tvk4
why: The new portable-pty release must keep the viola-pty Pty seam that consumers are tested against through its mockall double, and item 17's EXE_SUFFIX removal in the harness must keep the assert_cmd target-dir lookup resolving its binaries.
quote: mockall 0.15.0 on the `viola-pty` `Pty` trait and the liveness / `Clock` traits; native stub binaries (`viola-fake-agent`) | `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5)
@carried n=4 run=2 label=test-plan fact=fct-76bthv7a
quote: **Process mocking:** mockall `automock` on seam traits only; external processes are real stub executables (`viola-fake-agent`).
@carried n=4 run=3 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle.
@carried n=5 label=test-plan/ci-integration fact=fct-whs3hmpm
why: The MSRV job must read green, so the portable-pty release the chunk moves to must build and pass the unit suite on Rust 1.96.
quote: ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`
@carried n=5 run=2 label=test-plan fact=fct-bpswkwdk
quote: **MSRV:** an ubuntu job checks the workspace and runs the unit suite on Rust 1.96, the security toolchain floor.
@carried n=6 label=test-plan/ci-integration fact=fct-sd3lvsrr
why: The test job must read green at the wrap, its coverage floors must hold while COVERAGE_IGNORE's Windows-path comment and the per-OS wording are reworded under item 17 without widening the regex, and its ubuntu-latest JUnit is the artifact that v1-53's and v1-01's claims read through gh.
quote: `agent-run run --coverage` (cargo-llvm-cov 0.9.1 under nextest). In CI this is the `test` job (matrix `[ubuntu-latest]`). Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the `test` job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`
@carried n=6 run=2 label=test-plan fact=fct-ryyndnld
quote: Gates are enforced in the `test` job on `ubuntu-latest` with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.
@carried n=6 run=3 label=test-plan fact=fct-3mj52n2x
quote: Excluded via the separator-agnostic `--ignore-filename-regex` `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`, the harness const `COVERAGE_IGNORE`. llvm-cov reports Windows paths with `\`, so the forward-slash form never matched there (as measured at chunk 2026-09-24-quality-gates, research.md M5).
@carried n=6 run=4 label=test-plan fact=fct-b4avhaux
quote: NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate. The separator class that makes it match Windows paths is not a widening, because the four trees are unchanged.
@carried n=6 run=5 label=test-plan fact=fct-2ztzeoct
quote: Coverage below threshold for tier (per OS)
@carried n=6 run=6 label=test-plan fact=fct-plbmhdnw
quote: Test failure (any level, any OS)
@carried n=6 run=7 label=test-plan fact=fct-mos7j36w
quote: `target/nextest/ci/junit.xml`, the report of the job's single instrumented `run --coverage` nextest invocation: `junit-${{ matrix.os }}` (one path);
@carried n=6 run=8 label=test-plan fact=fct-kprehror
quote: The agent reads job conclusions with `gh run view --json jobs` and fetches artifacts with `gh run download`.
@carried n=7 label=test-plan/ci-integration fact=fct-4otx4vxd
why: The local test gate must read green after the dependency move and the remnant removal, through the harness's unit, integration and E2E runs.
quote: `scripts/agent-run.sh run --unit`
@carried n=7 run=2 label=test-plan/ci-integration fact=fct-a43jfmhh
quote: `agent-run run --integration`
@carried n=7 run=3 label=test-plan/ci-integration fact=fct-ijfdjbph
quote: `agent-run run --e2e` on `ubuntu-latest`
@carried n=7 run=4 label=test-plan fact=fct-wxdsyw3l
quote: **Unit / Integration / E2E:** `agent-run run --unit`, `--integration` and `--e2e`, the local equivalents of CI's one instrumented coverage run.
@carried n=8 label=test-plan/ci-integration fact=fct-bfdvmti3
why: The perf job must read green at the wrap, and its per-OS wording follows item 17's Linux-only remnant removal.
quote: Its own job `perf (${{ matrix.os }})` (`fail-fast: false`, `AGENT_RUN_KEEP_HOMES=1`): checkout, toolchain, rust-cache, `cargo install --locked hyperfine@1.20.0`, `jq --version`, `run --perf`, G2 (probe then check), `schema-check`, `secret-scan`, the scan-gated uploads, then `gate --require perf`.
@carried n=8 run=2 label=test-plan fact=fct-yoazurel
quote: `viola hook session-end` (synthetic payload via `--input`, live wrapper), per OS
@carried n=9 label=test-plan/quality-gates-coverage-targets fact=fct-ftncprbj
why: The re-read perf job must pass with every row present, and the plan's Windows dev-host perf reading is reworded as item 17 requires.
quote: A failed build (`build`), boot (`boot: <reason>`), row (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or skipped row.
@carried n=9 run=2 label=test-plan fact=fct-vxnii5yh
quote: Host reading at chunk 2026-09-28-hook-perf-gate (Windows dev host, release build): each row's `max` 72.8–73.7 ms.
@carried n=10 label=test-plan/ci-integration fact=fct-j6sjgyk3
why: The fuzz-replay job is one of CI's seven jobs and must read green at the wrap.
quote: ci.yml `fuzz-replay` (ubuntu, push and PR): `rustup toolchain install` (root, then `working-directory: fuzz`), `cargo install --locked cargo-fuzz@0.13.2`, `agent-run run --fuzz-replay` (`-runs=0` corpus replay), then `gate --require fuzz-replay`.
@carried n=11 label=test-plan/ci-integration fact=fct-2qyicniz
why: The release check and its probes must read green at the wrap, still building viola alone after the dependency move.
quote: `bash scripts/release-check.sh --probe` (last line `release-check probes: 5/5 refused, control clean`), then `bash scripts/release-check.sh`, which runs `cargo build --release --locked --bin viola --message-format=json` (fake agent excluded: feature off) and judges that build's own `compiler-artifact` records. It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable, and never lists `target/release/`. A plain exit-code job: no upload, no `viola-harness gate` step
@carried n=11 run=2 label=test-plan fact=fct-thnlptgc
quote: Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
@carried n=12 label=test-plan/ci-integration fact=fct-k7t43itf
why: Each re-read job ends in the harness gate, which must exit zero on the wrap's outputs.
quote: `viola-harness gate`, which reads `junit-*.xml`, `run-summary.json`, the llvm-cov JSON summary and the hyperfine JSON, and exits non-zero on any §10 breach
@carried n=12 run=2 label=test-plan fact=fct-bzvycnyj
quote: **Quality gates:** `viola-harness gate` reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any §10 breach.
@carried n=13 label=test-plan/test-data-fixtures fact=fct-6wjlyq3w
why: Item 1 decides whether trycmd under tests/cmd is the tool or the text changes, and this row follows that decision; item 17's edits to tests/cli_output_plain.rs must keep the HELP literal holding.
quote: trycmd `tests/cmd/*.toml`, with snapbox redactions for timestamps, cursors and paths; `viola --help` (the `HELP` literal, `tests/cli_output_plain.rs`), the `viola list` board (`tests/cli_list.rs`)
@carried n=13 run=2 label=test-plan fact=fct-puo2hazj
quote: **Human-mode expected output:** trycmd cases with snapbox redactions; `viola --help` and the `viola list` board are literals in their test files.
@carried n=14 label=test-plan/test-data-fixtures fact=fct-2mxqmdgt
why: Item 7 asks the founder's word on the socket-directory pair's wording, and this fixture row states that pair as the tests set it.
quote: Set by the test on the real directory, never by viola, whose check never chmods it: `tests/security_negatives_socket_dir.rs` `Form` (`apply` · `assert_as_found` · `restore`), a directory at 0755 or a symlink to 0700, the refused directory asserted as found.
@carried n=14 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-zsbxg2jw
quote: a loosened socket directory is made on the real file too, at 0755 or as a symlink to 0700 (`Form` in `tests/security_negatives_socket_dir.rs`), refused by the product's own check and read back as found, and the pure `socket_dir_verdict` takes a foreign owner's uid by injection at the unit tier
@carried n=15 label=test-plan/test-data-fixtures fact=fct-mw3qahin
why: Item 15 measures the channel-handle no-leak property on Linux under the fake agent, beside the fake agent's Unix-only fds receipt.
quote: `fds` {`fds`}: Unix only
@carried n=16 label=test-plan/test-data-fixtures fact=fct-vrevms4b
why: v1-53's one live viola run on Linux must confirm the fake agent's Unix behaviour stated here, and the live reading stays a local measurement that never stands as a fake-agent proof or a CI step.
quote: `--exit-no-eof` (a bounded re-exec holder keeps the child's console / PTY slave open for 10 s — not viola's stdout — writes a `hold` receipt, and on Unix spawns in its own process group)
@carried n=16 run=2 label=test-plan fact=fct-7o4nbra2
quote: on Unix every command it spawns (each hook, and the status line command) runs in a process group of its own (`process_group(0)`, as the `--exit-no-eof` holder does): its PTY's foreground group is what a hangup of that terminal reaches, at the agent's exit as its session leader or when a killed wrapper closes the PTY, and a `viola hook` reached in its exit-time coverage write leaves a cut-short profile (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`). The agent itself, its PTY's session leader, cannot leave that group (`setpgid` refuses a leader), so on Unix its exit ignores SIGHUP from its first step (`\x03` or stdin's end)
@carried n=16 run=3 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=16 run=4 label=test-plan fact=fct-lbxoplwk
quote: a route entry's own live measurement of a real session is local too and never a CI step.
@carried n=17 label=test-plan/quality-gates-coverage-targets fact=fct-s6rhzqu4
why: This records the fake agent's process-group mechanism and its census, the Unix behaviour that v1-53's live run confirms, and it holds the coverage merge the test job must keep green.
quote: A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).
@carried n=18 label=test-plan/mocking-stubbing-discipline fact=fct-6xaoh2kl
why: Item 8's survivor is recorded, not killed: the peer euid decision is held only by injection, and the chunk must add no env, flag or feature seam to reach it.
quote: `peer_euid_admitted(observed, own)` takes its euids as arguments (admitting `observed == Some(own)`; an unreadable euid refused) and the private `serve_admitting` takes its decision as an `Admission` argument (`crates/viola-channel/src/server.rs`): dependency injection, with no env, flag or feature seam
@carried n=19 label=test-plan/quality-gates-coverage-targets fact=fct-ocdr3xdn
why: Item 8 is this recorded survivor, to be read at the Version done-check and kept as a by-construction survivor, with no seam added.
quote: A mutant replacing a client seam's `peer_admitted` read (`connect` in `src/cmd/client.rs` and in `src/cmd/hook.rs`, `frame` in `crates/viola-mcp/src/tools.rs`) with an always-admit survives every committed test by construction: one account can make no foreign peer, and no seam may be added (security-plan §Security Anti-Patterns, Universal). The mutation audit reads that same survivor at the route entry "Version done-check".
@binding n=1
**ci-integration ↔ mocking-stubbing-discipline**: the portable-pty release that replaces =0.8.1 must clear RUSTSEC-2026-0441 with no ignore widening (test-plan/ci-integration fct-zwsm4wxk). It must also keep viola-pty a sync crate that passes the sole-root tokio ban and the tokio-free check (test-plan/ci-integration fct-3gkvok74) (test-plan/ci-integration fct-pk32flwi), build at MSRV 1.96 (test-plan/ci-integration fct-whs3hmpm), and keep the Pty seam whose mockall double the consumers are tested against (test-plan/mocking-stubbing-discipline fct-sik7tvk4).
@binding n=2
**ci-integration ↔ quality-gates-coverage-targets**: CI's seven jobs (lint, supply-chain, test, perf, msrv, fuzz-replay, release) must read green at the wrap, each read through gh run view (test-plan fct-kprehror), and each ending in its harness gate where it has one (test-plan/ci-integration fct-k7t43itf). The test job's floors of 85 lines, 95 functions and 80 regions must hold while COVERAGE_IGNORE's Windows-path comment and the per-OS wording are reworded under item 17, with the regex never widened (test-plan/ci-integration fct-sd3lvsrr) (test-plan fct-b4avhaux). The perf suite must pass with every row present (test-plan/quality-gates-coverage-targets fct-ftncprbj).
@binding n=3
**test-data-fixtures ↔ quality-gates-coverage-targets ↔ ci-integration**: v1-53's one live viola run on Linux must confirm the fake agent's Unix process-group and SIGHUP behaviour (test-plan/test-data-fixtures fct-vrevms4b) (test-plan fct-7o4nbra2), the same behaviour the profraw census records (test-plan/quality-gates-coverage-targets fct-s6rhzqu4). Its claim reads the ubuntu-latest JUnit artifact (test-plan fct-mos7j36w), and the live reading stays a local evidence measurement, never a CI step (test-plan fct-lbxoplwk).
@binding n=4
**test-data-fixtures ↔ the body (human-mode expected output)**: item 1's masters-agree decision of trycmd tool versus text sets this row's wording (test-plan/test-data-fixtures fct-6wjlyq3w) and the seed line beside it (test-plan fct-puo2hazj). Item 17's edits at tests/cli_output_plain.rs:104,116 must keep the HELP literal assertion holding.
@binding n=5
**test-data-fixtures ↔ mocking-stubbing-discipline**: the socket-directory pair, a directory at 0755 or a symlink to 0700, is worded in both the fixture row (test-plan/test-data-fixtures fct-2mxqmdgt) and the filesystem-faults row (test-plan/mocking-stubbing-discipline fct-zsbxg2jw). The founder's word on item 7 must move both rows together.
@binding n=6
**mocking-stubbing-discipline ↔ quality-gates-coverage-targets**: item 8's always-admit survivor stays recorded by construction (test-plan/quality-gates-coverage-targets fct-ocdr3xdn) because the peer euid decision is held only by injection, with no env, flag or feature seam (test-plan/mocking-stubbing-discipline fct-6xaoh2kl). The chunk must add no seam to kill it.
@binding n=7
**test-data-fixtures ↔ the scope's item 15**: the channel-handle no-leak property must be measured on Linux under the fake agent through /proc/<child pid>/fd, beside the fake agent's Unix-only fds receipt (test-plan/test-data-fixtures fct-mw3qahin).
