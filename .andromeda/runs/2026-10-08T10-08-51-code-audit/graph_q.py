"""A4 graph + A5 zero-ref raw pull: the canonical audit-pass.md SQL, verbatim, through code-graph.py query."""
import json, subprocess, sys

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
Q = {
 'cycles': """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start""",
 'fan_in': "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20",
 'fan_out': "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC",
 'edges': "SELECT count(*) AS n FROM crate_edges",
 'edge_list': "SELECT from_crate, to_crate FROM crate_edges ORDER BY 1, 2",
 'zero_ref': "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line",
}

def q(sql):
    p = subprocess.run([sys.executable, '-X', 'utf8', 'scripts/code-graph.py', 'query', R, 'code-audit', sql, 'rust'],
                       capture_output=True, text=True, encoding='utf-8')
    if p.returncode != 0:
        sys.exit(f'code-graph query failed ({p.returncode}): {p.stderr[-2000:]}\n{p.stdout[-2000:]}')
    out = p.stdout
    return json.loads(out[out.index('['):])

res = {k: q(v) for k, v in Q.items()}
graph = {'cycles': len(res['cycles']), 'cycle_paths': [r['path'] for r in res['cycles']],
         'fan_in_top': [[r['callee'], r['n']] for r in res['fan_in']],
         'fan_out': [[r['from_crate'], r['n']] for r in res['fan_out']],
         'cross_unit_edges': res['edges'][0]['n'],
         'edge_list': [[r['from_crate'], r['to_crate']] for r in res['edge_list']]}
json.dump(graph, open(f'{R}/c-graph.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
json.dump(res['zero_ref'], open(f'{R}/zero-ref-raw.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=0)
print('cycles', graph['cycles'], graph['cycle_paths'])
print('fan_out', graph['fan_out'], 'edges', graph['cross_unit_edges'])
print('edge_list', graph['edge_list'])
print('zero_ref raw', len(res['zero_ref']))
