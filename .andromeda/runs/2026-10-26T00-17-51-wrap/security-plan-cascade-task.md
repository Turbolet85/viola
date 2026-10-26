# Task — security-plan · 2026-10-25-windows-endpoint-admission

## The session
Added five Windows-only integration tests in tests/security_negatives_channel.rs that check the viola pipe listener's admission controls, all green on CI's test (windows-2025) leg (ci#40247500584).
Remote-client rejection is now asserted by a pipe-type read-back (NtQueryInformationFile, FilePipeLocalInformation, NamedPipeType bit 0x2) on a client end of a viola_channel::Server::bind listener. Two controls on test-owned pipes, one accept-remote and one reject-remote, tell the bit apart.
The DACL's effect is now asserted by an anonymous-token GENERIC_READ open of the viola listener, denied with ERROR_ACCESS_DENIED, beside a control where the same open of a default-descriptor pipe succeeds.
Added a root Windows dev-dependency table: windows-sys with the features Wdk_Storage_FileSystem and Win32_System_IO. Cargo.lock did not move, no dependency was added or bumped, and no product file, wire contract, exit code, refusal word or output line changed.
Routed the root bin's nine scoped tracing captures through a test-only helper, test_support::with_scoped_default, which pins a second, filterless dispatcher. The existing DACL read-back, non-inheritance, first-instance and squatted-name tests passed again unedited.
CI's one red is supply-chain: RUSTSEC-2026-0386 (unsound) against the unchanged tokio 1.53.1. A new route entry, "tokio RUSTSEC-2026-0386 lock bump" (tokio to >=1.53.3, no ignore widened), owns it, approved by the founder.

## Reached files
- tests/security_negatives_channel.rs
- Cargo.toml
- src/main.rs
- src/run/send/test_support.rs
- src/run/dialog.rs
- src/run/budget.rs
- src/run/gate.rs
- src/obs.rs
- src/cmd/run.rs
- src/cmd/hook/statusline.rs

## Touches
- Authentication & Authorization, IPC access control (Windows): the clause that remote clients stay rejected (accept_remote(false), the default PIPE_REJECT_REMOTE_CLIENTS) is unchanged, but it is now asserted on the windows-2025 CI leg and gains its witness test.
- Authentication & Authorization, IPC access control (Windows): the protected user-SID + SYSTEM DACL now has a witness for its effect (an anonymous open denied), not only for its text read-back.
- Authentication & Authorization, IPC access control (Windows): first-instance, non-inheritable handles and the squatted-name refusal were re-read as passing on windows-2025 and are unmoved.
- Authentication & Authorization, Library row: it lists windows-sys 0.61.2 with the feature Win32_Security_Authorization. The root package now also declares Wdk_Storage_FileSystem and Win32_System_IO, on a Windows dev-dependency only, as test instruments.
- Security Anti-Patterns, Authentication: the ban on a Windows listener without security_descriptor rests on the premise that the default DACL grants read to Everyone and anonymous (security-plan.md:434). That premise is now measured true on windows-2025 by the default-descriptor control.
- Threat Model Summary, the boundary-ipc group: any text saying Windows listener admission (remote rejection, DACL denial) cannot be exercised on hosted runners, or is owed to "Windows-only live measurements", is now stale for those two controls.
- Threat Model Summary, the boundary-ipc group: any statement of the per-OS enforcement as decided-but-unasserted on Windows now reads as asserted on CI for the listener side.
- Dependency Security, Pinning: Cargo.lock is unchanged and windows-sys stays at =0.61.2. Both new dev-dependency features already reached the Windows graph by unification (mio 1.2.4, interprocess 2.4.4, socket2 0.6.5), and under resolver 3 no release build gets them.
- Dependency Security, CI integration: cargo deny check bans licenses sources and scripts/channel-tokio-free.sh read green with the new dev-dependency table. No ignore, skip or allow was added.
- Dependency Security, the advisories gate: every run since the last wrap fails supply-chain alone on RUSTSEC-2026-0386 (tokio 1.53.1, unsound = "all"). The fix is the route entry "tokio RUSTSEC-2026-0386 lock bump", not a deny.toml ignore. RUSTSEC-2017-0008 stays the only expected ignore.
- Dependency Security, Critical CVE response SLA: the tokio advisory is one against a crate viola enables, so the bump path applies, not the disabled-feature ignore path.
- Dependency Security, the deps-supply-chain group: the tokio lock bump does not change the tokio bans for the sync crates (deny-sync.toml, channel-tokio-free.sh). Those bans read green.
- Data Protection, In transit, Handle inheritance: the listener's non-inheritable handles were re-asserted on windows-2025 by pipe_handles_on_both_ends_are_not_inheritable. The claim is unmoved.
- Bootstrap phases, auth-scaffolding-baseline (per-OS listener hardening in viola-channel): the Windows hardening now has CI witnesses for remote rejection and the DACL's effect.
- Bootstrap phases, error-sanitization-wire and logging-redaction-wire: every scoped capture still records lines emitted through obs_event!. The new helper only changes which dispatcher records them in tests, so no logged field and no redaction moved.

## New facts
- F1 · authentication-authorization · The Windows listener's remote-client rejection is asserted on CI test (windows-2025) by security_negative_viola_listener_rejects_remote_clients. It reads the pipe type on a client end of the viola listener, with controls security_negative_control_accept_remote_pipe_reads_the_bit_clear and security_negative_control_reject_remote_pipe_reads_the_bit_set.
- F2 · authentication-authorization · The Windows listener DACL's denial is asserted on CI test (windows-2025) by security_negative_viola_listener_denies_an_anonymous_open: an ImpersonateAnonymousToken GENERIC_READ open fails with ERROR_ACCESS_DENIED (5). Its control is security_negative_control_default_pipe_admits_an_anonymous_read: a null-security-attributes pipe admits the same open.
- F3 · threat-model-summary · For the Windows pipe, remote-client rejection and the DACL's effect are witnessed on hosted CI, so nothing about those two is owed to "Windows-only live measurements". The cross-user parts stay unexercised: a peer with a different euid and a second OS account.
- F4 · dependency-security · The root package carries a test-only `[target.'cfg(windows)'.dev-dependencies]` windows-sys (workspace pin =0.61.2) with features Wdk_Storage_FileSystem and Win32_System_IO. Cargo.lock is unchanged, and no release build carries the features (resolver 3).
- F5 · dependency-security · CI's supply-chain job is red on RUSTSEC-2026-0386 (unsound) against tokio 1.53.1. The founder-approved route entry "tokio RUSTSEC-2026-0386 lock bump" owns it: tokio to >=1.53.3, no ignore-list widening.

## Untouched
- IPC access control (Unix) and the server-side peer euid check: no Unix listener or socket-directory code changed. The rule lives in viola-channel's Unix listener.
- IPC client-side server verification and the SQOS identification-level open, with their dated interim gaps (hook.event, CLI send / wait / last / answer / pause / release, hook.dialog, viola mcp's tools): they still stand until "Server verification before any frame" and "Home and code-bearing file integrity" land.
- The `~/.viola/` home strict-modes check, replace_private and the 0700 / 0600 modes in viola-state: unchanged.
- Paste text and dialog free-text validation (validate_paste_text, control-character, empty-text) in viola-core: unchanged.
- MCP transport (rmcp features exactly server + transport-io, scripts/rmcp-features.sh, the deny.toml rmcp feature bans): unchanged.
- The vendored ConPTY audit (scripts/conpty-vendor.sh, the four pins in src/conpty.rs) and the fuzz workspace's lockfile exemption: unchanged.
- Repository fixtures, the verify --record scrub and viola-mcp-tap's fixture refusal: no fixture recorded.
- Secret Management (the R8 strip, the CI artifact canary scan, pre-push's env -i launcher): unchanged.
- Error Handling's fixed Display messages, channel -32603 and MCP isError codes in the library crates: unchanged.
- revive's session-id argv, its preflight and the claude agents --json read: unchanged.
- The deny.toml licence allow list and the 0BSD per-crate exceptions: no crate added, so no licence row moved.
