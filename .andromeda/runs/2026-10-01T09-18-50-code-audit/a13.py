"""A1 duplication + A3 sizes summarizers (pinned per collectors.md)."""
import json, math, re, sys

R = sys.argv[1]
norm = lambda p: p.replace('\\', '/')
is_test = lambda p: re.search(r'(^|/)tests/', p) is not None

# A1 — jscpd
d = json.load(open(f'{R}/jscpd/jscpd-report.json', encoding='utf-8'))
t = d['statistics']['total']
rows, split = [], {'src': {'pairs': 0, 'lines': 0}, 'test': {'pairs': 0, 'lines': 0}, 'mixed': {'pairs': 0, 'lines': 0}}
for c in d['duplicates']:
    a, b = norm(c['firstFile']['name']), norm(c['secondFile']['name'])
    ta, tb = is_test(a), is_test(b)
    k = 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += c['lines']
    rows.append([f"{a}:{c['firstFile']['startLoc']['line']}", f"{b}:{c['secondFile']['startLoc']['line']}", c['lines']])
rows.sort(key=lambda r: (-r[2], r[0], r[1]))
dup = {'pct': round(t['percentage'], 2), 'duplicated_lines': t['duplicatedLines'], 'total_lines': t['lines'],
       'clones': t['clones'], 'top': rows[:10], 'split': split, 'sources': t['sources']}
json.dump(dup, open(f'{R}/c-duplication.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print('dup', {k: v for k, v in dup.items() if k != 'top'})

# A3 — tokei, one population (source-files.txt)
tk = json.load(open(f'{R}/tokei.json', encoding='utf-8'))
per = {}
for lang, v in tk.items():
    if lang == 'Total':
        continue
    for rep in v.get('reports', []):
        p = norm(rep['name'])
        per[p] = per.get(p, 0) + rep['stats']['code']
pop = [norm(l.strip()) for l in open(f'{R}/source-files.txt', encoding='utf-8') if l.strip()]
missing = [p for p in pop if p not in per]
vals = sorted(per[p] for p in pop if p in per)
nr = lambda p: vals[max(0, math.ceil(p / 100 * len(vals)) - 1)]
top = sorted(([p, per[p]] for p in pop if p in per), key=lambda r: (-r[1], r[0]))[:10]
sizes = {'file_p50': nr(50), 'file_p90': nr(90), 'file_max': vals[-1], 'over_800': sum(1 for v in vals if v > 800), 'top': top}
totals = {'loc': sum(vals), 'files': len(vals)}
json.dump({'sizes': sizes, 'totals': totals, 'missing': missing}, open(f'{R}/c-sizes.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print('sizes', {k: v for k, v in sizes.items() if k != 'top'}, totals, 'missing', missing)
print('top', top)
