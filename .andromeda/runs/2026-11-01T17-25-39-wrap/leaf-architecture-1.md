group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «no line written, no child started; `viola verify`'s refusals each carry»
  source: .andromeda/architecture-detail/conventions.md:65 «Exit `1` also covers `viola run` and `viola revive`'s start refusing the socket directory (`socket_dir_ready` in `src/cmd/run.rs`, before any snapshot, child or `Server::bind`)»
  replace-with: «no line written, no child started; also `viola run` and `viola revive`'s start refusing the socket directory (`socket_dir_ready`, before any snapshot, child or `Server::bind`; a failed create and a refused check read this one code, a `Server::bind` failure does not): the stderr `unable:` / `hint:` pair, or under `viola revive --json` the `socket-dir-refused` document, and one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, no path, pid or uid; `viola verify`'s refusals each carry»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «`endpoint-lost` · `server-verify-failed`, no `RefusalReason`»
  source: .andromeda/architecture.md:226 «`<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`)»
  replace-with: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, no `RefusalReason`»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «a server that is not the recorded wrapper — the connected peer's pid or OS start time not the snapshot's, or the check cannot be made — no frame written, `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` (claimed off macOS only, `v1-18`'s named carve-out), after the client's own exit-13 refusals,»
  source: .andromeda/architecture-detail/conventions.md:101 «An exception: `server-verify-failed` is claimed on Linux, Windows and macOS, where on macOS it means an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry.»
  replace-with: «a server that is not the recorded wrapper — on Unix a connected peer whose euid is not this user's or cannot be read (`peer_admitted`), on Linux and Windows the connected peer's pid or OS start time not the snapshot's, or that check not made — no frame written, `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` (claimed on Linux, Windows and macOS, on macOS an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry), after the client's own exit-13 refusals, the strict-modes instance check and the socket-directory check,»
- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «`{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`, no log opened, no line written); `viola hook` always 0.»
  source: .andromeda/architecture-detail/conventions.md:119 «Exit `21`'s refused socket directory: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, `src/cmd/client.rs` `connect`'s refused check is `CallError::SocketDirRefused»
  replace-with: «`{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`, no log opened, no line written; and, on those same eight verbs, past that check and before any connect, a refused socket directory on the recorded endpoint (`check_endpoint_dir` in `client::connect`): the socket-directory hint last on stderr, or `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` under `--json`, one `process-exit` at WARN with `during` `connect` and no `channel-request`); `viola hook` always 0.»
- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «`pinned-hash-mismatch` · `squatted-name` · `internal-error`, by `client::refused_document`»
  source: .andromeda/architecture.md:229 «`batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`, written by `client::refused_document`»
  replace-with: «`pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`, by `client::refused_document`»
- leaf: .claude/docs/gotchas.md:123   class: stale
  leaf-text: «a client connect reads NotFound (the harness `endpoint_gone` rule)»
  source: .andromeda/architecture-detail/occupied-resources.md:238 «Test-only: harness `cleanup`'s `endpoint_gone` on Unix reads through `unconnectable` (`crates/viola-e2e/src/harness/cleanup.rs`): `UnixStream::connect` refused or `NotFound` reads gone»
  replace-with: «a client connect reads NotFound, on Unix refused or NotFound (the harness `endpoint_gone` rule, on Unix through `unconnectable`)»
- leaf: .claude/docs/services/viola-channel.md:6   class: omission
  leaf-text: «the per-OS listener hardening and the connected server's peer pid»
  source: .andromeda/architecture-detail/established-decisions.md:88 «`viola_channel` exports `peer_euid_admitted`, `peer_admitted` and `PEER_EUID_REPORTED`, the last `cfg!(unix)`: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`»
  replace-with: «the per-OS listener hardening, the peer euid decision (`peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED`, the last `cfg!(unix)`; a peer whose euid cannot be read is refused where it is reported), the connected server's euid (`Client::server_euid` / `TokioClient::server_euid`, writing and logging nothing) and the connected server's peer pid»
- leaf: .claude/docs/services/viola-channel.md:22   class: stale
  leaf-text: «Unix: 0700 per-user dir checked (`lstat`, owner, no symlink) before bind and before connect;»
  source: .andromeda/architecture-detail/established-decisions.md:81 «No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge»
  replace-with: «Unix: 0700 per-user dir checked before bind and before connect by `viola_state::strict::check_endpoint_dir` (one `symlink_metadata`; an absent path passes; a symlink, a non-directory, another owner or any group or other bit is refused, `socket-dir-refused`, left as found and never chmodded), never inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, so the check stands at the channel seam's callers;»
- leaf: .claude/docs/services/viola-channel.md:22   class: omission
  leaf-text: «`peer_creds` euid on accept.»
  source: .andromeda/architecture-detail/established-decisions.md:94 «The wrapper's `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and any frame»
  replace-with: «`accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))` (`peer_creds` euid), so a refused stream is dropped on the accept thread, before its worker thread and any frame, reading end-of-file and writing no line; `Server::serve` injects `peer_admitted` as a `type Admission` through the private `serve_admitting`, with no env, flag or feature seam.»
- leaf: .claude/docs/services/viola-channel.md:25   class: omission
  leaf-text: «then compares that peer pid and its OS start time with the snapshot's `pid` / `started_at` (`viola_state::liveness::verify_server` → `ServerVerdict`); a mismatch or a check that cannot be made writes no frame (CLI exit 21 / MCP `instance-unreachable`, its `detail` `server-verify-failed`, the hook fails open with no body);»
  source: .andromeda/architecture-detail/occupied-resources.md:20 «Ahead of that comparison each checks the recorded `endpoint`'s directory (`viola_state::strict::check_endpoint_dir`) before it connects, a refusal reading `socket-dir-refused`, and then, on Unix,»
  replace-with: «then checks the recorded endpoint's directory (`viola_state::strict::check_endpoint_dir`; a refusal writes no frame: CLI exit 21 / MCP `instance-unreachable`, its `detail` `socket-dir-refused`, the hook fails open with no body), connects, reads the connected peer's euid on Unix (`peer_admitted` over `Client::server_euid` / `TokioClient::server_euid`), then compares that peer pid and its OS start time with the snapshot's `pid` / `started_at` (`viola_state::liveness::verify_server` → `ServerVerdict`); a foreign or unreadable euid, a mismatch or a check that cannot be made writes no frame (CLI exit 21 / MCP `instance-unreachable`, its `detail` `server-verify-failed`, the hook fails open with no body);»
- leaf: .claude/docs/services/viola-channel.md:25   class: retired
  leaf-text: «macOS reports no peer pid and stays the recorded gap (`server_verdict` returns `Matches` with no comparison); its client euid and socket-directory checks are owed to the working-route entry "Unix endpoint and home hardening".»
  source: .andromeda/architecture-detail/established-decisions.md:100 «the recorded gap, named and owned by no route entry. The macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, stand built, so on macOS `server-verify-failed` means»
  replace-with: «macOS reports no peer pid and stays the recorded gap, owned by no route entry (`server_verdict` returns `Matches` with no comparison); its client euid and socket-directory checks stand built, so on macOS `server-verify-failed` means an euid mismatch alone.»
