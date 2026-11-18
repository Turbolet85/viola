# Cascade dispositions — 2026-11-18-trust-record-row-read-live (wrap run 2026-11-18T09-48-02Z-wrap)

## Step 2 — the sweep
The search: the pattern files of the dry runs each landing followed — `cascade-patterns-test-plan-1.toml` (pattern
`tests1-2`, fixed `verify_window_`, its control `decision/verify-window-60s-kill` r1). The architecture (tx 2656) and
security-plan (tx 2657) dry runs wrote no file: `patterns: none: 8 set(s) and retire(s) changed prose alone` and
`patterns: none: 3 set(s) and retire(s) changed prose alone` — step 3's leaves read each changed row's delta.
Main wrote no pattern file of its own: the amended claims (the first-screen except, the rig seed, the 80 s kill) carry
no retired mechanism wording beyond the 60 s figure the tool's pattern names.

Counts (the sweep's `total` and `per class` lines): `total (1 pattern) · 3 rows over 3 files` · `per class · new 0/0 ·
standing 1/1 · leaf 1/1 · curation 1/1 · base 0/0`; part `sweep-pass-1.1.txt`, read whole.
- `.andromeda/test-plan.md:1312` · standing `dec-5eznx6xr` (decision/verify-window-80s-kill, edited) — no change: the
  heir row itself, stating 80 s.
- `.claude/rules/testing.md:55` · curation — no change: the Session Additions entry names the `verify_window_` prefix
  and no kill figure; preserve-verbatim, nothing stale.
- `.claude/docs/gotchas.md:141` · leaf — re-derived in step 3 (`60 s under mutants` → `80 s`, leaf
  architecture+security-plan+test-plan #1).
Not looked for: the «unverified on the real CLI» wording, which no master or leaf holds (Phase 1's `rec.py find`, 0
hits; a grep of the leaves, 0 hits) — it stands in the route's own frozen line alone, the flip's.

## Leaves
`cascade.py leaves` — pass mode, epoch open (`markerless 7`); `total · pass · 9 group(s) · 23 leaves · 393128 B ·
unmapped 2`. `unmapped .claude/rules/host-linux.md (host)` — the host's own statements, no comparer's; `unmapped
.claude/rules/testing-src.md` — sources none of the changed rows (no test-plan figure it states moved), no change.
No `not compared` row printed.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 2 | 2 | 0 | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| architecture-3 | pass | 1 | 1 | 0 | 0 |
| architecture+security-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan | pass | 3 | 3 | 0 | 0 |
| architecture+test-plan | pass | 1 | 1 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| test-plan-1 | pass | 0 | 0 | 0 | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |

`leaves --apply` (the tool's `total` line): `total · blocks 9 · applied 9`; each leaf written and read back. Each
finding's source side was read at its cited row (dec-uun4jqhm, dec-zypgxbrh, dec-vyrhhge5, dec-5eznx6xr, ctr-vsm4qn3e,
dec-pvviak6k, dec-4yctyxtx), each the landed row of this pass.
