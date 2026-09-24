# Scope — 2026-09-24-epoch-1-cleanup

**Working entry (verbatim title + hint):** Epoch 1 cleanup — MAX_FRAME value witnessed by a test, harness and fake-agent functions within the
cognitive ceiling, run.rs under 800 lines, test clone pairs gone

**Epoch:** Epoch 2 — Windows slice I: wrapper, events, ledger (its first entry). This is an epoch-boundary cleanup chunk that the operator
inserted at the 0-pending wrap (`runs/2026-09-24T18-00-38-wrap/adaptation-record.md`). Its subject is Epoch 1's code, as measured by the
Epoch 1 code audit (`runs/2026-09-24T17-47-08-code-audit/proposals.md` B1/B4).

_Premise closure ran at P3 (research.md). Every `[inferred]` tag is now either dropped (verified) or replaced by
`[premise-corrected: …]`._

## What this chunk builds

A behaviour-preserving cleanup of the four Epoch 1 problem spots the audit measured at a28f696. It adds no product capability.

1. **MAX_FRAME pinned.** `crates/viola-core/src/lib.rs:9` `pub const MAX_FRAME: u64 = 16 * 1024 * 1024` gets a test that witnesses its
   value (16 MiB), so the 4 surviving mutants (`:9:31`, `:9:38` × {`*`→`+`, `*`→`/`}) are caught. The audit's proposal is a `viola-core`
   unit test (`assert_eq!(MAX_FRAME, 16 << 20)`). The security-plan's "`Read::take(MAX_FRAME)` (16 MiB)" invariant becomes mechanically
   witnessed.
   - The sole consumer is `src/obs.rs:195` (`file.take(MAX_FRAME)` in `read_diagnostics_level`), the arch `config.json` "read with a
     `MAX_FRAME` cap" row. A boundary witness there is achievable. The equality: a `config.json` of 16 MiB of leading whitespace followed
     by `{"v":1,"diagnostics_level":"debug"}` yields `(INFO, Some(Malformed))` under the cap and `(DEBUG, None)` without it. A file of
     exactly 16 MiB ending in that object yields `(DEBUG, None)`. It is a second guard test on the real guard. It cannot count toward the
     viola-core mutants, because cargo-mutants tests each mutant with its own package's tests only (test-plan §3 `run` step 4).
     [premise-corrected: verified achievable at obs.rs:190–200; it is an additional witness, not a substitute for the viola-core pin]
2. **Cognitive ceiling (15) held by every function in the harness and fake agent.** Re-measured at HEAD 9df9e45 (rust-code-analysis
   0.0.25):
   - `run_with`, `crates/viola-e2e/src/harness/run.rs:110`: 24
   - `main`, `crates/viola-e2e/src/bin/viola-harness.rs:107`: 19
   - `main`, `src/bin/viola-fake-agent.rs:419`: 16

   Each is brought to ≤ 15 by extracting helpers. Behaviour is unchanged: the harness's JSON documents (key order included), the typed
   exits and the fake agent's receipts and script semantics stay identical.
   - The extracted helpers stay ≤ 15 too, so the population ends with 0 functions over 15 (`scan_file` at 15 is at the ceiling, not over).
3. **`run.rs` under 800 lines.** `crates/viola-e2e/src/harness/run.rs` is 1 562 tokei code lines (1 728 physical). About 60 % of it is
   the inline test module (physical :734–1728). It is split by concern into submodules, with no behaviour change. The five names reached
   from outside (`Selection` + `from_flags`, `run_with`, `run_forwarding`, `COVERAGE_FLOORS`, `leg_verdict_path`) stay at
   `viola_e2e::harness::run::`.
   - "800 lines" is the audit's metric: tokei `code` lines (B4 `sizes`), with physical lines reported beside it. No file the split
     creates or leaves exceeds 800 either.
   - [premise-corrected: the split must move PRODUCTION code with its tests. An out-of-line `#[cfg(test)] mod x;` file is an orphan to the
     orphans gate (cargo-modules 0.27.0 probe, research.md), and inline tests do not shrink the file. So a test-only move is not available,
     and moved tests stay inline in each new submodule file.]
4. **Test clone pairs gone.** Removed by extracting shared test helpers:
   - `tests/cli_fake_agent.rs`: 5 internal pairs (379↔406 16 L, 245↔281 13 L, 280↔333 12 L, 243↔356 11 L, 245↔381 9 L).
   - `src/main.rs`↔`src/obs.rs`, inside their `#[cfg(test)]` modules: the panic-line asserts (`main.rs:218`↔`obs.rs:521`, 8 L) and the
     diag-detail schema-validator loading (`main.rs:293`↔`obs.rs:662`, 9 L).
   - **Kept, by operator decision:** the `LOG_FORMAT_EVENTS` clone (`crates/viola-core/src/obs.rs:175`↔`tests/contract_diag_schema.rs:18`,
     21 L). It is a deliberate independent copy.
   - [premise-corrected: jscpd at HEAD finds two more pairs of the same helper class in the same two files, `tests/cli_fake_agent.rs:138↔380`
     (6 L) and `src/obs.rs:516↔645` (6 L, test module). They go with the same extraction. The other audit pairs lie in files outside the
     entry and stay out: `tests/contract_diag_schema.rs:171↔tests/run_cli.rs:21`, `harness/boot.rs:350↔417`, `harness/gate.rs:434↔447`,
     `schema_check.rs:158↔secret_scan.rs:267`, `secret_scan.rs:196↔tests/scan_patterns.rs:10`,
     `tests/harness_lifecycle.rs:96↔125`. No new clone pair may appear.]

## Folded freight (working-route.md:28)

- **PREREQ: close Rust gate deferral** (deferred since `2026-09-24-workspace-tree-and-code-graph-planes`). That chunk's /implement deferred
  `cargo fmt --all --check`, `cargo clippy …` and `bash scripts/agent-run.sh run --unit` on zero `.rs` delta (its report.md:149). This is the
  first Rust-delta chunk since, so those gates run cold and green here, with no `defer` key.
- **CARRY (measured coordinates at a28f696):** folded into items 1–4 as hypotheses and re-derived at HEAD 9df9e45 (research.md §Measured
  facts). Cognitive 24/19/16, run.rs 1 562 code lines, the 7 named clone pairs and 154 run.rs mutants are all equal at HEAD. The cost fact
  follows from `--in-diff` (mechanism re-derived, research.md): moved production lines land in new files as wholly added lines, so up to
  run.rs's 154 viola-e2e mutants enter scope, per CI mutation leg (windows + ubuntu). The adaptation record estimates 50+ min per leg; the
  CI job timeout is 360 min.
- **CARRY (wrap obligation, not build work):** this chunk's wrap P2 proposes a `playbook.md` rule keeping obs-plan §1 (the verbatim obs-scope
  copy, `obs-plan.md:485`) out of the cascade sweep. The cascade edited it twice in Epoch 1 (`runs/2026-09-24T17-37-22-evolve-diagnose/proposals.md`
  P9). /implement writes nothing for it; the plan records it so the wrap finds it.

## Operator directions for this run (2026-09-24, overseer)

- **Tests with mutations (founder rule).** Every new guard test, starting with the MAX_FRAME pin, carries its own remove-the-guard mutation
  run: the guard is mutated or removed, the test is shown to fail, then it is restored. The run.rs split and the refactors must leave
  **0 missed mutants in scope**, meaning the `--in-diff` mutants of this chunk's diff, on both legs.
  - [premise-corrected: "never baselined" is too strong. Every production line of run.rs came from a commit whose CI `mutants` check is
    green (966b7aa, 61f9676, 3f385dd; b0236ca's red was superseded within its chunk), so its mutants were once caught at authoring.
    Measured by blame and check-runs, research.md. The split re-tests all of them against today's tests. Survivors are possible but not
    expected, and any that appear are this chunk's to kill with new tests.]
- **Fold, do not carry.** Any red found in this chunk is fixed in this chunk. That covers a gate, a surviving mutant, a CI leg, and a
  pre-existing defect the cleanup exposes.
- **Host cargo-nextest** was re-measured at 0.9.146, which matches the CI pin (`cargo nextest --version`, 2026-09-24). The handoff's 0.9.133
  line is stale and no install is owed.
- **CI on 9df9e45** (run 36038410173): verdict not yet available at Setup; 14/15 success at P3's read; **completed `success`** at the
  P5 re-read. There is no red to disposition and nothing to fold.

## Boundaries (out of scope)

- Adaptation item 4 (`arbitrary` in `fuzz/Cargo.toml`) and item 5 (host nextest) are out, by operator decision.
- No product behaviour change and no spec-source amendment is expected. The chunk touches test tooling, one `viola-core` test, root-crate
  test modules and `tests/`.
- cargo-mutants 27.1.0 generates no mutant inside a `#[cfg(test)]` module (verified: `--list` over run.rs yields 154 mutants at :44–721 and
  none at or after :734). So the clone extraction inside test modules and in `tests/` adds no mutants. Extracted PRODUCTION helpers in
  `viola-e2e`, the harness binary and the fake agent do.
- `harness/logs.rs` and `harness/status.rs` are outside the modify set, so the `logs`/`status` output is unchanged by construction.
- `run --browser` stays a usage error (exit 2). It is unbuilt at HEAD, and `cli.rs:60` asserts that.
- The code-metrics ledger correction (the `mutation.survivors` columns) rides the next ledger-mode code-audit record, not this chunk.

## Surfaces and contracts touched

- `viola-core` public const `MAX_FRAME`: value pinned, not changed.
- The `viola-e2e` harness module tree (`harness/run.rs` → `harness/run/*.rs` submodules) and `bin/viola-harness.rs`.
- `src/bin/viola-fake-agent.rs` (`main` decomposition; the `fake-agent` feature).
- Test modules in `src/main.rs` and `src/obs.rs`, and `tests/cli_fake_agent.rs`.
- The test-plan §3 harness contract (5 commands, one JSON document plus a typed exit per command) holds unchanged.
