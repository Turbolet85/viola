# security · Data Protection — category read

## Carried
- (.andromeda/security-plan.md:189) The chunk's Windows pipe DACL and remote-client rejection are the only confidentiality control on the channel in transit, so the listener must admit no principal beyond the ACL's two and no network client, with no transport encryption standing in for them.
  > **TLS:** N/A. The IPC channel is local named pipes or Unix sockets, and MCP is stdio. Confidentiality in transit depends on the IPC ACLs (Authentication & Authorization).
- (.andromeda/security-plan.md:192) The non-inheritable handle assertion this chunk re-verifies must hold on the listener side (`inheritable(false)`) and must not be loosened by any change to the listener options; the child's `bInheritHandles = 0` property must stay intact.
  > **Handle inheritance:** portable-pty 0.8.1 calls `CreateProcessW` with `bInheritHandles = 0`, and interprocess listeners stay `inheritable(false)`, so channel handles never leak into the `claude` child. Any portable-pty swap (portable-pty-psmux 0.9.7 is the named candidate) must keep this property.
- (.andromeda/security-plan.md:463) The ban the chunk's handle-inheritance test (`pipe_handles_on_both_ends_are_not_inheritable`) guards: no channel handle, server or client end, may become inheritable, and no PTY replacement that inherits handles may be adopted.
  > NEVER make channel handles inheritable, and NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`.

## Bindings
- **In-transit confidentiality ↔ Authentication & Authorization, IPC access control (Windows)**: Data Protection rests confidentiality on the IPC ACLs (.andromeda/security-plan.md:189); the ACL itself (user SID + SYSTEM, protected, remote rejected, first instance, not inheritable) is stated at (.andromeda/security-plan.md:108) and detailed in the Authentication & Authorization category file, so the chunk's DACL-effect and remote-rejection assertions satisfy both sides at once.
- **Handle inheritance ↔ Authentication & Authorization, IPC access control (Windows)**: the `inheritable(false)` listener (.andromeda/security-plan.md:192) and the ban (.andromeda/security-plan.md:463) are the same property the Windows access-control clause names as "the handle is not inheritable" (.andromeda/security-plan.md:108); one test reading both ends back answers both.
- **Category file ↔ this chunk**: the category file's Windows DACL is the home's at-rest control, not the pipe's (.andromeda/security-plan-detail/data-protection.md:10); home integrity is its own route entry per the scope's Boundaries, so it is not carried — (none).
