@amendment marker=2026-11-05-linux-only-code-and-ci
title: the OS scope: Linux only, one CI leg
section: §1 Test Scope Summary (body: the tier justification and drivers, the multi-os-compat trigger, the library-crate notes; test-scope-summary.md: the new OS scope entry); §2 Test Strategy (body: Unit; test-strategy.md: the Unit and Performance tier depth, the multi-os-compat row); §11 Test Strategy rules
change:
  - The tier justification reads the six agent-driven surfaces on Linux (CI on `ubuntu-latest`) and four drivers beyond Standard (was three required OSes and five drivers); the driver «the creator brief requires multi-platform CI» is retired.
  - A new §1 OS scope entry: Linux only, the core on any current distribution, CI on `ubuntu-latest`, the window features of 0.2.0 on Hyprland; the Windows and macOS code, tests, fixtures and CI jobs are out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
  - The multi-os-compat trigger's source reads the matrix `[ubuntu-latest]`; its required test is the full workspace and fake-agent E2E on `ubuntu-latest` plus the OS-gate census (was the three runners and the named-pipe, ConPTY, DACL and macOS branch tests).
  - The library-crate tests run on the `ubuntu-latest` leg (was all three CI OSes); §2 Unit runs on Linux (was three OSes less the macOS compile-outs); the Unit tier depth runs on `ubuntu-latest`, the server check's captured-span cases ungated; the `perf` job runs on its one `ubuntu-latest` leg (was per OS); the multi-os-compat row reads §9 Matrix builds as one leg, `ubuntu-latest`.
  - The rule «NEVER treat a Linux-only local run as proof for Windows or macOS» is retired.
why: the founder's own rulings of 2026-11-04: Viola is Linux-only, the Windows and macOS code is cut and recoverable at the tag `multi-os-final`, and the masters change once, with the code, at this wrap.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the PTY seam on openpty alone, the ConPTY sideload retired
section: §1 Test Scope Summary (body: the viola-pty entity, Path 1; test-scope-summary.md: Path 1, the chaos no-EOF case); §4 Unit Test Strategy (viola-pty); §5 Integration Test Strategy (body: Module ↔ PTY, Setup / teardown, DA1 answer; integration-test-strategy.md: the PTY test approach, Setup / teardown lifecycle, Readiness stall report, DA1 answer, Runner-inherited grants); §6 E2E Test Strategy (body: the tui driver; e2e-test-strategy.md: the tui row); §11 E2E rules
change:
  - viola-pty: the windows-sys `TerminateProcess` fallback leaves the §1 entity and §4, exit still detected on the handle; the E2E rule and the chaos no-EOF case lose «(ConPTY)».
  - Path 1's start sequence drops the fail-open ConPTY sideload step.
  - §5 Module ↔ PTY reads real openpty spawns: exit on the handle, a resize propagated, a key after a resize sent once the child reports the size (was ConPTY / openpty, a `.cmd` child refused, Windows x64 children on the sideloaded ConPTY); the windows-2025 key-loss readings, the H2 limit and the sideload boundary leave.
  - Setup / teardown has one carve-out, `tests/chaos_feed_panic.rs` (was two, the Windows x64 `seed_conpty` seeding first); the Windows plain-home reading and the seed timing leave.
  - The readiness stall report reads the last event, `/proc/<pid>/wchan` and the `stat` state (the macOS `sample` arm gone); the DA1 answer and the windows-2025 runner-inherited grants are retired.
  - The tui row's mode reads openpty (was ConPTY / openpty); its panic byte search reads the child's own bytes and none of viola's literals, with no per-OS split.
why: the report's Retired and Symbols facts: the ConPTY sideload, the `TerminateProcess` fallback and `PtyError::Load` left the code, and `pty_backend()` is always `openpty`; under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the channel on a Unix socket, the named pipe and the macOS carve-out retired
section: §1 Test Scope Summary (body: the viola-channel entity, Untestable zones; test-scope-summary.md: the viola-channel reason and as-landed, the ipc-internal surface); §2 (test-strategy.md: the IPC vector row); §4 Unit Test Strategy (viola-channel, viola-state's server check, root bin); §5 (body: Module ↔ IPC, the DACL read-back driver; integration-test-strategy.md: wrapper channel); §6 (body: the untestable list); §7 (impostor cases, `record_live`); §8 (Fake servers, Peer euid decisions); §10 (the hook timing)
change:
  - The ipc-internal surface is a Unix socket (was a named pipe / Unix socket); the viola-channel entity drops the Windows listener's remote-client and DACL admission cases.
  - Server verification is asserted on Linux (was Linux and Windows); the macOS no-peer-pid carve-out leaves §1, §4 and §5; the server-identity negatives, the hook impostor rows and the root bin's captured-span cases run ungated.
  - §1 Untestable zones (rewritten): the committed tests hold the real euid read on Linux (was Linux and macOS).
  - §4: endpoint vectors in `$XDG_RUNTIME_DIR/viola/` plus the `/tmp/viola-<uid>/` fallback (the pipe and macOS `$TMPDIR` forms gone); `peer_euid_admitted(observed, own)` admits `observed == Some(own)` (was a `reported` argument); the server pid read by `SO_PEERCRED` (the per-OS arms and Windows handle non-inheritance gone); `server_verdict` loses `peer_pid_reported`.
  - §5 drops the pipe DACL read-back, `security_negatives_channel.rs` and windows-sys; a frame over `MAX_FRAME` reads Linux's possible ECONNRESET (the Windows and macOS close readings gone); §6's untestable line reads socket modes read-back (was SDDL); §7 `record_live` drops the Windows `Serving` reason; §8 Fake servers drops the `CreateNamedPipeW` control pipes and `ImpersonateAnonymousToken`.
  - §10: the hook's socket-directory check and peer euid decision are real checks on every build (was on Unix only).
why: the report's Retired fact (the named-pipe endpoint and its SDDL, SQOS and remote-client controls; the macOS socket dir and no-peer-pid carve-out) and Symbols fact (`SERVER_PID_REPORTED` and `PEER_EUID_REPORTED` folded to their Linux values, an unreadable euid still refused, no seam added); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: strict-modes on Unix modes, the Windows owner/DACL arm retired
section: §1 (body: the viola-state and config / code-bearing state entities, Untestable zones, the Vector 7 trigger; test-scope-summary.md: the test-data strategy); §2 (test-strategy.md: the Filesystem vector row); §4 (body and unit-test-strategy.md: viola-state); §5 (integration-test-strategy.md: the on-disk checks); §6 (body: the untestable list); §7 (body and test-data-fixtures.md: On-disk product state, widened test homes, the widened seeded home, a home under a regular file); §8 (body: Filesystem; mocking-stubbing-discipline.md: Filesystem faults)
change:
  - The viola-state and config / code-bearing state entities read testable in a temp home (was partially testable for the Windows DACL cases); §1 and §6 drop the Windows ACL-volume / foreign-SID and macOS local-run bullets.
  - Vector 7: a read grant is a group or other bit; the ConPTY companion case, the Windows DACL owner / ACE cases and «on Unix» before `settings.json` leave.
  - §4 viola-state: the read rule is `unix_read_verdict`; `windows_read_verdict`, `READ_RIGHTS`, the SDDL stubs, the windows-2025 instance-tree, plain-home and board-scan cases, `check_socket_dir_off_unix_reads_nothing`, the `pin_companions` refusal and the Windows placement form leave; `events.ndjson` reads the write rule, a diagnostics file the read rule.
  - §5: modes as found (no ACL), the three-OS qualifiers and the seeded-home reason gone; the Windows outside-the-profile protected-DACL test leaves §5 and §2's Filesystem row.
  - §7: a `create_dir_all` home is refused (0755) by the read rule; the widened-home helpers are Unix modes (no `icacls`); the widened-seeded-home row is retired; a home under a regular file is a strict-modes refusal (the Windows `NotFound` arm gone).
  - §8: no SDDL stubs; Filesystem faults read real modes.
why: the report's Retired fact (the Windows home owner/DACL strict-modes arm) and Symbols fact (`windows_read_verdict`, `READ_RIGHTS`, `Refused::NullDacl`, `Refused::NoPersistentAcls`, `outside_profile`, the off-Unix `check_socket_dir` arm, `pin_companions` and `PinError::StrictModes` removed); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: program resolution on PATH, the .cmd/.bat refusal retired
section: §1 Test Scope Summary (body: the viola-agent-claude entity, the Vector 8 trigger; test-scope-summary.md: the CLI verbs entity, the cli surface, the 5-command requirements); §4 (body and unit-test-strategy.md: viola-agent-claude); §5 (body and integration-test-strategy.md: Module ↔ external API, verify's refusal codes); §6 (body: Chaos suite, Exit-cause matrix; e2e-test-strategy.md: the rewritten Exit-cause matrix and Chaos suite)
change:
  - npm-shim → `claude.exe` resolution reads program resolution: a given path as given, a bare name on `PATH` (`resolve_program`; `AgentError` holds `ProgramNotFound`, was also `BatchScriptChild`).
  - The `.cmd` / `.bat` child refusal (exit 1, `batch-script-child`) leaves the Vector 8 trigger, the 5-command readiness refusals, the Chaos suite and the Exit-cause matrix (its `.cmd` human pair and its revive row); `tests/cli_program_resolution.rs` and the fixture npm shim leave §4 and §5.
  - revive's closed `--json` refusal codes read ten (was eleven); verify's `process-exit` refusal codes read five beside `interrupted` (was six).
why: the report's Retired fact (the `.cmd`/`.bat` refusal, `batch-script-child`, `AgentError::BatchScriptChild` and its human pair) and Counts fact (revive's codes 11 → 10, verify's 6 → 5); `batch-script-child` left the closed words with its emitter by the overseer's technical answer of 2026-11-05 11:08Z; under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the statusline pass-through on every build
section: §1 Test Scope Summary (body: the hook entity, the budget governor path; test-scope-summary.md: Path 1's snapshot, the budget governor); §6 E2E Test Strategy (body: Path 1 and Path 6 signals; e2e-test-strategy.md: Path 1, Path 6 signal and as-landed); §7 (test-data-fixtures.md: recorded hook payloads, the statusline source)
change:
  - `viola hook statusline` passes the user's statusline through unchanged with no OS qualifier (was on Unix only, the Windows arm running no command); the arm runs `/bin/sh -c <command>`, `shell_argv` returning `[&str; 3]`.
  - `run` writes the instance's `settings.json` override at each start (the Windows no-write clause and the `cfg!(windows)` branch gone); the start's socket-directory check is stated unconditionally.
  - Path 6's two-instance leg prints the marker output and files the marker (the Windows no-marker arm gone); Run B's probe status-line recording loses «on Unix»; `Wrapper::boot_as_user` sets `HOME` (the `USERPROFILE` arm gone).
why: the report's Symbols fact: with `WRAPPED_HERE` folded, `statusline::shell_argv` returns `[&str; 3]` (was `Option`); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the R8 strip by exact name, the registry reader retired
section: §1 Test Scope Summary (body: the inherited credentials strip); §4 Unit Test Strategy (unit-test-strategy.md: the R8 strip); §6 E2E Test Strategy (body: E2 signal; e2e-test-strategy.md: E2 signals)
change:
  - The persistent-environment exception is `config.json` `claude_env_keep` (the Windows registry `Environment` names gone); the strip removes names by exact match (was ASCII case-insensitive on Windows); the floor and its literal stay the eleven, the differing Linux dev-host reading beside them.
  - E2: a `claude_env_keep` name survives and the child holds no viola socket or home path, with no Unix qualifier; the Windows handle non-inheritance line leaves.
why: the report's Symbols fact: `src/run/env.rs`'s registry reader and case-insensitive match left with `CASE_INSENSITIVE`, the exact strip, `claude_env_keep` and the eleven-name identity floor staying; under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: mutation without the Windows leg or the host scratch
section: §2 Test Strategy (body: machine-parseable output); §3 (5-command implementation: `run` Command, Classification, Verdict, Package, Output format, `gate` inputs, closed `reason`s); §9 CI Integration (body and ci-integration.md: Mutation, `mutants.out/`); §10 (quality-gates-coverage-targets.md: Mutation gate)
change:
  - `mutants.out/outcomes.json` is written at the repository root, the harness passing no `--output` (was under `<repo parent>/viola-mutants-scratch/` on a Windows host); `HOST_SCRATCH`, the scratch wipe, the `TMP`/`TEMP`/`--output` arm and the `scratch_bytes` key leave §3; `scratch-refused` and `scratch-wipe-failed` are recorded retired with `mutants/scratch.rs`.
  - The boundary audit's Windows leg `windows-mutants.yml` leaves §9; §10 reads that the audit has no Windows leg, that leg and the `cfg(windows)` code out of the tree at `multi-os-final` (`ecbe7ab3`), its dispatch runs kept as history and no Windows dispatch standing.
  - The host-unreachable rule names a `cfg` item proved false for the host or a const set by `cfg!` (the `HOST_SCRATCH` example gone); the two `prepare` misses stand recorded with `scratch.rs` deleted; the two real-cargo-mutants harness tests run ungated (was compiled out on macOS); the dev host's `TMPDIR` scratch stays an operator fact, the harness setting none.
why: the report's Symbols fact (`scratch.rs` deleted whole, its items serving only the Windows wipe) and Retired fact (`windows-mutants.yml` with `viola-mutants-scratch`, `HOST_SCRATCH` and `scratch_bytes`); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: one harness shim, agent-run.sh
section: §2 Test Strategy (test-strategy.md: the sync E2E suites, the socket-root helpers, the harness shim); §3 Test Harness Contract (body: Harness implementation; 5-command implementation: `boot` steps 2, 3 and 5, `cleanup` step 4, `secret-scan`, `pre-push`); §5 (integration-test-strategy.md: `Frozen`); §7 (test-data-fixtures.md: the socket root); §9 (ci-integration.md: Unit)
change:
  - `scripts/agent-run.sh` is the one thin shim (was the `{sh,ps1}` pair with byte-identical semantics); §9 Unit drops the pwsh arm; `pre-push` is forwarded by the shim and drops «The other OSes are CI's alone».
  - `tests/support`: the widen helpers are Unix modes; `Frozen` is a `SIGSTOP` guard (the Windows debugger attach and `run_refuses_a_frozen_wrapper_as_stale_on_windows` gone); `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR` (the macOS `TMPDIR` arm gone).
  - `boot`: viola makes the home 0700 (the Windows protected-DACL arm and plain create gone) and the fake agent is copied as `bin/claude` (no `[.exe]`); `cleanup` step 4 reads a connect to the socket path refused or the path absent, under `$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/` (the Windows pipe and macOS forms gone); `secret-scan`'s `mode_check` is `unix` (`skipped-windows` gone).
why: the report's Harness / gate surface fact (`scripts/agent-run.ps1` deleted, the harness `scripts/agent-run.sh` alone) and Symbols fact (`tests/support/home.rs` loses its `icacls` and `USERPROFILE` arms, `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR`, `frozen.rs` keeps the Unix `kill -STOP` twin); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the E2E witnesses on ubuntu-latest, three-OS greens as dated readings
section: §1 (test-scope-summary.md: the tui signal, the cli surface notes); §5 (body: CLI; integration-test-strategy.md: the concurrent-append check, forced-window placement, harness cleanup witnesses, CLI); §6 E2E Test Strategy (e2e-test-strategy.md: the cli row, Paths 2 to 5 and 7, E3, the terminated wrapper, `list` row integrity, the stale-row dim witness, the hangup case, the content hash, the Chaos suite and Security sweep readings)
change:
  - Paths 2, 3, 4 and 5 read covered in CI on `ubuntu-latest` (was all three CI OSes); Path 5's windows-2025 F-W3 arm leaves, the focus case holding the full assertion; Path 2's Windows `paste-newline` reading and its Windows not-measured item leave.
  - Greens read on the three runners stand as dated readings: run 38098238731, ci#40439188053, ci#40458923095 and ci#39383206773 as runs of the retired three-OS matrix, the forced-panic 0.414 s on the retired Windows host's debug build.
  - The stale-row dim witness is ungated, green on `ubuntu-latest` (was `cfg(unix)`, also `macos-latest`); the cli row drops the windows-2025 attribute set; `list` row integrity drops the Windows 200-column reading; the vt100-panic signal has no per-OS split.
  - The terminated-wrapper case is `kill -KILL` alone; the hangup case runs on `ubuntu-latest` (the macOS reasoning gone); the content-hash, `cli_slash_args`, `cli_output_plain`, board and concurrent-append cases lose their three-OS qualifiers; forced-window placement and the cleanup witness read `ubuntu-latest` alone; the chaos no-EOF case reads the Linux process-group condition (the ConPTY and macOS readings gone).
why: the report's Counts fact: the CI matrix moved from three OS legs to `[ubuntu-latest]`, and a dated `windows-2025` measurement that records history is re-worded, never left as current; under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the security sweep, control negatives and exit-cause matrix on Linux
section: §6 E2E Test Strategy (body: the Security sweep, Exit-cause matrix, Security control negatives and Chaos suite lines; e2e-test-strategy.md: the rewritten Security sweep, Exit-cause matrix, Security control negatives and Chaos suite)
change:
  - Security sweep: the broad line lists the not-the-recorded-wrapper, refused-socket-directory and other-euid hook cases with no OS qualifier, and the detail drops «on Linux and Windows»; the impostor rows run ungated (was `cfg_attr(not(target_os = "macos"), …)`); the exiting-wrapper Windows-pipe paragraph and its CARRY leave.
  - Exit-cause matrix: the strict-modes cause is a widened mode (the Windows `icacls` arm gone); the server-verification cause is the pid / start-time mismatch with no OS split, the macOS carve-out sentences gone; the `.cmd` / `.bat` cause and the `batch-script-child` row leave.
  - Security control negatives: the Windows client SQOS, the Windows `--home` outside-the-profile DACL, the Windows listener remote-client-rejection and DACL-denial bullets and the ConPTY companion bullet leave; the socket-directory code drops «off Unix the check returns `Ok(())`»; server identity is ungated; the strict-modes and single-writer cases lose «the three CI OSes» and «or ACL».
  - Chaos suite: the `.cmd` child case leaves; the vt100-panic case drops its OS qualifiers and its Windows sub-bullet.
why: the report's Files fact (`tests/security_negatives_channel.rs`, `tests/channel_sqos_open.rs`, `tests/conpty_sideload.rs` and `tests/cli_program_resolution.rs` deleted), Retired fact (the SQOS and pipe-ACL controls, the ConPTY sideload, the `.cmd`/`.bat` refusal, the macOS carve-out) and deviation 11 (`security_negatives_server_identity.rs` and the impostor cases made unconditional); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the CI matrix [ubuntu-latest] and two workflows
section: §9 CI Integration (body: Platform, Lint, Coverage report, Mutation, OS, Language version, the `test` job, `mutants.out/`, failure conditions; ci-integration.md: Lint, E2E, Coverage, workflows, concurrency, the `ci` kill line, the per-OS JUnit reader and reading); §10 Quality Gates (body: gate enforcement)
change:
  - The workflows are `ci.yml` and `nightly.yml` (was also the dispatch-only `windows-mutants.yml`), its tool line and `tests/contract_windows_mutation_scope.rs` gone; zizmor's declined `concurrency-limits` findings read 2 low (was 3), as measured at this chunk.
  - The OS leg is `ubuntu-latest`, the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release`, each job and artifact name keeping its `${{ matrix.os }}` form (was windows-2025, macos-latest and ubuntu-latest); Lint, E2E and Coverage read the one-leg matrix; the toolchain installs on the CI runner and the host.
  - The windows-2025 `ConPTY vendor verification` step, its probe, the `conpty_sideload` coverage binary and the ConPTY vendor failure condition leave; the dev-host Windows- and macOS-target clippy recipes leave Lint; the failure conditions drop «on any of the 3 OSes» and «on any OS».
  - The `ci` kill line reads 26 binaries (was 27).
  - Gates run in the `test` job on `ubuntu-latest` (the per-OS gating rationale gone); the per-OS JUnit reader `scripts/junit-os-sets.py` and its dated per-OS reading are retired.
why: the report's Counts fact (workflows 3 → 2, the matrix → `[ubuntu-latest]`, zizmor 3 → 2, the kill line 27 → 26) and Harness / gate surface fact (the ConPTY step and the `pwsh shim` steps left, `windows-mutants.yml` deleted, `nightly.yml` unchanged); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the recorded supply-chain red, RUSTSEC-2026-0441
section: §9 CI Integration (ci-integration.md: the Supply-chain stage)
change:
  - New: the `supply-chain` job reads red on one advisory, RUSTSEC-2026-0441 (`pin-utils` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1`), as measured at ci#42030025050; its owner is the CARRY on «Version done-check», whose remedy moves `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`; the ignore list is not widened.
why: the overseer's technical answer of 2026-11-05 11:51Z and the operator's own word of 12:02Z, approving the review card's two Beyond-the-plan rows as written: the advisory is owned by a CARRY on «Version done-check», no ignore widened, the 0.9.0 Windows bug behind the `=0.8.1` pin leaving with this chunk.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the OS-gate census reads another OS as a fault
section: §6 E2E Test Strategy (e2e-test-strategy.md: the census of OS-gated tests, the census count); §2 Test Strategy (test-strategy.md: the multi-os-compat row)
change:
  - `faults` reads three ways: an unnamed gate, a row that no longer stands, and a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`) even when a row names it, its parse catching a multi-line `cfg_attr` predicate a line grep cannot see; the self-check `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` joins, the eight planted cases building the Windows word from a `WINDOWS` const.
  - The census count reads 39 OS-gated test functions and 2 gated files, 41 `GATES` rows, as measured at this chunk (was 52, 7 and 59, its rule taking `tests/hook_fail_open.rs`'s `cfg_attr` rows).
why: the report's Harness / gate surface fact (the census's new fault class, its self-check re-planted) and deviation 9: the new class caught a multi-line `cfg_attr(not(target_os = "macos"), …)` pair in `tests/hook_fail_open.rs` that the line grep read clean, and both cases now run unconditionally.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the relied set, all twenty-four rows on every build
section: §3 (5-command implementation: `boot` step 4, `run --local-live`); §4 Unit Test Strategy (unit-test-strategy.md: the statusline arms' relied set, the version gate's older-stamp oracle); §5 (integration-test-strategy.md: `tests/cli_verify.rs`, `contract_ledger_probes.rs`)
change:
  - `LedgerRow::relied()` is all twenty-four rows on every build (was 24 on Unix, 21 on Windows); `STEPS_PASS`, `ROW_IDS` and `LEDGER_ROWS` are `[&str; 24]`, the literals ungated; the Windows tallies and the «on Unix» qualifiers of `tests/cli_verify.rs` leave.
  - The older-stamp oracle reads the eighteen with `resume-session` and `agents-join`, and that set with `local-command-compact`, both unverified (was the second verifying on Windows only).
  - `run --local-live`: the H2 key-after-resize question belongs to the open residuals of origin `2026-11-04T23-11-43-wrap`, target `another-os` (was the route entry «Windows-only live measurements»).
why: the report's Symbols fact (`WRAPPED_HERE` folded: `LedgerRow::relied()` is all twenty-four rows on every build) and Counts fact (the Windows relied set of twenty-one gone); the «Windows-only live measurements» entry was retired at the 2026-11-04T23-11-43-wrap, its residuals targeted `another-os`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the e2e-home headroom read on an injected figure
section: §4 Unit Test Strategy (unit-test-strategy.md: the new viola-e2e harness group)
change:
  - New group: `Workspace::ensure_e2e_home` keeps its signature for every caller and passes the free-bytes reading to a private function taking it as an argument, so the `harness::tests::e2e_home_*` cases drive it on an injected figure, the below-floor refusal among them; `HEADROOM_FLOOR_MIB`, `headroom_refusal` and its message hold, with no env, flag, feature or config key.
why: the report's Symbols fact (the msrv fix behind `ensure_e2e_home`) and Coverage fact (the chunk's one new code shape, nine `e2e_home_*` cases run); the msrv red of ci#41930831242 is closed by ci#42030025050's `msrv` green.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the bootstrap phases and test data bootstrap without the Windows arms
section: §3 (bootstrap phases: `test-runner-install`, the dependency pins, `5-command-discipline-wire`, G2's jq; 5-command implementation: the `booted_wrapper` chain; Test data bootstrap: Mechanism)
change:
  - `[profile.ci]`'s kill line lists twenty-six verify-driven binaries (was twenty-seven, `conpty_sideload` gone); the paste-hint reading at ci#39383206773 reads as a run of the retired three-OS matrix.
  - The windows-sys `Win32_Security_Authorization` pin leaves; libc `=0.2.189` is a plain `viola-e2e` dependency (was under `cfg(unix)`).
  - `5-command-discipline-wire` wires `scripts/agent-run.sh` as the shim (the two-shell binding gone); G2's runner jq reading is jq 1.7 on ubuntu-24.04 (the windows-2025 and macos-15 readings gone).
  - The root waits read `Instant::now() + WITHIN` at 31 sites in 23 files, as measured at this chunk's wrap (was 32 in 24); `unconnectable` and `holder_gone` read a connect to the path refused or the path absent (the Windows arms and the pipe-after-exit reading gone).
  - Test data bootstrap: the Windows x64 `seed_conpty` mechanism and the Windows plain create leave; `headroom_refusal` is unit-tested with no every-OS qualifier.
why: the report's Counts fact (the kill line 27 → 26; `Instant::now() + WITHIN` 32 sites in 24 files → 31 in 23) and Dependencies fact (`windows-sys` removed, `libc` moved to plain `[dependencies]`); under the founder's 2026-11-04 Linux-only rulings.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
