# Cascade dispositions — 2026-11-06-version-done-check (wrap 2026-11-06T20-57-05)

## The search
- One `cascade.py sweep` call over the pass's whole pattern set, baseline `aec2ec95` (the parent of the oldest pre-CI
  commit `849aad18`): the apply writers' `@pattern` records (`apply-architecture.p1.md` arch1-01…08,
  `apply-architecture.p2.md` arch2-01…09, `apply-security-plan.md` sec-01…08, `apply-design-system.md` ds-01…02,
  `apply-test-plan.p1.md` tp1-01…09, `apply-test-plan.p2.md` tp2-01…12, `apply-obs-plan.md` obs-01…02,
  `apply-a11y-plan.md` a11y-01…05) and main's own for the texts main wrote (`edits-main.md`, `edits-main-2.md`,
  `edits-main-3.md`) in `cascade-patterns.toml`: m-01 the retired pipe-bit controls (`CreateNamedPipeW`,
  `PIPE_ACCEPT/REJECT_REMOTE_CLIENTS`), m-02 the board read rule's delegate OPEN, m-03 the socket-directory pair's
  planning-form OPEN, m-04 the CLI test tools (`assert_cmd`, `trycmd`, `snapbox`, `predicates 3.1.4`, `tests/cmd`), m-05
  `pin-utils`, m-06 the owner «Epoch 7 boundary batch», m-07 «inherited risk». Every pattern's control fired on the
  pre-pass masters.
- Sections read: every master body, category file and key file the store renders (the sweep's scope), the three
  curation homes and the two judgment bases.
- Not looked for: wording of facts this pass did not amend.

## The first listing, and what it moved
The first call (all 62 patterns) printed 88 rows over 26 files (`per class · new 4/4 · standing 31/13 · leaf 53/11`).
Its m-04 and tp1-07 rows held 23 standing master sites still naming `assert_cmd` / `predicates` / `trycmd` / `snapbox`
as live tooling: test-plan `fct-4m5t4osf`, `fct-ennvbmje`, `fct-fbkvffnw`, `fct-aajqyzhy`, `fct-lzdm6e7w` (four lines),
`fct-sik7tvk4`; a11y-plan `fct-xmkdtzxa`, `fct-yzqlnwsx`, `fct-jkkyxmll`, `fct-ijnl2u6y`, `fct-2jdilj7v`, `fct-s4vibfo3`,
`fct-ms7djxet`, `fct-vtox22ts`, `fct-dx4kie52`, `fct-uok334yy` (three lines), `fct-g3h7n6e2`, `fct-es7le4as`. The tree
holds none of those tools (`git grep` over every tracked file outside `.andromeda/`, `.claude/`, `CLAUDE.md` and
`viola-0.1.0/`: 0 rows; `tests/cmd/` absent), so each was stale against CARRY 1's direction D1. Main amended all 18
facts in `edits-main-3.md` (TX 486), in D1's form, and the final listing below reads them gone.

## The final listing
`total (62 patterns) · 62 rows over 19 files` · `per class · new 2/2 · standing 7/6 · leaf 53/11 · curation 0/0 · base 0/0`

### Master rows (new 2 · standing 7)
1. `.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8` sec-04 standing `fct-s6rhzqu4` — no change: a true
   claim sharing the token (the profraw channel's own epistemic status, not the fd-inheritance mechanism).
2. `.andromeda/test-plan-detail/quality-gates-coverage-targets.md:20` sec-04 standing `fct-ocdr3xdn` — no change: a true
   claim sharing the token (a coverage red's status).
3. `.andromeda/registries/contracts/test-plan/test-data-bootstrap.md:15` sec-04 standing `fct-x3qmgvn2` @c1132 — no
   change: a true claim sharing the token (the ownerless-remnant mechanism, M2 open).
4. `.andromeda/test-plan.md:42` tp1-08 standing `fct-njspb5nv` — no change: a quotation of a source's own heading (the
   brief's creator excerpt «Must-Work (Windows ConPTY EOF)»), not a claim about the tree.
5. `.andromeda/test-plan-detail/integration-test-strategy.md:94` m-04 standing `fct-lzdm6e7w` (edited, mixed ×4) —
   amended this pass (`edits-main-3.md` E6): D1's absence statement, a true claim naming the four tools.
6. `.andromeda/architecture.md:25` m-05 standing `fct-omwrdnlc` (edited) — amended this pass: the graph holds no
   `pin-utils`, a true claim.
7. `.andromeda/architecture-detail/established-decisions.md:14` m-05 new `fct-ad44nn2e` — amended this pass, a true claim.
8. `.andromeda/security-plan-detail/dependency-security.md:16` m-05 standing `fct-6nwwowx3` (edited, mixed ×2) —
   amended this pass: RUSTSEC-2026-0441 closed by the move, a true claim.
9. `.andromeda/test-plan-detail/ci-integration.md:63` m-05 new `fct-zwsm4wxk` ×2 — amended this pass, a true claim.

### Leaf rows (53 over 11 files) — each re-derived at step 3 (`cascade.py leaves --add`)
- `CLAUDE.md:21` (arch1-06, sec-02, tp1-01, tp2-01, obs-01, a11y-05) — the `viola-pty` module line's `=0.8.1`.
- `.claude/rules/security.md:31` (arch1-06, sec-02, tp1-01, tp2-01, obs-01, a11y-05) — the cargo-deny ignore's `=0.8.1`;
  `.claude/rules/security.md:14` (m-02, m-06) — the board read rule's delegate OPEN.
- `.claude/rules/a11y.md:25` (a11y-03, m-04 ×2) — the CLI test tools.
- `.claude/rules/testing.md:18` (a11y-01, m-04 ×2) — the CLI test tools and their versions.
- `.claude/docs/services/viola-pty.md:6`, `:11` (arch1-06, sec-02, tp1-01, tp2-01, obs-01, a11y-05) — the pin;
  `:27` (arch1-02, m-05) — the RUSTSEC-2026-0441 owner «Version done-check».
- `.claude/docs/stack.md:12` (arch1-06, sec-02, tp1-01, tp2-01, obs-01, a11y-05) — the PTY row's pin; `:23` (arch1-03,
  arch1-08) — the error-enum divergences; `:47` (a11y-01, m-04 ×3) and `:50` (m-04 ×2) — the CLI test tools.
- `.claude/docs/conventions.md:15` (arch1-03, arch1-08) — the error-enum divergences.
- `.claude/docs/gotchas.md:24` (arch1-04) — the RUSTSEC-2026-0441 owner.
- `.claude/docs/a11y-summary.md:13` (a11y-03, m-04 ×2), `:18` (m-04 ×2), `:33` (m-04 ×2) — the CLI test tools.
- `.claude/docs/security-summary.md:64` (m-02, m-06) — the board read rule's delegate OPEN.
- `.claude/docs/tests-summary.md:32` (m-04) — «not trycmd».

### Curation homes · judgment bases
- curation 0/0 · base 0/0: no curation home and no rule or detector row holds a swept wording.

### The lateral binds
- test-plan §3 ↔ obs-plan §3 / §6 and a11y-plan's violation schema ↔ the log schema: no harness command, log field or
  violation field changed this pass; both sides read as before.

## Leaves
Mode: whole (this chunk closes Epoch 8), `cascade.py leaves --all`: 15 groups, 27 leaves, 418 607 B; `unmapped`
`.claude/rules/host-linux.md` (the host's own leaf, no master states it — no comparer's) and
`.claude/rules/testing-src.md` (section pointers into `.claude/rules/testing.md`, no master text, no swept token —
declined). One comparer per group in batch `leaf`, 15 of 15 pass. Tool total: `total · blocks 43 · applied 42 · escaped 1`.

| group | findings | applied | by hand | declined |
|---|---|---|---|---|
| a11y-plan | 4 | 4 | 0 | 0 |
| architecture-1 | 5 | 5 | 0 | 0 |
| architecture-2 | 13 | 13 | 0 | 0 |
| architecture-3 | 7 | 7 | 1 (sweep row `stack.md:23`, `:47`, `:50`, no finding) | 0 |
| architecture-4 | 0 | 0 | 0 | 0 |
| architecture+design-system+layout-templates | 0 | 0 | 0 | 0 |
| architecture+security-plan | 3 | 3 | 0 | 0 |
| architecture+test-plan+obs-plan | 2 | 1 | 1 (`commands.md:40#1`, refused `escaped`: its replace-with carries the master's backslash) | 0 |
| design-system+layout-templates | 0 | 0 | 0 | 0 |
| masters | 1 | 1 | 0 | 0 |
| obs-plan | 1 | 1 | 0 | 0 |
| security-plan | 1 | 1 | 2 (sweep rows `security.md:14`, `security-summary.md:64`, no finding) | 0 |
| security-plan+obs-plan | 0 | 0 | 0 | 0 |
| test-plan | 5 | 5 | 1 (sweep row `testing.md:18`, no finding) | 0 |
| test-plan+obs-plan | 1 | 1 | 0 | 0 |

By hand (each drafted whole to `draft-{path}` and landed by `rec.py harness land`, TXs 515–519):
- `.claude/docs/commands.md:40` — the coverage regex's separator class reads as kept from the retired Windows target
  (test-plan `fct-3mj52n2x`).
- `.claude/rules/security.md:14` and `.claude/docs/security-summary.md:64` — the board's read rule without its delegate
  OPEN and «on every OS» (security-plan `fct-pq4mk5dy`, the OPEN lifted on the founder's own word, 2026-11-06 19:34Z).
- `.claude/rules/testing.md:18` — the root tests' tooling as `std::process::Command` and literal pins (D1, test-plan
  `fct-lzdm6e7w`).
- `.claude/docs/stack.md:23` — the five error-enum divergences as ratified exceptions (architecture `fct-xqkfyxga`);
  `:47` — the dev-deps without `assert_cmd`, `predicates`, `trycmd` (test-plan `fct-2j6rht5l`); `:50` — the a11y
  tooling line in D1's form (a11y-plan `fct-trh3q5nm`).
- Left: `.claude/docs/tests-summary.md:32` «… not trycmd» — a true claim (the board is pinned by literals), no change.
