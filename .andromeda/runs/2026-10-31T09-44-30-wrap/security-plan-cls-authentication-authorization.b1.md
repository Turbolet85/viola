# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · no accounts or passwords, and no reached file holds a hashing rule
authentication-authorization#1.2 · yes · stale · the nine methods' per-method params checks sit in reached viola-core and viola-channel files, unconfirmed on the changed files
authentication-authorization#1.3 · yes · stale · the protected listener DACL and accept_remote(false) bind live in reached viola-channel server.rs, unconfirmed on the changed file
authentication-authorization#1.4 · yes · stale · the 0700 socket directory and 0600 chmod live in reached viola-channel server.rs, unconfirmed on the changed file
authentication-authorization#1.5 · yes · stale · the accept-time peer_creds check lives in reached viola-channel server.rs, unconfirmed on the changed file
authentication-authorization#1.6 · yes · stale · the client-side server verification lives in reached viola-channel client.rs, src/cmd/client.rs and viola-mcp, unconfirmed on the changed files
authentication-authorization#1.7 · yes · add-to · the release-from-driver body stands and the channel-response line now carries that word as detail (F6), reached in viola-channel lib.rs
authentication-authorization#1.8 · yes · stale · the statusline check and the budget gate sit in reached src/cmd/hook/statusline.rs and src/run/budget.rs, and only the viola-state strict-modes rules are confirmed untouched
authentication-authorization#1.9 · yes · stale · checked_exe and refuse_tree live in reached src/cmd/run.rs, and the check_home_tree rule itself is untouched
authentication-authorization#2.1 · yes · stale · the REJECT_REMOTE_CLIENTS bind lives in reached viola-channel server.rs
authentication-authorization#2.2 · yes · stale · the protected DACL that the anonymous-open test asserts lives in reached viola-channel server.rs
authentication-authorization#3.1 · yes · stale · server_pid lives in reached viola-channel client.rs
authentication-authorization#3.2 · yes · stale · the verdict's callers live_endpoint, connect and viola-mcp's frame sit in reached src/cmd/client.rs and viola-mcp
authentication-authorization#3.3 · yes · stale · link and unlink's refuse_untrusted and connect sit in reached src/cmd/client.rs
authentication-authorization#3.4 · yes · stale · the exit-21 refusal path lives in reached src/cmd/client.rs (connect, refuse_untrusted)
authentication-authorization#3.5 · no · its rule lives in src/cmd/hook.rs and sysinfo, neither reached, and the Windows CARRY stands
authentication-authorization#3.6 · no · the macOS CARRY and its cfg-gated tests live in unreached files and a CARRY route entry
authentication-authorization#3.7 · yes · stale · the bad-name frames' no-line read sits on the -32602 log path that F6 split in reached viola-channel files
