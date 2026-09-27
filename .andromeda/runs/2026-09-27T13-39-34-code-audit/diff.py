import json
R = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
BASE = 'a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d'
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
cur = recs[-1]
b = [r for r in recs[:-1] if r['sha'] == BASE][-1]
out = {}
def d(path):
    x, y = b, cur
    for k in path.split('.'):
        x = x.get(k) if isinstance(x, dict) else None
        y = y.get(k) if isinstance(y, dict) else None
    return [x, y]
for p in ['totals.loc', 'totals.files', 'totals.units', 'duplication.pct', 'duplication.clones', 'duplication.duplicated_lines', 'duplication.total_lines',
          'complexity.cyclomatic_p50', 'complexity.cyclomatic_p90', 'complexity.cognitive_p50', 'complexity.cognitive_p90', 'complexity.over_ceiling',
          'sizes.file_p50', 'sizes.file_p90', 'sizes.file_max', 'sizes.over_800', 'graph.cycles', 'graph.cross_unit_edges', 'dead.zero_ref_candidates',
          'coverage.line', 'churn.pct', 'churn.files_churned']:
    out[p] = d(p)
out['dup_split'] = [b['duplication']['split'], cur['duplication']['split']]
bt = {(x[0], x[1]) for x in b['duplication']['top']}
out['dup_top_cur'] = [x + (['standing'] if (x[0], x[1]) in bt else ['new']) for x in cur['duplication']['top']]
out['sizes_top'] = [cur['sizes']['top'], [x[0] for x in cur['sizes']['top'] if x[0] not in {y[0] for y in b['sizes']['top']}]]
out['complexity_top'] = cur['complexity']['top'][:10]
bfi = {x[0] for x in b['graph']['fan_in_top']}
out['fan_in_entrants'] = [x for x in cur['graph']['fan_in_top'] if x[0] not in bfi]
out['fan_out'] = [b['graph']['fan_out'], cur['graph']['fan_out']]
out['unused_deps'] = [b['dead']['unused_deps'], cur['dead']['unused_deps']]
out['mutation'] = {'base_scores': b['mutation']['scores'], 'cur_scores': cur['mutation']['scores'], 'cur_counts': cur['mutation']['counts'], 'survivors': cur['mutation']['survivors']}
out['hotspots'] = cur['hotspots']
out['tool_versions'] = {k: [b['tool_versions'].get(k), cur['tool_versions'].get(k)] for k in set(b['tool_versions']) | set(cur['tool_versions'])}
with open(R + '/c-diff.json', 'w', encoding='utf-8', newline='') as f:
    f.write(json.dumps(out, ensure_ascii=False, indent=1) + '\n')
for k, v in out.items():
    print(k, json.dumps(v, ensure_ascii=False)[:900])
