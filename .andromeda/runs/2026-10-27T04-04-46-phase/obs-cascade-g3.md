# obs · cascade read · group 3 of 10

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:199) An over-size or invalid channel frame refused on either side must be logged in that side's own process file, never in a shared or stdout sink.
  > - **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
  (.andromeda/obs-plan.md:123)
  > the log file of whichever process is on each side;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:203) The channel-frame boundary must keep `-32603` exactly `"internal error"` with `data:null` and must log `from` as self-reported, never as an identity.
  > - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  > - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:212) The MCP stdin and tool-argument rejections must land in `mcp.ndjson`, opened only after the home and its `diagnostics/` pass strict-modes, and the MCP witnesses must read them there.
  > - **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers).
  (.andromeda/obs-plan.md:127)
  > - **Exporter:** always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:213) The MCP line's `instance` and the `from` it adds must be a `VIOLA_NAME` that passed `ViolaName::try_new`; a refused one resolves no instance.
  > - **Service identity:** as for the short-lived verbs, with `process="mcp"`. `instance` is the server's own `VIOLA_NAME` when set, which is also the `from` it adds.
  (.andromeda/obs-plan.md:111)
  > the instance comes from `VIOLA_NAME` or an argument, never from the working directory;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:215) Every MCP call the chunk's bounds reject must return codes only and log a codes-only line, with no upstream text, path or type name.
  > - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:230) The forced vt100 panic witness must still meet the first-statement panic hook's one line, and an instance must resolve only from `VIOLA_NAME` or a validated argument, with a strict-modes refusal opening nothing and writing no line.
  > Init order per process:
  > 1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  > 2. Parse clap.
  > 3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:276) An oversize or malformed hook stdin must fail open with its line only in `hook-<name>.ndjson` and nothing on stderr; the `claude --version` / `claude agents --json` spawns the chunk bounds must leave no stripped `CLAUDE*` value in any line; a forced panic must be one ERROR `panic` line.
  > - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  > - No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
  > - **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
  (.andromeda/obs-plan.md:200)
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
  (.andromeda/obs-plan.md:210)
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values
  (.andromeda/obs-plan.md:83)
  > a parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:285) The role file basenames join the instance name, so that name must pass `ViolaName::try_new` before the join, and a refused name opens no role file and writes nothing under `diagnostics/`.
  > - **Log file location:** Tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:293)
  > The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed
- (.andromeda/obs-plan-detail/obs-scope-summary.md:300) The bounded `snapshot.json` and `ledger/stamps.json` reads must keep their outcomes logged under the start sequence's `state.snapshot_recover` and `run.version_gate`.
  > - **Must-trace spans:** `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind`
- (.andromeda/obs-plan-detail/obs-scope-summary.md:301) An oversize, malformed or strict-modes-refused `ledger/stamps.json` must read as `cli_verified:false` on the run line, with no path or stamp content.
  > - **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`.
  (.andromeda/obs-plan.md:215)
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate
- (.andromeda/obs-plan-detail/obs-scope-summary.md:309) The paste rule must run on `send.client` (CLI and MCP) before any `channel.request` and in the wrapper before the readiness gate's first wheel read, each refusal one `send-refused`.
  > - **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:310) Each deciding side's refusal must carry `detail` `control-character` / `empty-text` (and the vt100 degrade `input-not-ready`) with the sent text in no line.
  > - **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.

## Bindings
- **paths ↔ scope-surfaces**: the paste refusal is logged on the deciding side's own sink — the CLI client's, `mcp.ndjson` for MCP `send`, `run-<name>` for the wrapper — codes only, (.andromeda/obs-plan-detail/obs-scope-summary.md:310) with (.andromeda/obs-plan-detail/obs-scope-summary.md:212) and (.andromeda/obs-plan-detail/obs-scope-summary.md:215).
- **strategy ↔ logs ↔ scope-surfaces**: the instance resolves from a validated `VIOLA_NAME` or argument before the role file whose basename joins that name opens, so a refused name opens no file and writes no line, (.andromeda/obs-plan-detail/obs-scope-summary.md:233) with (.andromeda/obs-plan-detail/obs-scope-summary.md:285) and (.andromeda/obs-plan-detail/obs-scope-summary.md:213).
- **log-fields ↔ paths**: the vt100 panic degrade reads as `detail` `input-not-ready` at `run.readiness_gate` while the panic hook still writes its one ERROR `panic` line, which the forced-panic witness must account for, (.andromeda/obs-plan-detail/obs-scope-summary.md:278) with (.andromeda/obs-plan-detail/obs-scope-summary.md:309) and (.andromeda/obs-plan-detail/obs-scope-summary.md:231).
- **paths ↔ the body's Vector 7 signal**: a refused or oversize stamps read is `cli_verified:false` under `run.version_gate`, the one strict-modes line allowed, (.andromeda/obs-plan-detail/obs-scope-summary.md:301) with (.andromeda/obs-plan.md:215).
- **scope-surfaces ↔ the body's Vector 1 signal**: channel frame refusals on both sides log no `params` and treat `from` as self-reported, (.andromeda/obs-plan-detail/obs-scope-summary.md:204) with (.andromeda/obs-plan.md:190).
