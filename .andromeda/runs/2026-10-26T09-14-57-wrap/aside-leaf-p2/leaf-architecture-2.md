group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «unreachable / fault = `isError:true` + `{"error":"instance-unreachable"|"wrapper-fault","detail":…}`.»
  source: .andromeda/architecture-detail/conventions.md:53 «A frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper (its pid or OS start time not the snapshot's, or the check cannot be made»
  replace-with: «unreachable / fault = `isError:true` + `{"error":"instance-unreachable"|"wrapper-fault","detail":…}`; a frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper (its peer pid or OS start time not the snapshot's, or the check cannot be made; the start time read on `tokio::task::spawn_blocking`) returns the `instance-unreachable` result with no frame written and no log line (macOS reports no peer pid: the recorded gap).»

- leaf: .claude/docs/services/viola-state.md:6   class: stale
  leaf-text: «`liveness::live_endpoint(instance_dir)`, moved unchanged from `src/cmd/client.rs`, the liveness-only endpoint read shared by CLI `send` / `wait` / `last` / `answer` / `pause` and `viola-mcp`'s frame tools»
  source: .andromeda/architecture-detail/standard-contracts.md:93 «`live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call»
  replace-with: «`liveness::live_endpoint(instance_dir)` → `Option<LiveEndpoint>` (`{ endpoint, pid, started_at }`, all from its one `read_snapshot` call), shared by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` and `viola-mcp`'s frame tools; the server identity check (the closed `ServerVerdict` `Matches` · `Mismatch` · `Unverifiable`, the pure `server_verdict`, the joined `verify_server` reading the observed start time through `process_start_time`: the snapshot's pid and start time against the connected server's, made before the first frame by those CLI verbs, `viola hook` and `viola-mcp`'s frame tools; with no peer pid reported, as on macOS, `Matches` with no comparison, the recorded gap)»

- leaf: .claude/docs/services/viola.md:81   class: omission
  leaf-text: «`viola hook` exits 0 on clap errors, channel failures and caught panics.»
  source: .andromeda/architecture-detail/established-decisions.md:110 «The event and dialog paths share one connect: `read_snapshot`, then `Client::connect_by`, then `verify_server`»
  replace-with: «`viola hook` exits 0 on clap errors, channel failures, a failed server check and caught panics: its event and dialog paths share one connect (`read_snapshot`, then `Client::connect_by`, then `verify_server`), and a serving process whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame — the hook records `server-verify-failed` and exits 0 with no body (the event lost as for an unreachable endpoint, the dialog rendered for the human, SessionEnd falling back to its direct `try_lock` append).»

- leaf: .claude/docs/services/viola.md:91   class: stale
  leaf-text: «the liveness-only `live_endpoint` (exit 21), one request with `from` from `VIOLA_NAME`»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper»
  replace-with: «`client::connect` (the liveness-only `live_endpoint`, then the server identity check; exit 21 `instance-unreachable` when either fails, no frame written), one request with `from` from `VIOLA_NAME`»

- leaf: .claude/docs/services/viola.md:92   class: stale
  leaf-text: «the liveness-only `live_endpoint` (exit 21), one `answer` request»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper»
  replace-with: «`client::connect` (the liveness-only `live_endpoint`, then the server identity check; exit 21 `instance-unreachable` when either fails, no frame written), one `answer` request»

- leaf: .claude/docs/services/viola.md:93   class: omission
  leaf-text: «client paste check, the liveness pre-check, the readback mirror»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper»
  replace-with: «client paste check, `client::connect` (the liveness pre-check, then the server identity check; exit 21 `instance-unreachable` when either fails, no frame written), the readback mirror»

- leaf: .claude/docs/services/viola.md:94   class: stale
  leaf-text: «over `src/cmd/client.rs` (the plumbing the three client verbs share: the `cli` obs start, the reply shapes (the liveness-only endpoint read is `viola_state::liveness::live_endpoint`, moved there unchanged from `src/cmd/client.rs` and shared with `viola-mcp`'s frame tools), exit 20 / exit 21 — the latter at WARN)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «client.rs (a helper, no subcommand): `connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) opens»
  replace-with: «over `src/cmd/client.rs` (`connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) opens: the live endpoint (`viola_state::liveness::live_endpoint` → `LiveEndpoint`, shared with `viola-mcp`'s frame tools), then the server identity check, then the client; the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`; the channel client those verbs share, the `cli` obs start, the reply shapes, exit 20 / exit 21 — the latter at WARN, a failed server check logged `server-verify-failed` with `during:"connect"` and no `channel-request` line)»
