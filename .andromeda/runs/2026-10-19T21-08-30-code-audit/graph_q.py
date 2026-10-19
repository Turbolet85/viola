#!/usr/bin/env python3
"""A4 graph + A5 dead-code collectors through scripts/code-graph.py query (the views are the contract).

usage: graph_q.py <run_dir>   (repo root as cwd)
Writes c-graph.json, c-dead-raw.json (stages 1-2 of recipes.dead applied; stage 3 is the hand read), c-coverage.json.
"""
import json, re, subprocess, sys

R = sys.argv[1]

CYCLES = """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start"""
FANIN = "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20"
FANOUT = "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC"
EDGES = "SELECT count(*) AS n FROM crate_edges"
ZERO = ("SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol "
        "WHERE r.callee IS NULL ORDER BY s.file, s.def_line")


def q(sql):
    sql = ' '.join(sql.split())  # the query verb reads one line
    p = subprocess.run(['python3', 'scripts/code-graph.py', 'query', R, 'code-audit', sql, 'rust'],
                       capture_output=True, text=True, check=True)
    if not p.stdout.strip():
        sys.exit(f'empty query output; stderr: {p.stderr[:400]}')
    return json.loads(p.stdout)


cyc = q(CYCLES)
fi = q(FANIN)
fo = q(FANOUT)
ed = q(EDGES)
g = {'cycles': len(cyc), 'cycle_paths': [r['path'] for r in cyc],
     'fan_in_top': [[r['callee'], r['n']] for r in fi],
     'fan_out': [[r['from_crate'], r['n']] for r in fo],
     'cross_unit_edges': ed[0]['n']}
json.dump(g, open(f'{R}/c-graph.json', 'w'), indent=1)
print('graph cycles', g['cycles'], 'edges', g['cross_unit_edges'], 'fan_out', g['fan_out'])

rows = q(ZERO)
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TESTSEG = re.compile(r'(^|/)tests/')
IMPL = re.compile(r'impl#\[[^\]]*\]\[[^\]]+\]')
stage1, classes, residual = 0, {}, []
for r in rows:
    sym, f = r['symbol'], r['file'] or ''
    stripped = PFX.sub('', sym)
    if TESTSEG.search(stripped) or TESTSEG.search(f):
        stage1 += 1
        continue
    if r['kind'] == 'module':
        c = 'module (not a callable; reached by path)'
    elif (sym.endswith('main().') and '/' not in stripped) or (f.startswith('src/bin/') and sym.endswith('main().')):
        c = 'entry point'
    elif IMPL.search(sym):
        c = 'trait-impl method (dispatch)'
    elif f.startswith('crates/viola-e2e/') or 'test_support' in f or 'test_support' in sym:
        c = 'test-only helper (test crate / test-support)'
    else:
        residual.append(r)
        continue
    classes[c] = classes.get(c, 0) + 1
d = {'raw': len(rows), 'stage1_tests_excluded': stage1, 'stage2_classes': classes, 'residual': residual}
json.dump(d, open(f'{R}/c-dead-raw.json', 'w'), indent=1)
print('dead raw', len(rows), 'tests excluded', stage1, 'classes', classes, 'residual', len(residual))
for r in residual:
    print('  residual', r['kind'], r['file'], r['def_line'], PFX.sub('', r['symbol']))

cov = json.load(open(f'{R}/cov-raw.json'))['data'][0]['totals']
c = {'line': round(cov['lines']['percent'], 2), 'branch': None,
     'lines': {'count': cov['lines']['count'], 'covered': cov['lines']['covered']},
     'functions': round(cov['functions']['percent'], 2), 'regions': round(cov['regions']['percent'], 2),
     'tests': '2958 run: 2958 passed (cov-run.log Summary line)'}
json.dump(c, open(f'{R}/c-coverage.json', 'w'), indent=1)
print('coverage', c['line'], c['functions'], c['regions'])
