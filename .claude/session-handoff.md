# Session Handoff

**Last Updated:** 2026-10-03T22:18Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** chore(route): operator-requested adaptation — 0-pending wrap (the Linux-host route adaptation)

## Position
- Done: no chunk. This was a 0-pending wrap carrying the overseer's relay `linux-route-adaptation.md` (items A–D;
  C2 is the founder's ruling). The dev host is now Linux (Omarchy, btrfs) and the Viola pause is lifted.
  - **M2 is closed:** the D: volume is gone, the witness set read 63/0 on Linux, and CI is green on `9e3b850` and
    `d60f3d6`. Leak B and the in-repo coverage gate are closed with it. The TUI boundary cases stay as a CARRY on
    :68, because the ConPTY cases are witnessed on CI only.
- Next: **Mutation scoring completion** (working-route :68) → `/andromeda-phase`. It now holds:
  - M3 (the viola-e2e mutants);
  - the 12 `cfg(unix)` mutants, scored natively (the WSL leg and the `TMPDIR` question are retired);
  - C3: the harness diff-prefix pin. 13 unit tests are red on this host until it lands;
  - D: pre-push and the gate tools moved to the Linux host. The native stage keeps `env -i` HOME+PATH, and anything
    wider goes to the founder live.
- After it: the new **Windows boundary mutation workflow** entry (:70, the founder's C2 ruling).

## Work done
- The route tail was re-scoped (:68, :80, :85 rewritten; one entry inserted). The record is in
  `.andromeda/runs/2026-10-03T22-10-25-wrap/adaptation-record.md`.

## Drift resolved
- No spec body was edited. The bodies that name the WSL distro or the pre-push stages reconcile at :68's wrap,
  through its D CARRY.

## Notes
- **Founder calls pinned on the route:**
  - :80: where the live proof runs. A real-CLI run on CI needs a Claude credential on a runner, and a Linux live run
    would come before the Unix hardening.
  - :85: the tab-close leg, now in a Linux terminal.
- **`host-win32.md`** (always loaded) describes the retired Windows host. It is setup-rendered (U04), so its fate
  belongs to an `/andromeda-setup-project` re-run.
- **Code-graph:** no `.andromeda/cache/` on the fresh clone. Missing host tools: `scip-typescript`, and python
  `duckdb`/`protobuf` (`pip install -r scripts/requirements.txt`). The next phase builds the cache cold.
- **Installed `claude` is 2.1.287.** The only recorded fixture set is `fixtures/claude/2.1.283/`, so this CLI build
  is unverified until `viola verify` runs against it.
- **Deferred learnings** (max-3 cap, carried):
  - PS5 BOM-less `.ps1` parsing (0.8; moot off Windows);
  - let a red CI run finish before folding its fix (0.7);
  - the doubled-backslash guard recurrence; the cross-drive `git worktree` move.
- **Last failed command:** none.
