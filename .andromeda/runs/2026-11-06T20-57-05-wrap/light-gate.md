# Light gate — 2026-11-06-version-done-check (wrap 2026-11-06T20-57-05, P7.1)

`gate.py run --chunk 2026-11-06-version-done-check --run-dir .andromeda/runs/2026-11-06T20-57-05-wrap`, backgrounded, no
`--skip` and no `--void` (no entry carries `defer`); trail `gate-2026-11-06-version-done-check.json` in this run dir.

Summary line: `entries 50 · green 38 · red 1 (39) · recorded 2 · timeout 0 · not-run 9`

## The ASSERT, per entry
- 2–38, 40 — green by their `expect` (40, `pre-push`, 97.92 s).
- 1 (`df -h /tmp`) and 50 (`gh run list --workflow nightly.yml …`) — recorded, never counted green.
- 41–49 — `not run — leg operator`: re-verified by `{work_dir}/evidence/operator-pass.md` on the final HEAD
  `849aad181fedc5a56cc9883fa7717c99d0dfb012`: 41 hygiene `clean` (twice), 42 the push `aec2ec95→849aad18`, 43–49 the
  seven `ci.py conclusion` reads `verdict: green` on ci#42288148289 (supply-chain, test, lint, msrv, perf, release,
  fuzz-replay).
- 39 — `red · last line '0' ✗ (last: '2')`, the live-round reader `jq … kind == "reading" and .verdict != "green" |
  length` over the round's ledger. Outcome line:
  `red — not this chunk's: the account's status-line payload and an unverified CLI, not the tree — the overseer's
  technical answer, 2026-11-06 21:33Z, resting on the founder's own words (v1-27 deferred, 20:53Z; a red hold leaves
  the ceiling PROVISIONAL with 0.2.0's first entry, 19:34Z) → residual res-t3ngmhfi (target 0.2.0)`.
  Its two reds:
  1. the rig home's `viola verify` read `statusline-rate-limits` red, `stamped 2.1.287  23 pass  1 fail`, on an
     unchanged CLI: the account's status-line payload carries `five_hour` `unknown` and only `seven_day`;
  2. the near-ceiling dialog hold went unread: session 2 ran on an unverified CLI, which replies `null` to a dialog at
     once.
  Its basis is the directive, not a run over the tree without this chunk's edits: the operator's wrap directive item 10
  (`chunks/2026-11-06-version-done-check/inputs/I7-wrap-done-check-directive.md.txt`) rules it no tree red that blocks no
  flip. No claimed cap's `ref` runs under entry 39: v1-27, the one cap it bears on, is deferred at P7.3.
