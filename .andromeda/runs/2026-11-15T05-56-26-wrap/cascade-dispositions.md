# Cascade dispositions — 2026-11-14-status-line-interception-removed

## The search (step 2)
- One `cascade.py sweep` (`cascade v1.34`, baseline `7416035f`, the parent of the oldest pre-CI commit `fcdbeaa2`),
  its pattern files: `cascade-patterns-architecture-1.toml` · `-security-plan-1` · `-security-plan-2` ·
  `-test-plan-1` · `-test-plan-2` · `-obs-plan-1` (each the dry run its landing followed; none superseded) and main's
  own `cascade-patterns.toml` (`m-statusline` `(?i)status[ -]?line` · `m-settings-ovr` · `m-shell-run` ·
  `m-two-capture`: the retired claim by its verbs and phrasings, over all seven masters). The a11y-plan landing and
  main's prose-only files wrote no pattern file (`patterns: none: … changed prose alone`); their rows are read by
  step 3's deltas. No pattern file holds a term of digits alone.
- Counts, copied: `total (70 patterns) · 893 rows over 29 files` · `per class · new 43/2 · standing 259/6 · leaf
  584/21 · curation 4/1 · base 3/2`. Parts `sweep-pass-1.1.txt` · `.2` · `.3`, read whole in one message.
- Patterns at 0 rows, their control fired (a statement about the pattern): `arch1-2`, `obs1-24`, `obs1-39`, `obs1-40`,
  `obs1-43`.
- The sweep ran before ops-main-security-plan.2.md, the one fix it found; that file changed prose alone and wrote no
  pattern file.

## Master rows (new 43 · standing 259)
- **amended (by this pass, after the sweep):** `security-plan:468 dec-rphe2o4q` (`m-statusline`, the rule still named
  «the statusline command») and `security-plan:568 dec-qprwcjeq` (`m-statusline`, the name «Hook, MCP and statusline
  commands») — set by `ops-main-security-plan.2.md`, tx 1297.
- **re-derived, this pass's own text (`new`, 43 rows over 2 files):** `architecture:308 abs-lpwwhjjj` (the new
  absence) · `architecture:331 dec-zejotuie` (the widened except) · `test-plan:603 ctr-wdo4kiwe` (the new
  `viola-run-start/settings`) — each hit names the removed thing as removed; no change.
- **edited rows, read again for a duplicate of the retired mechanism:** `architecture:381 ctr-zgw4cnv3`
  (`hook-to-kind-map/hook-statusline`: its key cells name the retired word, its promise says it is retired — true) ·
  `architecture:469 dec-uevasxof` (`no-shell-out`, except emptied — true) · `obs-plan:275 ctr-7x4zh2re`
  (`replayed-snapshot/live-fields`: the promise set; its `item` key cell still lists `statusline_command` — a key cell
  is never set, and fit read a retire-and-add heir as standing under it; true of nothing for that one item, left, and
  named on the report) · `security-plan:147`, `:610`, `:73`, `:219`, `architecture:598`, `:1018`, `:106`, `:899`,
  `security-plan:227`, `obs-plan:626`, `:629` — true as set.
- **no change, a true claim sharing the token (`standing`, by class):** `arch1-34` (`ViolaName`, 32 standing) ·
  `arch1-38` (`VIOLA_DIR`, 25) · `obs1-15` (`VIOLA_NAME`, 63) · `obs1-4` / `obs1-16` / `obs1-23` (`duration_ms`) ·
  `obs1-17` (`deadline_hit`) · `obs1-20` / `obs1-21` (`hook_event`) · `obs1-22` (`count:1` oversize) · `security1-15`
  (`snapshot.json`, 15) · `security1-20` (`pinned_bin`, 9) — terms the old cells shared with claims that stand.
- **no change, true as written:** `architecture:97 dec-zn4iou3x` (`global-statusline-untouched`: viola adds no settings
  layer at all) · `obs-plan:4 dec-4qrikobi` · obs-plan `contract/child-spawn-log/*` — their `boundary` key cell «child
  and shell spawn log events» holds «shell», a key cell never set; the promise was set to child spawns; named on the
  report.

## Curation (4 rows over 1 file) and bases (3 over 2)
- `.claude/rules/testing.md:78` (`m-statusline`, nextest pads a status line's duration — another meaning) ·
  `:81`, `:97` (`VIOLA_NAME`/`VIOLA_DIR`, true) — no change, nothing routed to P3.
- `rows:rule:47`, `rows:rule:55` (a rule's history naming chunk 2026-10-10-statusline-pass-through) · `rows:detector:16`
  (`ViolaName`) — no change; nothing proposed to the playbook.

## Leaves (584 rows over 21 files)
- Every `leaf` row joins step 3's set through `cascade.py leaves` (it reads this sweep's listing itself); the stale
  leaf text the rows show (`.claude/rules/security.md`, `verification-harness.md`, `api.md`, `events.md`,
  `observability.md`, `testing.md`; the docs summaries and service notes) is re-derived there.

## Leaves (step 3)
- The set: `cascade.py leaves` (pass mode; `epoch · «Epoch 2 — The budget gate and the status line leave» ·
  markerless 1 · gated 0 · open`; `sweep leaves · 21 file(s) from sweep-pass-1.1.txt, sweep-pass-1.2.txt,
  sweep-pass-1.3.txt · 0 outside every group`); `total · pass · 13 group(s) · 26 leaves · 403670 B · unmapped 2`.
- Unmapped, no change: `.claude/rules/host-linux.md` (the host leaf: no source states it) and
  `.claude/rules/testing-src.md` (src-scoped pointers; 0 hits of the retired subject).
- One comparer a group, batch `leaf` (13 agents, 13 pass). Per group, every group pass mode, findings = applied by
  `cascade.py leaves --apply` (`total · blocks 72 · applied 72`), declined 0: a11y-plan 1 · architecture-1 11 ·
  architecture-2 6 · architecture-3 7 · architecture-4 2 · architecture+security-plan 3 ·
  architecture+security-plan+test-plan+obs-plan+a11y-plan 0 (CLAUDE.md, no finding) · architecture+test-plan+obs-plan 4 ·
  obs-plan 2 · security-plan 15 · security-plan+obs-plan 0 (`.claude/rules/ci.md`) · test-plan 16 · test-plan+obs-plan 5.
- By hand (a residue grep over every leaf after the apply found three the comparers missed), drafted whole and landed by
  `rec.py harness land`: `.claude/docs/services/viola-agent-claude.md` tx 1314 (chrono «for the pure `statusline`
  module» dropped — no source in the crate names chrono; the hygiene walk no longer validates `StatusLine.*.json`
  against the deleted schema; a missing space) · `.claude/docs/security-summary.md` tx 1315 (the ledger's row list and
  counts: twenty-four with three statusline rows → twenty-one, `/21`; it had also missed the prior chunk's
  `statusline-rate-limits` removal).
- Left, true: `.claude/docs/security-summary.md:72` «the user's own status line in Runs C and D» (claude's own status
  line, untouched); `.claude/rules/testing.md:78` (nextest's status line, Session Additions).
