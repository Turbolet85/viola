# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · password hashing is N/A (no accounts, Personal v1) and the session touches no account or credential code
authentication-authorization#1.2 · no · the nine served channel methods and the hook.event notification are unchanged (no channel method changed) and the MCP stdin cap is not an IPC method
authentication-authorization#1.3 · no · the Windows listener's protected user and SYSTEM DACL, accept_remote(false) and inheritable(false) are untouched (no IPC access-control change)
authentication-authorization#1.4 · no · the Unix per-user 0700 socket directory, its lstat check and the 0600 socket chmod are untouched (no endpoint change)
authentication-authorization#1.5 · no · the Unix peer-creds euid check on accept is untouched (no peer check changed)
authentication-authorization#1.6 · no · the client server verification and its check order before every frame, the viola mcp tools included, are unchanged (no MCP tool, endpoint, pid or start-time check changed; the stdin cap wraps only rmcp's input)
authentication-authorization#1.7 · no · the release-from-driver guard in the wrapper's channel dispatch is untouched and the session adds no release path
authentication-authorization#1.8 · no · reached file fs.rs changed only replace_private_shared's compare read (take(bytes.len() + 1)), and the report shows its write path, callers, 0600/0700 modes, persist and place_private_new unchanged, so the claim holds on the changed file
authentication-authorization#1.9 · no · the own-executable checked_exe read is re-read and true (the census only names it as no external input), and check_home_tree and TreeRefused are unchanged (home strict-modes rows untouched)
authentication-authorization#2.1 · no · the Windows remote-client rejection witness and its two control pipes are untouched (no listener change)
authentication-authorization#2.2 · no · the Windows anonymous-open denial witness and its control are untouched (no listener DACL change)
authentication-authorization#3.1 · no · the observed server pid read (Client::server_pid, TokioClient::server_pid) is untouched (no pid check changed)
authentication-authorization#3.2 · no · the ServerVerdict, verify_server and live_endpoint order are unchanged, the viola mcp path included (no MCP tool, endpoint, pid or start-time check changed)
authentication-authorization#3.3 · no · link and unlink's instance check, liveness pre-check and server check are untouched (no link or unlink code is reached)
authentication-authorization#3.4 · no · the CLI frame-verb refusal (exit 21, instance-unreachable, no frame written) is untouched (no exit code or error variant changed)
authentication-authorization#3.5 · no · the exiting wrapper's Windows pipe case is a CARRY owed to Windows-only live measurements and no hook or pipe code changed
authentication-authorization#3.6 · no · the macOS server-verification cases and their compiled-out negatives are unchanged (the CARRY is owed to Unix endpoint and home hardening)
authentication-authorization#3.7 · no · the link and unlink tests (channel_links, cli_links, security_negatives_server_identity) are untouched and no link code is reached
