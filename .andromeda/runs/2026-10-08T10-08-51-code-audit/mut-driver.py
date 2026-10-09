"""C1 mutation tier driver: one unit at a time, the harness's boundary form fired directly so the output lands in
the run dir. Run from the repository root: python3 <this> <unit> [<unit> ...]

Per unit: `cargo mutants --package {unit} [--features fake-agent] --test-tool=nextest --copy-target=true --caught
--unviable --build-timeout=400 --output {R}/mutants-{unit}` under TMPDIR={SCRATCH} NEXTEST_PROFILE=mutants
AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants, jobs 1 (no -j).
Cap: the 30-minute floor, re-sized ONCE at the floor from the unit's own rate (elapsed / live total_mutants,
x planned x 1.5); the operator's answer runs a unit whole under that cap even past 2 h. The stop is this poll's.
A finished unit leaves mut-done-{unit}.json (resume by its presence, or by c-mutation-{unit}.json).
Each poll also samples the copied tree for the route CARRY: what target/e2e-home is there, and any `backing` dir."""
import datetime, glob, json, os, shutil, signal, subprocess, sys, time

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
SCRATCH = '/home/turbolet/dev/projects/viola-mutants-scratch/e3'
HOME_BACKING = '/tmp/viola-e2e-home-1000'
PLANNED = {'viola-core': 46, 'viola-pty': 115, 'viola-channel': 176, 'viola-state': 269,
           'viola-agent-claude': 462, 'viola-e2e': 718, 'viola': 1047}
NO_FEATURE = {'viola-core'}  # its manifest declares no fake-agent feature
FLOOR, POLL = 1800, 15

now = lambda: datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')

def clean_env():
    """The session's own bridge and CLI variables never reach a mutated binary's tests (CI has none)."""
    return {k: v for k, v in os.environ.items() if not k.startswith(('VIOLA_', 'CLAUDE'))}

def log(msg):
    with open(f'{R}/mut-driver.log', 'a', encoding='utf-8') as f:
        f.write(f'{now()} {msg}\n')

def read_json(path):
    try:
        return json.load(open(path, encoding='utf-8'))
    except (OSError, ValueError):
        return None

def last_line(path):
    try:
        with open(path, 'rb') as f:
            f.seek(0, 2)
            f.seek(max(0, f.tell() - 600))
            lines = [l for l in f.read().decode('utf-8', 'replace').splitlines() if l.strip()]
            return lines[-1][-300:] if lines else ''
    except OSError:
        return ''

def sample_copy(state, unit_log):
    """The CARRY reading: per copied tree, what target/e2e-home is; any `backing` dir, first seen."""
    for copy in sorted(glob.glob(f'{SCRATCH}/cargo-mutants-*')):
        link = os.path.join(copy, 'target', 'e2e-home')
        if copy not in state['copies'] and os.path.lexists(link):
            st = os.lstat(link)
            state['copies'][copy] = {
                'seen': now(), 'islink': os.path.islink(link),
                'readlink': os.readlink(link) if os.path.islink(link) else None,
                'isdir_nofollow': os.path.isdir(link) and not os.path.islink(link),
                'mode': oct(st.st_mode)}
            log(f'copy {os.path.basename(copy)} target/e2e-home: {state["copies"][copy]}')
        for rel in ('backing', 'crates/viola-e2e/backing', 'target/backing', 'src/backing', 'tests/backing'):
            p = os.path.join(copy, rel)
            key = f'{os.path.basename(copy)}/{rel}'
            if key not in state['backing'] and os.path.lexists(p):
                try:
                    entries = len(os.listdir(p))
                except OSError:
                    entries = None
                state['backing'][key] = {'seen': now(), 'mode': oct(os.lstat(p).st_mode), 'entries': entries,
                                         'log_line': last_line(unit_log)}
                log(f'backing dir seen: {key} {state["backing"][key]}')
    try:
        n = len(os.listdir(HOME_BACKING))
        used = shutil.disk_usage('/tmp').used
        state['homes_max'] = max(state.get('homes_max', 0), n)
        state['homes_min'] = min(state.get('homes_min', n), n)
        state['tmp_used_max'] = max(state.get('tmp_used_max', 0), used)
        state['tmp_used_min'] = min(state.get('tmp_used_min', used), used)
    except OSError:
        pass

def stop(proc):
    for sig, wait in ((signal.SIGTERM, 30), (signal.SIGKILL, 10)):
        try:
            os.killpg(proc.pid, sig)
        except ProcessLookupError:
            return
        try:
            proc.wait(wait)
            return
        except subprocess.TimeoutExpired:
            continue

def run_unit(unit, label=None, extra=()):
    package, unit = unit, label or unit  # a labelled run is report-only: its own output, log and done file
    out = f'{R}/mutants-{unit}'
    done = f'{R}/mut-done-{unit}.json'
    if os.path.exists(done) or os.path.exists(f'{R}/c-mutation-{unit}.json'):
        log(f'{unit}: artifact present, skipped')
        return True
    if os.path.exists(out):
        log(f'{unit}: output dir {out} already exists (stale by construction) - not run')
        return False
    cmd = ['cargo', 'mutants', '--package', package]
    if package not in NO_FEATURE:
        cmd += ['--features', 'fake-agent']
    cmd += ['--test-tool=nextest', '--copy-target=true', '--caught', '--unviable', '--build-timeout=400',
            '--output', out] + list(extra)
    env = dict(clean_env(), TMPDIR=SCRATCH, NEXTEST_PROFILE='mutants', AGENT_RUN_KEEP_HOMES='0',
               AGENT_RUN_KEEP_FAILED='0', CARGO_TARGET_DIR='target/mutants')
    unit_log = f'{R}/mut-{unit}.log'
    state = {'copies': {}, 'backing': {}}
    scratch_before = sorted(os.listdir(SCRATCH))
    started, t0 = now(), time.monotonic()
    log(f'{unit}: start planned={PLANNED[package]} cmd={" ".join(cmd)}')
    with open(unit_log, 'wb') as lf:
        proc = subprocess.Popen(cmd, stdout=lf, stderr=subprocess.STDOUT, env=env, start_new_session=True)
    cap, resized, stopped, live_at_floor = FLOOR, False, False, None
    while True:
        try:
            proc.wait(POLL)
            break
        except subprocess.TimeoutExpired:
            pass
        sample_copy(state, unit_log)
        elapsed = time.monotonic() - t0
        if not resized and elapsed >= FLOOR:
            resized = True
            oc = read_json(f'{out}/mutants.out/outcomes.json') or {}
            live_at_floor = oc.get('total_mutants') or 0
            if live_at_floor == 0:
                log(f'{unit}: none tested at the floor - stopping (budget-exhausted)')
                stop(proc); stopped = True
                break
            rate = elapsed / live_at_floor
            cap = max(FLOOR, int(PLANNED[package] * rate * 1.5))
            log(f'{unit}: cap re-sized once at the floor: {live_at_floor} tested in {int(elapsed)} s -> '
                f'{rate:.2f} s/mutant -> cap {cap} s' + (' (over 2 h: whole, the operator\'s answer)' if cap > 7200 else ''))
        if resized and elapsed >= cap:
            log(f'{unit}: cap {cap} s spent - stopping (budget-exhausted)')
            stop(proc); stopped = True
            break
    elapsed = int(time.monotonic() - t0)
    oc = read_json(f'{out}/mutants.out/outcomes.json') or {}
    listed = read_json(f'{out}/mutants.out/mutants.json')
    n_listed = len(listed) if isinstance(listed, list) else None
    tested = oc.get('total_mutants')
    complete = bool(oc.get('end_time')) and n_listed is not None and tested == n_listed
    first = (oc.get('outcomes') or [{}])[0]
    if complete:
        st = 'complete'
    elif stopped:
        st = 'budget-exhausted'
    elif first.get('summary') not in (None, 'Success') and not tested:
        st = 'baseline-test-failure'
    else:
        st = 'exited-incomplete'
    wall = None
    if oc.get('start_time') and oc.get('end_time'):
        p = lambda s: datetime.datetime.fromisoformat(s.replace('Z', '+00:00'))
        wall = int((p(oc['end_time']) - p(oc['start_time'])).total_seconds())
    residue = sorted(set(os.listdir(SCRATCH)) - set(scratch_before))
    doc = {'unit': unit, 'package': package, 'state': st, 'exit_code': proc.returncode, 'planned': PLANNED[package], 'listed': n_listed,
           'tested': tested, 'wall_s': wall if wall is not None else elapsed, 'driver_elapsed_s': elapsed,
           'cap_s': cap, 'resized': resized, 'live_at_floor': live_at_floor, 'jobs': 1, 'scope': 'unit',
           'started': started, 'ended': now(), 'command': ' '.join(cmd),
           'env': 'every VIOLA_* and CLAUDE* name unset; TMPDIR=' + SCRATCH + ' NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants',
           'counts_tool': {k: oc.get(k) for k in ('caught', 'missed', 'timeout', 'unviable', 'success')},
           'baseline_outcome': {'summary': first.get('summary'),
                                'phases': [[ph.get('phase'), ph.get('duration')] for ph in first.get('phase_results', [])]},
           'carry': {'copies': state['copies'], 'backing': state['backing'],
                     'homes_min': state.get('homes_min'), 'homes_max': state.get('homes_max'),
                     'tmp_used_min': state.get('tmp_used_min'), 'tmp_used_max': state.get('tmp_used_max'),
                     'scratch_residue_after_exit': residue}}
    json.dump(doc, open(done, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
    log(f'{unit}: {st} exit={proc.returncode} tested={tested}/{n_listed} wall={doc["wall_s"]} s counts={doc["counts_tool"]} residue={residue}')
    return st == 'complete'

if __name__ == '__main__':
    units = sys.argv[1:]
    if units and units[0] == '--e2e-deselected':
        # the operator's answer (2026-10-08): after viola ends, viola-e2e once more with the one test that outlives
        # the mutants profile's kill deselected; report-only, never a ledger score
        while not os.path.exists(f'{R}/mut-done-viola.json'):
            time.sleep(30)
        run_unit('viola-e2e', label='viola-e2e-deselected',
                 extra=['--', '-E', 'not test(=boot_with_an_unknown_cli_version_is_verify_failed)'])
        log('driver: done viola-e2e-deselected')
        sys.exit(0)
    if units and units[0] == '--prebuild':
        units = units[1:]
        log('prebuild: cargo build --package viola --features fake-agent (CARGO_TARGET_DIR=<repo>/target/mutants)')
        with open(f'{R}/mut-prebuild.log', 'wb') as lf:
            rc = subprocess.run(['cargo', 'build', '--package', 'viola', '--features', 'fake-agent'],
                                stdout=lf, stderr=subprocess.STDOUT,
                                env=dict(clean_env(), CARGO_TARGET_DIR=os.path.join(os.getcwd(), 'target', 'mutants'))).returncode
        log(f'prebuild: exit {rc}')
        if rc != 0:
            sys.exit(f'prebuild failed ({rc})')
    for u in units:
        if u not in PLANNED:
            sys.exit(f'unknown unit {u}')
    for u in units:
        run_unit(u)
    log('driver: done ' + ' '.join(units))
