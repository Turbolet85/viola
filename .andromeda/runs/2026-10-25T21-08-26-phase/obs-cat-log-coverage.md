# obs · Log Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/log-coverage.md:26) The squatted-name start refusal must stay one of `process-exit` exit 1's closed `detail` codes, spelled `squatted-name`, unchanged by the chunk's listener work.
  > `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, and from `viola revive`'s start-arm preflight
- (.andromeda/obs-plan-detail/log-coverage.md:27) The `process-exit` `detail` enum in `schemas/diag-line.v1.json` must stay closed at its current count; the chunk adds no exit-1 cause or detail value (scope: no refusal word changes).
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values
- (.andromeda/obs-plan-detail/log-coverage.md:14) The CARRY's `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case asserts a `send-refused` line; whatever fixes or re-measures its log capture must still read the line's catalogued fields, the window word among them, and nothing outside the row.
  > `confirmed`, `refusal`, `detail`, `side` (`client|wrapper`), `wheel`, `window`, `used_percentage`, `resets_at`, `paused`, `duration_ms`,
- (.andromeda/obs-plan-detail/log-coverage.md:32) The window word that CARRY case reads must be the arch `RefusalReason` detail verbatim (`five-hour` / `seven-day`).
  > - Refusal details are the arch `RefusalReason` details verbatim: `manual-pause`, `five-hour`, `seven-day`,
- (.andromeda/obs-plan.md:161) Any exit-1 start cause the chunk touches (the squatted name) must log a distinct `detail` code with no path or pid.
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
- (.andromeda/obs-plan.md:239) The Windows-only listener hardening must not add an OS-specific field or line shape: the sink and schema stay identical on every OS.
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan.md:459) The squatted-name `process-exit` exit 1 must be logged at `error`.
  > | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
- (.andromeda/obs-plan.md:470) The `interprocess` target stays `OFF`: a listener-creation or admission failure the chunk surfaces must be captured at viola's own seam as a viola event, never through interprocess's own tracing.
  > | `rmcp`, `portable_pty`, `sysinfo`, `interprocess` | `OFF` (D-11; failures are captured at the viola seam as viola events) |
- (.andromeda/obs-plan.md:479) Any channel line on the Windows pipe path must stay `channel-request` / `channel-response` codes-only, with no params or result digest.
  > - **IPC invocation** (viola-channel): `channel-request` / `channel-response` with method, `corr`, `conn`, result class, error code and `sender`. **No argument digest**: `params` and result bodies are never recorded, not even hashed.
- (.andromeda/obs-plan.md:557) The pipe name, the SDDL's user SID and any principal the chunk handles must not reach a home-level line; the endpoint is never logged.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan.md:576) Any line the chunk adds must validate against `schemas/diag-line.v1.json` under G4 (default-deny).
  > Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:709) Any log line the chunk adds goes through `obs_event!`, never a raw tracing macro.
  > - NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`.
- (.andromeda/obs-plan.md:731) No field outside the Section 6 catalog may be logged by the chunk's new or changed code.
  > - NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:735) No pipe path, SID or pid may sit inside a `detail` code.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **`squatted-name` exit-1 detail ↔ security-plan §Authentication & Authorization (IPC access control, Windows) and architecture §Conventions (exit codes)**: the first-instance refusal the chunk asserts must end in the closed `process-exit{exit_code:1, detail:"squatted-name"}` at `error`, with the exit code and word unchanged (.andromeda/obs-plan-detail/log-coverage.md:26), (.andromeda/obs-plan.md:459)
- **closed `detail` enum ↔ `schemas/diag-line.v1.json` / G4 (obs §9)**: no new exit-1 value; any added line conforms under default-deny (.andromeda/obs-plan-detail/log-coverage.md:27), (.andromeda/obs-plan.md:576)
- **`send-refused` row and `RefusalReason` window words ↔ test-plan (the CARRY's libtest log-capture case in `src/run/send/budget.rs`)**: the case's fix or three-run re-measure reads the catalogued `send-refused` line with `window` / `detail` `five-hour` | `seven-day` (.andromeda/obs-plan-detail/log-coverage.md:14), (.andromeda/obs-plan-detail/log-coverage.md:32)
- **`interprocess` target OFF ↔ security-plan (interprocess defaults `accept_remote(false)`, first-instance, non-inheritable)**: admission failures surface as viola events at the seam (.andromeda/obs-plan.md:470)
- **endpoint never logged ↔ security-plan §Error Handling / NEVER-log floor**: the pipe name, SID and SDDL stay out of home-level lines and `detail` codes (.andromeda/obs-plan.md:557), (.andromeda/obs-plan.md:735)
- **same schema on every OS ↔ test-plan Windows CI leg**: Windows-only asserts add no OS-specific field (.andromeda/obs-plan.md:239)
