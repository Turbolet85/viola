"""Schema-gap fill: the baseline record (sha 69abc0d) carries no mutation.not_measured. Recompute at its sha:
cargo-mutants --list --json (the tool's list mode) over the extracted baseline tree, each baseline survivor
matched to its listed mutant by (site, text), then the pinned cover() against that tree."""
import json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mutsum import cover, host_cfg, mutant_text, site

BASE = sys.argv[1]  # the extracted baseline tree
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
b = [r for r in recs if r['sha'] == '69abc0d038732cff8d45602f82f8bebaf28bbb72'][-1]
surv = b['mutation']['survivors']
cfg = host_cfg(os.getcwd())  # the project's pinned toolchain (same rust-toolchain.toml at both shas)
listed = {}
for unit in b['mutation']['scoped_units']:
    if unit not in b['mutation']['scores']:
        continue
    out = subprocess.run(['cargo', 'mutants', '--list', '--json', '-p', unit], cwd=BASE, capture_output=True,
                         text=True, encoding='utf-8', check=True).stdout
    for m in json.loads(out):
        listed.setdefault((site(m), mutant_text(m)), m)
res = {'measured': [], 'not_measured': [], 'unmatched': []}
for s, t in surv:
    # the baseline record dropped the tool's ` in {function}` suffix on operator mutants: match it back
    m = listed.get((s, t)) or next((v for (ls, lt), v in listed.items() if ls == s and lt.startswith(t + ' in ')), None)
    if m is None:
        res['unmatched'].append([s, t]); continue
    p = cover(BASE, m, cfg)
    (res['not_measured'] if p else res['measured']).append([s, t] + ([f'cfg({p})'] if p else []))
json.dump(res, open('.andromeda/runs/2026-10-01T09-18-50-code-audit/c-baseline-not-measured.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(len(surv), 'baseline survivors ->', {k: len(v) for k, v in res.items()})
for k in ('not_measured', 'unmatched'):
    for r in res[k]:
        print(k, r)
