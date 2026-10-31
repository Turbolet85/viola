# obs · cascade read · group 2 of 9

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:302) Each logged exit-1 start cause, `revive`'s among them, must carry its own distinct `detail` with no path or pid, and a home-tree or instance check refused before the log must stay lineless with its cause in the stderr pair or the `--json` document, this text moving with any contract change the chunk makes.
  > - **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe`) refuses writes no line either, so that exit-1 cause has no `detail` code: the check precedes the wrapper log, and the output is the home pair (a home-level refusal) or the instance pair (an instance-level one), revive `--json` the `strict-modes-failed` document. No path or pid appears in the detail.
  (.andromeda/obs-plan.md:161)
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
  (.andromeda/obs-plan.md:215)
  > refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged); a refused `diagnostics/` folder or file is a failed open, its line dropped.
  (.andromeda/obs-plan.md:311)
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid)
  (.andromeda/obs-plan.md:311)
  > `viola revive` (its start arm logs as process `run` and makes `run`'s home-tree check before that log) adds three refusals to `run`'s five: state files another user could write, no logged session (or an `--id` the log does not hold) and a missing recorded directory; on a live name it prints `run`'s own collision pair.
  (.andromeda/obs-plan.md:264)
  > Section 1 is a verbatim copy of obs-scope.md, and it is kept current (the founder's ruling of 2026-10-04): a proposal or a cascade hit inside it is judged like any body amendment, and its label stays.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:301) A refusal the `run` start path logs must reach its `process-exit` on failure, its place among the collision check and the pinned copy's spans deciding which start refusal comes after the log.
  > **Must-trace spans:** `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:234) The closed list must split causes by this init order: a check at step 3 must end with nothing under the home opened and no line, so it takes a code on `--json` and MCP alone unless P4 names otherwise.
  > Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:239)
  > For `run`, this completes before the start sequence, so the collision check and version gate are logged. For `mcp`, it completes before the rmcp stdio transport starts.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:266) A log form the chunk gives a cause must ride an additive `detail` on an existing event with `corr` null, a new `event` value needing its own Decisions Log entry and no required field renamed or removed.
  > `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
  (.andromeda/obs-plan-detail/obs-scope-summary.md:271)
  > `process-start`, `process-exit`, `panic`: `corr` = null
  (.andromeda/obs-plan-detail/obs-scope-summary.md:275)
  > obs-plan may add fields but must not rename or remove these. The harness greps on them.
  (.andromeda/obs-plan.md:140)
  > The distinct detail codes for exit 21 and exit 1 can be an additive `detail` field.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:215) Every new exit-21 or exit-1 code MCP returns must keep codes only and join this list, with `instance-unreachable`'s word moving here if the contract choice moves it.
  > `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:216)
  > A peer's reply is screened before it reaches a result (`of_reply`): a `refusal` that is no code word (`viola_core::is_code_word`) reads `unknown`, a string `detail` that is no code word reads `null`, and `wrapper-fault`'s `detail` is the closed object `viola_channel::sanitised_fault` reads (the fixed `message` of the five codes, `data` null or closed values; a code outside the five reads `message` `wrapper fault`, `data` null). A code word outside the closed list stays visible, its exit `unknown`'s 14.
  (.andromeda/obs-plan.md:127)
  > `isError` results and the log lines for rejected calls carry codes only.
  (.andromeda/obs-plan.md:205)
  > rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:212) `viola mcp`'s refused home check and its oversize line before `initialize` must each join the table or be named outside it by a reason, keeping stdout for MCP frames alone and the one `parse-rejected` line the oversize's only record.
  > **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers). stdout is forbidden because it carries MCP frames. tracing-subscriber `fmt()` writes to stdout unless a writer is configured, so the writer must be set explicitly. rmcp reads stdin only through viola's line cap (`CappedLines` at `MAX_FRAME`): a line of at most `MAX_FRAME` bytes, its newline counted, passes whole; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, stderr empty, its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line. Such a line before `initialize` exits the process 1 with both streams empty and that same one record; 11 is the unit child's own code, not `viola mcp`'s exit.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:203) A new code must never travel in a `-32603`'s `error.data`, which must stay null, and the fixed-message error layer must hold for every code crossing the channel to a client.
  > `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`
  (.andromeda/obs-plan.md:47)
  > a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:199) The endpoint-lost code's line must land in the client process's own file, and a short-lived verb with no instance must have no line to carry a code, its record then being the stderr pair or the `--json` document alone.
  > **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
  (.andromeda/obs-plan.md:111)
  > a file sink only, JSON-per-line to `diagnostics/` when an instance resolves — no network OTLP (viola makes no outbound network calls) and no stdout exporter (stdout carries `--json` results); the instance comes from `VIOLA_NAME` or an argument, never from the working directory; refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:187) The catalogue must add no hook cause: a refused or absent instance must stay a silent exit 0 with no line, and a hook panic must stay a `panic` line with exit 0, outside `internal-error`'s exit-1 row.
  > The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:186)
  > stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`
  (.andromeda/obs-plan-detail/obs-scope-summary.md:191)
  > Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:184)
  > `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line
  (.andromeda/obs-plan.md:119)
  > without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:294) The closure of the cause list against the code must place a refused `diagnostics/` folder or role file, whose failed open leaves no role file for a cause's line to land in, and a table case asserting a line must keep to a home this check passes.
  > The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed — a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped).

## Bindings
- **The run-start path's `detail` list ↔ the init order**: a cause holds a `detail` only where its refusal comes after init step 4's open, so the pre-log carve-out at (.andromeda/obs-plan-detail/obs-scope-summary.md:302) and the step 3 check at (.andromeda/obs-plan-detail/obs-scope-summary.md:234) must name the same refusals, each code the table asserts on a line being one that order lets write.
- **The MCP surface ↔ the process-log fields**: one cause's word must read the same on `isError` at (.andromeda/obs-plan-detail/obs-scope-summary.md:215) and on `process-exit`'s `detail` under the closed `event` rule at (.andromeda/obs-plan-detail/obs-scope-summary.md:266), and the `of_reply` screen at (.andromeda/obs-plan-detail/obs-scope-summary.md:216) must let each new word through as a code word.
- **The channel surface's exporter ↔ its error frames**: the endpoint-lost code's line at (.andromeda/obs-plan-detail/obs-scope-summary.md:199) and the null `data` at (.andromeda/obs-plan-detail/obs-scope-summary.md:203) must hold together, the code carried by the client's own reading and never by the `-32603` frame.
- **The hook surface ↔ the creator-explicit trigger**: the distinct-code rule at (.andromeda/obs-plan.md:253) must stop at the hook, whose silent refusal at (.andromeda/obs-plan-detail/obs-scope-summary.md:187) has no exit-21 or exit-1 to code.
- **Section 1's text ↔ the plan's body**: a contract choice carrying the code outside `detail` must move (.andromeda/obs-plan.md:140), (.andromeda/obs-plan.md:253) and (.andromeda/obs-plan-detail/obs-scope-summary.md:302) together, as (.andromeda/obs-plan.md:264) rules a Section 1 hit a body amendment.
