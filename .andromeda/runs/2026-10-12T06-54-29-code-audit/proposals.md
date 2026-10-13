# Code Audit — viola · Epoch 4 — Session state & governance · 2026-10-12T21:23:40Z
mode trend · HEAD dffad3e02d · baseline e304994ae0 (Epoch 3 - Windows slice II: driving verbs and live proof) · span 1 · overshoot 0 commits (the baseline's: 0)

Trend-break: `code-graph.py` d0425fb1 → f22becbb (its blob hash, a token by construction) — `graph` and `dead` thresholds suppressed this run; both read 0 cycles and 0 dead candidates at both records.

Run conditions the numbers carry (all in the record's `commands`): the mutation tier ran at jobs 2 with `--copy-target=false` (the e304994 record: jobs 1, `--copy-target=true`) — verdicts do not depend on either, `mutation.timing` does; `viola` ran as two invocations (stopped at 678/1299 on the founder's word, resumed with `--iterate` over the other 636, merged — `merge_viola.py`); `viola-e2e` ran from a `git archive` extract of HEAD with `--copy-target=true` after its in-repo invocation failed its unmutated baseline.

## Proposals

### M1 — monotonic · sizes.file_max — 884 → 2678 → 2709
**Movement:** 884 → 2678 → 2709 code lines (Epoch 2b - Windows slice I b: events and ledger → Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance): worse at both of the last two diffs.
**Evidence:** the largest file at all three records is `crates/viola-agent-claude/src/ledger.rs` (884 → 2678 → 2709). The top 10 at HEAD (code lines, tokei over the record's population; e304994 value beside):

| # | file | HEAD | e304994 |
|---|---|---|---|
| 1 | `crates/viola-agent-claude/src/ledger.rs` | 2709 | 2678 |
| 2 | `src/run/send.rs` | 2088 | 1464 |
| 3 | `src/cmd/run.rs` | 1365 | 883 |
| 4 | `src/bin/viola-fake-agent.rs` | 1262 | 950 |
| 5 | `crates/viola-pty/src/lib.rs` | 1148 | 1040 |
| 6 | `src/run/dialog.rs` | 1065 | 930 |
| 7 | `src/human.rs` | 1064 | 509 |
| 8 | `src/cmd/hook.rs` | 1046 | 1012 |
| 9 | `crates/viola-e2e/src/harness/run/mutants.rs` | 1032 | 893 |
| 10 | `tests/cli_verify.rs` | 1028 | 1061 |

**Suspected shape:** the maximum is one file, the capability ledger, whose growth was the Epoch 3 jump (+1794) and is +31 this epoch; the second-largest, `src/run/send.rs`, grew +624 this epoch (1464 → 2088).
**Proposal:** consider splitting `ledger.rs` along its row families (the rows, the probes, the dialog-variant tables) and `src/run/send.rs` along its phases, so neither file carries the next epoch's growth whole.

### M2 — monotonic · sizes.over_800 — 2 → 12 → 19
**Movement:** 2 → 12 → 19 files over 800 code lines (Epoch 2b - Windows slice I b: events and ledger → Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance): worse at both of the last two diffs.
**Evidence:** all 19 files over 800 at HEAD (`c-sizes-over800.json`; the count reproduces the record, and the e304994 list reproduces that record's 12); 8 entered this epoch, 1 left (`tests/cli_answer.rs`, now under 800):

| # | file | HEAD | e304994 | |
|---|---|---|---|---|
| 1 | `crates/viola-agent-claude/src/ledger.rs` | 2709 | 2678 |  |
| 2 | `src/run/send.rs` | 2088 | 1464 |  |
| 3 | `src/cmd/run.rs` | 1365 | 883 |  |
| 4 | `src/bin/viola-fake-agent.rs` | 1262 | 950 |  |
| 5 | `crates/viola-pty/src/lib.rs` | 1148 | 1040 |  |
| 6 | `src/run/dialog.rs` | 1065 | 930 |  |
| 7 | `src/human.rs` | 1064 | 509 | entered |
| 8 | `src/cmd/hook.rs` | 1046 | 1012 |  |
| 9 | `crates/viola-e2e/src/harness/run/mutants.rs` | 1032 | 893 |  |
| 10 | `tests/cli_verify.rs` | 1028 | 1061 |  |
| 11 | `crates/viola-e2e/src/harness/run.rs` | 949 | 949 |  |
| 12 | `tests/cli_send.rs` | 938 | 769 | entered |
| 13 | `tests/cli_fake_agent.rs` | 924 | 788 | entered |
| 14 | `src/run/wheel.rs` | 919 | 759 | entered |
| 15 | `crates/viola-channel/src/server.rs` | 865 | 865 |  |
| 16 | `tests/cli_instance_state.rs` | 859 | 693 | entered |
| 17 | `crates/viola-agent-claude/src/hook.rs` | 849 | 761 | entered |
| 18 | `src/run/budget.rs` | 849 | absent | entered |
| 19 | `crates/viola-state/src/events.rs` | 845 | 525 | entered |

**Suspected shape:** 8 of the 19 are in the root package (`src/`); the largest growths this epoch are `src/run/budget.rs` +849, `src/run/send.rs` +624, `src/human.rs` +555, `src/cmd/run.rs` +482 — `src/human.rs` carries the new `viola list` board writers and `src/run/budget.rs` is new with the budget governor.
**Proposal:** consider a split for the files that entered by growth rather than by birth (`src/human.rs`, `src/run/wheel.rs`, `crates/viola-state/src/events.rs`, `crates/viola-agent-claude/src/hook.rs`) before the next chunk lands in them, and whether the three test files that entered (`tests/cli_send.rs`, `tests/cli_fake_agent.rs`, `tests/cli_instance_state.rs`) share setup a `tests/support` helper could hold.

## Informational

- trend-break — tool upgrade · `graph` (code-graph.py d0425fb1 → f22becbb): its thresholds suppressed this run
- trend-break — tool upgrade · `dead` (code-graph.py d0425fb1 → f22becbb): its thresholds suppressed this run
- dead-growth · dead.zero_ref_candidates 0 → 0 (fires at ≥ +5) — suppressed by the trend-break above; no movement either way
- mutation `viola-e2e`: first scored value 99.73 (scope unit, Epoch 4 - Session state & governance); no earlier record scores it (baseline-test-failure, declined) — no delta
- monotonic · `dead.zero_ref_candidates` 0 → 0 → 0 — suppressed by its trend-break
- count-under-ratio · duplication.clones 189 → 256 (+67), duplicated_lines 1668 → 2132 (+464) while pct 3.13 → 3.12 (-0.01 pt); population total_lines 53261 → 68257 (+28.2 %); split src 73→118 pairs / 622→1077 L · test 103→121 pairs / 1095→1146 L · mixed 13→17 pairs / 140→165 L; top standing pair crates/viola-core/src/obs.rs:175 ↔ tests/contract_diag_schema.rs:20 (21 L), in the top list since Epoch 2b - Windows slice I b: events and ledger
- top-N entrants · duplication.top: tests/cli_budget.rs:71 ↔ tests/cli_wheel.rs:46 (25 L); src/run/dialog.rs:456 ↔ src/run/wheel.rs:793 (19 L); tests/channel_sqos_open.rs:90 ↔ tests/security_negatives_channel.rs:72 (19 L); tests/cli_answer.rs:110 ↔ tests/cli_wait_last.rs:141 (19 L); crates/viola-channel/src/test_support.rs:13 ↔ crates/viola-state/src/lib.rs:49 (18 L); src/run/links.rs:120 ↔ src/run/snapshot.rs:58 (18 L); src/run/links.rs:120 ↔ src/run/wheel.rs:926 (18 L)
- top-N entrants · complexity.top: `send` src/run/send.rs:326 (cognitive 16); `eval` crates/viola-e2e/src/harness/run/mutants/host.rs:286 (cognitive 15); `read` crates/viola-state/src/board.rs:208 (cognitive 15); `next_line` crates/viola-state/src/events.rs:233 (cognitive 14)
- top-N entrants · sizes.top: src/human.rs (1064)
- top-N entrants · hotspots: src/cmd/hook.rs (44); crates/viola-state/src/events.rs (42); src/run/wheel.rs (42); crates/viola-state/src/replay.rs (30); crates/viola-state/src/strict.rs (30); src/run/dialog.rs (28); src/cmd/send.rs (24); tests/cli_answer.rs (22)
- top-N entrants · graph.fan_in_top: `support/home/StampedHome#` (viola, 110); `support/home/impl#[Wrapper]instance_dir().` (viola, 82); `crate/` (viola-state, 81); `snapshot/Wheel#` (viola-state, 78); `support/home/impl#[Wrapper]stop().` (viola, 76); `support/home/impl#[Wrapper]home().` (viola, 75)
- mutation survivors per unit (full list: the record's `mutation.survivors`, the `c-mutation-{unit}.json` twins): `viola` 6 → 32 · `viola-core` 0 → 0 · `viola-pty` 0 → 1 · `viola-state` 12 → 10 · `viola-agent-claude` 6 → 2 · `viola-e2e` — → 2
- `viola` survivors by file (32, = counts.missed): `src/cmd/verify/typed.rs` 18 · `src/run/wheel.rs` 9 · `src/run/dialog.rs` 2 · `src/cmd/hook.rs` 1 · `src/cmd/send.rs` 1 · `src/run/send.rs` 1 — the `viola` score moves 99.33 → 97.03, under the −10 pt line
- not measured on this host (x86_64-unknown-linux-gnu): 112 — `cfg(windows)` 108 · `cfg(all(windows, target_arch = "x86_64"))` 2 · `cfg(all(windows, not(target_arch = "x86_64")))` 2; the project's union verdict, by pointer: .andromeda/test-plan.md line 1150 (the dispatch-only windows-mutants.yml, 'the boundary audit's Windows mutation leg') — run 38215104120 at dffad3e02d: conclusion `failure`; 5 jobs green (viola-state, viola-channel, viola-pty, viola-agent-claude, viola-e2e); the 4 root-package shards failed their UNMUTATED baseline on `cmd::hook::statusline::tests::statusline_an_instance_viola_laid_down_passes_its_check` (`assertion failed: trusted(&instance)`, `src/cmd/hook/statusline.rs:510`) in the cargo-mutants copy, so no Windows verdict exists for `viola`'s 18 (`c-windows-readback.json`; never merged into these counts)

## Below threshold — no action

- new-cycle · graph.cycles 0 → 0 (code-graph.py trend-break: no cycle either way)
- duplication-up · duplication.pct 3.13 → 3.12 (-0.01 pt; fires at ≥ +0.5 pt AND ≥ +15 %)
- complexity-creep · complexity.over_ceiling 4 → 2 (fires at ≥ +3 AND ≥ +25 %)
- coverage-drop · coverage.line 98.12 → 98.45 (+0.33 pt; fires at ≤ −2 pt)
- mutation-drop · `viola` 99.33 → 97.03 (-2.30 pt, scope unit at both, Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance; fires at ≤ −10 pt)
- mutation-drop · `viola-core` 100.0 → 100.0 (+0.00 pt, scope unit at both, Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance; fires at ≤ −10 pt)
- mutation-drop · `viola-pty` 100.0 → 98.51 (-1.49 pt, scope unit at both, Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance; fires at ≤ −10 pt)
- mutation-drop · `viola-state` 93.26 → 96.25 (+2.99 pt, scope unit at both, Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance; fires at ≤ −10 pt)
- mutation-drop · `viola-agent-claude` 98.59 → 99.57 (+0.98 pt, scope unit at both, Epoch 3 - Windows slice II: driving verbs and live proof → Epoch 4 - Session state & governance; fires at ≤ −10 pt)
- monotonic · `duplication.pct` 1.35 → 3.13 → 3.12 (95c1a9b → e304994 → dffad3e): not worse at both diffs
- monotonic · `complexity.over_ceiling` 1 → 4 → 2 (95c1a9b → e304994 → dffad3e): not worse at both diffs
- monotonic · `coverage.line` not evaluable this run: null at 95c1a9b (that record skipped coverage) — never a zero, never a fire
- graph.cross_unit_edges 11 → 11; fan_out {'viola': 5, 'viola-e2e': 3, 'viola-state': 1, 'viola-agent-claude': 1, 'viola-channel': 1} → {'viola': 5, 'viola-e2e': 3, 'viola-state': 1, 'viola-channel': 1, 'viola-agent-claude': 1}
- complexity percentiles · cyclomatic p50 1 → 1, p90 5 → 4; cognitive p50 0 → 0, p90 2 → 2; max `sgr_attributes` 23 → 23
- sizes percentiles · file_p50 264 → 292, file_p90 788 → 865 (no threshold; file_max and over_800 are under Proposals)
- dead.unused_deps unchanged: [['viola-fuzz (fuzz/Cargo.toml, own workspace)', 'arbitrary']] (the fuzz workspace, outside the root graph)
- churn · pct 48.01 → 34.92, files_churned 52 → 40 (no threshold row past two records)
- population (never a movement) · totals.loc 46122 → 58997, totals.files 124 → 147, units 7 → 7

## Skips

- coverage.branch — declined (the project coverage command does not instrument branches (as at the e304994 record))
- mutation:viola-channel — not in scope: no source, manifest or build-script change since e304994 (touched units only); not a `skips[]` entry
