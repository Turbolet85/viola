# obs · cascade read · group 3 of 7

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:318) The `wait` / `last` clients' failed server verification before their frame must surface in these required fields as the `instance-unreachable` outcome with `exit_code` 21, and the `last_assistant_message` content must stay out of every line.
  > `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:325) The chunk edits both clients of this path, the hook's `hook.dialog` client and the CLI / MCP `answer` client, so each must take the verification before its frame.
  > **Surfaces involved:** cli (`hook` PreToolUse / PermissionRequest) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`
- (.andromeda/obs-plan-detail/obs-scope-summary.md:326) The verification must run inside `hook.handle` and `answer.client` ahead of their `channel.request`, and on a refused verification the dialog hook writes no `channel.request(hook.dialog)` and the `hook-invoked` it would log after the response is the fail-open path's to state.
  > **Must-trace spans:** `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`; on the driver side, `answer.client` › `channel.request(answer)`. Log events: `hook-invoked`, `dialog-raised`, `dialog-answered`, `hook-decision` (decision null on deadline expiry).
  (.andromeda/obs-plan.md:142)
  > (c) `hook-invoked` is emitted before the wrapper assigns `dialog_id`, so `corr` can only be filled from `hook-decision` onward, or `hook-invoked` must be logged after the `hook.dialog` response.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:327) The `answer` client's exit-21 verification failure must carry its own `detail` beside this path's existing codes, and neither it nor the hook's fail-open line may hold the question, answer or tool `input` text.
  > **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:334) The chunk edits the `pause` / `release` clients on the wheel path, so both must verify the server before their frames.
  > **Surfaces involved:** cli (`run`, key input) + `events.ndjson` (`wheel`) → cli or ipc-internal (mcp) `send` refused → cli `release` → ipc-internal
- (.andromeda/obs-plan-detail/obs-scope-summary.md:335) `pause.client` / `release.client` must take the verification before `channel.request(pause|release)`, so a refused check logs no `channel-request` / `channel-response` pair for a frame never written.
  > **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:353) The sync and Tokio channel clients' edited connect must keep the `channel-request` / `channel-response` scrub rule on every call it makes, and the server identity the clients now verify must not be logged as an authentication of the self-reported `from`.
  > A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  >   - `-32603` is logged as the fixed "internal error".
  >   - `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:123)
  > - **Exporter:** the log file of whichever process is on each side; propagation is the `corr` field in log lines, since channel `params` declare no trace-context field, and a merged-log join needs `process`, `instance` and a connection discriminator, a gap for Phase 3; `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:362) A CLI or MCP `send` refused before its frame by a failed verification must be logged codes-only, the sent text in no stderr, `--json`, MCP or home-level process-log field.
  > A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:370) The hook's server-verify mismatch (and a strict-modes failure, if P4 takes that half) must be logged as a fail-open path to `hook-<name>.ndjson` only, exit 0 with stderr empty; the as-landed statement that a strict-modes failure leaves no line changes if the hook clients take a strict-modes check.
  > Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: `viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).
  >   - Exit is 0 and stderr stays empty.
  (.andromeda/obs-plan.md:200)
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
  (.andromeda/obs-plan.md:119)
  > - **Exporter:** a file sink only, to `diagnostics/hook-<name>.ndjson`; stderr is forbidden, and stdout carries only the decision body (for `viola hook statusline`, the user's status-line command's output); a failed log write is swallowed and the hook still exits 0; without `VIOLA_NAME` the hook is a silent no-op; each line is one `write` on an append-mode handle, and logger init adds no blocking I/O beyond one file open and append.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:378) The MCP `send` / `wait` / `last` / `answer` tools' failed verification must return `instance-unreachable` as an `isError` code and be logged with codes only, no upstream text, absolute path or type name.
  > Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names.
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
  (.andromeda/obs-plan.md:127)
  > - **Exporter:** always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened; stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:396) Whether the clients take the strict-modes half is P4's fork, and if they do, this as-landed list of where a strict-modes failure has a line (and where it leaves none) must change with it; if they do not, it must keep holding.
  > Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Four strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  (.andromeda/obs-plan.md:215)
  > - **Required telemetry signal:** `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:412) The verification's instrumentation in `viola-channel`'s sync client, its endpoint read and `viola-state`'s `live_endpoint` must stay Tokio-free and add no obs dependency or C-building crate to the sync graph.
  > Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:419) Every channel call a verified client makes must keep its `corr` join and connection discriminator, and a `send` or `answer` refused before its frame must log no join value it never obtained.
  > `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  >   - `corr` = `dialog_id` joins hook → wrapper → driver.
  >   - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
  (.andromeda/obs-plan.md:264)
  > the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:428) The added peer-credential and sysinfo start-time read must keep each hook event inside its `duration_ms` budget (`session-end` under 1.0 s, spine hooks and `pre-tool-use` under the spine deadline), and the edited `live_endpoint` must keep the heartbeat staleness flip at >5 s.
  > `duration_ms` on `hook-decision`, or on `process-exit` for `process="hook"`, per hook event. The budget is `session-end` `max < 1.0 s`, and spine hooks and `pre-tool-use` on an unverified CLI must stay under the spine deadline (provisionally 1.0 s).
  >   - The heartbeat staleness flip happens at >5 s (4.9 s live / 5.1 s stale or gone).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:439) These fault classes are the outcomes the edited pre-check decides: an unreadable or newer snapshot must keep reading as absent (no replay recovery line from a client) under the founder's hold, an endpoint gone or a stale heartbeat beside a live pid must keep its exit 21 / `instance-unreachable`, and the Windows exiting-wrapper window must be stated against them.
  > corrupt or unsupported-`v` snapshot → replay recovery;
  >   - child exit detected on the process handle without EOF (ConPTY);
  >   - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  >   - stale heartbeat vs live pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:450) The server pid read on a named pipe and on a Unix socket, and the macOS euid + dir-only gap, must yield the same file sink, line schema and fields on every OS, with no OS-specific field for the gap.
  > **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  >   - permission application: Windows DACL vs Unix 0600/0700;
  >   - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  >   - named pipe vs Unix socket in channel logs;
  (.andromeda/obs-plan.md:239)
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:476) A CLI or MCP `send` refused before its frame by a failed verification must log the event its refusal is (never a `send-issued` for a frame not written), from the client's own role file.
  > `send-issued` and `send-refused` are logged events.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:485) The CLI verbs' new exit-21 outcome must carry a `detail` code distinct from the dead-instance cause, `server-verify-failed` (and `strict-modes-failed` if P4 takes that half), with no path or pid in it.
  > Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
  (.andromeda/obs-plan.md:140)
  > The distinct detail codes for exit 21 and exit 1 can be an additive `detail` field.
  (.andromeda/obs-plan.md:111)
  > refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.

## Bindings
- **obs-scope-summary (exit-21 causes) ↔ log-coverage**: the `server-verify-failed` (and, under P4's fork, `strict-modes-failed`) `detail` codes required at (.andromeda/obs-plan-detail/obs-scope-summary.md:485) must enter §6's closed catalog and the default-deny `schemas/diag-line.v1.json` (G4), at WARN for exit 21 and for the hook's fail-open `hook-decision`, (.andromeda/obs-plan.md:426).
- **obs-scope-summary (wait / last) ↔ span-trace-coverage**: the `instance-unreachable` / exit-21 outcome at (.andromeda/obs-plan-detail/obs-scope-summary.md:318) is the outcome the `wait` / `last` scenario's note says cannot occur until this entry lands; the note must change with the chunk, (.andromeda/obs-plan.md:361).
- **obs-scope-summary (dialog path) ↔ span-trace-coverage**: the `hook.dialog` client's fail-open on a refused verification at (.andromeda/obs-plan-detail/obs-scope-summary.md:326) meets the dialog scenario's rule that `hook-invoked` is written only after the `hook.dialog` response, which never comes on a refused check, (.andromeda/obs-plan.md:369).
- **obs-scope-summary (hook budget) ↔ slo-invariants-telemetry-budgets**: the hook latency budget at (.andromeda/obs-plan-detail/obs-scope-summary.md:428) is the SLO the added peer-credential and sysinfo read is gated against (`gate --require perf`), (.andromeda/obs-plan.md:629).
- **obs-scope-summary (strict-modes refusal telemetry) ↔ the security plan / P4 fork**: the as-landed strict-modes line rule at (.andromeda/obs-plan-detail/obs-scope-summary.md:396) and the hook fail-open list at (.andromeda/obs-plan-detail/obs-scope-summary.md:370) move together only if P4 takes the strict-modes half ("Home and code-bearing file integrity" otherwise owns it); both hold unchanged if it does not.
- **obs-scope-summary (channel scrub and join) ↔ the §3 harness contract**: the scrub rule at (.andromeda/obs-plan-detail/obs-scope-summary.md:353) and the `corr` join at (.andromeda/obs-plan-detail/obs-scope-summary.md:419) bind the edited channel clients' connect to D-25's `(conn, corr)` join key, (.andromeda/obs-plan.md:264).
- **obs-scope-summary (new typed error) ↔ error-capture-reporting**: the codes-only exit-21 / `instance-unreachable` outcomes at (.andromeda/obs-plan-detail/obs-scope-summary.md:485) and (.andromeda/obs-plan-detail/obs-scope-summary.md:378) come from a verification error whose fixed-message `Display` and seam crossing §7 rules, (.andromeda/obs-plan.md:490).
