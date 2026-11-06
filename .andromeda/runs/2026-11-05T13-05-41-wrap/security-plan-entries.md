@amendment marker=2026-11-05-linux-only-code-and-ci
title: the IPC endpoint is a Unix socket alone; the Windows named pipe retired
section: §Threat Model Summary (body: the tier justification, Networking; threat-model-summary.md: the IPC vector) · §Authentication & Authorization (body: Auth approach, IPC access control (Windows) retired; authentication-authorization.md: IPC access control (Unix) and its directory check, the IPC access control (Windows) row and its two witnesses retired) · §Data Protection (body: TLS) · §Security Anti-Patterns (body and security-anti-patterns.md: the no-switch-off ban; body: the listener and client-pipe bans retired) · §Bootstrap phases (body: auth-scaffolding-baseline; bootstrap-phases.md: the viola-channel line)
change:
  - The endpoint is one per-instance Unix socket `<dir>/viola-<h12>.sock` in a verified per-user 0700 directory, `$XDG_RUNTIME_DIR/viola/` or the `/tmp/viola-<uid>/` fallback, never the `/tmp` root (was named pipes on Windows or Unix sockets, with a macOS `$TMPDIR/viola/` form).
  - The tier justification reads the socket name predictable (was the names predictable in the shared Windows pipe namespace); Auth approach reads OS-user ambient trust enforced on Linux (was per OS); the enforcement is socket file mode and directory permissions (was the pipe DACL among them).
  - The socket is chmod 0600 right after the bind, `ListenerOptionsExt::mode(0o600)` unused because chmod after the bind is the one form (was because interprocess returns `Unsupported` on macOS); `check_socket_dir` has no off-Unix `Ok(())` arm.
  - Confidentiality in transit rests on the socket-directory and peer checks (was the IPC ACLs); the no-switch-off ban names the socket-directory check (was the IPC DACL or socket-directory check).
  - Retired: IPC access control (Windows) (the protected user + SYSTEM DACL, `accept_remote(false)`, the first-pipe-instance behaviour, `inheritable(false)`), its remote-client rejection and DACL-effect witnesses, and the bans on a Windows listener without `security_descriptor` and a client pipe without SQOS.
  - auth-scaffolding-baseline and the `viola-channel` bootstrap line read Linux socket hardening (socket directory, chmod 0600, peer euid check) (was per-OS listener hardening with the Windows SDDL).
why: the founder's own rulings of 2026-11-04 (three, the last at 2026-11-04 21:53Z; inputs#I1, inputs#I5 item 6): viola is Linux-only, the Windows and macOS code cut, recoverable at the tag `multi-os-final` (`ecbe7ab3`); the chunk removed `server/win.rs`, the named-pipe arms and the macOS socket-directory arm.
kept: the Linux abstract namespace is still never used; the per-user 0700 directory check and its `socket-dir-refused` refusal stand unchanged.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the home strict-modes check is owner and mode alone; the Windows DACL arm retired
section: §Threat Model Summary (body: the filesystem vector; threat-model-summary.md: the code-bearing artefacts, the board scan) · §Authentication & Authorization (body: ~/.viola/ access control; authentication-authorization.md: the strict-modes check, the board scan, check_home_tree) · §Data Protection (body: Files, the umask ban; data-protection.md: Files, the diagnostics opens) · §Error Handling (error-handling.md: the unstattable home, check_diagnostics) · §Input Validation (input-validation.md: Home resolution) · §Bootstrap phases (bootstrap-phases.md: the viola-state lines) · §Security Anti-Patterns (body: the exec-form command ban)
change:
  - The strict-modes check reads owner and mode (was owner and mode, or owner and DACL); the home is 0700 directories and 0600 files, the pinned `viola` 0700 (was also the inherited profile DACL or a protected user + SYSTEM DACL outside it); the read rule is any group or other bit (`mode & 0o077`).
  - Retired: the Windows strict-modes paragraphs (owner + DACL entry points, NULL DACL, persistent-ACL volumes, the ACE write and read rules, `READ_RIGHTS`, `CREATOR OWNER`), the Windows clauses of the umask ban, and the Windows `NotFound` exception: an unstattable home is a strict-modes refusal.
  - `events.ndjson` takes the write rule (was the read rule on Windows); a diagnostics file takes the read rule (was the write rule on Windows); `check_diagnostics` refuses a mode another user may use (was a mode or grant).
  - The board's read rule and `check_path` hold with no «every OS» qualifier, the OPEN on the board's read rule unchanged.
  - The strict-modes witnesses are `cfg(unix)` cases, green on Linux at ci#42030025050 (was also `cfg(windows)` cases, green on all three OSes at ci#40403389629); the six frame verbs proven on Linux.
  - The pinned path reads `bin/<version>-<hash>/viola` in every row (was `viola(.exe)`); `canonical_home` strips no Windows verbatim-drive prefix; windows-sys leaves the stack references.
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6), Linux-only; the chunk removed `strict.rs`'s Windows owner/DACL arm, `windows_read_verdict`, `READ_RIGHTS`, `Refused::NullDacl`, `Refused::NoPersistentAcls`, `outside_profile` and the DACL arm of `create_private_dir`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: CI runs one ubuntu-latest leg; windows-mutants.yml and the mutation scratch retired
section: §Threat Model Summary (body: CI/CD; threat-model-summary.md: the CI/CD rows) · §Dependency Security (body: CI integration, the concurrency and event-payload bullets, the windows-mutants.yml line retired; dependency-security.md: the windows-mutants.yml row retired) · §Security Anti-Patterns (body and security-anti-patterns.md: the run --mutants scratch-wipe ban retired) · §Authentication & Authorization (authentication-authorization.md: the socket-directory negatives)
change:
  - `ci.yml` runs on the one-leg `[ubuntu-latest]` matrix, every job and artifact name keeping its `${{ matrix.os }}` form (was a native `windows-2025`, `macos-latest` and `ubuntu-latest` matrix); the `rmcp features` step, the tests, `perf` and the release build lose their per-OS wording.
  - CI integration names `ci.yml` and `nightly.yml` (was also `windows-mutants.yml`); the event-payload bullet drops the dispatch-only `windows-mutants.yml`.
  - zizmor `concurrency-limits` reads 2 low findings, one per workflow, `ci.yml` and `nightly.yml`, as measured at implement entry 22 of this chunk (was 3, read with zizmor 1.30.1 on 2026-10-04); the decline of 2026-09-24 stands.
  - Retired: the `windows-mutants.yml` rows (dispatch-only, its permissions, pins and no uploads, the 2026-10-10 dispatch) and the ban on `run --mutants` wiping any directory but the Windows `viola-mutants-scratch`.
  - The socket-directory negatives run on CI's `ubuntu-latest` leg (was `ubuntu-latest` and `macos-latest`).
  - Added: the Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin, which holds `src/conpty.rs` and `.github/workflows/windows-mutants.yml`.
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk deleted `windows-mutants.yml` and `scratch.rs` (workflows 3 → 2) and pushed the tag `multi-os-final` → `ecbe7ab3` to origin.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: client server verification is one Linux flow; the macOS carve-out and the Windows SQOS open retired
section: §Authentication & Authorization (body: the library row, IPC client-side server verification; authentication-authorization.md: the server verification, peer pid, peer euid, verdict, link / unlink and witness rows; the exiting-wrapper Windows pipe, macOS cases and macOS carve-out rows retired) · §Bootstrap phases (bootstrap-phases.md: the viola-channel client line) · §Error Handling (error-handling.md: hook, mcp) · §Input Validation (input-validation.md: the frame verbs) · §Security Anti-Patterns (body and security-anti-patterns.md: the frame-before-verification ban, the peer-euid seam ban) · §Threat Model Summary (threat-model-summary.md: the IPC vector)
change:
  - Every frame-writing client checks the socket directory, the peer euid (`peer_creds().euid()`, an unreadable euid refused) and the peer pid (`SO_PEERCRED`) with its sysinfo start time against the snapshot before the first frame (was the pid check on Linux and Windows, the euid on Linux and macOS, macOS the named carve-out).
  - `peer_euid_admitted(observed, own)` admits only `observed == Some(own)` (was `(observed, own, reported)`, `!reported || observed == Some(own)`); `SERVER_PID_REPORTED` and the verdict's macOS `Matches` arm are gone.
  - The frame ban reads the socket-directory, pid, start-time and euid checks, none skipped, no carve-out standing (was the macOS pid and start-time carve-out, owned by no route entry); `server-verify-failed` loses its per-OS gloss; the squatting residuals read closed (was open on macOS).
  - Retired: the Windows SQOS client open and identity check (`CreateFileW`, `Stream::try_from(OwnedHandle)`, `GetNamedPipeServerProcessId`), the exiting-wrapper Windows pipe CARRY, the macOS cases and the macOS carve-out rows; windows-sys and `SecurityDescriptor::deserialize` leave the library row.
  - The server-identity table runs against an impostor server with its file ungated (was compiled out on macOS).
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6), Linux-only; the chunk removed the SQOS open arms, `SERVER_PID_REPORTED` and `PEER_EUID_REPORTED` (their Linux value folded) and `server_verdict`'s `peer_pid_reported` parameter.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: child program resolution is a given path or a bare name on PATH; the .cmd/.bat refusal retired
section: §Input Validation (body: Child executable resolution; input-validation.md: child program resolution, the `revive --json` codes) · §Data Protection (body: the `.cmd`/`.bat` spawn ban retired; data-protection.md: verify's refusal codes) · §Error Handling (error-handling.md: `resolve_program`) · §Threat Model Summary (threat-model-summary.md: the `claude agents` spawn)
change:
  - `viola run` resolves a given path as given and a bare name on `PATH` and hands portable-pty an absolute path; a program not found is `ProgramNotFound` (under `viola revive --json` the document `internal-error`) (was the PATHEXT search, the `claude.cmd` → `claude.exe` sibling and the `BatchScriptChild` refusal).
  - `resolve_program` has one refusal, `AgentError::ProgramNotFound` (was also `AgentError::BatchScriptChild`); `list` and `mcp` resolve `claude` as a bare name on their PATH (was npm-shim → `claude.exe` resolution).
  - `revive --json`'s closed exit-1 codes 11 → 10 and verify's refusal codes 6 → 5, `batch-script-child` gone from both.
  - Retired: the ban on spawning a `.cmd`/`.bat` PTY child through portable-pty.
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); `batch-script-child` left the closed words with its emitter, the Viola overseer's technical answer of 2026-11-05 11:08Z (inputs#I2).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the statusline shell-out, the settings override and the relied set hold on every build
section: §Input Validation (body: Statusline shell-out; input-validation.md: the statusline row) · §Data Protection (body: Files; data-protection.md: the settings override, the trusted runs' accepted risk) · §Bootstrap phases (bootstrap-phases.md: the override line) · §Security Anti-Patterns (security-anti-patterns.md: the stamp rule) · §Threat Model Summary (threat-model-summary.md: verify's Run B)
change:
  - `hook statusline` runs the recorded command as `/bin/sh -c` with no OS limit (was Unix only, `shell_argv` returning `None` on Windows); its ledger rows are relied on every build (was Unix only), the Windows shell-out owed to "Windows-only live measurements" dropped.
  - The `settings.json` override is written when every character of the pinned path is plain (was on Unix only, none on Windows), the Windows half's cut note dropped; Run B gets its one `--settings` when the pinned path and the captures dir are shell-plain (was on a wrapped OS, Unix, Windows passing none).
  - `LedgerRow::relied()` is all twenty-four rows on every build, counter `/24`, a `20 pass  4 fail` stamp recording `cli_verified:false` (was `/21` and `17 pass  4 fail` on Windows).
  - Run B's status line is the probe capture its `--settings` names (was the user's own on Windows).
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk folded `WRAPPED_HERE` (the relied set all twenty-four rows) and `statusline::shell_argv` returns `[&str; 3]`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the ConPTY sideload and its vendored binaries retired
section: §Input Validation (body: the DLL search line retired; input-validation.md: the DLL search order + ConPTY companions row retired) · §Data Protection (body: Files, the companion re-hash ban; data-protection.md: the companions, the pinned copy's placement) · §Dependency Security (body: the vendored audit, ConPTY pins and CI step lines retired; dependency-security.md: the vendored ConPTY audit and pins rows retired) · §Bootstrap phases (body: dep-audit-tooling-install, dep-security-ci-gate; bootstrap-phases.md: the viola-state line) · §Error Handling (error-handling.md: `PinError::StrictModes`) · §Security Anti-Patterns (body: the DLL-search ban retired) · §Threat Model Summary (threat-model-summary.md: the filesystem vector)
change:
  - Retired: the DLL search restriction and the sideloaded `conpty.dll` / `OpenConsole.exe` under `bin/<version>-<hash>/conpty/` (write-if-absent, the `FILE_SHARE_READ` held re-hash, the `sideload_fallback` codes), the ban on loading a companion without its re-hash, the vendored-binaries audit through `scripts/conpty-vendor.sh`, the four pins in `src/conpty.rs`, the `ConPTY vendor verification` CI step and the DLL-search ban.
  - Only the pinned binary is re-hashed before use (was also the ConPTY companions); `replace_private_shared` keeps only the plugin files; `pin_companions` leaves the filesystem vector.
  - `PinError::StrictModes` and its `sideload_fallback` mapping leave Error Handling; `check_present` leaves the viola-state bootstrap line; dep-audit-tooling-install and dep-security-ci-gate drop the vendor script and its CI step.
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk deleted `src/conpty.rs`, `viola_pty::sideload`, `scripts/conpty-vendor.sh` and the whole `vendor/conpty/` tree, `pty_backend()` now always `openpty`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: one licence exception and one deny target triple
section: §Dependency Security (body: [licenses]; dependency-security.md: the arch bans, the absent-crate exception rule) · §Data Protection (data-protection.md: the SHA-256 crate's `[graph]` triple)
change:
  - `0BSD` is admitted by one `[[licenses.exceptions]]`, for `doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph, as measured at `cargo deny check licenses` reading `licenses ok` at this chunk (was two, `doctest-file` and `recvmsg`, measured at chunk 2026-09-25-security-prerequisites); the `allow` list and the per-exception Decisions Log rule stand.
  - Added: a `[[licenses.exceptions]]` entry naming a crate absent from the graph is dropped, never kept: cargo-deny warns `license-exception-not-encountered` for it, and it narrows nothing and widens nothing.
  - `deny-sync.toml`'s tokio ban runs for the one target triple `x86_64-unknown-linux-gnu` (was three); sha2 builds no `cc` on the one `[graph]` triple (was any of the three).
why: the Viola overseer's technical answer (inputs#I5 item 7): an exception naming a crate no longer in the graph is dropped; `recvmsg` is interprocess's Windows-only dependency, which CI warned `license-exception-not-encountered`; the targets narrowed with the founder's Linux-only rulings of 2026-11-04 (inputs#I1).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the R8 strip and the fixture scrub match names exactly; the registry reader retired
section: §Secret Management (body: Production; secret-management.md: the inherited credentials) · §Input Validation (body: the registry line retired; input-validation.md: the registry row retired, `claude_env_keep`) · §Data Protection (data-protection.md: Repository fixtures)
change:
  - Every inherited `CLAUDE*` name, matched exactly, is removed except a name in `config.json`'s `claude_env_keep`, the identity floor always stripped (was ASCII case-insensitive on Windows, the persistent set also the Windows registry `Environment` value names); `claude_env_keep` is the R8 pass-list (was the Unix one).
  - Retired: the Persistent-environment names (Windows registry) row and its body line (value names only, under `HKCU\Environment`).
  - The fixture scrub matches the home prefix exactly (was case-folded on Windows).
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk removed `src/run/env.rs`'s registry reader and the `CASE_INSENSITIVE` fold, the exact strip, `claude_env_keep` and the eleven-name identity floor staying.
kept: the identity floor stays the eleven names, the note of their Windows-host measurement and the Linux dev-host reading beside it standing.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: verify's stop and the colour decision lose their Windows arms
section: §Data Protection (data-protection.md: the probe captures, the interactive probe dirs) · §Input Validation (input-validation.md: the terminal colour facts, the PTY output witness)
change:
  - verify's stop is `SIGINT` / `SIGTERM` (was also Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`); the probe captures and dirs are left to be removed by hand after SIGKILL, a crash or a failed handler install (was also a Windows close, logoff or shutdown); `typed/statusline.json` loses its (Unix) qualifier.
  - The colour decision is none under `--json`, a passthrough role, `NO_COLOR`, `TERM` exactly `dumb` or a non-terminal stdout (was also a failed Windows VT enable); `src/human/style.rs` loses `mod vt`.
  - The feed-panic witness reads the child's bytes reaching the outer terminal with no OS limit (was on Unix, the Windows reading owed to "Windows-only live measurements").
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk removed the console-control stop arm and the Windows VT enable.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: advisories red on RUSTSEC-2026-0441, owned by Version done-check
section: §Dependency Security (dependency-security.md: two added rows)
change:
  - Added: `cargo deny check advisories` reads red on RUSTSEC-2026-0441 alone (`pin-utils 0.1.0` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1` from `viola-pty`), in CI's `supply-chain` job and locally, as measured at this chunk; its owner is the CARRY on "Version done-check", whose remedy moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`; no ignore is widened, the `[advisories]` ignore list keeping its one entry.
  - Added: the `portable-pty =0.8.1` pin carries no Windows reason: portable-pty 0.9.0's Windows garbage-read bug reaches no code viola builds, and "Version done-check" measures moving off the pin.
why: the Viola overseer's technical answer (inputs#I3, 2026-11-05 11:51Z) and the operator's own word (inputs#I4, 2026-11-05 12:02Z) approving the P5 card's two Beyond-the-plan rows as written: the CARRY on "Version done-check" owns the red, no ignore widened.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
