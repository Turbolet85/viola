# P5 card 1 — revision 1 of 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin (plan rev 11)

The changed lines only (`inputs#I6` item 8). The plan approved on card 2 of the take-up
(`.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-2.md`, rev 9) stands in every other line.

**Door.** `/andromeda-implement` ran on rev 9 and stopped `stuck` before any step
(`.andromeda/runs/2026-11-19T06-14-06Z-implement`: the plan names 2.1.325, the chunk moved to 2.1.328). Nothing was
edited in the tree and no `evidence/` exists, so implement re-enters at **step 1**. The revision written in
`.andromeda/runs/2026-11-19T06-05-00Z-phase` (scope rev 4, plan rev 10) was re-checked here against `inputs#I6`; this
run (`inputs#I7`) redid nothing that stood.

## What changed, rev 9 → rev 11 (`inputs#I6`)
- **The version.** `L1` (the 24-row `viola verify` of `~/.viola` under the founder's real home, through `VH`) and `L2`
  (`/context` under the rig's private config, through `CX`) both run claude **2.1.328** by path
  (`~/.local/share/mise/installs/claude/2.1.328/claude`). Entry 10 reads that install; entries 15 and 18 carry
  `--cli 2.1.328` and assert `cli: 2.1.328`.
- **The cap is unchanged:** `L1` + `L2` + the spare `L1-2`, the spare only on a cause `L1` itself showed, after a plan
  revision.
- **The bundle readings, re-read on 2.1.328 with no start.** All four **hold**, so nothing went to `founder-forks.md`:
  - `claudeInChromeDefaultEnabled` (25 → 24 occurrences; the one fewer is an auto-connect check);
  - the confirm label `Yes, use my browser`;
  - `projects[<key>].hasTrustDialogAccepted`;
  - `disabledMcpServers`.

  The rig's seeded markers (`CX`) hold as well. `AddedFolderTrustPrompt` is new on 2.1.328, but only `--add-dir`
  reaches it, and viola never passes that flag. It is named and not used.
- **What a RED `L1` does now** (step 21): it replaces the green 21-row **2.1.328** entry written 2026-11-19T06:03Z with
  its failing rows. The running builder is not affected, but the product's next start reads 2.1.328 unverified until
  `L1-2` passes. The 2.1.325 entry is untouched either way.
- **The re-pin's stamps table** (step 24): after a green `L1`, the re-pinned binary reads 2.1.328 as verified (24
  rows). It reads 2.1.325 (21 rows, not touched by this chunk) and every older entry as unverified.
- **Nothing is claimed for 2.1.328 beyond what `L1` and `L2` read.** `trust-record` and `browser-tools-prompt` get
  their first 2.1.328 reading in `L1`. The recorded fixture set and `STAMPED` stay 2.1.325's.
- **Sizing.** `df -h /home` reads 287G free. `target/` is 13G, rebuilt by the revision-1 firings (clippy 13.21 s, unit
  layer 43.8 s, a release build at 06:09:52Z). The duration of that release build was never kept on disk, so the plan
  no longer cites it. No `timeout` changes: each is at least 1200 s, and the release entries are at 1800 s.

## Top acceptance criteria (as changed)
- (C3, C4) `L1` reads GREEN on `cli: 2.1.328`: 24/24, `failing: none`, `browser-tools-prompt: absent`,
  `stamp-others: kept` (the 2.1.325 entry among the kept), every teardown word, and `user-config: names-only`.
- (C1) P0 prints `present` before `L1` is made. The key lives in the user config, so the same gate covers 2.1.328.
- (C6) `disabled-mcp` and `context-post` are each recorded as read on 2.1.328, never as verification.

## Touchpoints · test commands
Unchanged: 9 files to modify, 0 new, and 28 `[[gate]]` entries. Entries 3, 5, 6 and 7 are `new`. Their `run` is
unchanged, so they keep their take-up baselines (`gate.py baseline`: seen green 5 · new 4 · leg 19).

Research: lists synced to the revision's delta (+0 −0), rev 3; not re-derived since the take-up's rev 2.

## Checks
- `gate.py scope --sync`: **PASS**, delta +0 −0 (plan rev 10 → 11, research rev 3).
- `gate.py scope --lists-only`: parsed, 9 items.
- `gate.py run --dry-run`: 28 entries parse.
- `planlint.py check`: 0 hits. The 12 (d) listing (11 claims) was read: each holds after the chunk.
- `inputs.py verify`: 7 entries, 0 unparsed, 0 drifted.
- Size: about 540 prose lines outside the fence, past the ~400 mark (a warning, as at rev 9).
- Validation-1: aligned with scope rev 5, which adds one verified bullet for the stopped implement run.
- Matrix: no capability is claimed, and the `coverage` view holds no row for this chunk.

## Documents
The plan's render is `.andromeda/cache/docs/2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan.md`; open it
in an editor. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin`
shows a diff of the chunk's records.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-2.md`) **The switch** is an operator step with no
  entry, owned by the overseer and the founder after the pass:
  - replace `~/.local/viola-product/bin/viola` with the placed `viola.<sha8>`;
  - restart this builder by `/exit` through the product, then the operator's launcher.
- (standing — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-2.md`) **C5's two unmeasured items** are left
  unmeasured. The wrap's route-resolve places or drops them with the reason.
- (new) **A 2.1.328 recorded fixture set, and `STAMPED` naming it**, is proposed as a carry for the wrap's
  route-resolve (`inputs#I6` item 5). Its proposed title is «The 2.1.328 recorded fixture set», placed after this
  entry. This chunk records no 2.1.328 set.

Apply? (yes / review / cancel)
