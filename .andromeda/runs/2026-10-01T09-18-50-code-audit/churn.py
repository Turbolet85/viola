"""c-dead finalisation + B1 churn + B2 hotspots (pinned per collectors.md)."""
import collections, json, re, subprocess

R = '.andromeda/runs/2026-10-01T09-18-50-code-audit'
BASE = '69abc0d038732cff8d45602f82f8bebaf28bbb72'
EXCL = re.compile(r'^(scripts|docs|refs|fuzz|\.andromeda|\.claude|viola-0\.2\.0-incubator)/')

# A5 final: dead.py's classing + the three residuals reviewed by hand (recorded, not hidden)
dc = json.load(open(f'{R}/dead-classing.json', encoding='utf-8'))
review = {
 'Pty#child_pid().': 'trait method reached by dispatch: a trait DECLARATION called as viola_pty::Pty::child_pid(&pty) at src/cmd/run.rs:397',
 'events/kind_str().': 'derive/attr-invoked: #[serde(serialize_with = "kind_str")] at crates/viola-state/src/events.rs:33',
 'StateError#Encode#': 'derive/attr-invoked: a thiserror #[from] variant (crates/viola-state/src/lib.rs:22)',
}
reviewed = [[s, k, loc, review[s]] for s, k, loc in dc['residual'] if s in review]
left = [[s, k, loc] for s, k, loc in dc['residual'] if s not in review]
dead = {'unused_deps': [['viola-fuzz (fuzz/Cargo.toml, own workspace)', 'arbitrary']],
        'zero_ref_candidates': len(left), 'top': left[:10],
        'classing': {'raw': dc['raw'], 'tests_segment_excluded': dc['stage1_tests_excluded'],
                     'fp_classes': dc['fp_classes'], 'residual_reviewed_fp': reviewed}}
json.dump(dead, open(f'{R}/c-dead.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print('dead candidates', dead['zero_ref_candidates'])

# B1 churn — numstat over BASE..HEAD, *.rs, source-path filter; adds in a file's 2nd..nth touching commit = churn
log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{BASE}..HEAD', '--', '*.rs'],
                     capture_output=True, text=True, encoding='utf-8', check=True).stdout
seen, commits = collections.Counter(), collections.Counter()
adds = churned = 0
for line in log.splitlines():
    if not line or line.startswith('@'):
        continue
    a, d, path = line.split('\t', 2)
    if a == '-':
        continue  # binary
    if ' => ' in path:  # rename: count to the new path
        path = re.sub(r'\{([^{}]*) => ([^{}]*)\}', r'\2', path) if '{' in path else path.split(' => ')[1]
        path = path.replace('//', '/')
    if EXCL.search(path):
        continue
    a = int(a)
    adds += a
    if seen[path] > 0:
        churned += a
    seen[path] += 1
    commits[path] += 1
churn = {'pct': round(100 * churned / adds, 2) if adds else None, 'files_churned': sum(1 for p, n in commits.items() if n > 1),
         'adds': adds, 'churned_adds': churned, 'files_touched': len(commits)}
json.dump(churn, open(f'{R}/c-churn.json', 'w', encoding='utf-8', newline='\n'), indent=1)
print('churn', churn)

# B2 hotspots — commits x max per-function cognitive in the file (fallback: file KLOC never needed when cog > 0)
per = json.load(open(f'{R}/c-complexity-perfile.json', encoding='utf-8'))
hs = sorted(([p, float(n * per.get(p, 0))] for p, n in commits.items()), key=lambda r: (-r[1], r[0]))[:10]
json.dump({'hotspots': hs, 'commits': dict(commits.most_common(15))}, open(f'{R}/c-hotspots.json', 'w', encoding='utf-8', newline='\n'), indent=1)
print('hotspots', hs)
