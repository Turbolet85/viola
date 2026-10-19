"""Compact screens over the q-*.json twins: view.py <run_dir> <health|typed|untyped|chains|facts> [arg]."""
import json
import sys

RUN, WHAT = sys.argv[1], sys.argv[2]
ARG = sys.argv[3] if len(sys.argv) > 3 else None


def load(n):
    return json.load(open(f'{RUN}/{n}', encoding='utf-8'))


def imp(i):
    return ','.join(f'{k[:4]}{v}' for k, v in (i or {}).items() if v) or '-'


if WHAT == 'health':
    h = load('q-health.json')
    for c, row in h['coverage'].items():
        print(c, ' · '.join(f"{sk} {v['n']}/{v['expected']}" + ('' if v['expected'] in (None, v['n']) else ' ' + ','.join(map(str, v['steps'])))
                            for sk, v in row.items()))
    print('untyped rate:', {k: f"{v['untyped']}/{v['friction']} of {v['step_runs']} runs" for k, v in h['untyped_rate'].items() if v['friction']})
    print('problem fill', h['problem_fill'], 'id fill', h['id_fill'])
    print('signal', h['signal_line'])
    print('outcomes non-ok:', {k: v for k, v in h['outcomes'].items() if not k.endswith('/ok')})
    print('retracted in epoch', h['retracted_in_epoch'], 'clause', h['clause_retracted_in_epoch'])
    print('malformed', h['malformed_ts_in_epoch'])
    r = load('q-retractions.json')
    print('unresolvable', r['unresolvable'])
elif WHAT == 'typed':
    for g in load('q-typed.json'):
        if (ARG == 'above') != g['above']:
            continue
        print(f"== {g['key']} n={g['n']} w={g['weight']} halt={g['with_halt']} rate={g['rate']} chunks={len(g['chunks'])} sites={g.get('sites')}")
        if ARG == 'above':
            for c in g['cases']:
                cr = f" [clause retracted: {c['clause_retracted']}]" if c['clause_retracted'] else ''
                print(f"  - {c['id']} | {c['chunk']} | {c['skill']}/{c['step']} | {imp(c['impact'])} | {c['what']}{cr} | ev={c['evidence']}")
elif WHAT == 'untyped':
    u = load('q-untyped.json')
    for side in ('target', 'previous'):
        print('##', side)
        for c in u[side]:
            print(f"  - {c['id']} | {c['chunk']} | {c['skill']}/{c['step']} | {imp(c['impact'])} | {c['what']}")
elif WHAT == 'chains':
    ch = load('q-chains.json')
    for s in ch['shapes']:
        print(s['n'], ' → '.join(s['shape']), s['chunks'])
    if ARG:
        for c in ch['chains']:
            if ' → '.join(c['shape']) == ARG or ARG == 'all':
                p = c['producer']
                print(f"  {c['chunk']} | consumer {c['consumer']['id']} {c['consumer']['type'] or ''} grade={c['consumer']['grade']} what={c['consumer']['what']}")
                print(f"     producer {p['id']} {p['outcome']} signals={p['signals']} frictions={[f['type'] for f in p['frictions']]}")
elif WHAT == 'facts':
    f = load('q-level-facts.json')
    for side in ('target', 'previous'):
        if ARG and side != ARG:
            continue
        print('##', side)
        for x in f[side]:
            print(f"  - {x['id']} | {x['chunk']} | {x['skill']}/{x['step']} | {x['nature']}/{x['solution']} | {x['note']}")
    print('ok-degraded', f['ok_degraded'], 'prev', f['ok_degraded_prev'])
    print('tooling', f['tooling_types'], 'prev', f['tooling_types_prev'])
