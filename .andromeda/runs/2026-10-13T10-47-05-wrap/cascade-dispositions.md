# Cascade dispositions — 2026-10-13-epoch-4-cleanup

Search: `cascade-patterns.toml` in this run dir, run by `cascade.py sweep` (cascade v1.8, baseline `d9ad9f1f`, the
pre-CI parent), every control fired. Five patterns, each a retired or moved claim of this pass:
- `split-files` — the citations of the two split files by path (`human.rs`, `run/send.rs`, `src/human`, `run/send`);
- `scratch-recreate` — the scratch's re-creation and its lay-down mode (removes / recreates / wipes / DACL / owner-only / inherit);
- `human-writer` — the "only human writer" claim;
- `moved-units` — the names of the moved product and test items the masters cite with a file;
- `send-phases` — the send module's phases by their verbs (the local-command decision, the driver relabel, the slot, the post-condition claim, `budget_paused`).
Sections read beyond the rows: architecture [Delivery Confirmation], [CLI Version Compatibility], §Occupied Resources;
test-plan §4, §5, §9 Lint row, §3 → 5-command implementation; security-plan Code Patterns; a11y-plan §4, §8.

Listing: `total (5 patterns) · 82 rows over 17 files` · `per class · new 2/2 · standing 65/10 · leaf 15/7 · curation 0/0 · base 0/0`.

## Master rows
- amended this pass (the `edited` / `new` rows): `architecture.md:49` (@c8090 slot.rs, @c9701 classify.rs; @c3828 `src/cmd/send.rs` stands), `:91`, `:452`; `security-plan.md:607`; `test-plan.md:582`, `:583` (the `mixed` row: the one remaining `src/run/send.rs` names the root's one trailing-newline test, true), `:655`; `a11y-plan.md:543`, `:853`; `registries/contracts/architecture/build-system.md:4` (`mixed`: `src/human.rs` named as the root holding the writers, true), `project-directory-structure.md:21`, `:23`, `:24`, `:34`, `:35`, `:36`; `registries/contracts/test-plan/5-command-implementation.md:45` (`mixed`: the old "removes and recreates it" kept, followed by its new mode — true). Each amended line re-read: no intra-line duplicate of a retired claim stands.
- no change, true as it stands (split-files): `security-plan.md:228` ×5 (`src/cmd/send.rs` is another file; the wrapper's `send` stays in `src/run/send.rs`); `test-plan.md:566` (product caller `send` in `src/run/send.rs`), `:590` (`send_empty_text_*` stay in the root); `obs-plan.md:556`, `:1157` (`refuse` → `write_refusal` stay in the root `src/human.rs`); `test-plan.md:1123`, `obs-plan.md:767` (dated "as read in … at chunk …" readings).
- no change (human-writer): `obs-plan.md:556` ("the root bin's only human-stderr writer for a start refusal" — `refuse` → `write_refusal`, still in the root).
- no change (moved-units): `security-plan.md:437`, `design-system.md:749`, `layout-templates.md:584`, `test-plan.md:591`, `a11y-plan.md:594` (`escape_row` named with no file); `test-plan.md:566` (the `typed_text` tables in `viola-agent-claude`), `:752` (`tests/cli_send.rs`, an integration test); `a11y-plan.md:843` (names the unit, no file).
- no change (send-phases): the `budget_paused` snapshot / channel / board field rows — `architecture.md:72`, `:93`, `:210`, `:211`, `:212`, `:247`, `:255`, `:286`, `:287`; `design-system.md:541`; `test-plan.md:187`, `:202`, `:281`, `:379`, `:552`, `:827`, `:852`, `:853`, `:857`, `:863`, `:892`, `:949`, `:952`; `5-command-implementation.md:74` — a field name, not the moved method.

## Leaf rows (join cascade step 3's set; re-derived there, never patched here)
- `.claude/docs/services/viola.md:6`, `:20`, `:24`, `:41`, `:42`, `:43`, `:44`, `:45`
- `.claude/rules/verification-harness.md:44` (the scratch's re-creation)
- `.claude/docs/tests-summary.md:26`
- `.claude/rules/frontend.md:39`, `.claude/docs/security-summary.md:67`, `.claude/rules/api.md:26`, `.claude/rules/events.md:30`

Curation homes: 0 rows. Judgment bases: 0 rows.

## Leaves
`cascade.py leaves` (pass mode — this chunk does not close Epoch 5), 8 groups · 26 leaves; the sweep's leaf rows added.
Apply: `total · blocks 10 · applied 9 · reshaped 1`.
- architecture-1 · pass · findings 0.
- architecture-2 · pass · findings 7 · applied 7 (`.claude/docs/services/viola.md:6`, `:41` ×2, `:42`, `:43`, `:44`, `:45` — the human and send split's files).
- architecture+security-plan · pass · findings 0 (`api.md:26`, `frontend.md:39` name fields and `escape_row`, no file).
- architecture+security-plan+test-plan+a11y-plan (CLAUDE.md) · pass · findings 0.
- architecture+test-plan · pass · findings 1 · applied 1 (`.claude/docs/commands.md:53`, the Windows-target clippy chunk gate).
- security-plan · pass · findings 1 · applied 1 (`.claude/docs/security-summary.md:73`, the owner-only re-create).
- test-plan · pass · findings 1 · by hand 1 (`.claude/rules/verification-harness.md:44`: `reshaped`, the replace-with carried a backslash; written by hand with the same facts, "the workspace drive's Users write grant" in place of the escaped group name, the run id left in the master).
- a11y-plan · pass · findings 0.
- Unmapped: `.claude/rules/host-linux.md` (the host's — no comparer); `.claude/rules/testing-src.md` (no statement of a changed fact: no hit for the split files, the scratch or the lint) — declined.
- The sweep's leaf rows `tests-summary.md:26` (`tests/cli_send.rs`, an integration test) and `events.md:30` (a field) were read by their groups' comparers: no change.
