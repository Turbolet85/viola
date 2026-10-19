#!/usr/bin/env python3
"""Code-audit collectors A1-A3, B1-B2 (pinned summarizers, collectors.md).

usage:
  collect.py pop   <tree_root> <sha>                    -> prints the population count
  collect.py sizes <tree_root> <sha> <out.json>
  collect.py dup   <jscpd-report.json> <out.json>
  collect.py cx    <tree_root> <sha> <rca_dir> <out.json>
  collect.py churn <base_sha> <head> <out.json>
  collect.py hot   <c-churn.json> <c-cx.json> <out.json>
Every percentile is nearest-rank over the sorted values.
"""
import json, math, os, re, subprocess, sys

EXCL = re.compile(r'^(scripts|docs|refs|fuzz|\.andromeda|\.claude|viola-0\.2\.0-incubator)/')
TEST = re.compile(r'(^|/)tests/')


def population(sha):
    out = subprocess.run(['git', 'ls-tree', '-r', '--name-only', sha], capture_output=True, text=True, check=True).stdout
    return sorted(p for p in out.splitlines() if p.endswith('.rs') and not EXCL.match(p))


def nr(vals, q):
    s = sorted(vals)
    if not s:
        return None
    return s[max(1, math.ceil(q * len(s))) - 1]


def sizes(root, sha, out):
    pop = population(sha)
    res = subprocess.run(['tokei', '--output', 'json', *pop], cwd=root, capture_output=True, text=True, check=True).stdout
    d = json.loads(res)
    per = {}
    for lang, body in d.items():
        if lang == 'Total':
            continue
        for rep in body.get('reports', []):
            name = os.path.relpath(rep['name'], root) if os.path.isabs(rep['name']) else rep['name']
            name = name[2:] if name.startswith('./') else name
            per[name] = per.get(name, 0) + rep['stats']['code']
    vals = list(per.values())
    top = sorted(per.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
    s = {'file_p50': nr(vals, .5), 'file_p90': nr(vals, .9), 'file_max': max(vals), 'over_800': sum(v > 800 for v in vals),
         'top': [[k, v] for k, v in top], 'totals': {'loc': sum(vals), 'files': len(vals)}, 'population': len(pop)}
    json.dump(s, open(out, 'w'), indent=1)
    print('sizes', s['totals'], 'p50', s['file_p50'], 'p90', s['file_p90'], 'max', s['file_max'], 'over_800', s['over_800'])


def kind(a, b):
    ta, tb = bool(TEST.search(a)), bool(TEST.search(b))
    return 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'


def dup(report, out):
    d = json.load(open(report))
    t = d['statistics']['total']
    rows, split = [], {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
    for c in d['duplicates']:
        a = f"{c['firstFile']['name']}:{c['firstFile']['startLoc']['line']}"
        b = f"{c['secondFile']['name']}:{c['secondFile']['startLoc']['line']}"
        rows.append([a, b, c['lines']])
        k = kind(c['firstFile']['name'], c['secondFile']['name'])
        split[k]['pairs'] += 1
        split[k]['lines'] += c['lines']
    rows.sort(key=lambda r: (-r[2], r[0], r[1]))
    s = {'pct': round(t['percentage'], 2), 'duplicated_lines': t['duplicatedLines'], 'total_lines': t['lines'],
         'clones': t['clones'], 'top': rows[:10], 'split': split,
         'split_rule': "test = both files carry a (^|/)tests/ path segment; src = neither; mixed = one"}
    assert s['clones'] == len(d['duplicates']), (s['clones'], len(d['duplicates']))
    json.dump(s, open(out, 'w'), indent=1)
    print('dup', s['pct'], s['duplicated_lines'], s['total_lines'], s['clones'], split)


def cx(root, sha, rca_dir, out):
    fns = []
    def walk(sp, file):
        if sp['kind'] == 'function':
            fns.append({'fn': sp['name'], 'file': f"{file}:{sp['start_line']}",
                        'cog': sp['metrics']['cognitive']['sum'], 'cyc': sp['metrics']['cyclomatic']['sum'], 'path': file})
        for c in sp['spaces']:
            walk(c, file)
    missing = []
    for p in population(sha):
        f = os.path.join(rca_dir, p + '.json')
        if not os.path.isfile(f):
            missing.append(p)
            continue
        walk(json.load(open(f)), p)
    cog = [f['cog'] for f in fns]
    cyc = [f['cyc'] for f in fns]
    top = sorted(fns, key=lambda f: (-f['cog'], f['file']))[:10]
    per_file = {}
    for f in fns:
        per_file[f['path']] = max(per_file.get(f['path'], 0), f['cog'])
    s = {'functions': len(fns), 'cyclomatic_p50': nr(cyc, .5), 'cyclomatic_p90': nr(cyc, .9),
         'cognitive_p50': nr(cog, .5), 'cognitive_p90': nr(cog, .9), 'over_ceiling': sum(c > 15 for c in cog),
         'max': {'fn': top[0]['fn'], 'file': top[0]['file'], 'val': top[0]['cog']},
         'top': [{'fn': f['fn'], 'file': f['file'], 'value': f['cog']} for f in top],
         'per_file_max_cognitive': per_file, 'missing_rca': missing}
    json.dump(s, open(out, 'w'), indent=1)
    print('cx fns', len(fns), 'over_ceiling', s['over_ceiling'], 'max', s['max'], 'missing', len(missing))


def churn(base, head, out):
    log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..{head}', '--', '*.rs'],
                         capture_output=True, text=True, check=True).stdout
    seen, adds_all, churned, commits = {}, 0, 0, {}
    for line in log.splitlines():
        if not line or line.startswith('@'):
            continue
        a, d, path = line.split('\t', 2)
        if a == '-':
            continue
        if '=>' in path:  # rename: count to the new path
            m = re.match(r'(.*)\{(.*) => (.*)\}(.*)', path)
            path = (m.group(1) + m.group(3) + m.group(4)).replace('//', '/') if m else path.split(' => ')[1]
        if EXCL.match(path) or not path.endswith('.rs'):
            continue
        a = int(a)
        adds_all += a
        commits[path] = commits.get(path, 0) + 1
        if seen.get(path):
            churned += a
        seen[path] = True
    s = {'pct': round(100 * churned / adds_all, 2) if adds_all else None, 'files_churned': sum(n >= 2 for n in commits.values()),
         'adds_all': adds_all, 'adds_churned': churned, 'files_touched': len(commits), 'commits_per_file': commits,
         'rule': 'per file, adds in its 2nd..nth touching commits are churn; files_churned = files touched by >= 2 commits'}
    json.dump(s, open(out, 'w'), indent=1)
    print('churn', s['pct'], 'files_churned', s['files_churned'], 'touched', s['files_touched'])


def hot(ch, cxf, out):
    c, x = json.load(open(ch)), json.load(open(cxf))
    pf = x['per_file_max_cognitive']
    sc = {f: n * pf.get(f, 0) for f, n in c['commits_per_file'].items() if f in pf}
    top = sorted(sc.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
    s = {'hotspots': [[k, float(v)] for k, v in top], 'rule': 'commits in the churn window x max per-function cognitive in the file'}
    json.dump(s, open(out, 'w'), indent=1)
    print('hot', s['hotspots'])


if __name__ == '__main__':
    cmd, *a = sys.argv[1:]
    if cmd == 'pop':
        print(len(population(a[1])))
    else:
        {'sizes': sizes, 'dup': dup, 'cx': cx, 'churn': churn, 'hot': hot}[cmd](*a)
