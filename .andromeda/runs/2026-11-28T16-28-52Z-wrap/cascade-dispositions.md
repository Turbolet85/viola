# Cascade dispositions — 2026-11-28-console-actions wrap

The sweep: `cascade.py sweep --patterns-file cascade-patterns-security-plan-1.toml --patterns-file cascade-patterns.toml`
(baseline a07542e9), one call after the last row TX (5476). `total (5 patterns) · 14 rows over 7 files` · `per class ·
new 0/0 · standing 3/1 · leaf 11/6 · curation 0/0 · base 0/0` (part `sweep-pass-1.1.txt`, read whole).
Patterns: `security1-1` (the tool's: `0x03`, the old decoder row's term); main's own — `own-qquit` (the frame floor's
«`q quit` on the last row»), `own-nostrip` («`console` act/name no strip»), `own-ascii` (the console's «plain ASCII with
no colour»), `own-unbound` («anything else unbound», «other than `q` and Ctrl-C»). Not looked for: the console's other
behaviours stated in other words (the leaves comparers read each changed row's delta for those). The six other masters'
landings changed prose alone and wrote no pattern file (their dry runs' `patterns: none` lines).

## Rows
- layout-templates.md:10 own-nostrip standing dec-lz4ax6x3 (edited) — no change: the landed row keeps «`console` name no strip» (it takes no strip argument) and adds that it runs the strip verbs on its selected row; true.
- layout-templates.md:222 own-ascii standing ctr-m3alcd57 — no change: `viola verify`'s output, a true claim sharing the token.
- layout-templates.md:248 own-ascii standing dec-vvpcu2np (edited) — amended this pass: its except now names the editing line.
- .claude/rules/api.md:38 security1-1 · own-unbound leaf — stale (one closed class a read); to step 3.
- .claude/rules/security.md:25 security1-1 · own-unbound leaf — stale, the same; to step 3.
- .claude/docs/security-summary.md:16 security1-1 · own-unbound leaf — stale, the same; to step 3.
- .claude/rules/frontend.md:8 own-qquit · own-ascii leaf — stale (`q quit` on the last row; plain ASCII with no exception); to step 3.
- .claude/docs/design-summary.md:49 own-qquit · own-ascii leaf — stale, the same; to step 3.
- .claude/docs/commands.md:23 own-ascii leaf — stale (plain ASCII with no exception); to step 3.

## Leaves
`cascade.py leaves` (pass mode, baseline a07542e9; epoch «Epoch 6 — The fleet console» open): 17 groups · 27 leaves, each
group one comparer of one batch (`fanout.py check`: 17 of 17 pass). `total · blocks 28 · applied 28` — every finding
read whole against its source row and applied by `leaves --apply`; by hand 0; declined 0. Per group: a11y-plan 4 applied
(a11y.md) · architecture-3 1 (services/viola.md) · architecture+design-system+layout-templates 4 (frontend.md) ·
architecture+security-plan 2 (api.md) · architecture+test-plan+obs-plan 1 (commands.md) · design-system+layout-templates
6 (design-summary.md) · masters 2 (CLAUDE.md: the NEVER-log floor's console line — no key byte reaches a log line, the
editing echo on its own screen, his Q9 — and the stack's unicode-width) · obs-plan 2 (obs-summary.md) · security-plan-1 1
(security-summary.md) · security-plan-2 1 (security.md) · test-plan-1 3 (tests-summary.md, testing.md) · test-plan+obs-plan
1 (observability.md); architecture-1, architecture-2, architecture-4, security-plan+obs-plan, test-plan-2 0 findings.
Every step-2 `leaf` row (api.md:38, security.md:25, security-summary.md:16, frontend.md:8, design-summary.md:49,
commands.md:23) is answered by an applied finding on its line.
Unmapped: `.claude/rules/host-linux.md` — the host leaf, a statement no master states, no comparer's; `.claude/rules/testing-src.md`
— scoped to `src/` tests, no source of it changed by this pass: not compared, nothing owed.
Binds: test-plan §3 ↔ obs-plan §6 — obs-plan adds `console-action` and the console-key `count`, test-plan
`decision/harness-log-fields-owned-by-test-plan` lets obs-plan add fields with no entry; none renames or removes one:
consistent.
