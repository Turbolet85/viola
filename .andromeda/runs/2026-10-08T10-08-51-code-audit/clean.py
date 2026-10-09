"""Size discipline: keep the capped twins, delete this run's own raw outputs. Deletes ONLY inside this run dir
(every target is resolved and asserted under it first). Before deleting, the three readings of the viola-e2e
baseline timeout are excerpted into e2e-baseline-timeout.txt, and win-outcomes.json is cut to names and grades."""
import json, os, re, shutil, sys

R = os.path.realpath('.andromeda/runs/2026-10-08T10-08-51-code-audit')
assert R.endswith('/.andromeda/runs/2026-10-08T10-08-51-code-audit') and os.path.isdir(R)
S = sys.argv[1]  # the session scratch holding the Windows job logs (outside the repository; not deleted here)
ANSI = re.compile(r'\x1b\[[0-9;?]*[A-Za-z]')
T = 'boot_with_an_unknown_cli_version_is_verify_failed'

def grep(path, pats, strip=False):
    out = []
    for line in open(path, encoding='utf-8', errors='replace'):
        line = ANSI.sub('', line).rstrip('\r\n') if strip else line.rstrip('\n')
        if any(p in line for p in pats):
            out.append(line.strip()[:240])
    return out

ex = ['# The three readings of the viola-e2e baseline timeout (F1), each the raw lines of its own log.', '',
      '## 1. this host, the unmutated baseline of `cargo mutants --package viola-e2e` (mutants-viola-e2e/mutants.out/log/baseline.log)']
ex += grep(f'{R}/mutants-viola-e2e/mutants.out/log/baseline.log', (T, 'Summary [', 'test run failed'))
ex += ['', '## 2. this host, the coverage command, profile ci (cov.log)'] + grep(f'{R}/cov.log', (T, 'Summary ['))
ex += ['', '## 3. the Windows runner, job mutants (viola-e2e) 113260250076 of run 37761947926'] + \
      grep(f'{S}/win-e2e.log', (T, 'Summary [', 'no mutants were tested'), strip=True)
ex += ['', '## the report-only run with the test deselected, its baseline (mutants-viola-e2e-deselected/.../baseline.log)'] + \
      grep(f'{R}/mutants-viola-e2e-deselected/mutants.out/log/baseline.log', ('Summary [',))
ex += ['', '## the Windows `mutants (viola)` job 113260249970: baseline and the cancel'] + \
      grep(f'{S}/win-viola.log', ('Unmutated baseline', 'Found ', 'The operation was canceled'), strip=True)
open(f'{R}/e2e-baseline-timeout.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(ex) + '\n')

w = json.load(open(f'{R}/win-outcomes.json', encoding='utf-8'))
for j in w.values():
    j['rows'] = [[r['grade'], r['name']] for r in j['rows']]
    j.pop('tail', None)
json.dump(w, open(f'{R}/win-outcomes.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=0)

RAW = ['rca', 'jscpd', '__pycache__', 'tokei.json', 'cov-raw.json', 'cov.log', 'zero-ref-raw.json', 'rca.log', 'jscpd.log',
       'machete.log', 'machete-fuzz.log', 'machete-deps.json', 'mut-prebuild.log', 'mutsum-viola.out', 'win-classify.out',
       'cov.start', 'cov.end', 'cov.exit', 'record.json.tmp']
RAW += [n for n in os.listdir(R) if n.startswith('mutants-') or re.fullmatch(r'mut-viola[a-z0-9-]*\.log', n)]
gone = []
for n in sorted(set(RAW)):
    p = os.path.realpath(os.path.join(R, n))
    assert os.path.dirname(p) == R, p  # a direct child of this run dir, nothing else
    if os.path.islink(os.path.join(R, n)) or not os.path.lexists(p):
        continue
    shutil.rmtree(p) if os.path.isdir(p) else os.remove(p)
    gone.append(n)
print('deleted', len(gone), gone)
print('kept', sorted(os.listdir(R)))
