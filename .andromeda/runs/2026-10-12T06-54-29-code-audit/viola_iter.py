"""Resume of the viola unit: the recorded form plus --iterate, against a further copy of mutants-viola-partial.

mutants-viola-iter/mutants.out is a byte copy of mutants-viola-partial/mutants.out (diff -rq clean, outcomes.json
sha256 7439ee70...). The listing read back before this run: --iterate tests 636 (621 untested + 15 missed), skips
the 663 caught or unviable. The tool rotates the copy to mutants.out.old; mutants-viola-partial/ is never touched.
Cap: the unit's recorded cap (31737 s) less the 6596 s the first invocation spent (07:17:31Z to 09:07:27Z).
Writes state-viola-iter.json and journal lines; the merge of the two invocations is a later step.
"""
import hashlib
import os
import signal
import subprocess
import sys
import time

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
SCRATCH = os.path.expanduser('~/dev/projects/viola-mutants-scratch')
UNIT, JOBS, CAP = 'viola', 2, 31737 - 6596
OUT = os.path.join(RUN, 'mutants-viola-iter')
SHA = '7439ee703ae08ec18b0e658728a8e3a4d7432bec9e9cb6d3fe55c575fa411c07'
STATE = os.path.join(RUN, 'state-viola-iter.json')


def now():
    return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


def journal(line):
    with open(os.path.join(RUN, 'journal.md'), 'a', encoding='utf-8', newline='\n') as f:
        f.write(f'- {now()} {line}\n')


def env():
    e = {k: v for k, v in os.environ.items() if not (k.startswith('VIOLA_') or k.startswith('CLAUDE'))}
    e.update(TMPDIR=SCRATCH, NEXTEST_PROFILE='mutants', AGENT_RUN_KEEP_HOMES='0', AGENT_RUN_KEEP_FAILED='0',
             CARGO_TARGET_DIR='target/mutants')
    return e


oc = os.path.join(OUT, 'mutants.out', 'outcomes.json')
if os.path.exists(STATE) or os.path.exists(os.path.join(OUT, 'mutants.out.old')) or not os.path.isfile(oc) \
        or hashlib.sha256(open(oc, 'rb').read()).hexdigest() != SHA:
    journal('viola (iterate): STOP - state present, copy already rotated, or the copy differs from the partial; not invoked')
    sys.exit(3)
cmd = ['cargo', 'mutants', '--package', UNIT, '--features', 'fake-agent', '--test-tool=nextest', '--copy-target=false',
       '--caught', '--unviable', '--build-timeout=400', '-j', str(JOBS), '--iterate', '--output', OUT]
journal(f'viola (iterate): start, expected 636 (621 untested + 15 missed), cap {CAP} s, jobs {JOBS}, output mutants-viola-iter/')
log = open(os.path.join(RUN, 'run-viola-iter.log'), 'w', encoding='utf-8')
t0 = time.time()
p = subprocess.Popen(cmd, cwd=REPO, env=env(), stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
how = None
while True:
    try:
        rc = p.wait(timeout=30)
        how = 'exited'
        break
    except subprocess.TimeoutExpired:
        pass
    if time.time() - t0 >= CAP:
        how = 'budget-exhausted (cap spent)'
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
      'cap_s': CAP, 'resized': False, 'jobs': JOBS, 'planned': 636, 'command': ' '.join(cmd),
      'resumes': 'mutants-viola-partial (678/1299, stopped 2026-10-12T09:07:27Z)'}
with open(STATE, 'w', encoding='utf-8', newline='\n') as f:
    import json
    json.dump(st, f, indent=1)
journal(f'viola (iterate): {how}, exit {rc}, wall {int(t1 - t0)} s')
