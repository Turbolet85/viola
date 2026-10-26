# security · cascade read

## Carried
- (.andromeda/security-plan-detail/authentication-authorization.md:15) The chunk's own clause: the Windows listener must carry the protected user + SYSTEM DACL (read back as exactly two allow ACEs, the logon SID not added), keep `accept_remote(false)`, the first-pipe-instance behaviour and `inheritable(false)`, each asserted on the Windows leg.
  > The listener gets an explicit protected DACL, `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`: the user SID + SYSTEM only. The SDDL string is converted to a descriptor with windows-sys (`Win32_Security_Authorization`), owned, and passed to `ListenerOptionsExt::security_descriptor`. Windows reads it back in its own canonical form, `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)` (GA maps to FA on a pipe), and may render the user SID as an alias (`LA` for the built-in Administrator), as measured in CI run 36313377307; the control is the protected flag plus exactly those two allow ACEs. The SID comes from `GetTokenInformation(TokenUser)` + `ConvertSidToStringSidW`. The logon SID is deliberately **not** added: a driver and a driven session in different logon sessions of the same user (an SSH or scheduled-task driver later) must still connect, and the boundary v1 promises is the OS user. Keep `accept_remote(false)` (the default, `PIPE_REJECT_REMOTE_CLIENTS`), the `FILE_FLAG_FIRST_PIPE_INSTANCE` behaviour (a squatted name gives exit 1 per Session Liveness) and `inheritable(false)`.
  (.andromeda/security-plan-detail/authentication-authorization.md:16)
  > `viola-channel` server, `#[cfg(windows)]`; interprocess 2.4.4; windows-sys 0.61.2
  (.andromeda/security-plan.md:108)
  > IPC access control (Windows) — the listener gets an explicit protected DACL granting the user SID and SYSTEM only; the logon SID is deliberately not added, since the boundary v1 promises is the OS user; remote clients stay rejected, the first-pipe-instance behaviour is kept and the handle is not inheritable.
  (.andromeda/security-plan.md:100)
  > Not used: interprocess `SecurityDescriptor::deserialize` (it needs a `widestring` dependency) and `ListenerOptionsExt::mode` (see IPC access control (Unix)).
  (.andromeda/security-plan.md:434)
  > NEVER create an interprocess Windows listener without `ListenerOptionsExt::security_descriptor`. The default DACL grants read to Everyone and anonymous.
  (.andromeda/security-plan.md:492)
  > NEVER enable interprocess `try_overwrite` (TOCTOU deletion of an attacker-swapped file).
  (.andromeda/security-plan.md:463)
  > NEVER make channel handles inheritable, and NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`.
  (.andromeda/security-plan.md:192)
  > **Handle inheritance:** portable-pty 0.8.1 calls `CreateProcessW` with `bInheritHandles = 0`, and interprocess listeners stay `inheritable(false)`, so channel handles never leak into the `claude` child.
- (.andromeda/security-plan-detail/authentication-authorization.md:12) Because any caller past the OS check gets the full surface, the DACL and the remote rejection the chunk asserts must be the whole Windows admission control, with no per-connection fallback added.
  > Capability-based, all or nothing. Any caller that passes the OS check (IPC) gets the full surface. No roles, because the only principal is the local OS user (Threat Model: Auth model).
  (.andromeda/security-plan.md:106)
  > RBAC / permissions — capability-based, all or nothing: any caller that passes the OS check gets the full surface; no roles, the only principal being the local OS user.
- (.andromeda/security-plan-detail/threat-model-summary.md:53) The pipe name sits in a machine-wide namespace, so the listener must admit only the same OS user and a name another process holds must refuse `viola run`'s start with exit 1, its refusal naming neither the holder nor the pipe.
  > On Windows, the named-pipe namespace is shared across the machine.
  (.andromeda/security-plan-detail/threat-model-summary.md:43)
  > One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows
  (.andromeda/security-plan-detail/threat-model-summary.md:45)
  > Whoever can connect can type prompts into the driven Claude session (`send`) and allow PermissionRequest dialogs (`answer` with `behavior:"allow"`). That amounts to running arbitrary tools, and so arbitrary code, as the user.
  (.andromeda/security-plan-detail/threat-model-summary.md:48)
  > v1 intends to trust every process of the same OS user and no other.
  (.andromeda/security-plan-detail/threat-model-summary.md:50)
  > The endpoint name is predictable: FNV-1a over `ViolaName` + home path, which is not secret (Conventions: Endpoint name).
  (.andromeda/security-plan-detail/threat-model-summary.md:55)
  > `run`'s exclusive bind means a squatted name blocks startup (exit 1).
  (.andromeda/security-plan.md:35)
  > **Vector:** Local IPC, the wrapper channel, the highest-impact surface: whoever can connect can type prompts into the driven session, allow PermissionRequest dialogs and read the whole conversation.
  (.andromeda/security-plan.md:507)
  > NEVER let a security refusal block the human. Refusals go only to automation (`send`/`answer`), and hooks fail open.
  (.andromeda/security-plan.md:408)
  > The `Display` impls on `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages. Variant fields that hold paths or payloads are excluded from `Display`.
  (.andromeda/security-plan.md:484)
  > NEVER expose absolute paths in any external error.
- (.andromeda/security-plan-detail/threat-model-summary.md:157) The chunk's admission assertions must close on the native `windows-2025` leg of `ci.yml`, and a DACL-effect reading no runner can take is owed to the Windows-only live measurement, never claimed as measured.
  > GitHub Actions `ci.yml`, triggered on push and PR, on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`
  (.andromeda/security-plan.md:75)
  > **CI/CD:** GitHub Actions `ci.yml` on push and PR over a native `windows-2025`, `macos-latest` and `ubuntu-latest` matrix
- (.andromeda/security-plan-detail/bootstrap-phases.md:14) This route entry is the "Epoch 6 admission entry" the bootstrap line names: the chunk closes `viola-channel`'s Windows listener hardening, while the `peer_creds` decision and the Unix directory verification stay with their own route entries.
  > `viola-channel`: per-OS listener hardening (Windows SDDL converted by windows-sys into interprocess `security_descriptor` + windows-sys SID lookup; Unix 0700 per-user socket directory + chmod 0600 after the bind + `peer_creds` euid check). The SDDL, the per-user directory and the 0600 socket landed with "Wrapper channel", ahead of Epoch 6's admission entry; the `peer_creds` decision and the directory verification stay with their route entries.
  (.andromeda/security-plan.md:325)
  > **auth-scaffolding-baseline:** narrowed to the local boundary, no auth library: per-OS listener hardening and client server verification in `viola-channel`,
- (.andromeda/security-plan-detail/authentication-authorization.md:24) `tests/security_negatives_channel.rs`, reached whole and its coordinate re-verified by the chunk, holds the client SQOS negative: the Windows client open must stay identification-level with no fallback to interprocess's default connect, and the verification itself stays the next entry's.
  > The client never falls back to interprocess's default connect, which opens without SQOS and lets the server impersonate (measured: `SecurityImpersonation`) (§ Authentication).
  (.andromeda/security-plan-detail/authentication-authorization.md:27)
  > **On mismatch:** CLI/MCP report `instance-unreachable`, and `hook` fails open (exit 0, no body, a note in `diagnostics/`). **Collision check:** `run` uses the same verification.
  (.andromeda/security-plan.md:441)
  > NEVER open a Windows client pipe without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION` (the default `connect_without_waiting` lets a squatter with SeImpersonatePrivilege impersonate the client).
  (.andromeda/security-plan.md:114)
  > the dated interim gaps stand until their two route entries land.
- (.andromeda/security-plan-detail/authentication-authorization.md:18) `endpoint.rs`, reached whole, holds `socket_dir` and `endpoint_path`'s Unix arm, which must keep the per-user 0700 directory order and the `<dir>/viola-<h12>.sock` endpoint while the Windows arm is touched.
  > **Primary control:** the socket lives in a per-user directory with mode 0700: `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, and `/tmp/viola-<uid>/` only as a fallback. The endpoint becomes `<dir>/viola-<h12>.sock`.
  (.andromeda/security-plan.md:110)
  > IPC access control (Unix) — the socket lives in a per-user 0700 directory, checked before bind and before any client connect (owner, mode, not a symlink), and is chmod 0600 right after the bind; a failed check refuses `run`, reports `instance-unreachable` to clients and fails `hook` open; `try_overwrite` stays off and the Linux abstract namespace is never used.
  (.andromeda/security-plan.md:435)
  > NEVER place a Unix socket directly in `/tmp` or at the `$TMPDIR` root (`/tmp/viola-<h12>.sock`).
  (.andromeda/security-plan.md:437)
  > NEVER use Linux abstract-namespace sockets (no filesystem permissions).
- (.andromeda/security-plan-detail/threat-model-summary.md:106) The endpoint name hashes the home that `--home` chooses, and the reached tests read the `--home` and default-home statusline sources, so the home choice must keep deciding which endpoint and state are used.
  > `--home <dir>` and `VIOLA_DIR` choose which state tree, snapshot, `endpoint`, `statusline_command` and ledger stamps are used (Cross-cutting Patterns: Config management).
- (.andromeda/security-plan-detail/threat-model-summary.md:20) `tests/cli_instance_state.rs`, reached whole, asserts the snapshot's `endpoint` shape and `pinned_bin`, and a squatted-name refusal must write no snapshot at all.
  > `snapshot.json` also stores `endpoint`, the socket path clients connect to, and `pinned_bin`.
  (.andromeda/security-plan-detail/threat-model-summary.md:17)
  > `instances/<ViolaName>/settings.json` is the per-session statusline override. It holds `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, never the user's command.
  (.andromeda/security-plan-detail/threat-model-summary.md:18)
  > `snapshot.json` stores `statusline_command`, the user-written shell string that `hook statusline` reads through `VIOLA_DIR` and runs through a shell. This is the only shell-out in viola.
  (.andromeda/security-plan.md:514)
  > NEVER let a process other than the instance's own wrapper write `instances/<name>/snapshot.json`.
  (.andromeda/security-plan.md:509)
  > It must be the absolute pinned `bin/<version>-<hash>/viola(.exe)` path, or the absolute path of the exe that ran `plugin install`.
- (.andromeda/security-plan-detail/threat-model-summary.md:32) `tests/cli_instance_state.rs`, reached whole, asserts the snapshot's `pid`, `child_pid` and `started_at` written before the spawn, and the recorded `cwd` must reach no log line, error body or stdout.
  > Snapshots hold `pid`, `child_pid`, `agent_session_id`, `started_at` and the optional `cwd` (the start's spawn directory: an absolute host path that can carry the OS username; it lives in the 0600 snapshot only, on no log line, error body or stdout, and is omitted when absent).
- (.andromeda/security-plan-detail/threat-model-summary.md:124) `tests/cli_instance_state.rs`, reached whole, asserts the tampered pinned copy refusal and the plugin folder rewrite, so the pinned binary's content-hash check and the rewrite at every start must keep holding.
  > Nothing checks integrity beyond the content hash in the folder name.
  (.andromeda/security-plan-detail/threat-model-summary.md:123)
  > The pinned binary and plugin folders are never deleted in v1.
  (.andromeda/security-plan.md:461)
  > NEVER reuse an existing `bin/<version>-<hash>/viola(.exe)` without re-hashing it (SHA-256, truncated) against `<hash>`, NEVER derive `<hash>` from a non-cryptographic hash (FNV, `DefaultHasher`), and NEVER reuse existing `plugin/` files without rewriting them.
- (.andromeda/security-plan-detail/authentication-authorization.md:33) `tests/cli_instance_state.rs`, reached whole, asserts the settings override at 0600 and the pinned copy under the home, so files the reached tests read must stay owner-only with the pinned `viola` the one 0700 file.
  > home (and any `--home`) is created 0700, and every file is created with `OpenOptionsExt::mode(0o600)`, except the pinned `bin/<version>-<hash>/viola`, which must be executable and is created 0700 (never group- or world-writable; at 0600 every hook and MCP exec would fail).
  (.andromeda/security-plan.md:459)
  > NEVER create `~/.viola/`, `--home` or any file under it with the default umask.
- (.andromeda/security-plan-detail/input-validation.md:52) The reached tests re-assert the live and gone outcomes of `run`'s collision check, which must keep reading the snapshot through `read_snapshot` ahead of the bind whose failure `refuse_squatted` reports.
  > The five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot`, so their handling of an absent, unreadable or newer snapshot is unchanged.
  (.andromeda/security-plan-detail/input-validation.md:53)
  > Every failure is no reading and writes no line.
  (.andromeda/security-plan.md:154)
  > every failed `budget.json` read is no reading.
- (.andromeda/security-plan-detail/input-validation.md:35) The edited budget test's cases sit on the 90 / 85 threshold defaults, so its log-capture fix must leave the thresholds and the gate's behaviour as they stand.
  > The budget thresholds are the two top-level keys `five_hour` and `seven_day` (defaults 90 and 85)
  (.andromeda/security-plan.md:144)
  > no value switches the budget gate off or makes the dialog hold zero or unbounded
- (.andromeda/security-plan-detail/threat-model-summary.md:150) The chunk leans on interprocess's defaults and may add a windows-sys feature or dev-dependency, all under `cargo deny check`, whose red on RUSTSEC-2026-0386 its CI runs must read red with no ignore added.
  > **Trust boundary:** `cargo deny check` over `deny.toml` (advisories, licences, sources, C-crate / telemetry-crate / feature bans)
  (.andromeda/security-plan-detail/threat-model-summary.md:146)
  > Cargo dependencies. Arch notes interprocess has a single main maintainer and rmcp releases nearly weekly with its minor version pinned.
  (.andromeda/security-plan.md:250)
  > This is the only expected ignore; re-check it whenever the portable-pty pin moves.
- (.andromeda/security-plan-detail/dependency-security.md:14) A windows-sys feature or dev-dependency added to `viola-channel` for the read-back must bring no C-building crate and keep its normal graph tokio-free under `scripts/channel-tokio-free.sh`.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.1`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`).
  (.andromeda/security-plan.md:261)
  > `[workspace.dependencies]` pins every third-party version of the root graph (portable-pty `=0.8.1`)
- (.andromeda/security-plan-detail/security-anti-patterns.md:37) A test that measures the DACL effect or the remote rejection must add no config key, `VIOLA_*` variable or flag that switches the DACL or the remote rejection off.
  > NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan: the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check or `validate_paste_text`.
  (.andromeda/security-plan.md:516)
  > NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan (the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check, `validate_paste_text`): environment variables are not a configuration channel.
  (.andromeda/security-plan.md:506)
  > NEVER add a listener (TCP port, WebSocket, `http` hooks, MCP HTTP transport) or a new channel method without a Security Decisions Log entry mapping it to this plan's IPC controls.
- (.andromeda/security-plan-detail/secret-management.md:8) The new tests must read no new environment variable: the only reads outside `VIOLA_*` stay the two test seams and the colour facts.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds
  (.andromeda/security-plan.md:369)
  > **Production:** no stored secrets in v1, and environment variables are not a configuration channel;
- (.andromeda/security-plan-detail/secret-management.md:15) The second listener and any test principal built to measure denial must live inside the test's own home, never under a product home with relaxed modes.
  > Tests use their own `--home` directory.
  (.andromeda/security-plan.md:500)
  > NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created;

## Bindings
- **Authentication & Authorization (Windows IPC) ↔ Threat Model (boundary-ipc)**: the protected DACL, `accept_remote(false)` and first-pipe-instance (.andromeda/security-plan-detail/authentication-authorization.md:15) are the enforcement of "every process of the same OS user and no other" in a machine-wide pipe namespace, and the squat they produce is the exit-1 start refusal (.andromeda/security-plan-detail/threat-model-summary.md:55).
- **Authentication & Authorization (RBAC) ↔ Authentication & Authorization (Windows IPC)**: all-or-nothing access (.andromeda/security-plan-detail/authentication-authorization.md:12) makes the listener's DACL and remote rejection the whole admission control (.andromeda/security-plan-detail/authentication-authorization.md:15).
- **Bootstrap phases ↔ Authentication & Authorization (Windows and Unix IPC)**: this entry closes the Windows listener hardening (.andromeda/security-plan-detail/bootstrap-phases.md:14) while the Unix directory verification of (.andromeda/security-plan-detail/authentication-authorization.md:18) and the client verification of (.andromeda/security-plan-detail/authentication-authorization.md:24) stay with their own entries.
- **Security Anti-Patterns ↔ Data Protection (handle inheritance)**: the inheritable-handle ban (.andromeda/security-plan.md:463) and the `inheritable(false)` listener (.andromeda/security-plan.md:192) bind the both-ends inheritance test the chunk re-verifies.
- **Security Anti-Patterns ↔ Secret Management**: no switch-off of the DACL (.andromeda/security-plan-detail/security-anti-patterns.md:37) and no new environment read (.andromeda/security-plan-detail/secret-management.md:8) together bound what a DACL-effect or remote-reject test may add.
- **Threat Model (supply chain) ↔ Dependency Security**: any windows-sys feature or dev-dependency rides `cargo deny check` (.andromeda/security-plan-detail/threat-model-summary.md:150) and the C-crate and tokio bans (.andromeda/security-plan-detail/dependency-security.md:14), and the red advisory is read with the one expected ignore unchanged (.andromeda/security-plan.md:250).
- **Threat Model (CI) ↔ Authentication & Authorization (Windows IPC)**: the Windows admission assertions (.andromeda/security-plan-detail/authentication-authorization.md:15) are measured on the `windows-2025` leg (.andromeda/security-plan-detail/threat-model-summary.md:157).
- **Threat Model (config data) ↔ Error Handling**: the squat refusal writes no snapshot (.andromeda/security-plan-detail/threat-model-summary.md:20) and any bind or first-instance failure it surfaces through `ChannelError` keeps a fixed message with no pipe path (.andromeda/security-plan.md:408).
- **Input Validation (config) ↔ Input Validation (own state files)**: the budget test's thresholds (.andromeda/security-plan-detail/input-validation.md:35) and the gate's fail-open reading (.andromeda/security-plan-detail/input-validation.md:53) stay unchanged by the log-capture fix.
