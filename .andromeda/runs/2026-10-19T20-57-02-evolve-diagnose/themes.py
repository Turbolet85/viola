"""Pass A themes: tag problem facts (target + previous epoch) by keyword, and list the ok-degraded / not-run-stable records.

themes.py <run_dir>  — writes q-level.json, prints per theme the matching facts.
"""
import json
import re
import sys

RUN = sys.argv[1]
f = json.load(open(f'{RUN}/q-level-facts.json', encoding='utf-8'))
typed = json.load(open(f'{RUN}/q-typed.json', encoding='utf-8'))
unt = json.load(open(f'{RUN}/q-untyped.json', encoding='utf-8'))

THEMES = {
    'T1-tool-output-filtered': r'(\| ?tail|piped|through tail|tail -n|grep -v|/dev/null|into grep|through grep|into tail|read through tail|by their last line)',
    'T2-long-line-edit-by-script': r'(multi-KB|replace scripts?|replacement scripts?|python replace|splice|exact-match python)',
    'T3-backticked-pipe-reshaped': r'(reshaped|table cell)',
    'T4-removal-denied': r'(recursive rm|deny rule|rm -r|classifier)',
    'T5-primary-language-line': r'primary language',
    'T6-time-ahead-of-clock': r'(ahead of the clock|stamp-ahead|placeholder time|without a read of the clock)',
    'T7-preservation-guard': r'(preservation guard|guard \(entry|guard reads red|guard left|entry 15|entries 12 .literal probe. and 13)',
    'T8-profraw-or-unstable': r'(profraw|truncated profile|flak|cause not known|two-sided)',
    'T9-other-tree-viola': r'(another tree|overseer pair.s prototype|bridge)',
    'T10-test-filter-selects-nothing': r'(selects no|no tests to run|filter .* selects|did not select)',
}
out = {'themes': {}}
for name, pat in THEMES.items():
    rx = re.compile(pat, re.I)
    hit = {side: [x for x in f[side] if rx.search(x.get('note') or '')] for side in ('target', 'previous')}
    out['themes'][name] = hit
    print(f'== {name}: target {len(hit["target"])} · previous {len(hit["previous"])}')
    for side in ('target', 'previous'):
        for x in hit[side]:
            print(f"  [{side[:4]}] {x['id']} | {x['chunk']} | {x['skill']}/{x['step']} | {x['nature']}/{x['solution']} | {(x['note'] or '')[:150]}")

rns = [g for g in typed if g['key'].endswith('tooling.result-not-run-stable')]
out['result_not_run_stable'] = rns
json.dump(out, open(f'{RUN}/q-level-themes-raw.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

# ok-degraded step records and the previous epoch's not-run-stable records
led = [json.loads(l) for l in open('.andromeda/friction-log.ndjson', encoding='utf-8')]
print('== ok-degraded (Epoch 5)')
for r in led:
    if 'Epoch 5' in (r.get('epoch') or '') and r.get('kind') == 'step' and r.get('outcome') == 'ok-degraded':
        print(' ', r['id'], r.get('chunk'), r.get('skill'), r.get('step'), [p.get('signals') for p in r.get('produced') or [] if isinstance(p, dict)], (json.dumps(r.get('consumed'), ensure_ascii=False))[:300])
print('== result-not-run-stable, all epochs')
for r in led:
    if r.get('type') == 'tooling.result-not-run-stable':
        print(' ', r['id'], r.get('epoch'), r.get('chunk'), r.get('what')[:200])
