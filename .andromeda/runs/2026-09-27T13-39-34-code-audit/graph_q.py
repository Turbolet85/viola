import json, subprocess, sys
R = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
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
 'dead': "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line",
}
out = {}
for k, sql in Q.items():
    p = subprocess.run([sys.executable, '-X', 'utf8', 'scripts/code-graph.py', 'query', R, 'code-audit', sql],
                       capture_output=True, text=True, encoding='utf-8')
    if p.returncode != 0:
        out[k] = {'error': p.returncode, 'stderr': p.stderr[-800:]}
        continue
    out[k] = json.loads(p.stdout)
with open(R + '/graph-raw.json', 'w', encoding='utf-8', newline='') as f:
    json.dump(out, f, ensure_ascii=False)
print({k: (len(v) if isinstance(v, list) else v) for k, v in out.items()})
