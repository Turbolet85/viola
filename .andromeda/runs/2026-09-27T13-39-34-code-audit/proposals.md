# Code Audit — viola · Epoch 2 — Windows slice I: wrapper, events, ledger · 2026-09-27T13:54:15Z
mode trend · HEAD 69abc0d038732cff8d45602f82f8bebaf28bbb72 · baseline a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d (Epoch 1 — Foundation) · span 1
Overshoot: 0 commits — HEAD is the boundary (the flip of 2026-09-27-wrapper-channel); the baseline record's overshoot is 0 commits too.
Trend-break: cargo-nextest 0.9.133 → 0.9.146, the runner under the coverage and mutation commands (cargo-llvm-cov 0.9.1 and cargo-mutants 27.1.0 unchanged); both metrics' thresholds are suppressed and their deltas labelled — neither would have fired (coverage −1.44 pt against a −2 pt floor; the one re-scored unit rose).
Correction carried in this record's `corrections[]`: the baseline's `mutation.survivors` rows lacked columns (two apparent duplicate pairs); re-keyed to `lib.rs:9:31` / `:9:38`, measured from the baseline's own source.

## Proposals

None — no row of the threshold table fires at span 1 (new-cycle, duplication-up, complexity-creep, dead-growth, coverage-drop, mutation-drop; monotonic is not yet evaluable). Everything that moved is below.

## Informational

- **count-under-ratio · duplication** — clones 16 → 34 (+18), duplicated lines 137 → 274 (+137), while pct 1.42 → 1.29 (-0.13 pt); population `total_lines` 9679 → 21210 (+119 %). Split src/test/mixed (pairs · lines): src 6·42 → 14·108 · test 8·84 → 12·106 · mixed 2·27 → 8·94. Top standing pair: `crates/viola-core/src/obs.rs:175` ↔ `tests/contract_diag_schema.rs:19` (21 lines; entered Epoch 1 — Foundation, then at `:18`, now `:19`: the same fragment moved one line).
- **top-N entrants · duplication** — 9 of the 10 top pairs are new this epoch (full list, all 10 rows):

| # | fragment A | fragment B | lines | vs baseline top |
|---|---|---|---|---|
| 1 | `crates/viola-channel/src/server.rs:919` | `tests/channel_endpoint.rs:385` | 21 | new |
| 2 | `crates/viola-core/src/obs.rs:175` | `tests/contract_diag_schema.rs:19` | 21 | standing family (baseline `:18`) |
| 3 | `tests/channel_sqos_open.rs:90` | `tests/security_negatives_channel.rs:72` | 19 | new |
| 4 | `crates/viola-channel/src/server.rs:967` | `tests/channel_endpoint.rs:330` | 17 | new |
| 5 | `crates/viola-channel/src/lib.rs:103` | `src/cmd/run.rs:372` | 13 | new |
| 6 | `tests/run_cli.rs:384` | `tests/tui_env_strip.rs:34` | 12 | new |
| 7 | `crates/viola-channel/tests/channel_frames.rs:40` | `tests/channel_endpoint.rs:51` | 11 | new |
| 8 | `crates/viola-channel/src/lib.rs:210` | `crates/viola-e2e/src/harness/cleanup.rs:324` | 10 | new |
| 9 | `crates/viola-pty/src/lib.rs:532` | `crates/viola-pty/src/lib.rs:713` | 10 | new |
| 10 | `tests/run_cli.rs:453` | `tests/tui_env_strip.rs:108` | 10 | new |

  Shapes visible in the list: the new `viola-channel` server has two src↔test pairs with `tests/channel_endpoint.rs` (21 + 17 lines); `viola-channel/src/lib.rs:103` ↔ `src/cmd/run.rs:372` (13 lines) is the tracing capture layer duplicated between the channel crate's `test_capture` module and the root bin's run test module; `tests/run_cli.rs` ↔ `tests/tui_env_strip.rs` holds two pairs (12 + 10). A change-direction the founder may weigh: one shared test-support capture layer and one endpoint-fixture helper.
- **top-N entrants · sizes** — files over 800 code lines 1 → 4: `crates/viola-e2e/src/harness/pre_push.rs` 1335 · `crates/viola-pty/src/lib.rs` 1074 · `crates/viola-e2e/src/harness/run/mutants.rs` 1029 · `crates/viola-channel/src/server.rs` 869 (all four new; the baseline's one, `harness/run.rs` 1562, is no longer over 800 — a `harness/run/` module dir now sits beside it). file_max 1562 → 1335, p50 177 → 191, p90 578 → 609. `sizes.over_800` is a monotonic-tracked scalar: this is its first worsening diff; a second consecutive rise at the Epoch 2b boundary fires `monotonic`.
- **complexity (improved)** — over-ceiling (cognitive > 15) 3 → 0; max 24.0 (`run_with`, harness/run.rs) → 15.0 (`scan_file`, `crates/viola-e2e/src/harness/secret_scan.rs:116` — at the ceiling, not over it). Percentiles unchanged (cyclomatic p50/p90 1/4, cognitive 0/2). New in the top-10: `eval` cfg_legs.rs:137 (14), `coverage` / `finish` / `native` in pre_push.rs (12 / 11 / 11), `union` gate.rs:120 (11), `readiness` boot.rs:271 (10).
- **coverage (trend-break — tool upgrade: nextest runner)** — line 98.21 → 96.77 (-1.44 pt; 4128 / 4266 lines; functions 96.01 %, regions 96.49 %; 605/605 tests passed). Below the −2 pt floor either way; the baseline stores no line count, so the population change is not separable.
- **mutation (trend-break — tool upgrade: nextest runner)** — viola-core 86.21 → 100.0 (the four `lib.rs:9` MAX_FRAME survivors are gone; the route entry 2026-09-24-epoch-1-cleanup names “MAX_FRAME value witnessed by a test”). First scores for the four new units: viola-channel 96.72 (118/4/16 caught/missed/unviable, complete 138/138) · viola-state 96.0 (48/2/8 caught/missed/unviable, complete 58/58) · viola-pty 92.75 (64/5/16 caught/missed/unviable, complete 85/85) · viola-agent-claude 100.0 (36/0/4 caught/missed/unviable, complete 40/40). Score formula caught/(caught+missed); single host leg (Windows), `-j 4` — the project's own gate verdict is the two-leg union, so a survivor here may die on the ubuntu leg. The complete survivor list (11 = Σ missed):

| unit | site | mutation | function |
|---|---|---|---|
| viola-channel | `crates/viola-channel/src/client.rs:195:5` | replace open -> io::Result<Stream> with Ok(Default::default()) | `open` |
| viola-channel | `crates/viola-channel/src/endpoint.rs:76:5` | replace host_socket_dir -> PathBuf with Default::default() | `host_socket_dir` |
| viola-channel | `crates/viola-channel/src/server.rs:75:9` | replace <impl Drop for Guard>::drop with () | `<impl Drop for Guard>::drop` |
| viola-channel | `crates/viola-channel/src/server.rs:108:5` | replace listen -> Result<(Listener, Guard), ChannelError> with Ok((Default::default(), Default::default())) | `listen` |
| viola-state | `crates/viola-state/src/fs.rs:16:5` | replace restrict -> io::Result<()> with Ok(()) | `restrict` |
| viola-state | `crates/viola-state/src/pin.rs:74:19` | replace match guard e.kind() == io::ErrorKind::NotFound with true | `pin_exe` |
| viola-pty | `crates/viola-pty/src/lib.rs:430:9` | replace HostTerminal::enter -> Option<Self> with None | `HostTerminal::enter` |
| viola-pty | `crates/viola-pty/src/lib.rs:430:9` | replace HostTerminal::enter -> Option<Self> with Some(Default::default()) | `HostTerminal::enter` |
| viola-pty | `crates/viola-pty/src/lib.rs:432:64` | replace != with == | `HostTerminal::enter` |
| viola-pty | `crates/viola-pty/src/lib.rs:437:71` | replace == with != | `HostTerminal::enter` |
| viola-pty | `crates/viola-pty/src/lib.rs:482:76` | replace != with == | `host_size` |

  Change-direction for the founder: the four viola-channel and two viola-state rows are candidate killing tests for the Epoch 2 cleanup chunk (`viola-state/src/fs.rs:16` `restrict` returns `Ok(())` — a Windows-host reading of a mode-setting fn, which only the ubuntu leg can judge); the five viola-pty rows all sit in the Windows `HostTerminal::enter` / `host_size` console-mode paths.
- **graph** — units 3 → 7, cross-unit edges 2 → 10, cycles 0 → 0. Fan-out: viola 5 (every product crate — the root bin), viola-e2e 3, viola-state 1, viola-channel 1 (each → viola-core). Fan-in entrants: `viola-pty Size#` 44, `viola-channel crate/` 42, `viola-e2e harness/run/` 33, `harness/run/Runner#` 30, `viola-channel ProtocolError#` 29, `ChannelError#` 27.
- **dead** — zero-ref candidates 0 → 0 (584 raw zero-ref rows: 557 tests by the pinned `tests/` segment union; 27 non-test, all classed — 11 trait-impl methods, 7 cfg(test) `test_capture` helpers, 3 entry points, 3 derive/attr-invoked, 2 format-string captures, 1 trait method reached by dyn dispatch; c-dead.json lists each). Unused deps: new `viola-e2e → proc-macro2` — a feature-enabling pin (`span-locations`, Cargo.toml:139-140, read at harness/cfg_legs.rs:78), a machete false positive; a `[package.metadata.cargo-machete] ignored` entry would state that. The fuzz `arbitrary` row stands from the baseline.
- **churn (first value)** — 16.97 % of added source lines were re-touched adds (30 of 58 touched files churned; 13 647 adds over 14 commits). Informational until the ledger holds 2+ churn values.
- **hotspots (first list; score = commits × max cognitive)** — `crates/viola-e2e/src/harness/pre_push.rs` 48 · `src/bin/viola-fake-agent.rs` 44 · `crates/viola-e2e/src/harness/run/mutants.rs` 42 · `crates/viola-e2e/src/harness/secret_scan.rs` 30 · `tests/run_cli.rs` 30 · `tests/tui_env_strip.rs` 30 · `tests/tui_passthrough.rs` 30 · `tests/support/home.rs` 27 · `crates/viola-pty/src/lib.rs` 21 · `crates/viola-e2e/src/harness/boot.rs` 20.

## Below threshold — no action

- duplication pct 1.42 → 1.29 (fell; duplication-up needs +0.5 pt and +15 %).
- complexity-creep: over_ceiling fell 3 → 0.
- dead-growth: 0 → 0.
- coverage-drop: −1.44 pt against the −2 pt floor (and suppressed by the trend-break).
- mutation-drop: viola-core rose; no other unit has a prior score.
- new-cycle: 0 → 0.
- monotonic: not evaluable — the baseline record has no predecessor (its `baseline_sha` is null), so no scalar has two diffs yet; all six tracked scalars are non-null in both records.

## Skips

- mutation:viola — declined (205 planned mutants - over the per-unit budget; attended decline)
- mutation:viola-e2e — declined (677 planned mutants - over the per-unit budget; attended decline)
- coverage.branch — declined (the project coverage command does not instrument branches (llvm-cov branch count 0))

## %TEMP% (operator request)

After each of the five mutation units, `%TEMP%\cargo-mutants-*` held only the four pre-existing directories — no copy from this run survived. The pre-existing ones (not created by this run, outside this skill's delete scope, left for the operator): `cargo-mutants-viola-Bk11Ik.tmp` 29.96 GB (2026-09-27 15:26 local) · `cargo-mutants-viola-uaVPZI.tmp` 29.65 GB (12:41 local) · two empty `cargo-mutants-conductor-*.tmp` (2026-09-15). Total 59.61 GB.
