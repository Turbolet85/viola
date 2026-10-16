# Live `claude` starts — 2026-10-16-unwrapped-sessions-on-the-board

The founder's cap for this chunk: 8 (his dialog 2026-10-16T19:54:09Z, `inputs#I1` fork 3): the record round 6, the
attended P4 / P12 leg 2. Each start is ledgered here with `date -u` before it is made. A red round is reported, never
re-fired.

| ledgered (date -u) | what | starts | running total |
|---|---|---|---|
| 2026-10-16T20:46:00Z | the record round, gate entry 8 through `gate.py run --live-legs`: `viola verify --record` against the 2.1.287 install by path (print probe, Runs A–E) | 6 | 6 of 8 |
| 2026-10-16T21:10:50Z | the P4 / P12 leg, start 7: `viola run p4a -- claude` (rig ledger `evidence/live-sessions.ndjson`) | 1 | 7 of 8 |
| 2026-10-16T21:11:49Z | the P4 / P12 leg, start 8: `viola run p4b -- claude --resume <p4a's id>` | 1 | 8 of 8 |

The revive typed at step (d) was refused `session-live` and started nothing (`evidence/live-p4.md`). No start beyond
8 was made.

Before it: `hyprctl locked` read `false` (the round types no desktop key and opens no window); `claude --version` by
path read `2.1.287 (Claude Code)`; `target/e2e-home` is a link onto `tmpfs`.
