# Task — obs · 2026-10-25-windows-endpoint-admission

## The chunk
Asserts the Windows half of the IPC access-control clause on the Windows CI leg: the wrapper's named-pipe listener in `viola-channel` rejects a remote (network) pipe client, read back from the pipe's mode, and holds the first-pipe-instance property, so a second listener on a held name fails.
Where a Windows CI runner can measure it without a second interactive user, it also asserts that the DACL works: an open by a principal outside the user-SID and SYSTEM ACEs is denied. Otherwise this is left to the Windows-only live measurement entry.
Re-verifies, without rebuilding, what HEAD already asserts: the protected DACL read back as two allow ACEs, non-inheritable handles on both ends, and the squatted-name exit-1 start refusal of `viola run`. It also re-verifies the CARRY's test-file coordinate.
Closes a folded CARRY: the budget send-refusal test case_4 went red once under a plain `cargo test` because its log capture found no `send-refused` line. The case is run three times under plain `cargo test` or its log capture is fixed; the cause is re-verified first.
No wire contract, exit code or refusal word changes: `squatted-name` and exit 1 stay as they are.

## Reached files
- crates/viola-channel/src/server.rs · edits the doc comment on the remote-client rejection default
- crates/viola-channel/src/server/win.rs · edits its unit tests (DACL read-back, non-inheritable handles, remote-client rejection, first-pipe-instance)
- crates/viola-channel/src/endpoint.rs
- tests/security_negatives_channel.rs
- tests/path1_run_start.rs
- tests/cli_instance_state.rs
- src/cmd/run.rs · edits `refuse_squatted`
- src/run/send/budget.rs · edits test `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` and its log-capture helper

## Needs
- The `viola-channel` server listener on Windows: the named-pipe bind and its pipe mode, the first-instance flag, handle inheritability, and the protected DACL. This is an ipc-internal surface whose instrumentation must stay sync, with no Tokio in the crate's normal graph.
- The `viola run` start sequence up to channel bind, which is the must-trace start scenario. The endpoint name, the pinned bin path and env values never appear in a span or line.
- The squatted-endpoint start refusal. It writes exactly two fixed stderr lines through `src/human.rs` `refuse`, with no path or pid, before any child spawns, and the terminal gets nothing else.
- The exit-1 start failure line: `process-exit` at ERROR with its closed, distinct `detail` code, which carries no path or pid. The `squatted-name` cause keeps its existing code.
- A listener or bind failure caused by a remote-client or first-instance refusal is captured at the viola seam as a viola event. The `interprocess` target is OFF in the sink, so nothing comes from the library's own logging.
- Any new or changed `ChannelError` variant keeps a fixed thiserror `Display`, with no path, pipe name or OS message. A `-32603` reply stays exactly `internal error` with `data: null`.
- Any log line added on this path goes only through `obs_event!` with explicit typed fields. Any span uses `#[instrument(skip_all, name = "channel.<op>", fields(...))]` with a static name and never `err` or `ret`. Raw tracing level macros or `event!` must not be used.
- `channel-request` / `channel-response` lines never record `params` or result bodies. The self-reported `from` is never logged as an authenticated identity.
- No new `event` value is added without an obs Decisions Log entry. The closed `detail` catalog is extended only through the plan, and its values never carry paths or pids.
- The default-deny log schema `schemas/diag-line.v1.json`, checked by gate G4: any line the chunk's code or tests emit must conform, with no field outside the catalog.
- Gate G2, zero panics, over home-level role files on the Windows leg. A panic in the new listener tests or in `run`'s start must leave its one `event:"panic"` line.
- Every new integration test's viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it.
- The same file sink and line schema on every OS: the Windows-only assertions add no OS-specific exporter, field or event.
- The `send-refused` line on the budget-paused refusal (the budget governor scenario's send-side gate). It carries the refusal code and window word but no sent text and no cursor, and the CARRY test case_4 reads it.
- How the CARRY test captures log output under libtest threads, through the `obs_event!` pipeline and its test capture helper. The capture must see the line the refusal emits on its own thread; nothing may be sent to stdout or stderr to make it visible.
- Zero-flakiness rule: `retries = 0` and no retry-once policy. A green re-run does not close the case_4 red; its cause must be known, or its capture fixed.
- The `diagnostics_level` and level filter are set in code only. No `RUST_LOG` or env var may be read to make the test capture work.

## Not needed
- The MCP server surface (`viola mcp`, `mcp.ndjson`, `isError` codes): the chunk does not touch the MCP tools or their Tokio client.
- Client-side server verification before the first frame (SQOS open, pid and start-time check, `server-verify-failed` / `strict-modes-failed` on clients, the dated liveness-only exceptions). That is the next route entry, "Server verification before any frame".
- Unix socket hardening and the home's 0700/0600 modes, including the Windows home and diagnostics-file DACL. These belong to "Unix endpoint and home hardening" and "Home and code-bearing file integrity", not to the pipe's DACL.
- The hook roles: their decision body, fail-open behaviour, exit-0 contract, `hook-invoked` corr timing, and hook latency budgets measured with hyperfine.
- The `wait` / `last`, dialog → `answer`, and wheel / `release` must-trace scenarios. The chunk changes none of their paths.
- The budget governor's statusline shell-out, `budget.json` write, and `budget-gate` product event. Only the send-side refusal line read by the CARRY test is reached.
- Derived metrics (Section 5): the chunk adds and changes none.
- The PTY seam, the child spawn, the R8 env strip counts, the readiness gate, and the vt100 feed panic handling.
- Edge flows E1, E3 and E5, torn-line healing, the capability ledger read in `run.version_gate`, and `verify` / `plugin install` logging.
- Panic hook design and the per-role panic exits. The chunk relies on them but does not change them.
- Error-reporter platform choices (sentry, a future reporter's preconditions).
- The `ci.yml` step order, the artifact upload gating, retention days, and the Actions pinning. The chunk adds tests that run in the existing jobs and edits no workflow.
- The `supply-chain` job's `cargo deny` advisories red (the tokio RUSTSEC-2026-0386 lock bump). Its own new route entry owns it, and this chunk carries no acceptance criterion on it.
- Heartbeat liveness transitions and the operational retention of `events.ndjson` / `diagnostics/`.
