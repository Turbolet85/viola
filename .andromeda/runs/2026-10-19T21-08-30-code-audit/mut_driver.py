#!/usr/bin/env python3
"""C1 driver: one unit at a time, the recorded form, per-unit wall-clock cap, resumable by artifact presence.

usage: mut_driver.py   (repo root as cwd). Progress lines -> {R}/mut-driver.log.
A unit whose c-mutation-{unit}.json exists is skipped. A unit still incomplete at its cap -> SIGINT to its
process group, c-mutation-{unit}.json NOT written, a budget-exhausted line in mut-states.json.
"""
import json, os, signal, subprocess, sys, time

R = '.andromeda/runs/2026-10-19T21-08-30-code-audit'
HOME = os.path.expanduser('~')
SCRATCH = f'{HOME}/dev/projects/viola-mutants-scratch/e5'
sys.path.insert(0, R)
import mutsum  # noqa: E402

ENV = {'HOME': HOME, 'PATH': f'{HOME}/.cargo/bin:/usr/local/bin:/usr/bin:/bin', 'TMPDIR': SCRATCH,
       'NEXTEST_PROFILE': 'mutants', 'AGENT_RUN_KEEP_HOMES': '0', 'AGENT_RUN_KEEP_FAILED': '0',
       'CARGO_TARGET_DIR': 'target/mutants'}
FORM = ['cargo', 'mutants', '--test-tool=nextest', '--caught', '--unviable', '--build-timeout=400', '-j', '2']
ROOT_FILES = ['src/bin/viola-fake-agent.rs', 'src/cmd/verify.rs']

# unit, cap_s, has_figure, scope, extra args
UNITS = [
    ('viola-pty', 1800, True, 'unit', []),
    ('viola-mcp', 1800, False, 'unit', []),
    ('viola-channel', 1800, False, 'unit', []),
    ('viola-agent-claude', 1800, True, 'unit', []),
    ('viola-state', 1800, True, 'unit', []),
    ('viola', 4465, True, 'files', [a for f in ROOT_FILES for a in ('--file', f)]),
    ('viola-e2e', 4323, True, 'unit', []),
]
STATES = f'{R}/mut-states.json'


def log(msg):
    with open(f'{R}/mut-driver.log', 'a') as f:
        f.write(time.strftime('%Y-%m-%dT%H:%M:%SZ ', time.gmtime()) + msg + '\n')


def state(unit, s):
    d = json.load(open(STATES)) if os.path.exists(STATES) else {}
    d[unit] = s
    json.dump(d, open(STATES, 'w'), indent=1)


def run_unit(unit, cap, has_fig, scope, extra):
    out = f'{R}/mutants-{unit}'
    cwd = '.'
    feats = ['--features', 'fake-agent']
    copy = '--copy-target=false'
    if unit == 'viola-e2e':
        x = f'{R}/x-src'
        os.makedirs(x, exist_ok=True)
        if not os.path.exists(f'{x}/Cargo.toml'):
            subprocess.run(f'git archive HEAD | tar -x -C {x}', shell=True, check=True)
        env = dict(ENV, CARGO_TARGET_DIR=os.path.abspath(f'{x}/target/mutants'))
        log('e2e: prebuild viola --features fake-agent in the extract')
        subprocess.run(['cargo', 'build', '--package', 'viola', '--features', 'fake-agent'], cwd=x, env=env, check=True,
                       stdout=open(f'{R}/mut-viola-e2e-prebuild.log', 'w'), stderr=subprocess.STDOUT)
        cwd, copy, out = x, '--copy-target=true', os.path.abspath(f'{R}/mutants-viola-e2e-x')
    argv = FORM + ['--package', unit] + feats + [copy] + extra + ['--output', out if unit == 'viola-e2e' else out]
    log(f'{unit}: start cap {cap}s argv {" ".join(argv)}')
    t0 = time.time()
    p = subprocess.Popen(argv, cwd=cwd, env=ENV, start_new_session=True,
                         stdout=open(f'{R}/mut-{unit}.log', 'w'), stderr=subprocess.STDOUT)
    resized = has_fig
    while p.poll() is None:
        time.sleep(20)
        el = time.time() - t0
        if el > cap:
            oc = f'{out}/mutants.out/outcomes.json'
            n = len(json.load(open(oc))['outcomes']) - 1 if os.path.exists(oc) else 0
            tot = len(json.load(open(f'{out}/mutants.out/mutants.json'))) if os.path.exists(f'{out}/mutants.out/mutants.json') else 0
            if not resized and n > 0 and tot:
                cap = max(cap, int(el / n * tot * 1.5))
                resized = True
                log(f'{unit}: re-sized once at the floor from its own rate ({n}/{tot} in {int(el)}s) -> cap {cap}s')
                continue
            log(f'{unit}: cap {cap}s spent at {n}/{tot}; SIGINT to the process group')
            os.killpg(p.pid, signal.SIGINT)
            p.wait()
            state(unit, {'state': 'budget-exhausted', 'tested': n, 'planned': tot, 'wall_s': int(el), 'cap_s': cap,
                         'scope': scope, 'jobs': 2})
            return
    el = int(time.time() - t0)
    mo = f'{out}/mutants.out'
    d = mutsum.summarize(R, unit, mo, scope, 2, cap, ROOT_FILES if scope == 'files' else None,
                         src_repo=(f'{R}/x-src' if unit == 'viola-e2e' else '.'))
    if d is None:
        log(f'{unit}: exited {p.returncode} after {el}s INCOMPLETE by the tool markers')
        state(unit, {'state': 'exited-incomplete', 'exit': p.returncode, 'wall_s': el})
        return
    log(f'{unit}: exited {p.returncode} after {el}s -> {d["unit_state"]} score {d["score"]} {d["counts"]}')
    state(unit, {'state': d['unit_state'], 'exit': p.returncode})


def main():
    os.makedirs(SCRATCH, exist_ok=True)
    for unit, cap, fig, scope, extra in UNITS:
        if os.path.exists(f'{R}/c-mutation-{unit}.json'):
            log(f'{unit}: c-mutation present, skipped')
            continue
        run_unit(unit, cap, fig, scope, extra)
    log('DRIVER-END')


main()
