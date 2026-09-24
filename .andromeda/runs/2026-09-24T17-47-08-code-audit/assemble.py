import json, os, subprocess

D = '.andromeda/runs/2026-09-24T17-47-08-code-audit'
L = lambda n: json.load(open(f'{D}/c-{n}.json', encoding='utf-8'))
SHA = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()

# C1 mutation — viola-core; read only when the invocation is complete by the tool's own markers
unit = 'viola-core'
mo = f'{D}/mutants-{unit}/mutants.out'
oc = json.load(open(f'{mo}/outcomes.json', encoding='utf-8'))
planned = len(json.load(open(f'{mo}/mutants.json', encoding='utf-8'))) if os.path.exists(f'{mo}/mutants.json') else None
complete = bool(oc.get('end_time')) and (planned is None or oc.get('total_mutants') == planned)
assert complete, ('incomplete invocation', oc.get('end_time'), oc.get('total_mutants'), planned)
cnt = {'mutants': oc.get('total_mutants'), 'caught': oc.get('caught'), 'missed': oc.get('missed'),
       'timeout': oc.get('timeout'), 'unviable': oc.get('unviable')}
surv = []
for o in oc['outcomes']:
    if o.get('summary') == 'MissedMutant':
        m = o['scenario']['Mutant']
        surv.append([f"{m['file']}:{m['span']['start']['line']}", (m.get('name') or '').split(': ', 1)[-1]])
tested = cnt['caught'] + cnt['missed']
score = round(100 * cnt['caught'] / tested, 2) if tested else None
mut = {'unit': unit, 'unit_state': f"complete {cnt['mutants']}/{planned}", 'counts': cnt, 'score': score,
       'score_formula': 'caught/(caught+missed)', 'survivors': surv, 'baseline_outcome': oc['outcomes'][0].get('summary')}
json.dump(mut, open(f'{D}/c-mutation-{unit}.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

dup, cx, sz, g, dead, cov = L('duplication'), L('complexity'), L('sizes'), L('graph'), L('dead'), L('coverage')
RD = D
rec = {
 'ts': os.environ['AUDIT_TS'], 'epoch': 'Epoch 1 — Foundation', 'mode': 'baseline',
 'sha': SHA, 'baseline_sha': None, 'span': None, 'ancestry_broken': False,
 'head_overshoot': {'boundary_sha': SHA, 'commits': 0, 'files': [], 'note': 'HEAD is the boundary: the flip of 2026-09-24-workspace-tree-and-code-graph-planes'},
 'tool_versions': {'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'cargo-machete': '0.9.2',
                   'cargo-mutants': '27.1.0', 'cargo-llvm-cov': '0.9.1', 'cargo-nextest': '0.9.133', 'rustc': '1.98.1',
                   'code-graph.py': 'd0425fb1'},
 'totals': {'loc': sz['loc'], 'files': sz['files'], 'units': 3},
 'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
 'complexity': {'cyclomatic_p50': cx['cyclomatic_p50'], 'cyclomatic_p90': cx['cyclomatic_p90'],
                'cognitive_p50': cx['cognitive_p50'], 'cognitive_p90': cx['cognitive_p90'],
                'over_ceiling': cx['over_ceiling'], 'max': cx['max'], 'top': cx['top']},
 'sizes': {k: sz[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
 'graph': {k: g[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
 'dead': {'unused_deps': dead['unused_deps'], 'zero_ref_candidates': dead['zero_ref_candidates'], 'top': dead['top']},
 'coverage': {'line': cov['line'], 'branch': None},
 'churn': {'pct': None, 'files_churned': None},
 'hotspots': [],
 'mutation': {'scoped_units': [unit], 'unit_states': {unit: mut['unit_state'], 'viola': 'declined', 'viola-e2e': 'declined'},
              'scores': {unit: score}, 'counts': {unit: cnt}, 'score_formula': 'caught/(caught+missed)', 'survivors': surv},
 'commands': {
   'sizes': f'tokei --output json $(cat {RD}/source-files.txt)  # population: git ls-files "*.rs" minus ^(scripts|docs|refs|fuzz|.andromeda|.claude)/ (33 files)',
   'duplication': f'jscpd --reporters json --output {RD}/jscpd --format rust --silent --ignore "fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**" .',
   'complexity': f'rust-code-analysis-cli -m -O json -o {RD}/rca $(sed "s/^/-p /" {RD}/source-files.txt)',
   'graph': f'python scripts/code-graph.py query {RD} code-audit "<audit-pass.md canonical CYCLES / FAN-IN / FAN-OUT / edge-count SQL, verbatim>"',
   'dead': f'cargo machete ; python scripts/code-graph.py query {RD} code-audit "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line"  # classed by summarize.py (tests/ segment union, then FP classes)',
   'coverage': f"cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path {RD}/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\\\]viola-e2e|tests[/\\\\]support|fuzz[/\\\\])'",
   'churn': None, 'hotspots': None,
   'mutation': f'NEXTEST_PROFILE=mutants timeout 900 cargo mutants -p {{unit}} --test-tool=nextest -j 1 --output {RD}/mutants-{{unit}}',
   'head_overshoot': "git log --format=%H -1 -S'2026-09-24-workspace-tree-and-code-graph-planes · complete' -- .andromeda/master-route.md ; git rev-list --count " + SHA + "..HEAD ; git diff --numstat " + SHA + "..HEAD -- (source paths)",
 },
 'corrections': [],
 'skips': [{'metric': 'churn', 'reason': 'no-baseline'}, {'metric': 'hotspots', 'reason': 'no-baseline'},
           {'metric': 'mutation:viola', 'reason': 'declined'}, {'metric': 'mutation:viola-e2e', 'reason': 'declined'},
           {'metric': 'coverage.branch', 'reason': 'declined', 'note': 'the project coverage command does not instrument branches (llvm-cov branch count 0)'}],
}
json.dump(rec, open(f'{D}/record.json', 'w', encoding='utf-8'), ensure_ascii=False)
print(json.dumps(mut, indent=1)); print('record ok', SHA)
