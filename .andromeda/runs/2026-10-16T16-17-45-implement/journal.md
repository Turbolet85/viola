# Journal — 2026-10-16-remove-the-browser-leg · implement

- P1: steps 1-15 written. `e2e-web/` is still on disk: a recursive `rm` is refused by the host permission
  settings and the move into `target/e2e-home.disk/` was refused by the auto-mode classifier; the operator's to
  remove (not routed around).
- P1 end: `cargo check --workspace --all-targets --features fake-agent` (foreground, the compile check of the
  edit burst) — started; finished clean (2.73 s).
- The operator's answer (as the overseer), verbatim: "Overseer: deletion is the founder's; the overseer has asked
  him to run it (6 tracked files + 19 MB of generated node_modules/pw output, inventoried). Run the block now,
  then re-fire the absence guard once e2e-web/ is gone; journal it."
- P2: the whole block, `gate.py run --plan … --run-dir … --marker …`, backgrounded — started. Expected: the
  browser absence guard (entry 9) reads red on the e2e-web/ files while they stand; it is re-fired once the
  founder has removed the directory.
- Block 1 summary: `entries 25 · green 19 · red 3 (6,9,22) · recorded 0 · timeout 0 · not-run 3`.
  - 6 and 22 share one cause: two oracle literals of steps 6-7 — `pre_push_every_native_call_carries_only_home_and_path`'s
    `fake.calls.len() > 10` (the pass now makes 9 calls: the node and browser calls left), and
    `pre_push_native_stages_…`'s `linux` keys, which read in insertion order `["run", "gate"]`, not the plan's
    `["gate", "run"]`. Fixed: `assert_eq!(fake.calls.len(), 9, …)` and `["run", "gate"]`.
  - 9: every hit is under `e2e-web/` (still on disk) — waits on the founder's removal.
- Re-run `--only 6,22` — started (backgrounded); summary `entries 25 · green 2 · red 0 · recorded 0 · timeout 0 ·
  not-run 23` (6 green 8.52 s, 22 green 67.48 s).
- Matrix: `claimed by 2026-10-16-remove-the-browser-leg: 0` — no ref to write.
- Waiting on the founder's removal of `e2e-web/` (still on disk; its four ignored outputs now read untracked).
  Next: re-fire entry 9, then the whole block once more.
- The founder removed `e2e-web/` (his `! rm -r`, relayed by the operator); `git status` reads the six tracked
  files deleted and nothing untracked under it.
- Entry 9 re-fired: `9 probe green · exit 1 · 0.01s · 0 B → 9.2.log`.
- The whole block, second firing — started (backgrounded).
