import json, os, sys
R = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
U = sys.argv[1]
D = f'{R}/mutants-{U}/mutants.out'
o = json.load(open(D + '/outcomes.json', encoding='utf-8'))
mj = json.load(open(D + '/mutants.json', encoding='utf-8')) if os.path.exists(D + '/mutants.json') else None
planned = len(mj) if mj is not None else None
complete = bool(o.get('end_time')) and (planned is None or o.get('total_mutants') == planned)
outs = o['outcomes']
base = outs[0]
c = {'mutants': 0, 'caught': 0, 'missed': 0, 'timeout': 0, 'unviable': 0}
surv = []
for x in outs[1:]:
    s = x.get('summary')
    c['mutants'] += 1
    k = {'CaughtMutant': 'caught', 'MissedMutant': 'missed', 'Timeout': 'timeout', 'Unviable': 'unviable'}.get(s)
    if k:
        c[k] += 1
    if s == 'MissedMutant':
        m = x['scenario']['Mutant']
        st = m['span']['start']
        fn = (m.get('function') or {}).get('function_name')
        name = m.get('name') or ''
        # the tool's replacement text: the name minus its "file:line:col: " site and its " in {function}" tail
        txt = name.split(': ', 1)[1] if ': ' in name else name
        if fn and txt.endswith(' in ' + fn):
            txt = txt[: -len(' in ' + fn)]
        surv.append([f"{m['file'].replace(chr(92), '/')}:{st['line']}:{st['column']}", txt, fn])
tested = c['caught'] + c['missed'] + c['timeout'] + c['unviable']
state = f"complete {c['mutants']}/{planned}" if complete else 'INCOMPLETE'
res = {'unit': U, 'complete': complete, 'end_time': o.get('end_time'), 'total_mutants': o.get('total_mutants'), 'mutants_json_len': planned,
       'baseline_summary': base.get('summary'), 'counts': c,
       'unit_state': f"{o.get('total_mutants')}/{planned}", 'score_formula': 'caught/(caught+missed)'}
keys = [(s[0], s[1]) for s in surv]
res['survivor_rows_unique'] = len(keys) == len(set(keys))
res['survivors_equal_missed'] = len(surv) == c['missed']
if complete and c['unviable'] > c['caught'] + c['missed'] + c['timeout']:
    res['state'] = f"unviable-dominant {c['unviable']}/{tested}"; res['score'] = None
elif complete:
    res['state'] = state
    res['score'] = round(100.0 * c['caught'] / (c['caught'] + c['missed']), 2) if (c['caught'] + c['missed']) else None
else:
    res['state'] = 'INCOMPLETE'; res['score'] = None
res['survivors'] = surv
if complete:
    with open(f'{R}/c-mutation-{U}.json', 'w', encoding='utf-8', newline='') as f:
        f.write(json.dumps(res, ensure_ascii=False, indent=1) + '\n')
print(json.dumps({k: res[k] for k in ('unit', 'state', 'score', 'counts', 'baseline_summary', 'survivor_rows_unique', 'survivors_equal_missed')}))
for s in surv:
    print('  ', s)
