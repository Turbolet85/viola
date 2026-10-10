"""The audit's pinned recipe (cover.py) read over the workflow's matrix items on the tree as it stands: per item
and per host, the mutants whose whole span the host excludes, each with its predicate. Read-only:
`cargo mutants --list --json` builds nothing and writes nothing. The phase run dir's forecast.py reads a matrix
item as two lines; an item now carries a `label:` line between them, so this copy reads three.
Run from the repository root: python -X utf8 <this file> > <a json file>."""
import collections, importlib.util, json, re, subprocess, sys

REPO = '.'
spec = importlib.util.spec_from_file_location('cover', REPO + '/.andromeda/runs/2026-10-08T10-08-51-code-audit/cover.py')
cover = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cover)

yml = open(REPO + '/.github/workflows/windows-mutants.yml', encoding='utf-8').read()
items = re.findall(r'- package: (\S+)\n\s+label: (\S+)\n\s+files: (.+)', yml)
cfgs = {'windows': cover.host_cfg(REPO, 'x86_64-pc-windows-msvc'), 'linux': cover.host_cfg(REPO)}
doc = {'items': [], 'total': collections.Counter()}
for package, label, files in items:
    cmd = ['cargo', 'mutants', '--list', '--json', '--package', package, '--features', 'fake-agent']
    for f in files.split():
        cmd += ['--file', f]
    out = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True)
    if out.returncode != 0:
        doc['items'].append({'label': label, 'package': package, 'list_failed': out.stderr[-300:]})
        continue
    mutants = json.loads(out.stdout)
    item = {'label': label, 'package': package, 'files': files.split(), 'mutants': len(mutants)}
    for host, cfg in cfgs.items():
        rows = []
        for m in mutants:
            p = cover.cover(REPO, m, cfg)
            if p:
                rows.append({'name': m['name'], 'cfg': p})
        item[host] = rows
        doc['total'][host] += len(rows)
    doc['total']['mutants'] += len(mutants)
    doc['items'].append(item)
json.dump(doc, sys.stdout, indent=1)
for item in doc['items']:
    print('%-20s mutants %3d · excluded on windows %2d · on linux %2d' % (
        item['label'], item.get('mutants', -1), len(item.get('windows', [])), len(item.get('linux', []))), file=sys.stderr)
print('total', dict(doc['total']), file=sys.stderr)
