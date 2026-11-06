@task_prose part=session
The session made Viola Linux-only. The Windows and macOS code, tests, fixtures and CI legs were removed after the pushed tag multi-os-final (ecbe7ab3), and every gate is green on ubuntu-latest.
Retired: the Windows named pipe and its SDDL / SQOS / remote-reject controls, the macOS TMPDIR socket dir and its no-peer-pid carve-out, the Windows owner/DACL strict-modes arm, the ConPTY sideload and vendored conpty tree, the .cmd/.bat refusal (batch-script-child), the registry env reader, the CASE_INSENSITIVE R8 fold, the Windows VT enable, the console-control stop arm, windows-mutants.yml, the windows-2025 and macos-latest CI legs, scripts/junit-os-sets.py, the junit-os artifacts, scripts/agent-run.ps1, and the windows-sys dependency.
The closed words batch-script-child, sideload_fallback and os windows / macos left schemas/diag-line.v1.json. os is now linux only, the process-exit detail enum has 19 members, and pty_backend is always openpty.
Counts moved: revive refusal codes 11 to 10, verify refusal codes 6 to 5, human::refuse sites in run 8 to 7, nextest ci kill-line binaries 27 to 26, Instant::now() + WITHIN waits from 32 sites in 24 files to 31 in 23, zizmor concurrency-limits suppressions 3 to 2, workflows 3 to 2, the Windows relied set from twenty-one to none (LedgerRow::relied() is all twenty-four), deny targets 3 triples to 1, and licence exceptions 2 to 1 (recvmsg dropped).
The OS-gate census contract_os_gated_tests gained a fault class: a gate naming another OS is a fault. CI run 42030025050 is green on test, lint, msrv, perf, release and fuzz-replay. supply-chain is red only on RUSTSEC-2026-0441, which a CARRY on Version done-check owns.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=crates/viola-channel/src/server/win.rs
@task_file n=4 path=crates/viola-e2e/src/harness/run/mutants/scratch.rs
@task_file n=5 path=tests/conpty_sideload.rs
@task_file n=6 path=tests/contract_windows_mutation_scope.rs
@task_file n=7 path=tests/channel_sqos_open.rs
@task_file n=8 path=tests/cli_program_resolution.rs
@task_file n=9 path=tests/security_negatives_channel.rs
@task_file n=10 path=.github/workflows/windows-mutants.yml
@task_file n=11 path=scripts/conpty-vendor.sh
@task_file n=12 path=scripts/junit-os-sets.py
@task_file n=13 path=scripts/agent-run.ps1
@task_file n=14 path=vendor/conpty/1.24.260710001/x64/conpty.dll
@task_file n=15 path=vendor/conpty/1.24.260710001/x64/OpenConsole.exe
@task_file n=16 path=.github/workflows/ci.yml
@task_file n=17 path=.config/nextest.toml
@task_file n=18 path=deny.toml
@task_file n=19 path=deny-sync.toml
@task_file n=20 path=schemas/diag-line.v1.json
@task_file n=21 path=.gitattributes
@task_file n=22 path=scripts/agent-run.sh
@task_file n=23 path=scripts/orphans-check.sh
@task_file n=24 path=scripts/install-ripgrep.sh
@task_file n=25 path=scripts/lint-probes.sh
@task_file n=26 path=tests/contract_os_gated_tests.rs
@task_file n=27 path=tests/support/home.rs
@task_file n=28 path=tests/support/frozen.rs
@task_file n=29 path=tests/run_cli.rs
@task_file n=30 path=tests/hook_fail_open.rs
@task_file n=31 path=tests/security_negatives_server_identity.rs
@task_file n=32 path=tests/cli_verify.rs
@task_file n=33 path=crates/viola-e2e/src/harness/mod.rs
@task_file n=34 path=crates/viola-e2e/src/harness/run/mutants.rs
@task_file n=35 path=crates/viola-e2e/src/harness/run/mutants/host.rs
@task_file n=36 path=crates/viola-state/src/strict.rs
@task_file n=37 path=crates/viola-channel/src/endpoint.rs
@task_file n=38 path=crates/viola-pty/src/lib.rs
@task_file n=39 path=crates/viola-agent-claude/src/ledger.rs
@task_file n=40 path=src/human/style.rs
@task_item n=1 part=touches
The OS target. Every claim that says three OSes, three required OSes, all three CI OSes, cross-platform, Windows first, or on every OS is stale. The product is Linux only: CI runs on ubuntu-latest, and the Windows and macOS code can be recovered at the tag multi-os-final. This includes the §1 tier justification (3 required OSes, the multi-platform CI driver), the §2 unit and library-only rows, and the §9 and §10 failure conditions on any of the 3 OSes.
@task_item n=2 part=touches
§1 entity viola-pty: the windows-sys TerminateProcess kill fallback, the ConPTY sideload, PtyError::Load and the ReadConsoleW host stdin reader are gone. pty_backend is always openpty. Child exit is still detected on the handle wait. The §4 viola-pty row that calls the TerminateProcess path cfg(windows) is stale.
@task_item n=3 part=touches
§1 entity viola-channel, §4, §5 and §6: the Windows listener rejecting remote pipe clients, the pipe DACL denying an anonymous open, the SQOS open, the named-pipe arms and Windows handle non-inheritance are retired, along with their tests (tests/channel_sqos_open.rs and tests/security_negatives_channel.rs are deleted). The socket dir forms are now only XDG_RUNTIME_DIR/viola/ and /tmp/viola-<uid>/, because the macOS TMPDIR/viola/ arm is gone.
@task_item n=4 part=touches
Server verification and peer euid: SERVER_PID_REPORTED and PEER_EUID_REPORTED are removed. peer_euid_admitted(observed, own) is observed == Some(own), and an unreadable euid is still refused. liveness::server_verdict / verify_server lose the peer_pid_reported parameter and the macOS Matches arm, and server_verdict_rows loses its no-pid row. Clauses that are stale: the macOS carve-out that compares no peer pid, the server-identity negatives on Linux and Windows, the connected stream's server pid on each OS, the real euid read on Linux and macOS, and the cases running on macOS too.
@task_item n=5 part=touches
§1 entities viola-state and config / code-bearing state, §4 viola-state, §6, §8: the Windows owner/DACL strict-modes arm is removed, together with windows_read_verdict, Refused::NullDacl, Refused::NoPersistentAcls, outside_profile, the off-Unix arm of check_socket_dir, the Windows board_scan read-grant case, and the synthetic SDDL descriptor stubs for foreign SIDs and non-persistent-ACL volumes. So is the partially-testable reason based on the Windows DACL limitation. The §4 test name check_socket_dir_off_unix_reads_nothing names a removed arm.
@task_item n=6 part=touches
viola-state fs and pin: replace_private is one attempt. retry_replace, replace_private_retrying, replace_private_with, write_snapshot_with, pin_companions, retry_open, open_held_waiting and PinError::StrictModes are gone. pin_bytes, the re-hash of the pinned copy, and replace_private_shared stay.
@task_item n=7 part=touches
§1 entity viola-agent-claude and §4 npm-shim resolution: Search::windows, pathext, is_script, SHIM_TARGET and AgentError::BatchScriptChild are removed. resolve_program takes a given path as given and a bare name on PATH. The shim-resolution ledger row now reads only run.version_answered (claude --version answering). The §1 child-spawn trigger clause that says the npm-shim fixture resolves to claude.exe is a Windows claim.
@task_item n=8 part=touches
R8 strip / inherited credentials entity: the Windows registry Environment reader and the CASE_INSENSITIVE fold are gone, so the strip matches names exactly. claude_env_keep and the eleven-name identity floor stay. The clause that cites the floor as measured on the Windows host needs re-wording.
@task_item n=9 part=touches
Ledger scrub: the case-fold parameter is gone and the scrub matches the home exactly. The test scrub_folds_the_home_case_only_when_asked was renamed scrub_matches_the_home_case_exactly. The ledger relied set is now all twenty-four rows on every build, so the Windows relied set of twenty-one is retired.
@task_item n=10 part=touches
Hook statusline: statusline::shell_argv returns [&str; 3]. The pass-through runs on every build, so clauses that limit it to Unix only, or say the Windows arm runs no command, are stale (§1 hook entity, Path 6 critical path and scenario).
@task_item n=11 part=touches
Path 1 run start sequence: the fail-open ConPTY sideload step (run.conpty_sideload, sideload_fallback) and the pin_companions call are removed from the ordered start steps. The plugin/settings.json clause that says none is written on Windows is stale.
@task_item n=12 part=touches
The .cmd/.bat refusal is retired: batch-script-child, AgentError::BatchScriptChild, and its human pair (the claude CLI is a .cmd or .bat script / pass the real executable). This affects the §1 child-spawn trigger (.cmd / .bat PTY child exits 1 on the windows runner), the §5 Module to PTY row (.cmd child refused), the §6 chaos suite (.cmd child refused exit 1), and the exit-cause matrix comment that names the .cmd human pair.
@task_item n=13 part=touches
ConPTY sideload: retired with its tests and fixtures. tests/conpty_sideload.rs, seed_conpty in tests/support/home.rs, the companion strict-modes case with sideload_fallback strict-modes-failed (§1 filesystem trigger, §6 security negatives), the §5 setup carve-out for Windows x64 seeded homes, the DA1 answer under the sideloaded ConPTY, and the §7 seed strategy exception for seed_conpty are all gone.
@task_item n=14 part=touches
§5 driver Windows DACL read-back: windows-sys, icacls grants and the canonical SDDL read-back are removed. tests/support/home.rs widen / grant_read / unwiden / still_widened lose their icacls arms, and SOCKET_ROOT_VAR reads XDG_RUNTIME_DIR.
@task_item n=15 part=touches
The multi-os-compat coverage trigger and the §9 matrix: the matrix is ubuntu-latest only. The OS-branch tests named there (named pipe vs Unix socket, ConPTY vs openpty, Windows DACL vs Unix modes, macOS euid plus dir-only verification) are retired. The jobs test, perf, lint and release keep strategy.matrix.os with one leg, and every job and artifact name keeps its matrix.os form.
@task_item n=16 part=touches
§9 CI platform: workflows go from 3 to 2 (ci.yml and nightly.yml). windows-mutants.yml is deleted, and with it the Windows mutation leg of the boundary audit. Clauses that are stale: the dispatch-only workflow in the Platform line, the Mutation row, and the mutants.out reasoning.
@task_item n=17 part=touches
§9 CI steps: the ConPTY vendor verification step, both pwsh shim steps, the coverage job on windows-2025 after vendor verification, the dev-host Windows and macOS check-only clippy runs, and the ConPTY vendor errors build-failure condition (scripts/conpty-vendor.sh) are all gone. The sh shim steps and Lint probes lose their runner.os conditions.
@task_item n=18 part=touches
§9 artifacts: scripts/junit-os-sets.py and the three junit-os artifacts are retired. Any JUnit reader that compares OS sets is stale.
@task_item n=19 part=touches
§3 harness implementation: scripts/agent-run.ps1 is deleted. The harness is scripts/agent-run.sh alone, so the identical sh and ps1 shim pair with byte-identical semantics on 3 OSes is stale.
@task_item n=20 part=touches
Mutation harness: scratch.rs is deleted, along with HOST_SCRATCH, the viola-mutants-scratch dir, the Windows scratch wipe, the scratch_bytes field, its TMP/TEMP/--output arm, and the scratch_bytes key of the harness document. The §2 invariant that places outcomes.json under the host mutation scratch on a Windows host is stale. mutants.rs planted any(unix, windows) was re-planted as any(unix, target_os = "none"), and the host.rs fixtures were re-planted on not(unix) and target_os = "freebsd".
@task_item n=21 part=touches
Kill-line count: the nextest ci binary(...) kill line drops from 27 names to 26 (binary(conpty_sideload) is gone, .config/nextest.toml line 24). The ci-integration site over 27 binaries is stale, and so is the contracts key bootstrap-phases-derive-for-route-setup-project that lists conpty_sideload among the binaries.
@task_item n=22 part=touches
Instant::now() + WITHIN waits: from 32 sites in 24 files to 31 in 23. The contracts key 5-command-implementation carries the count.
@task_item n=23 part=touches
Refusal code counts: revive's closed --json refusal codes go from 11 to 10, and verify's from 6 to 5. human::refuse sites in run go from 8 to 7 and in verify from 6 to 5. Pinned in tests/cross_exit_causes.rs and the CLI tests. src/cmd/verify.rs loses the BatchScriptChild arm and the Windows relied count.
@task_item n=24 part=touches
Schema conformance: diag-line.v1.json os is ["linux"], the sideload_fallback property leaves process-start, and the process-exit detail enum has 19 members. Any test-plan reading of three os words or of sideload_fallback is stale.
@task_item n=25 part=touches
Supply chain trigger and gates: deny.toml and deny-sync.toml targets are x86_64-unknown-linux-gnu only. deny.toml licence exceptions are doctest-file alone, because recvmsg was dropped. The ci-integration lint lines naming --target x86_64-pc-windows-msvc or aarch64-apple-darwin are stale. scripts/orphans-check.sh narrows to one triple, and its outputs 2/2 fired and 11/11 targets clean stand. zizmor concurrency-limits suppressions go from 3 to 2.
@task_item n=26 part=touches
Tests ungated: hook_fail_open's multi-line cfg_attr(not(target_os = "macos")) pair, run_refuses_a_stale_name, cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal, a_start_under_umask_000_makes_every_home_path_owner_only, the handlers impostor cases and security_negatives_server_identity.rs now run unconditionally. tests/support/frozen.rs keeps kill -STOP ungated, and tests/run_cli.rs keeps kill -KILL only.
@task_item n=27 part=touches
Untestable zones and skipped lists: the Windows non-persistent-ACL volumes and foreign-owner SIDs zone and the macOS local runs zone are retired (§1, §6 skipped list). The cross-user zone wording about the real euid read on Linux and macOS narrows to Linux.
@task_item n=28 part=touches
Route entry Windows-only live measurements: the entry is retired, and its residuals are re-pointed to origin 2026-11-04T23-11-43-wrap (target another-os). The child-spawn trigger clause that owes the Windows byte reading of the forced vt100 panic to that entry, and other wolm sites, are stale.
@task_item n=29 part=touches
Human style: the Windows VT enable (decision rung 7), the vt fact, the vt_enable_failed row and the Windows depth rows are removed. Rungs 1 to 6 and the depth rule are unchanged. The test colorterm_on_windows was renamed colorterm_without_term. src/cmd/verify/stop.rs loses the console-control arm, and SIGINT / SIGTERM are unconditional.
@task_item n=30 part=touches
§10 and §11 OS wording: per-OS gating justified by cfg(windows) code compiling only on its own runner, the Windows-path rationale of the separator-agnostic ignore regex, the Windows Defender warm-up example, the perf host reading on a Windows dev host, the anti-pattern that bans treating a Linux-only local run as proof for Windows or macOS, and the ConPTY note on child.wait() are now history or stale. The owner re-words them. A dated measurement is not left standing as current.
@task_fact n=1 slug=e2e-test-strategy
fact: The OS-gate census tests/contract_os_gated_tests.rs reads any gate whose predicate names windows or target_os = "macos" as a fault (another OS: ...), even when a GATES row names it. Its planted self-check census_reads_a_gate_naming_another_os_as_a_fault_even_when_named holds this, and its parse catches a multi-line cfg_attr OS predicate that a line grep cannot see.
@task_fact n=2 slug=test-scope-summary
fact: The Windows and macOS code, tests, fixtures and CI jobs can be recovered at the pushed tag multi-os-final (ecbe7ab3). Founder rulings 2026-11-04 decided that 0.1.0 is Linux only, with the 0.2.0 window features on Hyprland.
@task_fact n=3 slug=unit-test-strategy
fact: Workspace::ensure_e2e_home keeps its signature and passes the free-bytes reading to a private function as an argument (the msrv fix). The harness::tests::e2e_home_* unit cases (9) drive it on an injected figure, including the below-floor refusal. HEADROOM_FLOOR_MIB and the e2e-home backing message are unchanged, with no env, flag, feature or config key.
@task_fact n=4 slug=ci-integration
fact: The supply-chain job reads red on one advisory, RUSTSEC-2026-0441 (pin-utils unmaintained, through portable-pty =0.8.1 and nix 0.25.1). A CARRY on Version done-check owns it, and its remedy is moving portable-pty off 0.8.1 to a release whose graph drops pin-utils. The ignore list is not widened.
@task_item n=1 part=untouched
The behaviour of critical paths 2 to 5 and 7: send confirmation and CL-1 events, wait / last wake rules, dialog answers by dialog_id, the wheel and release rules, and the unverified CLI gate. Only their OS qualifiers move.
@task_item n=2 part=untouched
The MCP surface rules: tools/list is exactly five tools, isError code-only payloads, server-side re-validation, and the stdin MAX_FRAME line cap and its parse-rejected line.
@task_item n=3 part=untouched
Paste validation (validate_paste_text, control-character before other refusals) and the property suite's parser list with proptest cases 512 and committed proptest-regressions.
@task_item n=4 part=untouched
§10 coverage thresholds (lines 85, functions 95, regions 80), the COVERAGE_IGNORE trees, and the mutation verdict (missed 0 after host-excluded, timeout 0, unviable at most caught). Only their OS rationale wording moves.
@task_item n=5 part=untouched
Perf budgets: the max-sample gate, viola_core::SPINE_DEADLINE 1.0 s, CONNECT_DEADLINE 750 ms, the hyperfine invocation, and the five timed rows.
@task_item n=6 part=untouched
The budget governor thresholds (five_hour 90, seven_day 85), release --budget, and the budget.json seeding rule only through viola hook statusline.
@task_item n=7 part=untouched
The harness exit codes, the session record shape, the usage-error detail words, and the eight suites of gate::SUITES.
@task_item n=8 part=untouched
Fixture hygiene: the scrub codes, the workspace.repo exception, recorded fixtures per cli-version, and the contract suite's byte-for-byte comparison of fake-agent hook payloads.
@task_item n=9 part=untouched
Time and clock discipline: the injected viola_core::Clock, mock_instant limits, and the no-sleep rule.
@task_item n=10 part=untouched
Scenarios E1, E3 and E5 (unwrapped hooks no-op, link / unlink, snapshot replay recovery) and their landed tests.
@task_item n=11 part=untouched
The test tier Comprehensive and its non-OS drivers (chaos, performance budgets, multi-version fixtures, the founder-mandated mutation testing at the epoch boundary).
@task_item n=12 part=untouched
The Rust toolchain pin 1.98.1 and the MSRV 1.96 floor: dev-tool versions were re-read unchanged.
@task_item n=13 part=untouched
§12 Test Decisions Log pointer and the amendments history files.
