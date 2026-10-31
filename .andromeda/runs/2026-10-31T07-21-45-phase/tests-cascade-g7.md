# tests · cascade read · group 7 of 7

## Carried
- (.andromeda/test-plan-detail/test-data-fixtures.md:10) The status-line row's witness and the three `config.json` row witnesses must run in a home that passes strict-modes on every OS they run on, starting from `stamped_home` where viola must have made the home.
  > A home made with `create_dir_all` before viola is refused: on Unix (0755) by the read rule; on windows-2025 `Writable`, the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, as measured at ci#40439188053); that home's canonical SDDL is not recorded. A case that needs a home viola created on every OS starts from `stamped_home`, as the every-OS `hook_statusline` cases that run the arm's whole course do.
  (.andromeda/test-plan.md:637)
  > a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`:
- (.andromeda/test-plan-detail/test-data-fixtures.md:30) An oversize `config.json` planted before viola reads it must sit in a privately made home, and a CLI witness that drives a failure with a path in play through a planted regular file must keep this catch-site form and its per-OS oracle.
  > A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a `create_dir_all` home is refused, On-disk product state above), as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do. A catch-site case holds a viola-made home with a regular file where the checked entry goes (`tests/cli_wait_last.rs`, `diagnostics/`; `tests/cli_verify.rs`, `ledger/`). A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).
- (.andromeda/test-plan-detail/test-data-fixtures.md:14) The status-line row's cap-driving witness must plant its source through these helpers, written by the test and never by viola.
  > Written by the test, never by viola, which only reads it: `tests/support/home.rs` `plant_statusline_source(home, command)`, with the command from `statusline_echo_command(marker, extra)` and the marker at `statusline_marker(home)`. A default-home case takes `TestHome::default_of_user` and `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child.
  (.andromeda/test-plan.md:643)
  > - **The statusline source:** written by the test, never by viola, which only reads it.
- (.andromeda/test-plan-detail/test-data-fixtures.md:16) The child pipe drain row's cap-driving witness must take this fake `claude` in a test-scoped bin dir, its `oversize` mode set by a file beside the exe and by no env var.
  > `viola-fake-agent` placed as `claude` in a test-scoped bin dir (`tests/support/fake.rs` `claude_bin` / `claude_in`; `empty_bin` for a `PATH` holding no `claude`), answering `agents --json` from files beside its exe: `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json` (planted rows) and the live `sessions/<pid>.json` registry its interactive runs write; no env var (the founder's own answer to P4 fork 2 of chunk 2026-10-16-unwrapped-sessions-on-the-board, `inputs#I1`); every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on`, or a plain `spawn` of `list` under an empty bin dir)
  (.andromeda/test-plan.md:645)
  > - **`claude agents --json` output:** `viola-fake-agent` placed as `claude` in a test-scoped bin dir, answering from files beside its exe with no env var; every test-started `list` / `revive` runs under a test-scoped `PATH`.
- (.andromeda/test-plan-detail/test-data-fixtures.md:18) Where a changed MCP error body meets a recording, the recording must change only through `viola-mcp-tap` with its scrub and refusal, stay valid against its schema and replay, and pass the hygiene walk.
  > Checked-in `fixtures/claude/<cli-version>/Mcp.<variant>.json` (`Mcp.tools-list-1.json` in `2.1.302`), recorded by the test-only `viola-mcp-tap` (`crates/viola-e2e`): `record` sits between one live `claude -p` start and a direct spawn of `viola mcp`, one append per direction-tagged line through `take(MAX_FRAME)` into a 0600 capture that is never committed; `fixture` keeps the client frames up to and including the first `tools/list` request and each request's result keyed by method, through the `--record` scrub and refusal (closed codes `home-path` · `absolute-path` · `username` · `email`, the file name only). Validated against `schemas/claude-mcp-fixture.v1.json` and replayed by `tests/mcp_recorded_exchange.rs`.
  (.andromeda/test-plan-detail/test-data-fixtures.md:80)
  > the one MCP exchange of `2.1.302`, `Mcp.tools-list-1.json`, against `schemas/claude-mcp-fixture.v1.json` (`v` const 1; `client` JSON-RPC 2.0 frames that contain a `tools/list` request with an `id`; `results` keyed by method, `tools/list` holding a `tools` array of named tools) with the same path, username and email walk (`mcp_fixtures_pass_hygiene`; planted `planted_clean_mcp_exchange_passes`, `planted_dirty_mcp_exchange_is_rejected`, `planted_mcp_schema_violation_is_rejected`);
  (.andromeda/test-plan.md:649)
  > - **Recorded MCP exchange:** checked-in `Mcp.<variant>.json`, recorded by the test-only `viola-mcp-tap` through the same scrub and refusal, schema-validated and replayed.
- (.andromeda/test-plan-detail/test-data-fixtures.md:55) The planted stripped `CLAUDE*` value's witness must prove its strip by name from this receipt and find the value's absence in the error bodies themselves, because the receipt never holds a value.
  > - `env` {`names`}: sorted, names only, never a value
  (.andromeda/test-plan.md:670)
  > an `env` receipt holds names only, never a value.
- (.andromeda/test-plan-detail/test-data-fixtures.md:69) The status-line and pipe-drain witnesses must be built on these two mode words, and any oversize answer they need must add no env var to the fake agent.
  > Beside the options the agent takes two mode words. `statusline-echo`, landed at chunk 2026-10-10-statusline-pass-through: it prints its output and files its stdin, and its exit is the code after the marker path.
  (.andromeda/test-plan-detail/test-data-fixtures.md:70)
  > `agents` (chunk 2026-10-16-unwrapped-sessions-on-the-board): a first word `agents` answers `agents --json` from files beside its exe (`agents-mode`, `agents-rows.json`, and the live `sessions/<pid>.json` registry: a row written just before its launch SessionStart fires, its `cwd` the launch fixture's recorded `cwd`, `~` without one; a dead or reused pid skipped through one `viola_state::liveness::start_times()` read), built on the recorded `Agents.recorded-1.json`; no option and no env are added;
  (.andromeda/test-plan.md:674)
  > plus argv options (no env) for the statusline tests, verify's interactive runs, the test-only holds and screen switches and `viola revive`'s `--resume` / `--fork-session`, two mode words (`statusline-echo`, `agents`), and unknown arguments ignored.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) A handler fault forced for `-32603` must go through an existing seam trait or a real process, adding no double of a new trait and no failpoint crate, and the pipe-drain witness's `claude` must be the real stub executable.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only.
  (.andromeda/test-plan.md:728)
  > - **Process mocking:** mockall `automock` on seam traits only; external processes are real stub executables (`viola-fake-agent`).
  (.andromeda/test-plan.md:732)
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
  (.andromeda/test-plan.md:920)
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) A test-owned wrapper that plants upstream text in an error frame, or stands in for a server that is not the recorded wrapper when an `instance-unreachable` body is enumerated, must be a real `viola_channel` listener on the interprocess server API.
  > interprocess 2.4.4 server API (squatter listener; impostor listener); `viola-fake-agent` as the fake CLI;
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19)
  > in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener: `tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`),
  (.andromeda/test-plan.md:730)
  > - **Fake servers:** the interprocess server API binds the squatter before `run`, and `viola-fake-agent` is the fake CLI; no HTTP fake servers.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) A file fault driven for the path-in-play witnesses or for `-32603` must happen on a real home and be read through the product's own check, never through a stubbed filesystem.
  > none (a real tempfile home, real modes and ACLs)
  (.andromeda/test-plan.md:713)
  > - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
- (.andromeda/test-plan-detail/ci-integration.md:10) The plan must decide whether the `Refusal` carry, reaching the `.cmd` / `.bat` child refusal and `resolve_program`, lists this check-only Windows-target clippy as a chunk gate entry, since the Linux dev host does not build that `cfg(windows)` code.
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
- (.andromeda/test-plan-detail/ci-integration.md:31) The Path 1 watch's tally on this chunk's CI reads must read `path1_run_start_one_home_in_order` in this per-OS test job's single instrumented run, whose timing is the one the 20 s kill line is measured against.
  > **Runs:** `agent-run run --coverage` (cargo-llvm-cov 0.9.1 under nextest). In CI this is the per-OS test job;
  (.andromeda/test-plan-detail/ci-integration.md:31)
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it.
  (.andromeda/test-plan.md:753)
  > - **Coverage report:** `agent-run run --coverage` is the per-OS test job, on `windows-2025` after the ConPTY vendor verification; its one instrumented nextest run replaces the unit, integration and E2E rows, and each job ends in `viola-harness gate --require <suites>`.
- (.andromeda/test-plan-detail/ci-integration.md:39) P3's reading of the `v1-47` secret-scan leg must count the scan where it stands in CI, this job and the `test` job, and a changed `channel-response` line must pass the `schema-check` steps these jobs run.
  > Its own job `perf (${{ matrix.os }})` (`fail-fast: false`, `AGENT_RUN_KEEP_HOMES=1`): checkout, toolchain, rust-cache, `cargo install --locked hyperfine@1.20.0`, `jq --version`, `run --perf`, G2 (probe then check), `schema-check`, `secret-scan`, the scan-gated uploads, then `gate --require perf`.
  (.andromeda/test-plan.md:777)
  > In the per-OS `test` job, every test artifact leaves CI only behind obs-plan §9's secret scan (`id: secret-scan`):
- (.andromeda/test-plan-detail/ci-integration.md:51) Any test-only seam added to force a handler fault must stay behind `fake-agent`, so this job keeps judging a release build of `viola` alone.
  > `bash scripts/release-check.sh`, which runs `cargo build --release --locked --bin viola --message-format=json` (fake agent excluded: feature off) and judges that build's own `compiler-artifact` records. It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable, and never lists `target/release/`.
  (.andromeda/test-plan.md:763)
  > - **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
  (.andromeda/test-plan.md:798)
  > - Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`

## Bindings
- **Test data ↔ mocking (the pipe-drain witness)**: the fake `claude` answering from files beside its exe (.andromeda/test-plan-detail/test-data-fixtures.md:16) is the real stub executable the process-mocking row requires (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18); the witness must satisfy both, with no env var and no double.
- **Mocking ↔ CI (the forced handler fault)**: a fault forced through a seam (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) must not reach the release build the release job judges (.andromeda/test-plan-detail/ci-integration.md:51), and the plan may add no failpoint crate to do it (.andromeda/test-plan.md:920).
- **Test data ↔ fixture hygiene (a changed MCP recording)**: the tap's scrub and refusal (.andromeda/test-plan-detail/test-data-fixtures.md:18) and the hygiene walk (.andromeda/test-plan-detail/test-data-fixtures.md:80) must both hold over any recording a changed error body moves.
- **Test data ↔ mocking (faults in a home)**: a catch-site failure on a viola-made home (.andromeda/test-plan-detail/test-data-fixtures.md:30) is the real-file fault the filesystem row requires (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20), so a path-in-play witness must take its failure from a real home.
- **CI ↔ the Path 1 watch**: the tally reads the per-OS test job's run (.andromeda/test-plan-detail/ci-integration.md:31), and its coverage instrumentation is part of the time that job records.
