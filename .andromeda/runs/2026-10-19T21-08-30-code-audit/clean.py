#!/usr/bin/env python3
"""Size discipline: delete this run's own transients (raw tool output, report dirs, source extracts) — inside the run
dir ONLY (the sanctioned scripted-delete scope). usage: clean.py <run_dir>"""
import glob, os, shutil, sys

R = os.path.abspath(sys.argv[1])
assert R.endswith('/.andromeda/runs/2026-10-19T21-08-30-code-audit'), R
DIRS = ['x-src', 'x-base', 'rca', 'rca-base', 'rca-probe', 'jscpd', 'jscpd-base', '__pycache__'] + \
       [os.path.basename(p) for p in glob.glob(f'{R}/mutants-*') if os.path.isdir(p)]
FILES = ['cov-raw.json', 'cov-run.log', 'list-viola.json', 'verify-window-timing.log'] + \
        [os.path.basename(p) for p in glob.glob(f'{R}/mut-*.log') + glob.glob(f'{R}/list-*.err')]
for d in DIRS:
    p = os.path.join(R, d)
    assert os.path.realpath(p).startswith(R + '/'), p
    if os.path.isdir(p):
        shutil.rmtree(p)
        print('removed dir', d)
for f in FILES:
    p = os.path.join(R, f)
    assert os.path.realpath(p).startswith(R + '/'), p
    if os.path.isfile(p):
        os.remove(p)
        print('removed', f)
