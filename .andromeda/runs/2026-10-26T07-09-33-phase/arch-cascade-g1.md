# arch · cascade read · group 1 of 3

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:11) The server verification added to `hook` and every CLI verb must run on std threads and blocking I/O, with a Tokio runtime only inside `viola mcp`'s tools.
  > | Backend framework (concurrency model) | Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp requires it |
  (.andromeda/architecture.md:13)
  > std threads with Tokio built only inside `mcp`
  (.andromeda/architecture.md:442)
  > A sync std-thread core; Tokio 1.53.3 only in `mcp`
- (.andromeda/architecture-detail/stack-and-technologies.md:15) The server-identity read the chunk adds to the sync client, the Tokio client and the endpoint must keep the Windows SQOS Identification `open_handle` open (never interprocess's default connect), the Tokio flavour confined to `viola-channel`'s feature-gated `TokioClient` used by `viola-mcp` alone, and take its OS facts from windows-sys and libc.
  > the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`); on Windows the client opens the pipe itself with windows-sys 0.61.2 `CreateFileW(SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION \| FILE_FLAG_OVERLAPPED)` and adopts the handle with `local_socket::Stream::try_from`, never interprocess's default connect. The server's pipe DACL is converted from SDDL by windows-sys (`Win32_Security_Authorization`); on Unix, libc gives the uid for the per-user socket directory and the socket is chmod 0600 after the bind
  (.andromeda/architecture.md:444)
  > IPC: interprocess 2.4.4 local sockets, one endpoint per `viola run`, JSON-RPC 2.0 over ndjson with `v` + sender version in every `params`.
- (.andromeda/architecture-detail/stack-and-technologies.md:17) The clients must keep reading a snapshot whose `v` is unsupported as absent (exit 21 / fail open) behind the held CARRY; moving them onto the replay this row names is the founder's fork, never leaned.
  > a snapshot with an unsupported `v` is rebuilt by replaying the log
- (.andromeda/architecture-detail/stack-and-technologies.md:18) Should P4 take the strict-modes half here, the snapshot the verification reads must pass the existing strict-modes check built on libc (Unix modes and owner) and windows-sys (owner + DACL), with no new check technology.
  > the strict-modes check and private-dir creation through libc (Unix modes and owner) and windows-sys (owner + DACL, and the protected user + SYSTEM DACL on a home created outside the user's profile, `Win32_UI_Shell` `GetUserProfileDirectoryW`; a DACL that cannot be set fails the creation)
- (.andromeda/architecture-detail/stack-and-technologies.md:23) The check's comparison must read the snapshot's recorded `started_at` as an RFC 3339 UTC timestamp through chrono against the connected server's start time.
  > | Timestamps | chrono 0.4.45 | RFC 3339 UTC, millisecond precision. Already in the tree through rmcp |
  (.andromeda/architecture.md:448)
  > Timestamps: chrono 0.4.45, RFC 3339 UTC with milliseconds and `Z`, field `ts` / `*_at`.
- (.andromeda/architecture-detail/stack-and-technologies.md:26) The connected server's start time must come from sysinfo 0.39.6 by pid + process start time, the stack's one process-liveness reader, with no second process-inspection crate.
  > | Process liveness | sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file |
- (.andromeda/architecture-detail/stack-and-technologies.md:27) The edit to `viola mcp`'s `send` / `wait` / `last` / `answer` tools must leave the release graph's rmcp features exactly `server` + `transport-io`, and any E2E the plan adds for their `instance-unreachable` outcome must use the test-only `viola-e2e`'s rmcp `client` alone, owning the `viola mcp` child.
  > The release graph's rmcp features are exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); the test-only `viola-e2e` takes rmcp `client` alone, never `transport-child-process` (which kills the server 3 s after a close), its E2E owning the `viola mcp` child and awaiting its exit
- (.andromeda/architecture-detail/stack-and-technologies.md:32) Any dependency the identity read adds to `viola-channel` or `viola-state` must pass cargo-deny's C-crate and feature bans and the per-sync-crate tokio ban, and `hook`'s mismatch path must stay inside the workspace print bans.
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`
  (.andromeda/architecture-detail/stack-and-technologies.md:32)
  > cargo-deny 0.20.2 (advisories, licences, sources, bans: C crates, telemetry crates, feature bans; the tokio ban via `deny-sync.toml` per sync crate)
- (.andromeda/architecture.md:446) A mismatch or an unmade check must end each CLI verb on its typed exit code and leave `viola hook` exiting 0.
  > typed CLI exit codes; `viola hook` always exits 0.
- (.andromeda/architecture.md:6) The verification must hold the endpoint's owner to the wrapper the disk records, each wrapper the sole owner of its endpoint and every client a separate process.
  > Every wrapper owns its own local-socket endpoint, whether it was started by `viola run` or by a `viola revive` whose preflight passed (both go through the one start path). Every other verb is a separate process that owns no endpoint

## Bindings
- **stack concurrency row ↔ cross-cutting Tokio containment and the concurrency decision**: the std-only verification on `hook` and the CLI verbs (.andromeda/architecture-detail/stack-and-technologies.md:11) is the same rule as the cross-cutting containment line (.andromeda/architecture.md:410) and the KEYSTONE's sole-root tokio ban (.andromeda/architecture.md:23), which the code-quality row's `deny-sync.toml` gate witnesses (.andromeda/architecture-detail/stack-and-technologies.md:32).
- **stack IPC row ↔ the IPC decision and the endpoint resources**: the SQOS `open_handle` and libc uid facts (.andromeda/architecture-detail/stack-and-technologies.md:15) bind with the IPC decision (.andromeda/architecture.md:39) and the Unix socket resource the client connects through (.andromeda/architecture.md:263).
- **stack process-liveness and timestamp rows ↔ the session-liveness decision and the instance snapshot contract**: sysinfo's pid + start time (.andromeda/architecture-detail/stack-and-technologies.md:26) and chrono's RFC 3339 read (.andromeda/architecture-detail/stack-and-technologies.md:23) compare against the snapshot's `pid` / `started_at` (.andromeda/architecture.md:157) under the liveness rule (.andromeda/architecture.md:59, .andromeda/architecture.md:243).
- **stack migration row ↔ the snapshot envelope contract**: the replay of an unsupported `v` (.andromeda/architecture-detail/stack-and-technologies.md:17) is the envelope's rule (.andromeda/architecture.md:152) that the held CARRY keeps as absent for every client.
- **stack state-file primitives row ↔ module boundaries**: the strict-modes check's libc / windows-sys basis (.andromeda/architecture-detail/stack-and-technologies.md:18) and the identity read stay in `viola-state` / `viola-channel` under the crate layout (.andromeda/architecture.md:67).
