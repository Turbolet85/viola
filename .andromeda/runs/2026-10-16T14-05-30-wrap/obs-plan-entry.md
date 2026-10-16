
## 2026-10-14-resume-and-compact-ledger-rows — seven verify spawn pairs; a killed PTY child's exit status
**Section:** §4 Span / Trace Coverage → Capability ledger → `verify` / `plugin install` (the first and CI bullets); §6 Log Coverage → Boundary-call wrappers → Child / shell spawns (and its `diag-line.v1.json` subject sub-bullet)
**Change:**
- verify logs seven child spawns (was six): `version-probe`, `verify-probe` and five `verify-pty-probe`.
  - The five are Runs A–E. Run E is the `--resume <id>` start of Run B's first session id.
  - No line carries row text, a prompt, a session id or a path.
  - The `subject` enum is unchanged.
- MM is 22 on Unix and 19 on Windows, with `resume-session` relied on every OS.
- CI's fake-agent verify runs five interactive runs.
- A child killed at a Ctrl-C or TERM stop:
  - Was: it closes with `duration_ms` and no `child_exit_status`.
  - Now: a child of an interactive PTY run killed at the stop carries `child_exit_status` (the PTY pump's exit code; portable-pty reports a code for a signalled child) and `duration_ms`, as measured at this chunk by its resume-run stop case.
  - A print-mode or `--version` probe child killed at the stop is not measured.
**Why:**
- The spawn count follows Run E, the founder's own answer to P4 fork 2 (`inputs#I1`).
- The exit-status wording was measured false. The behaviour predates this chunk's code. It is amended to the measured scope on the overseer's founder-delegated word (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
