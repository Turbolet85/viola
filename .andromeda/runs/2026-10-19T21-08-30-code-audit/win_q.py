#!/usr/bin/env python3
"""The windows-mutants.yml dispatch read back: per job its conclusion, baseline line, tally line and the first failing
tests; the CARRY 3 sites from the viola-state job. Logs are fetched into a dir OUTSIDE the repo (argv[2]) and only
the summary lands here. usage: win_q.py <run_dir> <log_dir> <run_id>"""
import json, re, subprocess, sys

R, LD, RUN = sys.argv[1], sys.argv[2], sys.argv[3]
v = json.loads(subprocess.run(['gh', 'run', 'view', RUN, '--json', 'status,conclusion,headSha,createdAt,updatedAt,url,jobs'],
                              capture_output=True, text=True, check=True).stdout)
CARRY3 = [('crates/viola-state/src/fs.rs:295:18', 'replace += with *= in replace_private_retrying'),
          ('crates/viola-state/src/fs.rs:298:23', 'replace match guard attempts < REPLACE_ATTEMPTS && retry(e.error.raw_os_error()) with true in replace_private_retrying'),
          ('crates/viola-state/src/fs.rs:298:51', 'replace && with || in replace_private_retrying')]
OUT = re.compile(r'^(caught|MISSED|TIMEOUT|unviable)\s+(\S+): (.*?) in \d+s build(?: \+ (\d+)s test)?\s*$')
jobs = []
for j in v['jobs']:
    p = f"{LD}/job-{j['databaseId']}.log"
    with open(p, 'w') as f:
        subprocess.run(['gh', 'run', 'view', '--job', str(j['databaseId']), '--log'], stdout=f, stderr=subprocess.STDOUT, check=True)
    body = [l.split('\t', 2)[-1] for l in open(p, encoding='utf-8', errors='replace').read().splitlines()]
    body = [re.sub(r'^\S+Z ', '', l) for l in body]
    base = next((l.strip() for l in body if 'Unmutated baseline' in l), None)
    tally = next((l.strip() for l in body if re.search(r'\d+ mutants tested in', l)), None)
    fails = sorted({re.sub(r'\s+', ' ', l.strip()) for l in body if re.match(r'\s*(TIMEOUT|FAIL) \[', l)})
    panics = [l.strip() for l in body if 'assertion `left == right` failed' in l or l.strip().startswith(('left:', 'right:'))][:3]
    verdict = next((l.strip()[:400] for l in body if l.startswith('{"v":1,"cmd":"run"')), None)
    outs = {}
    for l in body:
        m = OUT.match(l.strip())
        if m:
            outs[(m.group(2), m.group(3))] = [m.group(1), int(m.group(4)) if m.group(4) else None]
    missed = sorted([s, t] for (s, t), (o, _) in outs.items() if o == 'MISSED')
    row = {'job': j['name'], 'id': j['databaseId'], 'conclusion': j['conclusion'], 'baseline': base, 'tally': tally,
           'baseline_failures': fails[:6], 'assertion': panics, 'harness_verdict': verdict, 'missed': missed}
    if j['name'] == 'mutants (viola-state)':
        row['carry3'] = [[s, t] + outs.get((s, t), ['absent', None]) for s, t in CARRY3]
    jobs.append(row)
doc = {'run': RUN, 'url': v['url'], 'head_sha': v['headSha'], 'created': v['createdAt'], 'updated': v['updatedAt'],
       'status': v['status'], 'conclusion': v['conclusion'], 'jobs': jobs,
       'note': 'dispatched at the founder\'s word (attended confirm 2026-10-19T21:21Z); logs read from ' + LD + ' (outside the repo), not kept'}
json.dump(doc, open(f'{R}/c-windows-mutants.json', 'w'), indent=1, ensure_ascii=False)
for r in jobs:
    print(r['job'], r['conclusion'], '|', r['baseline'], '|', r['tally'], '|', len(r['missed']), 'missed', r.get('carry3', ''))
