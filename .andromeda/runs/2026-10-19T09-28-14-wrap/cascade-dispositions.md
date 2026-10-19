# Cascade dispositions — 2026-10-19-dialog-hold-for-the-driver

## Step 2 — the sweep

`cascade.py sweep --patterns-file .andromeda/runs/2026-10-19T09-28-14-wrap/cascade-patterns.toml`, baseline `55699c4f`
(the parent of the oldest pre-CI commit 3def5b5b), run after every body of the pass was applied. The patterns are the
retired claims' wording and verbs:
- `deadline`: the retired constant, "dialog deadline", "rest of the deadline", "the deadline bounds".
- `timeout75`: the 75 s dialog timeout.
- `hookbound`: `DEADLINE + 5` and the 65 s bound.
- `kinds15`: 15 or fifteen kinds.
- `wake5`: a wake list that ends at `session-end` without `dialog-expired`.
- `hookonly`: "Only hook lines signal".
- `cfgfive`: five `CONFIG_KEYS`.
- `ondeadline`: "On deadline the hook" and "on expiry the hook".
- `nondialog`: "non-dialog kind".

Sections read besides the rows: obs-plan §6 (lines 717–797, no product-kind catalog) and §1 (no wake list or dialog
deadline).

Counts, copied: `total (9 patterns) · 17 rows over 9 files`; `per class · new 2/2 · standing 3/2 · leaf 12/6 ·
curation 0/0 · base 0/0`. `ondeadline` and `nondialog`: 0 rows, each control fired (the amended text no longer holds
them).

Master rows (5):
- `.andromeda/test-plan.md:525` deadline new: no change. The test name `dialog_deadline_is_the_longest_hold_plus_the_reply_grace` (the hook's function `dialog_deadline`) is a true claim sharing the token.
- `.andromeda/obs-plan.md:758` deadline standing: no change. The statusline bound, "not a dialog deadline".
- `.andromeda/architecture.md:49` wake5 new: no change. This pass's own list ends `session-end` or `dialog-expired` (the lookahead reads the word "or").
- `.andromeda/architecture.md:159` wake5 standing edited: no change. The normalised-kinds list, which now ends with `dialog-expired`, not a wake set.
- `.andromeda/architecture.md:237` hookonly standing edited: no change. This pass's new text, ending "(the wrapper's `send-*`, `link` and start records never wake)."

Curation homes: 0 rows. Judgment bases: 0 rows.

Leaf rows (12 over 6 files) go to step 3's set:
- `CLAUDE.md:20`
- `.claude/docs/services/viola-core.md:6` ×2 and `:20` ×2
- `.claude/docs/services/viola.md:41`, `:43` ×2
- `.claude/docs/services/viola-state.md:45`
- `.claude/rules/events.md:16`, `:17`
- `.claude/docs/obs-summary.md:72`

## Leaves

`cascade.py leaves` in the pass mode (this chunk does not close Epoch 5): `total · pass · 14 group(s) · 28 leaves · 340729
B · unmapped 2`. One comparer per group, in one batch inside one snapshot pair:
- before: `tree 92f374fe374937469dd8f4e3e3b91cf323d40fb3`;
- after: `tree 9d07a4bf3f80c2fd3806b36f628239b063391ca9`;
- index and head unchanged; `git diff-tree` printed exactly the 14 `A …/leaf-{group}.md` lines.

`leaves --apply`: `total · blocks 23 · applied 21 · held 2`. Every receipt matched its file's block count.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 6 | 6 (conventions.md:45, viola-core.md:6 ×2, :11, :20 ×2) | 0 | 0 |
| architecture-2 | pass | 4 | 4 (viola.md:41, :43; viola-state.md:6, :45) | 0 | 0 |
| architecture-3 | pass | 4 | 4 (stack.md:21; events.md:16, :17, :29) | 0 | 0 |
| architecture+security-plan | pass | 0 | 0 | 0 | 0 |
| masters | pass | 2 | 2 (CLAUDE.md:20 ×2) | 0 | 0 |
| architecture+design-system+layout-templates | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 2 | 2 (security.md:28, security-summary.md:66) | 0 | 0 |
| security-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| design-system+layout-templates | pass | 0 | 0 | 0 | 0 |
| test-plan | pass | 1 | 1 (testing.md:24) | 0 | 0 |
| test-plan+obs-plan | pass | 1 | 1 (observability.md:41) | 0 | 0 |
| obs-plan | pass | 2 | 1 (obs-summary.md:72) | 0 | 1 |
| a11y-plan | pass | 1 | 0 | 0 | 1 |

Declined:
- `a11y-summary.md:53` (omission): the line is about the MCP `wait` bound, and the leaf never stated the old dialog deadline, so nothing in it is false. The hold's detail stays in a11y-plan §8, and no reader of the summary acts against it.
- `obs-summary.md:42` (omission): the expiry's log detail stays in obs-plan §4. `rules/observability.md:41` (applied) carries it for anyone writing the code. The summary's event list stays true.

Unmapped:
- `.claude/rules/host-linux.md` (host): the host's leaf, no comparer's.
- `.claude/rules/testing-src.md`: the leaf layout's own pointer list; no master states it.
