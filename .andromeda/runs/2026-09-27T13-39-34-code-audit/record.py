import json, os, subprocess, datetime
R = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
BASE = 'a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d'
UNITS_RUN = ['viola-channel', 'viola-state', 'viola-pty', 'viola-agent-claude', 'viola-core']
UNITS_ALL = UNITS_RUN + ['viola', 'viola-e2e']
L = lambda n: json.load(open(f'{R}/{n}', encoding='utf-8'))
sha = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()
sz, dp, cx, gr, dd, ch, hs, cv = (L('c-sizes.json'), L('c-duplication.json'), L('c-complexity.json'), L('c-graph.json'),
                                  L('c-dead.json'), L('c-churn.json'), L('c-hotspots.json'), L('c-coverage.json'))
mut = {'scoped_units': UNITS_ALL, 'unit_states': {}, 'scores': {}, 'counts': {}, 'score_formula': 'caught/(caught+missed)', 'survivors': []}
skips = []
for u in UNITS_RUN:
    p = f'{R}/c-mutation-{u}.json'
    if not os.path.exists(p):
        mut['unit_states'][u] = 'budget-exhausted'
        skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted'}); continue
    m = json.load(open(p, encoding='utf-8'))
    assert m['survivor_rows_unique'] and m['survivors_equal_missed'], u
    mut['unit_states'][u] = m['state']
    mut['counts'][u] = m['counts']
    if m['score'] is not None:
        mut['scores'][u] = m['score']
    else:
        skips.append({'metric': f'mutation:{u}', 'reason': 'unviable-dominant'})
    mut['survivors'] += [[s[0], s[1]] for s in m['survivors']]
for u in ('viola', 'viola-e2e'):
    mut['unit_states'][u] = 'declined'
    skips.append({'metric': f'mutation:{u}', 'reason': 'declined', 'note': {'viola': '205 planned mutants', 'viola-e2e': '677 planned mutants'}[u] + ' - over the per-unit budget; attended decline'})
skips.append({'metric': 'coverage.branch', 'reason': 'declined', 'note': 'the project coverage command does not instrument branches (llvm-cov branch count 0)'})
run = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
rec = {
 'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
 'epoch': 'Epoch 2 — Windows slice I: wrapper, events, ledger', 'mode': 'trend', 'sha': sha, 'baseline_sha': BASE, 'span': 1,
 'ancestry_broken': False,
 'head_overshoot': {'boundary_sha': sha, 'commits': 0, 'files': [], 'note': 'HEAD is the boundary: the flip of 2026-09-27-wrapper-channel'},
 'tool_versions': {'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'cargo-machete': '0.9.2', 'cargo-mutants': '27.1.0',
                   'cargo-llvm-cov': '0.9.1', 'cargo-nextest': '0.9.146', 'rustc': '1.98.1', 'code-graph.py': 'd0425fb1'},
 'totals': sz['totals'],
 'duplication': {k: dp[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
 'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top')},
 'sizes': sz['sizes'],
 'graph': {k: gr[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
 'dead': {k: dd[k] for k in ('unused_deps', 'zero_ref_candidates', 'top')},
 'coverage': {'line': cv['line'], 'branch': None},
 'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
 'hotspots': hs['hotspots'],
 'mutation': mut,
 'commands': {
  'sizes': f'tokei --output json $(cat {run}/source-files.txt)  # population: git ls-files "*.rs" minus ^(scripts|docs|refs|fuzz|.andromeda|.claude)/ ({sz["totals"]["files"]} files)',
  'duplication': f'jscpd --reporters json --output {run}/jscpd --format rust --silent --ignore "fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**" .',
  'complexity': f'mkdir -p {run}/rca ; rust-code-analysis-cli -m -O json -o {run}/rca $(sed "s/^/-p /" {run}/source-files.txt)  # per-function cognitive = metrics.cognitive.sum (calibrated: the baseline run_with = 24 reproduces at {BASE[:7]})',
  'graph': f'python scripts/code-graph.py query {run} code-audit "<audit-pass.md canonical CYCLES / FAN-IN / FAN-OUT / edge-count SQL, verbatim>"  # driven by {run}/graph_q.py',
  'dead': f'cargo machete ; python scripts/code-graph.py query {run} code-audit "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line"  # classed by {run}/summarize.py (tests/ segment union on the prefix-stripped symbol path and the file path, then FP classes)',
  'coverage': f"cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path {run}/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\\\]viola-e2e|tests[/\\\\]support|fuzz[/\\\\])'  # run by path: {run}/cov.sh",
  'churn': ch['command'],
  'hotspots': 'c-churn per-file commit count x max per-function cognitive in the file (c-complexity)',
  'mutation': f'NEXTEST_PROFILE=mutants timeout 900 cargo mutants -p {{unit}} --test-tool=nextest -j 4 --output {run}/mutants-{{unit}}  # one unit at a time via {run}/mut.sh (stops rust-analyzer by exact ExecutablePath first)',
  'head_overshoot': "git log --format=%H -1 -S'2026-09-27-wrapper-channel · complete' -- .andromeda/master-route.md ; git rev-list --count " + sha + "..HEAD ; git diff --numstat " + sha + "..HEAD -- (source paths)",
 },
 'corrections': [{'target_sha': BASE, 'field': 'mutation.survivors',
   'was': [['crates/viola-core/src/lib.rs:9', 'replace * with +'], ['crates/viola-core/src/lib.rs:9', 'replace * with /'], ['crates/viola-core/src/lib.rs:9', 'replace * with +'], ['crates/viola-core/src/lib.rs:9', 'replace * with /']],
   'now': [['crates/viola-core/src/lib.rs:9:31', 'replace * with +'], ['crates/viola-core/src/lib.rs:9:31', 'replace * with /'], ['crates/viola-core/src/lib.rs:9:38', 'replace * with +'], ['crates/viola-core/src/lib.rs:9:38', 'replace * with /']],
   'note': 'the baseline survivor key dropped the column, so its four rows read as two duplicated pairs; line 9 at the baseline sha is `pub const MAX_FRAME: u64 = 16 * 1024 * 1024;` with `*` at columns 31 and 38, each carrying one + and one / mutant (4 missed = 2 operators x 2 sites). Measured from `git show <baseline>:crates/viola-core/src/lib.rs`, not from the prior run dir'}],
 'skips': skips,
}
with open(f'{R}/record.json', 'w', encoding='utf-8', newline='') as f:
    json.dump(rec, f, ensure_ascii=False, indent=1)
print('record.json written', rec['ts'], mut['unit_states'], mut['scores'])
