"""Epoch 2 cleanup measurements over the code-audit population: size, clones.

Each subcommand prints its detail lines, then its verdict count as the last line.
Tools (host, as the Epoch 2 code audit ran them): tokei, jscpd.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
EXCLUDED = ('scripts/', 'docs/', 'refs/', 'fuzz/', '.andromeda/', '.claude/')
SIZE_CEILING = 800
WATCHED = (
    'crates/viola-e2e/src/harness/pre_push.rs',
    'crates/viola-e2e/src/harness/pre_push/linux.rs',
    'crates/viola-pty/src/lib.rs',
    'crates/viola-pty/src/pump.rs',
    'crates/viola-e2e/src/harness/run/mutants.rs',
    'crates/viola-e2e/src/harness/run/mutants/base.rs',
    'crates/viola-e2e/src/harness/run/mutants/leg.rs',
    'crates/viola-e2e/src/harness/run/mutants/scratch.rs',
    'crates/viola-channel/src/server.rs',
    'crates/viola-channel/src/server/win.rs',
    'crates/viola-channel/src/test_support.rs',
)
ALLOWED_PAIRS = {
    frozenset(p)
    for p in [
        ('crates/viola-channel/src/lib.rs', 'crates/viola-e2e/src/harness/cleanup.rs'),
        ('crates/viola-channel/tests/channel_frames.rs', 'tests/channel_endpoint.rs'),
        ('crates/viola-core/src/obs.rs', 'tests/contract_diag_schema.rs'),
        ('crates/viola-e2e/src/harness/boot.rs', 'tests/support/home.rs'),
        ('crates/viola-e2e/src/harness/boot.rs', 'crates/viola-e2e/src/harness/boot.rs'),
        ('crates/viola-e2e/src/harness/gate.rs', 'crates/viola-e2e/src/harness/gate.rs'),
        ('crates/viola-e2e/src/harness/mod.rs', 'crates/viola-state/src/liveness.rs'),
        ('crates/viola-e2e/src/harness/pre_push.rs', 'crates/viola-e2e/src/harness/pre_push.rs'),
        ('crates/viola-e2e/src/harness/schema_check.rs', 'crates/viola-e2e/src/harness/secret_scan.rs'),
        ('crates/viola-e2e/src/harness/secret_scan.rs', 'crates/viola-e2e/tests/scan_patterns.rs'),
        ('crates/viola-e2e/tests/harness_lifecycle.rs', 'crates/viola-e2e/tests/harness_lifecycle.rs'),
        ('crates/viola-pty/src/lib.rs', 'crates/viola-pty/src/lib.rs'),
        ('crates/viola-state/src/fs.rs', 'crates/viola-state/src/fs.rs'),
        ('crates/viola-state/src/liveness.rs', 'tests/support/home.rs'),
        ('crates/viola-state/src/pin.rs', 'crates/viola-state/src/pin.rs'),
        ('src/run/env.rs', 'src/run/env.rs'),
        ('tests/channel_sqos_open.rs', 'tests/security_negatives_channel.rs'),
        ('tests/cli_fake_agent.rs', 'tests/cli_program_resolution.rs'),
        ('tests/cli_instance_state.rs', 'tests/cli_instance_state.rs'),
        ('tests/run_cli.rs', 'tests/tui_env_strip.rs'),
    ]
}
# A split moves a concern with its tests, so a clone the control run saw inside a split file is the same
# clone when it now sits in the file's submodule: the allowed-list check reads each end at its parent file.
SPLIT_PARENTS = {
    'crates/viola-e2e/src/harness/pre_push/linux.rs': 'crates/viola-e2e/src/harness/pre_push.rs',
    'crates/viola-pty/src/pump.rs': 'crates/viola-pty/src/lib.rs',
    'crates/viola-e2e/src/harness/run/mutants/base.rs': 'crates/viola-e2e/src/harness/run/mutants.rs',
    'crates/viola-e2e/src/harness/run/mutants/leg.rs': 'crates/viola-e2e/src/harness/run/mutants.rs',
    'crates/viola-e2e/src/harness/run/mutants/scratch.rs': 'crates/viola-e2e/src/harness/run/mutants.rs',
    'crates/viola-channel/src/server/win.rs': 'crates/viola-channel/src/server.rs',
}


def norm(path):
    rel = os.path.relpath(path, ROOT) if os.path.isabs(path) else path
    return rel.replace('\\', '/')


def tool(name):
    found = shutil.which(name)
    if not found:
        print(f'tool-missing: {name}')
        sys.exit(2)
    return found


def population():
    out = subprocess.run(
        ['git', 'ls-files', '-co', '--exclude-standard', '*.rs'],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout
    files = [f for f in out.splitlines() if f and not f.startswith(EXCLUDED)]
    return sorted(set(files))


def size():
    files = population()
    out = subprocess.run(
        [tool('tokei'), '--files', '--output', 'json', *files],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout
    reports = json.loads(out)['Rust']['reports']
    over = 0
    for r in sorted(reports, key=lambda r: norm(r['name'])):
        name, code = norm(r['name']), r['stats']['code']
        if name in WATCHED:
            print(f'watched: {name} {code}')
        if code > SIZE_CEILING:
            over += 1
            print(f'over {SIZE_CEILING}: {name} {code}')
    print(f'files {len(reports)}')
    print(over)


def in_scope(a, b):
    def channel_server(p):
        return p == 'crates/viola-channel/src/server.rs' or p.startswith('crates/viola-channel/src/server/')

    def channel_capture(p):
        return p in ('crates/viola-channel/src/lib.rs', 'crates/viola-channel/src/test_support.rs')

    for x, y in ((a, b), (b, a)):
        if x == 'tests/channel_endpoint.rs' and channel_server(y):
            return True
        if channel_capture(x) and y == 'src/cmd/run.rs':
            return True
    return False


def clones():
    out_dir = tempfile.mkdtemp(prefix='metrics-jscpd-')
    ignore = 'fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**'
    subprocess.run(
        [tool('jscpd'), '--reporters', 'json', '--output', out_dir, '--format', 'rust', '--silent',
         '--ignore', ignore, '.'],
        cwd=ROOT, capture_output=True, check=True,
    )
    with open(os.path.join(out_dir, 'jscpd-report.json'), encoding='utf-8') as fh:
        report = json.load(fh)
    shutil.rmtree(out_dir, ignore_errors=True)
    flagged = 0
    for d in report['duplicates']:
        a, b = norm(d['firstFile']['name']), norm(d['secondFile']['name'])
        scoped = in_scope(a, b)
        new = frozenset([SPLIT_PARENTS.get(a, a), SPLIT_PARENTS.get(b, b)]) not in ALLOWED_PAIRS
        if scoped or new:
            flagged += 1
            kind = 'in-scope' if scoped else 'new'
            print(f'{kind}: {a}:{d["firstFile"]["start"]} <-> {b}:{d["secondFile"]["start"]} {d["lines"]} L')
    print(f'pairs {len(report["duplicates"])}')
    print(flagged)


if __name__ == '__main__':
    commands = {'size': size, 'clones': clones}
    if len(sys.argv) != 2 or sys.argv[1] not in commands:
        print('usage: metrics.py size|clones')
        sys.exit(2)
    commands[sys.argv[1]]()
