#!/usr/bin/env python3
"""P3: assemble record.json from the c-*.json twins (audit-pass.md §Schema). usage: assemble.py <run_dir> <ts>"""
import json, os, re, subprocess, sys

R, TS = sys.argv[1], sys.argv[2]
RR = '.andromeda/runs/2026-10-19T21-08-30-code-audit'
J = lambda n: json.load(open(f'{R}/{n}'))
HEAD = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
BASE = 'dffad3e02d380362b8040effacc95781ceea8739'
UNITS = ['viola', 'viola-core', 'viola-pty', 'viola-channel', 'viola-state', 'viola-agent-claude', 'viola-mcp', 'viola-e2e']
EPOCH = next(l for l in open('viola-0.1.0/working-route.md', encoding='utf-8') if l.startswith('### Epoch 5 '))[4:].rstrip('\n')

recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
base = [r for r in recs if r['sha'] == BASE][-1]

sz, cx, dp, gr, dd, cv, ch, ht = (J(f'c-{n}.json') for n in
                                  ('sizes', 'complexity', 'duplication', 'graph', 'dead', 'coverage', 'churn', 'hotspots'))

mut = {'scoped_units': UNITS, 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
       'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': []}
skips = [{'metric': 'coverage.branch', 'reason': 'declined',
          'note': 'the project coverage command does not instrument branches (as at the two prior records)'}]
states = J('mut-states.json') if os.path.exists(f'{R}/mut-states.json') else {}
for u in UNITS:
    f = f'{R}/c-mutation-{u}.json'
    if os.path.exists(f):
        d = json.load(open(f))
        mut['unit_states'][u] = d['unit_state']
        mut['counts'][u] = d['counts']
        mut['timing'][u] = d['timing']
        assert len(d['survivors']) == d['counts']['missed'] and len({tuple(s) for s in d['survivors']}) == len(d['survivors'])
        assert len(d['not_measured']) == d['counts']['not_measured']
        mut['survivors'] += d['survivors']
        mut['not_measured'] += d['not_measured']
        if d['unit_state'].startswith('unviable-dominant'):
            skips.append({'metric': f'mutation:{u}', 'reason': 'unviable-dominant', 'note': json.dumps(d['counts'])})
        else:
            mut['scores'][u] = d['score']
    else:
        s = states.get(u, {})
        st = s.get('state', 'not run')
        mut['unit_states'][u] = st
        if st == 'budget-exhausted':
            mut['timing'][u] = {k: s[k] for k in ('scope', 'planned', 'tested', 'wall_s', 'jobs', 'cap_s')}
            skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted'})
        elif st == 'exited-incomplete' and s.get('exit') == 4:
            note = f"cargo mutants exit 4 after {s.get('wall_s')}s: the unmutated baseline failed; see {RR}/mut-{u}.log"
            if u == 'viola':
                note = ("files scope (CARRY 2's two files, 265 mutants listed): two attempts, both exit 4 at the unmutated baseline "
                        "under the nextest mutants profile's 10 s kill. 21:50Z (host load avg ~67, another project's sweep): TIMEOUTs in "
                        "cli_revive / cli_send / cli_verify and 4 send_window_* 'wrapper builder not ready'. 22:51Z (load avg ~4): "
                        "3 TIMEOUTs at 10.00 s, contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set "
                        "(10.577 s PASS unkilled in this run's coverage pass) and cli_send send_under_the_paste_hint_a_human_key_during_"
                        "the_gate_wait_wins / ..._on_a_verified_cli::case_1_hint (9.010 s / 9.346 s unkilled). CARRY 2's grading stays owed.")
            skips.append({'metric': f'mutation:{u}', 'reason': 'baseline-test-failure', 'note': note})
        else:
            skips.append({'metric': f'mutation:{u}', 'reason': 'declined', 'note': st})
skips.append({'metric': 'mutation:viola (outside src/bin/viola-fake-agent.rs and src/cmd/verify.rs: 1278 of 1543 listed)',
              'reason': 'declined', 'note': "the founder's word at the attended confirm: the root unit scoped to CARRY 2's two files"})
mut['host'] = next(l.split(': ', 1)[1] for l in subprocess.run(['rustc', '-vV'], capture_output=True, text=True).stdout.splitlines()
                   if l.startswith('host: '))

POP = "git ls-tree -r --name-only HEAD minus non-*.rs minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/ (179 files)"
MUT = ("TMPDIR=~/dev/projects/viola-mutants-scratch/e5 NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 "
       "CARGO_TARGET_DIR=target/mutants cargo mutants --test-tool=nextest --caught --unviable --build-timeout=400 -j 2 "
       "--package {unit} --features fake-agent --copy-target=false --output " + RR + "/mutants-{unit}")
rec = {
    'ts': TS, 'epoch': EPOCH, 'mode': 'trend', 'sha': HEAD, 'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': HEAD, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the flip of 2026-10-19-overseer-switch-to-viola'},
    'tool_versions': {'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'cargo-machete': '0.9.2',
                      'cargo-mutants': '27.1.0', 'cargo-llvm-cov': '0.9.1', 'cargo-nextest': '0.9.146', 'rustc': '1.98.1',
                      'code-graph.py': 'f22becbb'},
    'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': len(UNITS)},
    'duplication': {k: dp[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top')},
    'sizes': {k: sz[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': gr,
    'dead': {k: dd[k] for k in ('unused_deps', 'zero_ref_candidates', 'top')},
    'coverage': {'line': cv['line'], 'branch': None},
    'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
    'hotspots': ht['hotspots'],
    'mutation': mut,
    'commands': {
        'sizes': f"tokei --output json <population>  # population: {POP}; summarizer python3 {RR}/collect.py sizes . HEAD {RR}/c-sizes.json",
        'duplication': f'jscpd --reporters json --output {RR}/jscpd --format rust --silent --ignore "fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**,viola-0.2.0-incubator/**" .  # summarizer {RR}/collect.py dup (split: test = both files carry a (^|/)tests/ segment); report deleted after summarizing',
        'complexity': f"rust-code-analysis-cli -m -O json -o {RR}/rca -p <each population file>  # population as commands.sizes; per-function (closures included: every space of kind function) cognitive = metrics.cognitive.sum, cyclomatic = metrics.cyclomatic.sum; summarizer {RR}/collect.py cx",
        'graph': f'python3 scripts/code-graph.py query {RR} code-audit "<audit-pass.md canonical CYCLES / FAN-IN / FAN-OUT / edge-count SQL, verbatim, whitespace collapsed to one line>" rust  # driven by {RR}/graph_q.py',
        'dead': f'cargo machete ; python3 scripts/code-graph.py query {RR} code-audit "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line" rust  # classed by {RR}/graph_q.py (stages 1-2) + {RR}/dead.py (stage 3, the hand read); recipes.dead',
        'coverage': f"cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path {RR}/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\\\]viola-e2e|tests[/\\\\]support|fuzz[/\\\\])'  # run by path under env -i HOME PATH: {RR}/cov.sh; summarizer {RR}/graph_q.py",
        'churn': f'git log --reverse --numstat --format=@%H {BASE}..HEAD -- "*.rs"  # minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/; {RR}/collect.py churn',
        'hotspots': 'c-churn per-file commit count x max per-function cognitive in the file (c-complexity)',
        'head_overshoot': f"git log --format=%H -1 -S'2026-10-19-overseer-switch-to-viola · complete' -- .andromeda/master-route.md ; git rev-list --count {HEAD}..HEAD ; git diff --numstat {HEAD}..HEAD -- '*.rs' minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/",
        'mutation': MUT + f"  # one unit at a time via {RR}/mut_driver.py, under env -i (no VIOLA_* or CLAUDE* value crosses); viola-core without --features fake-agent (its manifest declares none), run by hand first in the same form; summarizer {RR}/mutsum.py",
        'mutation:viola': MUT.replace('{unit}', 'viola').replace('--copy-target=false', '--copy-target=false --file src/bin/viola-fake-agent.rs --file src/cmd/verify.rs') + "  # scope files: CARRY 2's two files, the founder's word at the confirm",
        'mutation:viola-e2e': f"git archive HEAD | tar -x -C {RR}/x-src ; (cd {RR}/x-src && CARGO_TARGET_DIR={RR}/x-src/target/mutants cargo build --package viola --features fake-agent) ; (cd {RR}/x-src && TMPDIR=~/dev/projects/viola-mutants-scratch/e5 NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants cargo mutants --test-tool=nextest --caught --unviable --build-timeout=400 -j 2 --package viola-e2e --features fake-agent --copy-target=true --output {RR}/mutants-viola-e2e-x)  # via {RR}/mut_driver.py",
    },
    'recipes': {'dead': base['recipes']['dead']},
    'corrections': [],
    'skips': skips,
}
json.dump(rec, open(f'{R}/record.json', 'w'), ensure_ascii=False, indent=1)
print('record assembled', len(json.dumps(rec, ensure_ascii=False)), 'bytes; units', mut['unit_states'])
