"""B1 churn + B2 hotspots + A5 dead-code final, pinned summarizers.

B1: git log --reverse --numstat --format=@%H {baseline}..HEAD -- "*.rs", minus the excluded prefixes; per file,
adds in its 2nd..nth touching commits are churn; churn % = churned adds / all adds; binary '-' rows skipped;
a rename line counts to the new path. files_churned = files touched by 2+ commits.
B2: per-file commit count x max per-function cognitive in the file (c-cx.json); top 10.
"""
import json
import os
import re
import subprocess

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
BASE = 'e304994ae0413c5cf5bf679a5b3d19abded0e472'
EXCL = re.compile(r'^(scripts|docs|refs|fuzz|\.andromeda|\.claude|viola-0\.2\.0-incubator)/')


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{BASE}..HEAD', '--', '*.rs'],
                     cwd=REPO, capture_output=True, text=True, check=True).stdout
commits, seen, adds_all, adds_churn = 0, {}, 0, 0
per_file_adds = {}
for line in log.splitlines():
    if line.startswith('@'):
        commits += 1
        continue
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-':
        continue
    path = newpath(parts[2])
    if EXCL.match(path) or not path.endswith('.rs'):
        continue
    a = int(parts[0])
    n = seen.get(path, 0) + 1
    seen[path] = n
    adds_all += a
    per_file_adds[path] = per_file_adds.get(path, 0) + a
    if n >= 2:
        adds_churn += a
churn = {'pct': round(100 * adds_churn / adds_all, 2) if adds_all else None,
         'files_churned': sum(1 for v in seen.values() if v >= 2),
         'commits_in_window': commits, 'files_touched': len(seen), 'adds_all': adds_all, 'adds_churned': adds_churn,
         'top_commits': sorted(([p, n] for p, n in seen.items()), key=lambda x: (-x[1], x[0]))[:10]}
with open(os.path.join(RUN, 'c-churn.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(churn, f, indent=1)
    f.write('\n')

cx = json.load(open(os.path.join(RUN, 'c-cx.json'), encoding='utf-8'))['per_file_max_cognitive']
hot = sorted(([p, float(n * cx[p])] for p, n in seen.items() if p in cx), key=lambda x: (-x[1], x[0]))[:10]
with open(os.path.join(RUN, 'c-hotspots.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump({'hotspots': hot, 'files_without_complexity': sorted(p for p in seen if p not in cx)}, f, indent=1)
    f.write('\n')

raw = json.load(open(os.path.join(RUN, 'c-dead-raw.json'), encoding='utf-8'))
FP = {
    'rust-analyzer cargo viola-pty 0.1.0 Pty#child_pid().':
        ['trait declaration reached by dispatch', 'src/cmd/verify/typed.rs:419 self.lock().child_pid()'],
    'rust-analyzer cargo viola-state 0.1.0 events/kind_str().':
        ['derive/attr-invoked', 'crates/viola-state/src/events.rs:35 #[serde(serialize_with = "kind_str")]'],
    'rust-analyzer cargo viola-state 0.1.0 StateError#Encode#':
        ['derive/attr-invoked', 'crates/viola-state/src/lib.rs:26 Encode(#[from] serde_json::Error)'],
    'rust-analyzer cargo viola 0.1.0 cmd/send/SendArgs#text.':
        ['derive/attr-invoked', 'src/cmd/send.rs:37 #[arg(hide = true, value_parser = refuse_text_argument)] (clap derive)'],
    'rust-analyzer cargo viola 0.1.0 human/MIRROR_WORD.':
        ['inline format-string capture', 'src/human.rs:50 {word:<MIRROR_WORD$}'],
    'rust-analyzer cargo viola 0.1.0 human/LIVE_WIDTH.':
        ['inline format-string capture', 'src/human.rs:265 {live:<LIVE_WIDTH$}'],
    'rust-analyzer cargo viola 0.1.0 human/STATUS_WIDTH.':
        ['inline format-string capture', 'src/human.rs:265 {status:<STATUS_WIDTH$}'],
    'rust-analyzer cargo viola 0.1.0 human/WHEEL_WIDTH.':
        ['inline format-string capture', 'src/human.rs:265 {wheel:<WHEEL_WIDTH$}'],
    'rust-analyzer cargo viola 0.1.0 human/DIALOG_WIDTH.':
        ['inline format-string capture', 'src/human.rs:266 {dialog:<DIALOG_WIDTH$}'],
}
reviewed, left = [], []
for r in raw['residual']:
    if r['symbol'] in FP:
        reviewed.append({**r, 'class': FP[r['symbol']][0], 'witness': FP[r['symbol']][1]})
    else:
        left.append(r)
machete = open(os.path.join(RUN, 'machete.txt'), encoding='utf-8').read()
unused = []
cur = None
for line in machete.splitlines():
    m = re.match(r'^(\S+) -- (\S+):$', line)
    if m:
        cur = m.groups()
    elif cur and line.startswith('\t'):
        unused.append([f'{cur[0]} ({cur[1].lstrip("./")}, own workspace)' if cur[1].startswith('./fuzz/') else
                       f'{cur[0]} ({cur[1].lstrip("./")})', line.strip()])
dead = {'unused_deps': unused, 'zero_ref_candidates': len(left), 'top': [[r['symbol'], r['file']] for r in left][:20],
        'classing': {'raw': raw['raw'], 'excluded_tests_union': raw['excluded_tests_union'],
                     'stage1': raw['stage1'], 'stage2_classes': raw['stage2_classes'],
                     'residual_reviewed_fp': reviewed}}
with open(os.path.join(RUN, 'c-dead.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(dead, f, ensure_ascii=False, indent=1)
    f.write('\n')
print(json.dumps({'churn': {k: churn[k] for k in ('pct', 'files_churned', 'commits_in_window', 'files_touched')},
                  'hotspots': hot, 'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates')}}, indent=1))
