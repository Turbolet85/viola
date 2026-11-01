group: architecture-2
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»

- leaf: .claude/docs/services/viola-core.md:6   class: stale
  leaf-text: «`UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word through `as_str`:»
  source: .andromeda/architecture-detail/conventions.md:33 «`viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`)»
  replace-with: «`UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`:»

- leaf: .claude/docs/services/viola-mcp.md:22   class: stale
  leaf-text: «`endpoint-lost` · `server-verify-failed`, `of_reply` reading»
  source: .andromeda/architecture-detail/conventions.md:125 «`strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`; `wrapper-fault` with»
  replace-with: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, `of_reply` reading»

- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «and `tools.rs` `frame` no `live_endpoint` through `viola_state::liveness::without_endpoint`»
  source: .andromeda/architecture-detail/established-decisions.md:144 «a refused `check_endpoint_dir` after `live_endpoint` as `SocketDir`, and a refused `peer_admitted(client.server_euid())` or a failed `server_verified` as `ServerVerify`, each with no frame»
  replace-with: «and `tools.rs` `frame` no `live_endpoint` through `viola_state::liveness::without_endpoint`, a refused `viola_state::strict::check_endpoint_dir` after `live_endpoint` as `SocketDir` and a refused `peer_admitted(client.server_euid())` or a failed `server_verified` as `ServerVerify`, each with no frame and no line, `isError` the record»

- leaf: .claude/docs/services/viola-mcp.md:22   class: stale
  leaf-text: «(its peer pid or OS start time not the snapshot's, or the check cannot be made; the start time read on `tokio::task::spawn_blocking`)»
  source: .andromeda/architecture-detail/conventions.md:132 «(on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its pid or OS start time not the snapshot's, or the check cannot be made; either reads `UnreachableCause::ServerVerify`;»
  replace-with: «(on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its peer pid or OS start time not the snapshot's, or the check cannot be made; either reads `UnreachableCause::ServerVerify`; the start time read on `tokio::task::spawn_blocking`)»

- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «(macOS reports no peer pid: the recorded gap)»
  source: .andromeda/architecture-detail/standard-contracts.md:57 «macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls.»
  replace-with: «(macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls)»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «`check_present` and `check_stamps`.»
  source: .andromeda/architecture-detail/established-decisions.md:106 «`viola_state::strict::check_socket_dir` takes one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, then `socket_dir_verdict` reads a symlink as»
  replace-with: «`check_present`, `check_stamps` and `check_socket_dir` (one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, then `socket_dir_verdict` reads a symlink as `Refused::Symlink`, a non-directory as the fixed-message `Refused::NotADirectory` and anything else by the read rule; it never chmods, a refused directory stays as found, and off Unix it returns `Ok(())`), with `check_endpoint_dir`, that check on a recorded endpoint's parent, run before a bind (`viola run`, `revive`'s start) and before each client's connect (the CLI frame verbs, `hook`, `viola-mcp`'s `frame`).»

- leaf: .claude/docs/services/viola-state.md:30   class: omission
  leaf-text: «`scan` checks the home, `instances/` and each instance without following a link»
  source: .andromeda/architecture-detail/occupied-resources.md:195 «`viola list` holds the home to the read rule on every OS: `board::scan` reads it through `check_read` with or without an instance, so a 0750 home»
  replace-with: «`scan` checks the home by the read rule (`check_read`, on every OS, with or without an instance, a refusal `ScanRefused::Home`), `instances/` by `check_path` and each instance without following a link»
