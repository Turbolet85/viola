@task_prose part=session
The session made viola Linux-only: all Windows and macOS code, tests, fixtures and CI jobs were removed after the pushed tag multi-os-final (ecbe7ab3), on the founder's rulings of 2026-11-04.
It retired the Windows named-pipe endpoint with its SDDL / SQOS controls, the macOS TMPDIR socket dir and the macOS no-peer-pid carve-out, the Windows owner/DACL strict-modes arm, the ConPTY sideload and its vendored binaries, the .cmd/.bat refusal batch-script-child, the Windows registry environment reader, the Windows VT enable and depth branch, the console-control stop arm, the CASE_INSENSITIVE R8 fold and the Windows relied set of twenty-one.
CI is now ubuntu-latest only: windows-mutants.yml, the windows-2025 and macos-latest legs, junit-os-sets.py, the pwsh shim steps and agent-run.ps1 are gone; ci.yml and nightly.yml remain.
The windows-sys dependency was removed, libc moved to plain dependencies, deny targets went to one Linux triple, and the recvmsg licence exception was dropped at this wrap.
The diag-line schema os enum now reads linux alone, sideload_fallback left process-start, and batch-script-child left the process-exit detail enum (20 to 19).
RUSTSEC-2026-0441 (pin-utils via portable-pty =0.8.1) reads red in supply-chain; a CARRY on Version done-check owns moving portable-pty off 0.8.1.
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
@task_file n=19 path=schemas/diag-line.v1.json
@task_file n=20 path=.github/workflows/ci.yml
@task_file n=21 path=.config/nextest.toml
@task_file n=22 path=.gitattributes
@task_file n=23 path=crates/viola-pty/src/lib.rs
@task_file n=24 path=crates/viola-channel/src/endpoint.rs
@task_file n=25 path=crates/viola-channel/src/client.rs
@task_file n=26 path=crates/viola-channel/src/test_support.rs
@task_file n=27 path=crates/viola-state/src/strict.rs
@task_file n=28 path=crates/viola-state/src/fs.rs
@task_file n=29 path=crates/viola-state/src/pin.rs
@task_file n=30 path=crates/viola-state/src/liveness.rs
@task_file n=31 path=crates/viola-agent-claude/src/lib.rs
@task_file n=32 path=crates/viola-agent-claude/src/ledger.rs
@task_file n=33 path=src/cmd/run.rs
@task_file n=34 path=src/main.rs
@task_file n=35 path=src/run/env.rs
@task_file n=36 path=src/human/style.rs
@task_file n=37 path=src/cmd/verify/stop.rs
@task_file n=38 path=tests/contract_os_gated_tests.rs
@task_file n=39 path=tests/support/home.rs
@task_file n=40 path=scripts/orphans-check.sh
@task_item n=1 part=touches
Design Philosophy and Project Intent: the cross-platform, Windows-first principle, the native cross-platform CLI, and the Scale path line about Windows, macOS and Linux CI and Windows live awaiting a Windows host are retired; the OS target is now Linux only (core on any current distribution, CI on ubuntu-latest, 0.2.0 window features on Hyprland), the Windows and macOS code recoverable at tag multi-os-final.
@task_item n=2 part=touches
[Scale / Product] Personal v1, cross-platform from day one: the built-and-CI-tested on Windows, macOS and Linux claim and its retrofit rationale are stale under the Linux-only ruling.
@task_item n=3 part=touches
Stack and Inherited Defaults: the binary viola / viola.exe naming, GitHub Actions on three OSes and the CI row windows-2025 / macos-latest / ubuntu-latest now read ubuntu-latest only; the windows-sys dependency is removed from the workspace pin and every cfg(windows) table; libc moved from cfg(unix) to plain dependencies in the root, viola-channel, viola-pty, viola-state and viola-e2e.
@task_item n=4 part=touches
build-system / supply-chain config: deny.toml and deny-sync.toml targets went from three triples to x86_64-unknown-linux-gnu alone; deny.toml licence exceptions went from two to one (doctest-file alone, recvmsg dropped as an Windows-only dependency absent from the Linux graph); .gitattributes lost vendor/conpty/** binary.
@task_item n=5 part=touches
[Concurrency / Backend Framework]: the cargo check of the sync crates on all three OSes now runs on one; the sole-root tokio ban itself stays.
@task_item n=6 part=touches
[PTY]: the Windows x64 ConPTY sideload failing open to the inbox one, the System32 DLL search restriction in every viola process (restrict_dll_search), the conpty-sideload and conpty pty_backend words (now always openpty), PtyError::Load, the ReadConsoleW host_stdin reader (now std stdin), the GetConsoleScreenBufferInfo host_size arm and the TerminateProcess kill fallback are removed; the rationale 0.9.0 has a Windows garbage-read bug no longer justifies the =0.8.1 pin.
@task_item n=7 part=touches
[PTY] OPEN item (key lost after resize, sideloaded ConPTY DA1 stall) and every other owner citing the working-route entry Windows-only live measurements: that entry was retired at the 2026-11-04T23-11-43-wrap; sites are re-pointed to the open residuals of origin 2026-11-04T23-11-43-wrap (target another-os) or retired with the Windows claim they owe.
@task_item n=8 part=touches
[Hook Contract]: the dialog hold's Windows leg owner Windows-only live measurements is retired; the ceiling's Version done-check owner stays.
@task_item n=9 part=touches
[Database / State Store] and [Snapshot writer]: the rationale that lock files are separate because append plus exclusive lock fails on Windows is a Windows claim; replace_private is now one attempt (retry_replace, REPLACE_ATTEMPTS, replace_private_retrying, replace_private_with, write_snapshot_with gone; write_snapshot calls replace_private; replace_private_shared stays); create_private_dir lost its DACL arm.
@task_item n=10 part=touches
[Message Broker / IPC] and Local endpoint trust boundary: the Windows named pipe, the SQOS Identification client open and the one-API-over-named-pipes-and-Unix-sockets rationale are retired; the endpoint is a Unix socket only.
@task_item n=11 part=touches
[CI/CD]: the three-OS native matrix, the Windows PATH search over PATHEXT, the .cmd/.bat child refusal (exit 1, batch-script-child) and the claude.cmd / claude.bat npm-shim resolution to claude.exe are retired; resolve_program takes a given path as given and a bare name on PATH, ProgramNotFound unchanged.
@task_item n=12 part=touches
[Agent Coverage]: the npm-shim to claude.exe resolution listed among viola-agent-claude's responsibilities is gone (Search::windows, pathext, is_script, SHIM_TARGET, AgentError::BatchScriptChild removed).
@task_item n=13 part=touches
[CLI Version Compatibility] and the capability ledger: the Windows relied set of twenty-one is gone (WRAPPED_HERE folded): LedgerRow::relied() is all twenty-four rows on every build; the shim-resolution row now reads only that claude --version answered (ProbeRun.program_is_script gone); the ledger scrub matches the home exactly (case-fold parameter gone).
@task_item n=14 part=touches
[Error Handling] and the Rust error types convention: PtyError::Load, PinError::StrictModes, Refused::NullDacl and Refused::NoPersistentAcls are removed variants; the session measured the thiserror::Error census at 10 both before and after (grep -rn over src and crates), where the convention says nine.
@task_item n=15 part=touches
[Deployment / Distribution] and Filesystem bin/: the pinned copy path viola(.exe) and the Windows x64 ConPTY companions bin/<version>-<hash>/conpty/ (pin_companions, the held-open companions, sideload_fallback strict-modes-failed / hash-mismatch) are retired; pin_bytes and the pinned copy re-hash stay.
@task_item n=16 part=touches
CLI --json output contract: viola revive's closed refusal codes went from eleven to ten (batch-script-child removed); verify's refusal codes went from six to five; human::refuse sites run 8 to 7, verify 6 to 5.
@task_item n=17 part=touches
Instance snapshot / server check: the clause on Linux and Windows and the macOS no-peer-pid recorded gap are retired; SERVER_PID_REPORTED and PEER_EUID_REPORTED are folded (peer_euid_admitted is observed == Some(own), an unreadable euid still refused); liveness::server_verdict and verify_server lose the peer_pid_reported parameter and the macOS Matches arm.
@task_item n=18 part=touches
Hook statusline contract: the clause none on Windows for the /bin/sh -c shell-out is stale; statusline::shell_argv now returns a fixed three-element argv, not an Option.
@task_item n=19 part=touches
Occupied Resources IPC endpoints: the Windows named pipe entry, the test-only SQOS pipe of tests/channel_sqos_open.rs (deleted) and the channel test endpoints as named pipes are retired; tests/security_negatives_channel.rs is deleted; the Linux / macOS socket entry is Linux only, socket dir forms XDG_RUNTIME_DIR/viola/ and /tmp/viola-<uid>/, the macOS TMPDIR/viola/ arm gone.
@task_item n=20 part=touches
Environment variables: the R8 strip's Windows registry reader (HKCU Environment) and case-insensitive match are removed (names only, exact match); claude_env_keep is no longer a Unix-qualified pass-list; VIOLA_BIN viola(.exe) naming is stale.
@task_item n=21 part=touches
Filesystem strict-modes rules: the Windows clauses (no read right granted to another SID, the write rule on Windows for diagnostics files, the read rule on Windows for events.ndjson) and the off-Unix Ok arm of check_socket_dir are gone; the Unix check body is unconditional; settings.json written only on Unix is now always.
@task_item n=22 part=touches
Workspace crates test-support feature: viola-channel test_support lost canonical_sddl, dacl_of and user_sid; the JsonFields visitor stays.
@task_item n=23 part=touches
Repository vendor and scripts: vendor/conpty/<version>/x64/ and scripts/conpty-vendor.sh are deleted; target/conpty-seed/ and seed_conpty are gone, so the chaos home is no longer the second carve-out beside seed_conpty; target/harness/ and target/viola-answers-*/ rationales citing Windows relink and windows-2025 drives are stale.
@task_item n=24 part=touches
Repository mutation scratch: viola-mutants-scratch, HOST_SCRATCH and the harness document's scratch_bytes key are retired with windows-mutants.yml (the boundary audit's Windows leg).
@task_item n=25 part=touches
CI artifacts and harness: the three junit-<os> artifacts and scripts/junit-os-sets.py are gone (one junit leg remains, job and artifact names keep the matrix.os form); the harness is scripts/agent-run.sh alone; the ConPTY vendor verification and pwsh shim steps are gone; workflows went from three to two (ci.yml, nightly.yml); zizmor concurrency-limits suppressions 3 to 2.
@task_item n=26 part=touches
Repository scripts: orphans-check.sh narrows to one triple (its probes verdict unchanged); install-ripgrep.sh lost its darwin and windows arms, .zip handling and the Schannel branch; lint-probes.sh native_root is the root (MSYS fallback gone).
@task_item n=27 part=touches
Cross-platform discipline and crosscut-os-gated-tests: every OS-specific branch tested on its CI runner no longer applies across OSes; the census tests/contract_os_gated_tests.rs GATES names exactly the remaining gates, and its cfg(unix) / target_os linux gate lines went from 327 in 78 files to 218 in 64.
@task_item n=28 part=touches
Counts in occupied-resources and test lines: root Instant::now() + WITHIN waits went from 32 sites in 24 files to 31 in 23; the nextest ci kill line binary names went from 27 to 26 (binary(conpty_sideload) removed at .config/nextest.toml line 24).
@task_item n=29 part=touches
Diag-line schema owned by obs, cited in Filesystem diagnostics: os enum now linux only, sideload_fallback property gone from process-start, process-exit detail enum 20 to 19 members.
@task_item n=30 part=touches
Root bin human output and stop: the Windows VT enable (decision rung 7 and the Windows depth rows) is removed, rungs 1 to 6 and the depth rule unchanged; viola verify's stop arm is SIGINT / SIGTERM only, the console-control CTRL_C / CTRL_BREAK arm removed.
@task_fact n=1 slug=established-decisions
fact: RUSTSEC-2026-0441 (pin-utils unmaintained, via portable-pty =0.8.1 then nix 0.25.1) reads red in supply-chain's advisories; a CARRY on Version done-check owns moving portable-pty off 0.8.1 to a release whose graph drops pin-utils, with no ignore-list widening.
@task_fact n=2 slug=established-decisions
fact: The OS target is Linux only by the founder's rulings of 2026-11-04: the core on any current distribution (CI on ubuntu-latest), the 0.2.0 window features on Hyprland; the Windows and macOS code is recoverable at tag multi-os-final (ecbe7ab3).
@task_fact n=3 slug=cross-cutting-patterns
fact: The OS-gate census tests/contract_os_gated_tests.rs reads any gate whose predicate names windows or target_os macos as a fault (another OS) even when a row names it, and its planted self-check proves that class fires.
@task_fact n=4 slug=stack-and-technologies
fact: libc is a plain (not target-gated) dependency of the root, viola-channel, viola-pty, viola-state and viola-e2e, and no crate depends on windows-sys.
@task_item n=1 part=untouched
The JSON-RPC 2.0 envelope, channel methods and their params / ok payloads, protocol v and sender version, the RefusalReason set and the normalised event kinds and event data are unchanged.
@task_item n=2 part=untouched
The CLI exit code set 0, 1, 2, 10 to 14, 20, 21 and exit 21's six unreachable causes stand.
@task_item n=3 part=untouched
The wheel, links, budget governor, delivery confirmation and the vt100 screen-model readiness gate are not reached.
@task_item n=4 part=untouched
MCP (rmcp, its tools, MAX_FRAME stdin bound), Tokio containment in viola-mcp, the sync-crates list and channel-tokio-free.sh are unchanged.
@task_item n=5 part=untouched
The endpoint name hash (FNV-1a, viola-<h12>) and the session liveness heartbeat rule (1 s touch, 5 s stale, pid plus start time for gone) are unchanged.
@task_item n=6 part=untouched
The [CLI Conventions] Git Bash rewritten-path warning stays: it runs on Linux.
@task_item n=7 part=untouched
The fuzz workspace, its six targets and corpus, nightly.yml, Rust 1.98.1 / MSRV 1.96 and the portable-pty =0.8.1 pin itself are unchanged; no dependency was added or bumped.
@task_item n=8 part=untouched
Plugin scope, hook transport tiers, the hook stdout contract and the registered hook events are not changed.
