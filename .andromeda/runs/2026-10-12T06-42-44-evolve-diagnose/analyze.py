"""Evolve diagnosis pass for one epoch: pre-pass, Stage 0 health, Stage 1 typed groups,
and the raw piles Stages 2-4 read by hand. Writes q-*.json into its own directory."""
import collections
import json
import os
import re
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
LEDGER = '.andromeda/friction-log.ndjson'
TARGET = 'Epoch 4 — Session state & governance'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
UNIVERSAL = {
    'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
    'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
    'contract.skill-reference-drift', 'contract.grammar-irregularity',
    'contract.jointly-contradictory-instructions',
}


def norm(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


def bare(s):
    return re.sub('^andromeda-', '', s or '')


def dump(name, obj):
    with open(os.path.join(RUN, name), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')


# ---- pre-pass (canonical, whole ledger) ----
recs, skips, bad_ts = [], 0, []
with open(LEDGER, encoding='utf-8') as f:
    for line in f:
        try:
            r = json.loads(line)
        except ValueError:
            skips += 1
            continue
        if not isinstance(r, dict):
            skips += 1
            continue
        if not ISO.fullmatch(str(r.get('ts'))):
            bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
        recs.append(r)
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
            unres.append([r.get('id'), 'unknown-id-or-scope', json.dumps(t, ensure_ascii=False)[:160]])
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
dump('q-retractions.json', {
    'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
    'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
    'clause_retracted': clause_notes, 'unresolvable': unres, 'retraction_of_retraction': retr_of_retr,
})


def problems(r):
    p = r.get('problem')
    if p is None:
        return []
    lst = p if isinstance(p, list) else [p]
    out = []
    for i, fact in enumerate(lst):
        if not isinstance(fact, dict):
            continue
        if (r.get('id'), None) in excl_probs or (r.get('id'), i) in excl_probs:
            continue
        out.append((i, fact))
    return out


live = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]

# ---- epoch scan ----
ep = collections.OrderedDict()
for r in recs:
    k = norm(r.get('epoch')) or '(null)'
    e = ep.setdefault(k, {'records': 0, 'step': 0, 'friction': 0, 'chunks': set(), 'spellings': set()})
    e['records'] += 1
    e[r.get('kind') if r.get('kind') in ('step', 'friction') else 'records'] += 0
    if r.get('kind') == 'step':
        e['step'] += 1
    elif r.get('kind') == 'friction':
        e['friction'] += 1
    if r.get('chunk'):
        e['chunks'].add(r['chunk'])
    e['spellings'].add(r.get('epoch'))
epochs = [{'epoch': k, 'records': v['records'], 'step': v['step'], 'friction': v['friction'],
           'chunks': len(v['chunks']), 'spellings': sorted(map(str, v['spellings']))} for k, v in ep.items()]
if '--scan' in sys.argv:
    print(json.dumps(epochs, ensure_ascii=False, indent=1))
    sys.exit(0)

T = norm(TARGET)
E = [r for r in live if norm(r.get('epoch')) == T]
Eall = [r for r in recs if norm(r.get('epoch')) == T]
steps = [r for r in E if r.get('kind') == 'step']
fric = [r for r in E if r.get('kind') == 'friction']

# ---- Stage 0 ----
cov = collections.defaultdict(collections.Counter)
for r in steps:
    cov[r.get('chunk') or '(null)'][bare(r.get('skill')) + '/' + str(r.get('step'))] += 1
cov_skill = {c: dict(collections.Counter(k.split('/')[0] for k in v.elements())) for c, v in cov.items()}
step_runs = collections.Counter((bare(r.get('skill')), r.get('step')) for r in steps)
untyped_by_step = collections.Counter((bare(r.get('skill')), r.get('step')) for r in fric if r.get('untyped') or not r.get('type'))
fric_by_step = collections.Counter((bare(r.get('skill')), r.get('step')) for r in fric)
prob_fill = sum(1 for r in steps if problems(r))
post_id = [r for r in Eall if str(r.get('ts', '')) >= '2026-08-18']
id_fill = sum(1 for r in post_id if r.get('id'))
# signal line
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= '2026-10-09']
Eera = len(steps) - len(K)
J = [r for r in K if r.get('step') == 'curation' and bare(r.get('skill')) == 'wrap-session']
Jids = {id(r) for r in J}
N = M = 0
odd = []
for r in K:
    for p in r.get('produced') or []:
        for s in (p.get('signals') or []) if isinstance(p, dict) else []:
            if not isinstance(s, str):
                continue
            for word in ('guard-refused', 'exact-hit'):
                if s.startswith(word):
                    m = re.fullmatch(word + r':(\d+)', s)
                    if not m:
                        odd.append([r.get('id'), s])
                    elif word == 'guard-refused':
                        N += int(m.group(1))
                    elif id(r) in Jids:
                        M += int(m.group(1))
                    else:
                        odd.append([r.get('id'), s])
outcomes = collections.Counter((bare(r.get('skill')), r.get('step'), r.get('outcome')) for r in steps)
health = {
    'epoch': TARGET, 'records_in_epoch': len(Eall), 'records_live': len(E), 'step': len(steps), 'friction': len(fric),
    'chunks': sorted(c for c in cov if c != '(null)'),
    'coverage_per_chunk': {c: dict(v) for c, v in sorted(cov.items())},
    'coverage_per_chunk_by_skill': cov_skill,
    'step_runs': {f'{a}/{b}': n for (a, b), n in sorted(step_runs.items())},
    'friction_per_step': {f'{a}/{b}': n for (a, b), n in sorted(fric_by_step.items())},
    'untyped_per_step': {f'{a}/{b}': n for (a, b), n in sorted(untyped_by_step.items())},
    'untyped_total': sum(untyped_by_step.values()),
    'problem_fill': [prob_fill, len(steps)], 'id_fill': [id_fill, len(post_id)],
    'unparseable': skips, 'malformed_ts_in_epoch': [b for b in bad_ts if b[0] in cov],
    'signal_line': {'guard_refused': N, 'K': len(K), 'exact_hit': M, 'J': len(J), 'E_era': Eera, 'odd': odd},
    'outcomes': {f'{a}/{b}/{c}': n for (a, b, c), n in sorted(outcomes.items(), key=str)},
    'retracted_in_epoch': [r.get('id') for r in Eall if r.get('id') in excl_ids],
    'clause_retracted_in_epoch': {i: n for i, n in clause_notes.items() if i in {r.get('id') for r in Eall}},
}
dump('q-health.json', health)

# ---- Stage 1 ----
W = lambda imp: 1 + imp.get('iterations', 0) + imp.get('retries', 0) + imp.get('reformulations', 0) \
    + 2 * imp.get('dialogue_rounds', 0) + 3 * imp.get('halted', 0) + 3 * imp.get('soft_exit', 0)


def num(v):
    return v if isinstance(v, (int, float)) else 0


def row(r):
    imp = {k: num(v) for k, v in (r.get('impact') or {}).items()} if isinstance(r.get('impact'), dict) else {}
    what = r.get('what') or ''
    if r.get('id') in clause_notes:
        what += ' [clause retracted: ' + ' | '.join(clause_notes[r['id']]) + ']'
    return {'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': bare(r.get('skill')), 'step': r.get('step'),
            'type': r.get('type'), 'what': what, 'impact': imp, 'weight': W(imp),
            'evidence': r.get('evidence'), 'artifacts': r.get('artifacts'), 'retracts': r.get('retracts')}


def grp(rows, key):
    out = []
    for k, rs in key.items():
        n = len(rs)
        halts = sum(x['impact'].get('halted', 0) + x['impact'].get('soft_exit', 0) for x in rs)
        w = sum(x['weight'] for x in rs)
        sk, st = k[0], k[1]
        runs = step_runs.get((sk, st)) if st != '*' else None
        out.append({'key': '/'.join(k), 'n': n, 'weight': w, 'halt_or_soft': halts,
                    'rate': (f'{n}/{runs}' if runs else None),
                    'above': n >= 3 or (n >= 2 and halts > 0),
                    'chunks': sorted({str(x['chunk']) for x in rs}), 'cases': rs})
    return sorted(out, key=lambda g: (-g['n'] * (g['weight'] / g['n']), g['key']))


typed = [row(r) for r in fric if r.get('type') and not r.get('untyped')]
per_step = collections.defaultdict(list)
cross = collections.defaultdict(list)
for x in typed:
    t = x['type']
    if t in UNIVERSAL or t.startswith('recall.'):
        cross[('*', '*', t)].append(x)
    else:
        per_step[(x['skill'], x['step'], t)].append(x)
groups = grp(typed, per_step) + grp(typed, cross)
groups.sort(key=lambda g: -g['weight'])
gi = collections.Counter()
for g in groups:
    if g['key'].endswith('contract.grammar-irregularity'):
        g['sites'] = dict(collections.Counter(str((c['artifacts'] or ['?'])[0]) for c in g['cases']))
dump('q-typed.json', {'groups': groups, 'above': [g['key'] for g in groups if g['above']]})

# ---- Stage 2 pile ----
untyped = [row(r) for r in fric if r.get('untyped') or not r.get('type')]
dump('q-untyped.json', untyped)
# lookback: previous epoch untyped
prev = 'Epoch 3 — Windows slice II: driving verbs and live proof'
dump('q-untyped-prev.json', [row(r) for r in live if norm(r.get('epoch')) == norm(prev)
                             and r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type'))])

# ---- Stage 3 ----
order = {id(r): i for i, r in enumerate(recs)}
anchors = []
for r in E:
    if r.get('kind') == 'friction' and str(r.get('type') or '').startswith('input.'):
        for a in (r.get('artifacts') or [None]):
            anchors.append({'kind': 'input-friction', 'rec': row(r), 'artifact': a, 'chunk': r.get('chunk'), 'pos': order[id(r)]})
    if r.get('kind') == 'step':
        for c in r.get('consumed') or []:
            if isinstance(c, dict) and c.get('quality') in ('thin', 'wrong', 'missing'):
                anchors.append({'kind': 'consumed-' + c['quality'], 'consumer': f"{bare(r.get('skill'))}/{r.get('step')}",
                                'consumer_id': r.get('id'), 'artifact': c.get('artifact'), 'note': c.get('note'),
                                'signals': c.get('signals'), 'chunk': r.get('chunk'), 'pos': order[id(r)]})
chains = []
for a in anchors:
    prods = []
    for r in E:
        if r.get('kind') != 'step' or r.get('chunk') != a['chunk'] or order[id(r)] >= a['pos']:
            continue
        for p in r.get('produced') or []:
            if isinstance(p, dict) and a['artifact'] and p.get('artifact') == a['artifact']:
                prods.append({'producer': f"{bare(r.get('skill'))}/{r.get('step')}", 'id': r.get('id'),
                              'outcome': r.get('outcome'), 'signals': p.get('signals'), 'note': p.get('note')})
    chains.append({**{k: v for k, v in a.items() if k != 'pos'}, 'producers': prods})
shape = collections.defaultdict(set)
for c in chains:
    for p in c['producers']:
        shape[(p['producer'], c['artifact'], c.get('consumer') or 'friction:' + str(c.get('rec', {}).get('step')))].add(c['chunk'])
dump('q-chains.json', {'anchors': chains,
                       'shapes': [{'shape': list(k), 'chunks': sorted(map(str, v)), 'n': len(v)} for k, v in shape.items()]})

# ---- Stage 4 pile ----
facts = []
for r in E:
    if r.get('kind') != 'step':
        continue
    for i, fct in problems(r):
        facts.append({'id': r.get('id'), 'index': i, 'chunk': r.get('chunk'), 'skill': bare(r.get('skill')),
                      'step': r.get('step'), 'nature': fct.get('nature'), 'solution': fct.get('solution'),
                      'note': fct.get('note')})
dump('q-facts.json', facts)
prevfacts = []
for r in live:
    if r.get('kind') == 'step' and norm(r.get('epoch')) == norm(prev):
        for i, fct in problems(r):
            prevfacts.append({'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{bare(r.get('skill'))}/{r.get('step')}",
                              'nature': fct.get('nature'), 'solution': fct.get('solution'), 'note': fct.get('note')})
dump('q-facts-prev.json', prevfacts)
degraded = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{bare(r.get('skill'))}/{r.get('step')}"}
            for r in steps if r.get('outcome') == 'ok-degraded']
dump('q-degraded.json', degraded)
deferral_open = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{bare(r.get('skill'))}/{r.get('step')}",
                  'artifact': p.get('artifact'), 'note': p.get('note')}
                 for r in steps for p in (r.get('produced') or []) if isinstance(p, dict)
                 and any(isinstance(s, str) and s.startswith('deferral-open') for s in p.get('signals') or [])]
dump('q-deferral-open.json', deferral_open)

print(json.dumps({'health': {k: health[k] for k in ('records_in_epoch', 'records_live', 'step', 'friction', 'untyped_total',
                                                    'problem_fill', 'id_fill', 'unparseable', 'signal_line',
                                                    'retracted_in_epoch')},
                  'groups': len(groups), 'above': [(g['key'], g['n'], g['weight']) for g in groups if g['above']],
                  'untyped': len(untyped), 'anchors': len(chains), 'facts': len(facts),
                  'facts_by_solution': dict(collections.Counter(f['solution'] for f in facts)),
                  'degraded': len(degraded), 'deferral_open': len(deferral_open)}, ensure_ascii=False, indent=1))
