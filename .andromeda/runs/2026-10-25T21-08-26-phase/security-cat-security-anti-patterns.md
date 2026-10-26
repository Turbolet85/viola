# security · Security Anti-Patterns — category read

## Carried
- (.andromeda/security-plan.md:434) The Windows listener this chunk asserts must be created with an explicit security descriptor; no listener the chunk builds or keeps may fall back to interprocess's default DACL.
  > - NEVER create an interprocess Windows listener without `ListenerOptionsExt::security_descriptor`. The default DACL grants read to Everyone and anonymous.
- (.andromeda/security-plan.md:108) The broad line the Authentication bans detail, and this chunk's acceptance: user SID and SYSTEM only, no logon SID, remote clients rejected, first-pipe-instance kept, handle non-inheritable.
  > IPC access control (Windows) — the listener gets an explicit protected DACL granting the user SID and SYSTEM only; the logon SID is deliberately not added, since the boundary v1 promises is the OS user; remote clients stay rejected, the first-pipe-instance behaviour is kept and the handle is not inheritable.
- (.andromeda/security-plan.md:463) The non-inheritable property the chunk re-verifies must hold for every channel handle, and no PTY swap may reintroduce handle inheritance.
  > - NEVER make channel handles inheritable, and NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`.
- (.andromeda/security-plan.md:192) The Data Protection line the inheritance ban rests on: listeners stay `inheritable(false)` so channel handles never reach the `claude` child.
  > - **Handle inheritance:** portable-pty 0.8.1 calls `CreateProcessW` with `bInheritHandles = 0`, and interprocess listeners stay `inheritable(false)`, so channel handles never leak into the `claude` child. Any portable-pty swap (portable-pty-psmux 0.9.7 is the named candidate) must keep this property.
- (.andromeda/security-plan.md:100) Any read-back of the pipe's descriptor or mode the chunk adds must use the named library surface, never interprocess `SecurityDescriptor::deserialize` (no `widestring` dependency).
  > Not used: interprocess `SecurityDescriptor::deserialize` (it needs a `widestring` dependency) and `ListenerOptionsExt::mode` (see IPC access control (Unix)).
- (.andromeda/security-plan.md:492) The listener options the chunk asserts keep `try_overwrite` off, so a held (squatted) name refuses the start instead of being replaced.
  > - NEVER enable interprocess `try_overwrite` (TOCTOU deletion of an attacker-swapped file).
- (.andromeda/security-plan.md:441) Any client pipe the chunk's product code opens keeps the SQOS identification-level open; a test client opening the pipe to measure remote rejection or the DACL's effect never changes the product client's open.
  > - NEVER open a Windows client pipe without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION` (the default `connect_without_waiting` lets a squatter with SeImpersonatePrivilege impersonate the client).
- (.andromeda/security-plan-detail/security-anti-patterns.md:8) Server verification before any frame is the next entry, not this chunk: the dated exceptions stay as they stand, and nothing this chunk adds may borrow them.
  > NEVER write any channel frame (`send`, `answer`, `hook.dialog`, `wait`/`last`, `pause`/`release`, and `hook.event` notifications, which carry prompt and reply text) before verifying the server's pid (Windows `server_process_id()`) or euid + pid (Linux/FreeBSD `peer_creds()`) plus sysinfo start time against `snapshot.json`
- (.andromeda/security-plan.md:439) The borrow ban on the dated gaps, which this chunk neither closes nor extends.
  > - NEVER write any channel frame before verifying the server against a `snapshot.json` that passed the strict-modes check in the same process (the euid check never skipped), except the dated gaps (`hook.event`; the CLI `send`, `wait` / `last`, `answer`, `pause` / `release` frames; `hook.dialog`; `viola mcp`'s tool frames) until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" land; no other frame or process may borrow them.
- (.andromeda/security-plan-detail/security-anti-patterns.md:37) No config key, `VIOLA_*` variable, CLI flag or new test seam may switch off the IPC DACL, remote rejection or first-instance behaviour the chunk asserts.
  > NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan: the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check or `validate_paste_text`.
- (.andromeda/security-plan-detail/security-anti-patterns.md:37) Measuring remote rejection or the DACL's effect on the Windows CI leg must not add a seam or environment read without its own Decisions Log entry.
  > NEVER add another such seam or environment read, let these reach a release build, or let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.
- (.andromeda/security-plan.md:506) The chunk adds no listener and no channel method; a second listener built only to prove first-instance behaviour stays test-side.
  > - NEVER add a listener (TCP port, WebSocket, `http` hooks, MCP HTTP transport) or a new channel method without a Security Decisions Log entry mapping it to this plan's IPC controls.
- (.andromeda/security-plan.md:484) If `src/cmd/run.rs`'s squatted-name start refusal is touched, it still exposes no absolute path (the pipe name or home) in any external error.
  > - NEVER expose absolute paths in any external error.

## Bindings
- **Windows listener DACL ban ↔ security-plan §Authentication & Authorization (IPC access control (Windows)) and its detail file `authentication-authorization.md`**: the ban states the floor, the body line states the target (user SID and SYSTEM, remote rejected, first-instance, non-inheritable) that the chunk asserts on the Windows CI leg (.andromeda/security-plan.md:434) (.andromeda/security-plan.md:108)
- **Handle-inheritance ban ↔ security-plan §Data Protection (Handle inheritance)**: both name `inheritable(false)` on the listener; the chunk's non-inheritable assertion serves both (.andromeda/security-plan.md:463) (.andromeda/security-plan.md:192)
- **Server-verification ban and its dated gaps ↔ working-route entry "Server verification before any frame"**: the chunk's Boundaries leave the gaps to that entry; the borrow ban holds meanwhile (.andromeda/security-plan-detail/security-anti-patterns.md:8) (.andromeda/security-plan.md:439)
- **No-switch-off ban ↔ security-plan §Input Validation (test seams) and architecture.md Cross-cutting Patterns (Config management)**: no new seam for the Windows measurement without a Decisions Log entry (.andromeda/security-plan-detail/security-anti-patterns.md:37)
- **Listener ban ↔ security-plan Security Decisions Log**: a new listener or method needs a Log entry; the chunk adds none (.andromeda/security-plan.md:506)
- **External-error path ban ↔ security-plan §Error Handling and architecture.md §Conventions (refusals, exit codes)**: `squatted-name`, exit 1 stand unchanged and path-free (.andromeda/security-plan.md:484)
