group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/security-summary.md:14   class: omission
  leaf-text: «Unix 0700 per-user socket dir + socket chmod 0600 after the bind + `peer_creds` euid check.»
  source: .andromeda/security-plan-detail/threat-model-summary.md:53 «On Unix the socket sits in the per-user 0700 directory, not in world-writable `/tmp`; a directory another user owns, a loosened one or a symlink is refused by the socket-directory check»
  replace-with: «Unix 0700 per-user socket dir + socket chmod 0600 after the bind + `peer_creds` euid check; a directory another user owns, a loosened one or a symlink is refused by the socket-directory check (`socket-dir-refused`), with no bind on `run` and no frame on a client.»
- leaf: .claude/docs/security-summary.md:15   class: stale
  leaf-text: «the client euid check and macOS (no peer pid) are owed to "Unix endpoint and home hardening".»
  source: .andromeda/security-plan-detail/threat-model-summary.md:56 «macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.»
  replace-with: «ahead of that comparison, on Linux and macOS, each runs the socket-directory check on the recorded endpoint before connecting (a refusal `socket-dir-refused`, no frame) and the peer euid decision past the connect (a refusal `server-verify-failed`, no frame); macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.»
- leaf: .claude/docs/security-summary.md:38   class: omission
  leaf-text: «another seam or environment read, or a colour fact changing a control or reaching a log line, error or refusal, needs its own Decisions Log entry.»
  source: .andromeda/security-plan-detail/security-anti-patterns.md:40 «NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values»
  replace-with: «another seam or environment read, or a colour fact changing a control or reaching a log line, error or refusal, needs its own Decisions Log entry. NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values (`peer_euid_admitted`, `serve_admitting`).»
- leaf: .claude/docs/security-summary.md:64   class: omission
  leaf-text: «`board::scan` checks the home, `instances/` and each instance before any file under the home is opened»
  source: .andromeda/security-plan-detail/authentication-authorization.md:66 «`scan` holds the home to `check_read` and `instances/` to `check_path` on every OS, the first failure refusing the whole board (`ScanRefused::Home` on the home)»
  replace-with: «`board::scan` holds the home to the read rule on every OS (`check_read`; a home another user may read is refused with or without an instance; that rule stands on a delegate's answer, OPEN, owner the Epoch 7 boundary batch), `instances/` to `check_path` and each instance to `check_instance`, before any file under the home is opened»
- leaf: .claude/docs/security-summary.md:65   class: stale
  leaf-text: «`pinned-hash-mismatch` · `squatted-name` · `internal-error`»
  source: .andromeda/security-plan-detail/input-validation.md:39 «`pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
  replace-with: «`pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
- leaf: .claude/rules/security.md:11   class: omission
  leaf-text: «`peer_creds` euid on both sides, `try_overwrite` off.»
  source: .andromeda/security-plan-detail/authentication-authorization.md:18 «a failed create and a refused check read one code: `run` and the start of `revive` exit 1 with `socket-dir-refused` and one `process-exit` of that `detail`»
  replace-with: «`peer_creds` euid on both sides (the wrapper drops a refused peer in `accept_loop` before any frame; a client refuses a peer whose euid cannot be read), `try_overwrite` off. The directory check (`check_socket_dir`, and `check_endpoint_dir` on a recorded endpoint's parent: owner == `geteuid()`, `mode & 0o077 == 0`, not a symlink, never a chmod) runs before the bind and before any client connect, at the seams and never inside `viola_channel::Client::connect`; `run` and the start of `revive` refuse before any snapshot, child or `Server::bind`, a failed create and a refused check alike: exit 1, `socket-dir-refused`, one `process-exit`.»
- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «(the client euid check, and macOS, where the peer reports no pid, owed to the route entry "Unix endpoint and home hardening")»
  source: .andromeda/security-plan.md:114 «macOS compares no peer pid or start time, the named carve-out, owned by no route entry, so there `server-verify-failed` means an euid mismatch alone»
  replace-with: «(the peer euid checked too on Unix, never skipped; macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry)»
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «then the liveness-only pre-check and, on Linux and Windows, the server check — CLI»
  source: .andromeda/security-plan-detail/authentication-authorization.md:48 «On every client seam the order is the directory check, the connect, the peer euid decision, then the server verification against the snapshot.»
  replace-with: «then the liveness-only pre-check, the socket-directory check on the recorded endpoint (`check_endpoint_dir`), the connect, on Unix the peer euid decision (`peer_admitted`) and, on Linux and Windows, the server check — CLI»
- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «then `client::connect` (the peer pid and its start time against the snapshot's `pid` and `started_at`)»
  source: .andromeda/security-plan-detail/authentication-authorization.md:101 «`cmd::client::connect` before their one `link` / `unlink` frame: the socket-directory check on the recorded endpoint, the connect, the peer euid decision on Unix and, on Linux and Windows, the server»
  replace-with: «then `client::connect` (the socket-directory check on the recorded endpoint, the connect, on Unix the peer euid decision and, on Linux and Windows, the peer pid and its start time against the snapshot's `pid` and `started_at`)»
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «and the server check after `TokioClient::connect`.»
  source: .andromeda/security-plan-detail/input-validation.md:37 «After `live_endpoint`, `frame` runs `check_endpoint_dir` (a refusal `socket-dir-refused`) and, beside the server check, the peer euid decision (a refusal `server-verify-failed`)»
  replace-with: «then `check_endpoint_dir` (a refusal `socket-dir-refused`) and, after `TokioClient::connect`, the peer euid decision (a refusal `server-verify-failed`) and the server check.»
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «On a mismatch, or a check that cannot be made, no frame is written:»
  source: .andromeda/security-plan-detail/authentication-authorization.md:18 «On failure `run` refuses to start (exit 1), CLI/MCP clients report `instance-unreachable` with `detail` `socket-dir-refused` (the CLI exit 21)»
  replace-with: «On a refused socket directory, before the connect, no frame is written: the CLI frame verbs exit 21 `instance-unreachable` (under `--json`, `detail` `socket-dir-refused`), MCP returns `instance-unreachable` with `detail` `socket-dir-refused` and writes no line, and `hook` fails open (exit 0, no body, one WARN `hook-decision` carrying `socket-dir-refused`). On a mismatch, a refused peer euid, or a check that cannot be made, no frame is written:»
- leaf: .claude/rules/security.md:14   class: omission
  leaf-text: «the frame verbs through `check_instance`, exit 21;»
  source: .andromeda/security-plan-detail/authentication-authorization.md:66 «`scan` holds the home to `check_read` and `instances/` to `check_path` on every OS, the first failure refusing the whole board (`ScanRefused::Home` on the home)»
  replace-with: «`viola list`'s `board::scan` through `check_read` on the home on every OS (a home another user may read refused with or without an instance; that rule stands on a delegate's answer, OPEN, owner the Epoch 7 boundary batch), `check_path` on `instances/` and `check_instance` per instance, the whole board refused, exit 21; the frame verbs through `check_instance`, exit 21;»
- leaf: .claude/rules/security.md:26   class: stale
  leaf-text: «`pinned-hash-mismatch` · `squatted-name` · `internal-error`»
  source: .andromeda/security-plan-detail/input-validation.md:39 «`pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
  replace-with: «`pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
- leaf: .claude/rules/security.md:37   class: omission
  leaf-text: «Another seam or environment read, a colour fact changing a control or reaching a log line, error or refusal, or another G2 exemption, needs a Decisions Log entry.»
  source: .andromeda/security-plan-detail/security-anti-patterns.md:40 «NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values»
  replace-with: «Another seam or environment read, a colour fact changing a control or reaching a log line, error or refusal, or another G2 exemption, needs a Decisions Log entry. Never add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values (`peer_euid_admitted`, `serve_admitting`).»
- leaf: .claude/rules/security.md:40   class: stale
  leaf-text: «`endpoint-lost` · `server-verify-failed`, as in CLI»
  source: .andromeda/security-plan-detail/error-handling.md:25 «Exit 21's `detail` is a closed set of six causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`»
  replace-with: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, as in CLI»
