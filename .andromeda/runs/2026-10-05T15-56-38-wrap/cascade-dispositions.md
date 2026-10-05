# Cascade dispositions — 2026-10-05-permission-end-to-end

**The search.** `cascade.py sweep` ran nine patterns (`cascade-patterns.toml`) over the seven masters, the registry files,
the curation homes, the judgment bases and the leaves. It ran after all six body amendments were applied (baseline
`2bd08e94`). Every control fired.

| Pattern | Kind | Covers |
|---|---|---|
| `pee-name` | fixed | `Permission end to end` |
| `unit-insta` | fixed | `unit / insta only` |
| `unit-level` | fixed | `stays unit-level` |
| `e2e-owed` | regex | `end-to-end (case\|witness) owed` |
| `perm-owed` | regex | the retired mechanism's phrasing however worded: `` `permission` (kind\|wake) … (unit\|owed) `` |
| `q-first-pr` | fixed | `first raised by PermissionRequest` |
| `null-list` | fixed | `another dialog already pending` |
| `script-set` | fixed | `gated-turn,path3,path4` |
| `prompt2-step` | fixed | `On prompt 2` |

The listing is `sweep-out.txt` (this run dir).

## Master rows
Every master row is an amended line of this pass. Each was re-read for an intra-line duplicate of the retired claim.

- `architecture.md:91` q-first-pr standing, edited (@c5052).
  - The new sentence (A1). The retired "owed to 'Permission end to end'" clause is gone: `pee-name` reads 0 master rows.
  - No duplicate on the line.
- `architecture.md:308` q-first-pr new · null-list standing, edited. The amended `null`-at-once list (A2). No duplicate.
- `test-plan.md:779` q-first-pr standing ×2, edited.
  - One match is the standing `v1-15` plan clause (a true claim sharing the token); the other is T1's new text.
  - `unit-insta` reads 0 master rows.
- `test-plan.md:1084` script-set, edited (T4).
- `test-plan.md:781` prompt2-step, edited (T3). The target-script step now names the as-landed separate script.
- `unit-level` and `e2e-owed`: 0 rows after the pass; their controls fired on `test-plan.md:757` pre-pass (T2 applied).

## Leaf rows
- `.claude/docs/tests-summary.md:27` (pee-name): re-derived from test-plan §6 Path 3 as amended. All three dialog
  kinds' wake witness landed with Path 4.
- `.claude/docs/tests-summary.md:28` (pee-name · unit-insta · perm-owed · q-first-pr): re-derived from test-plan §6
  Path 4 as amended. It now names the permission script, the two cases' coverage and the question left `null`.

## Leaves recomputed with no change
- **architecture.md amended in §Established Decisions [CLI Version Compatibility] and §Standard Contracts.**
  - CLAUDE.md `GENERATED:setup:*`: the overview, modules, warnings, pointer table and architecture paragraph name
    neither amended claim (the sweep found no CLAUDE.md row).
  - `docs/commands.md`, `conventions.md`, `gotchas.md`, `stack.md`, `rules/api.md`, `rules/events.md`: no statement of
    the `null`-at-once list or the owed clause (grep `hook.dialog|null` read; `events.md:21` "even when it replies
    `null`" stays true).
- **test-plan.md amended in §6 and §7.**
  - `docs/tests-summary.md`: the two lines above. Its §7 fake-script bullet names no script set.
  - `rules/testing.md` body: no Path 3 / Path 4 or script-set claim.
- `docs/services/viola-agent-claude.md:31` "PermissionRequest `allow` is ignored for it (measured twice)" is kept.
  - It is true for viola's bare `allow`, as research M6 records.
  - A PermissionRequest `allow` WITH `updatedInput` would satisfy ExitPlanMode statically, but viola never sends one.
  - The report carries it as a wrap note and amends nothing.
- Curation homes and judgment bases: 0 rows.
