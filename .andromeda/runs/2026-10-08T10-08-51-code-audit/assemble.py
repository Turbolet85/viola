"""P3: assemble the ledger record (audit-pass.md, Schema) from the c-{metric}.json twins -> record.json.
Asserts the per-unit survivor and not_measured rows against their counts before anything is written."""
import datetime, json, subprocess

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
BASE = '95c1a9b5fc5984a9a2024e5d6e5c51f59d6af255'
SCRATCH = '/home/turbolet/dev/projects/viola-mutants-scratch/e3'
UNITS = ['viola-core', 'viola-pty', 'viola-state', 'viola-channel', 'viola-agent-claude', 'viola', 'viola-e2e']
J = lambda n: json.load(open(f'{R}/{n}', encoding='utf-8'))
sha = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
assert sha == 'e304994ae0413c5cf5bf679a5b3d19abded0e472', sha

dup, cx, sz, gr, dead, cov, churn, hs = (J('c-duplication.json'), J('c-complexity.json'), J('c-sizes.json'),
                                         J('c-graph.json'), J('c-dead.json'), J('c-coverage.json'),
                                         J('c-churn.json'), J('c-hotspots.json'))

# --- mutation
unit_states, scores, counts, timing, survivors, not_measured, skips = {}, {}, {}, {}, [], [], []
e2e = J('mut-done-viola-e2e.json')
assert e2e['state'] == 'baseline-test-failure', e2e['state']
for u in UNITS:
    if u == 'viola-e2e':
        unit_states[u] = 'baseline-test-failure'
        continue
    m = J(f'c-mutation-{u}.json')
    c = m['counts']
    for rows, n in ((m['survivors'], c['missed']), (m['not_measured'], c['not_measured'])):
        keys = [(r[0], r[1]) for r in rows]
        assert len(keys) == len(set(keys)) == n, (u, len(keys), n)
    assert c['missed'] + c['not_measured'] == m['tool_totals']['missed'], (u, c, m['tool_totals'])
    unit_states[u] = m['unit_state']
    counts[u] = c
    survivors += m['survivors']
    not_measured += m['not_measured']
    if m['unit_state'].startswith('complete'):
        scores[u] = m['score']
        timing[u] = m['timing']
    elif m['unit_state'] == 'budget-exhausted':
        timing[u] = m['timing']
        skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted',
                      'note': f"stopped at its cap {m['timing']['cap_s']} s with {m['timing']['tested']} of {m['timing']['planned']} tested"})
    else:
        skips.append({'metric': f'mutation:{u}', 'reason': 'unviable-dominant', 'note': json.dumps(c)})

skips.append({'metric': 'mutation:viola-e2e', 'reason': 'baseline-test-failure',
              'note': "the unmutated tree's test run timed out on one test, viola-e2e::harness_lifecycle "
                      "boot_with_an_unknown_cli_version_is_verify_failed, killed at the mutants profile's 30 s "
                      "(package(viola-e2e): 15 s x 2); 257 of 258 passed. Three readings: the 30 s kill here; 34.477 s "
                      "PASS for the same test in this run's coverage command (profile ci); the Windows runner's job "
                      "killed it at 30.009 s (run 37761947926). Backing tmpfs, hostwatch QUIET for the window "
                      "(2026-10-08T11:00:00Z..11:02:10Z). Not re-run. A report-only run with that test deselected "
                      "(the operator's answer; stopped on a second answer at 165 of 718: 139 caught, 0 missed, 26 unviable) "
                      "is in the run dir, never in scores."})
skips.append({'metric': 'coverage.branch', 'reason': 'declined',
              'note': 'the project coverage command does not instrument branches (as at the 95c1a9b record)'})

MUT = ('TMPDIR=' + SCRATCH + ' NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 '
       'CARGO_TARGET_DIR=target/mutants cargo mutants --package {unit} --features fake-agent --test-tool=nextest '
       '--copy-target=true --caught --unviable --build-timeout=400 '
       '--output ' + R + '/mutants-{unit}  # jobs 1 (no -j); one unit at a time via ' + R + '/mut-driver.py after one '
       '`CARGO_TARGET_DIR=<repo>/target/mutants cargo build --package viola --features fake-agent`; every VIOLA_* and '
       'CLAUDE* name unset for the children; viola-core without --features fake-agent (its manifest declares none); '
       'TMPDIR is the one absolute path (the operator\'s NOCOW scratch, outside the repository); summarizer ' + R +
       '/mutsum.py + cover.py')
IGN = '"fuzz/**,scripts/**,.andromeda/**,.claude/**,docs/**,refs/**,e2e-web/**,target/**,viola-0.2.0-incubator/**"'
commands = {
 'sizes': f'tokei --output json $(cat {R}/source-files.txt)  # population: git ls-files "*.rs" minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/ ({sz["totals"]["files"]} files); summarizer {R}/a13.py',
 'duplication': f'jscpd --reporters json --output {R}/jscpd --format rust --silent --ignore {IGN} .  # summarizer {R}/a13.py',
 'complexity': f'mkdir -p {R}/rca ; rust-code-analysis-cli -m -O json -o {R}/rca $(sed "s/^/-p /" {R}/source-files.txt)  # per-function (closures included) cognitive = metrics.cognitive.sum, cyclomatic = metrics.cyclomatic.sum; {R}/cx.py {R}/rca all {R}/c-complexity.json',
 'graph': f'python scripts/code-graph.py query {R} code-audit "<audit-pass.md canonical CYCLES / FAN-IN / FAN-OUT / edge-count SQL, verbatim>" rust  # driven by {R}/graph_q.py',
 'dead': f'cargo machete ; python scripts/code-graph.py query {R} code-audit "SELECT s.symbol, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL ORDER BY s.file, s.def_line" rust  # classed by {R}/dead.py + {R}/churn.py (recipes.dead)',
 'coverage': "cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path " + R + "/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\\\]viola-e2e|tests[/\\\\]support|fuzz[/\\\\])'  # run by path: " + R + "/cov.sh (once); summarizer " + R + "/covsum.py",
 'churn': f'git log --reverse --numstat --format=@%H {BASE}..HEAD -- "*.rs"  # minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/; {R}/churn.py',
 'hotspots': 'c-churn per-file commit count x max per-function cognitive in the file (c-complexity)',
 'mutation': MUT,
 'head_overshoot': f"git log --format=%H -1 -S'2026-10-08-first-live-test-and-self-drive · complete' -- .andromeda/master-route.md ; git rev-list --count {sha}..HEAD ; git diff --numstat {sha}..HEAD -- \"*.rs\"  # minus ^(scripts|docs|refs|fuzz|.andromeda|.claude|viola-0.2.0-incubator)/",
}
DEAD_RECIPE = (
 "raw = the zero-ref query of commands.dead over the rust plane. Stage 1: strip the SCIP prefix "
 "^rust-analyzer cargo \\S+ \\S+ from the symbol, then exclude a row when (^|/)tests/ matches the stripped SYMBOL "
 "path OR the FILE path (the union; a leading segment counts). Stage 2, first match wins: kind == module -> "
 "'module (not a callable; reached by path)'; symbol ends main(). at a path root, or the file is under src/bin/ and "
 "the symbol ends main(). -> 'entry point'; symbol matches impl#\\[[^\\]]*\\]\\[[^\\]]+\\] -> 'trait-impl method "
 "(dispatch)'; file under crates/viola-e2e/, or test_support in the file or symbol -> 'test-only helper (test crate / "
 "test-support)'. Stage 3: each residual is read at HEAD by hand and classed false-positive only into a family "
 "collectors.md A5 names (trait declaration reached by dispatch; derive/attr-invoked; inline format-string capture), "
 "its witness line recorded in c-dead.json classing.residual_reviewed_fp; what is left is zero_ref_candidates.")
corrections = [
 {'target_sha': BASE, 'field': 'mutation.timing', 'was': None,
  'now': {u: {'scope': 'unit'} for u in ['viola-core', 'viola-pty', 'viola-state', 'viola-channel', 'viola-agent-claude', 'viola']},
  'note': "schema-gap fill: the record predates mutation.timing. Scope read from that record's own commands.mutation "
          "(`cargo mutants -p {unit} --test-tool=nextest -j 4`: no file filter, no shard flag) -> `unit` for its six "
          "scored units; planned = counts[unit].mutants; wall time UNKNOWN (never a default), jobs 4 by the command."},
 {'target_sha': BASE, 'field': 'recipes', 'was': None, 'now': {'dead': DEAD_RECIPE},
  'note': "schema-gap fill: the record predates recipes. The A5 classing recipe recovered once from that record's own "
          "evidence twin (its run dir's dead.py, dead-classing.json and c-dead.json, named by its commands.dead) and "
          "pinned inline in this record's recipes.dead; its three reviewed residuals were the first three of this run's five."},
]
tv = {'jscpd': '5.0.16', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'cargo-machete': '0.9.2',
      'cargo-mutants': '27.1.0', 'cargo-llvm-cov': '0.9.1', 'cargo-nextest': '0.9.146', 'rustc': '1.98.1',
      'code-graph.py': 'd0425fb1'}
rec = {
 'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
 'epoch': 'Epoch 3 — Windows slice II: driving verbs and live proof', 'mode': 'trend',
 'sha': sha, 'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
 'head_overshoot': {'boundary_sha': sha, 'commits': 0, 'files': [],
                    'note': 'at the boundary: HEAD is the flip of 2026-10-08-first-live-test-and-self-drive'},
 'tool_versions': tv,
 'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': 7},
 'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
 'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top')},
 'sizes': sz['sizes'],
 'graph': {k: gr[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
 'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates', 'top')},
 'coverage': {'line': cov['line'], 'branch': None},
 'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned']},
 'hotspots': hs['hotspots'],
 'mutation': {'scoped_units': UNITS, 'unit_states': unit_states, 'scores': scores, 'counts': counts, 'timing': timing,
              'score_formula': 'caught/(caught+missed)', 'survivors': survivors,
              'host': 'x86_64-unknown-linux-gnu', 'not_measured': not_measured},
 'commands': commands, 'recipes': {'dead': DEAD_RECIPE}, 'corrections': corrections, 'skips': skips,
}
assert len(rec['duplication']['top']) <= 10 and len(rec['graph']['fan_in_top']) <= 20 and len(rec['hotspots']) <= 10
assert len(rec['graph']['cycle_paths']) == rec['graph']['cycles']
assert sum(c['missed'] for c in counts.values()) == len(survivors)
assert sum(c['not_measured'] for c in counts.values()) == len(not_measured)
for k, v in commands.items():
    assert '{run_dir}' not in v and '{boundary_sha}' not in v and '/home/' not in v.replace(SCRATCH, ''), k
line = json.dumps(rec, ensure_ascii=False)
assert '\n' not in line
json.dump(rec, open(f'{R}/record.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False)
print('record.json', len(line.encode('utf-8')), 'bytes; scores', scores, 'states', unit_states)
print('survivors', len(survivors), 'not_measured', len(not_measured), 'skips', [s['metric'] for s in skips])
