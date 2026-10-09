"""Small evidence twins the renderer reads, each from a raw output that is deleted afterwards:
  c-duplication-perfile.json  duplicated lines a file takes part in (jscpd report), same-file pair count
  carry-e2e-deselected.json   the report-only viola-e2e run (one test deselected; stopped on the operator's answer
                              once harness/mod.rs and harness/cleanup.rs were graded): the tested prefix's grades,
                              `keeper` (every Workspace::ensure_e2e_home mutant) and `carry1` (the two viola-e2e
                              coordinates of the Windows CARRY, matched by mutation text). Never a score.
  scratch-residue.json        what stands in the operator's scratch dir after the tier (count + size)"""
import collections, json, os, re, subprocess, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cover import cover, host_cfg

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
SCRATCH = '/home/turbolet/dev/projects/viola-mutants-scratch/e3'
GRADE = {'CaughtMutant': 'caught', 'MissedMutant': 'missed', 'Unviable': 'unviable', 'Timeout': 'timeout'}

d = json.load(open(f'{R}/jscpd/jscpd-report.json', encoding='utf-8'))
f = collections.Counter()
for c in d['duplicates']:
    for k in ('firstFile', 'secondFile'):
        f[c[k]['name'].replace('\\', '/')] += c['lines']
same = sum(1 for c in d['duplicates'] if c['firstFile']['name'] == c['secondFile']['name'])
json.dump({'by_file': f.most_common(15), 'same_file_pairs': same, 'pairs': len(d['duplicates'])},
          open(f'{R}/c-duplication-perfile.json', 'w', encoding='utf-8', newline='\n'), indent=1)

done = json.load(open(f'{R}/mut-done-viola-e2e-deselected.json', encoding='utf-8'))
out = f'{R}/mutants-viola-e2e-deselected/mutants.out'
oc = json.load(open(f'{out}/outcomes.json', encoding='utf-8'))
listed = json.load(open(f'{out}/mutants.json', encoding='utf-8'))
assert oc['outcomes'][0]['scenario'] == 'Baseline'
cfg = host_cfg('.')
split = lambda name: re.match(r'^(.*?:\d+:\d+): (.*)$', name, re.S).groups()
names, by, survivors, not_measured, timeouts = [], collections.Counter(), [], [], []
files_done = collections.Counter()
for o in oc['outcomes'][1:]:
    mu = o['scenario']['Mutant']
    g = GRADE[o['summary']]
    names.append((mu['name'], g))
    files_done[mu['file']] += 1
    site, text = split(mu['name'])
    if g == 'missed':
        p = cover('.', mu, cfg)
        if p is not None:
            not_measured.append([site, text, f'cfg({p})']); by['not_measured'] += 1
            continue
        survivors.append([site, text])
    elif g == 'timeout':
        timeouts.append([site, text])
    by[g] += 1
per_file = collections.Counter(m['file'] for m in listed)
whole = sorted(p for p in files_done if files_done[p] == per_file[p])
partial = sorted(p for p in files_done if files_done[p] != per_file[p])
keeper = [x for x in names if 'ensure_e2e_home' in x[0]]
# cleanup.rs is byte-identical to its 60c569b state (the first dispatch's sha), so the CARRY's coordinates stand
carry1 = [x for x in names if x[0] in ('crates/viola-e2e/src/harness/cleanup.rs:106:35: replace + with - in cleanup_one',
                                       'crates/viola-e2e/src/harness/cleanup.rs:140:9: delete ! in unconnectable')]
assert len(keeper) == sum(1 for m in listed if 'ensure_e2e_home' in m['name']), 'a keeper mutant is ungraded'
assert len(carry1) == 2, carry1
first = oc['outcomes'][0]
doc = {'label': 'viola-e2e-deselected', 'report_only': True, 'driver_state': done['state'], 'exit_code': done['exit_code'],
       'listed': len(listed), 'tested': len(names), 'complete': bool(oc.get('end_time')) and len(names) == len(listed),
       'files_whole': whole, 'files_partial': partial,
       'counts': {'caught': by['caught'], 'missed': len(survivors), 'not_measured': len(not_measured),
                  'timeout': by['timeout'], 'unviable': by['unviable']},
       'wall_s': done['driver_elapsed_s'], 'started': done['started'], 'ended': done['ended'],
       'baseline': {'summary': first['summary'], 'argv': first['phase_results'][-1]['argv'][2:],
                    'phases': [[p['phase'], round(p['duration'], 1)] for p in first['phase_results']]},
       'survivors': survivors, 'not_measured': not_measured, 'timeouts': timeouts,
       'keeper': keeper, 'carry1': carry1, 'command': done['command'], 'env': done['env'], 'carry': done['carry']}
assert sum(doc['counts'].values()) == len(names)
json.dump(doc, open(f'{R}/carry-e2e-deselected.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)

entries = sorted(os.listdir(SCRATCH))
du = subprocess.run(['du', '-sh', SCRATCH], capture_output=True, text=True).stdout.split()[0]
json.dump({'dir': '~/dev/projects/viola-mutants-scratch/e3', 'entries': len(entries), 'human': du,
           'copies': [e for e in entries if e.startswith('cargo-mutants-')],
           'kinds': collections.Counter('.tmp*' if e.startswith('.tmp') else e.rsplit('-', 1)[0] for e in entries).most_common()},
          open(f'{R}/scratch-residue.json', 'w', encoding='utf-8', newline='\n'), indent=1)
print('perfile', f.most_common(4), 'same', same)
print('e2e', doc['tested'], '/', doc['listed'], doc['counts'], 'whole files', len(whole), 'partial', partial)
print('keeper', keeper)
print('carry1', carry1)
print('survivors', survivors, 'timeouts', timeouts)
print('backing', done['carry']['backing'])
print('scratch', len(entries), du, [e for e in entries if e.startswith('cargo-mutants-')])
