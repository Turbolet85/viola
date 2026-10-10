"""The chunk's instrument: the Epoch 3 audit's two scalars, re-taken on the audit record's own commands.

    scalars.py selftest      literal reports through the two summaries and checks; no tool runs
    scalars.py duplication   record.json commands.duplication (jscpd) into a fresh temporary directory
    scalars.py complexity    record.json commands.complexity (rust-code-analysis-cli), population commands.sizes

The summaries are the audit's (a13.py for duplication, cx.py for complexity, closures included). The file
population is the audit's rule read on the working tree: every `*.rs` file git lists, tracked or not yet tracked,
outside the excluded top-level directories. Every path is repository-relative; the one directory this script
makes is a temporary one it removes.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile

DUP_BOUND = 3.13
FRAGMENT_LINES = 20
CEILING = 15
LIFTED = {
    'tests/cli_answer.rs', 'tests/cli_wait_last.rs', 'tests/cli_wheel.rs', 'tests/tui_wheel.rs',
    'tests/cli_send.rs', 'tests/cli_instance_state.rs', 'tests/hook_events.rs',
    'tests/channel_paste_validation.rs',
}
ONE_FILE = 'tests/cli_verify.rs'
SPLIT = {
    'dialog_variants': 'crates/viola-agent-claude/src/ledger.rs',
    'record': 'src/cmd/verify.rs',
    'submit': 'src/bin/viola-fake-agent.rs',
}
JSCPD_IGNORE = ('fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**,'
                'viola-0.2.0-incubator/**')
EXCLUDED_TOP = ('scripts/', 'docs/', 'refs/', 'fuzz/', '.andromeda/', '.claude/', 'viola-0.2.0-incubator/')


def norm(path):
    return path.replace('\\', '/')


def named_fragment(first, second, lines):
    if lines < FRAGMENT_LINES:
        return False
    if first == second:
        return first == ONE_FILE
    return first in LIFTED and second in LIFTED


def summarize_duplication(report):
    total = report['statistics']['total']
    named = []
    for clone in report['duplicates']:
        first, second = norm(clone['firstFile']['name']), norm(clone['secondFile']['name'])
        if named_fragment(first, second, clone['lines']):
            named.append((first, second, clone['lines']))
    return {'pct': round(total['percentage'], 2), 'duplicated': total['duplicatedLines'],
            'total': total['lines'], 'clones': total['clones'], 'named': len(named)}


def grade_duplication(summary):
    return summary['pct'] < DUP_BOUND and summary['named'] == 0


def duplication_line(summary):
    return ('duplication: pct {pct:.2f} · duplicated {duplicated} of {total} · clones {clones} · '
            'named fragments {named}').format(**summary)


def functions(documents):
    out = []
    for document in documents:
        path = norm(document['name'])
        stack = [document]
        while stack:
            space = stack.pop()
            if space.get('kind') == 'function':
                out.append({'fn': space['name'], 'path': path, 'cog': space['metrics']['cognitive']['sum']})
            stack.extend(space.get('spaces', []))
    return out


def summarize_complexity(fns):
    summary = {'over_ceiling': sum(1 for f in fns if f['cog'] > CEILING)}
    for name, path in SPLIT.items():
        values = [f['cog'] for f in fns if f['fn'] == name and f['path'] == path]
        summary[name] = max(values) if values else None
    split_files = set(SPLIT.values())
    summary['over_in_split_files'] = sum(1 for f in fns if f['path'] in split_files and f['cog'] > CEILING)
    return summary


def grade_complexity(summary):
    return summary['over_ceiling'] == 1 and summary['over_in_split_files'] == 0


def shown(value):
    return 'absent' if value is None else str(int(value))


def complexity_line(summary):
    return 'complexity: over_ceiling {} · dialog_variants {} · record {} · submit {}'.format(
        summary['over_ceiling'], shown(summary['dialog_variants']), shown(summary['record']),
        shown(summary['submit']))


def repo_root():
    here = os.path.dirname(os.path.abspath(__file__))
    return subprocess.run(['git', 'rev-parse', '--show-toplevel'], cwd=here, check=True,
                          capture_output=True, text=True).stdout.strip()


def population(root):
    listed = subprocess.run(['git', 'ls-files', '--cached', '--others', '--exclude-standard', '--', '*.rs'],
                            cwd=root, check=True, capture_output=True, text=True).stdout.splitlines()
    return sorted(p for p in listed
                  if not p.startswith(EXCLUDED_TOP) and os.path.isfile(os.path.join(root, p)))


def run_duplication():
    root = repo_root()
    scratch = tempfile.mkdtemp(prefix='viola-scalars-')
    try:
        out = os.path.join(scratch, 'jscpd')
        subprocess.run(['jscpd', '--reporters', 'json', '--output', out, '--format', 'rust', '--silent',
                        '--ignore', JSCPD_IGNORE, '.'], cwd=root, check=True, stdout=subprocess.DEVNULL)
        with open(os.path.join(out, 'jscpd-report.json'), encoding='utf-8') as report:
            summary = summarize_duplication(json.load(report))
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    print(duplication_line(summary))
    return 0 if grade_duplication(summary) else 1


def run_complexity():
    root = repo_root()
    files = population(root)
    scratch = tempfile.mkdtemp(prefix='viola-scalars-')
    try:
        out = os.path.join(scratch, 'rca')
        os.mkdir(out)
        argv = ['rust-code-analysis-cli', '-m', '-O', 'json', '-o', out]
        for path in files:
            argv += ['-p', path]
        subprocess.run(argv, cwd=root, check=True, stdout=subprocess.DEVNULL)
        documents = []
        for base, _, names in os.walk(out):
            for name in names:
                if name.endswith('.json'):
                    with open(os.path.join(base, name), encoding='utf-8') as document:
                        documents.append(json.load(document))
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    summary = summarize_complexity(functions(documents))
    print(complexity_line(summary) + ' · files {} of {}'.format(len(documents), len(files)))
    return 0 if grade_complexity(summary) and len(documents) == len(files) else 1


def jscpd_report(pct, pairs):
    return {'statistics': {'total': {'percentage': pct, 'duplicatedLines': 100, 'lines': 5000,
                                     'clones': len(pairs)}},
            'duplicates': [{'firstFile': {'name': a}, 'secondFile': {'name': b}, 'lines': n}
                           for a, b, n in pairs]}


def rca_document(path, fns):
    return {'name': path, 'kind': 'unit', 'spaces': [
        {'name': name, 'kind': 'function', 'metrics': {'cognitive': {'sum': cog}}, 'spaces': [
            {'name': '<anonymous>', 'kind': 'function', 'metrics': {'cognitive': {'sum': inner}},
             'spaces': []} for inner in closures]}
        for name, cog, closures in fns]}


STANDING = rca_document('tests/cli_output_plain.rs', [('sgr_attributes', 23.0, [])])
QUIET_SPLITS = [
    rca_document('crates/viola-agent-claude/src/ledger.rs', [('dialog_variants', 9.0, [2.0])]),
    rca_document('src/cmd/verify.rs', [('record', 6.0, [])]),
    rca_document('src/bin/viola-fake-agent.rs', [('submit', 15.0, [])]),
]

DUPLICATION_CASES = [
    ('below the bound, nothing named', jscpd_report(2.9, []), True, 0),
    ('at the bound', jscpd_report(3.13, []), False, 0),
    ('above the bound', jscpd_report(3.36, []), False, 0),
    ('a 20-line pair between two lifted files',
     jscpd_report(2.9, [('tests/cli_answer.rs', 'tests/cli_wheel.rs', 20)]), False, 1),
    ('a 20-line pair inside cli_verify',
     jscpd_report(2.9, [('tests/cli_verify.rs', 'tests/cli_verify.rs', 20)]), False, 1),
    ('a 19-line pair between two lifted files',
     jscpd_report(2.9, [('tests/cli_answer.rs', 'tests/cli_wheel.rs', 19)]), True, 0),
    ('a long pair with one file outside the eight',
     jscpd_report(2.9, [('crates/viola-e2e/src/harness/mod.rs', 'tests/cli_instance_state.rs', 25)]), True, 0),
    ('a long pair inside one lifted file',
     jscpd_report(2.9, [('tests/cli_send.rs', 'tests/cli_send.rs', 25)]), True, 0),
    ('a backslash path is the same file',
     jscpd_report(2.9, [('tests\\cli_answer.rs', 'tests\\tui_wheel.rs', 27)]), False, 1),
]

COMPLEXITY_CASES = [
    ('one standing function over', [STANDING] + QUIET_SPLITS, True, 1),
    ('nothing over', QUIET_SPLITS, False, 0),
    ('a second function over in another file',
     [STANDING, rca_document('src/run/wheel.rs', [('step', 16.0, [])])] + QUIET_SPLITS, False, 2),
    ('the one function over sits in a split file',
     [rca_document('crates/viola-agent-claude/src/ledger.rs', [('dialog_variants', 16.0, [])])], False, 1),
    ('a helper closure over in a split file',
     [STANDING, rca_document('src/cmd/verify.rs', [('record', 6.0, [16.0])])], False, 2),
    ('a split function still over beside the standing one',
     [STANDING, rca_document('src/bin/viola-fake-agent.rs', [('submit', 16.0, [])])], False, 2),
]


def run_selftest():
    mismatches = 0
    for label, report, want_ok, want_named in DUPLICATION_CASES:
        summary = summarize_duplication(report)
        if (grade_duplication(summary), summary['named']) != (want_ok, want_named):
            mismatches += 1
            print('mismatch duplication · {} · {}'.format(label, duplication_line(summary)))
    for label, documents, want_ok, want_over in COMPLEXITY_CASES:
        summary = summarize_complexity(functions(documents))
        if (grade_complexity(summary), summary['over_ceiling']) != (want_ok, want_over):
            mismatches += 1
            print('mismatch complexity · {} · {}'.format(label, complexity_line(summary)))
    print('selftest: {} mismatches · {} duplication cases · {} complexity cases'.format(
        mismatches, len(DUPLICATION_CASES), len(COMPLEXITY_CASES)))
    return 0 if mismatches == 0 else 1


VERBS = {'selftest': run_selftest, 'duplication': run_duplication, 'complexity': run_complexity}

if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in VERBS:
        print('usage: scalars.py selftest|duplication|complexity')
        sys.exit(2)
    sys.exit(VERBS[sys.argv[1]]())
