import json
R = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
BASE = 'a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d'
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
cur = recs[-1]
b = [r for r in recs[:-1] if r['sha'] == BASE][-1]
assert cur['sha'] != b['sha']
m = cur['mutation']
surv = m['survivors']
n_missed = sum(c['missed'] for c in m['counts'].values())
assert len(surv) == n_missed and len({(s[0], s[1]) for s in surv}) == len(surv), 'survivor rows'
top = cur['duplication']['top']
assert len(top) == min(10, cur['duplication']['clones']) and len({tuple(t) for t in top}) == len(top), 'dup top rows'
bt = {(x[0], x[1]) for x in b['duplication']['top']}
fn_of = {}
for u in m['counts']:
    for s in json.load(open(f'{R}/c-mutation-{u}.json', encoding='utf-8'))['survivors']:
        fn_of[(s[0], s[1])] = s[2]
dp, bd = cur['duplication'], b['duplication']
o = []
w = o.append
w(f"# Code Audit — viola · {cur['epoch']} · {cur['ts']}")
w(f"mode trend · HEAD {cur['sha']} · baseline {BASE} ({b['epoch']}) · span 1")
w("Overshoot: 0 commits — HEAD is the boundary (the flip of 2026-09-27-wrapper-channel); the baseline record's overshoot is 0 commits too.")
w("Trend-break: cargo-nextest 0.9.133 → 0.9.146, the runner under the coverage and mutation commands (cargo-llvm-cov 0.9.1 and cargo-mutants 27.1.0 unchanged); both metrics' thresholds are suppressed and their deltas labelled — neither would have fired (coverage −1.44 pt against a −2 pt floor; the one re-scored unit rose).")
w("Correction carried in this record's `corrections[]`: the baseline's `mutation.survivors` rows lacked columns (two apparent duplicate pairs); re-keyed to `lib.rs:9:31` / `:9:38`, measured from the baseline's own source.\n")
w("## Proposals\n")
w("None — no row of the threshold table fires at span 1 (new-cycle, duplication-up, complexity-creep, dead-growth, coverage-drop, mutation-drop; monotonic is not yet evaluable). Everything that moved is below.\n")
w("## Informational\n")
w(f"- **count-under-ratio · duplication** — clones {bd['clones']} → {dp['clones']} (+{dp['clones'] - bd['clones']}), duplicated lines {bd['duplicated_lines']} → {dp['duplicated_lines']} (+{dp['duplicated_lines'] - bd['duplicated_lines']}), while pct {bd['pct']} → {dp['pct']} ({dp['pct'] - bd['pct']:+.2f} pt); population `total_lines` {bd['total_lines']} → {dp['total_lines']} (+{100.0 * (dp['total_lines'] - bd['total_lines']) / bd['total_lines']:.0f} %). Split src/test/mixed (pairs · lines): " +
  ' · '.join(f"{k} {bd['split'][k]['pairs']}·{bd['split'][k]['lines']} → {dp['split'][k]['pairs']}·{dp['split'][k]['lines']}" for k in ('src', 'test', 'mixed')) +
  ". Top standing pair: `crates/viola-core/src/obs.rs:175` ↔ `tests/contract_diag_schema.rs:19` (21 lines; entered Epoch 1 — Foundation, then at `:18`, now `:19`: the same fragment moved one line).")
w(f"- **top-N entrants · duplication** — 9 of the 10 top pairs are new this epoch (full list, all {len(top)} rows):")
w("\n| # | fragment A | fragment B | lines | vs baseline top |")
w("|---|---|---|---|---|")
for i, t in enumerate(top, 1):
    st = 'standing family (baseline `:18`)' if t[0] == 'crates/viola-core/src/obs.rs:175' else ('standing' if (t[0], t[1]) in bt else 'new')
    w(f"| {i} | `{t[0]}` | `{t[1]}` | {t[2]} | {st} |")
w("\n  Shapes visible in the list: the new `viola-channel` server has two src↔test pairs with `tests/channel_endpoint.rs` (21 + 17 lines); `viola-channel/src/lib.rs:103` ↔ `src/cmd/run.rs:372` (13 lines) is the tracing capture layer duplicated between the channel crate's `test_capture` module and the root bin's run test module; `tests/run_cli.rs` ↔ `tests/tui_env_strip.rs` holds two pairs (12 + 10). A change-direction the founder may weigh: one shared test-support capture layer and one endpoint-fixture helper.")
s, bs = cur['sizes'], b['sizes']
w(f"- **top-N entrants · sizes** — files over 800 code lines {bs['over_800']} → {s['over_800']}: `crates/viola-e2e/src/harness/pre_push.rs` 1335 · `crates/viola-pty/src/lib.rs` 1074 · `crates/viola-e2e/src/harness/run/mutants.rs` 1029 · `crates/viola-channel/src/server.rs` 869 (all four new; the baseline's one, `harness/run.rs` 1562, is no longer over 800 — a `harness/run/` module dir now sits beside it). file_max {bs['file_max']} → {s['file_max']}, p50 {bs['file_p50']} → {s['file_p50']}, p90 {bs['file_p90']} → {s['file_p90']}. `sizes.over_800` is a monotonic-tracked scalar: this is its first worsening diff; a second consecutive rise at the Epoch 2b boundary fires `monotonic`.")
c, bc = cur['complexity'], b['complexity']
w(f"- **complexity (improved)** — over-ceiling (cognitive > 15) {bc['over_ceiling']} → {c['over_ceiling']}; max {bc['max']['val']} (`{bc['max']['fn']}`, harness/run.rs) → {c['max']['val']} (`{c['max']['fn']}`, `{c['max']['file']}` — at the ceiling, not over it). Percentiles unchanged (cyclomatic p50/p90 1/4, cognitive 0/2). New in the top-10: `eval` cfg_legs.rs:137 (14), `coverage` / `finish` / `native` in pre_push.rs (12 / 11 / 11), `union` gate.rs:120 (11), `readiness` boot.rs:271 (10).")
w(f"- **coverage (trend-break — tool upgrade: nextest runner)** — line {b['coverage']['line']} → {cur['coverage']['line']} ({cur['coverage']['line'] - b['coverage']['line']:+.2f} pt; 4128 / 4266 lines; functions 96.01 %, regions 96.49 %; 605/605 tests passed). Below the −2 pt floor either way; the baseline stores no line count, so the population change is not separable.")
bs_ = b['mutation']['scores']
w(f"- **mutation (trend-break — tool upgrade: nextest runner)** — viola-core {bs_['viola-core']} → {m['scores']['viola-core']} (the four `lib.rs:9` MAX_FRAME survivors are gone; the route entry 2026-09-24-epoch-1-cleanup names “MAX_FRAME value witnessed by a test”). First scores for the four new units: " +
  ' · '.join(f"{u} {m['scores'][u]} ({m['counts'][u]['caught']}/{m['counts'][u]['missed']}/{m['counts'][u]['unviable']} caught/missed/unviable, {m['unit_states'][u]})" for u in ('viola-channel', 'viola-state', 'viola-pty', 'viola-agent-claude')) +
  f". Score formula {m['score_formula']}; single host leg (Windows), `-j 4` — the project's own gate verdict is the two-leg union, so a survivor here may die on the ubuntu leg. The complete survivor list ({len(surv)} = Σ missed):")
w("\n| unit | site | mutation | function |")
w("|---|---|---|---|")
for s_ in surv:
    u = s_[0].split('/')[1]
    w(f"| {u} | `{s_[0]}` | {s_[1].replace('|', chr(92) + '|')} | `{fn_of[(s_[0], s_[1])]}` |")
w("\n  Change-direction for the founder: the four viola-channel and two viola-state rows are candidate killing tests for the Epoch 2 cleanup chunk (`viola-state/src/fs.rs:16` `restrict` returns `Ok(())` — a Windows-host reading of a mode-setting fn, which only the ubuntu leg can judge); the five viola-pty rows all sit in the Windows `HostTerminal::enter` / `host_size` console-mode paths.")
w(f"- **graph** — units {b['totals']['units']} → {cur['totals']['units']}, cross-unit edges {b['graph']['cross_unit_edges']} → {cur['graph']['cross_unit_edges']}, cycles 0 → 0. Fan-out: viola 5 (every product crate — the root bin), viola-e2e 3, viola-state 1, viola-channel 1 (each → viola-core). Fan-in entrants: `viola-pty Size#` 44, `viola-channel crate/` 42, `viola-e2e harness/run/` 33, `harness/run/Runner#` 30, `viola-channel ProtocolError#` 29, `ChannelError#` 27.")
w(f"- **dead** — zero-ref candidates 0 → 0 (584 raw zero-ref rows: 557 tests by the pinned `tests/` segment union; 27 non-test, all classed — 11 trait-impl methods, 7 cfg(test) `test_capture` helpers, 3 entry points, 3 derive/attr-invoked, 2 format-string captures, 1 trait method reached by dyn dispatch; c-dead.json lists each). Unused deps: new `viola-e2e → proc-macro2` — a feature-enabling pin (`span-locations`, Cargo.toml:139-140, read at harness/cfg_legs.rs:78), a machete false positive; a `[package.metadata.cargo-machete] ignored` entry would state that. The fuzz `arbitrary` row stands from the baseline.")
w(f"- **churn (first value)** — {cur['churn']['pct']} % of added source lines were re-touched adds (30 of 58 touched files churned; 13 647 adds over 14 commits). Informational until the ledger holds 2+ churn values.")
w("- **hotspots (first list; score = commits × max cognitive)** — " + ' · '.join(f"`{h[0]}` {h[1]:g}" for h in cur['hotspots']) + ".\n")
w("## Below threshold — no action\n")
w(f"- duplication pct {bd['pct']} → {dp['pct']} (fell; duplication-up needs +0.5 pt and +15 %).")
w("- complexity-creep: over_ceiling fell 3 → 0.")
w("- dead-growth: 0 → 0.")
w("- coverage-drop: −1.44 pt against the −2 pt floor (and suppressed by the trend-break).")
w("- mutation-drop: viola-core rose; no other unit has a prior score.")
w("- new-cycle: 0 → 0.")
w("- monotonic: not evaluable — the baseline record has no predecessor (its `baseline_sha` is null), so no scalar has two diffs yet; all six tracked scalars are non-null in both records.\n")
w("## Skips\n")
for sk in cur['skips']:
    w(f"- {sk['metric']} — {sk['reason']}" + (f" ({sk['note']})" if sk.get('note') else ''))
w("\n## %TEMP% (operator request)\n")
w("After each of the five mutation units, `%TEMP%\\cargo-mutants-*` held only the four pre-existing directories — no copy from this run survived. The pre-existing ones (not created by this run, outside this skill's delete scope, left for the operator): `cargo-mutants-viola-Bk11Ik.tmp` 29.96 GB (2026-09-27 15:26 local) · `cargo-mutants-viola-uaVPZI.tmp` 29.65 GB (12:41 local) · two empty `cargo-mutants-conductor-*.tmp` (2026-09-15). Total 59.61 GB.")
open(R + '/proposals.md', 'w', encoding='utf-8', newline='').write('\n'.join(o) + '\n')
print('rendered', len(o), 'lines; survivors', len(surv), 'dup rows', len(top))
