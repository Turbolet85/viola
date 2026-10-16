proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Capability ledger → `verify` / `plugin install` (first bullet)'
    change: '`verify` logs its seven child spawns between `process-start` / `process-exit`: `version-probe`, `verify-probe`, then one `verify-pty-probe` pair for each of its five interactive PTY runs (Run A, Run B, Run C, Run D, then Run E, the `--resume <id>` start of Run B''s first session id in its own 0700 `<cwd>/.viola-verify-<pid>-resume/`, no key typed, no `--settings`), with no row text, prompt, session id or path in any line; "so the seven pairs are the uninterrupted run''s" and "The seven spawn pairs stay seven" replace the two "six" sentences.'
    sidecar: 'obs-plan §4 verify: the spawn set grows to seven pairs, five `verify-pty-probe` (Run E, the resume run, chunk 2026-10-14-resume-and-compact-ledger-rows).'
    rationale: 'Report Changes, Symbols: Run E `resume_run` is `measure`''s fifth `RunStep` spawned through `Run::spawn` with `io::empty()`; Counts "Spawn pairs" 6 → 7 (`version-probe`, `verify-probe` and five `verify-pty-probe`), pattern `spawn-pairs` standing at obs-plan.md:739; Coverage: Run E instrumentation is a `verify-pty-probe` spawn pair, no id or path in any line. The doc enumerates four runs / six pairs, so the new hot-path spawn is undescribed.'
    basis: 'obs-plan.md:739'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Capability ledger → `verify` / `plugin install` (CI bullet)'
    change: 'In CI `viola verify` runs only against the fake agent — its print mode plus the five interactive PTY runs (Run E, the resume run, included) over `--screens --turn-stop --trusted-root <workspace root> --dialogs --framing` — at the recorded version 2.1.287.'
    sidecar: 'obs-plan §4 verify CI bullet: four → five interactive PTY runs against the fake agent (Run E).'
    rationale: 'Report Harness / gate surface: `boot`''s `viola verify` stamp against the fake agent now runs Run E, print probe and five interactive runs; Counts "verify''s interactive runs" 4 → 5, pattern `verify-runs` standing at obs-plan.md:742.'
    basis: 'obs-plan.md:742'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6 Log Coverage → Boundary-call wrappers → Child / shell spawns'
    change: '`verify-pty-probe`: each of verify''s five interactive PTY runs (Run A through Run E); verify logs its seven spawns at the call site, and `run_bounded` itself stays unlogged.'
    sidecar: 'obs-plan §6 Child / shell spawns: verify''s five interactive PTY runs and seven spawns (Run E).'
    rationale: 'Report Counts "Spawn pairs" 6 → 7 and "verify''s interactive runs" 4 → 5; patterns `spawn-pairs` and `verify-runs` standing at obs-plan.md:871.'
    basis: 'obs-plan.md:871'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6 Log Coverage → Boundary-call wrappers → Child / shell spawns (the `diag-line.v1.json` subject sub-bullet)'
    change: 'each of the seven `verify` spawn pairs'' `process-exit` carries `child_exit_status` and `duration_ms` (D-35); the closed `subject` enum is unchanged (Run E reuses `verify-pty-probe`).'
    sidecar: 'obs-plan §6 subject sub-bullet: six → seven verify spawn pairs; no new subject value.'
    rationale: 'Report Counts "Spawn pairs" 6 → 7, pattern `spawn-pairs` standing at obs-plan.md:872; Outcome (obs) met: `viola verify` writes seven spawn pairs, each `process-exit` with `child_exit_status` and `duration_ms`.'
    basis: 'obs-plan.md:872'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Capability ledger → `verify` / `plugin install` (first bullet, the step-line count)'
    change: '`MM` the relied count (`LedgerRow::relied()`, `ALL` filtered in its order; a row is relied unless it is one of the three statusline rows where `statusline::WRAPPED_HERE` is false): 22 on Unix, 19 on Windows, the 22nd row `resume-session` relied on every OS.'
    sidecar: 'obs-plan §4 verify step lines: relied count 21/18 → 22/19 (`resume-session`).'
    rationale: 'Report Symbols: `LedgerRow::ResumeSession` is the 22nd member of `LedgerRow::ALL`, relied under `_ => true`; Counts "Ledger rows": relied set 21 → 22 Unix, 18 → 19 Windows, MM = 22 / 19; patterns `rows21` and `rows18` standing at obs-plan.md:739.'
    basis: 'obs-plan.md:739'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Capability ledger → `verify` / `plugin install` (first bullet, the stop sentence)'
    change: 'A `verify` stopped by Ctrl-C or TERM kills and reaps the child it is running and closes every pair it opened, a child killed at the stop closing with `child_exit_status` (the pump''s exit code: portable-pty reports a code for a signalled child) and `duration_ms`, then ends `process-exit{subject:"self", exit_code:1, detail:"interrupted"}`.'
    sidecar: 'obs-plan §4 verify stop: the killed child''s `process-exit` carries `child_exit_status` (measured; the "no `child_exit_status`" wording retired).'
    rationale: 'Report Spec claims disproved 1: "a child killed at the stop closes with `duration_ms` and no `child_exit_status`" is false; measured, verify''s interactive PTY runs close a killed child with the pump''s exit code as `child_exit_status` (stop case `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, tests/cli_verify.rs:1071-1178); pattern `killed-exit` standing at obs-plan.md:739. Pre-existing behaviour, not this chunk''s code.'
    basis: 'tests/cli_verify.rs:1071-1178'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6 Log Coverage → Boundary-call wrappers → Child / shell spawns (the `diag-line.v1.json` subject sub-bullet)'
    change: 'Drop the exception "except a child killed at a Ctrl-C or TERM stop, whose `process-exit` carries `duration_ms` and no `child_exit_status`": every verify spawn pair''s `process-exit`, a child killed at a stop included, carries `child_exit_status` and `duration_ms`; a stopped run writes only the pairs it started.'
    sidecar: 'obs-plan §6 subject sub-bullet: the killed-at-stop exception to `child_exit_status` retired (disproved by measurement).'
    rationale: 'Report Spec claims disproved 1, pattern `killed-exit` standing at obs-plan.md:872: a killed child''s `process-exit` carries the pump''s exit code as `child_exit_status`.'
    basis: 'obs-plan.md:872'
    dependent-of: D-obs-instrumentation
note: 'D-obs-stack: no drift (Dependencies none). D-obs-pii: no drift (Run E logs no id or path; the new fixture is scrubbed by the existing `--record` scrub). The `local-cmd` sites obs-plan.md:312, :313, :631 stay true: they word an unconfirmable send generically, which covers `/compact` (`send-issued` then `send-confirmed{confirmed:false}`, no hook awaited); the `spawn-pairs` site :836 and the `subject` enum at :820 state no count that moved. Proposals 4 and 7 both edit the :872 sub-bullet, and 1, 5 and 6 all edit the :739 bullet: apply them together.'
