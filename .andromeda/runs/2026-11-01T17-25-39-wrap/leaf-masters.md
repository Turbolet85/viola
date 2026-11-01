group: masters
read: CLAUDE.md · 129 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «both clients read the connected server's peer pid (`Client::server_pid` / `TokioClient::server_pid`, none on macOS where `SERVER_PID_REPORTED` is false)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:65 «the peer euid decision (`peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED`), the server's euid (`Client::server_euid`, `TokioClient::server_euid`), and server.rs's `accept_loop`»
  replace-with: «both clients read the connected server's peer pid (`Client::server_pid` / `TokioClient::server_pid`, none on macOS where `SERVER_PID_REPORTED` is false) and its peer euid (`Client::server_euid` / `TokioClient::server_euid`, none on Windows); the peer euid decision (`peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED`), and the server's `accept_loop` dropping a refused stream before any frame»
- leaf: CLAUDE.md:23   class: omission
  leaf-text: «`check_present`, `check_stamps`)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:71 «`check_stamps`, `check_socket_dir`, `check_endpoint_dir` and the plain `socket_dir_verdict`, with the fixed-message `Refused::NotADirectory`)»
  replace-with: «`check_present`, `check_stamps`, `check_socket_dir`, `check_endpoint_dir` and the plain `socket_dir_verdict`, with the fixed-message `Refused::NotADirectory`)»
- leaf: CLAUDE.md:25   class: omission
  leaf-text: «tools `send · wait · last · answer · list` (Tokio)»
  source: .andromeda/architecture-detail/conventions.md:138 «MCP's `frame`, after `live_endpoint`, runs `check_endpoint_dir` and answers `result::unreachable(UnreachableCause::SocketDir)`»
  replace-with: «tools `send · wait · last · answer · list` (Tokio); a frame tool's `frame` runs `check_endpoint_dir` after `live_endpoint` and the peer euid decision beside the server check, a refusal being `instance-unreachable` (`socket-dir-refused` or `server-verify-failed`) with no frame and no line»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «the same way before its log),»
  source: .andromeda/architecture-detail/established-decisions.md:287 «`viola run` and `viola revive`'s start run `socket_dir_ready`: `check_endpoint_dir`, create the parent only when absent, check again, so a race's loser reads the same;»
  replace-with: «the same way before its log), the start's socket-directory check (`src/cmd/run.rs`'s `socket_dir_ready`, for `run` and `revive`'s start: `check_endpoint_dir`, the parent created only when absent, checked again, before any snapshot, child or `Server::bind`; a failed create and a refused check both exit 1, `socket-dir-refused`, the directory left as found, and a `Server::bind` failure does not read that code),»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «the two `--capture` arms ahead of it)»
  source: .andromeda/architecture-detail/established-decisions.md:191 «`viola hook` meeting a refused socket directory (`NotSent::SocketDir`, `Asked::SocketDir`) exits 0 with empty streams and no frame and writes one `hook-decision` at WARN»
  replace-with: «the two `--capture` arms ahead of it; past that check the hook's `connect` runs `check_endpoint_dir`, the connect, `peer_admitted` and the server check, a refused directory or euid exiting 0 with empty streams and no frame and writing one `hook-decision` at WARN, `socket-dir-refused` or `server-verify-failed`)»
- leaf: CLAUDE.md:27   class: omission
  leaf-text: «then the liveness pre-check, the server check and one frame)»
  source: .andromeda/architecture-detail/established-decisions.md:218 «then through `cmd::client::connect` the socket-directory check (`check_endpoint_dir`, a refusal exit 21, `socket-dir-refused`), the connect, the peer euid check and the server identity check»
  replace-with: «then the liveness pre-check and, through `client::connect` (shared with the six CLI frame verbs), the socket-directory check on the recorded endpoint (`check_endpoint_dir`, exit 21, `socket-dir-refused`), the connect, the peer euid check and the server check (either refusal exit 21, `server-verify-failed`), and one frame)»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «by `viola list`'s human mode only to choose its SGR bytes).»
  source: .andromeda/security-plan-detail/security-anti-patterns.md:40 «NEVER add an env, flag or feature seam to force a refusing peer-euid decision»
  replace-with: «by `viola list`'s human mode only to choose its SGR bytes); never add an env, flag or feature seam to force a refusing peer-euid decision: the decisions are proven over injected values.»
- leaf: CLAUDE.md:39   class: omission
  leaf-text: «set 0700 dirs / 0600 files explicitly (never the umask)»
  source: .andromeda/architecture.md:265 «checked before the bind and before every client's connect and, when refused (`socket-dir-refused`), left as found, never narrowed»
  replace-with: «set 0700 dirs / 0600 files explicitly (never the umask), and never chmod a refused socket directory: it is left as found, never narrowed»
