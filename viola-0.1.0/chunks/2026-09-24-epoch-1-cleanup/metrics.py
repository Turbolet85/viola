"""Epoch 1 cleanup measurements over the code-audit population: size, cognitive, clones.

Each subcommand prints its detail lines, then its verdict count as the last line.
Tools (host, as the Epoch 1 code audit ran them): tokei, rust-code-analysis-cli, jscpd.
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
COGNITIVE_CEILING = 15
IN_SCOPE_FILES = ('tests/cli_fake_agent.rs', 'src/main.rs', 'src/obs.rs')
ALLOWED_PAIRS = {
    frozenset(p) if p[0] != p[1] else frozenset([p[0]])
    for p in [
        ('crates/viola-core/src/obs.rs', 'tests/contract_diag_schema.rs'),
        ('crates/viola-e2e/src/harness/boot.rs', 'crates/viola-e2e/src/harness/boot.rs'),
        ('crates/viola-e2e/src/harness/gate.rs', 'crates/viola-e2e/src/harness/gate.rs'),
        ('crates/viola-e2e/src/harness/schema_check.rs', 'crates/viola-e2e/src/harness/secret_scan.rs'),
        ('crates/viola-e2e/src/harness/secret_scan.rs', 'crates/viola-e2e/tests/scan_patterns.rs'),
        ('crates/viola-e2e/tests/harness_lifecycle.rs', 'crates/viola-e2e/tests/harness_lifecycle.rs'),
        ('tests/contract_diag_schema.rs', 'tests/run_cli.rs'),
    ]
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
        if name == 'crates/viola-e2e/src/harness/run.rs':
            print(f'run.rs code {code}')
        if code > SIZE_CEILING:
            over += 1
            print(f'over {SIZE_CEILING}: {name} {code}')
    print(f'files {len(reports)}')
    print(over)


def functions(node, path, rows):
    if node.get('kind') == 'function':
        rows.append((node['metrics']['cognitive']['sum'], path, node.get('start_line'), node.get('name')))
    for child in node.get('spaces', []):
        functions(child, path, rows)


def cognitive():
    rca = tool('rust-code-analysis-cli')
    out_dir = tempfile.mkdtemp(prefix='metrics-rca-')
    args = [rca, '-m', '-O', 'json', '-o', out_dir]
    for f in population():
        args += ['-p', f]
    subprocess.run(args, cwd=ROOT, capture_output=True, check=True)
    rows = []
    for base, _, names in os.walk(out_dir):
        for n in names:
            if n.endswith('.json'):
                with open(os.path.join(base, n), encoding='utf-8') as fh:
                    doc = json.load(fh)
                functions(doc, norm(doc['name']), rows)
    shutil.rmtree(out_dir, ignore_errors=True)
    over = sorted((r for r in rows if r[0] > COGNITIVE_CEILING), reverse=True)
    for cog, path, line, name in over:
        print(f'{cog:.0f} {path}:{line} {name}')
    print(f'functions {len(rows)}')
    print(len(over))


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
        pair = frozenset([a, b])
        in_scope = (a == b and a in IN_SCOPE_FILES) or pair == frozenset(['src/main.rs', 'src/obs.rs'])
        new = pair not in ALLOWED_PAIRS
        if in_scope or new:
            flagged += 1
            kind = 'in-scope' if in_scope else 'new'
            print(f'{kind}: {a}:{d["firstFile"]["start"]} <-> {b}:{d["secondFile"]["start"]} {d["lines"]} L')
    print(f'pairs {len(report["duplicates"])}')
    print(flagged)


if __name__ == '__main__':
    commands = {'size': size, 'cognitive': cognitive, 'clones': clones}
    if len(sys.argv) != 2 or sys.argv[1] not in commands:
        print('usage: metrics.py size|cognitive|clones')
        sys.exit(2)
    commands[sys.argv[1]]()
