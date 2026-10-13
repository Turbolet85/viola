"""Tier A sizes / duplication / complexity collectors with their pinned summarizers.

usage: collect.py {sizes|dup|cx} <tree-root> <rev> <out-json> [<work-dir>]
  tree-root  the tree measured (the repo, or a `git archive` extract of a past sha for the control)
  rev        the git revision whose file list is the population (HEAD, or the past sha)
Population (collectors.md, Source paths; the ledger's commands.sizes since the 95c1a9b record):
  `git ls-tree -r --name-only <rev>` *.rs minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/
Percentiles: nearest-rank on the sorted values.
"""
import json
import math
import os
import re
import shutil
import subprocess
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
EXCL = re.compile(r'^(scripts|docs|refs|fuzz|\.andromeda|\.claude|viola-0\.2\.0-incubator)/')
JSCPD_IGNORE = 'fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**,viola-0.2.0-incubator/**'


def population(rev):
    out = subprocess.run(['git', 'ls-tree', '-r', '--name-only', rev], cwd=REPO, capture_output=True, text=True,
                         check=True).stdout.split('\n')
    return sorted(p for p in out if p.endswith('.rs') and not EXCL.match(p))


def nr(vals, p):
    v = sorted(vals)
    return v[max(0, math.ceil(p / 100 * len(v)) - 1)] if v else None


def is_test(path):
    return bool(re.search(r'(^|/)tests/', path))


def sizes(root, rev, work):
    files = population(rev)
    raw = subprocess.run(['tokei', '--output', 'json', *files], cwd=root, capture_output=True, text=True,
                         check=True).stdout
    rep = json.loads(raw)
    per = {}
    for lang, d in rep.items():
        if lang == 'Total' or not isinstance(d, dict):
            continue
        for r in d.get('reports', []):
            name = os.path.relpath(os.path.join(root, r['name']), root).replace(os.sep, '/')
            per[name] = per.get(name, 0) + r['stats']['code']
    vals = [per.get(f, 0) for f in files]
    top = sorted(((f, per.get(f, 0)) for f in files), key=lambda x: (-x[1], x[0]))[:10]
    return {'population_files': len(files), 'tokei_files': len(per),
            'totals': {'loc': sum(vals), 'files': len(files)},
            'sizes': {'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals),
                      'over_800': sum(1 for v in vals if v > 800), 'top': [list(t) for t in top]}}


def dup(root, rev, work):
    out = os.path.join(work, 'jscpd')
    subprocess.run(['jscpd', '--reporters', 'json', '--output', out, '--format', 'rust', '--silent',
                    '--ignore', JSCPD_IGNORE, '.'], cwd=root, capture_output=True, text=True, check=True)
    with open(os.path.join(out, 'jscpd-report.json'), encoding='utf-8') as f:
        rep = json.load(f)
    tot = rep['statistics']['total']
    rows, split = [], {'src': {'pairs': 0, 'lines': 0}, 'test': {'pairs': 0, 'lines': 0},
                       'mixed': {'pairs': 0, 'lines': 0}}
    for d in rep.get('duplicates', []):
        a, b = d['firstFile'], d['secondFile']
        an, bn = a['name'].replace(os.sep, '/'), b['name'].replace(os.sep, '/')
        k = 'test' if is_test(an) and is_test(bn) else 'src' if not is_test(an) and not is_test(bn) else 'mixed'
        split[k]['pairs'] += 1
        split[k]['lines'] += d['lines']
        rows.append([f"{an}:{a['start']}", f"{bn}:{b['start']}", d['lines']])
    rows.sort(key=lambda r: (-r[2], r[0], r[1]))
    shutil.rmtree(out)
    return {'duplication': {'pct': tot['percentage'], 'duplicated_lines': tot['duplicatedLines'],
                            'total_lines': tot['lines'], 'clones': tot['clones'], 'top': rows[:10],
                            'split': split}, 'all_pairs': len(rows)}


def walk(space, path, acc):
    if space.get('kind') in ('function', 'closure'):
        m = space['metrics']
        acc.append({'fn': space.get('name'), 'file': f"{path}:{space.get('start_line')}",
                    'cyc': m['cyclomatic']['sum'], 'cog': m['cognitive']['sum'], 'path': path})
    for s in space.get('spaces', []) or []:
        walk(s, path, acc)


def cx(root, rev, work):
    files = population(rev)
    out = os.path.join(work, 'rca')
    os.makedirs(out, exist_ok=True)
    args = ['rust-code-analysis-cli', '-m', '-O', 'json', '-o', out]
    for f in files:
        args += ['-p', f]
    subprocess.run(args, cwd=root, capture_output=True, text=True, check=True)
    acc = []
    for dp, _, fs in os.walk(out):
        for fn in fs:
            with open(os.path.join(dp, fn), encoding='utf-8') as f:
                top = json.load(f)
            name = top.get('name', '').replace(os.sep, '/')
            name = os.path.relpath(os.path.join(root, name), root).replace(os.sep, '/') if not os.path.isabs(name) \
                else os.path.relpath(name, root).replace(os.sep, '/')
            walk(top, name, acc)
    shutil.rmtree(out)
    cyc = [a['cyc'] for a in acc]
    cog = [a['cog'] for a in acc]
    top = sorted(acc, key=lambda a: (-a['cog'], a['file']))[:10]
    per_file = {}
    for a in acc:
        per_file[a['path']] = max(per_file.get(a['path'], 0), a['cog'])
    return {'functions': len(acc), 'files_read': len(per_file),
            'complexity': {'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90),
                           'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
                           'over_ceiling': sum(1 for v in cog if v > 15),
                           'max': {'fn': top[0]['fn'], 'file': top[0]['file'], 'val': top[0]['cog']},
                           'top': [{'fn': a['fn'], 'file': a['file'], 'value': a['cog']} for a in top]},
            'per_file_max_cognitive': dict(sorted(per_file.items()))}


if __name__ == '__main__':
    which, root, rev, dest = sys.argv[1:5]
    work = os.path.abspath(sys.argv[5] if len(sys.argv) > 5 else os.path.dirname(os.path.abspath(dest)))
    res = {'sizes': sizes, 'dup': dup, 'cx': cx}[which](os.path.abspath(root), rev, work)
    with open(dest, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(res, f, ensure_ascii=False, indent=1)
        f.write('\n')
    print(json.dumps({k: v for k, v in res.items() if k != 'per_file_max_cognitive'}, ensure_ascii=False)[:1500])
