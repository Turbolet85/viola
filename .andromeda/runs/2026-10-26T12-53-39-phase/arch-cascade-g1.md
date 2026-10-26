# arch · cascade read · group 1 of 4

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:11) The two verbs must run as std-thread CLI processes on `viola-channel`'s sync client, and the wrapper must serve `link` / `unlink` on its std-thread channel server, with no Tokio runtime built on either path.
  > Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp`
  (.andromeda/architecture.md:13)
  > std threads with Tokio built only inside `mcp`
  (.andromeda/architecture.md:442)
  > A sync std-thread core; Tokio 1.53.3 only in `mcp`
- (.andromeda/architecture-detail/stack-and-technologies.md:12) clap must serve `link` and `unlink` as subcommands so the stated list holds, and the `viola --help` table's new `handoff:  link, unlink` row stays plain clap output with no colour crate in the graph.
  > clap 4.6.7 (derive; `default-features = false` with `std`, `derive`, `help`, `usage`, `error-context`, `suggestions` — its default set minus `color`, so help and usage output is plain and no colour/terminal crate is in the graph) | Subcommands `run · revive · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`
  (.andromeda/architecture.md:442)
  > clap 4.6.7 for the CLI
- (.andromeda/architecture-detail/stack-and-technologies.md:15) The verbs must reach the driven wrapper's endpoint through interprocess's sync `local_socket` client, on Windows only through the SQOS Identification open that adopts the handle, never interprocess's default connect.
  > interprocess 2.4.4 `local_socket` (sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`); on Windows the client opens the pipe itself with windows-sys 0.61.2 `CreateFileW(SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION \| FILE_FLAG_OVERLAPPED)` and adopts the handle with `local_socket::Stream::try_from`, never interprocess's default connect.
  (.andromeda/architecture.md:444)
  > IPC: interprocess 2.4.4 local sockets, one endpoint per `viola run`, JSON-RPC 2.0 over ndjson with `v` + sender version in every `params`.
- (.andromeda/architecture-detail/stack-and-technologies.md:16) The served `link` and `unlink` must change the link set only through the instance's ndjson log and its snapshot, with no other store.
  > None. ndjson append logs + atomically replaced JSON snapshots on the local filesystem | Authoritative audit trail and mutable state (wheel mirror, links, budget readings)
  (.andromeda/architecture.md:443)
  > Database: none. ndjson append logs + atomic JSON snapshots (tempfile 3.27.0 `persist` through one shared helper) + std `File::lock`, serde_json 1.0.151.
- (.andromeda/architecture-detail/stack-and-technologies.md:18) Each `link` / `unlink` line must be one append `write` under the `.lock` sibling, each `links` rewrite must go through `replace_private`, and the instance-check form the founder may name runs the strict-modes check through these same libc / windows-sys primitives.
  > tempfile 3.27.0 `persist` through one shared helper (`viola_state::fs::replace_private`: the mode is set on the temp file before the write, then `sync_all` and `persist`) for snapshots and every other replaced file; std `File::lock` on separate `.lock` files, opened for write (Windows refuses an exclusive lock on an append-only handle); std `OpenOptions::append` (one `write` per line); the strict-modes check and private-dir creation through libc (Unix modes and owner) and windows-sys (owner + DACL, and the protected user + SYSTEM DACL on a home created outside the user's profile, `Win32_UI_Shell` `GetUserProfileDirectoryW`; a DACL that cannot be set fails the creation)
- (.andromeda/architecture-detail/stack-and-technologies.md:20) The verbs' new `--json` documents must print `v` first and `ok` (or `error`) after it, in the declared key order serde_json's `preserve_order` keeps.
  > `preserve_order` keeps a printed document's declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`)
- (.andromeda/architecture-detail/stack-and-technologies.md:21) The `link` / `unlink` `params` the verbs write and the wrapper parses must reach no `Debug` render, as every other frame's `params` does not.
  > veil `=0.3.0` (`#[derive(Redact)]`, feature `toggle` banned), so a frame's `params` never reach a `Debug` render
- (.andromeda/architecture-detail/stack-and-technologies.md:22) `driver` and `driven` must pass `ViolaName::try_new` before any path join or frame, and a logged link name shown on a result line is parsed through it again.
  > nutype 0.8.0 | `ViolaName` (charset + length cap)
  (.andromeda/architecture.md:447)
  > Validation: tolerant serde for external payloads, versioned-tolerant own formats, nutype 0.8.0 newtypes.
- (.andromeda/architecture-detail/stack-and-technologies.md:23) The `since` on the `link … since …Z` line and in the `ok` payload must be chrono's RFC 3339 UTC with milliseconds and `Z`.
  > chrono 0.4.45 | RFC 3339 UTC, millisecond precision.
  (.andromeda/architecture.md:448)
  > Timestamps: chrono 0.4.45, RFC 3339 UTC with milliseconds and `Z`, field `ts` / `*_at`.
- (.andromeda/architecture-detail/stack-and-technologies.md:25) `link` and `unlink` must be request/response frames on the driven wrapper's own local socket, with no broker or daemon between the verb and the wrapper.
  > None. Per-wrapper local sockets (JSON-RPC 2.0 over ndjson) for request/response | Local IPC without a broker or daemon
  (.andromeda/architecture.md:13)
  > interprocess local sockets (JSON-RPC 2.0 over ndjson)
- (.andromeda/architecture-detail/stack-and-technologies.md:26) The verbs' liveness pre-check must decide the driven instance's liveness from the snapshot's pid and process start time through sysinfo, with the heartbeat as the primary signal.
  > sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file
- (.andromeda/architecture-detail/stack-and-technologies.md:27) No `link` / `unlink` MCP tool is added, so the tool list stays these five, and whether `v1-13`'s MCP half is provable without one is the scope's open read.
  > tools `send · wait · last · answer · list`.
- (.andromeda/architecture-detail/stack-and-technologies.md:32) With `LinkSet::remove`'s `#[cfg_attr(not(test), allow(dead_code))]` removed, the `unlink` method must be its product caller so clippy `-D warnings` stays green, and the verbs' stdout result goes through no `print!` / `eprintln!` / `dbg!`.
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`;
- (.andromeda/architecture-detail/stack-and-technologies.md:39) The verbs are frame-writing clients: each must read the serving process's peer pid from the connected stream and check it before its first `link` / `unlink` frame on Linux and Windows, macOS the recorded gap.
  > Wrapper IPC: both clients read the serving process's pid from the connected stream through interprocess's `StreamCommon::peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS), the identity each frame-writing client checks before its first frame (§Standard Contracts, Session liveness).
- (.andromeda/architecture-detail/stack-and-technologies.md:45) The server check the verbs take through `client::connect` must compare the peer's sysinfo start time with the snapshot's `started_at`, beside its pid.
  > Process liveness: sysinfo's process start time is also the observed side of the server check each frame-writing client makes before its first frame (the connected peer's pid and its start time against the snapshot's `pid` and `started_at`; §Standard Contracts, Session liveness).
- (.andromeda/architecture.md:6) `viola link` / `viola unlink` must be short-lived separate processes owning no endpoint, the link set they change shared only through the driven instance's log and snapshot.
  > Every other verb is a separate process that owns no endpoint (short-lived, except the long-running `mcp`; `viola revive --list` and a refused revive are among them), and the state all processes share lives in ndjson logs and atomic snapshots that survive a crash on either side.
  (.andromeda/architecture.md:429)
  > Modular monolith: one binary with compiler-enforced internal crates (`pty · channel · state · agent-claude · mcp` around `core`), and no daemon.
- (.andromeda/architecture.md:4) The scope's boundary of no ledger row, `viola verify` child or re-stamp holds only while a link relies on no `claude` CLI behaviour, which P3 re-reads.
  > Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger.
  (.andromeda/architecture.md:449)
  > Agent isolation: Claude-specific shapes only in `viola-agent-claude`, and every undocumented behaviour is a capability-ledger row.
- (.andromeda/architecture.md:431) The new capability must arrive as the two subcommands and two channel methods with `--json`, and as MCP tools only where a driver needs them.
  > New capabilities arrive as new subcommands or channel verbs, exposed consistently on the CLI (`--json`) and, where a driver needs them, as MCP tools.
  (.andromeda/architecture.md:446)
  > typed CLI exit codes;

## Bindings
- **Tokio-free verbs and methods ↔ established decisions and cross-cutting patterns**: the std-thread row binds with the [Concurrency] keystone and Tokio containment, so the verbs use the sync client and the methods the std-thread server (.andromeda/architecture.md:23), (.andromeda/architecture.md:410)
- **clap's subcommand list ↔ occupied resources and conventions**: the served subcommands row binds with the subcommand registry and the CLI verb shapes `viola link <driver> <driven>` / `viola unlink <driver> <driven>` (.andromeda/architecture.md:270), (.andromeda/architecture.md:125)
- **The sync client and peer-pid identity ↔ standard contracts**: the peer pid and sysinfo start time are checked against the snapshot's `pid` and `started_at` before the first frame (.andromeda/architecture.md:157)
- **The IPC stack ↔ established decisions**: one endpoint per wrapper, SQOS Identification open on Windows ([Message Broker / IPC]) (.andromeda/architecture.md:39)
- **State-file primitives and the store ↔ standard contracts and established decisions**: the `links` set follows the log, a failed append leaving it as it was, under the ndjson + `.lock` keystone (.andromeda/architecture.md:164), (.andromeda/architecture.md:35)
- **ViolaName and chrono ↔ conventions**: the name charset and the `ts` / `since` format (.andromeda/architecture.md:128), (.andromeda/architecture.md:130)
- **MCP tool list ↔ established decisions**: `link` and `unlink` are CLI-only human controls over the link topology ([MCP]) (.andromeda/architecture.md:43)
- **sysinfo liveness ↔ standard contracts**: the verbs' pre-check follows Session liveness's heartbeat and pid + start time rule (.andromeda/architecture.md:243)
- **The capability-ledger boundary ↔ cross-cutting patterns**: a link relies on no undocumented `claude` behaviour, so no ledger row is added (.andromeda/architecture.md:408)
