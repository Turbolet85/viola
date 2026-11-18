# P5 review — 2026-11-18-trust-record-row-read-live · «Trust-record row read live»

Plan: `doc:2026-11-18-trust-record-row-read-live/plan` rev 2, rendered to `.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md`
for an editor. No plan stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live`
is a diff of the chunk's records.

## Goal
The relied `trust-record` row is read live once on 2.1.325 (S0′, spelled `S0R`), with the instruments to say why
Runs B–F stopped short of the input box in S0. Verify stamps one closed first-screen word per run A–F and that settle's
wall, both under `measured`. The rig's `V` arm prints them, and its cwd is made a git root seeded at the root's key.
The `verify_window_` kill moves with the measured walls, and a `--record` verify gets a witness of where its record
went. Nothing of `run` or `revive` changes, and no capability is claimed.

Forecast, not a measurement: the P3 timing reading leans toward S0R reading RED again. That reading is the ~40 s wall
against four 8.5 s unsigned settles, and it predicts `none` on B–F. If so, implement stops before the operator pass
with the cause named, and the mend goes through a revision that spends `S0R-2`.

## Top acceptance criteria
- With screens the fake stamps `a: trust` and `b`–`f`: `box`, each below 8 500 ms. Without screens, all six read
  `none` at 8 500 ms or more. No row and no `run` field reads them.
- A `--record` verify's record goes only into the kept `CLAUDE_CONFIG_DIR` file, reads `pass` there and is gone after.
  The real `~/.claude.json` is never asserted.
- The V rehearsal reads `git-root: written`, the six words, the six walls and `failing: none`. `S0R-3` is admitted as
  a spare.
- The `verify_window_` kill is 80 s in both profiles, with a planted-hang pair recorded both ways.
- Controls `fs-trust`, `fs-record` and `rig-gitroot` hold, each fired with `--timeout 900`.
- S0R runs once: ledgered first, `keys: none`, `stop:` printed. The 21 floor rows are `lacks` atoms on its `failing:`
  line, so a red there is the RED end. Its reading takes one of step 11's four ends.

## Touchpoints (14, all modified, none new)
- **viola-agent-claude:** `screen.rs`, `ledger.rs`, `ledger/test_cases.rs`, `ledger/test_cases_stamps.rs`.
- **verify:** `src/cmd/verify/typed/{run,runs}.rs`, `src/cmd/verify/typed.rs`.
- **Tests:** `tests/cli_verify/runs.rs`, `tests/cli_verify.rs`.
- **Rig:** `crates/viola-e2e/src/rig/{isolate,mod,ledger}.rs`, `crates/viola-e2e/tests/rig_rehearsal.rs`.
- **Config:** `.config/nextest.toml`.

## Test commands (19 entries)
- **Before the live round:** fmt; clippy; `cargo nextest run -p viola-agent-claude`; `run --unit`; `run --integration`
  filtered to `cli_verify`, `contract_ledger_probes`, `cli_version_gate`, `contract_lints` and `rig_rehearsal`;
  `run --integration`; `run --e2e`.
- **The round:** the install read by path, census, build plus hash, `live S0R --cap 1 --spare 2`, census.
- **The operator pass:** smoke boot/status/cleanup `screen-smoke`, pre-push, hygiene, push, `ci.py conclusion`.
- **Baselines** on the untouched tree, fired at this P5:

  | Entry | Reading |
  |---|---|
  | 3 | green, 591 passed |
  | 4 | green, 2675 passed |
  | 5 | green, 73 passed |
  | 6 | green, 524 passed |
  | 7 | green, 320 passed |

- Entries 1–2 were seen green at the previous wrap; entries 8–19 are legs and exempt.
- **Not run:** the release check, with the reason in the prose. No mutation run.

## Concretization preview
None. No capability is claimed (cry-ufpnw7gv): `matrix#v2-33` stays pooled.

## Founder forks
None arose. The overseer's «git-initialised» is met by writing git's own minimal layout as files, not by spawning
`git`, so the rig gains no process crossing. That lean is stated in the plan's Constraints, citing security
`decision/live-rig-isolated-child` and directive item «most conservative option». No `founder-forks.md` was written.

## Directive item 12 — verbatim
Union, per master, as `rec.py rows-union` printed it:
```
architecture: rows 964 · packs 10 · picked 142 (100 by every draw) · standing 21 · 43709 B
security-plan: rows 610 · packs 7 · picked 99 (63 by every draw) · standing 18 · 30162 B
design-system: rows 316 · packs 4 · picked 60 (44 by every draw) · standing 10 · 14826 B
layout-templates: rows 222 · packs 3 · picked 32 (19 by every draw) · standing 12 · 6847 B
test-plan: rows 1343 · packs 14 · picked 221 (123 by every draw) · standing 38 · 58220 B
obs-plan: rows 1025 · packs 11 · picked 213 (130 by every draw) · standing 8 · 50831 B
a11y-plan: rows 235 · packs 3 · picked 69 (39 by every draw) · standing 8 · 19176 B
rows-union: 7 master(s) · picked 836 of 4715 · rule any
```

**How the rows were read**
- All seven rows files were read whole. Their sum is 223 771 B, under 250 000 B.
- The seven standing lists were read as follows:
  - tests, arch, security and obs: whole.
  - design, layouts and a11y: their `- S` lines only.
- The history files were read as follows:
  - arch, tests, security and obs: whole.
  - design, layouts and a11y: through a 260-character line cut, not whole. This is recorded as a deviation in the
    plan checkpoint.

**Cited rows:** all 24 rows the plan cites stand in their master's rows file. This was checked by grep over `*-rows.md`
and `*-standing.md`, one hit each.

**`[RETYPED]` lines and re-spawns**
- `[RETYPED]` lines: none printed.
- Re-spawns: one. `design-pick-p2.d3 · [EVERY_ROW] contract/exit-21-cause/socket-dir-refused answered 2 times — once` ·
  `… contract/revive-json-refusal-code/socket-dir-refused has no record`.
  - The re-spawn `pick-p1-r1` read `1 of 1 pass`.
  - Census: `re-spawned 1 agent(s) · EVERY_ROW 1`.

**Non-zero exits and refusals**
- **The bash guard, 4 refusals, each re-issued once in its passing form:**
  1. «a pipeline tool's output piped into a filter» (`ci.py … | head -0` at Setup 5a).
  2. «cd … moves the session cwd» (the sidecar contract read).
  3. «cd … moves the session cwd» (the evidence listing).
  4. «cd … moves the session cwd» (the standing-list grep).
- **`rec.py put --edit-file`, exit 3:** «`old:` «as they stand at `f862f193`, nothing wider (the overseer's p» hits 0
  times, one is owed — nothing written». The anchor straddled a wrapped line, and it was re-fired with a one-line
  fragment.
- **The stamp-ahead hook:** «Stamp ahead of the clock: 2026-11-18T00:52:00Z (+5 min) written to …plan.md». It was
  corrected from `date -u` to 00:46:30Z.

**Parse-verdict lines**
- `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`: none printed at any step.
- `planlint`: `hits: 0`. Its two 12 (d) listings were read and are consistent: `merge_stamp`'s signature is unchanged
  while `measured` grows, and the plan never asserts `~/.claude.json` untouched.

**Where a letter was wrong or silent**
- (a) Setup 5a's base pickaxe reads `.andromeda/master-route.md`, a path the spine now holds in the store. It still
  answered `0f3822b2`, so it worked through the render, but the letter does not say so.
- (b) The phase skill has its own copy of `evolve-system.md`. It was confirmed byte-equal to new-session's copy by
  `cmp`, and its §Tool reads `evolve v1.4`.
- (c) The rig ledger's spare rule reads only a `-2` suffix. The founder's R6 cap of two spares is unworkable without
  step 8, and no letter or carry named it. P4 found it in the code.

## Beyond the plan
none

Apply? (yes / review / cancel)
