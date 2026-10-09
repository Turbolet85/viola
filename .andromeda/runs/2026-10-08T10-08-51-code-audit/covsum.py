"""A6 coverage summarizer: line % + branch % totals only, from cargo-llvm-cov's --json --summary-only export."""
import json, re

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
d = json.load(open(f'{R}/cov-raw.json', encoding='utf-8'))
t = d['data'][0]['totals']
log = open(f'{R}/cov.log', encoding='utf-8', errors='replace').read()
m = re.search(r'Summary \[\s*([0-9.]+)s\] (\d+) tests run: (\d+) passed(?: \((\d+) slow\))?, (\d+) skipped', log)
out = {'line': round(t['lines']['percent'], 2),
       'branch': None,  # the project's coverage command instruments no branches (branches.count == 0)
       'lines': [t['lines']['covered'], t['lines']['count']],
       'functions': [t['functions']['covered'], t['functions']['count'], round(t['functions']['percent'], 2)],
       'regions': [t['regions']['covered'], t['regions']['count'], round(t['regions']['percent'], 2)],
       'branches_count': t['branches']['count'],
       'files': len(d['data'][0]['files']),
       'exit': int(open(f'{R}/cov.exit').read().strip()),
       'started': open(f'{R}/cov.start').read().strip(), 'ended': open(f'{R}/cov.end').read().strip(),
       'tests': {'wall_s': float(m.group(1)), 'run': int(m.group(2)), 'passed': int(m.group(3)),
                 'slow': int(m.group(4) or 0), 'skipped': int(m.group(5))} if m else None}
json.dump(out, open(f'{R}/c-coverage.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(out)
