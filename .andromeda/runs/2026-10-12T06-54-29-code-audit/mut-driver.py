"""Mutation tier driver: one unit at a time, jobs 2, each under its wall-clock cap.

Writes per unit, all under this run dir: mutants-{unit}/ (the tool's output), run-{unit}.log (the
tool's console, deleted after summarizing), state-{unit}.json (start, stop, exit, how it ended).
A unit whose state file says it finished is skipped on a re-launch (resume by artifact presence).
Appends one line per event to journal.md.
"""
import json
import os
import signal
import subprocess
import sys
import time

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
SCRATCH = os.path.expanduser('~/dev/projects/viola-mutants-scratch')
JOBS = 2
# cap_s per unit: planned x (baseline wall_s / tested at jobs 1, never credited for jobs 2) x 1.5,
# + 800 s for the two cold builds --copy-target=false brings (the baseline build and job 2's first
# build, each bounded by --build-timeout=400), floor 1800. viola-e2e: no figure -> floor, re-sized once.
UNITS = [
    ('viola-core', 1800, False),
    ('viola-e2e', 1800, True),
    ('viola-pty', 1800, False),
    ('viola-agent-claude', 1896, False),
    ('viola-state', 3074, False),
    ('viola', 31737, False),
]


def now():
    return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


def journal(line):
    with open(os.path.join(RUN, 'journal.md'), 'a', encoding='utf-8', newline='\n') as f:
        f.write(f'- {now()} {line}\n')


def outcomes(unit):
    p = os.path.join(RUN, f'mutants-{unit}', 'mutants.out', 'outcomes.json')
    try:
        with open(p, encoding='utf-8') as f:
            return json.load(f)
    except (OSError, ValueError):
        return None


def command(unit):
    feats = [] if unit == 'viola-core' else ['--features', 'fake-agent']
    return ['cargo', 'mutants', '--package', unit, *feats, '--test-tool=nextest', '--copy-target=false',
            '--caught', '--unviable', '--build-timeout=400', '-j', str(JOBS),
            '--output', os.path.join(RUN, f'mutants-{unit}')]


def env():
    e = {k: v for k, v in os.environ.items() if not (k.startswith('VIOLA_') or k.startswith('CLAUDE'))}
    e.update(TMPDIR=SCRATCH, NEXTEST_PROFILE='mutants', AGENT_RUN_KEEP_HOMES='0', AGENT_RUN_KEEP_FAILED='0',
             CARGO_TARGET_DIR='target/mutants')
    return e


def run_unit(unit, cap, resizable):
    state_p = os.path.join(RUN, f'state-{unit}.json')
    if os.path.exists(state_p):
        journal(f'{unit}: state file present, skipped (resume by artifact presence)')
        return
    out = os.path.join(RUN, f'mutants-{unit}')
    if os.path.exists(out):
        journal(f'{unit}: STOP - output dir already exists without a state file; not re-invoked (a re-invocation rotates it)')
        sys.exit(3)
    planned = len(json.load(open(os.path.join(RUN, f'list-{unit}.json'), encoding='utf-8')))
    journal(f'{unit}: start, planned {planned}, cap {cap} s, jobs {JOBS}' + (' (no recorded rate: re-sized once at the floor)' if resizable else ''))
    log = open(os.path.join(RUN, f'run-{unit}.log'), 'w', encoding='utf-8')
    t0 = time.time()
    p = subprocess.Popen(command(unit), cwd=REPO, env=env(), stdout=log, stderr=subprocess.STDOUT,
                         start_new_session=True)
    resized = False
    how = None
    while True:
        try:
            rc = p.wait(timeout=30)
            how = 'exited'
            break
        except subprocess.TimeoutExpired:
            pass
        el = time.time() - t0
        if resizable and not resized and el >= cap:
            o = outcomes(unit)
            tested = max(0, len((o or {}).get('outcomes', [])) - 1)
            if tested == 0:
                how = 'budget-exhausted (none tested at the floor)'
            else:
                cap = max(cap, int(el / tested * planned * 1.5))
                resized = True
                journal(f'{unit}: re-sized once at {int(el)} s from its own rate ({tested} tested): cap {cap} s')
                continue
        if how or el >= cap:
            how = how or 'budget-exhausted (cap spent)'
            os.killpg(p.pid, signal.SIGINT)
            try:
                rc = p.wait(timeout=120)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                rc = p.wait()
            break
    t1 = time.time()
    log.close()
    st = {'unit': unit, 'start': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(t0)),
          'stop': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(t1)), 'wall_s': int(t1 - t0),
          'exit': rc, 'how': how, 'cap_s': cap, 'resized': resized, 'jobs': JOBS, 'planned': planned,
          'command': ' '.join(command(unit))}
    with open(state_p, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(st, f, indent=1)
    journal(f'{unit}: {how}, exit {rc}, wall {int(t1 - t0)} s')


journal(f'driver start: units {[u for u, _, _ in UNITS]}, jobs {JOBS}, TMPDIR ~/dev/projects/viola-mutants-scratch')
for u, c, r in UNITS:
    run_unit(u, c, r)
journal('driver done')
