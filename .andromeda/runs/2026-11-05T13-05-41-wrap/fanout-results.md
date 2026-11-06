# Fan-out results — 2026-11-05-linux-only-code-and-ci (wrap run 2026-11-05T13-05-41-wrap, re-fanned at the 2026-11-06 resume)

The dead window's fan-out (stage1, classify, an unchecked strong batch) was moved aside to `.raw-*` on the operator's word
(the window closed; the overseer, 2026-11-06 14:14Z) and re-fanned from a `--fresh` render. Every batch below passed
`fanout.py check`: stage1 12 of 13 then stage1-r1 1 of 1 (obs-plan's task writer, `[UNTOUCHED_REACHED]`, one re-spawn) ·
classify 144 of 144 (yes 367 of 1066) · strong 26 of 26 · newdetail 5 of 5. `sampler: off` (unforced).

## Verdicts

- architecture: 48 proposals · fanout-architecture.md · 119 cascade proposals · fanout-architecture--cascade-g1.md (9) ·
  -g2 (21) · -g3 (25) · -g4 (14) · -g5 (24) · -g6 (20) · -g7 (5) · fanout-architecture--cascade-new-established-decisions.md (1) ·
  4 facts · 4 placed · 0 no home
- security-plan: 35 proposals · fanout-security-plan.md · 77 cascade proposals · fanout-security-plan--cascade-g1.md (9) ·
  -g2 (12) · -g3 (11) · -g4 (28) · -g5 (14) · fanout-security-plan--cascade-new-dependency-security.md (3) · 4 facts · 4 placed ·
  0 no home
- design-system: 9 proposals · fanout-design-system.md · 19 cascade proposals · fanout-design-system--cascade.md (19) ·
  fanout-design-system--cascade-new-surface.md (0, `@none`) · 2 facts · 1 placed · 1 no home (F1, the Linux-only target:
  the body's cli Platform line carries it, `design-system-newdetail-surface.md`)
- layout-templates: 11 proposals · fanout-layout-templates.md (whole doc, no cascade)
- test-plan: 64 proposals · fanout-test-plan.md · 139 cascade proposals · fanout-test-plan--cascade-g1.md (22) · -g2 (9) ·
  -g3 (18) · -g4 (17) · -g5 (31) · -g6 (33) · -g7 (7) · fanout-test-plan--cascade-new-test-scope-summary.md (1) ·
  fanout-test-plan--cascade-new-unit-test-strategy.md (1) · 4 facts · 4 placed · 0 no home
- obs-plan: 21 proposals · fanout-obs-plan.md · 65 cascade proposals · fanout-obs-plan--cascade-g1.md (18) · -g2 (3) ·
  -g3 (21) · -g4 (23) · 2 facts · 2 placed · 0 no home
- a11y-plan: 26 proposals · fanout-a11y-plan.md · 29 cascade proposals · fanout-a11y-plan--cascade-g1.md (22) · -g2 (7) ·
  1 fact · 1 placed · 0 no home

Totals: 214 doc-agent proposals · 448 cascade proposals · 662 in all. No fix was made to any file (no `[NO_ESCAPES]` row):
every file is its raw record.

## Remarks carried (the files' `@remark`, substance only)

- architecture: conventions fct-nhsg4k7g «nine thiserror derives» where the report read 10 at HEAD and after — a drift
  older than this chunk; the winget / Homebrew / Scoop v1.x channels (fct-qndsom6a, [Deferred] fct-jqic6r22) and Windows-only
  rationales on standing items (fct-73drmylz, fct-5q3bjmyd, fct-4u2vf5lk) left for the orchestrator.
- security-plan: fct-p7rzex2i / fct-iftwwapx justify the no-leak property by Windows `CreateProcessW` `bInheritHandles`; the
  report states no Linux mechanism. g3: #3.12's test name ending `where_it_is_reported` is not stated renamed.
- obs-plan: fct-3epowryi's `exit_source:"kill-fallback"`, fct-cstqzpm2's `junit-<os>` upload, «per-OS perf job»
  (fct-4573rvoy / fct-fruln4ot) and fct-2geebg2r left unproposed as unclear from the report; error-capture's derive count «9».
- test-plan: D-tests-obs-harness — the shim-pair change against obs-plan §3; g5 n=27 the OS-gate census literals 52/7/59.
- a11y-plan: proposals 17, 19, 20 escalate — the tui screen-reader target that replaces NVDA + Windows Terminal is a
  founder-owned choice.
- layout-templates: the «on Unix» qualifiers of the socket-directory refusals (fct-7zli33yx, fct-qhojlk2s) stay true on Linux.

## Dispositions

Validate (main), 2026-11-06. check 3: the scope record's eight lines (companion 7 · mechanical 1) each serve a listed file and hold; no widening. check 5: the plan's Expected amendments are covered per doc by the proposals below; the citation row `claim false` (fct-7d3yogoz, 27 → 26 binaries) is raised by fanout-test-plan--cascade-g6.md 20. check 6: no `Spec claims disproved` entry against a master. No proposal rests on a source the report does not carry (0 rejected so). No boundary widening met: every removal narrows a surface the code left. Escalations: Q1 (resolved, the overseer's technical answer, founder-delegated, 2026-11-06 14:49Z).

fanout-architecture.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 2: the resize-key question re-pointed to the open residuals of origin 2026-11-04T23-11-43-wrap (target another-os), the sideload's DA1 stall dropped — the report's Expected amendments allow the re-point; g2 18's full retirement is rejected for it
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
26. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
27. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
28. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
29. reject — check 2: opposes fanout-architecture--cascade-g6.md 20 on fct-7wuxnmfc; the bullet stays, re-worded to the Linux dev host's NOCOW TMPDIR fact (an environment fact the report does not retire)
30. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
31. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
32. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
33. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
34. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
35. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
36. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
37. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
38. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
39. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
40. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
41. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
42. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
43. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
44. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
45. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
46. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
47. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
48. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-architecture--cascade-g1.md
1. apply-merged → fanout-architecture.md 2 — the same fact; the writer applies one text
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 7: architecture-newgroup-stack-and-technologies.md · contradiction · fct-q5vugilx · rewritten by fanout-architecture--cascade-g1.md 5
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-architecture--cascade-g2.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 7: architecture-newgroup-established-decisions.md · distinct
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply-merged → fanout-architecture.md 5 — the same fact; the writer applies one text
17. apply-merged → fanout-architecture.md 6 — the same fact; the writer applies one text
18. reject — check 2: opposes fanout-architecture.md 7 on fct-bq65okf4; the re-point to the another-os residuals stands
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply-merged → fanout-architecture.md 8 — the same fact; the writer applies one text
21. apply-merged → fanout-architecture.md 9 — the same fact; the writer applies one text

fanout-architecture--cascade-g3.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. reject — check 1: rul-in3bj4pn (not this chunk's drift: the report read 10 `thiserror::Error` derives at HEAD before the chunk and after, the tenth name not in it); the family (architecture fct-gctodeyz, conventions fct-nhsg4k7g, obs-plan error-capture's «9») routed to a CARRY on «Version done-check» at P5
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-architecture--cascade-g4.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply-merged → fanout-architecture.md 13 — the same fact; the writer applies one text
14. apply-merged → fanout-architecture.md 11 — the same fact; the writer applies one text

fanout-architecture--cascade-g5.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply-merged → fanout-architecture.md 16 — the same fact; the writer applies one text
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply-merged → fanout-architecture.md 25 — the same fact; the writer applies one text
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply-merged → fanout-architecture.md 19 — the same fact; the writer applies one text
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply-merged → fanout-architecture.md 21 — the same fact; the writer applies one text
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply-merged → fanout-architecture.md 23 — the same fact; the writer applies one text
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply-merged → fanout-architecture.md 24 — the same fact; the writer applies one text
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-architecture--cascade-g6.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply-merged → fanout-architecture.md 24 — the same fact; the writer applies one text
19. apply-merged → fanout-architecture.md 28 — the same fact; the writer applies one text
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-architecture--cascade-g7.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · contradiction · fct-xxlqtlbc · extended by fanout-architecture--cascade-g7.md 4 (the census's another-OS fault class, report Harness / gate surface)
5. apply-merged → fanout-architecture.md 44 — the same fact; the writer applies one text

fanout-architecture--cascade-new-established-decisions.md
1. apply — check 7: architecture-newgroup-established-decisions.md · contradiction · fct-gmzv3mtg · rewritten by fanout-architecture--cascade-g2.md 1 · as: provenance (the rulings' times, the chunk, inputs#) goes to the sidecar entry, the body states the target and the tag

fanout-security-plan.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
26. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
27. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
28. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
29. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
30. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
31. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
32. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
33. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
34. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
35. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-security-plan--cascade-g1.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply-merged → fanout-security-plan.md 11 — the same fact; the writer applies one text
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply-merged → fanout-security-plan.md 27 — the same fact; the writer applies one text
9. apply — check 7: security-plan-newgroup-threat-model-summary.md · contradiction · fct-v2lh3e2i · rewritten by fanout-security-plan--cascade-g1.md 7

fanout-security-plan--cascade-g2.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply-merged → fanout-security-plan.md 34 — the same fact; the writer applies one text
9. apply-merged → fanout-security-plan.md 1 — the same fact; the writer applies one text
10. apply-merged → fanout-security-plan.md 5 — the same fact; the writer applies one text
11. apply-merged → fanout-security-plan.md 2 — the same fact; the writer applies one text
12. apply-merged → fanout-security-plan.md 12 — the same fact; the writer applies one text

fanout-security-plan--cascade-g3.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-security-plan--cascade-g4.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply-merged → fanout-security-plan.md 25 — the same fact; the writer applies one text
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply-merged → fanout-security-plan.md 23 — the same fact; the writer applies one text
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply-merged → fanout-security-plan.md 15 — the same fact; the writer applies one text
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply-merged → fanout-security-plan.md 13 — the same fact; the writer applies one text
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply-merged → fanout-security-plan.md 18 — the same fact; the writer applies one text
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply-merged → fanout-security-plan.md 19 — the same fact; the writer applies one text
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply-merged → fanout-security-plan.md 29 — the same fact; the writer applies one text
26. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
27. apply-merged → fanout-security-plan.md 35 — the same fact; the writer applies one text
28. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-security-plan--cascade-g5.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply-merged → fanout-security-plan.md 3 — the same fact; the writer applies one text
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply-merged → fanout-security-plan.md 32 — the same fact; the writer applies one text
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply-merged → fanout-security-plan.md 10 — the same fact; the writer applies one text
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-security-plan--cascade-new-dependency-security.md
1. apply — check 7: security-plan-newgroup-dependency-security.md · distinct
2. apply — check 7: security-plan-newgroup-dependency-security.md · distinct
3. apply — check 7: security-plan-newgroup-dependency-security.md · distinct

fanout-design-system.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply-merged → fanout-design-system--cascade.md 3 — the same fact; the writer applies one text
8. apply-merged → fanout-design-system--cascade.md 4 — the same fact; the writer applies one text
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-design-system--cascade.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. reject — check 7: design-system-newgroup-typography.md · contradiction · fct-64jvg77y (rewritten by fanout-design-system--cascade.md 1); rejected on its own: it restates the Linux fallback as the rendered target, which fanout-design-system--cascade.md 4 already writes, and adds a frontend claim the report does not carry
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply-merged → fanout-design-system.md 6 — the same fact; the writer applies one text
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-layout-templates.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
26. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
27. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
28. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
29. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
30. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
31. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
32. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
33. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
34. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
35. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
36. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
37. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
38. apply — check 2: kept against fanout-test-plan--cascade-g1.md 22, which keeps the trigger's trace row · as: the multi-os-compat trigger is re-worded, not retired: the product is Linux-only, the workspace and the fake-agent E2E run on `ubuntu-latest`, and the census reads a gate naming `windows` or `target_os = "macos"` as a fault
39. reject — check 2: the multi-os-compat trigger stands re-worded (fanout-test-plan.md 38), so the heading's source stays
40. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
41. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
42. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
43. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
44. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
45. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
46. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
47. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
48. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
49. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
50. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
51. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
52. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
53. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
54. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
55. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
56. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
57. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
58. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
59. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
60. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
61. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
62. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
63. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
64. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan--cascade-g1.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply-merged → fanout-test-plan.md 5 — the same fact; the writer applies one text
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply-merged → fanout-test-plan.md 29 — the same fact; the writer applies one text
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply-merged → fanout-test-plan.md 18 — the same fact; the writer applies one text
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply-merged → fanout-test-plan.md 37 — the same fact; the writer applies one text
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan--cascade-g2.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply-merged → fanout-test-plan.md 9 — the same fact; the writer applies one text
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply-merged → fanout-test-plan.md 15 — the same fact; the writer applies one text
5. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply-merged → fanout-test-plan.md 28 — the same fact; the writer applies one text
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply-merged → fanout-test-plan.md 34 — the same fact; the writer applies one text

fanout-test-plan--cascade-g3.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply-merged → fanout-test-plan.md 6 — the same fact; the writer applies one text
6. apply-merged → fanout-test-plan.md 7 — the same fact; the writer applies one text
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply-merged → fanout-test-plan.md 19 — the same fact; the writer applies one text
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply-merged → fanout-test-plan.md 20 — the same fact; the writer applies one text
13. apply-merged → fanout-test-plan.md 21 — the same fact; the writer applies one text
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply-merged → fanout-test-plan.md 46 — the same fact; the writer applies one text
17. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan--cascade-g4.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply-merged → fanout-test-plan.md 45 — the same fact; the writer applies one text
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan--cascade-g5.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
26. apply-merged → fanout-test-plan--cascade-g5.md 25 — check 7: test-plan-newgroup-e2e-test-strategy.md · contradiction · fct-vgds3skp · rewritten by fanout-test-plan--cascade-g5.md 27; the another-OS fault class is the same fact fanout-test-plan--cascade-g5.md 25 writes into the census group
27. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5 · as: main measured the rule at this wrap — the rows of `GATES` in `tests/contract_os_gated_tests.rs` read 41: 39 OS-gated test functions and 2 gated files; the `cfg_attr` clause (`tests/hook_fail_open.rs`) drops (report deviation 9)
28. apply-merged → fanout-test-plan.md 35 — the same fact; the writer applies one text
29. apply-merged → fanout-test-plan.md 27 — the same fact; the writer applies one text
30. apply-merged → fanout-test-plan.md 10 — the same fact; the writer applies one text
31. apply-merged → fanout-test-plan.md 26 — the same fact; the writer applies one text

fanout-test-plan--cascade-g6.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 7: test-plan-newgroup-ci-integration.md · distinct
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
20. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply-merged → fanout-test-plan.md 17 — the same fact; the writer applies one text
24. apply-merged → fanout-test-plan.md 14 — the same fact; the writer applies one text
25. apply-merged → fanout-test-plan.md 47 — the same fact; the writer applies one text
26. apply-merged → fanout-test-plan.md 22 — the same fact; the writer applies one text
27. apply-merged → fanout-test-plan.md 23 — the same fact; the writer applies one text
28. apply-merged → fanout-test-plan.md 49 — the same fact; the writer applies one text
29. apply-merged → fanout-test-plan.md 48 — the same fact; the writer applies one text
30. apply-merged → fanout-test-plan.md 50 — the same fact; the writer applies one text
31. apply-merged → fanout-test-plan.md 40 — the same fact; the writer applies one text
32. apply-merged → fanout-test-plan.md 42 — the same fact; the writer applies one text
33. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan--cascade-g7.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-test-plan--cascade-new-test-scope-summary.md
1. apply — check 7: test-plan-newgroup-test-scope-summary.md · contradiction · fct-jnkzde5v · rewritten by fanout-test-plan--cascade-g1.md 1 · as: provenance (the rulings' times, the chunk) goes to the sidecar entry

fanout-test-plan--cascade-new-unit-test-strategy.md
1. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct

fanout-obs-plan.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-obs-plan--cascade-g1.md
1. apply — check 7: obs-plan-newgroup-obs-scope-summary.md · contradiction · fct-3og7ujbn · rewritten by fanout-obs-plan--cascade-g1.md 14
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply-merged → fanout-obs-plan.md 2 — the same fact; the writer applies one text
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply-merged → fanout-obs-plan.md 13 — the same fact; the writer applies one text
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply-merged → fanout-obs-plan.md 10 — the same fact; the writer applies one text
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-obs-plan--cascade-g2.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-obs-plan--cascade-g3.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply-merged → fanout-obs-plan.md 18 — the same fact; the writer applies one text
21. apply-merged → fanout-obs-plan.md 17 — the same fact; the writer applies one text

fanout-obs-plan--cascade-g4.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
3. apply — check 7: obs-plan-newgroup-log-coverage.md · contradiction · fct-yb5jpaot · rewritten by fanout-obs-plan--cascade-g4.md 1 (and fct-dsupowm6 by fanout-obs-plan--cascade-g4.md 2) · as: provenance (the overseer's answer, the chunk) goes to the sidecar entry
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. apply — check 1: rul-yzz4mbxd (a count moved; its rule named beside it, as the report re-measured it) with rul-5hynfkq5
20. apply-merged → fanout-obs-plan.md 19 — the same fact; the writer applies one text
21. apply-merged → fanout-obs-plan.md 7 — the same fact; the writer applies one text
22. apply-merged → fanout-obs-plan.md 8 — the same fact; the writer applies one text
23. apply-merged → fanout-obs-plan.md 14 — the same fact; the writer applies one text

fanout-a11y-plan.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
14. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
17. escalate → Q1 — the tui row's replacement screen-reader target is founder-owned · resolved: apply (Q1) · as: the tui pairing reads Orca + foot on Hyprland (the founder's host's one terminal, the xdg-terminal-exec default); the cli pairing stays Orca + GNOME Terminal; the pass supplemental and founder-owned, nothing gated on it
18. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
19. escalate → Q1 — the tui row's replacement screen-reader target is founder-owned · resolved: apply (Q1) · as: the tui pairing reads Orca + foot on Hyprland (the founder's host's one terminal, the xdg-terminal-exec default); the cli pairing stays Orca + GNOME Terminal; the pass supplemental and founder-owned, nothing gated on it
20. escalate → Q1 — the tui row's replacement screen-reader target is founder-owned · resolved: apply (Q1) · as: the tui pairing reads Orca + foot on Hyprland (the founder's host's one terminal, the xdg-terminal-exec default); the cli pairing stays Orca + GNOME Terminal; the pass supplemental and founder-owned, nothing gated on it
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
23. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
24. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
25. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
26. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-a11y-plan--cascade-g1.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply-merged → fanout-a11y-plan.md 25 — the same fact; the writer applies one text
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
8. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
9. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
10. apply-merged → fanout-a11y-plan.md 26 — the same fact; the writer applies one text
11. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
12. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
13. escalate → Q1 — the tui row's replacement screen-reader target is founder-owned · resolved: apply (Q1) · as: the tui pairing reads Orca + foot on Hyprland (the founder's host's one terminal, the xdg-terminal-exec default); the cli pairing stays Orca + GNOME Terminal; the pass supplemental and founder-owned, nothing gated on it
14. apply — check 7: a11y-plan-newgroup-a11y-scope-summary.md · contradiction · fct-vtox22ts · rewritten by fanout-a11y-plan--cascade-g1.md 12
15. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
16. apply-merged → fanout-a11y-plan.md 11 — the same fact; the writer applies one text
17. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
18. apply-merged → fanout-a11y-plan.md 12 — the same fact; the writer applies one text
19. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
20. apply-merged → fanout-a11y-plan.md 13 — the same fact; the writer applies one text
21. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
22. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)

fanout-a11y-plan--cascade-g2.md
1. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
2. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
3. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
4. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
5. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
6. apply — check 1: rul-5hynfkq5 (accurate this-chunk change: the named code and claims are the report's Retired · Symbols / APIs · Schema / config · Harness / gate surface), the OS target the founder's rulings (inputs#I1, inputs#I5 item 6)
7. apply-merged → fanout-a11y-plan.md 14 — the same fact; the writer applies one text

## Apply rulings (main, after the apply batch)

- Apply batch: apply-p1 (14) · apply-p2 (13; apply-test-plan.p2 re-spawned once, `[LABELS_WELLFORMED]`, apply-p2-r1
  pass) · apply-p3 (1); 28 writer files landed, TX 242–269.
- `v=folded_into` rows: each confirmed against its `apply-merged` line above.
- `v=refused · a hand removal` (whole facts): 51 facts retired by `rec.py retire`, TX 270–320.
- `v=refused · a hand removal` (a line inside a fact): 12 facts re-written as a replacement fact placed after the old one
  (`edits-main-3.md`, TX 321), the old one then retired (TX 322–333): fct-i65r3xxr, fct-ykziik74, fct-lgbstcvc,
  fct-phy2lvsc, fct-syqfxw5d, fct-uiwmcfut, fct-45kqhafb, fct-7jgwgzx2, fct-eoindiy7, fct-7lzg7pgb, fct-6whu4pj5,
  fct-q5vugilx.
- `v=refused` for a key-file target the writer was not handed (architecture 30–43, a11y-plan 6–9, 20, 21, 24, obs-plan 6,
  11, 12, 15, 16, 21): raised again by main as its own edits — `edits-main-1.md` (TX 335), `edits-main-2.md` (TX 336),
  `edits-main-5.md` (TX 337) — each text re-derived from the report; the tui pairing per Q1.
- fct-irhgihut (architecture §Project directory structure, a fenced tree): its in-line edits `edits-main-4.md` (TX 334);
  its seven lines naming deleted paths cannot be removed inside a fence → Q2 — the seven lines' removal is refused by the
  store's fence invariant · resolved: re-word as pointers (Q2; the overseer's technical answer, founder-delegated,
  2026-11-06 15:20Z) → `edits-main-6.md` (TX 338).
- fct-yb5jpaot (obs-plan log-coverage, the `process-start` table row): the edit drops pipes the store counts as cells
  (`reshaped · a table cell boundary added or removed`) and a replacement fact is refused (`its entities would not read
  one to one`) → Q3 · resolved: left standing (Q3), owned by a CARRY on «Version done-check» with its target text
  (`pty_backend` `openpty` alone, `sideload_fallback` gone) — the overseer's technical answer, founder-delegated,
  2026-11-06 15:20Z. fanout-obs-plan--cascade-g4.md 1 is applied in every clause but that row's.
- The doc agents' `@remark` facts (window 3, after the cascade sweep): fct-73drmylz, fct-qndsom6a — already re-worded
  this pass, true; fct-jqic6r22 — a later-version deferral, kept; fct-3epowryi — `ExitSource::KillFallback` stands in
  `viola-pty`, true; fct-cstqzpm2 — `junit-${{ matrix.os }}` still uploaded, true; fct-5q3bjmyd (the Windows relink
  reason), fct-4573rvoy and fct-fruln4ot («per-OS `perf` job»), fct-2geebg2r («quickly on Windows») — stale, routine,
  `edits-main-8.md` (TX 380); fct-p7rzex2i / fct-iftwwapx (the no-leak property justified by Windows `CreateProcessW`)
  → Q4 — no Linux mechanism in the report · resolved: re-word (Q4, the overseer's technical answer, founder-delegated,
  2026-11-06 16:25Z) · as: the property kept, its Linux mechanism close-on-exec «recorded, not established», the ban's
  `CreateProcessW` clause dropped; a CARRY on «Version done-check» owes the measurement (the wrapped child's
  `/proc/<child pid>/fd` holding no channel socket or listener, under the fake agent) — `edits-main-8.md` (TX 380).
- The residual census after the remarks (window 3): «per-OS» job wording and Windows/macOS present-tense clauses no
  sweep pattern named — a cascade miss, folded into this pass (`edits-main-9.md`, `cascade-dispositions.md`
  §Residual census).
- Routed to P5: the `thiserror::Error` census family (architecture fct-gctodeyz, conventions fct-nhsg4k7g, obs-plan
  error-capture's «9»; rul-in3bj4pn) and fct-yb5jpaot's row (Q3), each a CARRY on «Version done-check».
