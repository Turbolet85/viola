"""The operator's answer (2026-10-08): stop the report-only viola-e2e run once harness/mod.rs (the keeper) and
harness/cleanup.rs are graded. Waits for that, then sends SIGINT to the invocation's own process group (the
tool's interrupt path removes its copied tree), selected by its --output argument, never by name alone."""
import datetime, json, os, signal, time

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
OUT = f'{R}/mutants-viola-e2e-deselected'
NEED = ('crates/viola-e2e/src/harness/mod.rs', 'crates/viola-e2e/src/harness/cleanup.rs')
now = lambda: datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')

def log(msg):
    with open(f'{R}/mut-driver.log', 'a', encoding='utf-8') as f:
        f.write(f'{now()} stop-e2e: {msg}\n')

listed = json.load(open(f'{OUT}/mutants.out/mutants.json', encoding='utf-8'))
need = {m['name'] for m in listed if m['file'] in NEED}
while True:
    if os.path.exists(f'{R}/mut-done-viola-e2e-deselected.json'):
        log('the run ended by itself; nothing to stop')
        raise SystemExit(0)
    try:
        oc = json.load(open(f'{OUT}/mutants.out/outcomes.json', encoding='utf-8'))
        graded = {o['scenario']['Mutant']['name'] for o in oc['outcomes'][1:]}
    except (OSError, ValueError):
        graded = set()
    if need <= graded:
        break
    time.sleep(10)
log(f'{len(need)} mutants of mod.rs and cleanup.rs graded ({len(graded)} tested in all) - interrupting')
leaders = []
for pid in filter(str.isdigit, os.listdir('/proc')):
    try:
        args = open(f'/proc/{pid}/cmdline', 'rb').read().split(b'\0')
    except OSError:
        continue
    if OUT.encode() in args and b'mutants' in args and os.getsid(int(pid)) == int(pid):
        leaders.append(int(pid))
log(f'session leaders holding --output {OUT}: {leaders}')
assert len(leaders) == 1, leaders
os.killpg(leaders[0], signal.SIGINT)
for _ in range(60):
    if os.path.exists(f'{R}/mut-done-viola-e2e-deselected.json'):
        break
    time.sleep(2)
log('done file present' if os.path.exists(f'{R}/mut-done-viola-e2e-deselected.json') else 'no done file 120 s after the interrupt')
