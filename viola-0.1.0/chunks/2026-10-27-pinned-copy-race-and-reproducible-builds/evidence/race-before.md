# The product race, read red at the base (plan step 3)

Read 2026-10-27, started 02:11:15Z, ended before 02:12:56Z, on the Linux dev host (homes on the tmpfs behind `target/e2e-home`, 22 GiB free;
host `QUIET`, `preconditions.md`), run dir `.andromeda/runs/2026-10-27T02-08-46-implement/`. The tree: HEAD
`99977116182b` plus plan step 2 only (the fake agent's `spawn_os_error`); `crates/viola-state/src/{fs,pin}.rs` unchanged.

Command: `python3 viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py before`
(rounds of 6 concurrent `target/harness/debug/viola-harness boot --unstamped`, spawned directly, stdout to files; each
fake agent `--receipt <scratch>/<session>-<name>.ndjson`; every session cleaned up; kill deadlines: boot 120 s,
cleanup 60 s). The harness build before it: exit 0 in 0.46 s (up to date).

| input | sessions | failed boots | failed boots' reasons | each failed instance's SessionStart `hook` receipt |
|---|---|---|---|---|
| `overseer` + `builder` (default pair), unstamped | 4 rounds × 6 = 24 | **4** | `readiness-timeout` × 4 (`missing: ["<name>:events"]`) | `ran:false, exit_code:null, spawn_os_error:26` × 4 — `builder` once (r2-s1), `overseer` three times (r2-s4, r4-s2, r4-s5); the other instance of each `ran:true, exit_code:0` |
| `builder` alone (control), unstamped | 4 rounds × 6 = 24 | **0** | — | every SessionStart `ran:true, exit_code:0, spawn_os_error:null` |

Raw OS error 26 on Linux is `ETXTBSY` ("Text file busy"): the agent's exec of `bin/<key>/viola` met an inode another
start still held open for write — the CARRY's hypothesis, now read on the product run itself. Round wall times: pair
2.40 s / 22.26 s / 2.48 s / 21.78 s (the two rounds with failures ran to boot's 20 s instance deadline), control
2.35–2.37 s each.

Stop rule S1 does not apply: the two-instance input read 4 failed of 24 at the base.

Residue: the census after the run found no running process under this repository's `target/` (a whole `ps -eo
pid,etimes,comm,args` listing, filtered by the fixed string `projects/viola/target`). The 4 failed boots each left
their home under `target/e2e-home/viola-session-*` (boot keeps a home it never recorded, so no `cleanup` removes it),
and the 48 sessions left `target/agent-run/race-*` dirs; the successful sessions' homes were removed by cleanup.
