# Task — tests · 2026-10-26-home-and-code-bearing-file-integrity

## The chunk
Closes the dated strict-modes exceptions: the home strict-modes check (`viola_state::strict::check_instance`) runs before any read or open of the home at every entry point (CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, `viola hook`'s nine events failing open, `viola mcp`'s frame tools, `verify`'s `update_stamps` stamps read, `run` over the home, `instances/<name>/`, `bin/`, `plugin/`, `ui/`, the trusted files and the Windows `conpty/` companions), always before `client::start`'s role-file open.
Makes the home owner-only (Unix 0700 dirs / 0600 files set explicitly; Windows a protected owner + SYSTEM DACL), adds the Windows owner + DACL check before any `diagnostics/detail-*.ndjson` or role file, proves `check_instance` on Windows (CI `windows-2025`), reads the DACL of a home `seed_conpty` made first and moves that seed so viola creates the home first.
Re-hashes the pinned `bin/<version>-<hash>/viola` before reuse and owns the `pin_exe` race of two first `run`s on a fresh home; covers `plugin/` rewritten atomically per start with absolute pinned commands, the `marketplace/` tree's integrity, and a test or guard proving the single writers of `snapshot.json` and `ledger/stamps.json`.
Re-measures whether a rebuild of one tree can read as another binary (two clean release builds compared by file and by section); re-reads whether the per-start plugin rewrite and the single-writer rule already hold.
The readers' move onto `read_snapshot_or_replay` (test-plan §6 E5 steps 1–3) is a founder P4 fork: neither planned in nor out until his answer.

## Reached files
- src/cmd/client.rs · edits start (the check before the role-file open)
- src/obs.rs · edits open_role_file (narrows an existing home to 0700)
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits the hook client's snapshot read through `VIOLA_DIR`
- src/cmd/hook/statusline.rs
- src/cmd/run.rs · edits start (the per-entry-point checks, the collision check's snapshot read, the per-start `plugin/` rewrite)
- src/cmd/verify.rs
- src/cmd/plugin.rs · edits the `marketplace/` tree write
- src/conpty.rs · edits the Windows `conpty/` companions load
- crates/viola-state/src/strict.rs · edits check_instance
- crates/viola-state/src/fs.rs · edits create_private_dir
- crates/viola-state/src/liveness.rs · edits live_endpoint
- crates/viola-state/src/stamps.rs · edits update_stamps
- crates/viola-state/src/pin.rs · edits pin_exe
- crates/viola-state/src/replay.rs
- tests/cli_controls_not_disableable.rs · edits the 0770 `--home` negative (exit 21), the table's completeness case
- tests/support/home.rs · edits seed_conpty
- tests/hook_statusline.rs · re-reads the three cases refused on `windows-2025` (CI run 38080061631)

## Needs
- The fixed order: the strict-modes check before any read or open of the home, never after a role-file open (a check after it passes a home at 0770), on `send`, `wait`, `last`, `answer`, `pause`, `release`, the hook's nine events, `verify` and `run`, on Unix and Windows.
- The strict-modes refusal per surface: CLI frame verbs exit 21 `instance-unreachable`, `run` / `mcp` / `revive` exit 1, hook exit 0 with no body and empty stderr, MCP `instance-unreachable` with `detail: null`; no frame written and no line or output holding a pid, start time, path or endpoint.
- The security-vector triggers this chunk's negatives answer: CLI and env (`--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0, no env / flag / config disabling a control) and filesystem (group/world-writable home or instance dir refused, pinned-exe hash mismatch exit 1, `plugin/` rewritten on start, Windows DACL owner / ACE cases where the runner allows, stubbed otherwise).
- `tests/cli_controls_not_disableable.rs`: its 0770 `--home` negative (exit 21) and the completeness case over every `VIOLA_*` var, global flag and `config.json` key (test-plan §5 CLI, no setting that disables a control).
- Windows strict-modes: the owner + DACL check over an instance tree refused on `windows-2025` (no case does today), the DACL read back in its canonical form, foreign-owner SID and non-persistent-ACL volumes as synthetic SDDL descriptor stubs at the check function (test-scope untestable zone).
- The seeded home: `seed_conpty` creating a test home before viola runs (the §5 setup carve-out and the §7 on-disk seed exception) must let viola create the home first, and the three `hook_statusline` cases a seeded home failed on `windows-2025` are answered by reading that DACL.
- The Windows `--home` outside the profile given a protected DACL (security control negatives, its test home never under `%TEMP%`) and the Windows `RUNNER_TEMP` negative case (§6, CARRY 2).
- Owner-only modes umask-independent (security control negatives), `diagnostics/` files 0600 (error sanitization suite), Unix modes and strict-modes in `viola-state`'s unit cases.
- The pinned exe: re-hash before reuse, a tampered pinned exe exits `run` 1 (Path 1), the pinned-exe content hash pinned by literal vectors (contract suite); the `pin_exe` race measured as concurrent unstamped two-instance harness boots failing, and the harness's one-instance Path 7 boot that works around it.
- The per-start `plugin/` rewrite: a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the instance's `settings.json` override (Path 1 verification signal).
- Single writers: only the wrapper writes `snapshot.json`, only `viola verify` writes `ledger/stamps.json` (security control negatives), and the test-data ban on hand-writing either with its carve-outs (chaos corruption of product-written files, `seed_conpty`, the server-identity impostors' test-built `snapshot.json`).
- `verify`'s `update_stamps` reading `ledger/stamps.json` under strict-modes; stamps come only from `viola verify` against the fake agent, and a case needing a viola-made home starts from `stamped_home`.
- `viola mcp`'s frame tools refusing through `live_endpoint` on a failed check; the MCP handler tests' `TestWrapper` (`crates/viola-mcp/tests/handlers.rs`, its snapshot written through `write_snapshot`) must stand on a home that passes the check.
- The hook fail-open matrix (§6 Security sweep) gains a strict-modes failure of the `VIOLA_DIR` home; `statusline_command` must not run when the home fails strict-modes (Path 6 and the hook stdin trigger).
- Indirect, through the hook's added check: the spine hooks (`session-start`, `user-prompt-submit`, `stop`, `pre-tool-use`) stay under `viola_core::SPINE_DEADLINE` on hyperfine `max` and inside the fail-open matrix's `Instant` bound, and `CONNECT_DEADLINE` stays below it.
- Indirect, through every booted-wrapper and integration test: each per-test home under `target/e2e-home` that viola creates must pass the new checks at every verb, and a path oracle compares `resolved(home)`.
- The Windows ConPTY companions under `bin/<version>-<hash>/conpty/` joining the strict-modes set, while a failing companion still degrades silently to the inbox ConPTY; the root tests' children hosted on the sideloaded ConPTY on Windows x64 (§5 Module ↔ PTY).
- `viola plugin install`'s `marketplace/` tree: an unknown entry left as found and the installed `.mcp.json` `command` not re-hashed (the §5 CLI `plugin install` cases).
- The build-reproducibility measurement: two clean release builds of one tree compared by file and by section; the `--release` build route of §9 Release build and §10's perf `cargo build --release` are the builds it compares against.
- E5 snapshot corruption → replay recovery (steps 1–3, its as-landed note and the two signals set against what landed), reached only if the founder's P4 answer moves the readers: the classified read taken only after `check_instance`, a replayed snapshot carrying no `pid` or `started_at` so no client writes a frame on it.
- Multi-OS proof: the Windows legs proved only on CI's `windows-2025` runner, `#[cfg(unix)]` / `#[cfg(windows)]` cases on their own runner, the two fixed orders measured on Windows too.

## Standing conventions
- S1 · test functions named `<subject>_<condition>_<expected>` in snake_case, rstest `#[case::label]` tables (literal tables in `viola-core`, labelled case tables in `viola-e2e`) · §2 Test Strategy naming / §4 Unit conventions (test-strategy.md, unit-test-strategy.md)
- S2 · integration binaries named `<surface>_<topic>.rs` (`cli_`, `hook_`, `security_negatives_*`, `chaos_`, `contract_`), helpers shared from `tests/support/` and never copied, live-appended files read through `tests/support/ndjson.rs` · §2 Test directory + naming conventions
- S3 · a fresh tempfile home per test at a not-yet-existing path under `target/e2e-home` that viola creates, wrappers in a `Drop` guard, nothing shared, env per child via `Command::env`, `.env_clear()` re-adding `LLVM_PROFILE_FILE` · §5 Setup / teardown lifecycle / §7 Test data lifecycle (integration-test-strategy.md, test-data-fixtures.md)
- S4 · no sleep for synchronisation; waits key on `events.ndjson` offsets, `viola wait --after` or a status field · §2 Agent-runnable invariants / §11 E2E
- S5 · state files (`snapshot.json`, `ledger/stamps.json`, `budget.json`) never hand-written; a reader's own unit case plants a malformed one only by altering the writer's bytes · §7 Seed strategies / §11 Test Data
- S6 · filesystem never mocked (a real tempfile home); mockall doubles only on existing seams; Windows foreign-SID / non-persistent-ACL descriptors only as synthetic SDDL stubs · §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md)
- S7 · after every E2E test the error sanitization and secret scan and the diagnostics schema conformance run: code-only error payloads, no absolute path, a hit reported by place and class · §6 Non-path suites (e2e-test-strategy.md)
- S8 · coverage gated per OS (lines 85, functions 95, regions 80) and zero flakiness (`retries = 0`, no `#[ignore]` parking) · §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md)
- S9 · OS-branch code compiled and tested on its own CI runner, a Linux-only local run never proof for Windows or macOS · §1 multi-os-compat trigger / §9 Matrix builds (ci-integration.md)

## Not needed
- Paths 2–5's own behaviour (the send confirmation and CL-1 events, `wait` / `last` wake rules, dialog decision bodies, the wheel and `release`): the chunk adds only the check ahead of their frames.
- E1 (R5 unwrapped hooks), E2 (R8 `CLAUDE*` strip) and E3 (`link` / `unlink`) scenarios.
- The `list` row integrity suite and `viola list`'s human-mode SGR and colour rules.
- The property suite's parser properties (`validate_paste_text`, ndjson framing, hook and agents parsers, vt100 feed, `prompt-submitted` normalisation) and the cargo-fuzz targets.
- The fixture hygiene walk, the `viola verify` recorder scrub and the recorded MCP exchange replay.
- The fake agent's turn scripts, framing, compact and clear replays and its mode words.
- Unix socket and socket-directory hardening, the client euid check and macOS server verification: "Unix endpoint and home hardening".
- Bounded readers (`take(MAX_FRAME)`) at every boundary: "Bounded inputs at every boundary".
- The exit-cause matrix `cross_exit_causes.rs` and any new exit-cause code or per-cause hint: "Exit-cause code catalogue".
- Live `viola verify` against the real CLI (`run --local-live`) and readings that need an interactive Windows host: "Windows-only live measurements".
- The mutation gate: judged only at the epoch boundary by `/andromeda-code-audit`, never per chunk.
- The budget governor's threshold decision, override expiry and `budget-gate` events.
