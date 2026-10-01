import json, re, collections, os, sys

ROOT = r'D:\dev\projects\viola'
RUN = os.path.join(ROOT, '.andromeda', 'runs', '2026-10-01T09-05-15-evolve-diagnose')
LEDGER = os.path.join(ROOT, '.andromeda', 'friction-log.ndjson')
TARGET = 'Epoch 2b — Windows slice I b: events and ledger'
PREV = 'Epoch 2 — Windows slice I: wrapper, events, ledger'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')


def norm(s):
    s = re.sub(r'^\s*#+\s*', '', s or '').strip()
    return re.sub('[\u2013\u2014]', '-', s)


def bare(s):
    s = s or ''
    return s[len('andromeda-'):] if s.startswith('andromeda-') else s


def save(name, obj):
    with open(os.path.join(RUN, name), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')


recs, skips, bad_ts = [], 0, []
with open(LEDGER, encoding='utf-8') as f:
    for i, line in enumerate(f, 1):
        try:
            r = json.loads(line)
            if not isinstance(r, dict):
                raise ValueError
        except ValueError:
            skips += 1
            continue
        r['_line'] = i
        r['skill'] = bare(r.get('skill'))
        if not ISO.fullmatch(str(r.get('ts'))):
            bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
        recs.append(r)

# ---- retraction pre-pass (whole ledger)
by_id = {r['id']: r for r in recs if r.get('id')}
excl_ids, excl_probs, unres, clause_notes = set(), set(), [], {}
for r in recs:
    rl = r.get('retracts')
    if not rl:
        continue
    for t in (rl if isinstance(rl, list) else [rl]):
        if not isinstance(t, dict):
            unres.append([r.get('id'), 'prose-form-pre-boundary', str(t)[:80]])
            continue
        tid, scope = t.get('id'), t.get('scope')
        tgt = by_id.get(tid)
        if tid is None or tgt is None or scope not in ('record', 'problem', 'clause'):
            unres.append([r.get('id'), 'unknown-id-or-scope', json.dumps(t, ensure_ascii=False)[:200]])
            continue
        if scope == 'record':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'record-scope-on-step', tid])
                continue
            excl_ids.add(tid)
        elif scope == 'clause':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'clause-scope-on-step', tid])
                continue
            if not t.get('note'):
                unres.append([r.get('id'), 'clause-without-note', tid])
                continue
            clause_notes.setdefault(tid, []).append(t['note'])
        else:
            excl_probs.add((tid, t.get('index')))
retr_of_retr = sorted(r['id'] for r in recs if r.get('retracts') and r.get('id') in excl_ids)
save('q-retractions.json', {'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
     'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
     'clause_retracted': clause_notes, 'unresolvable': unres, 'retraction_of_retraction': retr_of_retr})

filt = [r for r in recs if r.get('id') not in excl_ids]


def problems(r):
    p = r.get('problem')
    if p is None:
        return []
    lst = p if isinstance(p, list) else [p]
    out = []
    for i, x in enumerate(lst):
        if (r.get('id'), None) in excl_probs or (r.get('id'), i) in excl_probs:
            continue
        if isinstance(x, dict):
            out.append((i, x))
    return out


ep = [r for r in filt if norm(r.get('epoch')) == norm(TARGET)]
prev = [r for r in filt if norm(r.get('epoch')) == norm(PREV)]
steps = [r for r in ep if r.get('kind') == 'step']
fric = [r for r in ep if r.get('kind') == 'friction']

# ---- Stage 0
expected = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(lambda: collections.defaultdict(list))
for r in steps:
    cov[r.get('chunk')][r['skill']].append(r.get('step'))
step_runs = collections.Counter((r['skill'], r.get('step')) for r in steps)
untyped_rate = {}
for (sk, st), n in step_runs.items():
    u = sum(1 for r in fric if r['skill'] == sk and r.get('step') == st and r.get('untyped'))
    t = sum(1 for r in fric if r['skill'] == sk and r.get('step') == st)
    untyped_rate[f'{sk}/{st}'] = {'step_runs': n, 'friction': t, 'untyped': u}
prob_fill = sum(1 for r in steps if problems(r))
outcomes = collections.Counter(r.get('outcome') for r in steps)
save('q-health.json', {
    'epoch': TARGET, 'records': len(ep), 'step': len(steps), 'friction': len(fric),
    'coverage': {str(c): {sk: sorted(v) for sk, v in d.items()} for c, d in cov.items()},
    'expected_per_chunk': expected, 'unparseable': skips,
    'malformed_ts_in_epoch': [b for b in bad_ts],
    'untyped_rate': untyped_rate, 'problem_fill': [prob_fill, len(steps)],
    'id_fill': [sum(1 for r in ep if r.get('id')), len(ep)], 'outcomes': dict(outcomes),
    'excluded_by_retraction_in_epoch': [r['id'] for r in recs if r.get('id') in excl_ids and norm(r.get('epoch')) == norm(TARGET)],
})

# ---- Stage 1
UNIV = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
        'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
        'contract.skill-reference-drift', 'contract.grammar-irregularity',
        'contract.jointly-contradictory-instructions'}
KEYS = ['iterations', 'retries', 'reformulations', 'dialogue_rounds', 'extra_reads', 'halted', 'soft_exit', 'deferred']


def weight(imp):
    imp = imp or {}
    g = lambda k: imp.get(k, 0) or 0
    return 1 + g('iterations') + g('retries') + g('reformulations') + 2 * g('dialogue_rounds') + 3 * g('halted') + 3 * g('soft_exit')


def group(key_fn):
    g = collections.defaultdict(list)
    for r in fric:
        if r.get('untyped') or not r.get('type'):
            continue
        k = key_fn(r)
        if k:
            g[k].append(r)
    out = []
    for k, rs in g.items():
        imp = {kk: sum((r.get('impact') or {}).get(kk, 0) or 0 for r in rs) for kk in KEYS}
        w = sum(weight(r.get('impact')) for r in rs)
        haltish = any(((r.get('impact') or {}).get('halted') or (r.get('impact') or {}).get('soft_exit')) for r in rs)
        n = len(rs)
        out.append({'key': k, 'n': n, 'weight': w, 'impact': imp,
                    'above': n >= 3 or (n >= 2 and haltish),
                    'chunks': sorted({str(r.get('chunk')) for r in rs}),
                    'cases': [{'id': r.get('id'), 'line': r['_line'], 'chunk': r.get('chunk'),
                               'skill': r['skill'], 'step': r.get('step'),
                               'what': r.get('what'), 'impact': r.get('impact'),
                               'evidence': r.get('evidence'), 'artifacts': r.get('artifacts'),
                               'clause_retracted': clause_notes.get(r.get('id'))} for r in rs]})
    return sorted(out, key=lambda x: -x['n'] * x['weight'])


per_step = group(lambda r: f"{r['skill']}/{r.get('step')}/{r['type']}")
by_type = group(lambda r: r['type'] if (r['type'] in UNIV or r['type'].startswith('recall.')) else None)
all_type = group(lambda r: r['type'])
for g in per_step:
    sk, st, _ = g['key'].split('/', 2)
    g['rate'] = f"{g['n']}/{step_runs.get((sk, st), 0)}"
save('q-typed.json', {'per_step': per_step, 'universal_and_recall_by_type': by_type, 'all_by_type': all_type})

# ---- Stage 2 dump
unt = [{'id': r.get('id'), 'line': r['_line'], 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r.get('step'),
        'what': r.get('what'), 'impact': r.get('impact'), 'evidence': r.get('evidence')}
       for r in fric if r.get('untyped') or not r.get('type')]
unt_prev = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r.get('step'), 'what': r.get('what')}
            for r in prev if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type'))]
save('q-untyped-raw.json', {'target': unt, 'previous_epoch': unt_prev})

# ---- Stage 3
chains = []
for r in ep:
    anchors = []
    if r.get('kind') == 'friction' and str(r.get('type') or '').startswith('input.'):
        anchors += [a for a in (r.get('artifacts') or [])]
    if r.get('kind') == 'step':
        anchors += [c.get('artifact') for c in (r.get('consumed') or []) if isinstance(c, dict) and c.get('quality') in ('thin', 'wrong', 'missing')]
    for a in anchors:
        prods = [p for p in steps if p.get('chunk') == r.get('chunk') and p['_line'] < r['_line']
                 and any(isinstance(x, dict) and x.get('artifact') == a for x in (p.get('produced') or []))]
        prod = prods[-1] if prods else None
        q = None
        if r.get('kind') == 'step':
            q = [c for c in r.get('consumed') if isinstance(c, dict) and c.get('artifact') == a]
        chains.append({'chunk': r.get('chunk'), 'artifact': a,
                       'consumer': f"{r['skill']}/{r.get('step')}", 'consumer_id': r.get('id'),
                       'consumer_kind': r.get('kind'), 'consumer_verdict': q,
                       'consumer_what': r.get('what'),
                       'producer': f"{prod['skill']}/{prod.get('step')}" if prod else None,
                       'producer_id': prod.get('id') if prod else None,
                       'producer_outcome': prod.get('outcome') if prod else None,
                       'producer_signals': [x for x in prod.get('produced') if isinstance(x, dict) and x.get('artifact') == a] if prod else None})
shape = collections.defaultdict(set)
for c in chains:
    if c['producer']:
        shape[(c['producer'], c['artifact'], c['consumer'])].add(str(c['chunk']))
agg = [{'shape': list(k), 'chunks': sorted(v), 'k': len(v)} for k, v in shape.items()]
save('q-chains.json', {'anchors': chains, 'shapes': sorted(agg, key=lambda x: -x['k'])})

# ---- Stage 4 raw facts
facts = []
for r in steps:
    for i, x in problems(r):
        facts.append({'id': r.get('id'), 'index': i, 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r.get('step'),
                      'nature': x.get('nature'), 'solution': x.get('solution'), 'note': x.get('note')})
prev_facts = []
for r in prev:
    if r.get('kind') != 'step':
        continue
    for i, x in problems(r):
        prev_facts.append({'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r.get('step'),
                           'nature': x.get('nature'), 'solution': x.get('solution'), 'note': x.get('note')})
degr = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r.get('step')} for r in steps if r.get('outcome') == 'ok-degraded']
save('q-level-raw.json', {'facts': facts, 'previous_epoch_facts': prev_facts, 'ok_degraded': degr})

# ---- console summary
print('retractions: excl', len(excl_ids), 'clause', len(clause_notes), 'unres', len(unres), 'r-of-r', len(retr_of_retr))
print('epoch', len(ep), 'step', len(steps), 'friction', len(fric), 'malformed_ts total', len(bad_ts))
print('outcomes', dict(outcomes), 'problem fill', prob_fill, '/', len(steps))
print('coverage:')
for c, d in sorted(cov.items(), key=lambda x: str(x[0])):
    print(' ', c, {sk: len(v) for sk, v in d.items()})
print('untyped rates:')
for k, v in sorted(untyped_rate.items()):
    print(' ', k, v)
print('per-step groups', len(per_step), 'above', sum(g['above'] for g in per_step))
for g in per_step:
    print('  ', 'A' if g['above'] else '-', g['key'], 'n', g['n'], 'w', g['weight'], g['rate'])
print('universal/recall by type:')
for g in by_type:
    print('  ', 'A' if g['above'] else '-', g['key'], 'n', g['n'], 'w', g['weight'], 'chunks', len(g['chunks']))
print('untyped target', len(unt), 'prev', len(unt_prev))
print('chain anchors', len(chains), 'shapes k>=2', [a for a in agg if a['k'] >= 2])
print('facts', len(facts), collections.Counter((f['solution'], f['nature']) for f in facts))
print('prev facts', len(prev_facts), 'ok-degraded', len(degr))
