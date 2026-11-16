# Cascade dispositions — 2026-11-16-send-confirmation (wrap 2026-11-16T05-15-05-wrap)

## Step 2 — the sweep
- Skipped: no pattern file. Every `rows-land` dry run of this pass printed `patterns: none: {n} set(s) and retire(s)
  changed prose alone — no file` (architecture 3 · design-system 1 · layout-templates 1 · obs-plan 2 · security-plan 2
  · test-plan 3); main wrote no `cascade-patterns.toml`. Step 3's leaves read each changed row's delta
  (`leaf-delta-{doc}.diff`, six masters).
- The claim's stale statements across the seven masters were located before the landing by P1's site sweep
  (`cascade-2026-11-16-send-confirmation-pre-pass.json`, part `sweep-pre-pass-1.1.txt`: 20 rows over 8 files, standing
  15/3 · leaf 5/5). Each master row of the exact-match claim was changed this pass: architecture dec-xqxmtajo retired
  (heir decision/submitted-form-prompt-match), ctr-yg26p6cv and ctr-u4sxg2hi set; security-plan dec-ohzj3ett retired
  (heir decision/delivery-confirmation-trailing-tolerance), dec-piageym2 set; test-plan dec-fifv75b4 retired (heir
  decision/send-confirmation-trailing-trim-match). No change, a true claim sharing the token (a local command's own
  exact match, unchanged): architecture dec-xqby3ost; test-plan ctr-kf6wh5yt, dec-dkrk3kye. No change, history true
  as written: architecture dec-xqc6szcx (typed-text normalisation's why). No change: ctr-yecqt6py, ctr-u3uf5uxc
  (`origin` set only by the relabel, still true). Leaf rows: `.claude/docs/gotchas.md:84` — no change, the
  local-command match (read whole); `.claude/rules/security.md:24`, `.claude/docs/security-summary.md:61`,
  `.claude/rules/events.md:20`, `.claude/docs/services/viola.md:41` — re-derived in step 3 (below).
- Curation homes and judgment bases: 0 rows (`per class · curation 0/0 · base 0/0`).

## Leaves
`cascade.py leaves` pass mode, baseline 86fc64d0, `--add` the five leaves the site sweep named: `total · pass · 14
group(s) · 25 leaves · 395932 B · unmapped 2`. Unmapped: `.claude/rules/host-linux.md` (host — no source states it,
no comparer's); `.claude/rules/testing-src.md` (no changed master is its source; no row of this pass bears on it).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 1 | 1 | 0 | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| architecture-3 | pass | 2 | 2 | 0 | 0 |
| architecture-4 | pass | 1 | 1 | 0 | 0 |
| architecture+security-plan | pass | 1 | 1 | 0 | 0 |
| architecture+…+obs-plan (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+design-system+layout-templates | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| security-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| design-system+layout-templates | pass | 1 | 1 | 0 | 0 |
| test-plan | pass | 2 | 2 | 0 | 0 |
| test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 0 | 0 | 0 | 0 |

Applied: services/viola-agent-claude.md:25 · services/viola.md:41, :27 · rules/events.md:20 · rules/api.md:22 ·
docs/design-summary.md:7 · docs/security-summary.md:61 · rules/security.md:24 · docs/tests-summary.md:26 ·
rules/verification-harness.md:41 — each finding's source row checked at its handle (the rows read back at TX
1829–1834). Tool total: `total · blocks 10 · applied 10`.
