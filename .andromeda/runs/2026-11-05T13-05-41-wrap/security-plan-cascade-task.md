@task_prose part=session
The session made viola Linux-only: the Windows and macOS code, tests, fixtures and CI jobs were removed after the pushed tag multi-os-final (ecbe7ab3), where they stay recoverable.
Retired: the Windows named-pipe endpoint with its SDDL, SQOS and accept_remote(false) controls, the macOS TMPDIR socket dir and its no-peer-pid carve-out, the Windows owner/DACL strict-modes arm, the ConPTY sideload and its vendored binaries, the .cmd/.bat refusal, the Windows registry environment reader, the console-control stop arm and the CASE_INSENSITIVE R8 fold.
Dependencies: windows-sys removed everywhere; libc moved to plain dependencies; no crate added or bumped; portable-pty stays =0.8.1.
CI: windows-mutants.yml deleted, so the workflows are ci.yml and nightly.yml; the matrix is [ubuntu-latest]; the ConPTY vendor verification and pwsh shim steps are gone; deny targets are x86_64-unknown-linux-gnu alone; the recvmsg licence exception was dropped at this wrap.
Measured: CI run 42030025050 green on every job except supply-chain, which is red only on RUSTSEC-2026-0441 (pin-utils via portable-pty 0.8.1 and nix 0.25.1), owned by a CARRY on Version done-check.
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
@task_file n=16 path=deny.toml
@task_file n=17 path=deny-sync.toml
@task_file n=18 path=Cargo.toml
@task_file n=19 path=Cargo.lock
@task_file n=20 path=.github/workflows/ci.yml
@task_file n=21 path=schemas/diag-line.v1.json
@task_file n=22 path=.gitattributes
@task_file n=23 path=crates/viola-channel/src/client.rs
@task_file n=24 path=crates/viola-channel/src/endpoint.rs
@task_file n=25 path=crates/viola-channel/src/server.rs
@task_file n=26 path=crates/viola-state/src/strict.rs
@task_file n=27 path=crates/viola-state/src/fs.rs
@task_file n=28 path=crates/viola-state/src/pin.rs
@task_file n=29 path=crates/viola-state/src/liveness.rs
@task_file n=30 path=crates/viola-agent-claude/src/lib.rs
@task_file n=31 path=crates/viola-agent-claude/src/ledger.rs
@task_file n=32 path=crates/viola-agent-claude/src/statusline.rs
@task_file n=33 path=crates/viola-pty/src/lib.rs
@task_file n=34 path=src/main.rs
@task_file n=35 path=src/cmd/run.rs
@task_file n=36 path=src/run/env.rs
@task_file n=37 path=src/run/mod.rs
@task_file n=38 path=src/cmd/verify/stop.rs
@task_file n=39 path=crates/viola-e2e/src/harness/run/mutants.rs
@task_file n=40 path=tests/contract_os_gated_tests.rs
@task_item n=1 part=touches
The OS target: every cross-platform, Windows first, three OSes or Windows, macOS and Linux statement is now stale. The target is Linux only: the core on any current distribution (CI on ubuntu-latest), the 0.2.0 window features on Hyprland, the Windows and macOS code recoverable at tag multi-os-final. An on-every-OS clause is reworded to the Linux statement and keeps the claim it carries.
@task_item n=2 part=touches
The tier justification and the IPC attack-surface vector: the predictable names in the shared Windows pipe namespace are gone. The endpoint is a Unix domain socket in a per-user 0700 directory only.
@task_item n=3 part=touches
The Networking and TLS rows, and every per-instance named pipes (Windows) or named pipes or Unix sockets wording, are stale. Only Unix sockets remain. The socket directory forms are $XDG_RUNTIME_DIR/viola/ and /tmp/viola-<uid>/, and the macOS $TMPDIR/viola/ form is retired.
@task_item n=4 part=touches
The CI/CD line and every windows-2025 or macos-latest leg: the ci.yml matrix is [ubuntu-latest] for test, perf, lint and release. Job and artifact names keep their matrix.os form. scripts/junit-os-sets.py and the three junit-<os> artifacts are gone.
@task_item n=5 part=touches
The Auth & Authorization library row is stale. interprocess pid() no longer has a Windows GetNamedPipeServerProcessId arm or a macOS none case, and euid() no longer has a Windows none case. windows-sys 0.61.2 (GetTokenInformation, ConvertSidToStringSidW, SDDL conversion, CreateFileW with SQOS) is removed from every manifest.
@task_item n=6 part=touches
The IPC access control (Windows) row is retired: the protected user and SYSTEM DACL, SDDL, first-pipe-instance, accept_remote(false) and the test-only SQOS pipe (tests/channel_sqos_open.rs deleted).
@task_item n=7 part=touches
The client-side server verification rule loses its Windows SQOS arm and the macOS carve-out (no peer pid, server-verify-failed meaning an euid mismatch alone). The Linux path stands: peer pid and start time, the socket-dir check (socket-dir-refused) and the euid check (server-verify-failed). SERVER_PID_REPORTED and PEER_EUID_REPORTED are removed, peer_euid_admitted is observed == Some(own) with an unreadable euid still refused, and liveness server_verdict / verify_server lose peer_pid_reported.
@task_item n=8 part=touches
The ~/.viola/ access control row and the filesystem boundary vector: the Windows inherited profile DACL, the protected user and SYSTEM DACL outside the profile, the allow-ACE read-right test, and owner and DACL as a strict-modes form are all retired. The Unix owner-and-mode check is unconditional, and the off-Unix Ok(()) arm of check_socket_dir is gone. Refused::NullDacl, Refused::NoPersistentAcls, outside_profile, windows_read_verdict and READ_RIGHTS are removed.
@task_item n=9 part=touches
replace_private is now one attempt: retry_replace, REPLACE_ATTEMPTS, replace_private_retrying, replace_private_with and write_snapshot_with are gone. replace_private_shared and the mode-before-any-byte rule stay. Any text naming a retrying replace is stale.
@task_item n=10 part=touches
The pinned binary: pin_companions, the held-open companions, retry_open, open_held_waiting and PinError::StrictModes are removed. pin_bytes and the pinned copy's re-hash stay. Every viola(.exe) path form, and every mention of the ConPTY companions being re-hashed, is stale.
@task_item n=11 part=touches
The DLL search order and sideloaded ConPTY companions row is retired, as are the Universal ban on reaching a DLL load before restrict_dll_search and the Data Protection ban on loading a conpty companion without its re-hash. restrict_dll_search, the sideload pre-load, run.conpty_sideload, sideload_fallback, bin/<version>-<hash>/conpty/, target/conpty-seed/ and the inbox-ConPTY degrade are all removed. pty_backend() always reads openpty.
@task_item n=12 part=touches
The Persistent-environment names (Windows registry) input row is retired: the HKCU\Environment reader is removed. Any R8 wording that names the persistent set or registry is stale, and the CASE_INSENSITIVE fold is gone, so the R8 strip matches names exactly. claude_env_keep and the eleven-name identity floor stay.
@task_item n=13 part=touches
The child executable resolution row and the NEVER spawn a .cmd/.bat PTY child ban: the Windows .cmd/.bat refusal, the claude shim resolved to claude.exe, AgentError::BatchScriptChild, is_script, pathext and SHIM_TARGET are retired. resolve_program takes a given path as given and a bare name on PATH. ProgramNotFound and the absolute path handed to portable-pty stay.
@task_item n=14 part=touches
The Statusline shell-out row (Unix only, never on Windows) is now plain Linux wording. statusline::shell_argv returns [&str; 3]. It is still /bin/sh -c with the command as one argument and still the only shell-out.
@task_item n=15 part=touches
The Handle inheritance row cites the Windows CreateProcessW bInheritHandles behaviour of portable-pty and interprocess listener inheritable(false). The Windows pipe listener it covered is gone, so the row's Windows citation is stale, as is the matching NEVER CreateProcessW ban wording.
@task_item n=16 part=touches
The Files at rest row: the Windows DACL clause and the ConPTY companions re-hash clause are stale. Unix 0700/0600 and the pinned viola 0700 stay.
@task_item n=17 part=touches
Dependency Security: the vendored ConPTY audit row (conpty-vendor.sh --verify / --probe, Authenticode signer), the four ConPTY pins in src/conpty.rs, the committed-binary rule (.gitattributes vendor/conpty/** binary removed) and the whole dep-conpty group are retired.
@task_item n=18 part=touches
The [licenses] exceptions row (fct-32d4ty7r): the 0BSD exceptions are now doctest-file alone, so it reads two 0BSD crates and recvmsg where it should not. recvmsg is interprocess's Windows-only dependency and is out of the Linux graph (CI job 127253812277 warned license-exception-not-encountered at deny.toml:26). The allow list and the per-crate admission rule stay.
@task_item n=19 part=touches
The deny.toml and deny-sync.toml targets are now x86_64-unknown-linux-gnu alone (they were three triples). Any text naming the Windows or macOS triples or per-target lint lines is stale.
@task_item n=20 part=touches
The CI integration header and the windows-mutants.yml row (workflow_dispatch, epoch-boundary audit, permissions) are retired with the workflow, as is the npplilir mention of the dispatch-only windows-mutants.yml --package arm. The parenthetical that windows-mutants.yml uploads nothing in the concurrency row is stale.
@task_item n=21 part=touches
The zizmor concurrency-limits count moves from 3 to 2 (one per workflow over ci.yml and nightly.yml). The standing three low findings, one per workflow, is stale.
@task_item n=22 part=touches
The ConPTY vendor verification step on the windows-2025 test job is retired, as are both pwsh shim steps. The sh shim steps and Lint probes lose their runner.os conditions. The per-OS wording of the test, release and perf jobs now covers one leg.
@task_item n=23 part=touches
Bootstrap phases: the dep-audit-tooling-install ConPTY script clause and the dep-security-ci-gate windows-2025 ConPTY step are retired. auth-scaffolding-baseline per-OS listener hardening is now Linux socket hardening only.
@task_item n=24 part=touches
The Authentication anti-patterns are all stale: the Windows listener security_descriptor ban, the Windows SQOS client-pipe ban, the macOS reason in the ListenerOptionsExt::mode ban, and the macOS carve-out exception in the NEVER write any channel frame before verifying ban.
@task_item n=25 part=touches
The Data Protection anti-pattern on Windows --home outside %USERPROFILE%, the Windows owner and DACL check, NULL DACL, volumes without persistent ACLs and inherit-only GENERIC grants is retired with the Windows arm.
@task_item n=26 part=touches
The Code Patterns ban on run --mutants wiping only the Windows host mutation scratch viola-mutants-scratch is retired. scratch.rs, HOST_SCRATCH, the Windows scratch wipe, the TMP/TEMP/--output arm and the harness document's scratch_bytes key are all gone.
@task_item n=27 part=touches
The relied ledger rows that the stamp rule reads: the Windows relied set of twenty-one is retired, and LedgerRow::relied() is all twenty-four rows on every build. The shim-resolution row reads claude --version answering alone. The verify --record ledger scrub matches the home exactly, with no case fold.
@task_item n=28 part=touches
Error Handling Display list: the enums stay, but their Windows variants are removed (PtyError::Load, AgentError::BatchScriptChild, PinError::StrictModes, Refused::NullDacl, Refused::NoPersistentAcls). The closed words batch-script-child, sideload_fallback and the os words windows and macos leave the diag-line schema. The os enum is now linux alone, and the process-exit detail enum has 19 members. revive has 10 closed refusal codes and verify has 5.
@task_item n=29 part=touches
Security test inventory: tests/security_negatives_channel.rs, tests/channel_sqos_open.rs, tests/cli_program_resolution.rs and tests/conpty_sideload.rs are deleted. tests/security_negatives_server_identity.rs and the impostor cases run ungated. The census tests/contract_os_gated_tests.rs reads any gate naming windows or target_os macos as a fault.
@task_item n=30 part=touches
The working-route entry Windows-only live measurements was retired at the 2026-11-04T23-11-43-wrap. Security's four sites that name it are re-pointed to the residuals of that origin (target another-os) or retired with their Windows claim. The verify stop's console-control arm (CTRL_C_EVENT, CTRL_BREAK_EVENT) is gone, and SIGINT/SIGTERM are unconditional.
@task_fact n=1 slug=dependency-security
fact: RUSTSEC-2026-0441 (pin-utils 0.1.0 unmaintained, via portable-pty =0.8.1 then nix 0.25.1 into viola-pty) reads red in supply-chain's advisories family. It is owned by a CARRY on Version done-check, whose remedy moves portable-pty off 0.8.1 to a release whose graph drops pin-utils. No ignore-list widening: the one advisory ignore stays one.
@task_fact n=2 slug=dependency-security
fact: The portable-pty 0.9.0 Windows garbage-read bug that justified the =0.8.1 pin left with the Windows code, so the pin now stands on no Windows reason, and Version done-check measures moving off it.
@task_fact n=3 slug=dependency-security
fact: A licence exception that names a crate no longer in the graph is dropped (cargo-deny warns license-exception-not-encountered). Such an exception narrows nothing and widens nothing (overseer technical answer, inputs I5 item 7).
@task_fact n=4 slug=threat-model-summary
fact: The Windows and macOS code removed by the Linux-only ruling (founder, 2026-11-04) is recoverable at the git tag multi-os-final (ecbe7ab3), pushed to origin. It holds src/conpty.rs and windows-mutants.yml.
@task_item n=1 part=untouched
The paste-text and dialog free-text control-character set (validate_paste_text, control-character, empty-text) and its bans. viola-core was not edited.
@task_item n=2 part=untouched
The MCP stdin MAX_FRAME line cap and the MCP tool input checks (ViolaName, closed response enums, invalid-params). viola-mcp lost only a SERVER_PID_REPORTED argument.
@task_item n=3 part=untouched
Hook fail-open: exit 0, no body, never stderr, and the hidden capture arms. The hook contract and its bans are unchanged.
@task_item n=4 part=untouched
The secret-scan classes. The cookie: header match is case-insensitive, a true claim that only shares the casefold token with the retired R8 fold. The content canary and the stripped CLAUDE* needles stay.
@task_item n=5 part=untouched
Channel -32603 fixed internal error, the MCP isError structuredContent codes, and the instance-unreachable detail words (strict-modes-failed, server-verify-failed, socket-dir-refused and the rest). All stay. The two security rows naming verify's refusal writer carry no count and stay true.
@task_item n=6 part=untouched
The security tier (Minimal with local-boundary elevations), the data classification, the compliance triggers and the API Security rmcp transport row are unchanged.
@task_item n=7 part=untouched
The test seams FAKE_AGENT_PUMP_DELAY_MS and FAKE_AGENT_HOOK_PANIC, and the G2 single exemption. Only the fake agent's OS arms left.
@task_item n=8 part=untouched
The terminal colour facts NO_COLOR, TERM and COLORTERM. Rungs 1 to 6 and the depth rule are unchanged; only the Windows VT enable rung and depth branch left.
@task_item n=9 part=untouched
The driver prompt-injection accepted risk and the unverified-cli gate rule (no non-null decision without a viola verify stamp) are unchanged apart from the relied-set count.
@task_item n=10 part=untouched
Secret Management storage, the v1.x release prerequisites (dist, cargo-auditable, self_update signatures) and the cargo-deny and zizmor floors. No dev-tool version moved.
@task_item n=11 part=untouched
The target/e2e-home keepers ban. Workspace::ensure_e2e_home only moved its free-bytes reading into a private function, with no env, flag, feature or config key. HEADROOM_FLOOR_MIB and its refusal are unchanged.
@task_item n=12 part=untouched
The Git Bash rewritten-path warning in send stays (it runs on Linux), as do the notify tailed-path rule and the own-state-file read rules.
