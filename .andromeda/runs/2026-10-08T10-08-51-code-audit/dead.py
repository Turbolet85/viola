"""A5 zero-ref classing (collectors.md A5 CLASSING RECIPE): stage 1 excludes test symbols by a `tests/`
PATH SEGMENT in the prefix-stripped SYMBOL path OR the FILE path (the union; a leading segment counts);
stage 2 classes the false-positive families; the residual is the reportable candidate count."""
import collections, json, re, sys

R = sys.argv[1]
z = json.load(open(f'{R}/zero-ref-raw.json', encoding='utf-8'))
PREFIX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
SEG = re.compile(r'(^|/)tests/')

stage1, kept = 0, []
for r in z:
    sp = PREFIX.sub('', r['symbol'])
    if SEG.search(sp) or SEG.search(r['file']):
        stage1 += 1
        continue
    kept.append(dict(r, sp=sp))

def klass(r):
    sp, f = r['sp'], r['file']
    if r['kind'] == 'module':
        return 'module (not a callable; reached by path)'
    if re.search(r'(^|/)main\(\)\.$', sp) or f.startswith('src/bin/') and sp.endswith('main().'):
        return 'entry point'
    if re.search(r'impl#\[[^\]]*\]\[[^\]]+\]', sp):
        return 'trait-impl method (dispatch)'
    if f.startswith('crates/viola-e2e/') or '/test_support' in f or 'test_support' in sp:
        return 'test-only helper (test crate / test-support)'
    return None

classes, residual = collections.Counter(), []
for r in kept:
    c = klass(r)
    if c:
        classes[c] += 1
    else:
        residual.append(r)

out = {'raw': len(z), 'stage1_tests_excluded': stage1, 'after_stage1': len(kept),
       'fp_classes': dict(classes), 'residual_before_review': len(residual),
       'residual': [[r['sp'], r['kind'], f"{r['file']}:{r['def_line']}"] for r in residual]}
json.dump(out, open(f'{R}/dead-classing.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print({k: v for k, v in out.items() if k != 'residual'})
for row in out['residual']:
    print(row)
