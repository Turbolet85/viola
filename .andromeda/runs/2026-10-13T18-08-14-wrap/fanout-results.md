# Fan-out results — 2026-10-13-epoch-4-survivors-ii

Path check: before `tree 708a2f6d899603cf8bbb1b4d621053c343506ff7 index ba84587306a9ad87e15be2bacc160c8d2fc7bea2 head 94f0be906ceb52253bf9499b519b4173f8637cbe`,
after `tree f1ad6b2a28aa27df71e42ab9bf97b2d56f6e3c61 index ba84587306a9ad87e15be2bacc160c8d2fc7bea2 head 94f0be906ceb52253bf9499b519b4173f8637cbe`.
`git diff-tree` printed exactly the seven `A …/fanout-{doc}.md` lines; index and head unchanged. No file needed a fix,
so there are no twins. Every receipt's count matched its file.

- architecture: 0 proposals · fanout-architecture.md — note: D-arch-resources and D-arch-decisions hold (no
  resource; no dependency; Run B's ending order kept).
- security-plan: 0 proposals · fanout-security-plan.md — note: no new input surface, no auth, no dependency.
- design-system: 0 proposals · fanout-design-system.md — note: no UI element, every tokens flag n/a.
- layout-templates: 0 proposals · fanout-layout-templates.md — note: no surface added; §Output structure —
  `viola verify` stands.
- test-plan: 3 proposals · fanout-test-plan.md — note: D-tests-obs-harness no drift; the kill-filter fact is routed
  to P3 curation (`inputs#I2`).
  1. D-tests-coverage · §10 Quality Gates → Mutation gate · **apply** — check 1, playbook "Accurate this-chunk
     addition" (routine). It is also check 5's expected amendment (the plan's one entry), carried by the report's
     Counts / qualifiers moved bullet. The applied text is re-derived from the report, not pasted. Its count carries
     its rule: the 45 kills are the 29 of chunk 2026-10-13-epoch-4-survivors and the 16 of this chunk (playbook "A
     count in a master that carries no rule").
  2. D-tests-framework · §4 Unit Test Strategy, the "Module logic with mocked dependencies" bullet · **reject** —
     check 1, playbook "Registry over-reach" (routine). The bullet bounds mockall to the seam traits ("mockall doubles
     only for traits the product already defines at its seams"); it does not mandate mockall for every double. The
     proposal's own rationale reads "the seam-only rule held", so no claim of the body is false. Recording one
     module's hand-written `Recording` / `FixedClock` would write per-item realization into a strategy that names
     libraries and seams by category. The test framework (nextest, rstest) is the plan's, so D-tests-framework's
     invariant holds.
  3. D-tests-framework (dependent-of D-tests-framework) · §8 Mocking & Stubbing Discipline, the Process mocking row ·
     **reject** — check 6's group rule: its primary (2) is rejected. On its own the row also stands true: mockall
     0.15.0 is still the library on the `Pty` seam (viola-pty's `MockPty`), and the Anti-monkey-patching line
     "Doubles exist only for traits viola already defines at its seams" holds.
- obs-plan: 0 proposals · fanout-obs-plan.md — note: no log line added; `Run::finish` keeps its `process-exit`.
- a11y-plan: 0 proposals · fanout-a11y-plan.md — note: no UI element, no schema change.

Validate:
- check 2 (cross-contradiction): none.
- check 3 (intent-consistency): the report's two deviations are justified within the plan's intent; scope record
  none.
- check 4 (absence needs evidence): n/a; no proposal claims an absence.
- check 5 (expected amendments): the plan's one entry (test-plan §10 Mutation gate) is matched by proposal 1.
  citation-dispositions: no `claim false` row.
- check 6 (disproved claims): the report's one entry (the plan's and the leaf's kill filter
  `test(/(^|::)tests::<name>$/)` selects no rstest table case) is routed to P3 curation: a correction in place of
  `.claude/rules/verification-harness.md` `## Session Additions` 2026-09-27, on the operator's word `inputs#I2`. No
  master states the form (site sweep pattern `killfilter`, control unfired).

Proposals rejected for a source the report does not carry: 0. Escalations: 0.
