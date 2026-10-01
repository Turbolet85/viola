"""P4 diff + judge (audit-pass.md §Thresholds). Baseline re-found by sha among the records BEFORE this run's own."""
import json

R = '.andromeda/runs/2026-10-01T09-18-50-code-audit'
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
cur = recs[-1]
assert cur['ts'] == json.load(open(f'{R}/record.json', encoding='utf-8'))['ts']
prior = recs[:-1]
base = [r for r in prior if r['sha'] == cur['baseline_sha']][-1]
bb = [r for r in prior if r['sha'] == base['baseline_sha']][-1]
out = {'baseline': base['sha'], 'baseline_of_baseline': bb['sha'], 'rows': [], 'entrants': {}, 'monotonic': {}, 'mutation': {}}

def row(check, metric, a, b, fires, cls):
    out['rows'].append({'check': check, 'metric': metric, 'baseline': a, 'current': b, 'fires': fires, 'class': cls})

c, b = cur, base
row('new-cycle', 'graph.cycles', b['graph']['cycles'], c['graph']['cycles'], c['graph']['cycles'] > b['graph']['cycles'], 'proposal')
dp = c['duplication']['pct'] - b['duplication']['pct']
row('duplication-up', 'duplication.pct', b['duplication']['pct'], c['duplication']['pct'],
    dp >= 0.5 and dp >= 0.15 * b['duplication']['pct'], 'proposal')
oc = c['complexity']['over_ceiling'] - b['complexity']['over_ceiling']
row('complexity-creep', 'complexity.over_ceiling', b['complexity']['over_ceiling'], c['complexity']['over_ceiling'],
    oc >= 3 and (b['complexity']['over_ceiling'] == 0 or oc >= 0.25 * b['complexity']['over_ceiling']), 'proposal')
row('dead-growth', 'dead.zero_ref_candidates', b['dead']['zero_ref_candidates'], c['dead']['zero_ref_candidates'],
    c['dead']['zero_ref_candidates'] >= b['dead']['zero_ref_candidates'] + 5, 'proposal')
row('coverage-drop', 'coverage.line', b['coverage']['line'], c['coverage']['line'], None, 'not evaluable: current null (skip)')

# mutation-drop against the last SCORED value, with this run's corrections applied to the baseline (retraction filter)
corr = {(k['target_sha'], k['field']): k['now'] for k in cur['corrections']}
bscores = dict(b['mutation']['scores']); bscores.update(corr.get((b['sha'], 'mutation.scores'), {}))
for u in cur['mutation']['scoped_units']:
    now = cur['mutation']['scores'].get(u)
    last = bscores.get(u)
    if last is None:  # pass over unscored records to the one before
        last = bb['mutation']['scores'].get(u) if isinstance(bb.get('mutation'), dict) else None
    fires = None if now is None or last is None else now <= last - 10
    out['mutation'][u] = {'last_scored': last, 'now': now, 'fires': fires}
    row('mutation-drop', f'mutation.scores.{u}', last, now, fires, 'proposal' if fires is not None else 'not evaluable')

# monotonic: the closed set of six, worsened at BOTH of the last two diffs (bb->b, b->cur)
SIX = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1), ('sizes.file_max', 1),
       ('sizes.over_800', 1), ('coverage.line', -1)]
get = lambda r, k: (lambda a, f: r[a][f])(*k.split('.'))
for k, sgn in SIX:
    v = [get(bb, k), get(b, k), get(cur, k)]
    if None in v:
        out['monotonic'][k] = {'values': v, 'fires': None, 'note': 'not evaluable: null at one of the three records (coverage skip)'}
    else:
        out['monotonic'][k] = {'values': v, 'fires': (v[1] - v[0]) * sgn > 0 and (v[2] - v[1]) * sgn > 0}

# top-N entrants (informational)
def names(lst, key): return [key(x) for x in lst]
ent = out['entrants']
ent['complexity.top'] = [x for x in cur['complexity']['top'] if x['fn'] + '@' + x['file'].rsplit(':', 1)[0] not in
                         {y['fn'] + '@' + y['file'].rsplit(':', 1)[0] for y in base['complexity']['top']}]
ent['sizes.top'] = [x for x in cur['sizes']['top'] if x[0] not in {y[0] for y in base['sizes']['top']}]
ent['hotspots'] = [x for x in cur['hotspots'] if x[0] not in {y[0] for y in base['hotspots']}]
ent['graph.fan_in_top'] = [x for x in cur['graph']['fan_in_top'] if x[0] not in {y[0] for y in base['graph']['fan_in_top']}]
ent['duplication.top'] = [x for x in cur['duplication']['top'] if (x[0].rsplit(':', 1)[0], x[1].rsplit(':', 1)[0]) not in
                          {(y[0].rsplit(':', 1)[0], y[1].rsplit(':', 1)[0]) for y in base['duplication']['top']}]
# survivors: standing vs new (baseline measured survivors after the not_measured fill; site+text, text matched with or without ` in fn`)
nm_base = {(s, t) for s, t, *_ in corr.get((b['sha'], 'mutation.not_measured'), [])}
base_measured = [tuple(x) for x in b['mutation']['survivors'] if tuple(x) not in nm_base]
out['survivors_baseline_measured'] = base_measured
std = lambda t: t.split(' in ')[0] if ' with ' in t else t
out['survivors_new'] = [s for s in cur['mutation']['survivors'] if (s[0], std(s[1])) not in {(x[0], std(x[1])) for x in base_measured}]
out['graph_edges'] = [b['graph']['cross_unit_edges'], cur['graph']['cross_unit_edges']]
json.dump(out, open(f'{R}/c-judge.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
for r in out['rows']: print(r)
print('monotonic', json.dumps(out['monotonic']))
for k, v in ent.items(): print('entrants', k, json.dumps(v))
print('base measured survivors', base_measured)
print('new survivors', out['survivors_new'])
