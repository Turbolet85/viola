"""A6 coverage, run 2 (overseer direction: re-run after the mutation units with no foreign cargo/rustc). It failed
too, so coverage is NOT measured this run; c-coverage.json carries both runs' evidence (capped)."""
import collections, json, re

R = '.andromeda/runs/2026-10-01T09-18-50-code-audit'

def fails(p):
    t = open(p, encoding='utf-8', errors='replace').read()
    return t, set(re.findall(r'^\s+(?:FAIL|TIMEOUT)(?: \+ LEAK)? \[[^\]]*\] \(\s*\d+/\d+\) (\S+ \S+)', t, re.M))

t1, f1 = fails(f'{R}/cov-run1-stderr.txt')
t2, f2 = fails(f'{R}/cov-stderr.txt')
watch = [l.split() for l in open(f'{R}/cov-watch.log', encoding='utf-8') if l.strip()]
test_phase = [w for w in watch if w[0] >= '2026-10-01T12:03:38Z']  # Starting 985 tests: exit 12:08:54Z minus 316.4 s
run1 = json.load(open(f'{R}/c-coverage-run1.json', encoding='utf-8'))
out = {
 'line': None, 'branch': None,
 'status': 'not measured: the project coverage command fails at HEAD on this host in two runs',
 'command': f"cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path {R}/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\\\]viola-e2e|tests[/\\\\]support|fuzz[/\\\\])'  # run by path: {R}/cov.sh (twice)",
 'run1': {'window': '2026-10-01T09:20Z-10:00Z', 'nextest_summary': run1['nextest_summary'], 'failed': len(f1),
          'note': 'a foreign project build (cargo xtask pre-push:linux, toolchain 1.95) ran on the host (overseer measurement)'},
 'run2': {'window': 'build 11:33-11:58Z, tests 12:03:38-12:08:54Z', 'nextest_summary': re.search(r'Summary \[[^\]]*\] (.*)', t2).group(1).strip(),
          'failed': len(f2), 'failed_by_test_binary': dict(collections.Counter(x.split()[0] for x in f2).most_common()),
          'panic_first_lines': dict(collections.Counter(m.strip()[:120] for m in re.findall(r"panicked at [^\n]*\n\s*([^\n]+)", t2)).most_common(8)),
          'foreign_rustc_samples_in_test_phase': sum(int(w[1].split('=')[1]) for w in test_phase),
          'test_phase_samples': len(test_phase),
          'cpu_pct_in_test_phase': [min(int(w[2].split('=')[1]) for w in test_phase), max(int(w[2].split('=')[1]) for w in test_phase)],
          'foreign_rustc_seen': [' '.join(w) for w in watch if not w[1].endswith('=0')],
          'watcher': f'{R}/cov-watch.sh (rustc.exe outside the 1.98.1 toolchain path + total CPU, every 20 s)'},
 'overlap': {'both': len(f1 & f2), 'only_run1': len(f1 - f2), 'only_run2': sorted(f2 - f1)},
 'contrast': 'the unmutated viola package suite PASSED as cargo-mutants\' baseline in a copied tree (gitignored files not copied) at 10:54Z (367/367 tests across 26 binaries passed, 36 s build + 10.9 s test, NEXTEST_PROFILE=mutants); the same binaries fail in-repo under coverage (twice) and in a plain in-repo nextest probe (2/2, with every inherited CLAUDE*/VIOLA_*/ANTHROPIC* var unset) - a location/local-state-dependent red, cause unknown',
 'skips': [{'metric': 'coverage', 'reason': 'baseline-test-failure',
            'note': 'the project coverage command\'s test run failed both times (90, then 80 failed; 79 common); run 2\'s test phase had 0 foreign rustc in 14 samples and 8-24% CPU, so the host-load hypothesis is not supported for run 2'},
           {'metric': 'coverage.branch', 'reason': 'declined', 'note': 'the project coverage command does not instrument branches (as at the 69abc0d record)'}],
}
json.dump(out, open(f'{R}/c-coverage.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(json.dumps({k: out['run2'][k] for k in ('nextest_summary', 'failed', 'foreign_rustc_samples_in_test_phase', 'test_phase_samples', 'cpu_pct_in_test_phase')}), out['overlap']['both'], out['overlap']['only_run2'])
