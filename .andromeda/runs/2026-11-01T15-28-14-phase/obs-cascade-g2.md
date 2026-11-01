# obs · cascade read · group 2 of 9

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:172) The wrapper-side euid check and any trace of it sit on the `viola-channel` sync server inside `run`, and that instrumentation must stay sync, with no Tokio.
  > **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:174) A wrapper-side euid refusal happens while the child runs, so it must reach the file sink or nothing and never the terminal; a refused socket directory at a wrapper's start must take the pre-child two-line exception, and must change the recorded `human::refuse` count if it adds a site.
  > **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
  (.andromeda/obs-plan.md:115)
  > a file sink only, to `diagnostics/run-<name>.ndjson`; `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns
  (.andromeda/obs-plan.md:311)
  > `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed — the root bin's human-stderr refusal writer, called by `run` (7 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity; `list`'s colour layer adds no stderr writer and its `unable` / `hint:` pair carries no SGR.
  (.andromeda/obs-plan.md:311)
  > While the child runs, neither `run` nor a revived start writes anything of its own.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:185) The hook's connect must take the new refusals only as a fail-open, its trace the required `hook-decision` line, with a panic on that path still one `panic` line.
  > **Backend instrumentation hooks:** `tracing` spans around stdin parse → channel call (`hook.dialog` / `hook.event`) → decision emit, through the tracing-subscriber JSON logger. `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:187) A refusal of this chunk must leave `hook` on exit 0 with no body and no stderr, its trace in `hook-<name>.ndjson` alone, as the scope's boundary states.
  > **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
  (.andromeda/obs-plan.md:119)
  > a failed log write is swallowed and the hook still exits 0; without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame)
  (.andromeda/obs-plan.md:200)
  > fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:188) The new connect-time checks must fall after the hook's instance check, whose refusal stays a silent exit 0 with no line and no frame, and the plan must state where they fall against it.
  > The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:191) The socket-directory verification and the peer euid check join the hook's connect and so the timed path, and they must stay inside the `max < 1.0 s` budget and keep the five `run --perf` hook rows under `SPINE_DEADLINE`.
  > The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append; the instance check ahead of it (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path, and `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms (as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:198) A connection refused on euid or directory has no frame and no JSON-RPC `id` behind it, so its record must not read as a request on `corr`, and the check must add no trace-context or identity field to the channel `params`.
  > **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared").
- (.andromeda/obs-plan-detail/obs-scope-summary.md:200) A peer euid mismatch or refused socket directory on either side must be recorded only in the log file of the process on that side, and a short-lived verb without an instance must have none.
  > **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:201) Each side's refusal line must name its own `process`, and no refusal ahead of the first frame may rely on `params.sender`, which only a frame carries.
  > Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:205) A verified peer euid must not turn the self-reported `from` into an authenticated identity in any line the chunk adds or changes.
  > `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
  (.andromeda/obs-plan.md:123)
  > the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:190)
  > channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:211) Every Unix client checks, so the Tokio client wrapped by the MCP tool call must take the same checks as the sync client, with the outcome kept inside that tool's span.
  > **Backend instrumentation hooks:** `tracing` spans per MCP tool call (`send · wait · last · answer · list`) wrapping the `viola-channel` Tokio client, through the tracing-subscriber JSON logger. stdio has no HTTP-framework auto-instrumentation.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:216) A client-side refusal must map to `instance-unreachable` with a code-only `detail`, and each new cause must join this closed list.
  > `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable` (its `detail` one closed cause: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`; the CLI's document less `v`, its text `refused: instance-unreachable`), `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  (.andromeda/obs-plan.md:127)
  > `isError` results and the log lines for rejected calls carry codes only.
  (.andromeda/obs-plan.md:205)
  > rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:235) Each new refusal must be placed against this init order: a strict-modes check the chunk brings in before the first open ends with nothing opened and no line, while one after init is logged with its distinct `detail`, and a path newly under strict modes must stay off the closed trigger list or join it by an amendment.
  > Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:240)
  > For `run`, this completes before the start sequence, so the collision check and version gate are logged. For `mcp`, it completes before the rmcp stdio transport starts.
  (.andromeda/obs-plan.md:215)
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate, on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged) and on `verify`'s own `process-exit` for a refused stamp write (`detail` `strict-modes-failed`, exit 1); a refused `diagnostics/` folder or file is a failed open, its line dropped.
  (.andromeda/obs-plan.md:253)
  > each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
  (.andromeda/obs-plan.md:161)
  > exit-1 causes use a distinct `detail` code, with no path or pid in it.
  (.andromeda/obs-plan.md:111)
  > refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:268) A wrapper-side refused connection has no request behind it, so it must not be recorded as this pair, keyed by `conn` instead, and a client's refusal `process-exit` must carry no `corr`; the hook's fail-open write stays swallowed.
  > `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
  (.andromeda/obs-plan-detail/obs-scope-summary.md:272)
  > `process-start`, `process-exit`, `panic`: `corr` = null
  (.andromeda/obs-plan-detail/obs-scope-summary.md:278)
  > The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.

## Bindings
- **Surface exporters ↔ body §3 harness contract**: a refused socket directory at a wrapper's start must be written as the fixed pair and recorded in the start-refusal list and the `human::refuse` per-verb counts, while the wrapper-side euid refusal stays off the terminal (.andromeda/obs-plan.md:311).
- **Init order ↔ span-trace-coverage, the `wait` / `last` scenario**: the new client refusals must sit beside the instance check that ends first with no line and the `server-verify-failed` exit, and the euid check must not read as closing the macOS peer pid gap (.andromeda/obs-plan.md:361).
- **MCP closed cause list ↔ log-coverage `detail` catalogue and error-capture**: each new cause must be a closed code in the catalogue, and a frame tool's `instance-unreachable` must stay logged by no line, its `isError` code the only record (.andromeda/obs-plan.md:451) (.andromeda/obs-plan.md:537).
- **Hook fail-open ↔ log-coverage levels**: the fail-open trace on `hook-decision` must take its `detail` at WARN (.andromeda/obs-plan.md:460).
- **Sync instrumentation ↔ the Tokio client**: both clients must make the checks while `viola-channel`'s graph keeps Tokio behind its feature (.andromeda/obs-plan.md:59).
- **Section 1 ↔ §3, §6 and §12**: where a surface line here differs, those sections win until a wrap brings it current, the channel join key `(conn, corr)` among them (.andromeda/obs-plan.md:264).
