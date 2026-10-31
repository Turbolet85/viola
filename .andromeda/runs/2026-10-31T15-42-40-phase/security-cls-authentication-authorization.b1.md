# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · password hashing is N/A, no accounts or passwords exist, so the chunk's code and tests hold nothing for it
authentication-authorization#1.2 · no · the all-or-nothing capability model over the OS-user principal is not something the chunk's code changes; the IPC gates it names are carried by their own rows
authentication-authorization#1.3 · no · the Windows listener DACL is server-side listener construction, which the scope leaves unchanged
authentication-authorization#1.4 · yes · the Unix socket-directory check failing refuses `run`, reports `instance-unreachable` (exit 21) to CLI and MCP clients and fails `hook` open, which the chunk's exit-21 cause split must honour
authentication-authorization#1.5 · no · the Unix peer euid check is on the server accept path, which the chunk does not touch
authentication-authorization#1.6 · yes · the client-side server verification (pid, start time, euid, the mismatch or unverifiable verdict before any frame, no fallback connect) is what the new server-verification and endpoint-lost codes must separate without adding a frame or connect
authentication-authorization#1.7 · yes · the `release-from-driver` wire body kept through `viola_channel::sanitised_fault` for `data.reason` exactly `release-from-driver` is a passage the new codes must not break
authentication-authorization#1.8 · yes · the strict-modes check's place and refusal shape (no log, no line, no snapshot read, home mode as found, exit 21 or 1 with the pair or the `--json` document) across `run`, `revive`, `list`, `mcp`, `plugin install`, `verify`, the frame verbs, `link`/`unlink` and the hook arms binds the codes the chunk gives those refusals
authentication-authorization#1.9 · yes · the start-tree check through `checked_exe` and `refuse_tree` (the `unable:` / `hint:` pair, `revive --json` `strict-modes-failed`, no log, no child) is in the chunk's reached `run.rs` and `revive.rs` edits
authentication-authorization#2.1 · no · the Windows remote-client rejection witness tests the listener's pipe bit, which the chunk does not touch
authentication-authorization#2.2 · no · the Windows DACL anonymous-open witness tests the listener, which the chunk does not touch
authentication-authorization#3.1 · yes · `Client::server_pid` and `SERVER_PID_REPORTED` (false on macOS) are the peer-pid reading the chunk's server check relies on, and the reading writes and logs nothing
authentication-authorization#3.2 · yes · the closed `ServerVerdict { Matches, Mismatch, Unverifiable }` and `live_endpoint` ordering (liveness pre-check before the server check, only `Matches` writes a frame) is what the mismatch, replayed-snapshot and cannot-be-checked causes map onto
authentication-authorization#3.3 · yes · `link.rs` and `unlink.rs` are reached whole, and their order (strict-modes check via `refuse_untrusted`, liveness pre-check, server check, then one frame) is the client order the chunk's codes must keep
authentication-authorization#3.4 · yes · the CLI refusal's `instance-unreachable` document with null `detail` and the `unable` / `hint` line for a mismatch or unverifiable server are the exact external shapes the new server-verification code replaces or sits beside
authentication-authorization#3.5 · no · the exiting-wrapper pipe passing the hook check is `hook.rs` behaviour, which the chunk does not reach, and its carry belongs to another route entry
authentication-authorization#3.6 · yes · the macOS server-check gap and the `#![cfg(not(target_os = "macos"))]` compile-out of `security_negatives_server_identity.rs` bind the chunk's server-verification code and the assertions moved in that file, which a server-verification code must not claim on macOS
authentication-authorization#3.7 · yes · the `link` / `unlink` server-identity table rows (no frame, exit 21 in both modes) and `cli_links.rs` are where the moved document's assertions land, and must still prove no frame and no line
