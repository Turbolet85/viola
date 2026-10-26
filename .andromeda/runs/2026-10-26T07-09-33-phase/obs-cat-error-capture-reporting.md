# obs · Error Capture & Reporting — category read

## Carried
- (.andromeda/obs-plan.md:496) Any new server-verification error (the pid or start-time mismatch, the check that cannot be made) added to `ChannelError` / `StateError` or another typed seam error must be a typed error with a fixed-message `Display`, with anyhow only at the root-bin dispatch edge.
  > The mechanism is typed errors with fixed-message `Display` impls — hand-written on `PtyError` (`viola-pty` takes no thiserror), thiserror 2.0.20 on `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError`. anyhow 1.0.104 is used at the root-bin dispatch edge and its catch-site reporter (`viola::obs::report_internal_error`) only
- (.andromeda/obs-plan.md:509) A verification error's `Display` must hold no field carrying the recorded pid, the start time, the endpoint name or the snapshot path.
  > 1. fixed `Display` strings on every thiserror type (no fields holding paths or payloads);
- (.andromeda/obs-plan.md:510) A verification that fails while reading `snapshot.json` (a serde source in the chain) must reach stderr, `--json`, MCP `isError` and channel `error.data` only as the fixed message, its full chain written only to the instance detail file.
  > 2. at dispatch, an anyhow chain whose source is a `serde_json::Error` / `serde_path_to_error::Error` is replaced by the fixed message `internal error` before it reaches stderr, `--json`, MCP `isError` or channel `error.data`;
  > 3. the full chain is written as a one-line JSON array `chain:[...]` to `instances/<name>/diagnostics/detail-<process>.ndjson` only, and only when an instance resolves.
- (.andromeda/obs-plan.md:533) A refused verification in every client crosses a crate seam, so it must end in one logged codes-only line (`process-exit{detail}` for the CLI verbs, `hook-decision{detail}` for the hook client), at the Section 6 level.
  > Module-boundary errors (Standard+): every typed error that crosses a crate seam ends in a logged `process-exit{detail}`, `channel-response{result_class:"error", error_code}`, `hook-decision{detail}` or `send-refused`. Each carries codes only, at `WARN`/`ERROR` per Section 6.
- (.andromeda/obs-plan.md:537) The MCP `send` / `wait` / `last` / `answer` tools must report a failed verification as an `isError` code only, with a codes-only log line mirroring it.
  > MCP rejects are `isError` codes only, mirrored by codes-only log lines.
- (.andromeda/obs-plan.md:522) The hook client's verification (`hook.event`, `hook.dialog`) must fail open under the catch site: a mismatch, an unmakeable check or a panic in the check exits 0 with empty stdout.
  > `hook`: `catch_unwind` around dispatch, then exit 0 with empty stdout;
- (.andromeda/obs-plan-detail/error-capture-reporting.md:15) The verification code added to every role's client path runs inside the one main-thread catch site, so no client ends through an uncaught unwind.
  > **Main-thread catch site (every role):** everything after `set_hook` (clap parse, init and role dispatch) runs inside one `std::panic::catch_unwind` in the root bin's `main`. An `Err` maps to the role's exit below, so no role ends through an uncaught unwind (Rust's default exit 101).
- (.andromeda/obs-plan.md:523) The CLI verbs' internal-error path (exit 1, `error: internal error`) is for a dispatch `Err` or panic only; a verification refusal must be mapped to its own exit-21 outcome before it can fall through to this catch site.
  > short-lived `cli`: when an instance resolves, `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` to `cli-<name>.ndjson` (`obs::internal_error_exit_line` covers `Cli` as well as `Run`), the chain in `detail-cli.ndjson` only; then exactly `error: internal error\n` on stderr in one `write_all`, with no hint, and exit 1;
- (.andromeda/obs-plan.md:526) The MCP tools' verification runs in spawned Tokio tasks, so a panic there must stay contained at the task boundary with the hook's line, and the server keeps running.
  > A panic inside a spawned Tokio task is logged by the hook. The runtime contains it at the task boundary, and the process continues.
- (.andromeda/obs-plan-detail/error-capture-reporting.md:23) A verification or strict-modes check placed before any role file opens (the way `hook statusline`'s instance check is) falls in the no-file window, where a panic writes nothing. Any such new pre-log check must be added to this window's list by amendment.
  > - Before step 4 of init, no file exists, so the panic hook writes nothing. This window contains clap parsing and home resolution, and the instance check that `viola revive`'s start arm and `viola hook statusline` run before any role file is opened (chunk 2026-10-10-budget-governor), and `viola mcp`'s check of its home and `diagnostics/`, also run before any role file is opened (chunk 2026-10-17-mcp-server-for-drivers).
- (.andromeda/obs-plan.md:539) A verification failure's detail-file line, if one is written, must carry the same `event`, `corr` and `conn` as its home-level line so `jq` can join them.
  > The detail-file line for a given error carries the same `event`, `corr` and `conn` as its home-level counterpart, so `jq` joins them.
- (.andromeda/obs-plan.md:724) No verification error chain holding a serde source (the snapshot read) may be printed on any external surface.
  > NEVER print an anyhow chain that still contains a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError` or channel `error.data`.

## Bindings
- **§7 module-boundary error line ↔ body §6 log levels**: the refused verification's `process-exit` at exit 21 is a `warn` line, a `-32603` an `error` line; the closed `detail` code for the mismatch is a §6 catalog entry or a Decisions Log extension (.andromeda/obs-plan.md:533) · (.andromeda/obs-plan.md:460)
- **§7 codes-only errors ↔ body §11 PII ban on paths and pids in `detail`**: the compared pid, the start time and the recorded endpoint appear in no `detail` code, `Display` or `isError` text (.andromeda/obs-plan.md:509) · (.andromeda/obs-plan.md:735)
- **§7 error classes ↔ body §4 `wait` / `last` scenario**: `strict-modes-failed` / `server-verify-failed` become possible on `wait` / `last` once this entry lands, and the §4 line naming this route entry as the owner is due for amendment at the wrap (.andromeda/obs-plan.md:533) · (.andromeda/obs-plan.md:361)
- **§7 hook fail-open ↔ security-plan Vector 4 trigger in body §1**: a hook verification mismatch logs to `hook-<name>.ndjson` only, exits 0 with stderr empty (.andromeda/obs-plan.md:522) · (.andromeda/obs-plan.md:200)
- **§7 MCP rejects ↔ body §1 Vector 5 trigger**: rejected tool calls are logged with codes only, with no absolute paths or type names (.andromeda/obs-plan.md:537) · (.andromeda/obs-plan.md:205)
- **§7 module-boundary errors ↔ body §10 Standard+ invariant**: each refused verification is logged once at `WARN`/`ERROR` with a closed `detail`, the full chain going only to the instance detail file (.andromeda/obs-plan.md:533) · (.andromeda/obs-plan.md:643)
