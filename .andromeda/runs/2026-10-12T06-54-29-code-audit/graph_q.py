"""A4 graph + A5 dead-code collectors over the rust plane, through scripts/code-graph.py query.

Writes c-graph.json and c-dead-raw.json (stages 1-2 of the ledger's recipes.dead; stage 3 is the hand
read of the residuals, recorded into c-dead.json by dead_final.py). cargo machete runs separately.
"""
import json
import os
import re
import subprocess

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
REL = os.path.relpath(RUN, REPO)


def q(sql):
    p = subprocess.run(['python3', 'scripts/code-graph.py', 'query', REL, 'code-audit', sql, 'rust'], cwd=REPO,
                       capture_output=True, text=True, check=True)
    return json.loads(p.stdout)


CYCLES = """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start"""
FAN_IN = "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20"
FAN_OUT = "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC"
EDGES = "SELECT count(*) AS n FROM crate_edges"
DEAD = ("SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol "
        "WHERE r.callee IS NULL ORDER BY s.file, s.def_line")

cyc = q(CYCLES)
fin = q(FAN_IN)
fout = q(FAN_OUT)
edges = q(EDGES)
graph = {'cycles': len(cyc), 'cycle_paths': [r['path'] for r in cyc],
         'fan_in_top': [[r['callee'], r['n']] for r in fin],
         'fan_out': [[r['from_crate'], r['n']] for r in fout],
         'cross_unit_edges': edges[0]['n']}
with open(os.path.join(RUN, 'c-graph.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(graph, f, ensure_ascii=False, indent=1)
    f.write('\n')

raw = q(DEAD)
PREFIX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TESTSEG = re.compile(r'(^|/)tests/')
stage1, excluded_tests = [], 0
for r in raw:
    stripped = PREFIX.sub('', r['symbol'] or '')
    if TESTSEG.search(stripped) or TESTSEG.search(r['file'] or ''):
        excluded_tests += 1
        continue
    stage1.append({**r, 'stripped': stripped})
classes, residual = {}, []
for r in stage1:
    sym, fil = r['symbol'] or '', r['file'] or ''
    if r['kind'] == 'module':
        c = 'module (not a callable; reached by path)'
    elif (re.search(r'(^|/)main\(\)\.$', r['stripped'])) or (fil.startswith('src/bin/') and sym.endswith('main().')):
        c = 'entry point'
    elif re.search(r'impl#\[[^\]]*\]\[[^\]]+\]', sym):
        c = 'trait-impl method (dispatch)'
    elif fil.startswith('crates/viola-e2e/') or 'test_support' in fil or 'test_support' in sym:
        c = 'test-only helper (test crate / test-support)'
    else:
        residual.append({'symbol': sym, 'kind': r['kind'], 'file': fil, 'def_line': r['def_line']})
        continue
    classes[c] = classes.get(c, 0) + 1
out = {'raw': len(raw), 'excluded_tests_union': excluded_tests, 'stage1': len(stage1), 'stage2_classes': classes,
       'residual': residual}
with open(os.path.join(RUN, 'c-dead-raw.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(out, f, ensure_ascii=False, indent=1)
    f.write('\n')
print(json.dumps({'graph': {k: graph[k] for k in ('cycles', 'cycle_paths', 'fan_out', 'cross_unit_edges')},
                  'dead': {k: out[k] for k in ('raw', 'excluded_tests_union', 'stage1', 'stage2_classes')},
                  'residual': residual}, ensure_ascii=False, indent=1))
