"""Evolve diagnosis pass for one epoch: pre-pass, Stage 0-4 queries, q-*.json twins.

Run from the repository root:
    python -X utf8 .andromeda/runs/<run>/analyse.py <run_dir> "<epoch label>" "<previous epoch label>"
"""
import collections
import json
import re
import sys

RUN, TARGET, PREV = sys.argv[1], sys.argv[2], sys.argv[3]
LEDGER = '.andromeda/friction-log.ndjson'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
UNIVERSAL = {
    'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
    'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
    'contract.skill-reference-drift', 'contract.grammar-irregularity',
    'contract.jointly-contradictory-instructions',
}
IMPACT_KEYS = ['iterations', 'retries', 'reformulations', 'dialogue_rounds', 'extra_reads',
               'halted', 'soft_exit', 'deferred']


def norm_epoch(s):
    s = re.sub(r'^\s*#+\s*', '', s or '').strip()
    return s.replace('\u2013', '-').replace('\u2014', '-')


def fold_skill(s):
    return re.sub(r'^andromeda-', '', s or '')


def dump(name, obj):
    with open(f'{RUN}/{name}', 'w', encoding='utf-8', newline='\n') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')


def as_list(p):
    if p is None:
        return []
    return p if isinstance(p, list) else [p]


def num(v):
    return v if isinstance(v, (int, float)) and not isinstance(v, bool) else 0


def weight(imp):
    imp = imp or {}
    return (1 + num(imp.get('iterations')) + num(imp.get('retries')) + num(imp.get('reformulations'))
            + 2 * num(imp.get('dialogue_rounds')) + 3 * num(imp.get('halted')) + 3 * num(imp.get('soft_exit')))


def halts(imp):
    imp = imp or {}
    return num(imp.get('halted')) + num(imp.get('soft_exit'))


# ---- pre-pass over the WHOLE ledger
recs, skips, bad_ts = [], 0, []
for line in open(LEDGER, encoding='utf-8'):
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
    r['_skill'] = fold_skill(r.get('skill'))
    r['_epoch'] = norm_epoch(r.get('epoch'))
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
    """A step record's problem facts, the retracted ones dropped."""
    out = []
    for i, p in enumerate(as_list(r.get('problem'))):
        if (r.get('id'), None) in excl_probs or (r.get('id'), i) in excl_probs:
            continue
        if isinstance(p, dict):
            out.append(p)
    return out


live = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]

# ---- epoch scan (Setup)
scan = collections.OrderedDict()
for r in recs:
    e = r['_epoch'] or '(null)'
    s = scan.setdefault(e, {'records': 0, 'step': 0, 'friction': 0, 'chunks': set()})
    s['records'] += 1
    s[r.get('kind') if r.get('kind') in ('step', 'friction') else 'records'] += 0
    if r.get('kind') == 'step':
        s['step'] += 1
    elif r.get('kind') == 'friction':
        s['friction'] += 1
    if r.get('chunk'):
        s['chunks'].add(r['chunk'])
dump('q-epochs.json', {e: {**v, 'chunks': sorted(v['chunks'])} for e, v in scan.items()})

T, P = norm_epoch(TARGET), norm_epoch(PREV)
ep = [r for r in live if r['_epoch'] == T]
ep_all = [r for r in recs if r['_epoch'] == T]
prev = [r for r in live if r['_epoch'] == P]
steps = [r for r in ep if r.get('kind') == 'step']
fr = [r for r in ep if r.get('kind') == 'friction']

# ---- Stage 0
expected = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(lambda: collections.defaultdict(list))
for r in steps:
    cov[r.get('chunk') or '(null)'][r['_skill']].append(r.get('step'))
coverage = {}
for c, by in sorted(cov.items()):
    row = {}
    for sk, st in sorted(by.items()):
        row[sk] = {'n': len(st), 'expected': expected.get(sk), 'steps': st}
    coverage[c] = row
step_runs = collections.Counter((r['_skill'], r.get('step')) for r in steps)
untyped = collections.Counter((r['_skill'], r.get('step')) for r in fr if r.get('untyped') or not r.get('type'))
fr_by_step = collections.Counter((r['_skill'], r.get('step')) for r in fr)
untyped_rate = {f'{k[0]}/{k[1]}': {'step_runs': step_runs.get(k, 0), 'friction': fr_by_step.get(k, 0),
                                   'untyped': untyped.get(k, 0)}
                for k in sorted(set(step_runs) | set(fr_by_step), key=str)}
prob_fill = sum(1 for r in steps if problems(r))
id_fill = sum(1 for r in ep_all if r.get('id'))
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= '2026-10-09']
E = len(steps) - len(K)
N = M = 0
J = 0
odd = []
for r in K:
    is_cur = r['_skill'] == 'wrap-session' and r.get('step') == 'curation'
    J += is_cur
    for pr in r.get('produced') or []:
        for s in (pr.get('signals') or []) if isinstance(pr, dict) else []:
            if not isinstance(s, str):
                continue
            for word in ('guard-refused', 'exact-hit'):
                if s.startswith(word):
                    m = re.fullmatch(word + r':(\d+)', s)
                    if not m or (word == 'exact-hit' and not is_cur):
                        odd.append([r.get('id'), s])
                    elif word == 'guard-refused':
                        N += int(m.group(1))
                    else:
                        M += int(m.group(1))
outcomes = collections.Counter((r['_skill'], r.get('step'), r.get('outcome')) for r in steps)
dump('q-health.json', {
    'epoch': TARGET, 'records': len(ep_all), 'after_retractions': len(ep),
    'step': len(steps), 'friction': len(fr), 'unparseable_whole_ledger': skips,
    'malformed_ts_in_epoch': [b for b in bad_ts if b[0] in cov],
    'coverage': coverage, 'untyped_rate': untyped_rate,
    'problem_fill': [prob_fill, len(steps)], 'id_fill': [id_fill, len(ep_all)],
    'signal_line': {'guard_refused': N, 'K': len(K), 'exact_hit': M, 'J': J, 'E': E, 'odd': odd},
    'outcomes': {'/'.join(map(str, k)): v for k, v in sorted(outcomes.items(), key=str)},
    'retracted_in_epoch': sorted(i for i in excl_ids if by_id[i]['_epoch'] == T),
    'clause_retracted_in_epoch': {i: n for i, n in clause_notes.items() if by_id[i]['_epoch'] == T},
})


# ---- Stage 1
def cite(r):
    return {'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['_skill'], 'step': r.get('step'),
            'type': r.get('type'), 'what': r.get('what'), 'impact': r.get('impact'),
            'evidence': r.get('evidence'), 'artifacts': r.get('artifacts'),
            'clause_retracted': clause_notes.get(r.get('id')), 'retracts': r.get('retracts')}


groups = collections.defaultdict(list)
for r in fr:
    t = r.get('type')
    if not t or r.get('untyped'):
        continue
    if t in UNIVERSAL or t.startswith('recall.'):
        groups[('*', '*', t)].append(r)
    else:
        groups[(r['_skill'], r.get('step'), t)].append(r)
typed = []
for k, rs in groups.items():
    n = len(rs)
    h = sum(1 for r in rs if halts(r.get('impact')))
    w = sum(weight(r.get('impact')) for r in rs)
    tot = collections.Counter()
    for r in rs:
        for ik in IMPACT_KEYS:
            tot[ik] += num((r.get('impact') or {}).get(ik))
    runs = step_runs.get((k[0], k[1])) if k[0] != '*' else None
    entry = {'key': '/'.join(map(str, k)), 'n': n, 'weight': w, 'with_halt': h,
             'rate': f'{n}/{runs}' if runs else None,
             'above': n >= 3 or (n >= 2 and h >= 1),
             'chunks': sorted({r.get('chunk') or '(null)' for r in rs}),
             'impact': dict(tot), 'cases': [cite(r) for r in rs]}
    if k[2] == 'contract.grammar-irregularity':
        entry['sites'] = collections.Counter(str((r.get('artifacts') or ['?'])[0]) for r in rs)
    typed.append(entry)
typed.sort(key=lambda g: (-g['n'] * g['weight'], g['key']))
dump('q-typed.json', typed)

# ---- Stage 2
unt = [cite(r) for r in fr if r.get('untyped') or not r.get('type')]
unt_prev = [cite(r) for r in prev if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type'))]
dump('q-untyped.json', {'target': unt, 'previous': unt_prev})

# ---- Stage 3
anchors = []
for r in ep:
    if r.get('kind') == 'friction' and str(r.get('type') or '').startswith('input.'):
        anchors.append((r, None))
    if r.get('kind') == 'step':
        for c in r.get('consumed') or []:
            if isinstance(c, dict) and c.get('quality') in ('thin', 'wrong', 'missing'):
                anchors.append((r, c))
order = {id(r): i for i, r in enumerate(recs)}
chains, shapes = [], collections.defaultdict(set)
for r, c in anchors:
    arts = [c['artifact']] if c else [a for a in (r.get('artifacts') or []) if isinstance(a, str)]
    if not c:
        arts.append(re.sub(r'^input\.', '', r.get('type')).split('-')[0])
    for art in arts:
        prods = [p for p in ep if p.get('kind') == 'step' and p.get('chunk') == r.get('chunk')
                 and order[id(p)] < order[id(r)]
                 and any(isinstance(x, dict) and x.get('artifact') == art for x in p.get('produced') or [])]
        if not prods:
            continue
        p = prods[-1]
        sig = [x.get('signals') for x in p.get('produced') or [] if isinstance(x, dict) and x.get('artifact') == art]
        pf = [cite(f) for f in ep if f.get('kind') == 'friction' and f.get('chunk') == p.get('chunk')
              and f['_skill'] == p['_skill'] and f.get('step') == p.get('step')]
        shape = (f"{p['_skill']}/{p.get('step')}", art, f"{r['_skill']}/{r.get('step')}")
        shapes[shape].add(r.get('chunk'))
        chains.append({'shape': shape, 'chunk': r.get('chunk'),
                       'consumer': {'id': r.get('id'), 'kind': r.get('kind'), 'type': r.get('type'),
                                    'what': r.get('what'), 'grade': c},
                       'producer': {'id': p.get('id'), 'outcome': p.get('outcome'), 'signals': sig,
                                    'frictions': pf}})
dump('q-chains.json', {'anchors': len(anchors), 'chains': chains,
                       'shapes': [{'shape': list(k), 'chunks': sorted(v, key=str), 'n': len(v)}
                                  for k, v in sorted(shapes.items(), key=lambda kv: -len(kv[1]))]})


# ---- Stage 4
def facts(rs):
    out = []
    for r in rs:
        if r.get('kind') != 'step':
            continue
        for p in problems(r):
            out.append({'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['_skill'], 'step': r.get('step'),
                        'nature': p.get('nature'), 'solution': p.get('solution'), 'note': p.get('note')})
    return out


ft, fp = facts(ep), facts(prev)
degr = collections.Counter((r['_skill'], r.get('step')) for r in steps if r.get('outcome') == 'ok-degraded')
degr_prev = collections.Counter((r['_skill'], r.get('step')) for r in prev
                                if r.get('kind') == 'step' and r.get('outcome') == 'ok-degraded')
tool_t = collections.Counter(r.get('type') for r in fr if str(r.get('type') or '').startswith('tooling.'))
tool_p = collections.Counter(r.get('type') for r in prev
                             if r.get('kind') == 'friction' and str(r.get('type') or '').startswith('tooling.'))
dump('q-level-facts.json', {
    'target': ft, 'previous': fp,
    'solutions': collections.Counter(f['solution'] for f in ft),
    'ok_degraded': {'/'.join(map(str, k)): v for k, v in degr.items()},
    'ok_degraded_prev': {'/'.join(map(str, k)): v for k, v in degr_prev.items()},
    'tooling_types': tool_t, 'tooling_types_prev': tool_p,
})

print(f'ledger {len(recs)} parsed · {skips} unparseable · {len(bad_ts)} malformed ts')
print(f'retracted {len(excl_ids)} ({len(unres)} unresolvable) · clause {len(clause_notes)} · r-of-r {len(retr_of_retr)}')
for e, v in scan.items():
    print(f'  epoch {e!r}: {v["records"]} ({v["step"]} step / {v["friction"]} friction) · {len(v["chunks"])} chunks')
print(f'target {T}: {len(ep_all)} records · {len(steps)} step / {len(fr)} friction after retractions · '
      f'{len([c for c in cov if c != "(null)"])} chunks')
print(f'typed groups {len(typed)} · above {sum(g["above"] for g in typed)}')
print(f'untyped {len(unt)} (prev {len(unt_prev)}) · anchors {len(anchors)} · chains {len(chains)} · '
      f'shapes>=2 {sum(1 for v in shapes.values() if len(v) >= 2)}')
print(f'facts {len(ft)} (prev {len(fp)}) · solutions {dict(collections.Counter(f["solution"] for f in ft))}')
