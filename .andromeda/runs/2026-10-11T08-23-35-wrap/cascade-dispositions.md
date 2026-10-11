# Cascade dispositions — wrap of 2026-10-11-the-board-viola-list

Sweep: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.2, baseline 995dc55a, the pre-CI parent),
run after the last body amendment of the pass. `total (17 patterns) · 56 rows over 17 files` · `per class · new 8/5 ·
standing 38/9 · leaf 10/8 · curation 0/0 · base 0/0`. Every pattern's control fired.

**The search.** Seventeen patterns, one per retired claim or restated mechanism: the replay's one product reader or
caller (`one-prod`); `budget.json`'s one reader (`one-reader`); "no surface shows" the skip counts (`no-surface`); the
board entry as an owner (`board-owner`); "keeping `\n` and `\t`" for `list` (`keep-nt`); trycmd pinning the board
(`trycmd-list`); the fifteen-binary kill line (`fifteen`); `viola list` unbuilt (`list-unbuilt`); "Two readers check
these files" (`two-readers`); the thresholds "read once at the wrapper" (`read-once`); `list` spawning `claude agents
--json` (`lmu-spawn`); the unwrapped rack (`unwrapped-rack`); the `config.json` second read (`second-read`);
`--statusline-echo`'s owner (`sl-echo`); "Three strict-modes refusals" (`three-sm`); the `cli` verb lists missing
`list` (`verb-list`); the grouped help (`grouped-help`). Sections read beside the rows: architecture §Standard
Contracts (Channel methods, Session liveness, Snapshot envelope), §Occupied Resources (Binary, Filesystem),
§Conventions (exit codes), [API Style], [Database / State Store]; security-plan §Authentication (`~/.viola/` access
control), §Input Validation (CLI arguments, Configuration values, Own state files, Child executable resolution),
Threat Model; design-system cli patterns 1–2 and the exit table; layout-templates Output structure `viola list` and
Primary content blocks 1–2; test-plan §4, §5, §6 (E5, Path 6, Chaos, parity), the three test-plan keys; obs-plan §1,
§2, §4, §6, §10 and the OTel init key; a11y-plan §1, §4, §6, §8.

**Not looked for:** wording of the `claude agents --json` contract itself (the item shape, the join field, the
unwrapped row's fields) — it stands as the contract, owed to its entry; the web surfaces' statements of the board.

## Rows

- `one-prod` test-plan:551 (edited) — amended this pass (first and second product caller) → amended.
- `one-prod` obs-plan:731 @c625 (edited) — "Its first product caller is `viola revive`'s start arm": true, and the
  amended line names the second → amended.
- `one-prod` .claude/rules/events.md:30 (leaf) → re-derived (two product readers).
- `one-prod` .claude/docs/services/viola-state.md:28 (leaf) → re-derived.
- `one-reader` architecture:258 — `cwd`'s one reader is `viola revive`: true (the board's types hold no `cwd`) → no
  change.
- `board-owner` architecture:71 @c2452 — the link/unlink hold → amended to "Explicit session links".
- `board-owner` test-plan:839 (new), test-plan:925 (edited), 5-command key:4 (new) — this pass's own "carried past /
  moved from" pointers, true → no change.
- `board-owner` .claude/docs/commands.md:30, .claude/docs/tests-summary.md:30 (leaf) → re-derived (new owner).
- `keep-nt` security-plan:437 (edited), test-plan:591 (edited) — amended to `wait` / `last` only → amended.
- `keep-nt` design-system:747 — quotes the security floor and says `list` is stricter: still true → no change.
- `trycmd-list` test-plan:655, a11y-plan:843, a11y-plan:853 (new) — this pass's text naming the literal pin → no
  change.
- `read-once` security-plan:237 (edited) → amended. Leaves .claude/rules/security.md:30 and
  .claude/docs/conventions.md:45 → re-derived.
- `lmu-spawn` architecture:334, security-plan:127, security-plan:245 @c67 (edited) → amended (none spawns yet).
  `crate-dependency-direction.md`:5 — sysinfo "shared by CLI `list`, `mcp` and `ui`": the liveness check, true for
  `list` as built → no change.
- `unwrapped-rack` design-system:358 (the ` - ` rule's sample), design-system:741 and layout-templates:376 (the
  contract's samples), layout-templates:580 (the rack separator's contract), test-plan:1009 (a scenario's expected
  stdout once unwrapped rows exist) — the contract, owed to "Unwrapped sessions on the board", each section now
  carrying an As built note beside it → no change. design-system:749, layout-templates:397, test-plan:655 (new) →
  this pass's text. a11y-plan:173, :348, :498 (edited) → amended.
- `sl-echo` test-plan:839 (edited ×2), 5-command key:4 (edited ×4) — the flag's form and its new owner → amended.
  Leaves commands.md:30, tests-summary.md:30 → re-derived.
- `verb-list` architecture:53, :123, :276, security-plan:59, test-plan:130, .claude/rules/api.md:21 — channel
  method lists; `list` is no channel method → no change. architecture:369, security-plan:102 — MCP tools, `list`
  present → no change. obs-plan:516 — span areas, `cli.list` named → no change. a11y-plan:90 — CLI verbs, `list`
  present → no change. .claude/docs/services/viola.md:34 (leaf, the `role_of` list) → re-derived (`list` added).
- `grouped-help` layout-templates:364 (edited) → amended (not built). design-system:839, layout-templates:562,
  a11y-plan:174 — the grouped help's contract, owed to "CLI output discipline"; the built state is stated at
  layout-templates:364 and a11y-plan:348 / :498 → no change. a11y-plan:348, :498 (edited, mixed) → amended.
- Zero-row patterns, controls fired: `no-surface`, `fifteen`, `list-unbuilt`, `two-readers`, `second-read`,
  `three-sm` — each claim was amended at its one site this pass.

## Leaves re-derived (cascade step 3)

From the sweep: `.claude/rules/events.md`, `.claude/docs/services/viola-state.md`, `.claude/docs/commands.md`,
`.claude/docs/tests-summary.md`, `.claude/rules/security.md`, `.claude/docs/conventions.md`,
`.claude/docs/services/viola.md`. By provenance of the amended sections: `CLAUDE.md` (two Modules lines:
`viola-state`, `viola`), `.claude/rules/observability.md`, `.claude/docs/obs-summary.md`,
`.claude/docs/security-summary.md`. Read and left: `.claude/docs/design-summary.md` (the CLI board as design),
`.claude/docs/a11y-summary.md` (P6 as a path), `.claude/rules/frontend.md:39` (the row escape, true),
`.claude/rules/verification-harness.md:34` and `.claude/docs/obs-summary.md:21` (`status` over `viola list --json`:
the harness contract; the status key states it is not run yet).
