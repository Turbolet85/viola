import json, os, re, math, glob

D = '.andromeda/runs/2026-09-24T17-47-08-code-audit'
FILES = [l.strip() for l in open(f'{D}/source-files.txt', encoding='utf-8') if l.strip()]
norm = lambda p: p.replace('\\', '/').lstrip('./')

def nr(vals, q):  # nearest-rank percentile
    if not vals: return None
    s = sorted(vals); k = max(1, math.ceil(q / 100 * len(s)))
    return s[k - 1]

def dump(name, obj):
    json.dump(obj, open(f'{D}/c-{name}.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

def is_test(p):
    p = '/' + norm(p)
    return '/tests/' in p

# A3 sizes (tokei) — population = source-files.txt
t = json.load(open(f'{D}/tokei-raw.json', encoding='utf-8'))
per = {}
for lang, v in t.items():
    if lang == 'Total': continue
    for r in v.get('reports', []):
        per[norm(r['name'])] = r['stats']['code']
per = {f: per[f] for f in FILES if f in per}
vals = list(per.values())
sizes = {'population': 'source-files.txt (tracked *.rs of the root workspace: src/, tests/, crates/*/{src,tests}; fuzz/ excluded)',
         'files': len(per), 'loc': sum(vals), 'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals),
         'over_800': sum(1 for v in vals if v > 800),
         'top': sorted(([f, v] for f, v in per.items()), key=lambda x: -x[1])[:10],
         'missing_from_tokei': [f for f in FILES if f not in per]}
dump('sizes', sizes)

# A1 duplication (jscpd)
j = json.load(open(f'{D}/jscpd/jscpd-report.json', encoding='utf-8'))
tot = j['statistics']['total']
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
rows = []
for c in j.get('duplicates', []):
    a, b = norm(c['firstFile']['name']), norm(c['secondFile']['name'])
    k = 'test' if is_test(a) and is_test(b) else ('src' if not is_test(a) and not is_test(b) else 'mixed')
    split[k]['pairs'] += 1; split[k]['lines'] += c['lines']
    rows.append([f"{a}:{c['firstFile']['start']}", f"{b}:{c['secondFile']['start']}", c['lines']])
dup = {'pct': round(tot['percentage'], 2), 'duplicated_lines': tot['duplicatedLines'], 'total_lines': tot['lines'],
       'clones': tot['clones'], 'top': sorted(rows, key=lambda x: -x[2])[:10], 'split': split}
dump('duplication', dup)

# A2 complexity (rust-code-analysis) — per function space
fns = []
def walk(sp, file):
    for c in sp.get('spaces', []):
        if c.get('kind') == 'function':
            m = c['metrics']
            fns.append({'fn': c.get('name'), 'file': file, 'line': c.get('start_line'),
                        'cyc': m['cyclomatic']['sum'], 'cog': m['cognitive']['sum']})
        walk(c, file)
for p in glob.glob(f'{D}/rca/**/*.json', recursive=True):
    r = json.load(open(p, encoding='utf-8'))
    walk(r, norm(r.get('name', p)))
cyc = [f['cyc'] for f in fns]; cog = [f['cog'] for f in fns]
mx = max(fns, key=lambda f: f['cog'])
cx = {'functions': len(fns), 'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90), 'cyclomatic_max': max(cyc),
      'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
      'over_ceiling': sum(1 for v in cog if v > 15),
      'max': {'fn': mx['fn'], 'file': f"{mx['file']}:{mx['line']}", 'val': mx['cog']},
      'top': [{'fn': f['fn'], 'file': f"{f['file']}:{f['line']}", 'value': f['cog']} for f in sorted(fns, key=lambda f: -f['cog'])[:10]],
      'note': 'per rust-code-analysis function space (closures included where rca emits them as function spaces); ceiling = cognitive > 15'}
dump('complexity', cx)

# A4 graph
cyc_rows = json.load(open(f'{D}/g-cycles.json', encoding='utf-8'))
fin = json.load(open(f'{D}/g-fanin.json', encoding='utf-8'))
fout = json.load(open(f'{D}/g-fanout.json', encoding='utf-8'))
edges = json.load(open(f'{D}/g-edges.json', encoding='utf-8'))
dump('graph', {'cycles': len(cyc_rows), 'cycle_paths': [r['path'] for r in cyc_rows],
               'fan_in_top': [[r['callee'], r['n']] for r in fin][:20],
               'fan_out': [[r['from_crate'], r['n']] for r in fout], 'cross_unit_edges': edges[0]['n']})

# A5 dead — pinned classing recipe
zr = json.load(open(f'{D}/g-zeroref-raw.json', encoding='utf-8'))
P = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
def tpath(x):
    return '/tests/' in '/' + P.sub('', x['symbol']) or '/tests/' in '/' + norm(x['file'])
nontest = [x for x in zr if not tpath(x)]
classes = {'entry-point': [], 'trait-impl-dispatch': [], 'derive-attr-invoked': [], 'residual': []}
for x in nontest:
    s = P.sub('', x['symbol'])
    if s.endswith('main().'): classes['entry-point'].append(s)
    elif re.search(r'impl#\[[^\]]+\]\[[^\]]+\]', s): classes['trait-impl-dispatch'].append(s)
    elif s == 'harness/HarnessError#Json#': classes['derive-attr-invoked'].append(s + '  (#[from] serde_json::Error variant, harness/mod.rs:27)')
    else: classes['residual'].append(f"{s} @ {x['file']}:{x['def_line']}")
dump('dead', {'unused_deps': [['viola-fuzz (fuzz/Cargo.toml, own workspace)', 'arbitrary']],
              'zero_ref_all': len(zr), 'test_excluded': len(zr) - len(nontest),
              'fp_classes': {k: v for k, v in classes.items() if k != 'residual'},
              'zero_ref_candidates': len(classes['residual']), 'top': classes['residual'][:20],
              'note': 'candidates, never dead; test exclusion = tests/ segment in symbol path (SCIP prefix stripped) OR file path'})

# A6 coverage
cv = json.load(open(f'{D}/cov-raw.json', encoding='utf-8'))['data'][0]['totals']
dump('coverage', {'line': round(cv['lines']['percent'], 2), 'branch': None,
                  'functions': round(cv['functions']['percent'], 2), 'regions': round(cv['regions']['percent'], 2),
                  'lines_count': [cv['lines']['covered'], cv['lines']['count']],
                  'note': 'branch null: branch instrumentation not enabled (llvm-cov branches count 0); population = viola + viola-core (the project ignore regex excludes viola-e2e, fake agent, tests/support, fuzz)'})
print(json.dumps({'sizes': {k: sizes[k] for k in ('files', 'loc', 'file_p50', 'file_p90', 'file_max', 'over_800')},
                  'dup': {k: dup[k] for k in ('pct', 'clones', 'duplicated_lines', 'total_lines', 'split')},
                  'cx': {k: cx[k] for k in ('functions', 'cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max')},
                  'top_cx': cx['top'], 'top_sizes': sizes['top'], 'top_dup': dup['top'][:5],
                  'dead': len(classes['residual'])}, indent=0))
