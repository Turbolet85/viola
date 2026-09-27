# arch extract

## Relevance
Relevant. This chunk creates the planned `viola-channel` crate and lands the wrapper-channel keystone (JSON-RPC 2.0 over ndjson, one local-socket endpoint per `viola run`) and the start-order bind.

## Constraints
- **Crate placement.** Code lives in `crates/viola-channel`, a flat `crates/` member named `viola-<area>` (per architecture §Established Decisions [Module Boundaries]; §Conventions Naming patterns → Crates). Its manifest dependencies must follow the planned direction: `viola-core`, interprocess, and windows-sys on Windows only (per architecture §Infrastructure Patterns → Crate dependency direction). Adding serde/serde_json/thiserror through `[workspace.dependencies]` fits the shared-deps line. Adding tracing and veil to `viola-channel` goes beyond the listed direction, so it needs a wrap amendment.
- **Tokio containment.** The server and client on the `run` path use std threads and blocking I/O. The Tokio client exists only behind a `tokio` feature that only `viola-mcp` enables (per architecture §Established Decisions [Concurrency / Backend Framework]; §Cross-cutting Patterns → Tokio containment). The crate joins `scripts/sync-crates.txt`, so it gets the sole-root `deny-sync.toml` ban and the CI job 3 featureless `cargo check` (per architecture §Infrastructure Patterns → Build system Dependency policy; target job 3). The plan records a limit: `viola-channel` false-fails as its own root once `viola-mcp` enables `tokio`. That has no subject until the MCP chunk.
- **Envelope.** JSON-RPC 2.0, hand-rolled with serde, with no jsonrpsee (per architecture §Established Decisions [API Style]):
  - Every `params` carries `v` and `sender` (`CARGO_PKG_VERSION`), with no handshake.
  - Refusals go in `result.refusal`. `error` is reserved for protocol faults, with the codes -32700, -32600, -32601, -32602 and -32603.
  - JSON-RPC ids are integers, monotonic per connection.
  - Hook events are notifications with no id.
  - (Per architecture §Conventions Interfaces and versioning; Error handling schema; §Standard Contracts "Wrapper channel frames".)
- **Mixed-version tolerance.** When `params.v` is higher than the wrapper supports, it answers `-32602` "unsupported protocol version" with `data: {supported, wrapper}`. Readers skip unknown fields and never use `deny_unknown_fields` on own formats (per architecture §Cross-cutting Patterns → Mixed-version tolerance; §Established Decisions [Validation]; §Conventions Protocol versioning).
- **Endpoint naming and location.**
  - The name is `viola-<h12>`: an FNV-1a 64-bit hash over `ViolaName` + `\0` + the absolute home path, hand-written in `viola-channel`. std `DefaultHasher` is forbidden (per architecture §Conventions Data model conventions → Endpoint name).
  - Windows uses `\\.\pipe\viola-<h12>`.
  - Unix uses `$TMPDIR/viola-<h12>.sock` (`/tmp` when `TMPDIR` is unset), resolved once and recorded as `endpoint`. Other processes connect to the recorded path. Abstract-namespace names are never used (per architecture §Occupied Resources → IPC endpoints).
  - Conflict: the scope's "Unix per-user socket directory" does not match this registered path. P4 must reconcile it, or the wrap must amend §Occupied Resources.
- **Windows client open.** The client opens the pipe itself: windows-sys `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)`, then adopts the handle with `local_socket::Stream::try_from`. It never uses interprocess's default connect, and a non-overlapped handle hangs (per architecture §Stack and Technologies Wrapper IPC row; §Established Decisions [Message Broker / IPC]). The plan names only the windows-sys route. Picking the std `OpenOptions::security_qos_flags` route instead would change that row and needs an amendment.
- **Start order and arbiter.**
  - The exclusive bind sits after the version gate and before the first snapshot. A bind that finds the name taken is the same exit-1 refusal, so two concurrent starts cannot both win (per architecture §Established Decisions [Session Liveness]).
  - Once the endpoint exists, a name with an answering endpoint also refuses (`already-live`) (same section; §Conventions CLI exit codes `1`).
  - `endpoint` is written into `snapshot.json` once the bind succeeds. It is not recovered by replay (per architecture §Standard Contracts Instance snapshot; Snapshot envelope).
  - The plan requires that the child's first SessionStart finds a listening endpoint "from 'Wrapper channel' on". Whether the version gate step exists before the bind is research's question.
- **Error type and trust boundary.**
  - The error type is `ChannelError`, a `thiserror` enum, one per crate. The only ratified hand-written exception is `PtyError` (per architecture §Established Decisions [Error Handling]; §Conventions Rust error types). A thiserror enum with fixed `#[error("…")]` messages meets the scope's "fixed-`Display`". A hand-written impl would be a second exception and needs ratification.
  - The endpoint must not be connectable by another OS user. The enforcement mechanism is the security plan's (per architecture §Cross-cutting Patterns → Local endpoint trust boundary). Whether interprocess's default pipe DACL already satisfies this is research's question, and it feeds the scope's P4 fork.

## Patterns to follow
- **Frame line discipline.** One complete JSON object plus `\n` per single `write`, with multi-line text escaped on one line (per architecture §Conventions Data model conventions → ndjson line discipline). The plan already caps untrusted reads at `MAX_FRAME` for `config.json` (per architecture §Occupied Resources → Filesystem `config.json`). The same cap applies to frames.
- **Optional fields and wire enums.** Optional fields are `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`. JSON fields are snake_case, and enum values on the wire are kebab-case (per architecture §Conventions Naming patterns; Data model conventions). `RefusalReason` stays in `viola-core`.
- **Snapshot write.** The `endpoint` rewrite goes through the one shared `viola_state::fs::replace_private` helper, with no call-site temp + rename (per architecture §Established Decisions [Snapshot writer]).
- **Channel seam.** interprocess sits behind viola's own `channel` seam, which contains the single-maintainer risk (per architecture §Established Decisions [Message Broker / IPC]). The dispatch table follows the method set in §Conventions Channel methods. Unimplemented methods answer `-32601`.
- **Cross-platform branches.** Each OS branch (named pipe vs Unix socket) compiles and is tested on its own CI runner. The endpoint name stays hashed and short, within about 104 bytes on macOS (per architecture §Cross-cutting Patterns → Cross-platform discipline).

## Anti-patterns to avoid
- Tokio (direct or transitive) in the normal graph of `viola-channel` without its feature, or on any `run`/`hook`/channel-server path. Also no jsonrpsee (per architecture §Established Decisions [Concurrency], [API Style]).
- interprocess's default Windows connect, or a non-overlapped adopted handle (per architecture §Established Decisions [Message Broker / IPC]).
- `std::hash::DefaultHasher` for the endpoint name, `deny_unknown_fields` on channel frames, or Linux abstract-namespace sockets (per architecture §Conventions Endpoint name; [Validation]; §Occupied Resources → IPC endpoints).

## Contract bindings
- **Channel envelope ↔ security.** Frame bounds, SQOS client, the endpoint DACL/SDDL and the `accept_remote(false)` P4 fork.
- **Channel envelope ↔ tests.** `security_negatives_*` SQOS case, `MAX_FRAME` ± 1 proptest, and a fuzz target inside the separate `fuzz/` workspace, which depends on the product crates by path (per architecture §Occupied Resources → Workspace crates / Repository `fuzz/`).
- **Channel envelope ↔ obs.**
  - The obs-plan's `conn` field is additive under the no-bump rule (per architecture §Conventions Protocol versioning).
  - `run`'s process log goes to the home-level `diagnostics/run-<name>.ndjson`, and content-bearing detail goes only to the instance's `detail-run.ndjson`. While the child runs, `run` writes nothing to the terminal (per architecture §Cross-cutting Patterns → Diagnostic output channels).
- **Snapshot `endpoint` ↔ state/liveness.** Other processes read the recorded `endpoint` rather than rebuilding it (per architecture §Occupied Resources → IPC endpoints; §Standard Contracts Instance snapshot).
- **Sync-crate list ↔ CI.** `scripts/sync-crates.txt` feeds both CI job 3 and the `supply-chain` sole-root ban (per architecture §Infrastructure Patterns → CI/CD approach target job 3).

## Acceptance criteria contributions
- `crates/viola-channel` exists as a workspace member, inherits `[lints] workspace = true`, `license.workspace = true` and `publish = false`, and is listed in `scripts/sync-crates.txt`. The `deny-sync.toml` ban with it as sole root passes, and CI job 3's `cargo check -p viola-channel` passes without the `tokio` feature on all three OSes (per architecture §Infrastructure Patterns → Build system; §Inherited Defaults → Publishability).
- A request whose `params.v` exceeds the supported version gets exactly `{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"<CARGO_PKG_VERSION>"}}`. An unknown method gets `-32601`. A frame with extra unknown fields is accepted (per architecture §Standard Contracts "Wrapper channel frames"; §Cross-cutting Patterns → Mixed-version tolerance).
- Two concurrent `viola run` of one name in one home: exactly one binds. The other exits 1 with the start refusal and writes no snapshot. The winner's `snapshot.json` carries `endpoint` equal to `\\.\pipe\viola-<h12>` on Windows, or the resolved socket path on Unix, and `<h12>` matches the FNV-1a rule (per architecture §Established Decisions [Session Liveness]; §Conventions Endpoint name).
- On Windows the client connects only through the SQOS Identification + `FILE_FLAG_OVERLAPPED` handle adopted by `Stream::try_from`, and no call site uses interprocess's default connect (per architecture §Stack and Technologies Wrapper IPC row).

## Relevant amendment history
- **2026-09-25-security-prerequisites.**
  - Recorded the Windows SQOS client open (windows-sys `CreateFileW` + `FILE_FLAG_OVERLAPPED`, adopted by `Stream::try_from`; measured that a non-overlapped handle hangs) in the Wrapper IPC row and [Message Broker / IPC].
  - Added windows-sys (Windows only) to `viola-channel`'s planned deps.
  - Registered the test-only pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` outside the `viola-<h12>` namespace.
  - Added per-crate `0BSD` licence exceptions for `doctest-file` and `recvmsg`, which interprocess 2.4.4 pulls in.
  - Why: that chunk's measured SQOS spike, plus operator ratification of the exceptions.
- **2026-09-24-supply-chain-and-workflow-gates.** Moved the tokio ban to the sole-root `deny-sync.toml`, run per crate in `scripts/sync-crates.txt`, and recorded the `viola-channel` own-root false-fail limit once `viola-mcp` enables `tokio`. Why: `--exclude` false-fails under feature unification (measured).
- **2026-09-27-instance-state-and-start-order.**
  - Landed the start order with the version-gate and bind steps marked "later". The bind is assigned to "Wrapper channel", which also owns the guarantee that the first SessionStart finds a listening endpoint.
  - Made `endpoint?` a snapshot field written once bound.
  - Added `replace_private_shared` for the concurrent identical writes to the pinned copy and plugin files.
  - Why: that chunk's operator/overseer rulings. It is the source of this chunk's arbiter CARRY and the `replace_private_shared` recurrence watch.
- **2026-09-24-diagnostics-plane.** Registered the `MAX_FRAME`-capped `config.json` read and `run`'s home-level codes-only log versus instance detail files. Why: the shipped diagnostics plane. Frames reuse the same cap and destinations.
- **2026-09-24-workspace-tree-and-code-graph-planes, 2026-09-25-pty-wrapper-on-windows, 2026-09-27-instance-state-and-start-order.** Each landed crate was added to §Occupied Resources "Landed so far", the rust code-graph plane member list, the Licence inheritance list and the sync-crate note. Why: registry kept true per landing. `viola-channel` must be added to all four at this wrap.
