# Cascade dispositions — wrap 2026-11-17T14-02-45Z of 2026-11-17-supply-chain-green-and-booted-tests-end

Sweep: `sweep-pass-1.1.txt` over `cascade-patterns-security-plan-1.toml` (rows-land's, the one dry run that wrote a
file) and `cascade-patterns.toml` (main's), fired `--pattern security1-1 --pattern tied-verify-kill --pattern
rmcp-ground-only`; main's `one-ignore` dropped after its control never fired over the masters (its wording stands only
in the `.claude/rules/security.md` leaf, added to the leaf pass by `--add`). No rows-land patterns file superseded.

- `.andromeda/security-plan.md:411` security1-1 new `decision/advisory-ignore-list` — re-derived: the landed heir, current truth (four ids).
- `.claude/rules/security.md:31` security1-1 · rmcp-ground-only leaf — stale: «the advisory ignore list holds exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462 …» and «Never widen an ignore … the one admitted ignore is the Critical CVE response»; the leaf pass rewrites it.
- `.claude/rules/verification-harness.md:30` tied-verify-kill leaf — stale: «tying it and its stamping `viola verify` to the test thread by a parent-death kill»; verify takes SIGTERM, the supervisor SIGKILL; the leaf pass rewrites it.

Total: 4 rows over 3 files · 1 re-derived · 3 stale (2 leaves, one of them hit by two patterns).
