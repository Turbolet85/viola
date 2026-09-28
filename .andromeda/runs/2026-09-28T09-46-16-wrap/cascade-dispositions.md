# Cascade dispositions — wrap 2026-09-28T09-46-16 · chunk 2026-09-28-cli-output-tokens

**Search:** `cascade.py sweep` over `cascade-patterns.toml` (7 patterns, every control fired on the pre-pass masters at
baseline c04e3324): `writeln` (the retired two-`writeln!` helper) · `refuse-helper` (`` `refuse` ``) · `one-exception`
(the lone `.cmd`/`.bat` pre-spawn exception / "refusal fn") · `refusal-list` (`live or stale name`, the `unable:`/`hint:`
pair) · `human-stderr` (the human-stderr site) · `env-only` (the exhaustive env sentence and "not a configuration
channel") · `clap-pin` (`clap 4.6.7`). Sections read: arch Stack, Build system, Project directory structure, Config
management; obs §3, §11; test-plan §5 CLI; design-system cli Toolkit.

## Rows
- `writeln` — 0 rows (control fired at architecture.md:426 pre-pass): the retired helper wording is gone from all masters.
- `refuse-helper` — 4 `new` (architecture.md:426, :477; obs-plan.md:556, :1392): this pass's own text. No standing row.
- `one-exception` — obs-plan.md:556 `edited`: re-read — the pre-spawn exception now names all five start refusals and
  the writer; no duplicate of the retired single-refusal claim stands on the line. 13 standing rows (arch :4, :590;
  security :402, :479, :527; design :237, :296, :547, :862; test-plan :468, :831, :1351; obs :1176) — each a different
  "one exception" (ledger commands, launch URL, pinned bin, typography, gradient…): no change, a true claim sharing the
  token. 2 leaves (rules/security.md:6, rules/testing.md:20) — the same unrelated exceptions: no change.
- `refusal-list` — architecture.md:426 `edited mixed ×2`: re-read — the new list carries `live or stale name` once plus
  the writer description; no retired text. obs-plan.md:556 `new`. Standing: architecture.md:139 and security-plan.md:238,
  test-plan.md:1322 ("the two fixed `unable:`/`hint:` stderr lines") — true (text unchanged by this chunk): no change.
  test-plan.md:176, :529, obs-plan.md:620 (exit-1 start cases) — exit-code enumerations predating this chunk, naming no
  writer: no change (not this chunk's claim). Leaf `.claude/docs/services/viola.md:20` — run start order: no change
  (re-derived at :6 for the writer, below).
- `human-stderr` — architecture.md:426 `edited` (re-read, current), obs-plan.md:556 `new`, obs-plan.md:1392 `edited`
  (re-read: "may carry a local `#[allow]` … only where they use a print macro" — current), obs-plan.md:161 ("human stderr
  carries only fixed-message lines") — true: no change.
- `env-only` — architecture.md:587 `edited ×2` (re-read: the sentence plus the closed-route clause, current).
  security-plan.md:427 ×2 and :585 — cite arch's rule and name the two seams: true after this chunk, no change (the
  security detector read them the same way). arch :374, :375 (seam bullets), obs :39, :249, :604, :606, :1140, :1281,
  :1387, :1567 — "not a configuration channel" restatements: true, no change. Leaf CLAUDE.md:38 (Critical Warnings) —
  true, no change.
- `clap-pin` — architecture.md:16 `edited` (current), design-system.md:686 `edited` (current). Standing: arch :476
  (tree "clap 4.6.7 dispatch"), :627 (Framework line), security :229, layout-templates :322 ("Rust stable with clap
  4.6.7 (derive)"), obs :52, :829 — each names clap without a feature claim: true, no change. Leaves:
  `.claude/docs/stack.md:12` → **re-derived** (mirrors arch :16); `.claude/docs/services/viola.md:6` → **re-derived**
  (the human output module named).

## Leaves re-derived (step 3)
- architecture → `.claude/docs/stack.md` (§Stack row) · `.claude/docs/services/viola.md` (Responsibility: `src/human.rs`)
  · CLAUDE.md `GENERATED:setup:*` recomputed by read: overview/modules/warnings/pointer-table/architecture state no clap
  feature, refusal writer or env exhaustive claim beyond "env vars are not a configuration channel" — no change ·
  `.claude/docs/gotchas.md` (Cross-cutting leaf): no env-exhaustive claim — no change.
- test-plan → `.claude/docs/tests-summary.md` (no CLI-test enumeration) · `.claude/rules/testing.md`,
  `verification-harness.md` bodies (no CLI witness list) — no change.
- obs-plan → `.claude/docs/obs-summary.md:56` (the print ban, true) · `.claude/rules/observability.md` (no refusal
  site) — no change.
- design-system → `.claude/docs/design-summary.md` (no toolkit line) · `.claude/rules/frontend.md` CLI surface (colour
  decision order, true) — no change.
- Binds: test-plan §3 ↔ obs §3 — the harness is unchanged; obs §3's amended bullet is the run terminal rule, not the
  harness contract. a11y ↔ obs schema — unchanged.
- Curation homes and judgment bases: 0 rows in every pattern.
