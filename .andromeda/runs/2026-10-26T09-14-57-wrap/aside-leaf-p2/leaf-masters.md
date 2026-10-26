group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «sync client/server, Tokio client behind a feature»
  source: .andromeda/architecture-detail/established-decisions.md:67 «- `viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream's peer pid through interprocess's `peer_creds().pid()`: Linux `SO_PEERCRED`, Windows»
  replace-with: «sync client/server, Tokio client behind a feature; both clients read the connected server's peer pid (`Client::server_pid` / `TokioClient::server_pid`, none on macOS where `SERVER_PID_REPORTED` is false)»

- leaf: CLAUDE.md:23   class: omission
  leaf-text: «liveness, strict-modes»
  source: .andromeda/architecture-detail/standard-contracts.md:93 «- The server identity read is `viola_state::liveness`'s. `live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call»
  replace-with: «liveness (`live_endpoint` → `LiveEndpoint`: endpoint, pid and start time from one snapshot read) and the server identity check made before every client's first frame (`ServerVerdict`, `server_verdict`, `verify_server`; macOS the recorded gap), strict-modes»
