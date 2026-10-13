"""viola-e2e from a HEAD extract (the operator's remedy for the --copy-target=false baseline failure).

Waits for the main driver's 'driver done' journal line, then: `git archive dffad3e | tar -x` into x-src/ (this
run dir), the recorded prebuild into x-src/target/mutants, and the recorded cargo-mutants form with
--copy-target=true (copying only the extract's target), jobs 2, from x-src. Output mutants-viola-e2e-x/.
Cap: the 1800 s floor (no recorded figure), re-sized once at the floor from its own rate.
"""
import json
import os
import signal
import subprocess
import sys
import time

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
SHA = 'dffad3e02d380362b8040effacc95781ceea8739'
X = os.path.join(RUN, 'x-src')
SCRATCH = os.path.expanduser('~/dev/projects/viola-mutants-scratch')
UNIT, JOBS, OUT = 'viola-e2e', 2, os.path.join(RUN, 'mutants-viola-e2e-x')


def now():
    return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


def journal(line):
    with open(os.path.join(RUN, 'journal.md'), 'a', encoding='utf-8', newline='\n') as f:
        f.write(f'- {now()} {line}\n')


def env():
    e = {k: v for k, v in os.environ.items() if not (k.startswith('VIOLA_') or k.startswith('CLAUDE'))}
    e.update(TMPDIR=SCRATCH, NEXTEST_PROFILE='mutants', AGENT_RUN_KEEP_HOMES='0', AGENT_RUN_KEEP_FAILED='0')
    return e


while 'viola (iterate): exited' not in open(os.path.join(RUN, 'journal.md'), encoding='utf-8').read():
    time.sleep(60)
if os.path.exists(os.path.join(RUN, f'state-{UNIT}-x.json')) or os.path.exists(OUT):
    journal('viola-e2e (extract): output or state already present; not re-invoked')
    sys.exit(3)
journal(f'viola-e2e (extract): extracting {SHA[:10]} into x-src/ and prebuilding viola (fake-agent) into x-src/target/mutants')
os.makedirs(X)
arc = subprocess.Popen(['git', 'archive', SHA], cwd=REPO, stdout=subprocess.PIPE)
subprocess.run(['tar', '-x', '-C', X], stdin=arc.stdout, check=True)
arc.wait()
pb = subprocess.run(['cargo', 'build', '--package', 'viola', '--features', 'fake-agent'], cwd=X,
                    env={**env(), 'CARGO_TARGET_DIR': os.path.join(X, 'target', 'mutants')},
                    capture_output=True, text=True)
journal(f'viola-e2e (extract): prebuild exit {pb.returncode}')
if pb.returncode != 0:
    open(os.path.join(RUN, 'prebuild-e2e-x.log'), 'w', encoding='utf-8').write(pb.stderr[-20000:])
    sys.exit(4)
planned = len(json.load(open(os.path.join(RUN, f'list-{UNIT}.json'), encoding='utf-8')))
cmd = ['cargo', 'mutants', '--package', UNIT, '--features', 'fake-agent', '--test-tool=nextest', '--copy-target=true',
       '--caught', '--unviable', '--build-timeout=400', '-j', str(JOBS), '--output', OUT]
cap = 1800
journal(f'viola-e2e (extract): start, planned {planned}, cap {cap} s (floor; re-sized once from its own rate), jobs {JOBS}')
log = open(os.path.join(RUN, f'run-{UNIT}-x.log'), 'w', encoding='utf-8')
t0 = time.time()
p = subprocess.Popen(cmd, cwd=X, env={**env(), 'CARGO_TARGET_DIR': 'target/mutants'}, stdout=log,
                     stderr=subprocess.STDOUT, start_new_session=True)
resized, how = False, None
while True:
    try:
        rc = p.wait(timeout=30)
        how = 'exited'
        break
    except subprocess.TimeoutExpired:
        pass
    el = time.time() - t0
    if not resized and el >= cap:
        try:
            o = json.load(open(os.path.join(OUT, 'mutants.out', 'outcomes.json'), encoding='utf-8'))
            tested = max(0, len(o.get('outcomes', [])) - 1)
        except (OSError, ValueError):
            tested = 0
        if tested:
            cap = max(cap, int(el / tested * planned * 1.5))
            resized = True
            journal(f'viola-e2e (extract): re-sized once at {int(el)} s from its own rate ({tested} tested): cap {cap} s')
            continue
        how = 'budget-exhausted (none tested at the floor)'
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
st = {'unit': UNIT, 'start': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(t0)),
      'stop': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(t1)), 'wall_s': int(t1 - t0), 'exit': rc, 'how': how,
      'cap_s': cap, 'resized': resized, 'jobs': JOBS, 'planned': planned, 'command': ' '.join(cmd),
      'cwd': 'x-src (git archive of HEAD)'}
with open(os.path.join(RUN, f'state-{UNIT}-x.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(st, f, indent=1)
journal(f'viola-e2e (extract): {how}, exit {rc}, wall {int(t1 - t0)} s')
