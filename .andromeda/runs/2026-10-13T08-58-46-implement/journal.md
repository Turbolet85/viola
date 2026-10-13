# Journal — implement 2026-10-13-epoch-4-cleanup

Each long gate run is written here before it is waited on.

- 2026-10-13T09:06Z — firing 1: the whole block, `gate.py run --plan viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/plan.md`
  (backgrounded; 22 non-leg entries, `pre-push` bounded at 7200 s; the three `leg = 'operator'` entries are not fired
  by a plain run). Tree: P1 complete — both splits, `wipe` through `create_private_dir`, the Windows witness, the
  `viola-state` edge; workspace clippy and the size read green by hand before the firing.
  Read: 20 green · 1 red (6, orphans: `relabel` at `src/run/send/relabel.rs` — `cargo modules` analyses without
  `cfg(test)`, so a `#[cfg(test)] mod relabel;` declaration is absent and its file reads orphaned) · 3 operator not run.
- 2026-10-13 — firing 2: the whole block again after the fix (`mod relabel;` declared unconditionally; its file holds
  only a `#[cfg(test)] mod tests`) and two test structs narrowed back to private (`NumberFields`, `Lines`: used only
  inside `test_support.rs`). Backgrounded; `pre-push` read 67 s at firing 1.
  Read: 21 green · 0 red · 3 operator not run.
- 2026-10-13 — operator pass, firing 3: `--only 21` (`pre-push`) on the final uncommitted tree before the pre-CI commit.
  Read: green (67 s, `ok:true`, `linux-tests`). Then hygiene (refused 2 phase-run-dir files, rewritten, clean), the
  pre-CI commit `3b75dc7`, entry 23 through `--operator 23` (pushed, `d9ad9f1f→3b75dc72`).
- 2026-10-13 — operator pass: entry 24, `ci.py conclusion --sha HEAD --wait 1800`, fired as written, backgrounded,
  for `3b75dc7` (the Windows witness runs on `test (windows-2025)`).
  Read: `verdict: green` · 15/15 · ci#38375200217 · polled over 809 s; the witness PASS read from job 115221715737.
