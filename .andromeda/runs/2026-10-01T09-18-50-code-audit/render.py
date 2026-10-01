"""P5 render of proposals.md (proposal-template.md), every evidence table asserted against an INDEPENDENT n."""
import json

R = '.andromeda/runs/2026-10-01T09-18-50-code-audit'
J = lambda n: json.load(open(f'{R}/{n}', encoding='utf-8'))
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
cur, jd, cov = recs[-1], J('c-judge.json'), J('c-coverage.json')
base = [r for r in recs[:-1] if r['sha'] == jd['baseline']][-1]
m = cur['mutation']
L = []
w = L.append

def table(rows, header, n, key):
    assert len(rows) == n, (header, len(rows), n)
    assert len({key(r) for r in rows}) == n, ('duplicate natural key', header)
    w('| ' + ' | '.join(header) + ' |'); w('|' + '---|' * len(header))
    for r in rows: w('| ' + ' | '.join(str(c) for c in r) + ' |')

w(f"# Code Audit — viola · {cur['epoch']} · {cur['ts']}")
w(f"mode trend · HEAD {cur['sha'][:10]} · baseline {base['sha'][:10]} ({base['epoch']}) · span 1")
w(f"Overshoot: {cur['head_overshoot']['commits']} commits past the boundary {cur['head_overshoot']['boundary_sha'][:10]} (the flip of 2026-09-29-fake-agent-drift-contract) · source delta: none (files []) — .andromeda/**, .claude/**, refs/**, .gitignore only; operator-confirmed trend at HEAD. The baseline record's overshoot: 0 commits (at its boundary).")
w("Trend-breaks: none (every tool version token equals the baseline's). Ancestry: ok.")
w(f"Corrections carried in this record: {len(cur['corrections'])} schema-gap fills on {base['sha'][:10]} (`mutation.not_measured` · `mutation.counts` · `mutation.scores`) — 9 of its 11 survivors are `#[cfg(unix)]` spans this host never builds; viola-channel and viola-pty re-score 100.0. Every mutation comparison below reads the corrected values.")
w('')
w('## Proposals')
w('_None of the threshold table\'s checks fired (see Below threshold). The three items below sit OUTSIDE that table: a project gate this audit now judges, and two measurements this run could not take._')
w('')
total_missed = sum(c['missed'] for c in m['counts'].values())
w(f"### M1 — mutation gate (test-plan §10, judged at the boundary since the 2026-09-28 ruling) · {total_missed} survivors in {sum(1 for c in m['counts'].values() if c['missed'])} units")
w(f"**Movement:** measured survivors {len(jd['survivors_baseline_measured'])} → {total_missed} ({base['epoch']} → {cur['epoch']}); the gate (`missed == 0`, `timeout == 0`, `unviable <= caught`) fails on `missed` only — timeouts 0, unviable ≤ caught in every unit.")
w(f"**Evidence** (`c-mutation-{{unit}}.json`; host {m['host']}; site = cargo-mutants file:line:col; text = the tool's own):")
lab = lambda s: 'standing (line moved)' if (s.startswith('crates/viola-state/src/fs.rs') or s.startswith('crates/viola-state/src/pin.rs:81')) else 'first measured (unit declined at Epoch 2)' if s.startswith('src/') else 'new vs baseline'
rows = [[s, t, lab(s)] for s, t in m['survivors']]
table(rows, ['site', 'mutation', 'vs baseline'], total_missed, lambda r: (r[0], r[1]))
w('')
nm_n = sum(c['not_measured'] for c in m['counts'].values())
w(f"Not measured on this host ({m['host']}) — {nm_n} mutants whose whole span a false `cfg` excludes, never survivors:")
table(m['not_measured'], ['site', 'mutation', 'predicate'], nm_n, lambda r: (r[0], r[1]))
w('Project union verdict: none registered (no test-plan line names one for cargo-mutants) — no project union verdict.')
w('')
w("**Suspected shape:** five of the six non-standing survivors sit in code this epoch touched — `sideload.rs` is new (both constant returns of `search_restricted` survive), `pin_companions` is new, and `viola-pty/src/lib.rs` took 5 epoch commits (`PortablePty::resize`, `HostTerminal::drop`); `run.rs` `refuse_stale` predates the epoch and is measured for the first time. Each is an effect (DLL-search state, a resize reaching the child, a drop-time restore, a refusal) no assertion in its unit observes.")
w("**Proposal:** per the founder's ruling, these become corrective-chunk items: a killing test per survivor (an observable for `search_restricted`'s result, the child seeing a resize, `refuse_stale`'s refusal) — or, where a survivor is an equivalent mutant, a recorded exemption naming why.")
w('')
r2 = cov['run2']
w("### M2 — coverage not measured · the project coverage command is red at HEAD on this host")
w(f"**Movement:** coverage.line {base['coverage']['line']} → not measured ({base['epoch']} → {cur['epoch']}).")
w(f"**Evidence:** run 1 ({cov['run1']['window']}): {cov['run1']['nextest_summary']} — a foreign build ran beside it (overseer measurement). Run 2 (overseer direction, after the mutation units; {r2['window']}): {r2['nextest_summary']}; {r2['foreign_rustc_samples_in_test_phase']} foreign rustc in {r2['test_phase_samples']} test-phase samples, CPU {r2['cpu_pct_in_test_phase'][0]}–{r2['cpu_pct_in_test_phase'][1]} %. {cov['overlap']['both']} failures common to both runs; only in run 2: {', '.join(cov['overlap']['only_run2'])}.")
fb = r2['failed_by_test_binary']
assert sum(fb.values()) == r2['failed']
w('Failures by test binary (run 2): ' + ' · '.join(f'{k} {v}' for k, v in fb.items()) + f" = {r2['failed']}.")
w('First panic lines (run 2): ' + ' · '.join(f'"{k}" ×{v}' for k, v in r2['panic_first_lines'].items()) + '.')
w(f"Contrast: {cov['contrast']}.")
w("**Suspected shape:** spawned `viola` processes that never start or never exit when built and run in the repository tree, on a quiet host — the host-load hypothesis holds for run 1 at most; CI on 11f135c is green (overseer).")
w("**Proposal:** treat it as an open red until its cause is known (the project's own rule): diff what the in-repo run sees that cargo-mutants' gitignore-filtered copy does not (gitignored local state under the tree — e.g. `target/e2e-home/`, `target/conpty-seed/`, `target/baseline-target/` named in the handoff — or the tree's location), then re-take coverage for this boundary's record by a correction.")
w('')
w("### M3 — mutation tier blind to viola-e2e · baseline-test-failure by invocation")
w("**Movement:** viola-e2e unscored at both boundaries (Epoch 2: declined, 677 planned; Epoch 2b: baseline-test-failure, 727 planned).")
w("**Evidence:** `mutants-viola-e2e*/mutants.out/log/baseline.log` (summarized; dirs removed): 32 of 246 harness tests fail in the unmutated copied tree — they spawn `target\\debug\\viola-fake-agent.exe`, a root-package bin a `--package=viola-e2e` build never produces (os error 2). Three forms, all baseline-built and tested as `nextest run --package=viola-e2e@0.1.0`: per-unit · `--test-package viola-e2e --test-package viola` · `--test-workspace=true` (cargo-mutants 27.1.0).")
w("**Suspected shape:** the harness's spawn targets are another package's bins; the audit's per-unit form has no prebuild step, while the project's own `run --mutants` prebuilds `viola --features fake-agent` and passes `--copy-target=true`.")
w("**Proposal:** give the boundary tier a viola-e2e form that has the root bins in each copied tree (the project's prebuild + copy-target form, measured for its copy size first, or a viola-e2e test seam that builds what it spawns), so the 727 mutants of the largest unit enter the gate.")
w('')
w('## Informational')
e = jd['entrants']
w('- complexity top-10 entrants: ' + ' · '.join(f"{x['fn']} {x['file']} ({x['value']:g})" for x in e['complexity.top']) + f" — `sgr_attributes` is the one function over the cognitive ceiling (23 > 15), a test helper.")
w('- sizes top-10 entrants: ' + ' · '.join(f'{p} {n}' for p, n in e['sizes.top']))
w('- hotspot entrants: ' + ' · '.join(f'{p} {s:g}' for p, s in e['hotspots']) + f" — top: {cur['hotspots'][0][0]} {cur['hotspots'][0][1]:g}")
w('- fan-in top-20 entrants: ' + ' · '.join(f'`{s}` {n}' for s, n in e['graph.fan_in_top']))
w('- duplication top-10 entrants: ' + ' · '.join(f'{a} ↔ {b} {n} L' for a, b, n in e['duplication.top']) + f" — split now src {cur['duplication']['split']['src']} · test {cur['duplication']['split']['test']} · mixed {cur['duplication']['split']['mixed']}")
w(f"- graph: cross-unit edges {jd['graph_edges'][0]} → {jd['graph_edges'][1]} (new `viola-agent-claude → viola-core`); cycles 0; fan-out viola 5 · viola-e2e 3 · the three library crates 1 each.")
w('')
w('## Below threshold — no action')
for r in jd['rows']:
    w(f"- {r['check']} · {r['metric']}: {r['baseline']} → {r['current']}" + (' (not evaluable)' if r['fires'] is None else ''))
for k, v in jd['monotonic'].items():
    w(f"- monotonic · {k}: {' → '.join(str(x) for x in v['values'])}" + (f" — {v['note']}" if v['fires'] is None else ' — not worsened at both diffs'))
w(f"- duplication volume: clones {base['duplication']['clones']} → {cur['duplication']['clones']} · duplicated lines {base['duplication']['duplicated_lines']} → {cur['duplication']['duplicated_lines']} · population (jscpd total_lines) {base['duplication']['total_lines']} → {cur['duplication']['total_lines']} (+{100*(cur['duplication']['total_lines']/base['duplication']['total_lines']-1):.1f} %) — pct rose, so not a count-under-ratio line")
w(f"- churn {base['churn']['pct']} → {cur['churn']['pct']} % · files churned {base['churn']['files_churned']} → {cur['churn']['files_churned']}")
w(f"- sizes: p50 {base['sizes']['file_p50']} → {cur['sizes']['file_p50']} · p90 {base['sizes']['file_p90']} → {cur['sizes']['file_p90']} · max {base['sizes']['file_max']} → {cur['sizes']['file_max']} · over 800 {base['sizes']['over_800']} → {cur['sizes']['over_800']}; population {base['totals']['files']} → {cur['totals']['files']} files, {base['totals']['loc']} → {cur['totals']['loc']} LOC")
w(f"- complexity percentiles: cyclomatic p50/p90 {base['complexity']['cyclomatic_p50']:g}/{base['complexity']['cyclomatic_p90']:g} → {cur['complexity']['cyclomatic_p50']:g}/{cur['complexity']['cyclomatic_p90']:g} · cognitive p50/p90 {base['complexity']['cognitive_p50']:g}/{base['complexity']['cognitive_p90']:g} → {cur['complexity']['cognitive_p50']:g}/{cur['complexity']['cognitive_p90']:g}")
w(f"- dead: candidates 0 → 0 (817 raw zero-ref: 792 tests/-segment, 22 FP-classed, 3 residuals reviewed by hand as FP); unused deps: viola-e2e `proc-macro2` gone, fuzz `arbitrary` standing")
w('- mutation scores (corrected baseline → now): ' + ' · '.join(f"{u} {v['last_scored']} → {v['now']}" for u, v in jd['mutation'].items()))
w('')
w('## Skips')
for s in cur['skips']:
    w(f"- {s['metric']} — {s['reason']}: {s.get('note', '')}")
open(f'{R}/proposals.md', 'w', encoding='utf-8', newline='\n').write('\n'.join(L) + '\n')
print('rendered', len(L), 'lines')
