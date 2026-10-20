# Cascade dispositions — 2026-10-20-epoch-5-cleanup

## The search
- The pattern set: `cascade-patterns.toml`, 11 patterns, derived from every amendment of this pass (the `mutants`
  kill and override set, the paste-hint class, the keepers' checks, the hook process group and the hangup, the census
  and the short-profile mechanism, the retired web components, the SSE scenario, the plain sibling, `viola-e2e`'s
  dependency list, the cocked-strip motion), each spanning the claim's verbs and its old wording.
- The final listing: `total (11 patterns) · 54 rows over 20 files` · `per class · new 7/3 · standing 24/9 · leaf 21/9 ·
  curation 2/2 · base 0/0` (the last firing; trail `cascade-2026-10-20-epoch-5-cleanup.json`).
- Read beside it: the sections each amended line sits in (design-system's whole §Brand Identity, §Motion and
  §Self-Validation; test-plan §6 E4 and §7's fake-agent bullets; obs-plan §4's intro and its last scenario).
- An earlier firing of this set read `C1-mut-kill standing 2` at `bootstrap-phases…md:12` and `:20`: test-plan
  proposals 2 and 3, not yet applied then. Both were applied, and the C1-C3 re-fire then read `standing 0` for C1.
- Not looked for: wording of the moved facts that carries none of these tokens; the detectors' sweeps of their own
  masters stand as the control for reach.

## Rows
- C1-mut-kill (old «45 s» kill):
  - `.claude/docs/gotchas.md:139` → leaf, step 3.
- C2-overrides: `bootstrap-phases…md:11 @c157` → amended (this pass, «plus four»).
- C3-paste-hint:
  - `test-plan.md:671` ×2 → no change: names the pair and its gate wait, states no kill.
  - `registries/contracts/a11y-plan/keyboard-test-harness.md:3` → no change: cites the case by name.
  - `bootstrap-phases…md:11` (new ×3) and `:12` (new) → amended (this pass).
  - `.claude/docs/tests-summary.md:26` → leaf, step 3.
- C4-keepers:
  - Amended this pass: `architecture.md:371`, `:388` (@c1300, 1390, 1855, 2372), `security-plan.md:536`,
    `5-command-implementation.md:6`, `test-data-bootstrap.md:13`.
  - `bootstrap-phases…md:24` (new) → amended (the libc line names the headroom read).
  - `test-plan.md:562` → no change: it points to §3 Test data bootstrap for the contract.
  - `5-command-implementation.md:43` → no change: `verify-exit-none` already covers a base the keeper refused.
  - Leaves, step 3: `.claude/rules/testing.md:33`, `.claude/rules/verification-harness.md:28`, `:42`,
    `.claude/docs/security-summary.md:73`, `.claude/docs/tests-summary.md:20`.
- C5-fg-group:
  - `test-plan.md:893` → no change: the `--exit-no-eof` holder's measured group facts.
  - `test-plan.md:962` → no change: the holder spawns in its own group, a true claim.
  - `test-plan.md:963` and `:1092` (new) → amended (this pass).
  - Leaves, step 3: `.claude/rules/verification-harness.md:41`, `.claude/docs/services/viola-pty.md:44`.
- C6-census:
  - `architecture.md:395` and `project-directory-structure.md:98`, `:101` → amended (this pass).
  - `test-plan.md:963` (new) and `:1092` → amended. `:1092 @c1171` is the first mechanism, which holds; `@c2648` is
    the census line, amended.
  - `test-plan.md:624` → no change: rmcp's child kill, another mechanism.
  - `.claude/rules/ci.md:32` → curation home, no change: «a short sha», another subject.
  - `.claude/docs/session-learnings.md:145` → curation home, no change: the counter method's entry, still true; its
    page-cut limit is a P3 candidate (report, Decisions & corrections).
  - Leaves, step 3: `.claude/rules/verification-harness.md:41`, `.claude/docs/commands.md:59`.
- C7-web-comp → leaves, step 3: `.claude/docs/commands.md:22` and `.claude/docs/design-summary.md:55`, `:56`, `:57`,
  `:58`. No master row stands.
- C8-sse → leaf, step 3: `.claude/docs/tests-summary.md:32`. No master row stands.
- C9-plain-sib:
  - `architecture.md:388 @c3987` → no change: the plain directory after `cargo clean`.
  - `5-command-implementation.md:35 @c3800` → amended (this pass).
  - `test-data-bootstrap.md:16` → no change: the same `cargo clean` fact.
  - Leaves, step 3: `.claude/rules/verification-harness.md:28`, `.claude/docs/commands.md:87`, `.claude/docs/gotchas.md:132`.
- C10-e2e-deps:
  - `bootstrap-phases…md:24` → amended (this pass).
  - `.claude/docs/stack.md:47` → leaf, step 3.
- C11-motion:
  - `design-system.md:28` (new) → amended (this pass).
  - `:80`, `:137`, `:315` → no change: they stand in sections framed «A 0.2.0 frontend design input; 0.1.0 renders no
    page» (`:85`, `:121`, `:301`), describing the deferred frontend, not a standing 0.1.0 motion.

## Leaves
`cascade.py leaves` (pass mode, the step-2 leaf rows and `CLAUDE.md` added): `total · pass · 13 group(s) · 25 leaves ·
339692 B · unmapped 2`. One comparer per group, one snapshot pair (the path check read exactly the 13 `A` lines
`leaf-{group}.md`, index and head unchanged). The apply: `total · blocks 21 · applied 20 · preserved 1`.

- **architecture-1** (pass) — 1 finding, 1 applied: `gotchas.md:133` (omission, the keepers' headroom refusal).
- **architecture-2** (pass) — 0 findings.
- **architecture-3** (pass) — 0 findings.
- **architecture+security-plan** (pass, `api.md`) — 0 findings.
- **architecture+security-plan+design-system+test-plan+obs-plan** (pass, `CLAUDE.md`) — 0 findings.
- **architecture+design-system** (pass, `frontend.md`) — 2 findings:
  - 1 applied: `:17`, retired (the two web motions → the base 0.2 calibration).
  - 1 by hand: `:16`, the `## Motion` heading (read `preserved`, a heading). Written `## Motion (base expression 0.2;
    the frontend's level deferred to 0.2.0)`.
- **architecture+test-plan+obs-plan** (pass, `commands.md`) — 2 findings, 2 applied: `:59` (the census's `--case`
  form), `:87` (the headroom refusal).
- **security-plan** (pass) — 1 finding, 1 applied: `security-summary.md:73` (the headroom refusal).
- **security-plan+obs-plan** (pass, `ci.md`) — 0 findings.
- **design-system** (pass, `design-summary.md`) — 10 findings, 10 applied: `:7`, `:9`, `:11` ×2, `:42`, `:55`–`:59`
  (the web components, the web motion, the readback box's one 0.1.0 place).
- **test-plan** (pass) — 5 findings, 5 applied: `tests-summary.md:32` (E4), `:20`, `testing.md:33`,
  `verification-harness.md:28` (the headroom refusal), `verification-harness.md:41` (the hook process group).
- **test-plan+obs-plan** (pass, `observability.md`) — 0 findings.
- **obs-plan** (pass) — 0 findings.

**By hand beyond the comparers** (step-2 leaf rows no finding covered):
- `.claude/docs/gotchas.md:138-139` — the `mutants` kill classes: «30 s and 45 s» → `send_window_`,
  `send_under_the_paste_hint_` or `verify_window_` at 30 s, 30 s and 60 s; verify's floor «about 34.5 s» → about 43 s,
  and the paste-hint wait named. Source: `bootstrap-phases…md:11`, `:12`.
- `CLAUDE.md:100` (`GENERATED:setup:deeper-topics`) — the services list drops `viola-ui`: the file
  `.claude/docs/services/viola-ui.md` left the tree this chunk (a tree fact; no master lists the services docs).

**Declined:**
- `.claude/docs/stack.md:47` (C10 row) — architecture's leaf. Architecture lists no `viola-e2e` dependency
  (`crate-dependency-direction.md:9`), and test-plan's key owns that list. No comparer flagged it.
- `.claude/docs/tests-summary.md:26`, `.claude/rules/verification-harness.md:42`, `.claude/docs/services/viola-pty.md:44`
  — read by their comparers with no finding: they name cases or holder facts that stand.

**Unmapped:**
- `.claude/rules/host-linux.md` — the host leaf, which no master states.
- `.claude/rules/testing-src.md` — a pointer list into `testing.md`'s sections, stating no sourced fact.
