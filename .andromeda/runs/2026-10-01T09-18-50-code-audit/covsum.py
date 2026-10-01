"""A6 coverage: the project command exited 100 (test failures) and wrote no JSON, so there is no line/branch
total. Summarize the failure evidence (capped) into c-coverage.json; the raw stderr is deleted afterwards."""
import collections, json, re

R = '.andromeda/runs/2026-10-01T09-18-50-code-audit'
txt = open(f'{R}/cov-stderr.txt', encoding='utf-8', errors='replace').read()
summary = re.search(r'Summary \[[^\]]*\] (.*)', txt).group(1).strip()
fails = sorted(set(re.findall(r'^\s+(?:FAIL|TIMEOUT)(?: \+ LEAK)? \[[^\]]*\] \(\s*\d+/\d+\) (\S+) (\S+)', txt, re.M)))
by_binary = collections.Counter(b for b, _ in fails)
msgs = collections.Counter(m.strip() for m in re.findall(r"panicked at [^\n]*\n\s*([^\n]+)", txt))
out = {'line': None, 'branch': None, 'exit': 100, 'nextest_summary': summary,
       'failed_by_test_binary': dict(by_binary.most_common()),
       'panic_first_lines': dict(msgs.most_common(10)),
       'failed_tests_first_20': [f'{b} {t}' for b, t in fails[:20]],
       'repro_plain_build': {'command': "cargo nextest run --features fake-agent --profile ci -E 'test(=booted_wrapper_fixture_is_ready_and_receipting) | test(=run_refuses_a_live_name)'",
                             'result': '2/2 FAIL + LEAK, viola never exited (8.5 s) - also with every inherited CLAUDE*/VIOLA_*/ANTHROPIC* var unset',
                             'note': 'not coverage-specific and not the inherited session env; cause unknown, not investigated further (report-only skill)'}}
json.dump(out, open(f'{R}/c-coverage.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(summary); print(dict(by_binary.most_common())); print(len(fails))
