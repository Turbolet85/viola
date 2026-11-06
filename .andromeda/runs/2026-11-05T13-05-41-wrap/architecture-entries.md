@amendment marker=2026-11-05-linux-only-code-and-ci
title: the OS target: Linux only
section: architecture body (§Design Philosophy, §Established Decisions [Scale / Product], §Project Intent, §Inherited Defaults, §Existing Scopes subjects comment); established-decisions.md ([Scale / Product])
change:
  - §Design Philosophy: was «Cross-platform from the first commit, Windows first» (the Windows, macOS and Linux paths built and CI-passed on every commit, no tmux, shell or POSIX-only dependence, Windows live proven on the CI runner under the fake agent); now «Linux only»: the core on any current distribution, built and tested on every commit in CI on `ubuntu-latest` against the fake agent, the window features of 0.2.0 on Hyprland, the Windows and macOS code out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`); no design choice may depend on tmux or a shell. The R-L1 first live run stands.
  - [Scale / Product]: was «Personal v1, cross-platform from day one», CI-tested on Windows, macOS and Linux, its rationale the cost of retrofitting cross-platform support; now «Personal v1, Linux only», CI-tested on Linux (`ubuntu-latest`), its rationale the Windows and macOS cut, recoverable at `multi-os-final`; the public-version clause kept.
  - established-decisions.md gains the [Scale / Product] detail «Linux only»: the core on any current Linux distribution, CI on `ubuntu-latest`, the 0.2.0 window features on Hyprland; no Windows or macOS code, test, fixture or CI job in the tree, recoverable at `multi-os-final` (`ecbe7ab3`). The subjects comment gains `dec-scale-product`.
  - §Project Intent: was «a native cross-platform CLI binary», now «a native Linux CLI binary»; the scale line was Windows, macOS and Linux CI-tested with Windows live awaiting an interactive Windows host, now v1 personal and Linux only, CI-tested on `ubuntu-latest`, the Windows and macOS code recoverable at `multi-os-final`.
  - §Inherited Defaults CI: was windows-2025 / macos-latest / ubuntu-latest; now `ubuntu-latest`.
why: the founder's own rulings of 2026-11-04 21:43Z, 2026-11-04 21:50Z and 2026-11-04 21:53Z, as the report records them (inputs#I1, inputs#I5 item 6): Viola is Linux only, the Windows and macOS code cut, recoverable at the tag `multi-os-final`, the masters changed once, with the code, at this wrap; the tag was pushed to origin and resolves to `ecbe7ab3`, the parent of the chunk's pre-CI commit.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: CI on the one `ubuntu-latest` leg
section: architecture body (§Stack and Technologies, [Concurrency / Backend Framework], [CI/CD]); established-decisions.md ([Concurrency / Backend Framework], [Delivery Confirmation]); stack-and-technologies.md (CI/CD row); contracts/ci-cd-approach; contracts/project-directory-structure (`ci.yml`)
change:
  - §Stack line: was «GitHub Actions on three OSes»; now «GitHub Actions on `ubuntu-latest`».
  - [CI/CD]: was «native matrix on three OSes»; now «GitHub Actions on `ubuntu-latest` (`strategy.matrix.os: [ubuntu-latest]`)».
  - [Concurrency / Backend Framework]: the sync crates' `cargo check` was on all three OSes, now on `ubuntu-latest`; the `deny-sync.toml` tokio ban was evaluated for the Windows, macOS and Linux triples, now for the one triple `x86_64-unknown-linux-gnu`.
  - [Delivery Confirmation]: the empty-text refusal was proved on the three CI OSes, now on the Linux CI runner (`ubuntu-latest`).
  - Stack CI/CD row: was the matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`; now `[ubuntu-latest]` for `test`, `perf`, `lint` and `release`, every job and artifact name keeping its `${{ matrix.os }}` form.
  - ci-cd-approach: was three workflows, now two (`ci.yml`, matrix `os: [ubuntu-latest]`, and `nightly.yml`); the `windows-mutants.yml` paragraph is now one line: the boundary audit has no Windows leg, `windows-mutants.yml` left with the Windows code, recoverable at `multi-os-final`. The jobs line was «7; 15 check-runs per push» (ci#36483042659), now 7, every one green on ci#42030025050 but `supply-chain`; `test` loses the `ConPTY vendor verification` step and runs `run --coverage` through `scripts/agent-run.sh` at the floors 85/95/80 (were per OS); `perf`, `lint`, `release` and the target jobs run on `ubuntu-latest` (were per OS); the pre-push drops «The other OSes are CI's alone».
  - project-directory-structure: `ci.yml` is push + PR on `ubuntu-latest` (was 3-OS test, perf and release).
why: the report's gate surface: `test`, `perf`, `lint` and `release` keep `strategy.matrix.os: [ubuntu-latest]`, the `ConPTY vendor verification` step and both `pwsh shim` steps leave, `windows-mutants.yml` is deleted (workflows 3 → 2), `deny-sync.toml` targets one Linux triple; ci#42030025050 on `d3690534` read `test`, `lint`, `msrv`, `perf`, `release` and `fuzz-replay` green; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: child program resolution: a given path as given, a bare name on `PATH`
section: architecture body ([Agent Coverage], [CI/CD]); established-decisions.md ([Session Liveness] start order, [PTY], [Error Handling], [CLI Version Compatibility]); standard-contracts.md (the `claude agents --json` read); contracts/project-directory-structure (`viola-agent-claude/`)
change:
  - [Agent Coverage]: was «npm-shim → `claude.exe` resolution»; now «child program resolution (`resolve_program`: a given path as given, a bare name on `PATH`)».
  - [CI/CD]: was a Windows PATH search over PATHEXT only, a `.cmd`/`.bat` child refused except the `claude` npm shim resolving to `claude.exe`; now a given path as given, a bare name searched on `PATH`.
  - [Session Liveness]: `run`'s first step is program resolution (a given path as given, a bare name on `PATH`; it writes nothing), was the `.cmd`/`.bat` refusal.
  - [PTY]: the reason that portable-pty's Windows search hits the extensionless npm sh shim dropped; viola still resolves the child to an absolute path itself.
  - [Error Handling]: `AgentError` holds `resolve_program`'s one refusal, `ProgramNotFound` (was two, with `BatchScriptChild`).
  - [CLI Version Compatibility]: the item «npm-shim → `claude.exe` resolution» is now `shim-resolution`: `claude --version` answering (the row reads `run.version_answered` alone).
  - standard-contracts.md: `list` and `revive` resolve `claude` with `run::resolve_program` (a given path as given, a bare name on `PATH`), was the npm-shim → `claude.exe` resolution.
  - project-directory-structure: the `viola-agent-claude/` comment reads «program resolution» (was «shim resolution»).
why: the report's Symbols: `resolve_program`'s `Search::windows`, `pathext`, `HOST_IS_WINDOWS`, `is_script`, `SHIM_TARGET` and `AgentError::BatchScriptChild` removed, `ProgramNotFound` unchanged; `ProbeRun.program_is_script` left with `is_script`, so the `shim-resolution` row reads `run.version_answered` alone, its verdict unchanged on Linux, where a `.cmd` name is an ordinary executable (deviation 1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: [PTY]: openpty alone, the Windows ConPTY host retired
section: architecture body ([PTY] and its OPEN line); established-decisions.md ([PTY], the retired 1-column ConPTY OPEN); stack-and-technologies.md (PTY layer row); contracts/project-directory-structure (`main.rs`, `viola-pty/`)
change:
  - [PTY] (body): was the sideloaded ConPTY on Windows x64, failing open to the inbox one, every `viola` process restricting its DLL search to System32, rationale the 0.9.0 Windows garbage-read bug; now `pty_backend()` is `openpty`, rationale 0.8.1 spike-proven and moving `portable-pty` off it to a release whose graph drops `pin-utils` owed, owner «Version done-check».
  - established-decisions.md [PTY]: retired the ConPTY no-EOF reason, the `TerminateProcess` fallback, the `ReadConsoleW` host stdin, the Windows console modes, the `%USERPROFILE%` cwd default, the input writer's `CTRL_CLOSE`, the H2 resize key-loss readings on `windows-2025` and the sideload with its DA1 stall. Now `host_stdin()` is `std::io::stdin()`, `HostTerminal` sets raw mode (Unix termios), the pump keeps the input writer until the child exits, `pty_backend()` is always `openpty` and `host_size()` reads `TIOCGWINSZ`; wezterm#6783 does not bear on a Linux-only build; the resize rule (no human key held behind a resize) stands.
  - The OPEN was the resize key and the sideload DA1 stall, owner «Windows-only live measurements»; now the resize key alone, owner the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`). The OPEN on the 1-column ConPTY wide-character reading retired.
  - Stack PTY layer row: windows-sys and the vendored ConPTY host gone; libc `=0.2.189` for raw mode behind `HostTerminal` (was Unix only), input `std::io::stdin()`, size `TIOCGWINSZ`; `claude` hosted in openpty.
  - project-directory-structure: `main.rs` loses its DLL-search note; `viola-pty/` reads libc termios and `std::io::stdin()`, `sideload` removed, recoverable at `multi-os-final`.
why: the report's Symbols remove `mod sideload`, the `conpty-sideload` · `conpty` words, the `ReadConsoleW` reader, the `GetConsoleScreenBufferInfo` arm, the `TerminateProcess` fallback and `PtyError::Load`, child exit still on the handle wait; «Windows-only live measurements» was retired at the 2026-11-04T23-11-43-wrap, its residuals at that origin, target `another-os`; the 0.9.0 bug that justified `=0.8.1` leaves with this chunk.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: RUSTSEC-2026-0441 red, owned by «Version done-check»
section: established-decisions.md ([PTY]); architecture body ([PTY] rationale); contracts/ci-cd-approach (the jobs line)
change:
  - New fact: RUSTSEC-2026-0441 (`pin-utils` unmaintained, via `portable-pty =0.8.1` → `nix 0.25.1`) reads red in supply-chain's advisories family, as measured at ci#42030025050; its owner is a CARRY on the working-route entry «Version done-check»: moving `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`, with no ignore-list widening.
  - The [PTY] rationale names that move as owed; the ci-cd-approach jobs line names `supply-chain` red on RUSTSEC-2026-0441 alone at ci#42030025050.
why: ci#42030025050 on `d3690534` failed `supply-chain` on that one error, the same red on the untouched tree at the phase baseline; the CARRY is the overseer's technical answer (inputs#I3, 2026-11-05 11:51Z), approved by the operator's own word (inputs#I4, 2026-11-05 12:02Z).
kept: `portable-pty` stays pinned `=0.8.1` until that move.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the separate lock files: history of the Windows target, the layout kept
section: architecture body ([Database / State Store]); established-decisions.md ([Database / State Store])
change:
  - body: was «lock files are separate because append + exclusive lock fails on Windows»; now lock files are separate, a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.
  - established-decisions.md: lock files are separate from logs, so no file is both appended to and exclusively locked (that pair fails on Windows, rust-lang/rust#54118); was «because append + exclusive lock fails on Windows».
why: the OS target is Linux only (the founder's rulings of 2026-11-04, inputs#I1); the report's os-target class re-words a Windows claim to the Linux statement, the claim it carries kept, and the chunk reports no change to the lock layout.
kept: the ndjson logs, the atomic snapshots and the separate `.lock` files, unchanged.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the wrapper endpoint is a Unix socket only
section: architecture body ([Message Broker / IPC], §Occupied Resources IPC endpoints, §Cross-cutting Patterns Local endpoint trust boundary); established-decisions.md ([Message Broker / IPC]); occupied-resources.md (IPC endpoints); stack-and-technologies.md (Wrapper IPC row)
change:
  - Retired: the Windows named pipe `\\.\pipe\viola-<h12>`, the client's SQOS Identification open (`CreateFileW` with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`, adopted by `Stream::try_from`) and the pipe's SDDL DACL; the rationale was «one API over named pipes and Unix sockets», now «one API over Unix sockets»; the macOS socket-path cap note dropped.
  - The socket entry: was «Linux / macOS», `$TMPDIR/viola/` on macOS, never the `/tmp` or `$TMPDIR` root; now «Linux»: `$XDG_RUNTIME_DIR/viola/`, `/tmp/viola-<uid>/` as the fallback, never the `/tmp` root.
  - Test-only endpoints: the SQOS pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` (`tests/channel_sqos_open.rs`) and `tests/security_negatives_channel.rs` dropped; `viola-test-chan-<pid>-<label>` is a socket of that base name.
  - Trust boundary: each endpoint, a Unix socket file, must not be connectable by another OS user; the security specialist owns its enforcement (was «per OS»).
  - Stack: the retired Wrapper IPC row is replaced by one naming the sync API, the feature-gated `TokioClient`, libc's uid for the socket directory, the 0600 chmod after the bind and `check_endpoint_dir`; one endpoint per `viola run`, a Unix domain socket.
why: the report retires the Windows named-pipe endpoint, its SDDL / SQOS / `accept_remote(false)` controls and the test-only SQOS pipe, and the macOS `$TMPDIR/viola/` socket dir; its Symbols remove `server/win.rs` and the SQOS and named-pipe arms of `client.rs`, `endpoint.rs`, `tokio_client.rs` and `server.rs`, and the two test files are deleted; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the Windows legs owed to «Windows-only live measurements» retired
section: architecture body ([Hook Contract]); established-decisions.md ([Hook Contract], [Delivery Confirmation], [Budget Governor], [CLI Version Compatibility], [Deployment / Distribution], [Plugin Scope]); cross-cutting-patterns.md (Capability ledger); occupied-resources.md (settings override); standard-contracts.md (Hook contract shell-out)
change:
  - [Hook Contract]: the dialog hold's Windows leg dropped from the body line and from the hold-ceiling item; the PROVISIONAL ceiling's owner «Version done-check» stays.
  - Capability ledger and [CLI Version Compatibility]: the Windows readings of `paste-newline`, `resume-session`, `local-command-compact`, `agents-join` and the three statusline rows, each owed to that entry, dropped; their Linux dev-host stamps on 2.1.287 stand.
  - [Delivery Confirmation]: «an LF typed inside a paste on Windows» leaves the not-measured list.
  - [Budget Governor]: the statusline wrap's Unix-only cut and its Windows half owed to that entry dropped.
  - [Deployment / Distribution] (the exit-1 `viola mcp` reading) and [Plugin Scope] (plugin precedence): their Windows legs dropped.
  - The settings override's Windows override and the statusline shell-out's Windows half, both owed to that entry, dropped.
why: the working-route entry «Windows-only live measurements» was retired at the 2026-11-04T23-11-43-wrap (its 17 residuals to origin `2026-11-04T23-11-43-wrap`, target `another-os`); the report's expected amendments re-point its master sites to those residuals or retire them with the Windows claim they owe; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the server identity check on every connect, the macOS gap retired
section: architecture body (§Standard Contracts instance snapshot); conventions.md (exit 21, the MCP frame tool, the retired macOS exception); established-decisions.md ([Message Broker / IPC], [Session Liveness]); occupied-resources.md (the server check); stack-and-technologies.md (Wrapper IPC peer creds); standard-contracts.md (channel methods, session liveness); contracts/project-directory-structure (`viola-channel/`)
change:
  - Retired: the macOS no-peer-pid gap (`server_verdict` returning `Matches` with no comparison, `server-verify-failed` an euid mismatch alone on macOS) and the conventions exception naming it; the Windows `GetNamedPipeServerProcessId` pid; the «no euid on Windows» clause.
  - `server_pid` reads `peer_creds().pid()` (`SO_PEERCRED`); `server_verdict(recorded_pid, started_at, observed_pid, observed_start)` and `verify_server(recorded_pid, started_at, observed_pid)` lose `peer_pid_reported` and always compare the peer's pid and OS start time with the snapshot's.
  - `viola_channel` exports `peer_euid_admitted` and `peer_admitted` (`PEER_EUID_REPORTED` gone): `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so an unreadable euid is refused.
  - The client's checks before its first frame, exit 21's `server-verify-failed` cause and the MCP frame tool read with no OS split (were Unix euid, Linux and Windows pid).
  - `check_socket_dir` loses its off-Unix `Ok(())` arm.
why: the report retires the macOS no-peer-pid carve-out; its Symbols remove `SERVER_PID_REPORTED` and `PEER_EUID_REPORTED` (their Linux value folded, no seam added), the `peer_pid_reported` parameter with the macOS `Matches` arm, and `check_socket_dir`'s off-Unix arm, its Unix body unconditional; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the `.cmd`/`.bat` refusal `batch-script-child` retired
section: architecture body (§Standard Contracts CLI `--json` output); conventions.md (exit codes); established-decisions.md ([CLI Version Compatibility], [CLI Conventions]); occupied-resources.md (subcommands, verify's refusal codes); standard-contracts.md (`Launch`); contracts/build-system (the human writer's callers)
change:
  - revive's closed `--json` refusal codes: was eleven, with `batch-script-child`; now ten: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
  - verify's `Ended::Refused` codes: was six, now five: `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`.
  - conventions exit `1`: `viola run`'s refusal of a `.cmd`/`.bat` child and verify's pair for a `.cmd` or `.bat` CLI removed.
  - `refused_with` prints `refused_document` for `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` (was with `batch-script-child`); the human writer's callers lose the `.cmd`/`.bat` child refusal.
why: the report retires the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild`, its human pair); its Counts move revive's codes 11 → 10 and verify's 6 → 5, pinned in `tests/cross_exit_causes.rs`; a closed word no build can emit leaves with its emitter (the overseer's technical answer, inputs#I2, 2026-11-05 11:08Z).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the statusline wrap and its probe on every build
section: architecture body (§Standard Contracts Hook contract, §Occupied Resources settings override); established-decisions.md ([Budget Governor], [Session Liveness], [CLI Version Compatibility], [Plugin Scope]); occupied-resources.md (child flags, verify's probe plugin, settings override, `instances/<ViolaName>/`, `ledger/probes/<pid>/`); standard-contracts.md (Hook contract shell-out)
change:
  - The per-session settings override is written for a pinned path of plain characters on every build (was Unix only, none on Windows), and the child's `--settings` follows it; [Session Liveness]'s override step and `settings.json` lose «Unix only».
  - `viola hook statusline` runs the user's command through `/bin/sh -c` with no Windows exception; `statusline::shell_argv` is the fixed three-element argv `[&str; 3]` (was `None` for every input on Windows).
  - Run B's probe `--settings <probe dir>/typed/statusline.json` is passed for shell-plain paths on every build (was on a wrapped OS, Unix); `typed/statusline.json` loses «on Unix»; [Plugin Scope]: Run B's status line is the probe capture, Runs C and D the user's own (the Windows Run B case dropped).
  - [Budget Governor]: the wrap is no longer stated Unix only; `statusline-override`, `statusline-shell` and `statusline-rate-limits` are relied on every build (were Unix only).
why: the report's Symbols fold `WRAPPED_HERE`'s `true`, so the wrap no longer branches by OS, and `shell_argv` returns `[&str; 3]` because with `WRAPPED_HERE` folded it was always `Some` (deviation 3); the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: one binary name, `viola`
section: architecture body (§Stack and Technologies, §Occupied Resources Binary, `VIOLA_BIN`, pinned copy); stack-and-technologies.md (Language / runtime row); established-decisions.md ([Deployment / Distribution]); occupied-resources.md (pinned copy, `check_home_tree`, `target/agent-run/<session>/`)
change:
  - was `viola` / `viola.exe` (and `viola(.exe)`, «`viola.exe` on Windows»); now `viola` alone: the binary, the Stack line and Language row, `VIOLA_BIN`'s `<viola home>/bin/<version>-<hash>/viola`, the pinned copy (0700, was «0700 on Unix»), `check_home_tree`'s pinned `viola` and [Deployment / Distribution]'s copy path.
  - The harness session's copy is `bin/claude` (was `bin/claude[.exe]`).
why: the OS target is Linux only, the Windows code removed, recoverable at `multi-os-final` (the founder's rulings of 2026-11-04, inputs#I1); no Windows binary is built.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: strict modes: the Windows owner/DACL arm retired
section: architecture body (§Occupied Resources: the `viola-channel` test-only feature, `diagnostics/`, `instances/<ViolaName>/`); occupied-resources.md (the `diagnostics/` opens, the `instances/` checks, the read rule, the checked entry points, a path under a regular file, `viola list`'s home rule, `ledger/stamps.json`, the exit lines; three retired Windows ACL notes); stack-and-technologies.md (State-file primitives); contracts/project-directory-structure (`viola-state/`)
change:
  - The read rule: was the write rule plus Unix group/other bits and, on Windows, an allow ACE granting `READ_RIGHTS` to an untrusted SID (`windows_read_verdict`); now the write rule plus a refusal of any group or other bit (`mode & 0o077`, `unix_read_verdict`).
  - `diagnostics/` files take the read rule (were the write rule on Windows), `events.ndjson` the write rule (was the read rule on Windows); modes 0700 / 0600 lose «on Unix»; the checked entry points leave the mode as found (was mode or ACL); `viola list`'s home rule and `ledger/stamps.json`'s strict read (owner-only modes) lose their Windows clauses.
  - A path under a regular file is a strict-modes refusal (was `NotFound` on Windows, a per-OS oracle); the check is proven by five widened modes (the `cfg(windows)` cases dropped); the exit lines lose «on each OS».
  - Retired: the three `windows-2025` ACL notes (the protected DACL's SDDL, the runner-inherited temp-dir ACL, the `icacls` grant through `seed_conpty`'s links) and `test_support`'s `canonical_sddl`, `dacl_of`, `user_sid`.
  - State-file primitives: the checks and private-dir creation through libc (modes and owner); windows-sys's owner + DACL arm and the Windows append-lock note dropped. The `strict` comment lists `unix_read_verdict` alone.
why: the report retires the Windows home owner/DACL strict-modes arm; its Symbols remove `strict.rs`'s Windows arm, `windows_read_verdict`, `READ_RIGHTS`, `Refused::NullDacl`, `Refused::NoPersistentAcls`, `outside_profile`, `create_private_dir`'s DACL arm and the SDDL helpers; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the R8 strip: exact names, `claude_env_keep` the one pass-list
section: architecture body (§Occupied Resources environment variables, `config.json`); occupied-resources.md (the R8 strip, `config.json`); cross-cutting-patterns.md (Config settings)
change:
  - was every inherited `CLAUDE*` name (case-insensitive on Windows) removed except a name the user defines persistently: on Windows a value name under `HKCU\Environment` or the machine `Session Manager\Environment`, read by the registry reader, on Unix a name in `claude_env_keep`; now every inherited `CLAUDE*` name, matched exactly, except a name in `config.json` `claude_env_keep`, the identity floor's eleven names removed whatever the persistent environment says; the strip and the logs never read a value (the registry reader dropped).
  - `claude_env_keep` is «the R8 pass-list» (was «the Unix R8 pass-list») in the body, occupied-resources.md and the Config settings.
why: the report's Symbols remove `src/run/env.rs`'s registry reader (`HKCU\Environment`) and case-insensitive match, the exact strip, `claude_env_keep` and the eleven-name identity floor staying, and `CASE_INSENSITIVE` (the R8 strip matches names exactly).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the ConPTY companions and their vendored binaries retired
section: architecture body (§Occupied Resources: the chaos home's carve-out; the retired companion, `vendor/conpty/` and `target/conpty-seed/` items); established-decisions.md ([Session Liveness] start order, [Deployment / Distribution], the retired `pin_companions` check); stack-and-technologies.md (Content hash row); contracts/deployment-model; contracts/project-directory-structure (`conpty.rs`, `conpty-vendor.sh`, `vendor/conpty/`)
change:
  - Retired: `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (written if absent, owner/DACL-checked, held `FILE_SHARE_READ` and re-hashed, failing open to the inbox ConPTY), `vendor/conpty/<version>/x64/` with `src/conpty.rs` and `scripts/conpty-vendor.sh`, `target/conpty-seed/<key>/` with `seed_conpty`, and `pin_companions`' check (`PinError::StrictModes`).
  - [Deployment / Distribution]: the companions' embedding, held re-hash and first-start write timing dropped; `viola run` pins its own exe and re-hashes an existing copy, a mismatch refusing the start.
  - [Session Liveness]: `run`'s start loses its ConPTY sideload step (`run.conpty_sideload`).
  - Content hash row: the companions' full SHA-256 dropped; the exe key and the pinned-copy re-hash stay.
  - The chaos home `viola-chaos-*` is the one named carve-out to the test-data rule (was the second, beside `seed_conpty`).
  - deployment-model: the pinned copy carries no `conpty/` subdirectory and no `conpty.dll` pre-load; project-directory-structure marks `conpty.rs`, `conpty-vendor.sh` and `vendor/conpty/` removed, recoverable at `multi-os-final`.
why: the report retires the ConPTY sideload (`vendor/conpty/<version>/x64/`, `bin/<version>-<hash>/conpty/`, `target/conpty-seed/`, `scripts/conpty-vendor.sh`, `sideload_fallback`, `run.conpty_sideload`); it deletes `src/conpty.rs`, `scripts/conpty-vendor.sh` and the `vendor/conpty/` tree and removes `pin_companions`, the companions' held open, `PinError::StrictModes` and `seed_conpty`, `pin_bytes` and the pinned copy's re-hash staying.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: test support and harness: the Windows and macOS arms retired
section: architecture body (§Occupied Resources `target/viola-answers-*/`); occupied-resources.md (`Frozen`, the test socket roots, the `viola-harness` shim, the `target/e2e-home` keepers, the retired `scripts/junit-os-sets.py`); contracts/project-directory-structure (`tests/support/`, `scripts/agent-run.sh`)
change:
  - `tests/support/frozen.rs` `Frozen` is ungated: `kill -STOP` polled to `ps` state `T`, `kill -CONT` on drop (the Windows debugger twin dropped).
  - `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR` (was `TMPDIR` on macOS).
  - `viola-harness` sits behind `scripts/agent-run.sh` alone (was `scripts/agent-run.{sh,ps1}`).
  - The `target/e2e-home` keepers: the linked-base contract holds with no «On Unix», the Windows plain-create clause dropped; `HEADROOM_FLOOR_MIB` unchanged.
  - `target/viola-answers-*/`: the Windows two-drive reason dropped; it sits under the current dir.
  - Retired: `scripts/junit-os-sets.py`, the operator pass's per-OS JUnit reading.
why: the report's Symbols keep `Frozen`'s `kill -STOP` twin ungated, make `SOCKET_ROOT_VAR` read `XDG_RUNTIME_DIR` and keep `ensure_e2e_home`'s signature, `HEADROOM_FLOOR_MIB` and its message unchanged; `scripts/agent-run.ps1` and `scripts/junit-os-sets.py` are deleted; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: mutation runs: no scratch arm, no Windows leg
section: architecture body (§Occupied Resources `<repo parent>/viola-mutants-scratch/`); occupied-resources.md (`viola-mutants-scratch/`, `AGENT_RUN_CHUNK_BASE`); contracts/project-directory-structure (`windows-mutants.yml`)
change:
  - `viola-mutants-scratch/`: was the host mutation scratch, where on a Windows host `run --mutants` put cargo-mutants' temp copies and `--output`, guarded, wiped and recreated, dispatched by `windows-mutants.yml`; now on the Linux dev host the operator's NOCOW `TMPDIR` for mutation runs, an environment fact, not harness code; `run --mutants` has no scratch arm and `mutants.out/` stays at the repository root.
  - `AGENT_RUN_CHUNK_BASE`: no CI workflow sets it, `ci.yml` and `nightly.yml` running no mutation job (the `windows-mutants.yml` clause dropped).
  - project-directory-structure marks `windows-mutants.yml` removed, recoverable at `multi-os-final`: no workflow replaces the boundary audit's Windows leg, and the audit runs `run --mutants` on Linux alone.
why: the report retires `windows-mutants.yml` (the boundary audit's Windows leg) with `viola-mutants-scratch` / `HOST_SCRATCH` / `scratch_bytes`; it deletes `scratch.rs` whole, `mutants.rs` losing `scratch_bytes` and its `TMP`/`TEMP`/`--output` arm (deviation 4).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the OS-gate census faults a gate naming another OS
section: architecture body (§Cross-cutting Patterns); cross-cutting-patterns.md (OS-gated tests named with their reason)
change:
  - body: was «Cross-platform discipline», now «Process and path discipline»; nothing shells out through `sh` or `bash` (was `sh`, `bash` or `cmd`); was «every OS-specific branch compiles and is tested on its CI runner», now the product builds and is tested on Linux (CI on `ubuntu-latest`), every OS-gated test names its reason in `tests/contract_os_gated_tests.rs`, and a gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a row names it.
  - cross-cutting-patterns.md: the same fault class, which `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` proves fires; the planted Windows cases build the word from a `WINDOWS` const.
why: the report's gate surface: `GATES` names exactly the remaining gates and a gate naming `windows` or `target_os = "macos"` reads as a fault even when named, its self-check re-planted; `contract_os_gated_tests` read green with every planted class firing; the new class caught a multi-line `cfg_attr` pair in `tests/hook_fail_open.rs` that a line grep cannot see (deviation 9).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the relied set: all twenty-four rows on every build
section: cross-cutting-patterns.md (Capability ledger); established-decisions.md ([CLI Version Compatibility]); occupied-resources.md (the 2.1.287 fixture set, settings override); standard-contracts.md (Ledger stamps, the agents read); contracts/ci-cd-approach (`viola verify` in CI); contracts/crate-dependency-direction (`viola-agent-claude`)
change:
  - `LedgerRow::relied()`: was `ALL` filtered by `relied_on(row, statusline::WRAPPED_HERE)`, twenty-four rows on Unix and twenty-one on Windows (the first eighteen, `resume-session`, `agents-join`, `local-command-compact`); now every row of `ALL`, in its order, all twenty-four on every build.
  - verify's step lines: `MM` is 24, the last `[24/24] local-command-compact …` (the `[21/21]` Windows form dropped).
  - The stamp verdicts read on Windows alone (unverified there, or verified there only) dropped; an all-`pass` stamp of an older id set reads unverified.
  - `agents-join`, `resume-session`, `local-command-compact` and `statusline-override` are relied on every build (were «every OS» or Unix only); the 2.1.287 fixtures are stamped at all twenty-four rows, `24 pass  0 fail`, with no «on Unix».
  - `viola-agent-claude`'s exports lose `WRAPPED_HERE`, `CASE_INSENSITIVE` and `is_script`.
why: the report's Symbols fold `WRAPPED_HERE`'s `true`, so `LedgerRow::relied()` is all twenty-four rows on every build, and remove `CASE_INSENSITIVE` and `is_script`; its Counts move the Windows relied set twenty-one → none; `viola verify` reads `[NN/24]` on Linux (`tests/cli_verify.rs`).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: [Snapshot writer]: `replace_private` is one attempt
section: established-decisions.md ([Snapshot writer]); stack-and-technologies.md (State-file primitives)
change:
  - was: on Windows a `persist` refused with raw OS error 5 retried every 10 ms, at most `REPLACE_ATTEMPTS` = 100, through `replace_private_retrying` and `replace_private_with` (`retry_replace(_, HOST_IS_WINDOWS)`), and the companions' held open retrying Win32 error 32 (`retry_open`, `open_held_waiting`); now `replace_private` is one attempt, a failed `persist` returning at once with the target as it was, and `write_snapshot` calls it.
  - `replace_private_shared` serves the plugin files (the ConPTY companions dropped from its list).
  - `place_private_new_placed_copy_execs_at_once` execs the placed copy on Unix; its Windows twin `place_private_new_placed_copy_runs_at_once` dropped.
  - State-file primitives: `replace_private`, one attempt.
why: the report's Symbols remove `fs.rs`'s `HOST_IS_WINDOWS`, `retry_replace`, `REPLACE_ATTEMPTS`, `replace_private_retrying` and `replace_private_with` (`write_snapshot_with` gone, `replace_private_shared` stays) and `pin.rs`'s `retry_open` and `open_held_waiting`; their only role was the retry pause (deviation 6).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: [Human Takeover / Wheel]: the Windows input passage retired
section: established-decisions.md ([Human Takeover / Wheel])
change:
  - Retired: what arrives on Windows under the founder's live ruling F-W3 (2026-10-04): the inbox ConPTY outer terminal swallowing focus reports, and a mouse report under the sideloaded ConPTY's win32-input-mode arriving as win32 key-down records and taking the wheel (ci#37227518624).
why: the report retires the Windows target and the ConPTY sideload and removes `host_stdin()`'s console reader, so no Windows input path remains; the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
kept: the bridge send as one bracketed write + CR, human bytes held during it, and the prompt-origin rules that move the wheel, unchanged.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: verify's stop: `SIGINT` / `SIGTERM` alone
section: occupied-resources.md (`ledger/probes/<pid>/`, verify's interactive probe dirs); contracts/project-directory-structure (`verify/stop.rs`)
change:
  - verify's probe dirs are removed when it is stopped by `SIGINT` / `SIGTERM` (`cmd::verify::stop`), was also Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`; the leftovers are SIGKILL, a crash and a failed handler install (a Windows close, logoff or shutdown dropped).
  - project-directory-structure: `verify/stop.rs` reads «(SIGINT / SIGTERM)».
why: the report's Symbols remove `src/cmd/verify/stop.rs`'s console-control arm (`CTRL_C_EVENT` / `CTRL_BREAK_EVENT`), `SIGINT` / `SIGTERM` unconditional.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the root `WITHIN` waits: 31 sites in 23 files
section: occupied-resources.md (`<temp dir>/viola-root-watch/`)
change:
  - `Instant::now() + WITHIN` reads 31 sites in 23 files under `tests/` (was 32 in 24, at chunk 2026-11-04-linux-and-macos-parity's wrap), as measured at chunk 2026-11-05-linux-only-code-and-ci's wrap on the Linux dev host.
  - `Frozen`'s `attach` polls `ps -o stat=` with no «on Unix»; `holder_gone` is a connect refused or the path absent (the Windows pipe-not-found arm dropped); the kill-and-revive case is cited green on a run of the retired three-OS matrix.
why: the report's Counts re-measure the root waits by `grep -rn` / `grep -rln` at this wrap, 32 sites in 24 files → 31 in 23; the Windows pipe endpoint is retired.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: windows-sys out, libc a plain dependency
section: stack-and-technologies.md (new OS bindings row); established-decisions.md ([Error Handling] `PtyError`); contracts/crate-dependency-direction (`viola-pty`, `viola-channel`, `viola-state`, the root bin); contracts/project-directory-structure (`human/` `style.rs`, `verbs.rs`)
change:
  - New Stack row, OS bindings: libc, a plain (not target-gated) `[dependencies]` entry of the root, `viola-channel`, `viola-pty`, `viola-state` and `viola-e2e`; no crate depends on windows-sys.
  - `viola-pty` → portable-pty, tracing, libc (windows-sys «Windows only» and libc «Unix only» gone); `PtyError`'s rationale lists portable-pty, tracing and libc.
  - `viola-channel` → libc for the socket directory's uid; windows-sys (the SQOS open, the listener DACL) and `client::open_handle` dropped.
  - `viola-state` → libc for the `strict` owner + mode check; windows-sys (owner + DACL, the protected DACL, `Win32_UI_Shell`) dropped.
  - The root bin → all members, clap, anyhow, tempfile and libc `=0.2.189` (the panic frames, `sigaction`); windows-sys (the R8 registry reader, Windows panic frames, `SetConsoleCtrlHandler`, the VT enable) dropped; the project tree's `style.rs` loses the Windows VT enable, `verbs.rs` its `windows-mutants.yml` note.
why: the report's Dependencies remove `windows-sys` from the workspace pin and every `cfg(windows)` table and move libc from `[target.'cfg(unix)'.dependencies]` to plain `[dependencies]` in those five manifests, no dependency added or bumped; its Symbols remove the Windows VT enable, rungs 1–6 unchanged (deviation 8).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: cargo-deny: one Linux triple, one 0BSD exception
section: contracts/build-system (Dependency policy)
change:
  - `deny.toml` and `deny-sync.toml` are evaluated for the one target triple `x86_64-unknown-linux-gnu` (was the Windows, macOS and Linux triples, so `cfg`-gated dependencies were covered).
  - The 0BSD licence exceptions: was `doctest-file` and `recvmsg`, the two 0BSD crates interprocess 2.4.4 pulls in; now `doctest-file` alone, the one it pulls into the Linux graph; the 2026-09-25 operator ratification stands.
why: the report's Schema / config: both configs read `targets = ["x86_64-unknown-linux-gnu"]`; the `recvmsg` exception was dropped at this wrap, its crate interprocess's Windows-only dependency, out of the Linux graph (CI job 127253812277 warned `license-exception-not-encountered`), on the overseer's technical answer (inputs#I5 item 7) that an exception naming a crate no longer in the graph narrows and widens nothing; `cargo deny check licenses` read `licenses ok` at this wrap.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
