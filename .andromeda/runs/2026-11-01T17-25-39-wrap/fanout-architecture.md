proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts
    change: "In CLI --json output, the exit-21 document's <cause> list reads six words: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`)."
    sidecar: "Standard Contracts: exit 21's closed causes extended with `socket-dir-refused`, the sixth."
    rationale: "The report's Symbols / APIs adds `UnreachableCause::SocketDir` with `as_str` `socket-dir-refused`, and Counts / qualifiers moved has the closed causes go five to six (pattern `causes`, a list of the five extends); the §Standard Contracts list still holds five."
    basis: "crates/viola-core/src/lib.rs:190"
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The `viola-core` row's `UnreachableCause` list reads `StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word from `as_str`, no serde path."
    sidecar: "Project directory structure: `UnreachableCause` gains `SocketDir` on the viola-core row."
    rationale: "The same moved fact as the exit-21 list: the report's Symbols / APIs names `viola_core::UnreachableCause::SocketDir`, the sixth cause, no serde path; this key row enumerates the five."
    basis: "crates/viola-core/src/lib.rs:201"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The `client.rs` row reads: `connect`, opened by the eight CLI frame verbs (send, wait, last, answer, pause, release, link, unlink), takes the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check; the closed `Unreachable` reads `Dead` · `Unwrapped` · `Lost` · `ServerUnverified` · `SocketDir`, a refused directory reading `CallError::SocketDirRefused` and a refused euid `CallError::ServerUnverified`."
    sidecar: "Project directory structure: client.rs `connect` holds the socket-directory and peer-euid checks; `Unreachable` gains `SocketDir`."
    rationale: "The report's Symbols / APIs for `src/cmd/client.rs` `connect` gives these checks, this order, the 8 callers and `Unreachable::SocketDir`; the key row names four `Unreachable` variants and a connect of the live endpoint, the server identity check and the client alone."
    basis: "src/cmd/client.rs:173-175"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts
    change: "In CLI --json output, the `viola revive --json` exit-1 `<code>` list reads eleven codes, `socket-dir-refused` added beside `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`."
    sidecar: "Standard Contracts: `revive --json`'s exit-1 codes extended with `socket-dir-refused`, ten to eleven."
    rationale: "The report's Symbols / APIs has `start` call `refused_with(launch.json, socket-dir-refused, …)`, shared by `revive`'s start, and Counts / qualifiers moved takes the codes ten to eleven (pattern `revive-codes`, each list that holds all ten extends)."
    basis: "src/cmd/run.rs:822-829"
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The IPC endpoints Linux / macOS row also says: `viola run` and `viola revive`'s start check the socket directory before any snapshot, child or bind and create it only when absent; the eight CLI frame verbs, `viola hook` and `viola mcp`'s frame tools check the recorded endpoint's directory (`check_endpoint_dir`) before connecting, and admit only a peer of their own euid; the wrapper drops a client of another euid in `accept_loop` before any frame. A refused directory reads `socket-dir-refused` and stays as found, never chmodded."
    sidecar: "Occupied Resources: the Unix socket row records the verified per-user directory and the peer euid checks on both sides."
    rationale: "The report's Symbols / APIs lands the directory check at the bind and the three client seams and the euid decision on both sides; its Expected amendments carry §Occupied Resources (search `socket-dir`, `peer-euid`); the body row still states only the directory's mode and that other processes connect to the recorded endpoint."
    basis: "crates/viola-state/src/strict.rs:149-168"
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The `hook.rs` row's `connect` reads: the snapshot through the replay, a replayed one unreachable, then `check_endpoint_dir`, the client and `peer_admitted(client.server_euid())` ahead of the server identity check, in the CLI's order; a refused directory is `NotSent::SocketDir` / `Asked::SocketDir` and a refused euid `NotSent::ServerUnverified`, exit 0 with empty streams, no frame and one `hook-decision` at WARN."
    sidecar: "Project directory structure: hook.rs `connect` holds the socket-directory and peer-euid checks."
    rationale: "The report's Symbols / APIs for `src/cmd/hook.rs` `connect` states the same three checks in the same order and these outcomes; the key row describes that connect by the snapshot read alone."
    basis: "src/cmd/hook.rs:231-233"
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The `viola-state` row's `strict` list adds `check_socket_dir`, `check_endpoint_dir` and the plain `socket_dir_verdict`, with the fixed-message `Refused::NotADirectory`."
    sidecar: "Project directory structure: the socket-directory check registered on the `strict` list."
    rationale: "The report's Symbols / APIs lands these under `viola_state::strict`; the key row enumerates the module's checks and names none of them, an unregistered symbol by the detector's check."
    basis: "crates/viola-state/src/strict.rs:183-198"
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The `viola-channel` row adds the peer euid decision: `peer_euid_admitted`, `peer_admitted` and `PEER_EUID_REPORTED`, `Client::server_euid` and `TokioClient::server_euid`, and `accept_loop`'s drop of a refused stream before any frame."
    sidecar: "Project directory structure: viola-channel's peer euid decision registered."
    rationale: "The report's Symbols / APIs re-exports these from viola-channel and adds the wrapper's drop in `crates/viola-channel/src/server.rs`; the key row carries no peer-identity API, and the check treats a new API absent from the registry as drift."
    basis: "crates/viola-channel/src/lib.rs:20-22"
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The Filesystem list gains a row: Test-only, outside the viola home: the test-owned socket roots under `/tmp`, `tests/support/home.rs` `SocketRoot`, given to a wrapper by `Wrapper::boot_on_socket_root` and `first_life_on` through `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`)."
    sidecar: "Occupied Resources: the test-owned socket roots registered."
    rationale: "The report's Expected amendments names the test-owned socket roots under `/tmp` and their row in §Occupied Resources, and its Symbols / APIs lists the helpers; the body registers each other test-only directory outside the home and not these."
    basis: "tests/support/home.rs:585-609"
note: "D-arch-decisions read clean (Dependencies none, `viola-channel`'s normal graph tokio-free) and the Retired items hold no standing passage in the body or key files; the owed clauses, the off-macOS exception, the two home checks and the five-cause row sit in architecture-detail, the cascade's; `crate-dependency-direction`'s `libc` purpose on viola-channel may widen for `own_euid`, which the report does not state, so it is not proposed."
