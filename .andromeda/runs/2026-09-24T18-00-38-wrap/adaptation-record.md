# Adaptation record — 0-pending wrap · 2026-09-24T18:00:38Z

Path: Setup step 6 no-op (0 pending; the tree held only expected-transient bookkeeping: `friction-log.ndjson`,
`session-handoff.md`, the new `code-metrics.ndjson`, and two untracked run dirs). The conversation carried an
operator ROUTE-ADAPTATION request, so route-resolve (P5) ran under §Operator-requested adaptation.

## Direction
Founder's standing rule: at every epoch boundary, clean the codebase as soon as the code audit shows the problem
spots, as ONE cleanup chunk at the head of the queue before the next epoch's first entry.

Sources:
- `runs/2026-09-24T17-47-08-code-audit/proposals.md`
- `runs/2026-09-24T17-37-22-evolve-diagnose/proposals.md`

Anchor: immediately before "Security prerequisites". The overseer approved "as printed", including the run.rs
split, noting: "the re-tested mutants give viola-e2e its first mutation witness (the audit declined its baseline)".
Dialogue: 2 rounds. The first print did not reach the overseer; it was re-printed as text with (a)/(b)/(c).

## Edit applied (working-route.md, markerless tail only)
- **Inserted** as the first entry under `### Epoch 2 — Windows slice I: wrapper, events, ledger`, followed by
  `↓`, then "Security prerequisites". Placing it above the header would have reopened the already-diagnosed and
  audited Epoch 1.
- **Entry:** "Epoch 1 cleanup — MAX_FRAME value witnessed by a test, harness and fake-agent functions within the
  cognitive ceiling, run.rs under 800 lines, test clone pairs gone". It carries:
  - the moved PREREQ;
  - a CARRY with the measured coordinates;
  - a CARRY for its own wrap's P2 playbook proposal.
- **PREREQ moved:** `PREREQ: close Rust gate deferral (deferred since
  2026-09-24-workspace-tree-and-code-graph-planes)` went from Security prerequisites to the new entry, with its
  origin preserved. Two grounds:
  - route-resolve §Operator-requested adaptation: an insertion ahead of the first markerless entry re-pins its
    next-entry PREREQs;
  - measurement: items 1–3 are Rust deltas in `viola-core`, `viola-e2e` and the root crate's tests, so this is
    the first Rust-delta chunk.
- **Diff:** 3 added / 1 removed. LF kept. No frozen line touched.
- **`route.py` after the edit:**
  - `next working-route.md:28 · Epoch 1 cleanup`
  - Epoch 1 still 8/8 complete
  - Epoch 2 now 9 entries
  - 3 freight blocks parsed on line 28; no INDETERMINATE.

## Item dispositions (measured at HEAD a28f696)
| # | Item | Disposition | Basis |
|---|---|---|---|
| 1 | MAX_FRAME 4 survivors (`crates/viola-core/src/lib.rs:9:31`, `:9:38` × {+, /}) | in scope | Sole consumer `src/obs.rs:195` (`file.take(MAX_FRAME)`). No test in any package names 16 MiB. A pinning test adds no mutants (cfg(test) code is skipped). |
| 2 | Cognitive > 15: `run_with` (harness/run.rs:110) 24, `viola-harness` main 19, `viola-fake-agent` main 16; run.rs 1 562 code lines | in scope | All four are test tooling. run.rs is the only file over 800. Cost (`cargo mutants --list` at a28f696), counted below. |
| 3 | Clones: `tests/cli_fake_agent.rs` 5 internal pairs; `src/main.rs`↔`src/obs.rs` 2 pairs (9 + 8 lines) | in scope, except LOG_FORMAT_EVENTS (kept, operator) | The main/obs pairs sit inside `#[cfg(test)]` modules: panic-line asserts (`main.rs:218`↔`obs.rs:521`) and diag-detail schema-validator loading (`main.rs:293`↔`obs.rs:662`). |
| 4 | `arbitrary = "=1.4.2"` flagged unused (fuzz/Cargo.toml:11) | out of scope | **Measured:** the fuzz target names `arbitrary` only in a doc comment. The direct entry is an exact version pin three masters name (test-plan.md:1323, security-plan.md:318, architecture.md:36). **Hypothesis, not measured:** `fuzz_target!(\|name: &str\| …)` resolves typed input through libfuzzer-sys's re-exported `arbitrary`. libfuzzer-sys 0.4.13 is not in the host cargo registry (fuzz builds CI-only). An earlier console statement gave this as fact; this line corrects it. |
| 5 | Host cargo-nextest 0.9.133 vs CI pin 0.9.146 | elsewhere: a host install (the overseer installs it) | A host tool, not code; wrap installs nothing. |
| 6 | obs-plan §1 (verbatim obs-scope copy, obs-plan.md:485) edited by the cascade twice (diagnosis P9: L91, L112) | elsewhere: CARRY for the cleanup chunk's own wrap P2 (propose → approve → append a playbook.md rule) | 0 hits in playbook.md / drift-base.md. A rule append is a chunk wrap's P2 channel, which this 0-pending path does not run. |

**Cost of item 2's split**, counted by `cargo mutants --list` at a28f696:
- run.rs holds 154 of viola-e2e's 431 mutants. A file split moves every line, which puts them in `--in-diff`
  scope, per leg (windows + ubuntu).
- The complexity work alone touches: `run_with` 12 + viola-harness.rs 4 + fake-agent `main` 1, plus the
  extracted helpers.
- Runtime is an estimate from the slowest viola-e2e test, 20.6 s in this session's coverage run: about
  154 × 21 s plus builds, 50+ min per leg.

## Not in this record
- No P1/P2 (no chunk).
- Curation ran with 0 writes; see the handoff's "Deferred learnings" for the filtered candidates.
- The code-audit ledger correction (`mutation.survivors` sites lost their column) rides the next ledger-mode
  record's `corrections[]`. It is named in the audit's proposals.md and in the handoff.
