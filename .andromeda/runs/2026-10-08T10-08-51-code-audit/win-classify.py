"""Route CARRY (windows-mutants survivors and their cfg(unix) twins): every mutant the Windows leg graded MISSED
(run 37761947926 at e304994, win-outcomes.json) classified against two independent readings:
  - the pinned cover() recipe under the WINDOWS runner's cfg set (rustc --print cfg --target x86_64-pc-windows-msvc):
    a covering predicate proven false there means the runner never built the span;
  - this audit's own Linux grade of the same mutant (same sha, so the tool's name is the join key).
Report-only: writes carry-windows.json; nothing here enters the ledger's counts.
python3 -B win-classify.py"""
import collections, json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cover import cover, host_cfg

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
JOB = {'pty': 'viola-pty', 'channel': 'viola-channel', 'state': 'viola-state', 'agent-claude': 'viola-agent-claude',
       'viola': 'viola', 'e2e': 'viola-e2e'}
# where each unit's Linux outcomes live (viola-e2e: the operator's report-only deselected run)
LINUX = {u: f'{R}/mutants-{u}/mutants.out' for u in JOB.values()}
LINUX['viola-e2e'] = f'{R}/mutants-viola-e2e-deselected/mutants.out'
win = json.load(open(f'{R}/win-outcomes.json', encoding='utf-8'))
wcfg, lcfg = host_cfg('.', 'x86_64-pc-windows-msvc'), host_cfg('.')
GRADE = {'CaughtMutant': 'caught', 'MissedMutant': 'missed', 'Unviable': 'unviable', 'Timeout': 'timeout'}

rows, per_job = [], {}
for job, unit in JOB.items():
    j = win[job]
    listed, linux, complete = {}, {}, False
    try:
        for m in json.load(open(f'{LINUX[unit]}/mutants.json', encoding='utf-8')):
            listed[m['name']] = m
        oc = json.load(open(f'{LINUX[unit]}/outcomes.json', encoding='utf-8'))
        complete = bool(oc.get('end_time'))
        for o in oc['outcomes'][1:]:
            linux[o['scenario']['Mutant']['name']] = GRADE[o['summary']]
    except OSError:
        pass
    graded = {r['name'] for r in j['rows']}
    planned = int(j['found'].split()[1]) if j['found'] else None
    per_job[job] = {'unit': unit, 'found': planned, 'graded': len(graded), 'tally': j['tally'],
                    'harness_ok': (j['doc'] or {}).get('ok'), 'harness_reason': (j['doc'] or {}).get('reason'),
                    'linux_outcomes_complete': complete}
    for r in j['rows']:
        if r['grade'] != 'MISSED':
            continue
        name = r['name']
        m = listed.get(name)
        site, text = name.split(': ', 1)
        wp = cover('.', m, wcfg) if m else None
        lp = cover('.', m, lcfg) if m else None
        lg = linux.get(name)
        lin = (f'not measured on Linux: cfg({lp})' if lp is not None else lg) or \
              ('pending' if not complete else 'absent from the Linux listing')
        if m is None:
            klass = 'unjoined (no Linux listing for this name)'
        elif wp is not None and lp is not None:
            klass = 'excluded on both hosts'
        elif wp is not None:
            klass = 'cfg twin: the Windows build excludes the span'
        elif lp is not None:
            klass = 'Windows-only body, missed on Windows'
        elif lg == 'caught':
            klass = 'shared body: missed on Windows, caught on Linux'
        elif lg == 'missed':
            klass = 'shared body: missed on both hosts'
        else:
            klass = f'shared body: missed on Windows, Linux {lin}'
        rows.append({'job': job, 'site': site, 'mutation': text, 'windows_cover': f'cfg({wp})' if wp else None,
                     'linux': lin, 'class': klass})

keys = [(r['site'], r['mutation']) for r in rows]
assert len(keys) == len(set(keys)), 'rows not unique on (site, mutation)'
n_missed = sum(v['tally'].get('MISSED', 0) for v in per_job.values())
assert len(rows) == n_missed, (len(rows), n_missed)
by = collections.Counter(r['class'] for r in rows)
json.dump({'run': 37761947926, 'sha': 'e304994ae0413c5cf5bf679a5b3d19abded0e472', 'jobs': per_job,
           'missed_total': n_missed, 'classes': dict(by), 'rows': rows},
          open(f'{R}/carry-windows.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
for job, v in per_job.items():
    print(job, v)
print('missed', n_missed, dict(by))
for r in rows:
    print(f"  {r['job']:12} {r['site']:55} | {r['class']} | linux: {r['linux']} | {r['mutation'][:70]}")
