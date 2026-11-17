# Cascade dispositions — 2026-11-17-first-readings-on-the-standing-rig

The search: one `cascade.py sweep` call (pass mode, baseline 88e310ba) over `cascade-patterns-security-plan-1.toml`
(`security1-1` fixed `sha256` — the term the old `decision/live-rig-isolated-child` rule held, control its r1) and main's
`cascade-patterns.toml` (`main-keys-stop` — the retired keys-row phrasing `stop's alone|anew for each chunk|anew per
chunk|Ctrl-C is its stop`; `main-stamp` — the retired witness `prefix and mtime|sha256 prefix|before and after each
start|stamp-only`). Part read whole: `sweep-pass-2.1.txt` (14 lines). Counts copied: `total (3 patterns) · 9 rows over
5 files` · `per class · new 1/1 · standing 4/2 · leaf 4/2 · curation 0/0 · base 0/0`.

An earlier sweep of the tool's file alone (`sweep-pass-1.1.txt`, 6 rows) is superseded by this one call, which holds
its rows.

The architecture landing (tx 2426) wrote no pattern file: `patterns: none: 2 set(s) and retire(s) changed prose alone`
— step 3's leaves read its rows' delta.

- `.andromeda/architecture.md:18` dec-vx3wc35b (content-hash-sha256) — no change: a true claim sharing the token.
- `.andromeda/architecture.md:993` dec-v5dvgnaa — no change: a tool's published sha256, true, sharing the token.
- `.andromeda/test-plan.md:666` dec-hbvsiznr ×2 — no change: the SHA-256 vector test, true, sharing the token.
- `.andromeda/test-plan.md:1301` dec-eifr7tk6 — no change: an install action's published sha256, true.
- `.andromeda/security-plan.md:123` dec-wpnve6qm — amended: this pass's own new text («anew per chunk binds only the
  … argv grant»), the intended wording; no duplicate of the retired «stop's alone» on the line.
- `.claude/docs/commands.md:11` leaf — no change: ripgrep's sha256 verification, true.
- `.claude/docs/security-summary.md:73` leaf — re-derived in step 3 (`security1-1`, `main-keys-stop` ×2, `main-stamp`
  ×2): it states the old keys rule and the stamp-only witness.
No curation-home or base row.

## Leaves
Set call: pass mode, baseline 88e310ba, `total · pass · 6 group(s) · 19 leaves · 298585 B · unmapped 2`.
- architecture-1 · pass · findings 2 · applied 2 (conventions.md:54, services/viola-agent-claude.md:6 — the test-only rig exception) · by hand 0 · declined 0
- architecture-2 · pass · findings 0
- architecture-3 · pass · findings 0
- architecture-4 · pass · findings 0
- architecture+security-plan · pass · findings 2 · applied 2 (CLAUDE.md:43 warnings, CLAUDE.md:25 module row) · by hand 0 · declined 0
- security-plan · pass · findings 3 · applied 3 (security-summary.md:73 — the keys stop, the mark witness, the capture) · by hand 0 · declined 0
- unmapped `.claude/rules/host-linux.md` (host) — declined: a host leaf, no master source.
- unmapped `.claude/rules/testing-src.md` — declined: a pointer file into testing.md, whose source (test-plan) this pass did not change.
Apply: `total · blocks 7 · applied 7`.
