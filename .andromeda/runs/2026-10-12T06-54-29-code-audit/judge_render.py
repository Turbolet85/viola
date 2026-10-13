"""Phase 4: diff this run's record against the baseline (re-found by sha, never the ledger's last line), apply
audit-pass.md's threshold table, write judge.json (proposals.md is rendered from it; render_check.py asserts its tables).

Run after the append: this run's record is the ledger's LAST parseable line and must equal record.json.
"""
import json
import os
import re

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
recs, bad = [], 0
for line in open(os.path.join(REPO, '.andromeda/code-metrics.ndjson'), encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        bad += 1
cur = json.load(open(os.path.join(RUN, 'record.json'), encoding='utf-8'))
assert recs[-1] == cur, 'the ledger last line is not this run record'
prior = recs[:-1]
base = [r for r in prior if r['sha'] == cur['baseline_sha']][-1]
bb = [r for r in prior if r['sha'] == base['baseline_sha']][-1]
P, I, B = [], [], []  # proposals (dicts), informational lines, below-threshold lines


def g(r, path):
    v = r
    for k in path.split('.'):
        v = None if v is None else v.get(k)
    return v


# ---- trend-breaks: leading token against leading token ----
TOOLS = {'duplication': ['jscpd'], 'sizes': ['tokei'], 'complexity': ['rust-code-analysis'],
         'graph': ['code-graph.py'], 'dead': ['code-graph.py', 'cargo-machete'],
         'coverage': ['cargo-llvm-cov', 'cargo-nextest', 'rustc'], 'mutation': ['cargo-mutants', 'cargo-nextest', 'rustc']}
broken = {}
for metric, tools in TOOLS.items():
    for t in tools:
        a, b = base['tool_versions'].get(t), cur['tool_versions'].get(t)
        if a is None:
            I.append(f'`{metric}`: baseline `{t}` carries no version token — UNKNOWN, noted (not a trend-break)')
        elif a != b:
            broken.setdefault(metric, []).append(f'{t} {a} → {b}')
for metric, why in broken.items():
    I.append(f'trend-break — tool upgrade · `{metric}` ({"; ".join(why)}): its thresholds suppressed this run')

# ---- single-epoch thresholds (span 1) ----
span1 = cur['span'] == 1
assert span1
c, b0 = cur['graph']['cycles'], base['graph']['cycles']
line = f'new-cycle · graph.cycles {b0} → {c}'
if c <= b0:
    B.append(line + (' (code-graph.py trend-break: no cycle either way)' if 'graph' in broken else ''))
elif 'graph' in broken:
    I.append(line + ' — suppressed by the trend-break above')
else:
    P.append({'id': 'new-cycle', 'line': line})
dp, dpb = cur['duplication']['pct'], base['duplication']['pct']
fires = dp >= dpb + 0.5 and dp >= dpb * 1.15
B.append(f'duplication-up · duplication.pct {dpb} → {dp} ({dp - dpb:+.2f} pt; fires at ≥ +0.5 pt AND ≥ +15 %)') if not fires else P.append({'id': 'duplication-up'})
oc, ocb = cur['complexity']['over_ceiling'], base['complexity']['over_ceiling']
fires = oc >= ocb + 3 and oc >= ocb * 1.25
B.append(f'complexity-creep · complexity.over_ceiling {ocb} → {oc} (fires at ≥ +3 AND ≥ +25 %)') if not fires else P.append({'id': 'complexity-creep'})
zr, zrb = cur['dead']['zero_ref_candidates'], base['dead']['zero_ref_candidates']
txt = f'dead-growth · dead.zero_ref_candidates {zrb} → {zr} (fires at ≥ +5)'
if 'dead' in broken:
    I.append(txt + ' — suppressed by the trend-break above; no movement either way')
elif zr >= zrb + 5:
    P.append({'id': 'dead-growth'})
else:
    B.append(txt)
cl, clb = cur['coverage']['line'], base['coverage']['line']
cov_fire = cl is not None and clb is not None and cl <= clb - 2 and 'coverage' not in broken
if cov_fire:
    P.append({'id': 'coverage-drop', 'line': f'coverage.line {clb} → {cl}'})
else:
    B.append(f'coverage-drop · coverage.line {clb} → {cl} ({(cl - clb) if cl is not None and clb is not None else 0:+.2f} pt; fires at ≤ −2 pt)')

# ---- mutation-drop: each unit against its last SCORED value, equal scope ----
mut_lines = []
for u in cur['mutation']['scoped_units']:
    s = cur['mutation']['scores'].get(u)
    sc = (cur['mutation']['timing'].get(u) or {}).get('scope')
    last = None
    for r in reversed(prior):
        ps = (g(r, 'mutation.scores') or {}).get(u)
        if ps is not None:
            last = (r, ps, (g(r, 'mutation.timing') or {}).get(u, {}).get('scope'))
            break
    if s is None:
        I.append(f'mutation `{u}`: no score this run ({cur["mutation"]["unit_states"][u]})')
        continue
    if last is None:
        I.append(f'mutation `{u}`: first scored value {s} (scope {sc}, {norm(cur["epoch"])}); no earlier record scores it '
                 f'({", ".join(sorted({str((g(r, "mutation.unit_states") or {}).get(u)) for r in prior}))}) — no delta')
        continue
    r, ps, psc = last
    if psc is None:
        for cc in cur.get('corrections', []) + base.get('corrections', []):
            if cc['target_sha'] == r['sha'] and cc['field'] == 'mutation.timing':
                psc = (cc['now'].get(u) or {}).get('scope')
    if psc != sc or psc is None:
        I.append(f'mutation `{u}`: not comparable — {ps} (scope {psc}, {norm(r["epoch"])}) · {s} (scope {sc}, {norm(cur["epoch"])})')
    elif s <= ps - 10:
        P.append({'id': 'mutation-drop', 'unit': u, 'line': f'{ps} → {s}'})
    else:
        mut_lines.append(f'mutation-drop · `{u}` {ps} → {s} ({s - ps:+.2f} pt, scope unit at both, {norm(r["epoch"])} → '
                         f'{norm(cur["epoch"])}; fires at ≤ −10 pt)')
B.extend(mut_lines)

# ---- monotonic: the closed six, both of the last two diffs ----
SIX = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1), ('sizes.file_max', 1),
       ('sizes.over_800', 1), ('coverage.line', -1)]
mono = []
for path, sgn in SIX:
    vals = [g(bb, path), g(base, path), g(cur, path)]
    metric = path.split('.')[0]
    if any(v is None for v in vals):
        miss = [r['sha'][:7] for r, v in zip((bb, base, cur), vals) if v is None]
        sk = [s for r in (bb, base, cur) for s in r.get('skips', []) if s['metric'].startswith(metric)]
        B.append(f'monotonic · `{path}` not evaluable this run: null at {", ".join(miss)} '
                 f'({"; ".join(s["metric"] + " " + s["reason"] for s in sk) or "no skip named in that record"}) — never a zero')
        continue
    worse = lambda a, z: (z - a) * sgn > 0
    chain = ' → '.join(str(v) for v in vals)
    if metric in broken:
        I.append(f'monotonic · `{path}` {chain} — suppressed by its trend-break')
    elif worse(vals[0], vals[1]) and worse(vals[1], vals[2]):
        mono.append((path, vals))
    else:
        B.append(f'monotonic · `{path}` {chain} ({bb["sha"][:7]} → {base["sha"][:7]} → {cur["sha"][:7]}): not worse at both diffs')

# ---- count-under-ratio ----
d, db = cur['duplication'], base['duplication']
if (d['clones'] > db['clones'] or d['duplicated_lines'] > db['duplicated_lines']) and d['pct'] <= db['pct']:
    standing = [t for t in d['top'] if any(t[0] == x[0] and t[1] == x[1] and t[2] == x[2] for x in db['top'])]
    top = max(standing, key=lambda t: t[2]) if standing else None
    entered = None
    if top:
        for r in recs:
            if any(x[0] == top[0] and x[1] == top[1] and x[2] == top[2] for x in g(r, 'duplication.top') or []):
                entered = norm(r['epoch'])
                break
    sp = lambda k: f'{k} {db["split"][k]["pairs"]}→{d["split"][k]["pairs"]} pairs / {db["split"][k]["lines"]}→{d["split"][k]["lines"]} L'
    I.append(f'count-under-ratio · duplication.clones {db["clones"]} → {d["clones"]} ({d["clones"] - db["clones"]:+d}), '
             f'duplicated_lines {db["duplicated_lines"]} → {d["duplicated_lines"]} ({d["duplicated_lines"] - db["duplicated_lines"]:+d}) '
             f'while pct {db["pct"]} → {d["pct"]} ({d["pct"] - db["pct"]:+.2f} pt); population total_lines {db["total_lines"]} → '
             f'{d["total_lines"]} ({(d["total_lines"] / db["total_lines"] - 1) * 100:+.1f} %); split {sp("src")} · {sp("test")} · '
             f'{sp("mixed")}; top standing pair {top[0]} ↔ {top[1]} ({top[2]} L), in the top list since {entered}')

# ---- top-N entrants ----
def entrants(name, cur_l, base_l, key, show):
    bk = {key(x) for x in base_l}
    new = [x for x in cur_l if key(x) not in bk]
    if new:
        I.append(f'top-N entrants · {name}: ' + '; '.join(show(x) for x in new))


entrants('duplication.top', d['top'], db['top'], lambda t: (t[0].rsplit(':', 1)[0], t[1].rsplit(':', 1)[0], t[2]),
         lambda t: f'{t[0]} ↔ {t[1]} ({t[2]} L)')
entrants('complexity.top', cur['complexity']['top'], base['complexity']['top'], lambda t: (t['fn'], t['file'].rsplit(':', 1)[0]),
         lambda t: f'`{t["fn"]}` {t["file"]} (cognitive {t["value"]:g})')
entrants('sizes.top', cur['sizes']['top'], base['sizes']['top'], lambda t: t[0], lambda t: f'{t[0]} ({t[1]})')
entrants('hotspots', cur['hotspots'], base['hotspots'], lambda t: t[0], lambda t: f'{t[0]} ({t[1]:g})')
entrants('graph.fan_in_top', cur['graph']['fan_in_top'], base['graph']['fan_in_top'], lambda t: t[0],
         lambda t: f'`{t[0].split(" 0.1.0 ")[-1]}` ({t[0].split()[2]}, {t[1]})')

# ---- below threshold: the rest, one line each ----
B.append(f'graph.cross_unit_edges {base["graph"]["cross_unit_edges"]} → {cur["graph"]["cross_unit_edges"]}; fan_out '
         f'{dict(base["graph"]["fan_out"])} → {dict(cur["graph"]["fan_out"])}')
cx, cxb = cur['complexity'], base['complexity']
B.append(f'complexity percentiles · cyclomatic p50 {cxb["cyclomatic_p50"]:g} → {cx["cyclomatic_p50"]:g}, p90 '
         f'{cxb["cyclomatic_p90"]:g} → {cx["cyclomatic_p90"]:g}; cognitive p50 {cxb["cognitive_p50"]:g} → {cx["cognitive_p50"]:g}, '
         f'p90 {cxb["cognitive_p90"]:g} → {cx["cognitive_p90"]:g}; max `{cx["max"]["fn"]}` {cxb["max"]["val"]:g} → {cx["max"]["val"]:g}')
s, sb = cur['sizes'], base['sizes']
B.append(f'sizes percentiles · file_p50 {sb["file_p50"]} → {s["file_p50"]}, file_p90 {sb["file_p90"]} → {s["file_p90"]} '
         f'(no threshold; file_max and over_800 are under Proposals)')
B.append(f'dead.unused_deps unchanged: {cur["dead"]["unused_deps"]} (the fuzz workspace, outside the root graph)')
B.append(f'churn · pct {base["churn"]["pct"]} → {cur["churn"]["pct"]}, files_churned {base["churn"]["files_churned"]} → '
         f'{cur["churn"]["files_churned"]} (no threshold row past two records)')
B.append(f'population (never a movement) · totals.loc {base["totals"]["loc"]} → {cur["totals"]["loc"]}, totals.files '
         f'{base["totals"]["files"]} → {cur["totals"]["files"]}, units {base["totals"]["units"]} → {cur["totals"]["units"]}')

json.dump({'proposals': P, 'monotonic': mono, 'informational': I, 'below': B, 'unparseable': bad,
           'baseline': base['sha'], 'chain': bb['sha']},
          open(os.path.join(RUN, 'judge.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('proposals', len(P) + len(mono), 'monotonic', [m[0] for m in mono], 'informational', len(I), 'below', len(B))
