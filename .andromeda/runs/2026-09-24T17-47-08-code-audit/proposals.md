# Code Audit — viola · Epoch 1 — Foundation · 2026-09-24T17:47:08Z
mode baseline · HEAD a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d · baseline none · span —
Overshoot: 0 commits. HEAD is the boundary, the flip of `2026-09-24-workspace-tree-and-code-graph-planes`.

This is the ledger's first record (`.andromeda/code-metrics.ndjson`, 1 record). Every finding below is absolute. **Trend judgments begin at the next boundary.**
Population: 33 tracked `*.rs` files of the root workspace (`source-files.txt`): `src/`, `tests/`, and `crates/viola-core` and `crates/viola-e2e`, each with its `src` and `tests`. `fuzz/` is its own workspace and is excluded. Units: `viola` · `viola-core` · `viola-e2e`.

## Baseline findings

### B1 — mutation · viola-core — 4 survivors, score 86.21 %
**Counts:** 30 mutants · 25 caught · 4 missed · 0 timeout · 1 unviable. Score = caught/(caught+missed) = 25/29 = 86.21 %. Unit state: `complete 30/30` (`outcomes.json` `end_time` set, `total_mutants` 30 = `mutants.json` 30; unmutated baseline `Success`).
**Survivors (all 4 = `counts.missed`):**

| site | mutation |
|---|---|
| crates/viola-core/src/lib.rs:9:38 | replace * with + |
| crates/viola-core/src/lib.rs:9:38 | replace * with / |
| crates/viola-core/src/lib.rs:9:31 | replace * with + |
| crates/viola-core/src/lib.rs:9:31 | replace * with / |

**Suspected shape:** all four survivors mutate the arithmetic in `pub const MAX_FRAME: u64 = 16 * 1024 * 1024`, the 16 MiB cap on every external reader. The run was package-scoped (`-p viola-core`), but a workspace grep finds that `MAX_FRAME`'s only consumer is `src/obs.rs:195` (`file.take(MAX_FRAME)`). No test in any package names the value, so a wider test scope would not catch these either. This fits the handoff: `viola-channel`, the main planned consumer of `MAX_FRAME`, does not exist yet.
**Proposal:** pin the constant's value in a `viola-core` unit test (`assert_eq!(MAX_FRAME, 16 << 20)`), or pin the bound at its first consumer's boundary test once the channel reader exists. Either would make the security-plan's "`Read::take(MAX_FRAME)` (16 MiB)" invariant mechanically witnessed.

### B2 — graph · cycles — 0
The unit graph has 2 edges (`viola → viola-core`, `viola-e2e → viola-core`). There are no cycles, and each unit's fan-out is 1.
Fan-in top 5:
- `viola-e2e harness/Outcome#doc.` 63
- `harness/Workspace#` 63
- `harness/Outcome#code.` 62
- `viola support/home/TestHome#` 43
- `harness/Workspace#root.` 36

The fan-in top 20 is dominated by test-harness types. The first product types are `viola-core obs/ObsProcess#` (30) and `__nutype_ViolaName__/ViolaName#` (29).

### B3 — dead code — 0 candidates
- **Zero-reference symbols:** 272 in total. The `tests/` segment excludes 266 of them (union of symbol path with the SCIP prefix stripped, and file path).
- **The remaining 6 all fall into false-positive classes:**
  - Entry points (3): `main()` in `src/main.rs`, `src/bin/viola-fake-agent.rs` and `crates/viola-e2e/src/bin/viola-harness.rs`.
  - Trait-impl dispatch (2): `Display::fmt` for `ObsEvent` and for `ObsProcess`.
  - Derive/attr-invoked (1): `HarnessError::Json`, a `#[from] serde_json::Error` variant at `harness/mod.rs:27`.
- **Unused dependencies (cargo-machete):** one, `arbitrary` in `fuzz/Cargo.toml`, a separate workspace. The fuzz target `viola_name.rs` names it only in a doc comment. This may be an `arbitrary` feature pulled in through `libfuzzer-sys`, which is a false-positive class for machete. It's a candidate, not a verdict.

### B4 — starting tables
| metric | value |
|---|---|
| sizes | 33 files · 8 602 code lines · per-file p50 177 · p90 578 · max 1 562 · over 800: 1 |
| duplication (jscpd) | 1.42 % · 137 duplicated of 9 679 lines · 16 clones · split: src 6 pairs / 42 L, test 8 / 84 L, mixed 2 / 27 L |
| complexity | 831 functions · cyclomatic p50 1 / p90 4 / max 24 · cognitive p50 0 / p90 2 · over ceiling (cognitive > 15): 3 |
| coverage (line) | 98.21 % (879/895) · functions 96.55 % · regions 98.34 % · branch — (not instrumented) |
| graph | 0 cycles · 2 cross-unit edges |
| dead | 0 candidates · 1 unused-dep candidate (fuzz) |
| mutation | viola-core 86.21 % (25/29) |

**Over the complexity ceiling (all 3):**
- `run_with` at `crates/viola-e2e/src/harness/run.rs:110`, cognitive 24
- `main` at `crates/viola-e2e/src/bin/viola-harness.rs:107`, cognitive 19
- `main` at `src/bin/viola-fake-agent.rs:419`, cognitive 16

All three are in test-side tooling (the harness and the fake agent). The first product-code entries, `feed` in `viola-fake-agent.rs:363` (11) and the `src/obs.rs` functions, sit below the ceiling.

**Largest files (top 10):**

| file | code lines |
|---|---|
| `crates/viola-e2e/src/harness/run.rs` | 1 562 (the only file over 800) |
| `src/obs.rs` | 632 |
| `tests/cli_fake_agent.rs` | 610 |
| `src/bin/viola-fake-agent.rs` | 578 |
| `crates/viola-e2e/src/harness/gate.rs` | 462 |
| `tests/run_cli.rs` | 459 |
| `crates/viola-e2e/src/harness/boot.rs` | 448 |
| `crates/viola-e2e/src/harness/secret_scan.rs` | 435 |
| `crates/viola-e2e/src/harness/mod.rs` | 303 |
| `src/main.rs` | 274 |

**Top clones (all 10):**

| fragment A | fragment B | lines |
|---|---|---|
| `crates/viola-core/src/obs.rs:175` | `tests/contract_diag_schema.rs:18` | 21 |
| `tests/cli_fake_agent.rs:379` | `tests/cli_fake_agent.rs:406` | 16 |
| `tests/cli_fake_agent.rs:245` | `tests/cli_fake_agent.rs:281` | 13 |
| `tests/cli_fake_agent.rs:280` | `tests/cli_fake_agent.rs:333` | 12 |
| `tests/cli_fake_agent.rs:243` | `tests/cli_fake_agent.rs:356` | 11 |
| `tests/contract_diag_schema.rs:171` | `tests/run_cli.rs:21` | 10 |
| `src/main.rs:293` | `src/obs.rs:662` | 9 |
| `tests/cli_fake_agent.rs:245` | `tests/cli_fake_agent.rs:381` | 9 |
| `src/main.rs:218` | `src/obs.rs:521` | 8 |
| `crates/viola-e2e/src/harness/boot.rs:350` | `crates/viola-e2e/src/harness/boot.rs:417` | 7 |

The largest clone is a mixed pair: `contract_diag_schema.rs` repeats 21 lines of `viola-core/src/obs.rs`. Both sites hold `const LOG_FORMAT_EVENTS: [&str; 19]`, the event vocabulary pinned as a literal list: the diagnostics-plane chunk decided to pin both schemas to the literal list. The duplicate is deliberate: the test holds an independent copy so it can check the code against it.

## Informational
- **Coverage population:** the line figure covers `viola` + `viola-core` only. The project's own ignore regex excludes `viola-e2e` (the largest unit, which includes `run.rs` at 1 562 lines), the fake agent, `tests/support` and `fuzz`. Coverage is a trend-only metric, and the next boundary compares against 98.21 %.
- **Where the size and complexity sit:** most of the code volume and complexity is in the test-side harness (`viola-e2e`). The product crates are still thin at this boundary, because the wrapper, channel, state, mcp and ui crates are later epochs. Expect the population to shift as they land. The next record's `totals` names that change as population, not movement.
- **Mutation scope:** only `viola-core` was run, by operator choice. The per-chunk CI mutants union (windows + ubuntu) already covers every diff. A full `viola` (141 mutants) or `viola-e2e` (431) baseline can ride a later boundary.
- **nextest host version:** the host has `0.9.133` and CI pins `0.9.146`. Coverage and mutation ran on the host version, recorded in `tool_versions`.

## Below threshold — no action
- `graph.cycles` 0: the `new-cycle` rule does not fire in baseline mode at 0.
- `dead.zero_ref_candidates` 0.
- Every threshold rule except `new-cycle` needs a baseline: `duplication-up`, `complexity-creep`, `dead-growth`, `coverage-drop`, `mutation-drop`, `monotonic` and `count-under-ratio` are all not evaluable at the first record.

## Corrections (the proposals.md channel: the correction surfaced after the append)
- **2026-09-24, target record `sha a28f69684d32…`, `ts 2026-09-24T17:51:10Z`, field `mutation.survivors`:** the record states each survivor's site as `file:line` (`crates/viola-core/src/lib.rs:9`), so its 4 rows read as 2 duplicated pairs. The true sites are `:9:38` ×2 and `:9:31` ×2, as in the table above and in `c-mutation-viola-core.json`. The count fields (`counts.missed` 4, score 86.21) are correct.
- The next ledger-mode record should carry this as `corrections: [{target_sha: "a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d", field: "mutation.survivors", was: "[[lib.rs:9, replace * with +], [lib.rs:9, replace * with /] ×2]", now: "sites :9:38 and :9:31 × {+, /}", note: "summarizer dropped the column"}]`.

## Skips
- churn — `no-baseline`
- hotspots — `no-baseline` (needs B1's per-file commit counts)
- mutation: viola — `declined` (operator: the CI mutants union covers every diff)
- mutation: viola-e2e — `declined` (same)
- coverage.branch — `declined` (the project's coverage command does not instrument branches)
