"""Phase 3: assemble record.json (audit-pass.md, Schema) from this run dir's evidence twins. Writes nothing else.

Refuses (exit non-zero, no file) when a twin is missing, a unit's survivors are not unique on (site, mutation) or
their number differs from counts[unit].missed, the not_measured list likewise, or missed + not_measured differs from
the tool's own missed tally.
"""
import datetime
import json
import os
import subprocess
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))
RD = os.path.relpath(RUN, REPO)
EPOCH = 'Epoch 4 — Session state & governance'
BASE = 'e304994ae0413c5cf5bf679a5b3d19abded0e472'
UNITS = ['viola', 'viola-core', 'viola-pty', 'viola-state', 'viola-agent-claude', 'viola-e2e']
EXCL = '^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/'
IGN = 'fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**,viola-0.2.0-incubator/**'


def load(name):
    return json.load(open(os.path.join(RUN, name), encoding='utf-8'))


def git(*a):
    return subprocess.run(['git', *a], cwd=REPO, capture_output=True, text=True, check=True).stdout.strip()


def tok(cmd):
    out = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True).stdout.split()
    for w in out:
        if w[:1].isdigit():
            return w
    return None


head = git('rev-parse', 'HEAD')
assert subprocess.run(['git', 'merge-base', '--is-ancestor', BASE, head], cwd=REPO).returncode == 0, 'ancestry'
boundary = git('log', '--format=%H', '-1', '-S2026-10-11-the-board-viola-list · complete', '--', '.andromeda/master-route.md')
assert boundary == head and git('rev-list', '--count', f'{boundary}..HEAD') == '0'

tv = {'jscpd': tok(['jscpd', '--version']), 'tokei': tok(['tokei', '--version']),
      'rust-code-analysis': tok(['rust-code-analysis-cli', '--version']),
      'cargo-machete': tok(['cargo', 'machete', '--version']), 'cargo-mutants': tok(['cargo', 'mutants', '--version']),
      'cargo-llvm-cov': tok(['cargo', 'llvm-cov', '--version']), 'cargo-nextest': tok(['cargo', 'nextest', '--version']),
      'rustc': tok(['rustc', '--version']), 'code-graph.py': git('rev-parse', 'HEAD:scripts/code-graph.py')[:8]}
assert all(tv.values()), tv

sz, dp, cx, gr, dd = load('c-sizes.json'), load('c-dup.json')['duplication'], load('c-cx.json')['complexity'], \
    load('c-graph.json'), load('c-dead.json')
ch, hs, cov = load('c-churn.json'), load('c-hotspots.json')['hotspots'], load('c-coverage.json')
units_n = len(json.loads(subprocess.run(['cargo', 'metadata', '--no-deps', '--format-version', '1'], cwd=REPO,
                                        capture_output=True, text=True, check=True).stdout)['packages'])

mut = {'scoped_units': UNITS, 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
       'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': []}
skips = []
for u in UNITS:
    m = load(f'c-mutation-{u}.json')
    c = m['counts']
    st = m['state']
    mut['unit_states'][u] = st
    mut['counts'][u] = c
    if mut['host'] is None:
        mut['host'] = m['host']
    assert m['host'] == mut['host']
    sv, nm = [tuple(s) for s in m['survivors']], [tuple(s[:2]) for s in m['not_measured']]
    assert len(set(sv)) == len(sv) == c['missed'], (u, 'survivors')
    assert len(set(nm)) == len(nm) == c['not_measured'], (u, 'not_measured')
    assert c['missed'] + c['not_measured'] == m['tool_tallies']['missed'], (u, 'missed split')
    mut['survivors'] += [list(s) for s in m['survivors']]
    mut['not_measured'] += [list(s) for s in m['not_measured']]
    if st.startswith('complete'):
        mut['scores'][u] = m['score']
        mut['timing'][u] = m['timing']
    elif st.startswith('budget-exhausted'):
        mut['timing'][u] = m['timing']
        skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted'})
    elif st.startswith('unviable-dominant'):
        skips.append({'metric': f'mutation:{u}', 'reason': 'unviable-dominant', 'note': json.dumps(c)})
    else:
        sys.exit(f'{u}: state {st} not recordable')
if cov['branch'] is None:
    skips.append({'metric': 'coverage.branch', 'reason': 'declined',
                  'note': 'the project coverage command does not instrument branches (as at the e304994 record)'})

pop = f'git ls-tree -r --name-only HEAD "*.rs" minus {EXCL} ({sz["population_files"]} files)'
mtpl = (f'TMPDIR=~/dev/projects/viola-mutants-scratch NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 '
        f'AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants cargo mutants --package {{unit}} --features fake-agent '
        f'--test-tool=nextest --copy-target=false --caught --unviable --build-timeout=400 -j 2 '
        f'--output {RD}/mutants-{{unit}}')
commands = {
    'sizes': f'tokei --output json <population>  # population: {pop}; summarizer python3 {RD}/collect.py sizes . HEAD {RD}/c-sizes.json',
    'duplication': f'jscpd --reporters json --output {RD}/jscpd --format rust --silent --ignore "{IGN}" .  # summarizer {RD}/collect.py dup (report deleted after summarizing)',
    'complexity': f'rust-code-analysis-cli -m -O json -o {RD}/rca -p <each population file>  # population as commands.sizes; per-function (closures included) cognitive = metrics.cognitive.sum, cyclomatic = metrics.cyclomatic.sum; summarizer {RD}/collect.py cx',
    'graph': f'python3 scripts/code-graph.py query {RD} code-audit "<audit-pass.md canonical CYCLES / FAN-IN / FAN-OUT / edge-count SQL, verbatim>" rust  # driven by {RD}/graph_q.py',
    'dead': f'cargo machete ; python3 scripts/code-graph.py query {RD} code-audit "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line" rust  # classed by {RD}/graph_q.py + {RD}/churn.py (recipes.dead)',
    'coverage': f"cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path {RD}/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\\\]viola-e2e|tests[/\\\\]support|fuzz[/\\\\])'  # run by path: {RD}/cov.sh (once); summarizer {RD}/covsum.py",
    'churn': f'git log --reverse --numstat --format=@%H {BASE}..HEAD -- "*.rs"  # minus {EXCL}; {RD}/churn.py',
    'hotspots': 'c-churn per-file commit count x max per-function cognitive in the file (c-cx)',
    'head_overshoot': f"git log --format=%H -1 -S'2026-10-11-the-board-viola-list · complete' -- .andromeda/master-route.md ; git rev-list --count {head}..HEAD ; git diff --numstat {head}..HEAD -- '*.rs' minus {EXCL}",
    'mutation': (mtpl + '  # one unit at a time via ' + RD + '/mut-driver.py; every VIOLA_* and CLAUDE* name unset for the '
                 'children; viola-core without --features fake-agent (its manifest declares none); --copy-target=false '
                 'is a deviation from the e304994 form (--copy-target=true at jobs 1), the operator\'s, to keep the 100 GB '
                 'repo target/ from being copied into each build dir; no prebuild (no copy reaches a build dir); '
                 f'summarizer {RD}/mutsum.py'),
    'mutation:viola': (mtpl.replace('{unit}', 'viola') + ' , stopped on the founder\'s word at 678/1299 (copied byte for '
                       f'byte to {RD}/mutants-viola-partial) ; then the same form with --iterate --output {RD}/mutants-viola-iter '
                       f'against a further copy of it (636 = 621 untested + 15 missed, read back by --iterate --list --json first) ; '
                       f'merged by {RD}/merge_viola.py (the first\'s 663 caught/unviable + the resume\'s 636) and scored by mutsum.py; '
                       'wall_s is the two invocations summed'),
    'mutation:viola-e2e': (f'git archive {head} | tar -x -C {RD}/x-src ; (cd {RD}/x-src && CARGO_TARGET_DIR={RD}/x-src/target/mutants '
                           'cargo build --package viola --features fake-agent) ; (cd ' + RD + '/x-src && TMPDIR=~/dev/projects/viola-mutants-scratch '
                           'NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants '
                           'cargo mutants --package viola-e2e --features fake-agent --test-tool=nextest --copy-target=true --caught '
                           f'--unviable --build-timeout=400 -j 2 --output {RD}/mutants-viola-e2e-x)  # via {RD}/e2e_x.py; the first '
                           f'invocation in the repository under the --copy-target=false form failed its unmutated baseline '
                           f'(viola-fake-agent absent from the build dir; {RD}/c-mutation-viola-e2e-first.json)'),
}
rec = {
    'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': EPOCH, 'mode': 'trend', 'sha': head, 'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': head, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the flip of 2026-10-11-the-board-viola-list'},
    'tool_versions': tv,
    'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': units_n},
    'duplication': {'pct': round(dp['pct'], 2), 'duplicated_lines': dp['duplicated_lines'], 'total_lines': dp['total_lines'],
                    'clones': dp['clones'], 'top': dp['top'], 'split': dp['split']},
    'complexity': cx, 'sizes': sz['sizes'],
    'graph': {k: gr[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
    'dead': {'unused_deps': dd['unused_deps'], 'zero_ref_candidates': dd['zero_ref_candidates'], 'top': dd['top']},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
    'hotspots': hs, 'mutation': mut, 'commands': commands,
    'recipes': {'dead': [json.loads(l) for l in open(os.path.join(REPO, '.andromeda/code-metrics.ndjson'), encoding='utf-8')
                         if l.strip() and json.loads(l).get('sha') == BASE][-1]['recipes']['dead']},
    'corrections': [], 'skips': skips,
}
assert len(rec['duplication']['top']) <= 10 and len(rec['complexity']['top']) <= 10 and len(rec['sizes']['top']) <= 10
assert len(rec['hotspots']) <= 10 and len(rec['graph']['fan_in_top']) <= 20
assert rec['graph']['cycles'] == len(rec['graph']['cycle_paths'])
with open(os.path.join(RUN, 'record.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(rec, f, ensure_ascii=False, indent=1)
    f.write('\n')
print('record.json', {u: rec['mutation']['unit_states'][u] for u in UNITS}, 'scores', rec['mutation']['scores'],
      'survivors', len(mut['survivors']), 'not_measured', len(mut['not_measured']), 'skips', skips, 'tv', tv)
