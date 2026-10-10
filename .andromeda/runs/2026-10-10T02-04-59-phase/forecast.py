"""Forecast, at HEAD, of the mutants the audit's pinned recipe (cover.py) reads as host-excluded on each host,
per workflow matrix item. Read-only: `cargo mutants --list --json` builds nothing and writes nothing."""
import collections, importlib.util, json, re, subprocess, sys

REPO = '.'  # run from the repository root
spec = importlib.util.spec_from_file_location('cover', REPO + '/.andromeda/runs/2026-10-08T10-08-51-code-audit/cover.py')
cover = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cover)

yml = open(REPO + '/.github/workflows/windows-mutants.yml', encoding='utf-8').read()
items = re.findall(r'- package: (\S+)\n\s+files: (.+)', yml)
cfgs = {'windows': cover.host_cfg(REPO, 'x86_64-pc-windows-msvc'), 'linux': cover.host_cfg(REPO)}
total = collections.Counter()
for package, files in items:
    cmd = ['cargo', 'mutants', '--list', '--json', '--package', package, '--features', 'fake-agent']
    for f in files.split():
        cmd += ['--file', f]
    out = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True)
    if out.returncode != 0:
        print(package, 'LIST FAILED', out.stderr[-300:])
        continue
    mutants = json.loads(out.stdout)
    per = collections.Counter()
    for host, cfg in cfgs.items():
        ex = [(m, cover.cover(REPO, m, cfg)) for m in mutants]
        ex = [(m, p) for m, p in ex if p]
        per[host] = len(ex)
        total[host] += len(ex)
        if host == 'windows':
            byfile = collections.Counter((m['file'], p) for m, p in ex)
            for (f, p), n in sorted(byfile.items()):
                print('   windows-excluded %2d  %s  under cfg(%s)' % (n, f, p))
    total['mutants'] += len(mutants)
    print('%-20s mutants %3d · excluded on windows %2d · on linux %2d' % (package, len(mutants), per['windows'], per['linux']))
print('total', dict(total))
