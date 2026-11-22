# Light-gate red — the 2026-11-22T13-02-53Z wrap of 2026-11-22-paste-write-bounded

Recorded before any re-run (none was made). `gate.py run --chunk 2026-11-22-paste-write-bounded --run-dir
.andromeda/runs/2026-11-22T13-02-53Z-wrap`: `entries 25 · green 16 · red 1 (17) · recorded 1 · timeout 0 · not-run 7`
(the seven not-run entries are the operator pass's `leg operator`, recorded green in `evidence/operator-pass.md`).

- **Entry 17**, `bash scripts/agent-run.sh run --integration`: `red · exit 0 ✗ (exit 1) · exit 1 · 86.32s` — the run
  document: `"suite":"nextest-integration","passed":616,"failed":1` · failure
  `viola-e2e::harness_lifecycle boot_that_never_gets_ready_times_out_and_stops_its_supervisor`.
- **First diagnostic line:** `TIMEOUT [  20.005s] (614/617) viola-e2e::harness_lifecycle
  boot_that_never_gets_ready_times_out_and_stops_its_supervisor` — stdout `(test timed out)`; no assertion failed, no
  panic line. The nextest summary: `617 tests run: 616 passed (67 slow), 1 timed out, 0 skipped`.
- **Host (entry 15, `hostwatch.py read --last 15`, 13:39:49Z..13:54:49Z):** `verdict CONTENDED - 802 s stalled on IO:
  13:39:49Z..13:54:46Z 802 s (rule: >= 8 s with io some >= 50%); build tools alive in them: escher 802 s, viola 80 s` —
  io some mean 77 %, dirty at or above the pacing line for 787 s, load mean 271.2; escher's mutation tier
  (cargo-mutants, 38 rustc) alive the whole window.
- **The same source tree read green before:** implement's entry 17 (617 passed, QUIET host), pre-push (coverage suite
  4007 passed) and CI ci#45024255877 on `eb656971` (7/7); this wrap changed no source file.
- **Not settled — why the wrap halts:** the chunk changed the wrapper this test boots (the pump's writer thread; its
  teardown waits at most `KILL_WAIT` for the writer); no basis reads the same red WITHOUT this chunk's edits (P7.1's
  not-this-chunk's arm), and the directive forbids a re-run for green under a stall. Cause unread; hypothesis: the
  20 s test bound missed under the host's IO stall, as the agents-read WATCH did.
- **Next, for the resumed wrap:** a re-fire of the whole light gate (never `--only`) once `hostwatch.py read --last 15`
  reads QUIET — a measurement on a quiet host, this record standing whatever it reads; green → P7.1's «green over a red
  the read no longer holds» arm, the red routed by P5's Recurrence-watch row before the flip; red again → its basis
  (the parent `66eb718c` in a clean worktree) and its owner, or the chunk's.

## The resumed wrap's re-fire (added after it; nothing above edited)

- **Host before:** the neighbour ended its tier at 2026-11-22T15:36:50Z (the operator's relay at the resume);
  `hostwatch.py read --from 2026-11-22T15:37:00Z --to 2026-11-22T15:37:26Z`: `QUIET - 0 s stalled on IO`.
- **The light gate re-fired whole** (no `--only`): `entries 25 · green 17 · red 0 · recorded 1 · timeout 0 · not-run 7`;
  entry 17 `green · exit 0 · 68.47s`. Host over the gate's run, `--from 2026-11-22T15:37:00Z --to 2026-11-22T15:39:36Z`:
  `QUIET - 0 s stalled on IO`, viola's build alone alive. (Entry 15's own `--last 15` read CONTENDED, 375 s stalled,
  every stalled second before 15:32:16Z with escher alive — before this gate began.)
- **Disposition:** P7.1's «green over a red the read no longer holds»; no cause found → P5's Recurrence-watch row:
  `WATCH:` `cry-kyoy7e6j` on «Lineage registry» (tx 3972), `0/3 since 2026-11-22-paste-write-bounded`.
