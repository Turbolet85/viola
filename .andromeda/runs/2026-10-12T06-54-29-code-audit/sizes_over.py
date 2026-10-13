"""Evidence for the sizes proposals: every population file over 800 code lines at HEAD and at the baseline sha.

collect.py's own population and tokei call, per file. The baseline tree is the population files only, extracted by
`git archive {BASE} -- <files>` into <extract-dir> (outside the repository; argv[1]). Asserts each over-800 count
equals the matching record's sizes.over_800 (19 at HEAD from c-sizes.json, 12 at e304994 from the ledger), and
file_max likewise. Writes c-sizes-over800.json.
"""
import json
import os
import subprocess
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, RUN)
import collect  # noqa: E402

REPO = collect.REPO
BASE = 'e304994ae0413c5cf5bf679a5b3d19abded0e472'


def per_file(root, rev):
    files = collect.population(rev)
    rep = json.loads(subprocess.run(['tokei', '--output', 'json', *files], cwd=root, capture_output=True, text=True,
                                    check=True).stdout)
    per = {}
    for lang, d in rep.items():
        if lang == 'Total' or not isinstance(d, dict):
            continue
        for r in d.get('reports', []):
            name = os.path.relpath(os.path.join(root, r['name']), root).replace(os.sep, '/')
            per[name] = per.get(name, 0) + r['stats']['code']
    return {f: per.get(f, 0) for f in files}


ext = sys.argv[1]
os.makedirs(ext, exist_ok=False)
bfiles = collect.population(BASE)
arc = subprocess.Popen(['git', 'archive', BASE, '--', *bfiles], cwd=REPO, stdout=subprocess.PIPE)
subprocess.run(['tar', '-x', '-C', ext], stdin=arc.stdout, check=True)
assert arc.wait() == 0
now, base = per_file(REPO, 'HEAD'), per_file(ext, BASE)
cur = json.load(open(os.path.join(RUN, 'c-sizes.json'), encoding='utf-8'))['sizes']
brec = [json.loads(l) for l in open(os.path.join(REPO, '.andromeda/code-metrics.ndjson'), encoding='utf-8')
        if l.strip() and json.loads(l).get('sha') == BASE][-1]['sizes']
over_now = sorted(((f, v, base.get(f)) for f, v in now.items() if v > 800), key=lambda x: (-x[1], x[0]))
over_base = sorted(((f, v) for f, v in base.items() if v > 800), key=lambda x: (-x[1], x[0]))
assert len(over_now) == cur['over_800'] and max(now.values()) == cur['file_max'], (len(over_now), cur)
assert len(over_base) == brec['over_800'] and max(base.values()) == brec['file_max'], (len(over_base), brec)
res = {'head': [[f, v, b] for f, v, b in over_now], 'head_n': len(over_now),
       'baseline': [list(x) for x in over_base], 'baseline_n': len(over_base),
       'entered': [f for f, _, _ in over_now if f not in dict(over_base)],
       'left': [f for f, _ in over_base if f not in {x[0] for x in over_now}],
       'note': '[file, code lines at HEAD, code lines at e304994 (null: absent there)]'}
with open(os.path.join(RUN, 'c-sizes-over800.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(res, f, indent=1)
    f.write('\n')
print(json.dumps(res, indent=0))
