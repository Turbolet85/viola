"""P3 record assembly (audit-pass.md §Schema) -> record.json, with the Caps-paragraph asserts before any append."""
import json, os, subprocess

R = '.andromeda/runs/2026-10-01T09-18-50-code-audit'
RUN = R
J = lambda n: json.load(open(f'{R}/{n}', encoding='utf-8'))
BASE = '69abc0d038732cff8d45602f82f8bebaf28bbb72'
BOUND = '093bffb24c7fa2aae5fb79e27742b993c8cccb51'
HEAD = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
UNITS = ['viola-core', 'viola-pty', 'viola-state', 'viola-channel', 'viola-agent-claude', 'viola', 'viola-e2e']

recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
base = [r for r in recs if r['sha'] == BASE][-1]

dup = J('c-duplication.json'); dup.pop('sources', None)
cx = J('c-complexity.json'); cx.pop('functions', None)
sz = J('c-sizes.json')
gr = J('c-graph.json'); gr.pop('edge_list', None)
dd = J('c-dead.json')
ch = J('c-churn.json')
hs = J('c-hotspots.json')['hotspots']
cov = J('c-coverage.json')
bnm = J('c-baseline-not-measured.json')

mut = {'scoped_units': UNITS, 'unit_states': {}, 'scores': {}, 'counts': {}, 'score_formula': 'caught/(caught+missed)',
       'survivors': [], 'host': None, 'not_measured': []}
skips = []
for u in UNITS:
    p = f'{R}/c-mutation-{u}.json'
    if not os.path.isfile(p):
        mut['unit_states'][u] = 'baseline-test-failure'
        skips.append({'metric': f'mutation:{u}', 'reason': 'baseline-test-failure',
                      'note': 'the unmutated tree fails 32 of 246 harness tests in cargo-mutants\' copied tree: they spawn target/debug/viola-fake-agent.exe, a ROOT-package bin a -p viola-e2e build never produces (os error 2). Three forms tried, all built and tested --package=viola-e2e only: per-unit; --test-package viola-e2e --test-package viola; --test-workspace=true. The project\'s own run --mutants prebuild + --copy-target form was not tried.'})
        continue
    m = J(f'c-mutation-{u}.json')
    mut['unit_states'][u] = m['unit_state']
    mut['counts'][u] = m['counts']
    if m['score'] is not None:
        mut['scores'][u] = m['score']
    mut['survivors'] += m['survivors']
    mut['not_measured'] += m['not_measured']
    mut['host'] = m['host']
    # Caps-paragraph asserts, per unit, before the append
    assert len({tuple(r) for r in m['survivors']}) == len(m['survivors']) == m['counts']['missed'], u
    assert len({tuple(r) for r in m['not_measured']}) == len(m['not_measured']) == m['counts']['not_measured'], u
    assert m['counts']['missed'] + m['counts']['not_measured'] == m['tool_tally']['missed'], u

overshoot_files = []  # git diff --numstat BOUND..HEAD filtered by the source-path definition: measured empty
rec = {
 'ts': os.environ['AUDIT_TS'], 'epoch': 'Epoch 2b — Windows slice I b: events and ledger', 'mode': 'trend',
 'sha': HEAD, 'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
 'head_overshoot': {'boundary_sha': BOUND, 'commits': 3, 'files': overshoot_files,
                    'note': '3 commits past the flip of 2026-09-29-fake-agent-drift-contract (11f135c registry migration, 4a3062d refs doc, 95c1a9b .gitignore): no source path touched - .andromeda/**, .claude/**, refs/**, .gitignore only; operator-confirmed trend at HEAD'},
 'tool_versions': base['tool_versions'],
 'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': 7},
 'duplication': dup, 'complexity': cx, 'sizes': sz['sizes'], 'graph': gr,
 'dead': {'unused_deps': dd['unused_deps'], 'zero_ref_candidates': dd['zero_ref_candidates'], 'top': dd['top']},
 'coverage': {'line': cov['line'], 'branch': cov['branch']},
 'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
 'hotspots': hs, 'mutation': mut,
 'commands': {
  'sizes': f'tokei --output json $(cat {RUN}/source-files.txt)  # population: git ls-files "*.rs" minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/ (98 files); summarizer {RUN}/a13.py',
  'duplication': f'jscpd --reporters json --output {RUN}/jscpd --format rust --silent --ignore "fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**,viola-0.2.0-incubator/**" .  # summarizer {RUN}/a13.py (reproduces the 69abc0d record exactly)',
  'complexity': f'mkdir -p {RUN}/rca ; rust-code-analysis-cli -m -O json -o {RUN}/rca $(sed "s/^/-p /" {RUN}/source-files.txt)  # per-function (closures included) cognitive = metrics.cognitive.sum, cyclomatic = metrics.cyclomatic.sum; {RUN}/cx.py ... all (reproduces the 69abc0d record exactly)',
  'graph': f'python scripts/code-graph.py query {RUN} code-audit "<audit-pass.md canonical CYCLES / FAN-IN / FAN-OUT / edge-count SQL, verbatim>" rust  # driven by {RUN}/graph_q.py',
  'dead': f'cargo machete ; python scripts/code-graph.py query {RUN} code-audit "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line" rust  # classed by {RUN}/dead.py + {RUN}/churn.py (tests/ segment union, FP classes, 3 residuals reviewed by hand)',
  'coverage': cov.get('command'),
  'churn': f'git log --reverse --numstat --format=@%H {BASE}..HEAD -- "*.rs"  # minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/; {RUN}/churn.py',
  'hotspots': 'c-churn per-file commit count x max per-function cognitive in the file (c-complexity)',
  'mutation': f'NEXTEST_PROFILE=mutants timeout 14400 cargo mutants -p {{unit}} --test-tool=nextest -j 4 --output {RUN}/mutants-{{unit}}  # one unit at a time via {RUN}/mut.sh, TMP/TEMP set to a session scratch dir outside the repo; viola and viola-e2e add --features fake-agent; viola-e2e re-invoked by {RUN}/mut-e2e.sh as mutants-viola-e2e-b (--test-package viola-e2e --test-package viola) and mutants-viola-e2e-c (--test-workspace=true)',
  'head_overshoot': f"git log --format=%H -1 -S'2026-09-29-fake-agent-drift-contract · complete' -- .andromeda/master-route.md ; git rev-list --count {BOUND}..HEAD ; git diff --numstat {BOUND}..HEAD -- (source paths)",
 },
 'corrections': [
  {'target_sha': BASE, 'field': 'mutation.not_measured',
   'was': None, 'now': bnm['not_measured'],
   'note': 'schema-gap fill: the record predates not_measured. Recomputed at 69abc0d (cargo mutants --list --json over a git-archive extract + the pinned collectors.md cover(), host x86_64-pc-windows-msvc): 9 of its 11 survivors are #[cfg(unix)] spans the host never builds. Matched back by site + text; the record had dropped the tool\'s " in {function}" suffix on 3 operator mutants and one match guard. By ' + RUN + '/base_nm.py'},
  {'target_sha': BASE, 'field': 'mutation.counts',
   'was': {u: base['mutation']['counts'][u] for u in ('viola-channel', 'viola-pty')},
   'now': {'viola-channel': dict(base['mutation']['counts']['viola-channel'], missed=0, not_measured=4),
           'viola-pty': dict(base['mutation']['counts']['viola-pty'], missed=0, not_measured=5)},
   'note': 'the not_measured split of the fill above; viola-state (2 missed), viola-agent-claude and viola-core unchanged (0 not_measured)'},
  {'target_sha': BASE, 'field': 'mutation.scores',
   'was': {u: base['mutation']['scores'][u] for u in ('viola-channel', 'viola-pty')},
   'now': {'viola-channel': 100.0, 'viola-pty': 100.0},
   'note': 'caught/(caught+missed) with host-excluded mutants out of missed: 118/118 and 64/64; mutation-drop reads these'},
 ],
 'skips': skips + cov.get('skips', []),
}
# not_measured asserted against the corrected baseline counts too
assert len(bnm['not_measured']) == 4 + 5 and not bnm['unmatched']
json.dump(rec, open(f'{R}/record.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(json.dumps({k: rec[k] for k in ('sha', 'coverage', 'skips')}, ensure_ascii=False)[:1500])
print('survivors', len(mut['survivors']), 'not_measured', len(mut['not_measured']), 'scores', mut['scores'])
