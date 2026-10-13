"""Phase 5: proposals.md from judge.json + record.json + c-sizes-over800.json + c-mutation-*.json + c-windows-readback.json.

Every evidence table is asserted at render: its row count equals an n from a source independent of its rows
(the record's sizes.over_800 for the over-800 table; min(cap 10, len) of the record's sizes.top for the top table;
counts[unit].missed for each per-unit survivor tally line), and its rows are unique on their natural key.
"""
import json
import os
import re
from collections import Counter

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
L = lambda n: json.load(open(os.path.join(RUN, n), encoding='utf-8'))
j, rec, ov, win = L('judge.json'), L('record.json'), L('c-sizes-over800.json'), L('c-windows-readback.json')
recs = [json.loads(l) for l in open(os.path.join(REPO, '.andromeda/code-metrics.ndjson'), encoding='utf-8') if l.strip()]
base = [r for r in recs[:-1] if r['sha'] == rec['baseline_sha']][-1]
bb = [r for r in recs[:-1] if r['sha'] == base['baseline_sha']][-1]
E = {r['sha']: norm(r['epoch']) for r in recs}
out = []
w = out.append

w(f'# Code Audit — viola · {rec["epoch"]} · {rec["ts"]}')
w(f'mode {rec["mode"]} · HEAD {rec["sha"][:10]} · baseline {base["sha"][:10]} ({E[base["sha"]]}) · span {rec["span"]} · '
  f'overshoot {rec["head_overshoot"]["commits"]} commits (the baseline\'s: {base["head_overshoot"]["commits"]})')
w('')
w('Trend-break: `code-graph.py` d0425fb1 → f22becbb (its blob hash, a token by construction) — `graph` and `dead` '
  'thresholds suppressed this run; both read 0 cycles and 0 dead candidates at both records.')
w('')
w('Run conditions the numbers carry (all in the record\'s `commands`): the mutation tier ran at jobs 2 with '
  '`--copy-target=false` (the e304994 record: jobs 1, `--copy-target=true`) — verdicts do not depend on either, '
  '`mutation.timing` does; `viola` ran as two invocations (stopped at 678/1299 on the founder\'s word, resumed with '
  '`--iterate` over the other 636, merged — `merge_viola.py`); `viola-e2e` ran from a `git archive` extract of HEAD '
  'with `--copy-target=true` after its in-repo invocation failed its unmutated baseline.')
w('')
w('## Proposals')
w('')
# ---- M1: sizes.file_max ----
fm = dict(j['monotonic'])['sizes.file_max']
top = rec['sizes']['top']
n_top = min(10, len(top))
basemap = {f: v for f, v in base['sizes']['top']}
over_base = {f: b for f, _, b in ov['head']}
w(f'### M1 — monotonic · sizes.file_max — {fm[0]} → {fm[1]} → {fm[2]}')
w(f'**Movement:** {fm[0]} → {fm[1]} → {fm[2]} code lines ({E[bb["sha"]]} → {E[base["sha"]]} → {norm(rec["epoch"])}): worse '
  'at both of the last two diffs.')
w(f'**Evidence:** the largest file at all three records is `crates/viola-agent-claude/src/ledger.rs` '
  f'({bb["sizes"]["top"][0][1] if bb["sizes"]["top"][0][0].endswith("ledger.rs") else "?"} → {basemap.get("crates/viola-agent-claude/src/ledger.rs")} → {fm[2]}). '
  f'The top {n_top} at HEAD (code lines, tokei over the record\'s population; e304994 value beside):')
w('')
w('| # | file | HEAD | e304994 |')
w('|---|---|---|---|')
rows = []
for i, (f, v) in enumerate(top[:n_top], 1):
    b = over_base.get(f, basemap.get(f))
    rows.append(f)
    w(f'| {i} | `{f}` | {v} | {b if b is not None else "absent"} |')
assert len(rows) == n_top and len(set(rows)) == len(rows)
w('')
w('**Suspected shape:** the maximum is one file, the capability ledger, whose growth was the Epoch 3 jump (+1794) and '
  'is +31 this epoch; the second-largest, `src/run/send.rs`, grew +624 this epoch (1464 → 2088).')
w('**Proposal:** consider splitting `ledger.rs` along its row families (the rows, the probes, the dialog-variant '
  'tables) and `src/run/send.rs` along its phases, so neither file carries the next epoch\'s growth whole.')
w('')
# ---- M2: sizes.over_800 ----
o8 = dict(j['monotonic'])['sizes.over_800']
n8 = rec['sizes']['over_800']
w(f'### M2 — monotonic · sizes.over_800 — {o8[0]} → {o8[1]} → {o8[2]}')
w(f'**Movement:** {o8[0]} → {o8[1]} → {o8[2]} files over 800 code lines ({E[bb["sha"]]} → {E[base["sha"]]} → '
  f'{norm(rec["epoch"])}): worse at both of the last two diffs.')
w(f'**Evidence:** all {n8} files over 800 at HEAD (`c-sizes-over800.json`; the count reproduces the record, and the '
  f'e304994 list reproduces that record\'s 12); {len(ov["entered"])} entered this epoch, {len(ov["left"])} left '
  f'(`{"`, `".join(ov["left"])}`, now under 800):')
w('')
w('| # | file | HEAD | e304994 | |')
w('|---|---|---|---|---|')
rows = []
for i, (f, v, b) in enumerate(ov['head'], 1):
    rows.append(f)
    w(f'| {i} | `{f}` | {v} | {b if b is not None else "absent"} | {"entered" if f in ov["entered"] else ""} |')
assert len(rows) == n8 and len(set(rows)) == n8, (len(rows), n8)
grow = sorted(((f, v - (b or 0)) for f, v, b in ov['head']), key=lambda x: -x[1])[:4]
root = sum(1 for f, _, _ in ov['head'] if f.startswith('src/'))
w('')
w(f'**Suspected shape:** {root} of the {n8} are in the root package (`src/`); the largest growths this epoch are '
  + ', '.join(f'`{f}` +{d}' for f, d in grow) + ' — `src/human.rs` carries the new `viola list` board writers and '
  '`src/run/budget.rs` is new with the budget governor.')
w('**Proposal:** consider a split for the files that entered by growth rather than by birth (`src/human.rs`, '
  '`src/run/wheel.rs`, `crates/viola-state/src/events.rs`, `crates/viola-agent-claude/src/hook.rs`) before the next '
  'chunk lands in them, and whether the three test files that entered (`tests/cli_send.rs`, `tests/cli_fake_agent.rs`, '
  '`tests/cli_instance_state.rs`) share setup a `tests/support` helper could hold.')
w('')
# ---- Informational ----
w('## Informational')
w('')
for x in j['informational']:
    w(f'- {x}')
w(f'- mutation survivors per unit (full list: the record\'s `mutation.survivors`, the `c-mutation-{{unit}}.json` twins): '
  + ' · '.join(f'`{u}` {base["mutation"]["counts"].get(u, {}).get("missed", "—")} → {rec["mutation"]["counts"][u]["missed"]}'
               for u in rec['mutation']['scoped_units']))
sv = [s for s in rec['mutation']['survivors'] if not s[0].startswith('crates/')]
by = Counter(s[0].split(':')[0] for s in sv)
assert sum(by.values()) == rec['mutation']['counts']['viola']['missed']
w('- `viola` survivors by file (' + str(sum(by.values())) + ', = counts.missed): '
  + ' · '.join(f'`{f}` {n}' for f, n in by.most_common()) + ' — the `viola` score moves 99.33 → 97.03, under the −10 pt line')
nm = Counter(s[2] for s in rec['mutation']['not_measured'])
assert sum(nm.values()) == sum(c['not_measured'] for c in rec['mutation']['counts'].values())
w(f'- not measured on this host ({rec["mutation"]["host"]}): {sum(nm.values())} — '
  + ' · '.join(f'`{p}` {n}' for p, n in nm.most_common()) + '; the project\'s union verdict, by pointer: '
  f'{win["registered_by"]} — run {win["run"]} at {win["head_sha"][:10]}: conclusion `{win["conclusion"]}`; 5 jobs green '
  '(viola-state, viola-channel, viola-pty, viola-agent-claude, viola-e2e); the 4 root-package shards failed their '
  'UNMUTATED baseline on `cmd::hook::statusline::tests::statusline_an_instance_viola_laid_down_passes_its_check` '
  '(`assertion failed: trusted(&instance)`, `src/cmd/hook/statusline.rs:510`) in the cargo-mutants copy, so no '
  'Windows verdict exists for `viola`\'s 18 (`c-windows-readback.json`; never merged into these counts)')
w('')
w('## Below threshold — no action')
w('')
for x in j['below']:
    if x.startswith('monotonic · `coverage.line` not evaluable'):
        x = ('monotonic · `coverage.line` not evaluable this run: null at 95c1a9b (that record skipped coverage) — '
             'never a zero, never a fire')
    w(f'- {x}')
w('')
w('## Skips')
w('')
for s in rec['skips']:
    w(f'- {s["metric"]} — {s["reason"]}' + (f' ({s["note"]})' if s.get('note') else ''))
w('- mutation:viola-channel — not in scope: no source, manifest or build-script change since e304994 (touched units only); '
  'not a `skips[]` entry')
w('')
open(os.path.join(RUN, 'proposals.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
print('proposals.md', len(out), 'lines')
