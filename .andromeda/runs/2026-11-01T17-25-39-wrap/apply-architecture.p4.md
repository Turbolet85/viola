# Apply — architecture · part 4 of 4

## Verdicts
- fanout-architecture.md 2 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:58#1
- fanout-architecture.md 3 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:32#1
- fanout-architecture.md 6 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:31#1
- fanout-architecture.md 7 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:70#1
- fanout-architecture.md 8 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:64+#1

## Edits
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:58
  old: «`EndpointLost` · `ServerVerify`, each word from `as_str`»
  new: «`EndpointLost` · `ServerVerify` · `SocketDir`, each word from `as_str`»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:32
  old: «`connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) opens (the live endpoint, then the server identity check, then the client), the closed `Unreachable` (`Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, each an `UnreachableCause` through `cause()`) and `CallError`,»
  new: «`connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) and `link` / `unlink` open (the live endpoint, then `check_endpoint_dir` on it, then the client, then `peer_admitted(client.server_euid())`, then the server identity check), the closed `Unreachable` (`Dead` · `Unwrapped` · `Lost` · `ServerUnverified` · `SocketDir`, each an `UnreachableCause` through `cause()`) and `CallError` (a refused directory `CallError::SocketDirRefused`, a refused euid `CallError::ServerUnverified`),»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:31
  old: «(the snapshot through the replay, a replayed one unreachable)»
  new: «(the snapshot through the replay, a replayed one unreachable, then `check_endpoint_dir`, the client and `peer_admitted(client.server_euid())` ahead of the server identity check, in client.rs's `connect` order; a refused directory `NotSent::SocketDir` / `Asked::SocketDir`, a refused euid `NotSent::ServerUnverified`: exit 0, empty streams, no frame, one `hook-decision` at WARN)»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:70
  old: «`check_present`, `check_stamps`), the budget reading's write»
  new: «`check_present`, `check_stamps`, `check_socket_dir`, `check_endpoint_dir` and the plain `socket_dir_verdict`, with the fixed-message `Refused::NotADirectory`), the budget reading's write»
- after: .andromeda/registries/contracts/architecture/project-directory-structure.md:64
  anchor: «│   ├── viola-channel/          # JSON-RPC 2.0 ndjson over interprocess local sockets; tokio_client.rs: the Tokio client behind the `tokio` feature»
  add: «│   │                           #   the peer euid decision (`peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED`), the server's euid (`Client::server_euid`, `TokioClient::server_euid`), and server.rs's `accept_loop`, dropping a refused stream before any frame»

## Patterns
[[pattern]]
id = "arch4-01"
fixed = '`EndpointLost` · `ServerVerify`, each word'

[[pattern]]
id = "arch4-02"
regex = 'then the server identity check, then the client|`Lost` · `ServerUnverified`, each'

[[pattern]]
id = "arch4-03"
fixed = 'through the replay, a replayed one unreachable)'

[[pattern]]
id = "arch4-04"
fixed = '`check_present`, `check_stamps`)'
