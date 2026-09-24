
## 2026-09-24-quality-gates — per-job gate, coverage JUnit and regex, two-leg mutation union, seeded fuzz replay, rustup MSRV
**Section:** §2 (Property-based row) · §3 (preamble usage details; `run` body, `--coverage`, `--fuzz-replay`, exit semantics, Output format; `gate`; Bootstrap `ci-tool-install`; Closed enums) · §6 (Property suite) · §9 (Coverage report, Mutation, MSRV, Fuzz replay and Quality gates rows; tool install paragraph; Matrix builds; Test report format) · §10 (Stack adjustments; Mutation gate) · §11 (CI ×2) · §12 (new entry)
**Change:**
- The coverage JUnit source is `target/nextest/ci/junit.xml`, not `target/llvm-cov-target/…`.
- The coverage ignore regex is separator-agnostic (`crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`, harness `COVERAGE_IGNORE`). This is not a widening.
- `run` gains `--leg` (per-leg `mutants-verdict-<leg>.json`, survivors deferred to the gate), a `detail` field, the llvm-cov failure codes, and `--fuzz-replay` on the `fuzz/rust-toolchain.toml` channel (`fuzz-linux-only` off Linux; `tool-missing`, `corpus-empty`).
- `gate` gains `--mutants-legs`, the union rule, fixed `detail` codes and usage `unknown-suite` / `invalid-leg`. The mutants legs defer their gate to `mutants-verdict`.
- §9: mutation is a two-leg matrix plus `mutants-verdict`; MSRV and fuzz use rustup (not dtolnay); `mutants.out/` is not uploaded; the fuzz-replay job and the nightly fuzz job are recorded.
- §6 / §2: `viola_name` is the eighth, pre-parser seed target in the separate `fuzz/` workspace.
- §12: one dated entry (union verdict, seed target, declined `concurrency:`, rustup toolchains, new closed values).
**Why:** chunk 2026-09-24-quality-gates. Research M6 (JUnit path) and M5 (the Windows regex) are measured; the cargo-mutants `#[cfg]` limitation explains CI run 36005608858's `file_mode` misses. Operator P4 decisions: the windows leg with a union verdict, and seeding the fuzz pipeline now. P5-approved leans: rustup, no `concurrency:`, `mutants.out/` removed.
**Sweep** (all 7 masters + CLAUDE.md, `.claude/rules/*`, `.claude/docs/**`, playbook, drift-base; control `grep -c dtolnay` on a planted line → 1):
- `dtolnay` 0 · `cargo +nightly fuzz` 0 · `runs only the weekly` 0 · `the integration pass` 0 · `single jobs by design` 0 · the fixed string `crates/viola-e2e|tests/support` 0 in masters.
- `llvm-cov-target` 1: this entry's own `:647` "never exists".
- `mutants\.out` / `outcomes\.json`: 14 master hits, all the local `run --mutants` verdict source (`:441, :556, :557, :565, :649, :1495, :1608, :1652, :1781, :1788`), all still true, no change; `:1457` and `:1807` are this pass's text.
- Leaves re-derived: `.claude/docs/commands.md` (run/gate/built-today lines, llvm-cov command, fuzz replay command, deny scope), `.claude/docs/tests-summary.md` (Mutation row), `.claude/docs/workflow.md` (PRs line, verdict sources), `.claude/rules/testing.md` (Running tests line), `.claude/rules/verification-harness.md` (`--leg`, runner seam).
