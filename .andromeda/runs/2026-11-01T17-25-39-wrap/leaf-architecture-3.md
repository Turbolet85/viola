group: architecture-3
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
- leaf: .claude/docs/services/viola.md:20   class: omission
  leaf-text: «→ exclusive endpoint bind (taken → exit 1 `squatted-name`)»
  source: .andromeda/architecture-detail/established-decisions.md:279 «→ endpoint bind (`socket_dir_ready` first, a refusal `socket-dir-refused`, exit 1, with no bind) →»
  replace-with: «→ `socket_dir_ready` (`src/cmd/run.rs`: `check_endpoint_dir`, create the parent only when absent, check again; a refused or uncreatable directory → exit 1 `socket-dir-refused`, left as found, before any snapshot, child or `Server::bind`, the `unable:` / `hint:` pair and one `process-exit`; a `Server::bind` failure does not read this code) → exclusive endpoint bind (taken → exit 1 `squatted-name`)»
- leaf: .claude/docs/services/viola.md:21   class: stale
  leaf-text: «`squatted-name` · `internal-error`, a `claude`»
  source: .andromeda/architecture-detail/established-decisions.md:330 «`squatted-name` · `socket-dir-refused` · `internal-error`, a `claude` not on `PATH`»
  replace-with: «`squatted-name` · `socket-dir-refused` · `internal-error`, a `claude`»
- leaf: .claude/docs/services/viola.md:32   class: stale
  leaf-text: «then `Client::connect_by`, then `verify_server`), and a serving process whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame — the hook records `server-verify-failed` and exits 0 with no body (»
  source: .andromeda/architecture-detail/established-decisions.md:175 «then `check_endpoint_dir` on the recorded endpoint, then `Client::connect_by`, then `peer_admitted(client.server_euid())`, then `verify_server`»
  replace-with: «then `check_endpoint_dir` on the recorded endpoint, then `Client::connect_by`, then `peer_admitted(client.server_euid())`, then `verify_server`); a refused directory gets no frame — the hook records `socket-dir-refused` (`NotSent::SocketDir`, `Asked::SocketDir`: one `hook-decision` at WARN, on the dialog path after `hook-invoked{corr:null}` with `decision_emitted:false`) and exits 0 with no body —, and a serving process of another euid, or whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame — the hook records `server-verify-failed` and exits 0 with no body (»
- leaf: .claude/docs/services/viola.md:42   class: stale
  leaf-text: «then `client::connect` (the liveness-only `live_endpoint`, then the server identity check; exit 21 `instance-unreachable` when either fails, no frame written), one request with»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «(the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check)»
  replace-with: «then `client::connect` (the liveness-only `live_endpoint`, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check; exit 21 `instance-unreachable` when any fails, `socket-dir-refused` for the directory and `server-verify-failed` for the euid or identity check, no frame written), one request with»
- leaf: .claude/docs/services/viola.md:42   class: stale
  leaf-text: «then `client::connect` (the liveness-only `live_endpoint`, then the server identity check; exit 21 when either fails, no frame written) and one request»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «(the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check)»
  replace-with: «then `client::connect` (the liveness-only `live_endpoint`, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check; exit 21 when any fails, `socket-dir-refused` for the directory and `server-verify-failed` for the euid or identity check, no frame written) and one request»
- leaf: .claude/docs/services/viola.md:43   class: stale
  leaf-text: «`client::connect` (the liveness-only `live_endpoint`, then the server identity check; exit 21 `instance-unreachable` when either fails, no frame written), one `answer` request»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «(the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check)»
  replace-with: «`client::connect` (the liveness-only `live_endpoint`, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check; exit 21 `instance-unreachable` when any fails, `socket-dir-refused` for the directory and `server-verify-failed` for the euid or identity check, no frame written), one `answer` request»
- leaf: .claude/docs/services/viola.md:44   class: stale
  leaf-text: «`client::connect` (the liveness pre-check, then the server identity check; exit 21 `instance-unreachable` when either fails, no frame written)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «(the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check)»
  replace-with: «`client::connect` (the liveness pre-check, then `check_endpoint_dir` on the recorded endpoint, then the client, then `peer_admitted(client.server_euid())`, then the server identity check; exit 21 `instance-unreachable` when any fails, `socket-dir-refused` for the directory and `server-verify-failed` for the euid or identity check, no frame written)»
- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «shared with `viola-mcp`'s frame tools), then the server identity check, then the client;»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «(the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check)»
  replace-with: «shared with `viola-mcp`'s frame tools), then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check;»
- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «(`Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, each an»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «the closed `Unreachable` (`Dead` · `Unwrapped` · `Lost` · `ServerUnverified` · `SocketDir`, each an `UnreachableCause` through `cause()`)»
  replace-with: «(`Dead` · `Unwrapped` · `Lost` · `ServerUnverified` · `SocketDir`, each an»
- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «`endpoint-lost` · `server-verify-failed`;»
  source: .andromeda/architecture.md:226 «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`)»
  replace-with: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`;»
- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «and no `channel-request` line for a failed server check;»
  source: .andromeda/architecture-detail/occupied-resources.md:129 «a CLI frame verb writes `process-exit{subject:"self"}`, exit 21 with `detail:"socket-dir-refused"` + `during:"connect"`, at WARN and no `channel-request` line»
  replace-with: «and no `channel-request` line for a failed server check, a server of another euid included, or `detail:"socket-dir-refused"` + `during:"connect"` and no `channel-request` line for a refused socket directory;»
- leaf: .claude/docs/stack.md:14   class: omission
  leaf-text: «on Unix, libc gives the uid for the per-user socket dir and the socket is chmod 0600 after the bind»
  source: .andromeda/architecture-detail/stack-and-technologies.md:15 «the directory itself is read by `viola_state::strict::check_endpoint_dir` (the recorded endpoint's parent, through `check_socket_dir`, one std `symlink_metadata`) before a bind»
  replace-with: «on Unix, libc gives the uid for the per-user socket dir and the socket is chmod 0600 after the bind; the directory itself is read by `viola_state::strict::check_endpoint_dir` (the recorded endpoint's parent, through `check_socket_dir`, one std `symlink_metadata`) before a bind (`viola run`, `revive`'s start) and before each client's connect (the CLI frame verbs, `hook`, MCP's `frame`), a symlink, a non-directory, another owner or a group or other bit refused and never chmodded, an absent path passing»
