"""C1 summarizer: python3 -B mutsum.py <unit>  ->  c-mutation-{unit}.json (written only for a unit the tool's own
markers read complete, or one stopped at its cap: budget-exhausted carries timing and no score).
Counts from mutants.out/outcomes.json; a missed mutant whose whole span this host's build excludes is
`not measured on this host` (cover.py, the pinned recipe); survivors FULL, keyed on the tool's own name."""
import collections, json, os, re, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cover import cover, host_cfg

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
unit = sys.argv[1]
# a labelled (report-only) run: python3 -B mutsum.py <label> <output name>; never a c-mutation twin
target = sys.argv[2] if len(sys.argv) > 2 else f'c-mutation-{unit}.json'
assert len(sys.argv) > 2 or unit in ('viola-core', 'viola-pty', 'viola-state', 'viola-channel', 'viola-agent-claude', 'viola', 'viola-e2e')
done = json.load(open(f'{R}/mut-done-{unit}.json', encoding='utf-8'))
out = f'{R}/mutants-{unit}/mutants.out'
oc = json.load(open(f'{out}/outcomes.json', encoding='utf-8'))
listed = json.load(open(f'{out}/mutants.json', encoding='utf-8'))
complete = bool(oc.get('end_time')) and oc.get('total_mutants') == len(listed)
if done['state'] == 'complete' and not complete:
    sys.exit(f'{unit}: driver read complete, markers do not (end_time={oc.get("end_time")}, '
             f'{oc.get("total_mutants")}/{len(listed)})')
if done['state'] not in ('complete', 'budget-exhausted'):
    sys.exit(f'{unit}: state {done["state"]} - no artifact; classify by its exit and surface')

def split(name):  # "file:line:col: text" -> (site, text)
    m = re.match(r'^(.*?:\d+:\d+): (.*)$', name, re.S)
    if not m:
        sys.exit(f'unparsed mutant name: {name!r}')
    return m.group(1), m.group(2)

cfg = host_cfg('.')
by = collections.Counter()
survivors, not_measured, timeouts, unviable = [], [], [], []
assert oc['outcomes'][0]['scenario'] == 'Baseline', oc['outcomes'][0]['scenario']  # outcomes[0] = the unmutated tree
tested = oc['outcomes'][1:]
for o in tested:
    mu = o['scenario']['Mutant']
    site, text = split(mu['name'])
    s = o['summary']
    by[s] += 1
    if s == 'MissedMutant':
        p = cover('.', mu, cfg)
        if p is not None:
            not_measured.append([site, text, f'cfg({p})'])
        else:
            survivors.append([site, text])
    elif s == 'Timeout':
        timeouts.append([site, text])
    elif s == 'Unviable':
        unviable.append([site, text])

tool = {k: oc.get(k) for k in ('total_mutants', 'caught', 'missed', 'timeout', 'unviable', 'success')}
counts = {'mutants': oc['total_mutants'], 'caught': by['CaughtMutant'], 'missed': len(survivors),
          'not_measured': len(not_measured), 'timeout': by['Timeout'], 'unviable': by['Unviable']}
# two sources: the per-outcome tally against the tool's own totals
assert by['CaughtMutant'] == tool['caught'] and by['Unviable'] == tool['unviable'] and by['Timeout'] == tool['timeout'], (by, tool)
assert counts['missed'] + counts['not_measured'] == tool['missed'], (counts, tool)
assert sum(by.values()) == tool['total_mutants'] == len(tested), (by, tool, len(tested))
for rows, n in ((survivors, counts['missed']), (not_measured, counts['not_measured'])):
    keys = [(r[0], r[1]) for r in rows]
    assert len(keys) == len(set(keys)) == n, 'rows not unique on (site, mutation) or count mismatch'
missed_txt = [l.strip() for l in open(f'{out}/missed.txt', encoding='utf-8') if l.strip()]
assert len(missed_txt) == tool['missed'], (len(missed_txt), tool['missed'])

dominant = counts['unviable'] > counts['caught'] + counts['missed'] + counts['not_measured'] + counts['timeout']
denom = counts['caught'] + counts['missed']
score = round(100 * counts['caught'] / denom, 2) if denom and done['state'] == 'complete' and not dominant else None
state = (f'unviable-dominant {counts["unviable"]}/{counts["mutants"]}' if done['state'] == 'complete' and dominant
         else f'complete {oc["total_mutants"]}/{len(listed)}' if done['state'] == 'complete' else 'budget-exhausted')
first = oc['outcomes'][0]
doc = {'unit': unit, 'unit_state': state, 'score': score, 'score_formula': 'caught/(caught+missed)',
       'counts': counts, 'tool_totals': tool,
       'timing': {'scope': 'unit', 'planned': done['planned'], 'tested': oc['total_mutants'],
                  'wall_s': done['wall_s'], 'jobs': done['jobs'], 'cap_s': done['cap_s']},
       'start_time': oc.get('start_time'), 'end_time': oc.get('end_time'),
       'baseline': {'summary': first.get('summary'),
                    'phases': [[p.get('phase'), p.get('duration')] for p in first.get('phase_results', [])]},
       'host': 'x86_64-unknown-linux-gnu',
       'survivors': survivors, 'not_measured': not_measured, 'timeouts': timeouts,
       'unviable_sites': unviable,
       'command': done['command'], 'env': done['env'], 'carry': done['carry']}
json.dump(doc, open(f'{R}/{target}', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(unit, state, 'score', score, counts, 'wall', done['wall_s'], 's')
for r in survivors: print('  MISSED', r)
for r in not_measured: print('  NOT-MEASURED', r)
for r in timeouts: print('  TIMEOUT', r)
