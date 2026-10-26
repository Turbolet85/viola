# Task — security · 2026-10-25-windows-endpoint-admission

## The chunk
Brings the Windows named-pipe listener of `viola-channel` from partly landed to fully asserted on the Windows CI leg: admission for the owning user's SID and SYSTEM only, remote (network) pipe clients rejected, the first-pipe-instance behaviour kept, handles non-inheritable.
New assertions: (a) the listener's remote-client rejection read back from the pipe's mode, not only a doc comment; (b) first-pipe-instance as a listener property (a second listener on a held name fails), apart from the CLI squat refusal; (c) the DACL's effect, an open by a principal outside the two ACEs denied, only where a Windows CI runner can measure it without a second interactive user.
Re-verifies at HEAD, never re-builds: the protected DACL read back as two allow ACEs, non-inheritable handles on both ends, the squatted-name exit-1 start refusal of `viola run`, and the CARRY's test-file coordinate for the DACL read-back.
Folded obligation: the `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 log capture made reliable under a plain `cargo test` (libtest threads), or run three times green there.
No wire contract, exit code or refusal word changes (`squatted-name`, exit 1 stand); no boundary is widened.

## Reached files
- crates/viola-channel/src/server.rs · edits the remote-client rejection doc comment and the Windows listener options
- crates/viola-channel/src/server/win.rs · edits the listener build and its unit tests (`owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system`, `pipe_handles_on_both_ends_are_not_inheritable`, new remote-reject, first-instance and DACL-effect tests)
- crates/viola-channel/src/endpoint.rs
- tests/security_negatives_channel.rs
- src/cmd/run.rs · edits `refuse_squatted`
- tests/path1_run_start.rs
- tests/cli_instance_state.rs
- src/run/send/budget.rs · edits the `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` test and its log capture helper

## Needs
- IPC access control (Windows): the listener's explicit protected DACL granting the user SID and SYSTEM only, the logon SID deliberately not added
- Remote pipe clients rejected (interprocess `accept_remote(false)`, the `PIPE_REJECT_REMOTE_CLIENTS` default), now asserted by reading the pipe's mode back
- First-pipe-instance behaviour kept on the listener, so a name another process already holds fails the bind and refuses `viola run` at start (`squatted-name`, exit 1)
- Handle inheritance: channel listener and connection handles stay non-inheritable on both ends, so they never leak into the `claude` child
- The ban on creating an interprocess Windows listener without `ListenerOptionsExt::security_descriptor` (the default DACL grants Everyone and anonymous)
- The ban on enabling interprocess `try_overwrite`
- The ban on making channel handles inheritable
- The library set for Windows IPC: interprocess 2.4.4 `security_descriptor`, windows-sys 0.61.2 (`GetTokenInformation(TokenUser)`, `ConvertSidToStringSidW`, the SDDL-to-descriptor conversion under `Win32_Security_Authorization`); interprocess `SecurityDescriptor::deserialize` stays unused
- Threat model, local IPC boundary: the endpoint is a code-execution interface, its target principal every process of the same OS user and no other, its pipe name predictable in a shared namespace (squatting)
- Capability-based all-or-nothing access: any caller passing the OS check gets the full surface, so the DACL and remote rejection are the whole Windows admission control
- No control switched off by `config.json`, a `VIOLA_*` env var or a CLI flag: a test that measures the DACL effect or the remote rejection adds no env seam or flag that disables the DACL (a new seam needs its own Decisions Log entry)
- No new listener and no new channel method without a Security Decisions Log entry: the chunk asserts the existing listener only
- Error sanitization through `ChannelError`: any bind or first-instance failure it surfaces keeps a fixed `Display` message, no pipe path or absolute path in an external error
- The squatted-name start refusal stays a refusal of `viola run`'s own start, its word and exit unchanged, never a block on a human keystroke in a running wrapper
- A test-side principal or second listener built to measure denial is created only inside the test, never under the product's home with relaxed modes
- Dependency pinning if a windows-sys feature or a dev-dependency is added for the read-back: through `[workspace.dependencies]` pins and `cargo deny check`, no C-building crate
- The supply-chain red (RUSTSEC-2026-0386 against `tokio 1.53.1`) is read red on this chunk's CI runs: no `deny.toml` ignore is added and no ban widened to reach green
- The budget gate's `budget-paused` send refusal as exercised by its test: only the test's log capture of the `send-refused` line changes; the gate's fail-open `budget.json` read and thresholds keep their behaviour
- Windows-only live measurement: a DACL-effect reading no CI runner can take is owed to that route entry, not claimed as measured

## Not needed
- Client-side server verification before any frame (the SQOS identification-level client open, the pid and start-time check, the dated interim gaps for `hook.event`, `send`, `wait`/`last`, `answer`, `pause`/`release`, `hook.dialog` and `viola mcp`): the next entry "Server verification before any frame"
- IPC access control (Unix): the per-user 0700 socket directory, the chmod 0600 after bind, the abstract-namespace ban: the entry "Unix endpoint and home hardening"
- IPC server-side peer euid check (Unix)
- `~/.viola/` access control and the home strict-modes check, Windows home DACL outside `%USERPROFILE%`, `replace_private`, the pinned binary re-hash: the entry "Home and code-bearing file integrity"
- Paste text and dialog free-text validation (`validate_paste_text`, `control-character`, `empty-text`)
- Channel frame bounds (`MAX_FRAME`, serde depth, `v` check, `-32600` / `-32602`): framing is unchanged
- Hook stdin parsing, the hidden capture arms and the hook fail-open contract
- MCP tool inputs, rmcp feature bans and the MCP `isError` shape
- Statusline source and shell-out, `budget.json` write by `hook statusline`
- DLL search restriction and the sideloaded ConPTY companions' re-hash
- `viola verify`'s interactive children, Run E's resume argv, and the `--record` fixture scrub
- The NEVER-log floor for R8-stripped `CLAUDE*` values and secret management
- The tokio RUSTSEC-2026-0386 lock bump and the critical CVE response SLA: owned by the new route entry placed after this one
- Driver-originated `release` with `from` and the dialog-decision stamp gate
