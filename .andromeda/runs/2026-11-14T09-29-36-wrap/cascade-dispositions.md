# Cascade dispositions — 2026-11-14-hyprland-seam (wrap 2026-11-14T09-29-36)

## Step 2 — the sweep
- The landings' patterns files: none — every `rows-land` dry run printed `patterns: none` (its sets changed prose
  alone, or it set and retired no row): architecture (tx 1070, 1071), security-plan (tx 1072, 1073), test-plan
  (tx 1074), obs-plan (tx 1075). No file superseded.
- Main's own patterns (`cascade-patterns.toml`, the amended claims' old wording): `main-subject` (the subject list
  without hyprctl-probe), `main-parser` (the parser list without hyprctl-json), `main-recorder` (the recorder row
  naming viola-mcp-tap alone), `main-rootbin` («root bin depends on all members»).
- `cascade.py sweep` (baseline 4255b3ed): `total (4 patterns) · 1 rows over 1 files` · `per class · new 0/0 · standing
  0/0 · leaf 1/1 · curation 0/0 · base 0/0` — part `sweep-pass-1.1.txt`, read whole.
  - `main-subject`, `main-parser`, `main-recorder` · 0 rows (each control fired on the pre-pass master) — no master,
    leaf, curation home or base still states the old list.
  - `.claude/docs/conventions.md:54 main-rootbin leaf` — «the root bin depends on all members.» — stale: re-derived at
    step 3 (its source `architecture` changed).
- Lateral binds: test-plan §3 ↔ obs-plan §3 untouched (no harness command changed); test-plan §3 Log format ↔ obs-plan
  §6 — the two new obs words (`hyprctl-probe`, `hyprctl-json`) are obs §6 additions, no renaming; a11y schema untouched.
- Curation homes and judgment bases: 0 rows.

## Leaves
`cascade.py leaves` (pass mode, baseline 4255b3ed): `total · pass · 12 group(s) · 24 leaves · 395461 B · unmapped 2`;
one comparer per group, batch `leaf` 12 of 12 pass. `leaves --apply` (all twelve files): `total · blocks 20 · applied 20`.
- architecture-1 · pass · findings 3 · applied 3 (conventions.md:22 omission, :54 stale, :54 omission) · by hand 0 · declined 0
- architecture-2 · pass · findings 0
- architecture-3 · pass · findings 1 · applied 1 (services/viola.md:11 stale)
- architecture-4 · pass · findings 1 · applied 1 (rules/events.md:36 omission)
- architecture+security-plan · pass · findings 0
- architecture+security-plan+test-plan+obs-plan · pass · findings 7 · applied 7 (CLAUDE.md :13 :15 :22 :27 :36 :87 omission, :28 retired)
- architecture+test-plan+obs-plan · pass · findings 0
- security-plan · pass · findings 1 · applied 1 (rules/security.md:22 omission) · by hand 1: rules/security.md:31 «over the
  four crates of `scripts/sync-crates.txt`» → «five crates» (the pre-pass sweep's `sync-crates` leaf row, sync crates
  4 → 5, which the comparer did not raise), drafted whole and landed by `rec.py harness land` (tx 1085)
- security-plan+obs-plan · pass · findings 0
- test-plan · pass · findings 5 · applied 5 (tests-summary.md:21 ×2, :51 omission; rules/testing.md:35 stale; rules/verification-harness.md:38 omission)
- test-plan+obs-plan · pass · findings 2 · applied 2 (rules/observability.md:42, :28 omission)
- obs-plan · pass · findings 0
- unmapped `.claude/rules/host-linux.md` (the host leaf — states the host's facts, no master's): no comparer
- unmapped `.claude/rules/testing-src.md` (pointers into testing.md only): no comparer

## Leaves — whole mode (this chunk closes Epoch 1)
The pass mode above was the wrong mode for an epoch-closing chunk; the whole mode ran after the light gate, before the
flip. `cascade.py leaves --all`: `total · whole · 15 group(s) · 27 leaves · 418910 B · unmapped 2`. First batch `leaf`:
4 of 15 pass, 11 `[ABSENT]` and 10 `[PATH_CHECK]` stray `M` lines — the pass-mode comparer files of the same names were
not moved aside before the snapshot (main's omission); on the overseer's technical answer the 11 were re-spawned
(`leaf-r1`, their files moved to `.raw-leaf-*.md` by the render), 11 of 11 pass. `leaves --apply` over the 15 whole-mode
files (the pass-mode `leaf-architecture+security-plan+test-plan+obs-plan.md` not among them: its group is no longer
printed): `total · blocks 13 · applied 11 · held 2`.
- applied: rules/events.md:34 stale · rules/frontend.md:24, :25 omission · docs/commands.md:23 omission ·
  docs/security-summary.md:72 ×2 omission · rules/security.md:17 omission · rules/verification-harness.md:42 stale,
  :29 omission · rules/observability.md:35 stale · docs/obs-summary.md:55 stale
- declined: rules/security.md:22#1 — the deadline folded into the validators line; the detail stays in its master row
  (security-plan decision/hyprctl-read-deadline), and the always-loaded file pays for every byte
- declined: rules/security.md:41#1 — the dropped-reply-text fact already stands in CLAUDE.md:36 (the pass-mode apply)
- groups with 0 findings: architecture-1, -2, -3, architecture+security-plan, masters, security-plan+obs-plan,
  design-system+layout-templates, a11y-plan
