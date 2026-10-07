# The contended driver's own control (step 8)

Before its first graded use the driver (`evidence/contended-driver.txt`, a byte copy of the script that ran) was
fired once on the quiet host with its trigger forced open (`--force-open`), writing to a scratch file and not to
`evidence/contended-reading.ndjson`. The operator was told in one line before it ran: the control's own 260 MB
on the shared volume and 260 MB on tmpfs, then the wait.

## Before
- The probe's payload: a copy of `target/debug/viola` in the session scratchpad (tmpfs), 52 118 312 B.
- The harness build was fresh: the quiet `binary(cli_verify)` gate entry read 29 passed, 0 failed in the gate run
  of 07:48Z to 07:50Z.
- `start_probe.py` was run by path from the sibling checkout; its bytes hash to the `sha256` inputs#I4 records
  (`f8df8f7b…`), and `window_watch.py`'s to inputs#I5's (`82594e7a…`). The sibling's HEAD has moved since the
  snapshot; the two files have not.
- `/proc/meminfo` just before: `Dirty` 908 kB, `Writeback` 0 kB.

## The run (2026-10-07T07:51:47Z to 07:52:08Z)
The driver exited 0 and printed `contended_driver: 1 window(s); ended forced at 2026-10-07T07:52:08Z`; its stderr
was empty. Its one line, as written to the scratch file:

```
{"kind":"reading","t_open":"2026-10-07T07:51:47Z","dirty_mb":0,"other_builds":{},"control_write_med_s":0.059,"control_write_max_s":0.065,"backing_total_max_s":0.142,"suite_passed":29,"suite_failed":0,"suite_wall_s":21.3,"forced":true}
```

## The two verdicts the plan asks of it
| check | reading | verdict |
|---|---|---|
| the line carries a `control_write_med_s` under 7 | 0.059 | holds |
| the reader's `jq` program over that scratch file exits 1 (a quiet window is not a contended reading) | printed `false`, exit 1 | holds |

The reader's other side (a stalled window with a green suite exits 0, a stalled window with a failed suite exits
1) was read at P5 over hand-minted lines, as the entry's `baseline` records.

## After
- `target/contended-control/` was removed by the driver (the path no longer exists).
- `ls -A target/e2e-home/`: 0 entries, so the backing probe left nothing behind.
- The watcher for the graded reading started at 2026-10-07T07:52:20Z: 60 minutes at most, three windows at most.

## The graded run (2026-10-07T07:52:20Z to 08:09:13Z)
The driver exited 0 and printed `contended_driver: 2 window(s); ended contended at 2026-10-07T08:09:13Z`. It met
two natural windows, both inside one other project's build (`other_builds` names it, with 129 and 123 build
processes alive), and wrote one line for each to `evidence/contended-reading.ndjson`:

| window opened | `dirty_mb` | control write, median / max (s) | backing total, max (s) | suite | counted |
|---|---|---|---|---|---|
| 08:07:53Z | 173 | 6.471 / 7.206 | 0.103 | 29 passed, 0 failed, 23.4 s | no: the median is under 7 |
| 08:08:52Z | 243 | 10.999 / 11.109 | 0.285 | 29 passed, 0 failed, 21.5 s | yes |

The first line stays in the record as the plan asks; the driver cooled down 30 s and took the next window. The
second is the proof's window: the pinned-copy-sized write stalled about 11 s on the shared volume, where the homes
were, while in the same seconds the same write through the link took under 0.3 s and the verify-driven binary ran
green at its quiet-host wall time (21.2 s in the gate block, 21.5 s here). `target/contended-control/` was removed
at the end, and `target/e2e-home/` held no entry.

The reader entry (`jq -e -s …` over the record) then read green in the gate block: exit 0, `true`.
