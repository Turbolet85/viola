# Cascade dispositions — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## Sweep (Cascade step 2)
The search: `cascade.py sweep --patterns-file apply-architecture.md --patterns-file apply-security-plan.md
--patterns-file apply-test-plan.md --patterns-file cascade-patterns.toml` (baseline `b1249fb1`, the parent of the
oldest pre-CI commit 948f7f01), every pattern's control fired. The patterns: the writers' `arch-01` · `arch-02` ·
`sec-01` (the re-spawned test-plan writer listed none), and main's own `main-rustsec` (`RUSTSEC-2026-0386`),
`main-owner` (the owner title «tokio RUSTSEC-2026-0386 lock bump»), `main-ver` (`1\.53\.1` and the `tokio … 1.53.1`
spellings — the claim's version token over all seven masters, their category files and every registry file) and
`main-openadv` (the retired line's verbs: «open advisory», «advisory … fails … supply-chain»). The sections read: the
seven masters' bodies and category sets, `.andromeda/registries/**`, the leaves, the curation homes and the two
judgment bases, as the tool sweeps them.

Counts (the listing's lines): `total (7 patterns) · 4 rows over 2 files` · `per class · new 0/0 · standing 0/0 · leaf 4/2 · curation 0/0 · base 0/0`.

- Masters and registries: 0 rows — no master, category file or key file still states tokio 1.53.1, the open
  advisory, its owner title or its failing-supply-chain wording after this pass (the eight `tokio-ver` sites and the
  retired line of the P1 site sweep all amended).
- `.claude/docs/stack.md:10` (arch-02, main-ver) · leaf — re-derived in step 3 (the Backend framework row's «Tokio 1.53.1 built only inside `mcp`»).
- `.claude/docs/stack.md:47` (main-ver) · leaf — re-derived in step 3 (the e2e dev-deps' «tokio 1.53.1 (`rt` + `process`»).
- `.claude/docs/services/viola-mcp.md:11` (main-ver) · leaf — re-derived in step 3 (viola-mcp's «tokio 1.53.1 (`rt`)»).
- Curation homes and judgment bases: 0 rows — no P3 or propose→approve routing.

Not looked for: other tokio facts that did not move (its features, its containment) — the report's Crates / modules
bullet states them unchanged.

## Leaves
`cascade.py leaves --add .claude/docs/stack.md --add .claude/docs/services/viola-mcp.md` (pass mode, baseline
`b1249fb1`): `total · pass · 9 group(s) · 23 leaves · 325611 B · unmapped 2`. One comparer per group (batch leaf, 9 of
9 pass); `leaves --apply --only …` → `total · blocks 3 · applied 3`.

- architecture-1 · pass · 0 findings · 0 applied · 0 by hand · 0 declined
- architecture-2 · pass · 1 finding · 1 applied (`.claude/docs/services/viola-mcp.md:11` stale «tokio 1.53.1 (`rt`)» → 1.53.3) · 0 by hand · 0 declined
- architecture-3 · pass · 2 findings · 2 applied (`.claude/docs/stack.md:10` «Tokio 1.53.1 built only inside `mcp`» → 1.53.3; `.claude/docs/stack.md:47` «tokio 1.53.1 (`rt` + `process`» → 1.53.3) · 0 by hand · 0 declined
- architecture+security-plan · pass · 0 findings
- architecture+security-plan+test-plan (CLAUDE.md) · pass · 0 findings
- architecture+test-plan · pass · 0 findings
- security-plan · pass · 0 findings
- test-plan-1 · pass · 0 findings
- test-plan-2 · pass · 0 findings
- unmapped `.claude/rules/host-linux.md` (host) — the host's own facts, no source's; no step-3 edit.
- unmapped `.claude/rules/testing-src.md` — read by main: `grep -n -i -E 'tokio|1\.53|rustsec|supply-chain'` 0 hits; no change.

After the write, `grep -rn '1\.53\.1' .claude/docs CLAUDE.md .claude/rules` → 0 hits.
