# obs · Log Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/log-coverage.md:26) A present pinned copy that fails its re-hash must still end `run`'s start as the exit-1 `pinned-hash-mismatch` detail, and any pin-write failure the race fix surfaces must take a code already on this closed exit-1 list (`internal-error` included), never a new free-text cause.
  > `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, and from `viola revive`'s start-arm preflight
- (.andromeda/obs-plan-detail/log-coverage.md:27) The home-tree check over `bin/<key>/`, the pinned copy and `plugin/<key>/` must keep running before the wrapper log opens, and a refusal must keep writing no line, even when a second start meets a copy another start is still writing.
  > `viola run` and `viola revive`'s start arm also run the home-tree check before the wrapper log opens (`strict::check_home_tree` through `checked_exe` and `refuse_tree`: the home to the read rule, `check_instance`'s set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists); its refusal is the home pair (a home-level refusal) or the instance pair (an instance-level one) on stderr and exit 1, with no log opened, no line and no child (revive under `--json` prints the `strict-modes-failed` document), so a refused home tree carries no `process-exit` detail either.
- (.andromeda/obs-plan-detail/log-coverage.md:28) The `process-exit` `detail` enum stays closed at 13 values: a fix that adds an exit cause must extend `schemas/diag-line.v1.json` and this count together, or it adds no code.
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
- (.andromeda/obs-plan-detail/log-coverage.md:36) `pinned-hash-mismatch` must remain the filesystem-refusal telemetry carried on `process-exit` exit 1, unchanged by the absent-copy write's concurrency fix.
  > `pinned-hash-mismatch` (on `process-exit` exit 1), `symlink-ignored`.
- (.andromeda/obs-plan-detail/log-coverage.md:10) If the exe's arm on Windows takes the same sharing-violation wait as `pin_companions`, the companions' degrade must keep these closed `sideload_fallback` values, never a refusal or an exit change, and the exe arm adds no `sideload_fallback` value.
  > `sideload_fallback` (claude-child, Windows, present only on a degrade: `hash-mismatch|unreadable|load-failed|not-built`, or `strict-modes-failed`, the last when the write-rule check `pin_companions` makes before its held open refuses `bin/<key>/conpty/` or a companion in it, the child then on the inbox ConPTY with its exit unchanged and nothing written; D-36)
- (.andromeda/obs-plan.md:161) Every exit-1 cause on the `run` start path, the pin step included, must use a distinct `detail` code that holds no path or pid.
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
- (.andromeda/obs-plan.md:215) Pinned-exe hash mismatches must keep their refusal telemetry.
  > plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks;
- (.andromeda/obs-plan.md:451) Any `detail` code the chunk writes must come from the closed kebab-case catalog and must never be a path or pid.
  > **`detail` code catalog** (closed, kebab-case, never a path or pid; Founder Direction 4):
- (.andromeda/obs-plan.md:459) An exit-1 `process-exit` from the pin step (`pinned-hash-mismatch`, `internal-error`) must be logged at `error`.
  > | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
- (.andromeda/obs-plan.md:557) No line the race fix or its witness adds may carry the pinned path or the endpoint.
  > `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan.md:731) No field outside the Section 6 catalog may be logged: a retry count, a wait duration or the build hash on the pin step needs a catalog entry first.
  > - NEVER log any field not named in the Section 6 catalog (default-deny).

## Bindings
- **`pinned-hash-mismatch` ↔ security.md hash-before-reuse rule (scope Boundaries)**: the re-hash refusal's exit-1 line stays as it is while only the absent-copy write's concurrency changes, (.andromeda/obs-plan-detail/log-coverage.md:36) · (.andromeda/obs-plan.md:215)
- **closed `process-exit` `detail` enum ↔ body §9 G4 schema conformance / §8 default-deny**: a new exit code must land in `schemas/diag-line.v1.json` and in the stated count together, or G4 fails, (.andromeda/obs-plan-detail/log-coverage.md:28) · (.andromeda/obs-plan.md:731)
- **home-tree check writes no line ↔ «Home and code-bearing file integrity»'s landed strict-modes rules**: the chunk changes neither the check's order before the log nor its silent refusal, (.andromeda/obs-plan-detail/log-coverage.md:27)
- **`sideload_fallback` ↔ scope A's Windows `pin_companions` sharing-violation wait**: whether the exe arm needs the same wait is P3's to read, and the companions' closed degrade values stay unchanged, (.andromeda/obs-plan-detail/log-coverage.md:10)
- **scope B reproducible-build measurement ↔ Log Coverage**: (none), since the measurement is a build comparison and writes no product log line
