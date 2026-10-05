
## 2026-10-05-real-cli-verify-probes — verify-pty-probe: four verify spawn pairs
**Section:** §4 Edge flows → `verify` (spawns, CI) · §6 event table → `process-start` · §6 Child / shell spawns (subject set, schema)
**Change:**
- `verify` logs four child spawn pairs between its own start and exit (was two): `version-probe`, `verify-probe` (the print probe), then one `verify-pty-probe` pair per interactive PTY run (Run A, Run B). Each `process-exit` carries `child_exit_status` and `duration_ms`, and no line holds row text, a prompt or a path.
- `subject` (the event table, the child-spawn set, `schemas/diag-line.v1.json` `$defs.subject.enum`) gains `verify-pty-probe`.
- In CI, verify drives the fake agent's print mode plus its two interactive runs (`--screens --turn-stop --trusted-root`).
**Why:** the chunk added verify's two interactive PTY runs, instrumented at the call site like the other two spawns.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/
