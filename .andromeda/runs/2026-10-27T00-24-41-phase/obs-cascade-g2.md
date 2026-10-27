# obs · cascade read · group 2 of 5

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:186) The hook spawn the race fails, and any reader of a concurrently rewritten `plugin/<key>/` `hooks.json`, must keep the hook's file-only sink, empty stderr and exit 0: a failure there is observed only as this outcome, never as a new line holding a path.
  > **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
  (.andromeda/obs-plan.md:119)
  > a file sink only, to `diagnostics/hook-<name>.ndjson`; stderr is forbidden, and stdout carries only the decision body (for `viola hook statusline`, the user's status-line command's output); a failed log write is swallowed and the hook still exits 0;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:187) A hook reached on an absent or refused instance during the race must stay a silent exit 0 with no line, and any build change the reproducible-build measurement motivates must keep `service.version` compile-time with no CI or env metadata injected.
  > If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`"). The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append.
  (.andromeda/obs-plan.md:133)
  > `service.version` is compile-time `env!("CARGO_PKG_VERSION")`. `deployment.environment` is N/A (local-only).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:233) The chunk edits `verify`'s `pin_exe` call and `run`'s start `pin_bytes` call: `verify`'s home-tree check and `run`'s `checked_exe` + `check_home_tree` must stay ahead of the log, a refused tree writing no line, and the pin step must stay inside the start sequence that runs after the log opens, so its outcome is logged.
  > 3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument). Where the role checks the home before its first open, that strict-modes check runs here — every `viola hook` arm but the two `--capture` arms (`canonical` + `trusted`); `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`refuse_untrusted`); `verify` (`check_home_tree`); `run` and `viola revive`'s start (`checked_exe` + `check_home_tree`) — and a refusal ends the process with nothing under the home opened and no line.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:238)
  > For `run`, this completes before the start sequence, so the collision check and version gate are logged.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:231)
  > 1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr.
  (.andromeda/obs-plan.md:115)
  > `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns; instrumentation is sync, with no Tokio;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:300) The fix edits the absent-copy arm behind `run.pin_copy`: the span must stay in its place in the start order, between `run.collision_check` and `run.version_gate`, and a failure in it must still end in `process-exit`.
  > **Must-trace spans:** `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:301) A new or changed exit-1 cause on the absent-copy write (a lost race that cannot be settled) must take a distinct closed `detail` code beside the sha-256 mismatch, with no path or pid, while the home-tree refusal ahead of the log keeps writing no line.
  > Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe`) refuses writes no line either, so that exit-1 cause has no `detail` code: the check precedes the wrapper log, and the output is the home pair (a home-level refusal) or the instance pair (an instance-level one), revive `--json` the `strict-modes-failed` document. No path or pid appears in the detail.
  (.andromeda/obs-plan.md:161)
  > exit-1 causes use a distinct `detail` code, with no path or pid in it.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:371) The race's failed hook spawn and any concurrent plugin-rewrite read failure must be observed only through the hook's existing fail-open telemetry, exit 0 and stderr empty, adding no new path-bearing line.
  > Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:373)
  > Exit is 0 and stderr stays empty.
  (.andromeda/obs-plan.md:200)
  > fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:398) When a concurrent start finds a present copy and re-hashes it, the pinned-exe hash-mismatch refusal telemetry must keep its line shape and level, and if the exe's Windows arm takes `pin_companions`' wait the `conpty/` `strict-modes-failed` line must stay as it is, exit unchanged.
  > Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged; a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`) leaves only verify's own `process-exit` (with an instance; exit 1, no `detail`).
  (.andromeda/obs-plan.md:215)
  > refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged);
- (.andromeda/obs-plan-detail/obs-scope-summary.md:406) The `claude` child's `process-start` is the line `pin_companions`' outcome lands on as `sideload_fallback`: an edit to the Windows x64 `conpty/` arm must keep that line and its values, a sideload outcome never a refusal or an exit change.
  > `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:414) Any instrumentation or dependency the pin fix adds to `viola-state` (a lock or wait in the absent-copy arm included) must stay Tokio-free, add no C-building crate and pass `cargo deny` on all three targets.
  > Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
  (.andromeda/obs-plan.md:225)
  > obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:452) If the exe's Windows arm takes a wait or a line the Unix arm does not, the line must carry identical fields on every OS, the permission application differing only underneath.
  > - **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  >   - permission application: Windows DACL vs Unix 0600/0700;
  (.andromeda/obs-plan-detail/obs-scope-summary.md:459)
  > No OS-specific exporter.
  (.andromeda/obs-plan.md:239)
  > the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:478) The two-sided race witness must read the failed boot and the missing `session-start` through the harness `logs` merge of `diagnostics/` with `events.ndjson` and `status`'s closed `state` / `last_error`.
  > Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  (.andromeda/obs-plan.md:146)
  > The agent-readable consumption path is the tests harness `logs` command (`scripts/agent-run.sh logs` / `.ps1`, via `viola-harness`). It merges `events.ndjson` (`src:"events"`, byte `offset`) with `diagnostics/*.ndjson` (`src:"diag"`), emits torn lines as `{"torn":true}`, and filters by `--instance`, `--kind`, `--process` and `--after`. The output is ndjson on stdout, ready for `jq` or pasting. `agent-run status` provides the aggregate health snapshot: `state` and `last_error` closed enum (tests excerpt, Status JSON shape).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:488) Each exit-1 cause the chunk keeps or adds on the pin step (the SHA-256 mismatch, and any lost-race cause) must carry its own distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line.
  > Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
  (.andromeda/obs-plan.md:253)
  > each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:506) The race reproduction and its clean re-read must wait on hook events and `events.ndjson` offsets, never on a sleep or screen content, so the witness cannot pass by timing alone.
  > - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
  >   - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  >   - No log-based waits on sleeps.

## Bindings
- **obs-scope-summary's run path (must-trace spans, exit-1 `detail` codes) ↔ span-trace-coverage**: the `run.pin_copy` span the fix edits sits in the scenario «`viola run` start sequence to child spawn», which owns the span tree and its fields (.andromeda/obs-plan.md:339), while §1 fixes its order (.andromeda/obs-plan-detail/obs-scope-summary.md:300).
- **obs-scope-summary's exit-1 `detail` rule ↔ log-coverage and the Obs Decisions Log**: a lost-race exit-1 cause needs its distinct code (.andromeda/obs-plan-detail/obs-scope-summary.md:301) and that code's catalog row and Decisions Log entry (.andromeda/obs-plan.md:426, .andromeda/obs-plan.md:765).
- **obs-scope-summary's Vector 7 refusal telemetry ↔ the security plan's hash-before-reuse rule**: a concurrent start that finds a present copy re-hashes it and refuses on any difference, never overwriting, and the refusal's line keeps its shape (.andromeda/obs-plan-detail/obs-scope-summary.md:398, .andromeda/obs-plan.md:215).
- **obs-scope-summary's supply-chain signal ↔ ci-integration**: a dependency or lock the fix adds to `viola-state` is judged by the per-sync-crate `deny-sync.toml` gate CI runs (.andromeda/obs-plan-detail/obs-scope-summary.md:414, .andromeda/obs-plan.md:588).
- **obs-scope-summary's deterministic-emission signal ↔ slo-invariants-telemetry-budgets and the harness contract**: the race witness's waits are keyed on events and offsets (.andromeda/obs-plan-detail/obs-scope-summary.md:506) under the zero-flakiness rule, a red kept open until its cause is known (.andromeda/obs-plan.md:629), read through the harness `logs` / `status` (.andromeda/obs-plan.md:303).
- **obs-scope-summary's hook fail-open signal ↔ log-coverage**: the race's failed hook spawn and a concurrent plugin rewrite are observed only in the existing `hook-<name>.ndjson` outcome, adding no path-bearing line (.andromeda/obs-plan-detail/obs-scope-summary.md:371, .andromeda/obs-plan.md:426).
