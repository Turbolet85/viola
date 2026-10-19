#!/usr/bin/env python3
"""P4 + P5: diff the appended record against its baseline (re-found by sha), judge, render proposals.md.
usage: render.py <run_dir>   (after P3's append; the record is re-read from record.json, the baseline by sha)"""
import json, sys

R = sys.argv[1]
rec = json.load(open(f'{R}/record.json'))
lines = [l for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
recs = []
for l in lines:
    try: recs.append(json.loads(l))
    except ValueError: pass
me_i = max(i for i, r in enumerate(recs) if r['ts'] == rec['ts'] and r['sha'] == rec['sha'])
base = [r for r in recs[:me_i] if r['sha'] == rec['baseline_sha']][-1]
prev = [r for r in recs[:me_i] if r['sha'] == base['baseline_sha']][-1]
E, B, P = rec['epoch'], base['epoch'], prev['epoch']
short = lambda e: e.split(' — ')[0]
out, info, below, props = [], [], [], []


def get(r, path):
    for k in path.split('.'):
        r = r[k] if r is not None else None
    return r


# monotonic: the six tracked scalars, worse at both of the last two diffs
TRACK = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1),
         ('sizes.file_max', 1), ('sizes.over_800', 1), ('coverage.line', -1)]
mono = {}
for path, sgn in TRACK:
    a, b, c = get(prev, path), get(base, path), get(rec, path)
    if None in (a, b, c):
        below.append(f'`{path}` — not evaluable for monotonic: null at one of {short(P)} / {short(B)} / {short(E)}')
        continue
    worse = lambda x, y: (y - x) * sgn > 0
    mono[path] = (a, b, c, worse(a, b) and worse(b, c))

# --- sizes monotonic proposals
sz_b = dict((f, n) for f, n in base['sizes']['top'])
for path in ('sizes.file_max', 'sizes.over_800'):
    a, b, c, fire = mono[path]
    if not fire:
        below.append(f'`{path}` {a} → {b} → {c} ({short(P)} → {short(B)} → {short(E)}) — not worse at both diffs')
        continue
    props.append((path, a, b, c))

# single-epoch thresholds (span 1)
d_pct = rec['duplication']['pct'] - base['duplication']['pct']
d_rel = d_pct / base['duplication']['pct'] * 100
dup_fire = d_pct >= 0.5 and d_rel >= 15
oc_b, oc_c = base['complexity']['over_ceiling'], rec['complexity']['over_ceiling']
cx_fire = oc_c - oc_b >= 3 and (oc_c - oc_b) / max(oc_b, 1) * 100 >= 25
dead_fire = rec['dead']['zero_ref_candidates'] - base['dead']['zero_ref_candidates'] >= 5
cov_fire = rec['coverage']['line'] <= base['coverage']['line'] - 2
cyc_fire = rec['graph']['cycles'] > base['graph']['cycles']
json.dump({'dup': [d_pct, d_rel, dup_fire], 'cx': [oc_b, oc_c, cx_fire], 'dead': dead_fire, 'cov': cov_fire, 'cyc': cyc_fire,
           'mono': mono}, open(f'{R}/judge.json', 'w'), indent=1)
print(json.dumps({'dup': [round(d_pct, 2), round(d_rel, 1), dup_fire], 'cx': [oc_b, oc_c, cx_fire], 'dead': dead_fire,
                  'cov': cov_fire, 'cyc': cyc_fire, 'mono': {k: v[3] for k, v in mono.items()}}))

# mutation-drop at equal scope
mb, mc = base['mutation'], rec['mutation']
mut_lines = []
for u in mc['scoped_units']:
    sc = mc['scores'].get(u)
    if sc is None:
        mut_lines.append((u, 'unscored', mc['unit_states'].get(u)))
        continue
    scope = mc['timing'][u]['scope']
    last = None
    for r in reversed(recs[:me_i]):
        if u in r.get('mutation', {}).get('scores', {}):
            last = r
            break
    if last is None:
        mut_lines.append((u, 'first', sc, scope))
        continue
    lscope = last['mutation'].get('timing', {}).get(u, {}).get('scope')
    if lscope != scope or (scope == 'files' and last['mutation']['timing'][u].get('files') != mc['timing'][u].get('files')):
        mut_lines.append((u, 'not-comparable', sc, scope, last['mutation']['scores'][u], lscope, last['epoch']))
        continue
    drop = sc <= last['mutation']['scores'][u] - 10
    mut_lines.append((u, 'drop' if drop else 'below', sc, scope, last['mutation']['scores'][u], lscope, last['epoch']))
json.dump(mut_lines, open(f'{R}/judge-mutation.json', 'w'), indent=1)
for m in mut_lines:
    print(m)
