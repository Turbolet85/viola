# security extract

## Relevance
relevant — the chunk is the Windows half of the IPC endpoint access control the plan elevates (local privilege boundary on the wrapper channel).

## Constraints
- The wrapper's Windows listener must carry the explicit protected DACL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` (user SID from `GetTokenInformation(TokenUser)` + `ConvertSidToStringSidW`, passed through `ListenerOptionsExt::security_descriptor`); the control is the protected flag plus exactly those two allow ACEs, read back canonically as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)` with the user SID possibly rendered as an alias (`LA`); the logon SID is deliberately NOT added (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- `accept_remote(false)` (`PIPE_REJECT_REMOTE_CLIENTS`) must be kept so a network pipe client is rejected; whether the code today asserts this beyond a doc comment is research's question (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- The `FILE_FLAG_FIRST_PIPE_INSTANCE` behaviour must be kept so a name another process already holds refuses `viola run` at start with exit 1 (Session Liveness); `try_overwrite` stays off (per security-plan §Authentication & Authorization, IPC access control (Windows); per security-plan §Threat Model Summary, the IPC vector: `run`'s exclusive bind means a squatted name blocks startup).
- Channel handles must be non-inheritable on both ends (`inheritable(false)`); no portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE` (per security-plan §Security Anti-Patterns, Data Protection).
- The trust boundary is every process of the same OS user and no other; the Windows named-pipe namespace is machine-shared and the endpoint name (FNV-1a over `ViolaName` + home) is predictable, not secret — the DACL, remote rejection and first-instance bind are what hold the boundary (per security-plan §Threat Model Summary, Local IPC vector).
- No `config.json` key, `VIOLA_*` env var or CLI flag may switch off the IPC DACL or any other endpoint control; no new environment read or test seam without its own Decisions Log entry (per security-plan §Security Anti-Patterns, Universal).
- Any `windows-sys` feature added to read a pipe's mode or state back must keep the root graph inside `cargo deny check` (C-building crate ban, crates.io-only sources, the licence allow list); the current `supply-chain` red (RUSTSEC-2026-0386) is closed by a lock bump on its own route entry, never by widening the advisory ignore list (per security-plan §Dependency Security).

## Patterns to follow
- SDDL → owned descriptor through windows-sys `Win32_Security_Authorization`, handed to `ListenerOptionsExt::security_descriptor`; not interprocess `SecurityDescriptor::deserialize` (needs a `widestring` dependency) (per security-plan §Authentication & Authorization, Library).
- Read the control back from the live object rather than trusting the configured text: the plan's DACL control is stated as the descriptor Windows returns (protected flag + exactly two allow ACEs, CI-measured in run 36313377307) — the same read-back form fits the remote-reject pipe mode and the first-instance refusal (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- Windows-only controls live under `#[cfg(windows)]` in the `viola-channel` server; interprocess 2.4.4 and windows-sys 0.61.2 are the pinned stack (per security-plan §Authentication & Authorization, IPC access control (Windows) stack reference).
- A security refusal reaches automation only: the squatted-name start refusal is `run`'s exit 1, never a block on the human's keystrokes (per security-plan §Security Anti-Patterns, Universal).

## Anti-patterns to avoid
- NEVER create an interprocess Windows listener without `ListenerOptionsExt::security_descriptor` — the default DACL grants read to Everyone and anonymous (per security-plan §Security Anti-Patterns, Authentication).
- NEVER enable interprocess `try_overwrite`, and NEVER make channel handles inheritable (per security-plan §Security Anti-Patterns, Code Patterns; per security-plan §Security Anti-Patterns, Data Protection).
- NEVER add a listener (TCP, WebSocket, HTTP) or a new channel method without a Security Decisions Log entry mapping it to the plan's IPC controls; a widening found on the way halts for the founder's live word (per security-plan §Security Anti-Patterns, Universal).

## Contract bindings
- security ↔ tests: the Windows-only assertions (remote-client rejection read back, first-instance as a listener property, DACL effect against an outside principal) bind to the `windows-2025` CI leg; a measurement no runner can make without a second interactive user stays a Windows-only live measurement, never claimed as asserted (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- security ↔ architecture: the squatted-name refusal's exit 1 and refusal word are Session Liveness / Conventions contracts the chunk must not change (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- security ↔ the next entry: client-side SQOS open and server pid/start-time verification, with their dated interim gaps, stay owned by "Server verification before any frame" (per security-plan §Authentication & Authorization, IPC client-side server verification).

## Acceptance criteria contributions
- (security) A Windows test reads the bound listener's pipe back and finds remote clients rejected (`PIPE_REJECT_REMOTE_CLIENTS`), red if `accept_remote(false)` is dropped (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- (security) A Windows test binds a second listener on a held endpoint name and it fails (first-pipe-instance as a listener property), alongside the existing CLI-level `squatted-name` exit-1 refusal, unchanged (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- (security) The bound listener's DACL still reads back protected with exactly the user + SYSTEM allow ACEs and both pipe ends stay non-inheritable on the Windows CI leg (per security-plan §Authentication & Authorization, IPC access control (Windows)).
- (security) `cargo deny check` passes bans, licences and sources with any added windows-sys feature, and the advisories ignore list is unchanged (per security-plan §Dependency Security).
