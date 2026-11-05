@task_prose part=chunk
Makes viola Linux-only: removes every cfg(windows) and cfg(target_os = "macos") arm with its tests and fixtures, the four Windows-only files, vendor/conpty and the Windows target dependencies with their deny.toml entries; a cfg(unix) arm whose only sibling left loses its gate.
Removes the Windows and macOS jobs of ci.yml (test, lint, perf, release) and nightly.yml, windows-mutants.yml whole, and any pre-push or harness step that runs only for those OSes; the census of OS-gated tests is re-pointed or retired with the gates it counted.
First sets and pushes the tag multi-os-final on the last three-OS commit, before any removal commit; re-reads at take-up that no multi-os-* tag stands and that the four files hold 1 146 lines.
Fixes the two viola-e2e harness cases that read the host's real free space against the 9216 MiB e2e-home floor, so they decide on a figure the test controls and msrv is green again (ci#41930831242, abf00915).
Changes no Linux behaviour; acceptance is every gate green on Linux, no Windows or macOS job in any workflow, no Windows or macOS cfg left in the tree, and the tag resolving on origin; the test-plan's 3-OS CI matrix and per-OS legs are named as expected amendments the wrap writes.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=tests/conpty_sideload.rs
@task_file n=4 path=tests/contract_windows_mutation_scope.rs
@task_file n=5 path=vendor/conpty
@task_file n=6 path=Cargo.toml mark=edits
what: the [target.'cfg(windows)'] dependencies
@task_file n=7 path=deny.toml mark=edits
what: the entries of the removed Windows dependencies
@task_file n=8 path=.github/workflows/ci.yml mark=edits
what: the Windows and macOS jobs (test, lint, perf, release), the two pwsh shim steps
@task_file n=9 path=.github/workflows/nightly.yml mark=edits
what: the Windows and macOS jobs
@task_file n=10 path=.github/workflows/windows-mutants.yml
@task_file n=11 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests, re-pointed or retired with its gates
@task_file n=12 path=scripts/agent-run.ps1
@task_file n=13 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the cases e2e_home_backing_gone_target_is_made_again_owner_only and e2e_home_backing_owner_only_target_is_kept_untouched, the free-space floor panic message
@task_item n=1 part=needs
The CI matrix of section 9: windows-2025, macos-latest and ubuntu-latest with fail-fast false shrinks to ubuntu-latest, with the per-OS test, lint and perf jobs and their per-OS artifact names (harness-, diag-, junit-, perf-, diag-perf-, secret-scan- by matrix.os).
@task_item n=2 part=needs
The build failure conditions worded per OS: a test failure or coverage breach on any of the 3 OSes, and the ConPTY vendor errors on windows-2025 (scripts/conpty-vendor.sh --verify and its --probe), which leave with vendor/conpty and the Windows job.
@task_item n=3 part=needs
The audit's Windows mutation leg: the dispatch-only, report-only windows-mutants.yml named in sections 9 and 10, tests/contract_windows_mutation_scope.rs, and the Windows-host mutation scratch directory beside the repository named in the agent-runnable invariants.
@task_item n=4 part=needs
nightly.yml: its Windows and macOS jobs leave, while the time-boxed cargo fuzz runs and the weekly advisories over the root and fuzz lockfiles stay on ubuntu.
@task_item n=5 part=needs
The multi-os-compat coverage trigger and its OS-branch tests (named pipe vs Unix socket, ConPTY vs openpty, Windows DACL and strict-modes vs Unix modes, macOS euid and dir-only server verification), and the Test-Relevant Convention that every OS branch is tested on its own runner, retired with the gates.
@task_item n=6 part=needs
The harness shim pair of section 3: scripts/agent-run.sh and scripts/agent-run.ps1 as identical thin shims on all 3 OSes; only Windows CI steps run the .ps1 (the two pwsh shim steps and the windows-mutants step), and CLAUDE.md, the harness letters and health check 13 name the sh/ps1 pair.
@task_item n=7 part=needs
The ConPTY sideload and its test carve-outs: tests/conpty_sideload.rs (a companion another user may write leaves the child on the inbox ConPTY with sideload_fallback strict-modes-failed), the seed_conpty seeding of Windows x64 homes in the integration lifecycle and seed strategies, the DA1 answer under the sideloaded ConPTY, and the root tests' children hosted on the sideloaded ConPTY.
@task_item n=8 part=needs
Windows DACL and SDDL tests: the icacls set and read-back in tests/support/home.rs, the pipe DACL read in canonical form, the synthetic SDDL descriptor stubs for foreign-owner SIDs and non-persistent-ACL volumes, Windows strict-modes over real grants on windows-2025, the read-granted-home board scan case, and a Windows --home outside the profile given a protected DACL.
@task_item n=9 part=needs
Windows channel and process tests: the named-pipe listener rejecting remote clients and denying an anonymous open, client SQOS at the identification level, Windows handle non-inheritance, and the windows-sys TerminateProcess fallback of viola-pty, the cfg(windows) path of its unit cases.
@task_item n=10 part=needs
The .cmd / .bat PTY child refusal (run exit 1 on the windows runner, the chaos suite's .cmd case and the exit-cause matrix's .cmd human pair): whether its code and text name a removed arm is listed by P3 and decided by P4.
@task_item n=11 part=needs
macOS carve-outs: the units compiled out on macOS (the two real-cargo-mutants harness tests and the server check's captured-span cases), server_verdict_rows comparing no peer pid on macOS, tests/security_negatives_server_identity.rs on Linux and Windows and tests/security_negatives_socket_dir.rs on Linux and macOS, the macOS 26 arm64 runner, and the untestable zone of macOS local runs.
@task_item n=12 part=needs
cfg(unix) arms made unconditional where their only sibling left: the statusline pass-through on Unix only, the instance settings.json written on Unix, claude_env_keep on Unix, diagnostics 0600 on Unix, and the fake agent's per-command process group and SIGHUP-ignoring exit on Unix, each keeping its Linux behaviour.
@task_item n=13 part=needs
The test-scope entities and surfaces whose text names Windows or macOS code: viola-channel's Windows listener asserted on test (windows-2025), viola-state's and config state's Windows DACL limitation, the R8 strip's Windows registry Environment names and the identity floor measured on the Windows host, hook statusline's Windows arm that runs no command, and library tests running on all three CI OSes.
@task_item n=14 part=needs
OS text without a cfg gate that the removal may leave naming gone code: closed words conpty-sideload, conpty and sideload_fallback, SERVER_PID_REPORTED and the macOS carve-out, the schemas' OS words, help strings and doc comments; P3 lists them and P4 decides each against the masters' OS-target change.
@task_item n=15 part=needs
Coverage gating per OS in section 10: per-OS gating justified by cfg(windows) and cfg(unix) compiling only on their own runner, and the COVERAGE_IGNORE regex's separator class that exists for Windows backslash paths; the regex is never widened.
@task_item n=16 part=needs
The lint job's per-OS wording and the dev-host Windows- and macOS-target check-only clippy runs, which are no gates and lose their purpose.
@task_item n=17 part=needs
Supply chain over the removed Windows dependencies: cargo deny with the trimmed deny.toml, deny-probes ending 14/14 banned with control clean, the sole-root tokio ban per sync crate, channel-tokio-free, and the release-check probes, each still ending its required line after the manifests change.
@task_item n=18 part=needs
The e2e-home free-space floor in crates/viola-e2e/src/harness/mod.rs (cases at :481 and :501, the panic message built at :152): the cases must decide on a test-controlled free-space figure, never the runner's disk; the msrv job (ubuntu, Rust 1.96, unit suite) must be green; df -h /tmp is read before a stamped-home gate.
@task_item n=19 part=needs
Linux behaviour unchanged, binding every test the removal touches: the typed exit set 0, 1, 2, 10-14, 20, 21; hook exit 0 with no body and no stderr; every Linux wire contract and closed word; the human-wins invariant and the NEVER-log floor.
@task_item n=20 part=needs
No env, flag or feature seam may force a refusing peer-euid decision: the euid wiring of the three client seams stays held by no committed test, and the untestable zone of cross-OS-user peers stays as it is.
@task_item n=21 part=needs
Live sessions are 0: everything runs on the fake agent, and CI never touches the real claude CLI.
@task_item n=22 part=needs
The tag multi-os-final on the last three-OS commit, pushed to origin before any removal commit, and its resolving on origin as part of the acceptance.
@task_item n=23 part=needs
The masters' OS-target change in the test-plan (the 3-OS CI matrix, the per-OS legs, the tier justification's 3 required OSes and multi-platform CI), and the re-pointing of the owner Windows-only live measurements, such as the vt100 panic's Windows byte reading: amendments the wrap writes, which this chunk's plan names as expected.
@task_item n=24 part=needs
Pre-push and harness steps that run only for Windows or macOS leave with them; the local pre-push gate still checks the host's tools against ci.yml's pins.
@task_standing n=1
convention: each new or renamed test is named in snake_case as subject, condition, expected, and a critical-path scenario as path<N>_<slug>
home: section 2, Test Strategy (Test function naming)
@task_standing n=2
convention: table-driven cases in viola-e2e are a labelled case table in one #[test] with each assertion naming its label, adding no rstest manifest edge
home: section 4, Unit Test Strategy (Test grouping)
@task_standing n=3
convention: a test decides on values it injects, never on the host's clock, disk or state: no sleep for synchronisation, no test-owned timer, waits keyed on logged events or offsets
home: section 11, Test Anti-Patterns (E2E and Universal)
@task_standing n=4
convention: each test makes its own --home through tempfile, sets env per child with Command::env and never std::env::set_var, and re-adds LLVM_PROFILE_FILE after env_clear
home: section 11, Test Anti-Patterns (Integration)
@task_standing n=5
convention: every workflow has permissions {} at the top and contents read per job, every uses SHA-pinned, no concurrency block, and rust-cache as the only cache, in ci.yml only
home: section 9, CI Integration
@task_standing n=6
convention: each tool has one version source, ci.yml's pins, and the pre-push gate checks the host's tools against them and installs nothing
home: section 9, CI Integration
@task_standing n=7
convention: each CI test job ends in viola-harness gate --require its suites, and every artifact leaves CI only behind the secret scan, with retention-days 7
home: file ci-integration
@task_standing n=8
convention: no flaky test is tolerated: nextest retries 0, no retry budget, and no #[ignore] as a parking place
home: section 10, Quality Gates & Coverage Targets (Zero-flakiness budget)
@task_standing n=9
convention: a harness command prints exactly one JSON document on stdout with exit 0, 1 or 2, never a human-only message
home: section 3, Test Harness Contract
@task_item n=1 part=not_needed
Path 3 wait and last readback: its wake kinds, cursor and timeout signals do not change on Linux.
@task_item n=2 part=not_needed
Path 4 dialog answers: the decision bodies per dialog kind and their insta snapshots per CLI version.
@task_item n=3 part=not_needed
Path 5 wheel and release: the wheel state machine and its refusals are untouched.
@task_item n=4 part=not_needed
Path 6 budget thresholds: the 90 and 85 percent crossings and release --budget.
@task_item n=5 part=not_needed
Path 7 version gate and the viola verify stamped line.
@task_item n=6 part=not_needed
Scenario E3 link and unlink, and the link line's since stamp.
@task_item n=7 part=not_needed
Scenario E4: no frontend in 0.1.0, deferred to 0.2.0.
@task_item n=8 part=not_needed
The list row integrity suite: escaped names and column offsets on the board.
@task_item n=9 part=not_needed
MCP handler re-validation, tools/list holding exactly five tools, and the stdin line cap.
@task_item n=10 part=not_needed
The property suite's parser targets and proptest settings.
@task_item n=11 part=not_needed
The fake agent's turn-script schema, receipt kinds, control file and mode flags.
@task_item n=12 part=not_needed
The status endpoint shape of viola list --json.
@task_item n=13 part=not_needed
The mutation verdict's arithmetic in outcomes.json (missed, timeout, unviable against caught): no mutation run belongs to this chunk.
@task_item n=14 part=not_needed
Time mocking through the Clock seam and the budget gate's override expiry.
@task_item n=15 part=not_needed
The Test Decisions Log and its amendments archive.
