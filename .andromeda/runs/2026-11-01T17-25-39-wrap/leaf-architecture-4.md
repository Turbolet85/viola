group: architecture-4
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»
- leaf: .claude/rules/events.md:31   class: omission
  leaf-text: «a replayed or absent snapshot read as unreachable, then `Client::connect_by`)»
  source: .andromeda/architecture-detail/standard-contracts.md:63 «a refused directory writes no frame: `src/cmd/client.rs` `connect` (`CallError::SocketDirRefused`, one `process-exit`), the hook's `connect` (`NotSent::SocketDir`, one `hook-decision`)»
  replace-with: «a replayed or absent snapshot read as unreachable), then checks the recorded endpoint's directory (`viola_state::strict::check_endpoint_dir`; a refusal reads `socket-dir-refused` and writes one `process-exit` at the CLI, one `hook-decision` at the hook and no line at MCP), then connects (the hook through `Client::connect_by`)»
- leaf: .claude/rules/events.md:31   class: omission
  leaf-text: «, then, on Linux and Windows, compares the connected peer's pid»
  source: .andromeda/architecture-detail/established-decisions.md:282 «then `check_endpoint_dir` on the recorded endpoint, then connects and reads `peer_admitted(client.server_euid())`, then, on Linux and Windows, compares»
  replace-with: «, then, on Unix, refuses a connected peer of another euid or one it cannot read (`viola_channel::peer_admitted` over `server_euid`; a refusal reads `server-verify-failed`), then, on Linux and Windows, compares the connected peer's pid»
- leaf: .claude/rules/events.md:31   class: stale
  leaf-text: «a mismatch, or a check that cannot be made, writes no frame.»
  source: .andromeda/architecture-detail/standard-contracts.md:57 «a refusal, a mismatch, or a check that cannot be made, writes no frame.»
  replace-with: «a refused directory, a refused euid, a mismatch, or a check that cannot be made, writes no frame.»
- leaf: .claude/rules/events.md:31   class: omission
  leaf-text: «macOS reports no peer pid, so `server_verdict` returns `Matches` with no comparison: the recorded gap.»
  source: .andromeda/architecture-detail/established-decisions.md:100 «The macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, stand built, so on macOS `server-verify-failed` means an euid mismatch alone.»
  replace-with: «macOS reports no peer pid, so `server_verdict` returns `Matches` with no comparison: the recorded gap, owned by no route entry; the socket-directory and euid checks stand built there, so on macOS `server-verify-failed` means an euid mismatch alone.»
