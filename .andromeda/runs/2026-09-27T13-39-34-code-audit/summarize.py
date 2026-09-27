import json, os, re, subprocess, collections
R = '.andromeda/runs/2026-09-27T13-39-34-code-audit'
BASE = 'a28f69684d3202e85b0de2e4e0ad26e23cbe0e7d'
EXCL = re.compile(r'^(scripts|docs|refs|fuzz|\.andromeda|\.claude)/')
pop = [l.strip() for l in open(R + '/source-files.txt', encoding='utf-8') if l.strip()]
base = json.loads(open('.andromeda/code-metrics.ndjson', encoding='utf-8').readline())

def dump(name, obj):
    with open(R + '/' + name, 'w', encoding='utf-8', newline='') as f:
        f.write(json.dumps(obj, ensure_ascii=False, indent=1) + '\n')

def nr(vals, p):  # nearest-rank percentile
    s = sorted(vals)
    if not s:
        return None
    k = max(1, -(-p * len(s) // 100))
    return s[int(k) - 1]

def is_test(p):
    p = p.replace('\\', '/')
    return p.startswith('tests/') or '/tests/' in p

# A3 sizes
tk = json.load(open(R + '/tokei-raw.json', encoding='utf-8'))
per = {r['name'].replace('\\', '/'): r['stats']['code'] for r in tk['Rust']['reports']}
assert set(per) == set(pop), (set(pop) ^ set(per))
vals = list(per.values())
top = sorted(per.items(), key=lambda kv: -kv[1])[:10]
sizes = {'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals), 'over_800': sum(v > 800 for v in vals), 'top': [[a, b] for a, b in top]}
units = {p.split('/')[1] for p in pop if p.startswith('crates/')} | {'viola'}
totals = {'loc': sum(vals), 'files': len(vals), 'units': len(units)}
dump('c-sizes.json', {'totals': totals, 'sizes': sizes, 'population': 'git ls-files "*.rs" minus ^(scripts|docs|refs|fuzz|.andromeda|.claude)/ (%d files)' % len(pop)})

# A1 duplication
jr = json.load(open(R + '/jscpd/jscpd-report.json', encoding='utf-8'))
st = jr['statistics']['total']
rows = []
split = {'src': {'pairs': 0, 'lines': 0}, 'test': {'pairs': 0, 'lines': 0}, 'mixed': {'pairs': 0, 'lines': 0}}
for d in jr['duplicates']:
    a = d['firstFile']['name'].replace('\\', '/'); b = d['secondFile']['name'].replace('\\', '/')
    ta, tb = is_test(a), is_test(b)
    k = 'test' if ta and tb else ('src' if not ta and not tb else 'mixed')
    split[k]['pairs'] += 1; split[k]['lines'] += d['lines']
    rows.append([f"{a}:{d['firstFile']['startLoc']['line']}", f"{b}:{d['secondFile']['startLoc']['line']}", d['lines']])
rows.sort(key=lambda r: -r[2])
btop = {(x[0], x[1]) for x in base['duplication']['top']}
bfiles = {(x[0].rsplit(':', 1)[0], x[1].rsplit(':', 1)[0]) for x in base['duplication']['top']}
dup = {'pct': round(st['percentage'], 2), 'duplicated_lines': st['duplicatedLines'], 'total_lines': st['lines'], 'clones': st['clones'],
       'top': rows[:10], 'split': split}
new_top = [r for r in rows[:10] if (r[0], r[1]) not in btop]
dump('c-duplication.json', {**dup, 'new_in_top_vs_baseline': new_top,
     'standing_file_pairs': [r for r in rows if (r[0].rsplit(':', 1)[0], r[1].rsplit(':', 1)[0]) in bfiles]})

# A2 complexity (cognitive sum per function space - calibrated against the baseline's run_with = 24)
fns = []
for p in pop:
    j = os.path.join(R, 'rca', p + '.json')
    d = json.load(open(j, encoding='utf-8'))
    def walk(n):
        if n.get('kind') == 'function':
            m = n['metrics']
            fns.append({'fn': n['name'], 'file': f"{p}:{n['start_line']}", 'cyc': m['cyclomatic']['sum'], 'cog': m['cognitive']['sum'], 'path': p})
        for c in n.get('spaces', []):
            walk(c)
    walk(d)
cy = [f['cyc'] for f in fns]; cg = [f['cog'] for f in fns]
tops = sorted(fns, key=lambda f: -f['cog'])[:10]
cx = {'cyclomatic_p50': float(nr(cy, 50)), 'cyclomatic_p90': float(nr(cy, 90)), 'cognitive_p50': float(nr(cg, 50)), 'cognitive_p90': float(nr(cg, 90)),
      'over_ceiling': sum(v > 15 for v in cg), 'max': {'fn': tops[0]['fn'], 'file': tops[0]['file'], 'val': tops[0]['cog']},
      'top': [{'fn': f['fn'], 'file': f['file'], 'value': f['cog']} for f in tops]}
dump('c-complexity.json', {**cx, 'functions': len(fns), 'over_ceiling_list': [{'fn': f['fn'], 'file': f['file'], 'value': f['cog']} for f in sorted(fns, key=lambda f: -f['cog']) if f['cog'] > 15]})
maxcog = collections.defaultdict(float)
for f in fns:
    maxcog[f['path']] = max(maxcog[f['path']], f['cog'])

# A4 graph + A5 dead
g = json.load(open(R + '/graph-raw.json', encoding='utf-8'))
graph = {'cycles': len(g['cycles']), 'cycle_paths': [r['path'] for r in g['cycles']],
         'fan_in_top': [[r['callee'], r['n']] for r in g['fan_in']],
         'fan_out': [[r['from_crate'], r['n']] for r in g['fan_out']], 'cross_unit_edges': g['edges'][0]['n']}
dump('c-graph.json', {**graph, 'edge_list': [[r['from_crate'], r['to_crate']] for r in g['edge_list']]})
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
seg = lambda p: p.startswith('tests/') or '/tests/' in p
classes = collections.Counter(); residual = []; classed = []
for r in g['dead']:
    sp = PFX.sub('', r['symbol']); fp = (r['file'] or '').replace('\\', '/')
    if seg(sp) or seg(fp):
        classes['test (tests/ segment, symbol or file path)'] += 1; continue
    if sp.startswith('test_capture/') or '/test_support/' in sp:
        c = 'test-only helper (cfg(test) module)'
    elif sp == 'main().':
        c = 'entry point'
    elif re.search(r'impl#\[[^\]]*\]\[[^\]]*\]', sp):
        c = 'trait-impl method reached by dispatch'
    elif sp == 'Pty#child_pid().':
        c = 'trait method reached by dyn dispatch (called via Box<dyn Pty> in the crate test module)'
    elif re.search(r'#(Json|Encode)#$', sp):
        c = 'derive/attr-invoked (#[from] variant)'
    elif sp == 'events/kind_str().':
        c = 'derive/attr-invoked (serde serialize_with)'
    elif sp in ('harness/pre_push/CLONE_DIR.', 'harness/pre_push/SCRATCH_DIR.'):
        c = 'inline format-string capture'
    else:
        residual.append([sp, fp, r['def_line']]); continue
    classes[c] += 1; classed.append([c, sp, fp, r['def_line']])
p = subprocess.run(['cargo', 'machete'], capture_output=True, text=True, encoding='utf-8')
unused = []; cur = None
for ln in p.stdout.splitlines():
    m = re.match(r'^(\S+) -- \.[\\/](.+Cargo\.toml):$', ln)
    if m:
        cur = f"{m.group(1)} ({m.group(2).replace(chr(92), '/')}" + (', own workspace)' if m.group(2).startswith('fuzz') else ')'); continue
    if ln.startswith('\t') and cur:
        unused.append([cur, ln.strip()])
    elif not ln.strip():
        cur = None
dead = {'unused_deps': unused, 'zero_ref_candidates': len(residual), 'top': residual[:20]}
dump('c-dead.json', {**dead, 'unused_deps_notes': {'proc-macro2': 'feature-enabling dependency: the workspace pin carries features = ["span-locations"] (Cargo.toml:139-140) so syn spans give real lines in harness/cfg_legs.rs:78; no `proc_macro2` path in source is by design; a machete false positive'}, 'raw_zero_ref': len(g['dead']), 'classes': dict(classes), 'classed_non_test': classed,
     'machete_exit': p.returncode, 'machete_stdout_tail': p.stdout[-1200:]})

# B1 churn + B2 hotspots
log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{BASE}..HEAD', '--', '*.rs'], capture_output=True, text=True, encoding='utf-8').stdout
seen = collections.Counter(); adds_all = 0; adds_churn = 0; commits_per = collections.Counter(); ncommit = 0
for line in log.splitlines():
    if line.startswith('@'):
        ncommit += 1; continue
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-':
        continue
    a, dl, path = parts
    if '=>' in path:  # rename: count to the new path
        path = re.sub(r'\{[^{}]*=> ([^{}]*)\}', r'\1', path) if '{' in path else path.split('=>')[1].strip()
        path = path.replace('//', '/')
    if EXCL.match(path):
        continue
    a = int(a); adds_all += a
    if seen[path] >= 1:
        adds_churn += a
    seen[path] += 1; commits_per[path] += 1
churn = {'pct': round(100.0 * adds_churn / adds_all, 2) if adds_all else None, 'files_churned': sum(1 for v in seen.values() if v >= 2)}
dump('c-churn.json', {**churn, 'adds_all': adds_all, 'adds_churned': adds_churn, 'files_touched': len(seen), 'commits': ncommit,
     'command': f'git log --reverse --numstat --format=@%H {BASE}..HEAD -- "*.rs"  # minus ^(scripts|docs|refs|fuzz|.andromeda|.claude)/'})
hs = []
for path, n in commits_per.items():
    if path in maxcog:
        hs.append([path, round(n * maxcog[path], 1), n, maxcog[path]])
hs.sort(key=lambda r: -r[1])
bh = {h[0] for h in (base.get('hotspots') or [])}
dump('c-hotspots.json', {'hotspots': [[h[0], h[1]] for h in hs[:10]], 'detail': hs[:10], 'score': 'commits x max cognitive in file', 'entrants_vs_baseline': [h[0] for h in hs[:10] if h[0] not in bh]})
print('sizes', totals, sizes['file_max'], sizes['over_800'])
print('dup', dup['pct'], dup['clones'], dup['duplicated_lines'], dup['total_lines'], split)
print('cx', {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling')}, cx['max'])
print('graph', graph['cycles'], graph['cross_unit_edges'], graph['fan_out'])
print('dead', len(residual), dict(classes), 'machete', p.returncode)
print('churn', churn, adds_all, len(seen), ncommit)
print('hot', hs[:5])
