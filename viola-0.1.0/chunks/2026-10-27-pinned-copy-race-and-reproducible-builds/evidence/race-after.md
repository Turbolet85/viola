# The product race, read clean after the fix (plan step 8)

Read 2026-10-27, started 02:14:32Z, on the Linux dev host (same rig as `race-before.md`), run dir
`.andromeda/runs/2026-10-27T02-08-46-implement/`. The tree: HEAD `99977116182b` plus plan steps 2 and 4–7
(`place_private_new` in `crates/viola-state/src/fs.rs`, `pin_bytes`'s absent-copy arm through `place_absent` in
`crates/viola-state/src/pin.rs`). The harness rebuild before it: exit 0 in 3.22 s; `target/harness/debug/viola` was
written at 03:14:47 local, after `pin.rs`'s last edit (03:14:16 local), and `strings -a` reads 17 lines naming
`place_private_new` in it.

Command: `python3 viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py after`.

| input | sessions | failed boots before (step 3) | failed boots after | SessionStart `hook` receipts after |
|---|---|---|---|---|
| `overseer` + `builder` (default pair), unstamped | 24 | 4 (`readiness-timeout`; `spawn_os_error` 26, `ETXTBSY`, × 4) | **0** | all 48 `ran:true, exit_code:0, spawn_os_error:null` |
| `builder` alone (control), unstamped | 24 | 0 | **0** | all 24 `ran:true, exit_code:0, spawn_os_error:null` |

Round wall times after: pair 2.48 / 2.50 / 2.47 / 2.47 s, control 2.37 / 2.37 / 2.25 / 2.35 s; every session's
cleanup `ok:true`, `processes_gone:true`, `endpoint_gone:true`.

The verdict is the pair: the two-instance input fails 4 of 24 at the base, each failure the agent's exec of the
pinned copy refused `ETXTBSY`, and 0 of 24 with the copy placed closed and no-clobber; the single-instance control
reads 0 of 24 on both sides, so the rig itself does not fail boots. A sampled 0 of 24 is no proof the window is
gone; the forced-window case (`red-green.md`) is.

Residue: the census after the run (a whole `ps -eo pid,comm,args` listing, filtered by the fixed string
`projects/viola/target`) found no running process; `target/agent-run/` holds 96 `race-*` session dirs from steps 3
and 8.
