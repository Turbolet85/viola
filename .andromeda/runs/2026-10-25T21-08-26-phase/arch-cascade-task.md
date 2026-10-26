# Task — arch · 2026-10-25-windows-endpoint-admission

## The chunk
Asserts on the Windows CI leg the Windows named-pipe listener's admission: only the owning user's SID and SYSTEM, a remote (network) pipe client rejected, first-pipe-instance kept, handles non-inheritable.
Builds what chunk 2026-09-27-wrapper-channel left unasserted: a test reading the pipe's reject-remote mode back, a second listener on a held name failing as a listener property, and an open by a principal outside the two ACEs denied where a Windows CI runner can measure it (else owed to "Windows-only live measurements").
Re-verifies at HEAD, never rebuilds: the protected DACL read back as two allow ACEs, non-inheritable handles on both ends, the squatted-name exit-1 start refusal, and the CARRY's test-file coordinate.
Fixes or re-runs three times under plain `cargo test` the budget-paused send test case whose log capture missed its `send-refused` line under libtest threads.
No wire contract, exit code or refusal word changes; no boundary is widened.

## Reached files
- crates/viola-channel/src/server.rs · edits the remote-client rejection (today a doc comment naming interprocess's reject-remote default)
- crates/viola-channel/src/server/win.rs · edits its Windows listener unit tests (the DACL read-back, the non-inheritable handles test, new remote-rejection and first-instance cases)
- crates/viola-channel/src/endpoint.rs
- tests/security_negatives_channel.rs
- src/cmd/run.rs · edits `refuse_squatted` (possibly)
- tests/path1_run_start.rs
- tests/cli_instance_state.rs
- src/run/send/budget.rs · edits `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` and its log capture helper

## Needs
- The Windows IPC endpoint: one named pipe `\\.\pipe\viola-<h12>` per wrapper, owned by that wrapper's start (a `viola run` or a passed `viola revive`).
- The local endpoint trust boundary: every process of the same OS user is trusted and no other; an endpoint must not be connectable by another OS user, nor from the network.
- interprocess 2.4.4 `local_socket` behind viola's own `channel` seam: the listener keeps interprocess's defaults (remote clients rejected, first instance, non-inheritable), built with the sync API.
- The endpoint name: `viola-` plus the first 12 hex of an FNV-1a 64-bit hash of `ViolaName` + NUL + the absolute home path, hand-written in `viola-channel` (never `DefaultHasher`); a second listener on a held name is what the first-instance property refuses.
- Test-only endpoint names for any new channel test: `viola-test-chan-<pid>-<label>` (and `viola-test-sqos-<pid>-<label>` beside it), unique per test, outside the `viola-<h12>` namespace, never bound by `viola`.
- The `viola-channel` `test-support` feature (on Windows `canonical_sddl`, `dacl_of`, an independent `user_sid`): enabled only by the root package's dev-dependency entry, never by a product build; release-check refuses an artifact built with it.
- The start's squatted-name refusal: exit 1, closed word `squatted-name`, printing no `--json` document as landed (that document is owed to "Exit-cause code catalogue"); the bind precedes the first snapshot, so a refused bind writes no `endpoint`.
- CLI exit codes stand: `1` for a start refusal; no new code or refusal word.
- `ChannelError`: one thiserror enum per crate with fixed messages, no path or pipe name in an error body.
- Tokio containment: the channel server and the wrapper's start path stay std-thread and blocking; only `viola-mcp` builds a runtime, and the channel's Tokio client stays behind its `tokio` feature.
- Cross-platform discipline: every OS-specific branch compiles and is tested on its CI runner (`windows-2025` for this chunk's assertions); any helper process a test spawns is spawned directly, never through a shell.
- The CI matrix: GitHub Actions on windows-2025 / macos-latest / ubuntu-latest against the fake agent; Windows live behaviour is proven on the CI runner until an interactive Windows host exists.
- Test homes for any start-level test: each test in its own `--home` under `target/e2e-home/viola-test-*/home`, so parallel tests never share an endpoint.
- The budget gate's send refusal: `budget-paused` with the crossed window's detail `five-hour` / `seven-day`, refused before `send-issued` with no cursor, and its one wrapper `send-refused` record that the test's capture reads.
- Diagnostic output channels: the wrapper's codes-only process log goes through the obs logger, not stdout or stderr, which is where a log-capture fix must read it.

## Not needed
- The client side of the channel and server verification before any frame (the SQOS Identification open's rule, the pid and start-time check, the dated liveness-only exceptions): the next route entry, "Server verification before any frame".
- The Unix socket endpoint (the 0700 per-user socket dir, the post-bind chmod 0600, peer credentials, no abstract namespace): owned by "Unix endpoint and home hardening".
- Home and code-bearing file integrity: the pinned binary's re-hash, the ConPTY companions and the DLL sideload, plugin files and `marketplace/`.
- The tokio RUSTSEC-2026-0386 advisory and any `Cargo.lock` bump or `deny.toml` change: a new route entry placed after this one; the chunk only reads `supply-chain` red.
- Wire contracts: the channel frame shapes, the method `params` / `ok` payloads, the event line, the snapshot and ledger envelopes and the `list` payload are unchanged.
- The MCP server and its tools, the hook transport and hook contract, and `hook statusline`.
- The capability ledger, `viola verify` and its probes, the fixtures and their schemas.
- `viola revive`, session liveness and the `claude agents --json` join of `viola list`.
- The wheel, links, delivery confirmation and the screen model's readiness gate.
- The config file, the dialog hold and the environment variable registry: nothing here reads a new variable or key.
- Repository dirs for fuzzing, mutation, perf, coverage and the harness's own target dirs.
