# Cascade dispositions — 2026-11-18-session-folder-trusted-by-viola

Step 2's sweep (`sweep-pass-1.1.txt`): patterns `cascade-patterns-security-plan-1.toml` (the tool's, from the
ops-security-plan.md dry run its landing followed) and `cascade-patterns.toml` (main's: `ccverify`, `norecord`,
`nearest`). The other landings' dry runs wrote no file (`patterns: none: … changed prose alone`); none superseded.

Master rows (standing 18 over 4 files):
- `security1-11` (`claude_env_keep`) — 16 standing: architecture :682 dec-wkwptjhn, :684 dec-ysshfvlo, :810
  ctr-vbif2fqp, security-plan :209 dec-stjgcxbf, :307 ctr-qtnnf4ig, test-plan :52 dec-lv7ik2go, :539 dec-kvgndxkh,
  :629 dec-hvj7iqom, obs-plan :410 dec-5odyjbkd, :1037 ctr-4hxbzvo4 — no-change, true claims sharing the token (the
  pass-list's names); architecture :699, :700, :776 and security-plan :499, :500, :513 — edited this pass, re-derived.
- `ccverify` — 0 standing (every master site was edited this pass).
- `norecord` — 2 standing: security-plan :374 dec-sjrilbtp (probe-artifacts-cleanup) and :390 dec-pzgvqgc3
  (verify-probe-dirs-cleanup) — no-change: each states verify's own record as a kill or crash residual, still true.
- `nearest` — 0 standing (test-plan :841 edited this pass).

Leaf rows (18 over 10 files) go to step 3's comparers: CLAUDE.md:38, `.claude/rules/security.md` :17 :37,
`.claude/rules/verification-harness.md` :41, `.claude/docs/conventions.md` :13 :45, `.claude/docs/gotchas.md` :94,
`.claude/docs/obs-summary.md` :72, `.claude/docs/security-summary.md` :38 :72 :73,
`.claude/docs/services/viola-agent-claude.md` :28, `.claude/docs/services/viola.md` :6,
`.claude/docs/tests-summary.md` :21.

## Leaves
`cascade.py leaves` (pass mode; epoch «Epoch 4 — Driver reliability: ledger rows and bounds» open, markerless 7): 15
groups · 27 leaves · unmapped 2. `leaves --apply` total: `blocks 32 · applied 31 · held 1`.
- a11y-plan · pass · 1 finding · applied 1 (a11y-summary:53, the time bounds).
- architecture-1 · pass · 3 · applied 3 (conventions :13 :45, services/viola-agent-claude :28 — CLAUDE_CONFIG_DIR).
- architecture-2 · pass · 3 · applied 3 (services/viola :6 stale «run and revive write no trust record»; :20 and :21
  omissions, the start's trust step and the revive's key — a reader of the run notes would otherwise miss it).
- architecture-3 · architecture+layout-templates · architecture+security-plan · security-plan+obs-plan · pass · 0.
- architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan · pass · 1 · applied 1 (CLAUDE.md:38).
- architecture+test-plan+obs-plan · pass · 3 · applied 3 (commands :23 stale; :19 and :27 omissions).
- layout-templates · pass · 1 · applied 1 (design-summary:50, omission).
- obs-plan · pass · 1 · applied 1 (obs-summary:42, the event list gains `trust-record`).
- security-plan · pass · 9 · applied 9 (security-summary :38 :72 :73; rules/security :14 ×2, :16 omission, :17, :37,
  :41).
- test-plan-1 · pass · 5 · applied 5 (tests-summary :21 ×2, :20; rules/testing :34 :33 — the test-owned user home).
- test-plan-2 · pass · 4 · applied 3, declined 1: `verification-harness.md:42#2` — its rewrite «every other home lives
  under …» reads the trust-start folders as test homes, which they are not (they are the spawn folders under `/tmp`);
  the fact stands in the master (test-plan decision/trust-start-folders-outside-repository) and in
  `rules/testing.md`:33.
- test-plan+obs-plan · pass · 1 · applied 1 (rules/observability:38).
- unmapped: `.claude/rules/host-linux.md` (the host's own leaf, no source states it) · `.claude/rules/testing-src.md`
  (pointers into testing.md only) — no step-3 edit.
- Curation homes: no sweep row landed in a `USER:*` block or a `## Session Additions` section (curation 0/0).
