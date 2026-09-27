
## 2026-09-27-wrapper-channel — pre-push VM release and Windows tests, endpoint readiness, channel corr rules, E2 fd premise fix
**Section:** §1 (boot readiness; viola-channel coverage scope) · §2 (Property-based row; test directory conventions) · §3 (`boot`, `run --fuzz-replay`, `cleanup`, `pre-push` stages and document, Closed enums, Log format) · §5 (Module ↔ IPC; Wrapper channel oversize) · §6 E2 · §12 (new `2026-09-27` entry)
**Change:**
- `pre-push` stages add `vm-release` (`wsl.exe --terminate Ubuntu` once the ubuntu verdict is back) and `windows-tests` (`run --coverage` + `gate --require coverage,doctest` on the host; a red stops before `windows-leg`); host stages at `CARGO_BUILD_JOBS=16`; the document gains `vm{terminated,free_kib_before,free_kib_after}` and `windows{run,gate}`.
- `boot` requires snapshot `endpoint` (missing `<name>:endpoint`); `cleanup` reports `endpoint_gone` (Windows: client connect NotFound; Unix: socket path absent) — was null.
- Log format: `channel-*` corr null for an id-less `hook.event` and on a -32700/-32600 response; a dialog `hook-invoked` and a `hook-decision` with `detail` may be null; `diag-line.v1.json` requires `corr` exactly on the listed lines.
- E2: was "fds only 0/1/2 plus the PTY slave"; now the live `/proc/<pid>/fd` table shows nothing of viola's (no socket, no path under the home, 0–2 the PTY; fixture files exempt by exact path) — measured `[0,1,2,3,4]`, and portable-pty `close_random_fds()` closes fds above 2.
- §5: DACL set from the GA SDDL and read back canonical FA, SID possibly an alias; Unix 0600 by chmod after the bind; an oversize close reads 0 (Windows), may reset (Linux) or EPIPE the tail write (macOS).
- Fuzz: `channel_frame` (7 seeds) beside `viola_name`; `tests/support/ndjson.rs` the one complete-lines reader.
**Why:** the wrapper channel chunk's measured facts. The E2 change is a premise fix, not a widening — the overseer's live answer at this wrap.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/
