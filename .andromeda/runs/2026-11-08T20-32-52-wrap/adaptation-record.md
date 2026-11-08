# Adaptation record — the 2026-11-08T20-32-52-wrap 0-pending wrap

**Request:** apply the 0.2.0 version census (`doc:viola-0.2.0/census` rev 2) on the card's word, the founder's own, typed
2026-11-08T19:59:58Z, «confirm the card as proposed» — the rows that describe what was never built (186 `retire` ·
`built: no`, 44 `replace` · `built: no`); the `built: yes` rows stay for their removing chunks. The directive:
`inputs#I1` (the Viola overseer, 2026-11-08). Its item 2: design-system's CLI ban on spinners, progress bars and
live-redrawing dashboards stays, scoped to the CLI verbs.

**Status: STOPPED AT A ROW BOUNDARY (directive item 4).** Applied in full apply-form (body, the cascade's sweep and
leaves, sidecar entry): every master and key-file row; noted: the 0.1.0 ledger rows; skipped: the residual rows the route
run already flipped. Not yet applied: `doc:input` (23 rows), `.andromeda/project.yaml` (1 row) and the
preserve-verbatim curation-home rows (listed below). A resumed 0-pending wrap skips every row marked applied, noted or
skipped here.

## Masters and key files — 89 rows: 87 applied, 2 held
Edits: `edits-main-1.md` … `edits-main-5.md` (TX 549, 550, 552, 560, 744); retires: fct-2zt7gsrv (TX 551) and the
design-system facts of `retire-design-system.txt` / `retire-design-system-2.txt` (183 facts). Sidecar entries TX 760–766
(amd-yhjsmkbm, amd-7ooyrxb2, amd-rtzftyxp, amd-iq2x6bhq, amd-p7aoh5og, amd-rdzriqfg, amd-prscbdsh, amd-tj7y5ae5).
Read-backs: `detail.py check --all --growth` VERDICT 0 defects; `classify.py check` PASS on every edited section and
key file. Sweep and leaves: `cascade-dispositions.md`.
- architecture — applied: C-005, C-008, C-009, C-015, C-016, C-027, C-062, C-063, C-067 (key build-system).
- security-plan — applied: C-091, C-109, C-110, C-116.
- layout-templates — applied: C-192, C-207, C-208, C-211, C-214.
- test-plan — applied: C-226, C-237, C-242, C-253, C-283, C-288, C-296. **Held: C-240** (a11y among the tier's
  downstream readers): the a11y plan stands with its cli/tui scope and the route entry «Fleet a11y verification», so
  removing a11y as a reader would write a false statement — raised to the operator.
- obs-plan — applied: C-312. **Held: C-301** (the instrumentability rule for a user-written shell command): its
  subject is the status line's `/bin/sh -c` shell-out, which the tree holds (search S-B: `statusline_command` 22 files),
  though the row reads `built: no · search S-W`; it waits for «Status-line interception removed» — raised to the operator.
- a11y-plan — applied: C-336, C-338, C-339, C-340, C-341, C-342, C-345, C-346, C-347, C-353, C-354, C-355, C-356, C-357.
- design-system — applied: C-130–C-136, C-139–C-144, C-146, C-147, C-149–C-170, C-172, C-173, C-175, C-176, C-177,
  C-183, C-185, C-188, C-190, C-191; and directive item 2 on fct-izuy43cb (C-187, a `keep` row: the ban now reads «in a
  verb's output», the console's redraw rule «Console scaffold»'s).
- Beside a row, by hand: C-010 (`built: yes`, the GUI page's budget-reading clause) left the body with the page C-009
  replaces, the clause having no page to stand in; the budget's removal stays with «Budget gate removed». Four
  standing citations of the retired claims the sweep caught were folded (`edits-main-5.md`).

## The 0.1.0 ledger — 7 rows noted
`matrix.py note --dir viola-0.1.0`, a dated note citing the row, status kept `deferred`: v1-11 (C-669), v1-22 (C-680),
v1-48 (C-706), v1-49 (C-707), v1-50 (C-708), v1-51 (C-709), v1-52 (C-710).

## Residuals — 30 rows skipped (already applied)
The route run flipped them (`.andromeda/runs/2026-11-08T19-45-30-route/residual-flips.txt`): C-714–C-726 and C-730–C-745
`dropped`, C-728 (res-ycydja7a) `absorbed` into v2-10.

## Leaves — the cascade's re-derivation, and what waits
The generated leaf rows (CLAUDE.md, `.claude/rules/{a11y,api,frontend}.md`, `.claude/docs/{a11y-summary,commands,
conventions,design-summary,gotchas,stack,tests-summary}.md`, `services/viola-pty.md`) were re-derived in this pass: 69
comparer findings, 64 written by tool, 5 headings by hand (`cascade-dispositions.md` §Leaves). Among them the census leaf
rows C-358, C-365, C-371, C-373, C-376–C-378, C-380–C-382, C-392, C-403, C-404, C-406–C-410, C-492–C-498, C-500–C-503,
C-509, C-525, C-528, C-533, C-535, C-536, C-538–C-546, C-548, C-550, C-561, C-613, C-620, C-650.
**Not yet applied** (preserve-verbatim curation homes, P3's channel, or a leaf no comparer of this pass read as stale):
C-395, C-396 (ci.md Session Additions) · C-438–C-444 (testing.md) · C-445–C-448 (testing-src.md) · C-456–C-458
(verification-harness.md) · C-596, C-600, C-601, C-607 (session-learnings.md) · C-556, C-560, C-563–C-566 (gotchas.md) ·
C-522 (conventions.md test prefixes) · C-582 (security-summary.md) · C-635 (services/viola-core.md) · C-657
(services/viola-state.md).

## Outside the directive's count
The 14 `clause retired · note now` rows on 0.1.0 caps (C-660, C-662, C-666, C-667, C-668, C-672, C-676, C-683, C-687,
C-698, C-700, C-701, C-702, C-704) are `built: no` but neither `retire` nor `replace`, so they fall outside the
directive's 230; left for the operator's word. C-694 is `keep`.

## Not yet applied — `doc:input` and `project.yaml`
C-459–C-463, C-465–C-472, C-474–C-476, C-478, C-479, C-481, C-483, C-485–C-487 (`doc:input`, the founder's intake and
brief) and C-488 (`.andromeda/project.yaml:4`).
