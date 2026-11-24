# P5 review card 1 — 2026-11-24-viola-up («`viola up`») · plan rev 5

**Read the plan:** `.andromeda/cache/docs/2026-11-24-viola-up/plan.md` (rendered from `doc:2026-11-24-viola-up/plan` rev
5). No plan stands in a chunk's folder: writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-viola-up`.

**The operator is away** (`inputs#I1`): this card waits for his word, or a delegate's relayed through the overseer.
His forks are in `.andromeda/runs/2026-11-24T04-17-54Z-phase/founder-forks.md` (U1–U5, each built on the arm taken
there, PROVISIONAL); U2 is a boundary widening (playbook `rul-r74hshif` · `rul-p2zl2kgl`), whose ratification stays his
own word at the wrap whatever answers this card.

## Goal
`viola up [<name>…] [--dry-run] [--json]` re-opens, after a reboot, every instance whose heartbeat is older than this
boot and whose log holds no `session-end` after its newest `session-start` (named ones whatever that reading): each
in a new `foot` running the pinned `viola revive <name> --origin <cli|agent>`, revive's window step moving the window
to the workspace its own snapshot recorded; parents first; live and stale ones skipped; one line per instance; one
`--json` document. Sequential, each awaited live under `OPEN_LIVE_BOUND` (no new bound). CLI-only.

## Top acceptance criteria
- The selection (boot-relative beat, no session-end after the newest start, the named form), the parent-first order,
  the live / stale skip — unit cases each held by a remove-the-guard control (`up-skip-live`, `up-order`, `up-boot`,
  `up-ended`).
- Each re-opened instance resumes its logged id in place through revive in a closed-argv `foot`, its revived start's
  records read from its own offset; its window moved to its recorded workspace, or `unplaced` with none.
- One line per instance in a closed word set; one `--json` document; `--dry-run` spawns nothing; a refused board exit 21
  with no line; no snapshot or stamp written by `up`.
- **matrix#v2-12, GATED on R4** (below).

## Touchpoints
New: `src/cmd/up.rs`, `tests/cli_up.rs`, `scripts/r4-up.sh`. Modified (16): `src/cmd/mod.rs`, `src/cmd/revive.rs`,
`src/cmd/open.rs` (shared spawn and live wait), `src/human/verbs.rs`, `src/human/hints.rs`,
`src/bin/viola-fake-agent/foot.rs`, `src/bin/viola-fake-agent/test_cases_modes.rs`,
`crates/viola-state/src/replay.rs`, `crates/viola-state/src/liveness.rs`, `schemas/diag-line.v1.json`, and six test
binaries' tables. `src/cmd/run.rs` and `src/run/` untouched.

## Test commands
22 entries: fmt, clippy; root units (baseline green, 1313 passed); the new unit cases (vacuous red, 0 selected);
viola-state units (green, 307); the `^up_` integration cases (vacuous red, 0 of 4258); ten reader binaries (green, 195);
`bash -n` and the R4a rehearsal (red on an absent script, a must-pass reading owed to implement's first firing); the
host read (QUIET); the three harness layers (seen green at the last wrap); the gated entry 14 (red on an absent record,
controlled by hand: `control-entry-14.md`); the operator pass (pre-push, hygiene, push, CI); and three operator-only
live phases (the R4a run, R4b `plant`, R4b `after`).

## Concretization preview — matrix#v2-12 (the never-weaken checkpoint)
- **Now:** After a host restart, one command re-opens every session that was live before it, each resumed in place
  in its recorded workspace and placement, parents before the sessions under them; it skips a session still live and
  prints one line per session.
- **Proposed (claimed GATED):** After a real host restart, one `viola up` re-opens the instances that were live before
  it — read as: heartbeat older than this boot and no `session-end` after the newest `session-start` (founder-forks U1
  arm 1, PROVISIONAL), or named — each resumed in place on its logged id through `viola revive`, in a new window moved
  to its recorded workspace (none recorded: `unplaced`), parents first; it skips a live or stale instance and prints
  one line per instance. Proven over the fakes for selection, order, skip, lines and argv; «after a real host restart»
  and «in its recorded workspace» on a real compositor only by R4a and R4b's green lines in `evidence/r4.md`.
- **Where the proposed form could read narrower:** «every session that was live» becomes U1's disk reading; if a
  graceful shutdown writes `session-end` (unmeasured, R4b reads it), the default selects nothing and the named form
  restores the fleet. That reading is U1's question to him, not a weakening made here.
- **Gated criterion:** `gated: R4, viola up once on a real compositor over a planted fleet (R4a) and once after a
  real reboot over a real parent and child (R4b), at the operator host, unlocked, its moments his to name
  (founder-forks U1-U5)` — a stand-in, stub or recorded artifact never verifies it. **R4b needs a real reboot.** R4a:
  0 real `claude` starts, 3 windows. R4b: 2 starts before the reboot, 2 resumes after it (4 in all).

## Findings for the review
- The plan's prose runs about 370 lines outside its 22-entry fence (524 lines in all), inside the 150–400 guide.
- The rows files were read by search, not whole: 426,657 B across the seven, past the 250,000 B cap; every row the
  plan cites was found in its rows file.
- v2-30 lists «an up» among its legs; «`viola adopt`» carries another, so v2-30 is not claimed.
- A plain `viola revive` (no `--origin`) writes a snapshot with no window: an instance revived by hand after a reboot
  comes back `unplaced` at the next `up`. Recorded, not changed.

## Beyond the plan
- none

Apply? (yes / review / cancel)
