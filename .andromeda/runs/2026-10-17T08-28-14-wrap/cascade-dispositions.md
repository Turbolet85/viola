# Cascade dispositions — 2026-10-17-cli-output-discipline

## Step 2 — the sweep
Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-17T08-28-14-wrap/cascade-patterns.toml` (baseline 6d8b662c, the
pre-CI parent), seven patterns derived from every amendment of this pass, each its claim's wording and mechanism:
`env-exhaustive` (the exhaustive env-read claims: "only other variables", "two ratified exceptions", "two variables outside",
"two carve-outs", "only env vars outside", "never reads env", "not a configuration channel", "reads no `NO_COLOR`") ·
`uncoloured` ("uncoloured", "emits no SGR", "no SGR yet", "prints words only", "no styling", "no bold NAME", "colour layer is
owed / lands") · `owed-cod` (`owed to "CLI output discipline"`) · `truncation` ("never truncated (", "nothing is truncated",
"no truncation", "clipping is not built", `COLUMNS`) · `help-built` ("clap's ordered list", "grouped verb table (not built",
the `handoff: link` row, the five-label list) · `viola-help` (`viola-help`, `` `viola --help` ``) · `stderr-writer` ("only
human-stderr writer", "called by `run` and by `revive`", "pre-spawn refusals, and `revive --list`"). Every control fired.
Listing: `total (7 patterns) · 49 rows over 14 files` · `per class · new 13/6 · standing 28/8 · leaf 8/6 · curation 0/0 · base 0/0`.

Rows:
- env-exhaustive (14 standing): architecture.md:157 amended (edited; "The two ratified exceptions are…" re-worded "Two of the
  ratified exceptions are…" beside the third, after the sweep) · architecture.md:404, security-plan.md:411, :543 amended
  (edited rows; each re-read — the exhaustive claim now names the three colour reads) · architecture.md:333, :334 no change
  (each a test seam's own "not a configuration channel", true) · obs-plan.md:39, :217, :820, :960, :1066, :1096 and
  registries obs-plan/service-identity.md:3, :5 no change (each says env vars are not a configuration channel, or the product
  never reads env as configuration — true: the colour reads configure nothing) · leaves CLAUDE.md:37 (true, no change),
  `.claude/rules/security.md:37` and `.claude/docs/conventions.md:13` → step 3.
- uncoloured (1 new · 3 standing): layout-templates.md:91 new (the amended As built: uncoloured under `--json` / pipe /
  `NO_COLOR` / dumb / failed VT — true) · design-system.md:79, :504, layout-templates.md:29 no change (the kind word and
  `fail` are uncoloured as built) · leaves CLAUDE.md:27, `.claude/rules/a11y.md:21`, `.claude/rules/frontend.md:28`,
  `.claude/docs/design-summary.md:50` → step 3.
- owed-cod (0 master rows): leaf `.claude/rules/frontend.md:28` → step 3.
- truncation (3 new · 3 standing): architecture.md:335, security-plan.md:208, design-system.md:420 new (the `COLUMNS` is
  not read clauses) · design-system.md:421 no change ("Otherwise nothing is truncated" — the unknown-width rule) ·
  layout-templates.md:91 amended (edited; "with no known width nothing is truncated", true) · test-plan.md:166 no change
  (events never truncated — a true claim sharing the token).
- help-built (1 new · 3 standing): design-system.md:526 amended (the planning row kept as the target; the As built sentence
  beside it) · layout-templates.md:211 no change (the wireframe's target row, its As built note beneath) ·
  layout-templates.md:215 new (the As built note) · a11y-plan.md:209 amended (the target label set kept; the As built clause).
- viola-help (8 new · 5 standing): every new row is this pass's own text (layout-templates.md:215, test-plan.md:542, :585,
  :614, :931, a11y-plan.md:209, :234, :303) · design-system.md:526, layout-templates.md:59, test-plan.md:589 amended ·
  layout-templates.md:202 no change (the heading) · test-plan.md:595 no change (the completeness case reads `Options:` on).
- stderr-writer: 0 rows, the control fired — the retired wording stands nowhere in the masters or the leaves.

## Leaves
Mode: pass (this chunk closes no epoch). Set: `cascade.py leaves --add CLAUDE.md --add .claude/rules/security.md --add
.claude/docs/conventions.md --add .claude/rules/a11y.md --add .claude/rules/frontend.md --add .claude/docs/design-summary.md`
(the step-2 `leaf` rows) → `total · pass · 14 group(s) · 28 leaves · 310105 B · unmapped 2`. Unmapped:
`.claude/rules/host-linux.md` (the host's own leaf — no comparer) and `.claude/rules/testing-src.md` (a pointer list into
testing.md that states no master fact — no comparer). Comparers: one batch of 14 inside one snapshot pair (before
`tree df10cc6e… index 7d15df5e… head 4a8a0ca5…`, after `tree e4fb7f4a…`, `git diff-tree` printed exactly the fourteen
`leaf-{group}.md` additions, index and head unchanged; every receipt equal to its file's blocks).
Apply: `total · blocks 21 · applied 19 · preserved 1 · reshaped 1`.
- architecture-1: 1 finding · applied 1 (`.claude/docs/conventions.md:13`, the exceptions).
- architecture-2: 5 · applied 5 (`.claude/docs/stack.md:11`; `.claude/docs/services/viola.md:6`, `:11`, `:29`, `:45`).
- architecture-3: 0.
- architecture+security-plan: 1 · applied 1 (`.claude/rules/api.md:29`, the colour reads beside human text).
- masters: 2 · applied 2 (`CLAUDE.md:27` `list` no longer uncoloured; `CLAUDE.md:37` the env reads outside `VIOLA_*`).
- architecture+design-system+layout-templates: 4 · applied 3 (`.claude/rules/frontend.md:28` ×2, `:29`) · declined 1
  (`frontend.md:27#1`, a `tree`-class rewording of the section heading: preserved by the tool, and the heading is the leaf
  layout's own (setup's template), no comparer's and no step-3 edit).
- architecture+test-plan+obs-plan: 0.
- security-plan: 3 · applied 3 (`.claude/rules/security.md:37` ×2, `.claude/docs/security-summary.md:38`).
- security-plan+obs-plan: 0.
- design-system+layout-templates: 1 · applied 1 (`.claude/docs/design-summary.md:50`).
- test-plan: 1 · applied 1 (`.claude/rules/testing.md:19`, the SGR parser in `tests/support/sgr.rs`).
- test-plan+obs-plan: 0.
- obs-plan: 0.
- a11y-plan: 3 · applied 2 (`.claude/docs/a11y-summary.md:29`, `.claude/rules/a11y.md:21#2`) · by hand 1
  (`.claude/rules/a11y.md:21#1`, reshaped by the tool over the `\xHH` backslash: written with the Edit tool as the finding's
  replace-with).

Not looked for: the [trycmd] family beyond the `viola-help` sites (carried to "Version done-check" on the halt's answer,
`inputs#I5`, never amended here).
