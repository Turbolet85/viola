"""A6 summarizer: cov-raw.json (cargo llvm-cov --json --summary-only) -> c-coverage.json {line, branch} totals only.

branch is null with a skips[] reason 'declined' when the report counts no branch (the project command instruments
none). Functions and regions ride along as informational fields, never judged.
"""
import json
import os
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
raw = json.load(open(os.path.join(RUN, 'cov-raw.json'), encoding='utf-8'))
t = raw['data'][0]['totals']
br = t.get('branches') or {}
res = {
    'line': round(t['lines']['percent'], 2),
    'branch': round(br['percent'], 2) if br.get('count') else None,
    'lines': {k: t['lines'][k] for k in ('count', 'covered')},
    'informational': {'functions': round(t['functions']['percent'], 2), 'regions': round(t['regions']['percent'], 2)},
    'branch_note': None if br.get('count') else 'the project coverage command does not instrument branches',
    'exit': int(sys.argv[1]) if len(sys.argv) > 1 else None,
}
with open(os.path.join(RUN, 'c-coverage.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(res, f, indent=1)
    f.write('\n')
print(json.dumps(res))
