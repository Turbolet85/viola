# obs · Obs Scope Summary — category read

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:300) The `run` start path's must-trace spans keep `run.pin_copy` between the collision check and the version gate, and the product event order a two-sided witness reads (`session-start` landing) stays as stated; a race fix must keep that span and order.
  > - **Must-trace spans:** `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure. The product event order to verify in `events.ndjson` is `wheel{cause:"start"}` → `budget-gate` → `session-start`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:301) The start's exit-1 causes, the pinned copy's sha-256 mismatch among them, must each keep a distinct `detail` code; a cause the race fix adds needs its own.
  > - **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:488) Founder Direction 4 requires a distinct, path-free and pid-free `detail` code for every exit-1 cause, the SHA-256 mismatch included; the pin change must not merge or blur those causes.
  > Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:398) Refusal telemetry for a pinned-exe hash mismatch must stay, and the Windows `conpty/` companion's refusal stays a `sideload_fallback` on `process-start` with the exit unchanged, whatever the exe arm's concurrency fix does.
  > - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) A lock or rename the pin write adds or changes is instrumented at viola's call site, not in the OS internals.
  > Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:133) The `pinned_bin` absolute path must stay out of external output.
  > - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:238) The `run` logger is initialised before the start sequence, so the pin step (inside that sequence) is logged; the fix must not move the pin ahead of the log init.
  > For `run`, this completes before the start sequence, so the collision check and version gate are logged.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:159) `viola verify`, the other caller of `pin_exe`, logs as `cli` only after its home-tree check passes; a refused check writes no line.
  > `viola verify` logs as `cli` (with an instance) once its home-tree check (`check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`) has passed; a refused check exits 1 with the home pair, opens no log and writes no line, and a refused `update_stamps` (`StampsError::StrictModes`) exits 1 with the same pair.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:452) The Windows arm of the pin fix (a sharing-violation wait or same-bytes acceptance) and the Unix arm must yield one sink and identical fields.
  > - **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
- (.andromeda/obs-plan-detail/obs-scope-summary.md:506) The two-sided race witness must key on hook events and `events.ndjson` offsets.
  > - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:508) The witness must not wait on logs through sleeps.
  > - No log-based waits on sleeps.
- (.andromeda/obs-plan.md:345) The start scenario's span and line bans hold for the pin and sideload paths: no `pinned_bin` path or companion paths or hashes in a span, a sideload outcome never a refusal or exit change, and no path or pid in a failure's `detail`.
  > - Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change; a failure's `detail` carries no path or pid; the child's `process-exit` comes from the handle-wait thread, never on reader EOF.
- (.andromeda/obs-plan.md:311) A tampered pinned copy stays a start refusal written as exactly two fixed stderr lines before any child, with no path or pid.
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid)
- (.andromeda/obs-plan.md:215) The Vector 7 trigger keeps refusal telemetry for pinned-exe hash mismatches.
  > plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks;
- (.andromeda/obs-plan.md:576) Any field the race fix logs (a wait or retry count, an outcome) must first be named in the Section 6 catalog.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog.
- (.andromeda/obs-plan.md:715) A new `event` value for the pin race needs a Decisions Log entry.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:735) No `detail` code the chunk adds may carry a path or pid.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:600) The concurrent-start witness's viola homes must live under `target/e2e-home/` so G2, G4 and the secret scan cover them.
  > Every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it — a `hook` panic exits 0, so a home outside that root would hide it — except the one `tests/chaos_feed_panic.rs` home (below).

## Bindings
- **`run.pin_copy` span and exit-1 `detail` codes ↔ security-plan §Local trust boundary (Home: re-hash before reuse, never overwritten)**: the obs side keeps the hash-mismatch refusal's distinct code and telemetry, the security side keeps the rule that a differing present copy is refused, never replaced (.andromeda/obs-plan-detail/obs-scope-summary.md:300) (.andromeda/obs-plan-detail/obs-scope-summary.md:398)
- **Founder Direction 4 detail codes ↔ the body's §6 `detail` catalog and §12 Decisions Log**: a new exit cause, field or event from the race fix enters the closed catalog or enum through a Decisions Log entry, path-free (.andromeda/obs-plan-detail/obs-scope-summary.md:488) (.andromeda/obs-plan.md:576) (.andromeda/obs-plan.md:715)
- **Multi-OS identical fields ↔ the scope's Windows `pin_companions` sharing-violation wait**: the exe arm's Windows handling yields the same lines as Unix (.andromeda/obs-plan-detail/obs-scope-summary.md:452)
- **Deterministic emission ↔ test-plan §3 harness and the two-sided race witness**: the witness reads `session-start` by `events.ndjson` offsets with no sleep-based waits, its homes under `target/e2e-home/` (.andromeda/obs-plan-detail/obs-scope-summary.md:506) (.andromeda/obs-plan.md:600)
- **`pinned_bin` scrub ↔ security-plan §Error Handling (no absolute paths in external errors)**: the pinned path stays out of external output and span fields (.andromeda/obs-plan-detail/obs-scope-summary.md:133) (.andromeda/obs-plan.md:345)
