# Cascade dispositions — 2026-11-04-linux-and-macos-parity

## Step 2 — the sweep

The search: ONE `cascade.py sweep` over the apply writers' `@pattern` records (`apply-architecture.md`,
`apply-security-plan.md`, `apply-design-system.md`, `apply-test-plan.md`, `apply-obs-plan.md`, `apply-a11y-plan.md`) and
main's `cascade-patterns.toml`, baseline fdb28e90 (the parent of the oldest pre-CI commit 50e7e094), over the seven
masters (bodies, category sets, key files), the leaves, the three curation homes and the two judgment bases.

- The first call refused (exit 3): `tp-01 · control UNFIRED over the seven masters (bodies and sets) at fdb28e90` — its
  fixed text «`harness_lifecycle.rs`) on `ubuntu-latest`» opens a backtick the master does not hold there (fct-5qxwlgko
  reads «(`crates/viola-e2e/tests/harness_lifecycle.rs`) on `ubuntu-latest`»). Dropped, and controlled by main's
  `main-01` (`harness_lifecycle.rs`) on `ubuntu-latest``), whose control fired.
- The patterns run (9): `arch-01` «reads 28 sites in 20 files» (the WITHIN count, fct-jv5gqhbl and its companion
  fct-hgewdmj2, edits-main) · `tp-02` «`Frozen` (`tests/cli_instance_state.rs`) is a guard» · `tp-03` «its outer-PTY witness
  a CARRY on» (ignore case) · `tp-04` «under the `Frozen` guard (`tests/cli_instance_state.rs`» · `tp-05` «twenty-six
  verify-driven binaries» (ignore case) · `a11y-01` «is proven at the unit tier only; its outer-PTY witness is owed to» ·
  `a11y-02` «is proven at the unit tier only (`board_row_style_…`), its outer-PTY witness a CARRY on» · `a11y-03` «is proven
  at the unit tier only, its outer-PTY witness a CARRY on» · `main-01` (above).
- The listing: `total (9 patterns) · 0 rows over 0 files` · `per class · new 0/0 · standing 0/0 · leaf 0/0 · curation 0/0
  · base 0/0`. Each pattern prints `0 rows (control fired)`: every control fired on the pre-pass text, and the old
  wording of each amended passage stands nowhere after the pass — no other master, key file, leaf, curation home or
  base cites it.
- No row to disposition. What was NOT looked for: the amended claims' other phrasings (the same facts reworded) —
  the leaf comparers of step 3 read every leaf against the changed sources whole; the pre-pass sweep's true hangup
  leaf (`.claude/rules/verification-harness.md:41`, the fake agent's exit ignoring SIGHUP on Unix — true, unchanged).
- The two binds: test-plan §3 ↔ obs-plan §3 and test-plan §3 Log format ↔ obs-plan §6, and the a11y-plan schema ↔ the
  log schema — no harness command, log field or violation schema moved this pass (obs-plan's two rows are §9 artifact
  and pipeline rows); each pair stands consistent.

## Leaves

The set: `cascade.py leaves` (pass mode — Epoch 7 keeps the markerless entry «Windows-only live measurements», so the
chunk closes no epoch), `total · pass · 16 group(s) · 27 leaves · 439885 B · unmapped 2`. Unmapped:
`.claude/rules/host-linux.md` (host) — the host leaf, no master's statement, no comparer's; `.claude/rules/testing-src.md`
— no master named in its provenance, a pointer rule; neither placed. The comparers: one per group, `--batch leaf`, 16 of 16
pass. The decision: `leaves --apply --dry-run` over the 16 comparer files, `total · blocks 8 · applied 8` (all `omission`),
then `--only`; final `total · blocks 8 · applied 5 · already applied 1 · held 2` (the first `--only` call wrote
verification-harness.md alone: a repeated `--only` flag keeps its last id — the five then named after one flag).

- a11y-plan · pass · 1 finding · applied 1 — `.claude/docs/a11y-summary.md:18`: the stale-row dim witness beside the
  socket-directory refusal on the Unix legs (a reader would read the P6 witness owed).
- architecture-1, -2, -4 · pass · 0 findings each.
- architecture-3 · pass · 2 findings · applied 1 · declined 1 — `.claude/docs/workflow.md:48` applied: a new OS gate
  lands with its `GATES` row or the census goes red (a reader adding a gate would act against the source);
  `.claude/docs/workflow.md:11` declined: the per-OS JUnit reading's detail stays in its masters and in
  `.claude/docs/commands.md` (applied below); the workflow line names the CI reads already.
- architecture+design-system · pass · 0 findings.
- architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan · pass · 1 finding · declined 1 — `CLAUDE.md:86`:
  the census clause stays out of the always-loaded tier; `.claude/docs/workflow.md:48` and `.claude/rules/testing.md:19`
  carry it where a reader adding a gate meets it.
- architecture+security-plan · pass · 0 findings.
- architecture+test-plan+obs-plan · pass · 1 finding · applied 1 — `.claude/docs/commands.md:84`: `scripts/junit-os-sets.py`
  joins the command reference beside `gh run download`.
- design-system · obs-plan · security-plan · security-plan+obs-plan · test-plan+obs-plan · pass · 0 findings each.
- test-plan-1 · pass · 2 findings · applied 2 — `.claude/rules/testing.md:18` (the one freeze guard `Frozen` in
  `frozen.rs` among the shared helpers: a root test file defines no copy) and `:19` (the `contract_` binary
  `tests/contract_os_gated_tests.rs` in the E2E layer).
- test-plan-2 · pass · 1 finding · applied 1 — `.claude/rules/verification-harness.md:41`: the SIGHUP-ignoring exit is
  witnessed on Linux only, `ps -o sigignore=` having no macOS form (a reader would re-widen the case).
