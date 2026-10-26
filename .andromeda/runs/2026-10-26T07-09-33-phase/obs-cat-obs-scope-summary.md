# obs · Obs Scope Summary — category read

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:485) A server-verification failure behind exit 21 must carry its own distinct `detail` code, apart from dead instance and strict-mode fail, with no path or pid in it.
  > Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:163) The CLI verbs' new exit-21 outcome on a mismatch must be logged as the machine-readable exit code field, with its cause told apart by detail code.
  >   - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  >   - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:370) The hook client's server-verify mismatch must be a fail-open path logged only to `hook-<name>.ndjson`, with exit 0 and stderr empty; a strict-modes failure, if this chunk adds one to the hook, stays unlogged as landed unless amended.
  >   - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: `viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).
  >   - Exit is 0 and stderr stays empty.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:185) Both hook clients (`hook.event`, `hook.dialog`) must keep the hook sink rules on a mismatch: no stderr, no stdout body, and a failed log write swallowed with exit 0.
  > A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:189) The verification added to every hook (a server pid and start-time read) must stay inside the per-hook latency budget.
  > The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:214) The MCP frame tools' mismatch must return `instance-unreachable` as a code only, and the log line for that rejected call must follow the same codes-only rule.
  > `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:378) A rejected MCP tool call on a failed verification must log no upstream text, absolute path or type name.
  > - **Required telemetry signal:** Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:318) `wait`'s logged outcome on a failed verification must use the existing `instance-unreachable` outcome and exit code 21, and no `last_assistant_message` content.
  > - **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:158) A CLI verb's verification refusal is logged only to its file sink when an instance resolves; its stderr carries only fixed-message lines.
  > A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:162) The unreachable refusal a mismatch prints on stderr must be a fixed string carrying no recorded pid, start time or endpoint path.
  >   - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:133) The snapshot's `endpoint` path the verification reads must be scrubbed from every external output of the check.
  >   - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) The server-identity reads (socket peer credentials, pipe server process id, sysinfo start time) are OS-API boundary calls, instrumented at viola's call site only.
  > Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) Any instrumentation the verification adds in `viola-channel`'s sync client must be sync.
  > N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:60) Any instrumentation the verification adds in `viola-state` (snapshot read, sysinfo liveness) must be sync.
  > N/A. Stack bans Tokio here, so instrumentation must be sync.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:396) If the chunk takes the strict-modes half with it, the client-side strict-modes refusal's telemetry must be decided against the landed rule that only the `ledger-stamps` read logs one.
  > Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:437) The fault classes the client readers touch (a corrupt or unsupported-`v` snapshot, stale heartbeat vs live pid, endpoint vanished during `wait`) are owed telemetry; the replay-recovery class stays bound to the held CARRY 7 / 8.
  > - **Required telemetry signal:** Fault telemetry per class:
  >   - torn ndjson line healed, with file and offset;
  >   - corrupt or unsupported-`v` snapshot → replay recovery;
  >   - child exit detected on the process handle without EOF (ConPTY);
  >   - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  >   - stale heartbeat vs live pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:450) The verification's log lines must carry identical fields on every OS, whether the server identity comes from a named pipe or a Unix socket.
  > - **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  >   - permission application: Windows DACL vs Unix 0600/0700;
  >   - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  >   - named pipe vs Unix socket in channel logs;
  >   - ConPTY vs openpty in `pty` spans.
  > 
  >   No OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:462) A panic in the new verification code must yield exactly one single-line panic record, and the hook must still exit 0.
  > - **Required telemetry signal:** A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0. No formal latency or availability SLO beyond the tests perf gates.
- (.andromeda/obs-plan.md:361) Landing client-side server verification makes `server-verify-failed` reachable on `wait` / `last`; the clause tying it to this route entry must be carried to the wrap as an amendment.
  > `strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands, owned by the route entries "Server verification before any frame" and "Home and code-bearing file integrity".
- (.andromeda/obs-plan.md:451) The server-verify `detail` code must be a member of the closed kebab-case catalog, never a path or pid.
  > **`detail` code catalog** (closed, kebab-case, never a path or pid; Founder Direction 4):
- (.andromeda/obs-plan.md:576) Any new `detail` value or field the verification logs must be in the §6 catalog and admitted by `schemas/diag-line.v1.json`, or G4 fails the build.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:715) The verification must reuse existing `event` values; a new one needs a Decisions Log entry.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:460) A hook's fail-open on mismatch logs at WARN, and a CLI verb's exit 21 `process-exit` logs at WARN.
  > | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
- (.andromeda/obs-plan.md:709) Every line the verification emits must go through `obs_event!`, never a raw tracing macro.
  > - NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
- (.andromeda/obs-plan.md:691) Any span on the verification function must be `#[instrument(skip_all, fields(...))]`, never bare.
  > - NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:557) The verification's lines must not log the snapshot's `endpoint` or any path.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan.md:394) Only `read_snapshot_or_replay` writes `state-recovered`; moving a client reader to the replay (CARRY 8, held behind CARRY 7) is what would add that line for the clients.
  > - **E5** (snapshot corruption): `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one line per call; the other snapshot readers write no `state-recovered` line.
- (.andromeda/obs-plan.md:649) The hook perf gate must still pass with verification added on the spine hooks and `pre-tool-use`.
  > Every budget gates the hyperfine `max` sample through `gate --require perf`: `viola hook session-end` under 1.0 s, the spine hooks and `pre-tool-use` on an unverified CLI under the spine deadline (tests-owned, provisionally 1.0 s), and obs overhead on `hook` inside the same gate, which holds only when every hook command runs with `VIOLA_NAME` set and a home under `target/e2e-home/`; the heartbeat flips at >5 s.
- (.andromeda/obs-plan.md:600) The mismatch tests' homes must live under `target/e2e-home/` so G2, G4 and the secret scan cover the hook's fail-open lines.
  > Every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it — a `hook` panic exits 0, so a home outside that root would hide it — except the one `tests/chaos_feed_panic.rs` home (below).
- (.andromeda/obs-plan.md:642) The chaos class "stale heartbeat vs live pid" stays gated through the short-lived readers' liveness; the verification must not change that reading.
  > The chaos class "stale heartbeat vs live pid" is gated on the short-lived readers' `liveness` (`list --json`) through `agent-run status` (`state: degraded`) (§3 Heartbeat ticks).
- (.andromeda/obs-plan.md:264) Where §1 differs from §3 / §6 / §12 (the `event` enum extensions, the channel join key), the keyed sections win for the verification's lines.
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21), `hook-invoked` `corr` timing (D-07), the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`), and the MSRV floor (D-22: 1.96).

## Bindings
- **server-verify fail-open on the hook ↔ security.md §Local trust boundary ("Before the FIRST frame … verify the server") and the universal hook invariant**: the security side requires the check and fail-open; the obs side requires that path to log only to `hook-<name>.ndjson` with exit 0 and stderr empty (.andromeda/obs-plan-detail/obs-scope-summary.md:370), at WARN (.andromeda/obs-plan.md:460).
- **distinct exit-21 `detail` code ↔ obs §6 `detail` catalog (`.andromeda/obs-plan-detail/log-coverage.md`) and `schemas/diag-line.v1.json`**: Founder Direction 4 requires a server-verify-fail code distinct from dead instance and strict-mode fail (.andromeda/obs-plan-detail/obs-scope-summary.md:485); the catalog must name it and G4's schema must admit it (.andromeda/obs-plan.md:576), or a Decisions Log entry when it is a new `event` (.andromeda/obs-plan.md:715).
- **MCP `instance-unreachable` ↔ security-plan §Error Handling (MCP `isError` codes only)**: both sides require a codes-only result and log line (.andromeda/obs-plan-detail/obs-scope-summary.md:214).
- **`server-verify-failed` reachability clause ↔ the working route's entries "Server verification before any frame" / "Home and code-bearing file integrity"**: the body's §4 `wait` / `last` scenario line goes stale when this chunk lands and is a wrap amendment (.andromeda/obs-plan.md:361).
- **client-side strict-modes refusal telemetry ↔ "Home and code-bearing file integrity" and the P4 boundary question**: if this chunk takes the strict-modes half, the landed "line only on the `ledger-stamps` read" rule binds or is amended (.andromeda/obs-plan-detail/obs-scope-summary.md:396).
- **snapshot replay telemetry ↔ the scope's held CARRY 7 / CARRY 8**: `state-recovered` is written by the replay reader only, so the clients gain it only if the founder rules their move to the replay (.andromeda/obs-plan.md:394), (.andromeda/obs-plan-detail/obs-scope-summary.md:437).
- **hook verification cost ↔ test-plan §10 perf gate**: the per-hook `max < 1.0 s` budget must hold with the server read added (.andromeda/obs-plan-detail/obs-scope-summary.md:189), (.andromeda/obs-plan.md:649).
- **identical fields per OS ↔ the Windows pipe server-pid read vs Unix `peer_creds` (security.md, `viola-channel`)**: the line schema must not differ by OS (.andromeda/obs-plan-detail/obs-scope-summary.md:450).
