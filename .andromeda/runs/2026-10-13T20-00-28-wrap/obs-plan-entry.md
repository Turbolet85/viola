
## 2026-10-13-interrupted-verify-cleanup — `detail:"interrupted"` on verify's process-exit
**Section:** §4 Span / Trace Coverage → Edge flows (the `verify` bullet and its `--capture` sub-bullet) · §6 Log Coverage → `detail` code catalog · §6 Log Coverage → Child / shell spawns
**Change:**
- `process-exit` exit 1 gains `interrupted`: a `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked ends `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` in `cli-<name>.ndjson` under a `VIOLA_NAME`, after every spawn pair it opened is closed. Was 11 schema members; now `schemas/diag-line.v1.json` closes the `detail` at 12 (the rule: the enum's members).
- A stopped run starts no further spawn. A child killed at the stop closes its pair with `duration_ms` and no `child_exit_status`, so the six pairs are the uninterrupted run's.
- `verify`'s outcome reads 1 on a failing row, a refusal or a stop, with the fixed stderr pair, empty stdout and no stamp.
- Was: the probe dir was "removed on every exit path". Now it is removed on a pass, a fail and a stop; SIGKILL, a crash and a Windows close, logoff or shutdown leave it.
- `run`'s version gate observes no stop.
**Why:** The operator's P4 answer gave the interrupted run a twelfth closed `detail` member. The handler itself logs nothing (its one act is an atomic store, async-signal-safe), so the `process-exit` line is its trace.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/
