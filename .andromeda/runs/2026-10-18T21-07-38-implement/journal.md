# Journal — /andromeda-implement of 2026-10-18-compact-post-condition-ledger-row

Long runs, each with its UTC time from `date -u` in the call before it.

- 2026-10-18T21:23:19Z — gate firing 1: `gate.py run … --only 1,2,3,4,5,6,14,15,16` (lint, unit, the new unit cases, the literal probe, the preservation guard, `cargo deny`), backgrounded.
  Read 21:24Z: `entries 26 · green 9 · red 0` (entry 6: nextest-unit 16 passed 0 failed). The unit gates are green.
- 2026-10-18T21:24:14Z — live round 1 of at most 2 (the founder's cap: 12 live `claude` 2.1.287 starts; this round
  spends 6: the print probe and Runs A–E). `gate.py run … --live-legs`, backgrounded. Starts spent before it: 0.
  Read 21:25:47Z: `round: COMPLETE · legs fired 1/1 · red 0`; `stamped 2.1.287  24 pass  0 fail`. Starts spent: 6 of
  12. No second round (no red). Record: `chunks/…/evidence/live-round.md`, `round-212414Z.txt`.
- 2026-10-18T21:26:57Z — gate firing 3: the whole block, plain `gate.py run` (non-leg entries, `pre-push` included),
  backgrounded. Read: `green 17 · red 1 (23)`; `pre-push` red on two cases: `cli_answer
  path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` (`tests/support/home.rs` `dialogless_home`
  pinned `19 pass  4 fail`, the set now reads `20 pass  4 fail`: a companion literal the plan missed) and
  `tui_channel_fds_the_child_holds_nothing_of_violas` (`fd target: NotFound`: an fd listed under `/proc/<child>/fd`
  closed before its link was read). The fd red passed 15/15 standalone; a harness fd sampler showed fds that vanish
  between listing and `readlink` while the fake agent works. Both fixed, recorded in `scope-record.md`.
- 2026-10-18T21:39:56Z — gate firing 4: the whole block again, backgrounded.
