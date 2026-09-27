
## 2026-09-27-wrapper-channel — listener hardening as landed: SDDL through windows-sys, chmod 0600 after the bind
**Section:** §Authentication & Authorization (Library: IPC; IPC access control (Windows); IPC access control (Unix)) · §Bootstrap phases (auth-scaffolding-baseline: amendment 2, `viola-channel` listener hardening) · §Security Anti-Patterns → Authentication
**Change:**
- Windows: the protected DACL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` is converted by windows-sys (`Win32_Security_Authorization`) into `ListenerOptionsExt::security_descriptor`; was `SecurityDescriptor::deserialize` (not used: it needs `widestring`). Windows reads it back canonical as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)` with the SID possibly an alias (`LA`); the control is the protected flag plus exactly the two allow ACEs.
- Unix secondary control: was `ListenerOptionsExt::mode(0o600)` "where supported, `Unsupported` on macOS ignored"; now chmod 0600 right after the bind on every Unix OS (`mode` fails the bind on macOS, and ignoring it left the socket at the umask mode). The anti-pattern bans `ListenerOptionsExt::mode` and keeps "never the only control".
- Bootstrap: arch amendment 2 is folded (arch IPC endpoints); the SDDL, the per-user dir and the 0600 socket landed with "Wrapper channel", ahead of Epoch 6's admission entry; the `peer_creds` decision and the directory verification stay with their route entries.
**Why:** the report disproved the `mode(0o600)` premise and the GA read-back literal (CI run 36313377307). Weighed against "Boundary widening": not that class — nothing new crosses, and the socket now reaches 0600 on every Unix OS.
**Kept:** the Decisions Log IPC entry and the Threat Model Summary (a verbatim copy) keep their wording as history.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/
