import json, collections, re, sys
d = json.load(open('.andromeda/runs/2026-10-08T10-08-51-code-audit/win-outcomes.json'))
for job, v in d.items():
    rows = v['rows']
    print('==', job, v['found'], v['tally'], v['first_ts'][:19], v['last_ts'][:19], 'rows', len(rows))
    if job == 'pty':
        print('  sample row:', json.dumps(rows[0])[:300])
    c = collections.Counter()
    g = collections.Counter()
    secs = collections.defaultdict(float)
    for r in rows:
        text = r if isinstance(r, str) else ' '.join(str(x) for x in r)
        m = re.search(r'((?:crates/[\w-]+/)?src/[\w/]+\.rs):\d+:\d+', text)
        f = m.group(1) if m else '?'
        c[f] += 1
        gm = re.search(r'\b(caught|unviable|MISSED|TIMEOUT)\b', text)
        g[(f, gm.group(1) if gm else '?')] += 1
        for tm in re.finditer(r'(\d+(?:\.\d+)?)s (build|test)', text):
            secs[(f, tm.group(2))] += float(tm.group(1))
    for f, n in sorted(c.items()):
        gs = {k[1]: x for k, x in g.items() if k[0] == f}
        print('   %-48s %4d %s build %.0fs test %.0fs' % (f, n, gs, secs[(f, 'build')], secs[(f, 'test')]))
