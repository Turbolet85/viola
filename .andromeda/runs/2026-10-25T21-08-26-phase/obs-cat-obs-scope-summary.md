# obs · Obs Scope Summary — category read

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) Any instrumentation the chunk adds to the `viola-channel` listener (`server.rs`, `server/win.rs`, `endpoint.rs`) must be sync, with no Tokio in the crate's normal graph.
  > N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) The named-pipe admission controls (remote-client rejection, first-instance, DACL) are observed only at viola's own bind and connect call sites, never by instrumenting the OS internals.
  > Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:133) The pipe name is the snapshot's `endpoint`, so no refusal, `--json` output or error the chunk touches may carry it.
  > Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:162) The squatted-name start refusal must stay a fixed stderr string with no path, pid, chain or upstream text.
  > Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:163) The squatted-name refusal's exit 1 must be logged as a field, with its own distinct detail code.
  > Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  >   - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:237) `run`'s logger must be up before the start sequence, so a bind refused on a held pipe name is logged.
  > For `run`, this completes before the start sequence, so the collision check and version gate are logged.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:299) `channel.bind` is a must-trace span on the `run` start path, and the chunk's listener changes must keep it in place.
  > - **Must-trace spans:** `run.start` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:300) A squatted-name start refusal must log `process-exit` with the required fields and the distinct `squatted-name` detail code, with no path or pid.
  > - **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child,
  > and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). No path or pid appears in the detail.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:450) Whatever the Windows named-pipe path logs must match the Unix-socket path's sink and fields exactly, with no Windows-only exporter.
  > - **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  >   - permission application: Windows DACL vs Unix 0600/0700;
  >   - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  >   - named pipe vs Unix socket in channel logs;
  >   - ConPTY vs openpty in `pty` spans.
  > 
  >   No OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:485) The squatted-name cause must keep a detail code of its own, separate from the other exit-1 causes, with no path or pid.
  > - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:344) The CARRY obligation (`send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4) asserts the budget path's `send-refused` line, so its capture fix must keep that event and its required fields.
  > Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:345) The fields the budget-paused `send-refused` line must carry, which the CARRY case reads.
  > - **Required log fields:** `event`, `process`, `instance`, `corr`, `window` (`five-hour` / `seven-day`), `used_percentage`, `resets_at`, `paused`, `refusal="budget-paused"`, `exit_code` (11),
- (.andromeda/obs-plan-detail/obs-scope-summary.md:268) `corr` on a `send-refused` line is the send cursor, which is what the CARRY case's "no cursor" assertion checks.
  >     - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
- (.andromeda/obs-plan-detail/obs-scope-summary.md:273) The CARRY fix may change the test's capture, but it must not rename or remove a required field on the line.
  >   obs-plan may add fields but must not rename or remove these. The harness greps on them.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:476) `send-refused` must stay a logged event that the CARRY case can read under both plain `cargo test` and nextest.
  >   - `send-issued` and `send-refused` are logged events.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:504) The CARRY case's log-capture fix must not wait on the line with a sleep.
  >   - No log-based waits on sleeps.
- (.andromeda/obs-plan.md:264) Where §1 differs from §3, §6 or §12, those sections win until a wrap reconciles them, so the items above are read under them (the `corr` null encoding and the D-25 channel join key included).
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21), `hook-invoked` `corr` timing (D-07), the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`), and the MSRV floor (D-22: 1.96).

## Bindings
- **squatted-name exit-1 detail ↔ security-plan §Authentication & Authorization (Windows pipe first-instance) and architecture §Conventions (exit codes, refusal words)**: the scope keeps `squatted-name` at exit 1. On the obs side that refusal must log `process-exit` with its own detail code and no path or pid; on the security side it is the first-pipe-instance control the chunk asserts as a listener property (.andromeda/obs-plan-detail/obs-scope-summary.md:300) (.andromeda/obs-plan-detail/obs-scope-summary.md:485)
- **`channel.bind` must-trace span ↔ obs-plan §4 Scenario `viola run` start sequence**: §1 names the span, and §4 bans the endpoint name in any span or a path or pid in a failure's `detail`. Both sides bind any bind-path edit the chunk makes (.andromeda/obs-plan-detail/obs-scope-summary.md:299) (.andromeda/obs-plan.md:345)
- **endpoint scrub ↔ security-plan §Error Handling (fixed messages, no paths)**: the pipe name stays out of every external output, both the stderr refusal pair and the error bodies (.andromeda/obs-plan-detail/obs-scope-summary.md:133) (.andromeda/obs-plan-detail/obs-scope-summary.md:162)
- **multi-OS identical fields ↔ test-plan CI matrix (`windows-2025` leg)**: the Windows assertions the chunk adds run on the leg where named-pipe channel lines must match the Unix-socket ones field for field (.andromeda/obs-plan-detail/obs-scope-summary.md:450)
- **budget `send-refused` line ↔ test-plan (the CARRY case_4 under plain `cargo test`) and obs-plan §6 (`corr` null by key absence)**: the case's capture reads the line's `refusal` and the absence of `corr`. §1 fixes what `corr` holds, and §6 fixes how null is encoded (.andromeda/obs-plan-detail/obs-scope-summary.md:268) (.andromeda/obs-plan-detail/obs-scope-summary.md:345) (.andromeda/obs-plan.md:440)
- **§1 verbatim copy ↔ obs-plan §3 / §6 / §12**: where they differ, the later sections win until a wrap brings §1 current (.andromeda/obs-plan.md:264)
