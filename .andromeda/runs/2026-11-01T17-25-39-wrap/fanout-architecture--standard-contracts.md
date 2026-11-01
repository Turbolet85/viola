proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 112
    section: "## Standard Contracts"
    change: "Replace the line with: - The exit-21 document's `detail` is one of six closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`; its `v` is 1 and its `error` `instance-unreachable` on all six."
    sidecar: "Exit 21's closed causes five to six, the sixth `socket-dir-refused` (`UnreachableCause::SocketDir`)."
    rationale: "Report Symbols / APIs names `UnreachableCause::SocketDir` as the sixth cause; Counts / qualifiers moved, pattern `causes`, moves five to six and gives the list words."
    basis: crates/viola-core/src/lib.rs:190
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 122
    section: "## Standard Contracts"
    change: "Replace «for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and a program not found» with «for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and a program not found»; the rest of the line stands."
    sidecar: "`refused_with`'s list of codes under `Launch.json` gains `socket-dir-refused`."
    rationale: "Report Symbols / APIs, `src/cmd/run.rs`: `start` calls `refused_with` with `socket-dir-refused`, and `revive`'s start shares it; pattern `revive-codes` moves `revive --json`'s exit-1 codes ten to eleven."
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 139
    section: "## Standard Contracts"
    change: "Replace «so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the server check» with «so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the socket-directory check on the recorded endpoint, then on its connected stream the peer euid decision, then the server check»; the macOS sentence and the rest of the line stand."
    sidecar: "The frame-writing client's order of checks gains the socket-directory check and the peer euid decision."
    rationale: "Report Symbols / APIs: `src/cmd/client.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame` run `check_endpoint_dir` after the endpoint read and `peer_admitted(client.server_euid())` ahead of the server check; pattern `mac-gap` keeps the statement that macOS compares no peer pid."
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 93
    section: "## Standard Contracts"
    change: "Replace «once its instance check (`trusted`) and its server check have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot or a failed server check sends nothing and the hook fails open)» with «once its instance check (`trusted`), its socket-directory check and its server check have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot, a refused socket directory or a failed server check, a refused peer euid among them, sends nothing and the hook fails open)»; the rest of the line stands."
    sidecar: "`hook.event`'s preconditions gain the socket-directory check and the peer euid decision."
    rationale: "Report Symbols / APIs, `src/cmd/hook.rs` `connect`: the same three checks in the same order, `NotSent::SocketDir` and `NotSent::ServerUnverified`, exit 0 and no frame."
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 63
    section: "## Standard Contracts"
    change: "Add the item: - After the instance's strict-modes check and its endpoint read, and before that comparison, each client holds the recorded endpoint's parent to `viola_state::strict::check_endpoint_dir`, connects, then reads `viola_channel::peer_admitted(client.server_euid())`: this order in `connect` of `src/cmd/client.rs` (its eight callers `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`), `connect` of `src/cmd/hook.rs` (`hook.event` and `hook.dialog`) and `frame` of `crates/viola-mcp/src/tools.rs`. `viola_channel::Client::connect` itself holds no check: `viola-channel` has no `viola-state` edge."
    sidecar: "New: the socket-directory check and the peer euid decision at the three client seams, in their order."
    rationale: "Report Symbols / APIs, the `client.rs`, `hook.rs` and `tools.rs` bullets; Reverted / negative API facts for the absent check inside `Client::connect`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 63
    section: "## Standard Contracts"
    change: "Add the item: - The socket-directory check is `viola_state::strict`'s. `check_socket_dir(dir)` makes one `symlink_metadata`, an absent path passing and an unstattable one `Refused::Unreadable`, then takes `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)`: a symlink `Refused::Symlink`, not a directory `Refused::NotADirectory` («a trusted state path is not a directory»), else `unix_read_verdict`. Off Unix `check_socket_dir` returns `Ok(())`. `check_endpoint_dir(endpoint)` is the check on a recorded endpoint's parent. The check never chmods: a refused directory stays as found."
    sidecar: "New: `check_socket_dir`, `check_endpoint_dir`, `socket_dir_verdict` and `Refused::NotADirectory`."
    rationale: "Report Symbols / APIs, the `viola_state::strict` bullet, and Deviations item 2 for `check_endpoint_dir`."
    basis: crates/viola-state/src/strict.rs:149
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 63
    section: "## Standard Contracts"
    change: "Add the item: - A refused socket directory writes no frame and reads the closed word `socket-dir-refused` at every client: a CLI frame verb through `CallError::SocketDirRefused` to `Unreachable::SocketDir` (exit 21, `during` `connect`, hint cause `socket-dir`, no `channel-request`); `viola hook` through `NotSent::SocketDir` and `Asked::SocketDir` (exit 0, empty streams, one `hook-decision` at WARN, `detail` `socket-dir-refused`); MCP `frame` through `result::unreachable(UnreachableCause::SocketDir)` (no line, `isError` the record)."
    sidecar: "New: the reading of a refused socket directory on the CLI, hook and MCP seams."
    rationale: "Report Symbols / APIs for the three seams; Coverage of new surfaces for the instrumentation; Decisions & corrections for the founder's Q1 word, `inputs#I2`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 63
    section: "## Standard Contracts"
    change: "Add the item: - A peer euid `peer_admitted` refuses writes no frame and reads as the failed server check: `CallError::ServerUnverified` at a CLI frame verb, `NotSent::ServerUnverified` at `viola hook`, `UnreachableCause::ServerVerify` at MCP `frame`. So `server-verify-failed` holds on Linux, Windows and macOS, where it means an euid mismatch alone. Windows reports no euid: the DACL and the pid comparison stay the controls there."
    sidecar: "New: a refused peer euid reads `server-verify-failed`, whose reach now includes macOS."
    rationale: "Report Symbols / APIs for the three seams; Counts / qualifiers moved, pattern `verify-word`, for the reach; Decisions & corrections for the founder's Q2 word, `inputs#I2`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 63
    section: "## Standard Contracts"
    change: "Add the item: - The wrapper admits each connection on its accept thread: `Server::serve` calls `serve_admitting(dispatch, peer_admitted)`, and `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped before its worker thread and before any frame, reading end-of-file, writing no line and nothing on the terminal. The decision is dependency-injected (`type Admission = fn(Option<u32>) -> bool`), with no env, flag or feature seam."
    sidecar: "New: the wrapper's drop of a refused peer in `accept_loop`."
    rationale: "Report Symbols / APIs, the `crates/viola-channel/src/server.rs` bullet; Coverage of new surfaces, the wrapper's drop."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 122
    section: "## Standard Contracts"
    change: "Add the item: - `run`'s start and `revive`'s, which shares it, take `socket_dir_ready(endpoint)` (`src/cmd/run.rs`, with its own off-Unix form) before any snapshot, child or `Server::bind`: `check_endpoint_dir`, the parent created only when absent, then the check again, so a race's loser reads the same. A failed create and a refused check read the one code `socket-dir-refused` at exit 1, the human pair from `refuse_socket_dir(name)`; a `Server::bind` failure stays what it was."
    sidecar: "New: `socket_dir_ready` and `refuse_socket_dir` on the start path."
    rationale: "Report Symbols / APIs, the `src/cmd/run.rs` bullet; Decisions & corrections, the leans the P5 word took, for the one code."
    basis: src/cmd/run.rs:578
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 140
    section: "## Standard Contracts"
    change: "Add the item: - The peer euid decision is `viola_channel`'s. `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where reported; `peer_admitted(observed)` is that decision over this process's `own_euid()` and `PEER_EUID_REPORTED` (`cfg!(unix)`). `Client::server_euid` and `TokioClient::server_euid` read the connected peer through `peer_euid_of` (`peer_creds().ok()?.euid()` on Unix, none on Windows), writing and logging nothing."
    sidecar: "New: `peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED` and the two `server_euid` readers."
    rationale: "Report Symbols / APIs, the `viola_channel` bullet; Deviations item 1 for the name `peer_euid_of`."
    basis: crates/viola-channel/src/lib.rs:20
note: "D-arch-decisions fires nothing (no dependency, runtime or crate moved); no D-retired, this file names neither the retired owner nor `viola list`'s two rules; the macOS gap wording stays per pattern `mac-gap`; the body's own five-cause and ten-code lists under CLI `--json` output lie outside this file."
