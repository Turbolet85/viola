"""The Windows leg, read per job from its log (run 37761947926): the harness document and cargo-mutants' outcome
lines. python3 -B winlog.py <scratch dir holding win-<job>.log>  ->  prints a per-job tally; writes win-outcomes.json
(the outcome lines only, names as the tool printed them) into the run dir. Report-only: never merged into the
ledger's counts."""
import collections, json, re, sys

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
S = sys.argv[1]
ANSI = re.compile(r'\x1b\[[0-9;?]*[A-Za-z]')
TS = re.compile(r'^﻿?\d{4}-\d\d-\d\dT[\d:.]+Z ')
OUT = re.compile(r'^(caught|MISSED|unviable|TIMEOUT)\s+(.*?:\d+:\d+: .*?) in (\d+)s build(?: \+ (\d+)s test)?\s*$')
jobs = {}
for job in ('pty', 'channel', 'state', 'agent-claude', 'viola', 'e2e'):
    rows, doc, found, first_ts, last_ts, tail = [], None, None, None, None, []
    for raw in open(f'{S}/win-{job}.log', encoding='utf-8', errors='replace'):
        m = TS.match(raw)
        ts = m.group(0).strip().lstrip('﻿') if m else None
        line = ANSI.sub('', TS.sub('', raw)).rstrip('\r\n')
        if ts:
            first_ts = first_ts or ts
            last_ts = ts
        o = OUT.match(line.strip())
        if o:
            rows.append({'grade': o.group(1), 'name': o.group(2), 'build_s': int(o.group(3)),
                         'test_s': int(o.group(4)) if o.group(4) else None, 'ts': ts})
        elif line.startswith('{"v":1,"cmd":"run"'):
            try: doc = json.loads(line)
            except ValueError: doc = {'unparsed': line[:300]}
        elif line.startswith('Found '):
            found = line.strip()
        if line.strip():
            tail = (tail + [line.strip()[:220]])[-6:]
    tally = collections.Counter(r['grade'] for r in rows)
    names = [r['name'] for r in rows]
    jobs[job] = {'found': found, 'tally': dict(tally), 'outcome_lines': len(rows), 'unique_names': len(set(names)),
                 'first_ts': first_ts, 'last_ts': last_ts, 'doc': doc, 'rows': rows, 'tail': tail}
    print(job, found, dict(tally), 'lines', len(rows), 'unique', len(set(names)), first_ts, '..', last_ts)
    print('   doc:', json.dumps(doc)[:600] if doc else None)
    if doc is None:
        for t in tail: print('   tail:', t)
json.dump(jobs, open(f'{R}/win-outcomes.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=0)
