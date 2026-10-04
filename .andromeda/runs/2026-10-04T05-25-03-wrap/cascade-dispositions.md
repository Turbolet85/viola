# Cascade dispositions — 2026-10-04-readiness-gate-and-timing-constants

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (baseline 5bcb0a0c, the pre-CI parent), 11 patterns, each
control fired on the pre-pass masters: `spine-s` (`SPINE_DEADLINE_S`) · `spine-750` (the hook's 750 ms named
`SPINE_DEADLINE`) · `spine-prov` (a provisional `1.0 s` spine bound) · `spine-naming` (naming the constant "stays open")
· `pump-thread` (vt100 on the pump thread) · `open-items` (quiet period / maximum wait / fallback window as open items)
· `sig-with-gate` (signature rows landing with the readiness gate) · `not-proc-panic` (the vt100 panic "not a process
panic" — the retired claim that no panic line is written) · `vt100-arrive` (vt100 as a not-yet-arrived dependency) ·
`fuzz-3list` (a three-target fuzz list) · `term-indep` (the half-removed class).
Sections read beside the sweep: obs-plan §7 (Panic hooks, Error classes captured), §10 panic budget (:1089, :1100) and
G2 (:1020-1034) — standing, `:74`'s G2 question; security-plan :245, :354, :588, :591 — standing true.

## Rows
| row | disposition |
|---|---|
| `.andromeda/architecture.md:102` spine-naming standing | no change — "naming its executable after `--`": a true claim sharing the token |
| `.andromeda/test-plan.md:524` spine-naming standing | no change — "each assertion naming its label": unrelated |
| `.claude/docs/stack.md:14` pump-thread leaf | re-derived from architecture §Stack Screen model row (feed thread) |
| `registries/contracts/architecture/project-directory-structure.md:68` fuzz-3list new | this pass's own text (`{…,hook_stdin,vt100_feed}`) — current truth |
| `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:15` term-indep standing edited | amended this pass (R2): the `terminate`-independent count stands; its cause and fix are added |

Zero rows for `spine-s`, `spine-750`, `spine-prov`, `open-items`, `sig-with-gate`, `not-proc-panic`, `vt100-arrive`
after the pass (each control fired on the pre-pass text, so the zero is a statement about these patterns).

## Leaves re-derived (step 3)
Enumerated by the changed masters (architecture + its two keyed contracts, security-plan, obs-plan, test-plan + its two
keyed contracts) and a leaf grep for the amended facts (`SPINE_DEADLINE|750 ?ms|vt100|fuzz_targets|hook_stdin|readiness
gate|quiet period|maximum wait|confirmation window|Clock|pump thread|maintenance|half-removed` over CLAUDE.md,
`.claude/docs/**`, `.claude/rules/*.md`):
- `.claude/docs/stack.md:14` — Screen model row (feed thread).
- `.claude/docs/tests-summary.md:44` — perf budget reads `viola_core::SPINE_DEADLINE`; hook 750 ms = `CONNECT_DEADLINE`.
- `.claude/docs/tests-summary.md:61` — "arch requests pending" drops the two named constants.
- `.claude/docs/services/viola-core.md:6` — `SPINE_DEADLINE`, `Clock` / `SystemClock`.
- `.claude/docs/services/viola-agent-claude.md:11` — vt100 `=0.16.2` as landed (the `screen` module, provisional consts); "Planned: vt100" retired.
- `.claude/docs/services/viola.md:46` — perf bound named.
- `CLAUDE.md` `GENERATED:setup:modules` — viola-core (`SPINE_DEADLINE`, `Clock`), viola-agent-claude (screen model), viola (feed thread).
Hits left as true: `gotchas.md:79` (the gate's design), `rules/observability.md:36` (`catch_unwind` … and the vt100
feed), `rules/verification-harness.md:42` (`--vt100-panic-bytes` lands with its consumer), `rules/testing.md:18/:29`
(`Clock` seam), `services/viola.md:6/:37` ("readiness-gate wiring", `src/run/`), `services/viola-agent-claude.md:42`.
`obs-summary.md` and `security-summary.md` carry none of the amended facts (grep 0). Curation homes and judgment bases:
0 rows for every pattern.
