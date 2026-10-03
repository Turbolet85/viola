# P5 — can a `claude` child outlive its wrapper? (step 15)

/implement run `2026-10-02T19-38-47-implement`, 2026-10-03. Every stopped pid was spawned by this step, and each was
stopped by its exact pid (`Stop-Process -Id`). The operator's own `viola.exe` sessions were never touched. Recorded here:
pids, liveness readings (pid + OS start time), seconds and row counts. No `CLAUDE*` value, no agents row and no path.
Probe scripts live in the session scratchpad.

## Leg 1 — fake agent, through the harness
`agent-run boot --instance builder` (C: dev copy), then `pid` / `child_pid` read from the snapshot, then `Stop-Process`
on the wrapper pid only. The child is then polled every 100 ms for 10 s, by pid + start time. Then `agent-run cleanup`.

| reading | wrapper pid | child pid | wrapper gone at | child gone at | child alive at 10 s | cleanup |
|---|---|---|---|---|---|---|
| 1 | 20120 | 61144 | 0.01 s | 0.01 s | no | `ok`, `processes_gone`, `endpoint_gone`, `home_removed` |
| 2 | 48868 | 23400 | 0.01 s | 0.01 s | no | the same |

## Leg 2 — the real CLI (`claude` 2.1.283)
`target/harness/debug/viola.exe --home target/agent-run/p5/<home> run p5 -- claude`, started with `Start-Process` in its
own console window (a real terminal, no prompt sent). Then `Stop-Process` on the wrapper's exact pid, then the
`claude.exe` child's liveness for 15 s. `claude agents --json --all` rows were counted before and after; a row counts when
it contains the session's id, matched as a string and never printed.

| reading | cwd | session-start seen | rows naming the session (before → after) | wrapper gone at | child gone at | child alive at 15 s |
|---|---|---|---|---|---|---|
| a | a fresh empty dir | no (60 s; the CLI likely held at the new folder's trust dialog) | not measurable (no session id) | 0.14 s | 0.50 s | no |
| b | the repository (a trusted folder) | yes | 1 → 0 (total rows 1 → 1) | 0.14 s | 1.27 s | no |

After each leg-2 reading, no process whose executable or command line names that home survived: no `viola mcp`, no hook,
no `node`. The process list was read by executable path and command line.

## Reading
On this host, terminating the wrapper takes its child with it. The fake agent goes at once, and the real CLI within 1.3 s,
without leaving an agents row for its session. The study's hypothesis ("with no job object a grandchild could survive")
is not observed for the direct child or for anything holding the home. No job object exists in the product, and none is
needed for this result.

**Pinned:** `tests/run_cli.rs` `run_child_ends_when_its_wrapper_is_terminated` (Windows). It reads the booted wrapper's
`pid` / `child_pid` from the snapshot, terminates the wrapper with `TerminateProcess`, and waits on the child by pid + start
time, never on master EOF, up to `WITHIN`.
- green on the C: copy;
- red with the watched pid swapped for the test's own process (which never ends): `the child outlived its wrapper` at
  7.97 s. Restored afterwards.

**Owed:** the tab-close leg is founder-attended (the study's §5) and is not run here. It stays owed unless the founder runs
it.
