"""Screen printer for the q-*.json twins: python show.py {health|typed|untyped|facts|chains|prev}."""
import json
import os
import sys

RUN = os.path.dirname(os.path.abspath(__file__))


def load(n):
    with open(os.path.join(RUN, n), encoding='utf-8') as f:
        return json.load(f)


what = sys.argv[1]
if what == 'health':
    h = load('q-health.json')
    for c, v in h['coverage_per_chunk_by_skill'].items():
        print(c, v)
    print('runs', h['step_runs'])
    print('fric', h['friction_per_step'])
    print('untyped', h['untyped_per_step'])
    print('outcomes non-ok', {k: v for k, v in h['outcomes'].items() if not k.endswith('/ok')})
    print('malformed', h['malformed_ts_in_epoch'], 'clause', h['clause_retracted_in_epoch'])
    r = load('q-retractions.json')
    print('retr', len(r['retracted_ids']), 'unres', r['unresolvable'], 'rr', r['retraction_of_retraction'],
          'clause', len(r['clause_retracted']))
    for c, v in h['coverage_per_chunk'].items():
        print(' ', c, v)
elif what == 'typed':
    t = load('q-typed.json')
    sel = sys.argv[2:]  # group keys or 'below'
    for g in t['groups']:
        if sel == ['below']:
            if not g['above']:
                print(f"{g['key']} · n {g['n']} · w {g['weight']} · rate {g['rate']} · {g['chunks']}")
                for c in g['cases']:
                    print('   -', c['chunk'], '|', c['what'][:220])
            continue
        if sel and g['key'] not in sel:
            continue
        if not sel and not g['above']:
            continue
        print(f"## {g['key']} · n {g['n']} · w {g['weight']} · halt {g['halt_or_soft']} · rate {g['rate']} · sites {g.get('sites')}")
        for c in g['cases']:
            imp = {k: v for k, v in c['impact'].items() if v}
            print(f"  - {c['chunk']} · {c['skill']}/{c['step']} · {imp} · {c['id']} · ev {c['evidence']}")
            print('    ', c['what'])
elif what in ('untyped', 'prev'):
    for c in load('q-untyped.json' if what == 'untyped' else 'q-untyped-prev.json'):
        imp = {k: v for k, v in c['impact'].items() if v}
        print(f"- {c['chunk']} · {c['skill']}/{c['step']} · {imp} · {c['id']}")
        print('   ', c['what'])
elif what in ('facts', 'factsprev'):
    sols = sys.argv[2:]
    for f in load('q-facts.json' if what == 'facts' else 'q-facts-prev.json'):
        if sols and f['solution'] not in sols:
            continue
        st = f.get('step') if what == 'factsprev' else f"{f['skill']}/{f['step']}"
        print(f"- {f['chunk']} · {st} · {f['nature']} · {f['solution']} · {f['id']}")
        print('   ', f['note'])
elif what == 'chains':
    c = load('q-chains.json')
    for s in c['shapes']:
        print(s)
    for a in c['anchors']:
        print('-', a['kind'], a['chunk'], a.get('consumer') or a['rec']['step'], a['artifact'], '|',
              (a.get('note') or a.get('rec', {}).get('what') or '')[:200])
        for p in a['producers']:
            print('     <-', p['producer'], p['outcome'], p['signals'], (p['note'] or '')[:150])
