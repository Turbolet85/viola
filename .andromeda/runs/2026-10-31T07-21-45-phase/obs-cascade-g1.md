# obs · cascade read · group 1 of 9

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:17) The `v1-47` re-read of detail files and of the secret scan must read home-level role files and instance detail files each at its own root, the §3, §6 and §12 reconciliation winning over this older wording.
  > the `diagnostics/` directory root (tests `<home>/diagnostics/` vs arch and security `instances/<name>/diagnostics/`);
  (.andromeda/obs-plan.md:144)
  > obs aligns to tests on the basenames `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson` and the `logs` wrapper; the directory root is reconciled with a Decisions Log entry, and a home-level `mcp.ndjson` never carries user content; every file is 0600 inside a 0700 directory; there is no rotation in v1.
  (.andromeda/obs-plan.md:264)
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21), `hook-invoked` `corr` timing (D-07), the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`), and the MSRV floor (D-22: 1.96).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:18) The MCP witness and the pre-`initialize` oversize case must read `viola mcp`'s record from `mcp.ndjson`, treat stderr JSON as its sink only when that file cannot be opened, and hold both sinks to codes only.
  > `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
  (.andromeda/obs-plan.md:127)
  > always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened; stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
  (.andromeda/obs-plan.md:205)
  > rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:19) Telling the `-32602` causes apart, `release-from-driver` among them, must add a field or take a Decisions Log entry for any new `event` value, and must rename and remove nothing on the `channel-response` line.
  > the `event` enum extensions (`release-from-driver` and the chaos signals);
  (.andromeda/obs-plan.md:137)
  > Obs may add fields and must not rename or remove any.)
  (.andromeda/obs-plan.md:139)
  > Alignment gaps for Phase 3. Each needs additive fields or an enum extension through a Decisions Log entry; nothing is renamed.
  (.andromeda/obs-plan.md:140)
  > (a) Founder Direction 4 says `release-from-driver` "must log as its own event". It is not in the closed `event` enum, so it needs a Decisions Log entry.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:28) The root bin must turn a serde-sourced anyhow chain into a fixed message before stderr, keep the full chain out of every surface but the diagnostics detail file, and hold fault lines free of paths and upstream text.
  > **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan.md:47)
  > the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan.md:95)
  > the absolute paths in `snapshot.json` and config values are scrubbed from external output.
  (.andromeda/obs-plan.md:111)
  > refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:36) The closed `refusal` and `detail` codes on the CLI and MCP surfaces, and any new code the chunk adds for a cause or a folded refusal, must be viola-core's kebab-case values, and viola-core must keep emitting nothing.
  > It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing.
  (.andromeda/obs-plan.md:51)
  > a pure type crate with no I/O that supplies log-field values but emits nothing
- (.andromeda/obs-plan-detail/obs-scope-summary.md:44) The `v1-47` content leg and the `PtyError` message must keep the PTY byte stream, as user content, out of every error body and every log line.
  > The PTY byte stream is user content and is never logged (security Data Classifications, user content; brief §3.2 "Nobody parses the rendered screen for content").
  (.andromeda/obs-plan.md:55)
  > the PTY byte stream is user content, never logged.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:50) The channel enumeration must cover the error bodies a wrapper answers across this method list, each with its fixed message, a code or null in `error.data`, and no `params` or result body.
  > Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:190)
  > channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) Any instrumentation change the chunk makes in viola-channel for the `-32602` causes must stay sync, with no Tokio in the crate's graph unless its feature is enabled.
  > N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
  (.andromeda/obs-plan.md:59)
  > instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:68) The planted stripped `CLAUDE*` value must reach no sink at all, the strip staying a count or names only, and drift reports and the `claude_env_keep` reader's values must stay out of external output, in files held to 0600 in 0700.
  > **Reason if not fully instrumentable:** N/A. Its outputs are the most sensitive in the product:
  >   - Drift reports go to `diagnostics/` only (security Vector 4).
  >   - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
  (.andromeda/obs-plan.md:67)
  > instrumentable, its outputs the most sensitive in the product: drift reports go to `diagnostics/` only, and R8-stripped `CLAUDE*` values are never logged anywhere.
  (.andromeda/obs-plan.md:99)
  > static configuration and data; only the parse at load time is observable.
  (.andromeda/obs-plan.md:200)
  > fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
  (.andromeda/obs-plan.md:210)
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values, and Phase 3 confirms the names are admissible; a serde-sourced anyhow chain becomes a fixed message before stderr; no env, flag or config may disable redaction.
  (.andromeda/obs-plan.md:215)
  > `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:99) The owed test of a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes must find the line refused whole, its one `parse-rejected` line with no byte of the input, and the session ended as at a closed stdin.
  > `viola mcp` reads that payload in lines of at most `MAX_FRAME`, a longer line refused whole. Instrument `hook-invoked` / `hook-decision`, MCP tool-call handling and the `viola mcp` stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line).
  (.andromeda/obs-plan.md:125)
  > its stdin read in lines of at most `MAX_FRAME`, a longer line refused whole and the session ended as at a closed stdin
- (.andromeda/obs-plan-detail/obs-scope-summary.md:107) The cap-driving case of the child pipe drain must find a parse failure surfacing as `unknown` plus its counter, with the untrusted rows in no line.
  > External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
  (.andromeda/obs-plan.md:83)
  > boundary-only — an external CLI: its spawn/exit and parse outcome; a parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
  (.andromeda/obs-plan.md:220)
  > parse failures surface as `unknown` plus a counter, and C0/C1 controls are escaped in any human terminal rendering of logged rows.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:115) The cap-driving case of the status-line command's stdout must find only the shell-out's duration and exit status logged, and the command string outside every home-level file.
  > A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
  (.andromeda/obs-plan.md:87)
  > boundary-only — a user-written shell command: its duration and exit status only; the command string is never logged outside `diagnostics/`.
  (.andromeda/obs-plan.md:181)
  > the `statusline_command` string never appears outside `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) The enumeration must name the body a failed server check gives on each surface, `instance-unreachable` with no line on MCP and `server-verify-failed` on the CLI, its hardening staying with the chunk that owns it.
  > Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.
  (.andromeda/obs-plan.md:91)
  > boundary-only — viola's call sites, not the OS internals.
  (.andromeda/obs-plan.md:253)
  > `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
- (.andromeda/obs-plan.md:310) The status-line cap-driving case must hold `viola hook statusline`'s stdout to the user's output byte for byte or to nothing on these arms, with no telemetry on it.
  > The one other thing a `hook` stdout carries is `viola hook statusline`'s: the user's status-line command's output byte for byte (nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing). No telemetry reaches stdout on either arm.
- (.andromeda/obs-plan.md:311) Either outcome of the `Refusal` carry must keep the `.cmd`/`.bat` start refusal's two fixed stderr lines, with no path or pid, through this one writer.
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed
- (.andromeda/obs-plan.md:312) The `v1-47` detail-file leg must find role logs codes-only and content-bearing detail in the instance's detail file alone.
  > Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.

## Bindings
- **The `-32602` causes on the `channel-response` line ↔ §5 and §6**: the additive field or closed value (.andromeda/obs-plan.md:139) must be named in the §6 catalog and must retire the recorded over-count of `viola.compat.v_rejected` (.andromeda/obs-plan.md:418), its level read from the §6 mapping (.andromeda/obs-plan.md:460).
- **The serde-sourced chain ↔ §7 scrubbing layers**: the root-bin boundary (.andromeda/obs-plan-detail/obs-scope-summary.md:28) and the ban (.andromeda/obs-plan.md:724) hold together with layers 1 to 3 of §7 (.andromeda/obs-plan.md:510), the chain going only to the instance detail file.
- **The `Refusal` carry ↔ §7's enum list**: a fold into `AgentError` or a ratified exception must agree with the fixed-`Display` list (.andromeda/obs-plan.md:496) and keep the start refusal's fixed pair (.andromeda/obs-plan.md:311).
- **The never-log floor ↔ §8 and §9 secret scan**: the floor (.andromeda/obs-plan.md:734) is proved by the scan's patterns and inputs (.andromeda/obs-plan.md:570, .andromeda/obs-plan.md:621), so a witness's planted value must fail that scan only if a real leak carries it into those inputs.
- **The `viola mcp` stdin cut ↔ §7 MCP rejects and §6 `detail` catalog**: the one `parse-rejected` line (.andromeda/obs-plan-detail/obs-scope-summary.md:99) and the codes-only sink (.andromeda/obs-plan.md:127) bind with the rejected-call rule (.andromeda/obs-plan.md:537).
