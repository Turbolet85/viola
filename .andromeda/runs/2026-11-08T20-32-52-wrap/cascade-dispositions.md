# Cascade dispositions — the 2026-11-08T20-32-52-wrap 0-pending wrap (census 0.2.0, the masters' built: no rows)

## The search
One call, `cascade.py sweep --patterns-file .andromeda/runs/2026-11-08T20-32-52-wrap/cascade-patterns.toml`, baseline
`df9af357` (trail `cascade-no-marker.json`, listing `sweep-pass-1.1.txt`, read whole). Fourteen patterns, each the
retired claim's own phrasing beside its name token:
c-01 the deferred-frontend phrasings (`deferred to 0.2.0`, `0.2.0 frontend`, `frontend design input`, `deferred 0.2.0
page`, `0.1.0 renders no page`) · c-02 `technology (is )?not chosen` · c-03 `another-os` · c-04 winget/Homebrew/Scoop ·
c-05 Azure Artifact Signing · c-06 the «on Unix» socket-directory qualifier · c-07 `e2e-web` / browser leg / browser
step · c-08 the Windows faces (Bahnschrift, Cascadia Mono, Windows 10 1709) · c-09 web strip / web racks / on the web /
a future page · c-10 `cookie_file`, `run/hook/mcp/ui` · c-11 D: dev volume · c-12 TUI mode · c-13 the cocked strip's
12px / 160ms and `--cock-offset` · c-14 the web spacing, radius and `--c-*` / `--rule-strong` tokens.
Every control fired. Not looked for: the budget gate and status-line phrasings (their census rows are `built: yes`,
left for their removing chunks), and `doc:input`'s phrasings (its rows wait for the resumed wrap).

## Rows — total 87 over 22 files (per class: new 0/0 · standing 11/9 · leaf 74/12 · curation 2/1 · base 0/0)
Standing (11):
- `.andromeda/design-system.md:78` fct-ajwnfbif (c-01 ×2) — no change: Border Progression's premise line, census C-148
  `built: yes`, left for «Budget gate removed».
- `.andromeda/a11y-plan-detail/aria-patterns-roles.md:29` fct-uok334yy (c-01) — amended (edits-main-5): the P6 cli
  mirror's caption order is the one the terminal fleet console reuses.
- `.andromeda/security-plan-detail/threat-model-summary.md:183` fct-v2lh3e2i (c-04 ×3) — amended (edits-main-5): v1.x
  release workflow without winget/Homebrew/Scoop.
- `.andromeda/test-plan-detail/test-scope-summary.md:98` fct-67zmkx76 (c-07) — amended (edits-main-5): `run`'s third
  suite is none, no browser suite.
- `.andromeda/a11y-plan.md:70` fct-ktqfqjb4, `registries/contracts/a11y-plan/ci-integration.md:8` fct-56elcbkj,
  `registries/contracts/test-plan/5-command-implementation.md:31` fct-hgewdmj2 (c-07, `edited`) — amended this pass
  (C-338, C-354, C-283); the new text names the browser leg only to say there is none, no duplicate of the retired claim.
- `.andromeda/a11y-plan.md:63` fct-gglbhu4k (c-12) — no change: it states the cli verbs' boundary (no prompt, TUI mode
  or pager), which stands — the CLI ban kept, scoped to the verbs; the console's a11y is «Fleet a11y verification»'s.
- `.andromeda/design-system-detail/color-palette.md:13` fct-ylpqayvy, `:27` fct-hx4kd35c (c-14) — no change: Core
  Colors (C-138 keep) and Border Progression (C-148 `built: yes`); their `--rule-strong` name leaves with those rows.
- `.andromeda/layout-templates.md:37` fct-lxjb27mz (c-14) — amended (edits-main-5): the column-gutter row's first cell
  «Spacing», no longer the retired `space-xs` token.
Curation (2): `.claude/docs/session-learnings.md:157`, `:237` (c-07) — no change: preserve-verbatim history of past
events, neither a census row.
Leaf (74 over 12 files): every leaf joins step 3 (below).

## Leaves
Pass mode, `cascade.py leaves` against `df9af357`: 15 groups, 27 leaves; one comparer per group (`--batch leaf`,
15 of 15 pass, path check PASS). `total · blocks 69 · applied 64 · preserved 5`: the 64 written by
`cascade.py leaves --apply` (each finding's source side read at its cited fact); the 5 `preserved` are headings —
`.claude/rules/a11y.md:15`, `.claude/rules/frontend.md:10`, `:16`, `.claude/docs/design-summary.md:13`, `:52` —
written by hand, each file drafted whole (`draft-*.md`) and landed by `rec.py harness land` (TX 757–759). Declined: 0.
Groups with no finding: architecture-4, obs-plan, security-plan, security-plan+obs-plan, test-plan+obs-plan. Unmapped:
`.claude/rules/host-linux.md` (host leaf, no comparer's), `.claude/rules/testing-src.md` (its census rows C-445–C-448
are Session-Additions pointers, curation's, owed to the resumed wrap).
