"""P4 + P5: judge the movement against the fixed threshold table and render proposals.md.
Reads the ledger AFTER the append: this run's record is the last parseable line; the baseline is the last record
BEFORE it whose sha == baseline_sha; the third record of `monotonic` is found through the baseline's own
baseline_sha. Every evidence table's row count is asserted against a count from an independent source."""
import collections, json, re

R = '.andromeda/runs/2026-10-08T10-08-51-code-audit'
J = lambda n: json.load(open(f'{R}/{n}', encoding='utf-8'))
recs, bad = [], 0
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try: recs.append(json.loads(line))
    except ValueError: bad += 1
cur = recs[-1]
assert cur['sha'].startswith('e304994') and cur['ts'] == J('record.json')['ts'], 'the last line is not this run'
prior = recs[:-1]
base = [r for r in prior if r['sha'] == cur['baseline_sha']][-1]
base2 = [r for r in prior if r['sha'] == base['baseline_sha']][-1]
short = lambda s: s[:7]
ep = lambda r: r['epoch'].split(' — ')[0]
out = []
P = out.append

def table(header, rows, n, key, what):
    """rows rendered; n from an independent source; rows unique on the natural key"""
    assert len(rows) == n, f'{what}: {len(rows)} rows against n={n}'
    keys = [key(r) for r in rows]
    assert len(keys) == len(set(keys)), f'{what}: rows not unique on the key'
    P('| ' + ' | '.join(header) + ' |')
    P('|' + '---|' * len(header))
    return rows

cell = lambda s: str(s).replace('|', '\\|')

# ---------------------------------------------------------------- judge
d0, d1 = base['duplication'], cur['duplication']
c0, c1 = base['complexity'], cur['complexity']
dup_fires = d1['pct'] >= d0['pct'] + 0.5 and d1['pct'] >= d0['pct'] * 1.15
cx_fires = c1['over_ceiling'] >= c0['over_ceiling'] + 3 and c1['over_ceiling'] >= c0['over_ceiling'] * 1.25
dead_fires = cur['dead']['zero_ref_candidates'] >= base['dead']['zero_ref_candidates'] + 5
cyc_fires = cur['graph']['cycles'] > base['graph']['cycles']
TRACKED = [('duplication.pct', lambda r: r['duplication']['pct'], 1), ('complexity.over_ceiling', lambda r: r['complexity']['over_ceiling'], 1),
           ('dead.zero_ref_candidates', lambda r: r['dead']['zero_ref_candidates'], 1), ('sizes.file_max', lambda r: r['sizes']['file_max'], 1),
           ('sizes.over_800', lambda r: r['sizes']['over_800'], 1), ('coverage.line', lambda r: r['coverage']['line'], -1)]
mono = {}
for name, f, sign in TRACKED:
    v = [f(base2), f(base), f(cur)]
    mono[name] = {'values': v, 'evaluable': None not in v,
                  'fires': None not in v and (v[1] - v[0]) * sign > 0 and (v[2] - v[1]) * sign > 0}
mono_fired = [k for k, v in mono.items() if v['fires']]
mut_rows = []
for u in cur['mutation']['scoped_units']:
    s1, s0 = cur['mutation']['scores'].get(u), base['mutation']['scores'].get(u)
    mut_rows.append([u, s0, s1, None if None in (s0, s1) else round(s1 - s0, 2)])
mut_drop = [r for r in mut_rows if r[3] is not None and r[3] <= -10]
assert dup_fires and cx_fires and not dead_fires and not cyc_fires and not mut_drop, 'the prose below was written for this verdict'
assert mono_fired == ['duplication.pct', 'complexity.over_ceiling'], mono_fired
for t in cur['tool_versions']:
    assert cur['tool_versions'][t] == base['tool_versions'].get(t), f'trend-break {t}'
n_prop = 3

# ---------------------------------------------------------------- header
bo = base['head_overshoot']
P(f"# Code Audit — viola · {cur['epoch']} · {cur['ts']}")
P(f"mode {cur['mode']} · HEAD {short(cur['sha'])} · baseline {short(base['sha'])} ({base['epoch']}) · span {cur['span']}")
P(f"overshoot: {cur['head_overshoot']['commits']} commits — HEAD is the boundary (the flip of `2026-10-08-first-live-test-and-self-drive`). "
  f"The baseline record sat {bo['commits']} commits past its own boundary with {len(bo['files'])} source files in the delta; that delta is attributed to its epoch and not re-diffed here.")
P("no ancestry break · no trend-break (all nine tool tokens equal the baseline's)")
P(f"mutation tier host: `{cur['mutation']['host']}`; the baseline record's was `{base['mutation']['host']}` — the two records measure opposite halves of the `cfg` split, so each unit's `not measured` set differs by construction.")
P('')
P('The ledger after this append, the six tracked scalars per record:')
P('')
for r in table(['record (ts)', 'epoch', 'sha', 'dup %', 'over ceiling', 'zero-ref', 'file max', 'files > 800', 'line cov %'],
               recs, len(recs), lambda r: r['ts'], 'records'):
    P(f"| {r['ts']} | {ep(r)} | {short(r['sha'])} | {r['duplication']['pct']} | {r['complexity']['over_ceiling']} | {r['dead']['zero_ref_candidates']} | {r['sizes']['file_max']} | {r['sizes']['over_800']} | {r['coverage']['line'] if r['coverage']['line'] is not None else 'null (skip)'} |")
P('')

# ---------------------------------------------------------------- proposals
P('## Proposals')
P('')
rel = round(100 * (d1['pct'] - d0['pct']) / d0['pct'], 1)
P(f"### M1 — duplication-up · duplication.pct — +{round(d1['pct'] - d0['pct'], 2)} pt")
P(f"**Movement:** {d0['pct']} % → {d1['pct']} % ({ep(base)} → {ep(cur)}); +{rel} % relative. Fires at +0.5 pt and +15 %.")
P(f"**Evidence:** clones {d0['clones']} → {d1['clones']}; duplicated lines {d0['duplicated_lines']} → {d1['duplicated_lines']} of {d0['total_lines']} → {d1['total_lines']} "
  f"(the population grew {round(100 * (d1['total_lines'] - d0['total_lines']) / d0['total_lines'], 1)} %, the duplicated lines {round(100 * (d1['duplicated_lines'] - d0['duplicated_lines']) / d0['duplicated_lines'], 1)} %). "
  "Split by path, pairs / lines: " + ' · '.join(f"{k} {d0['split'][k]['pairs']} / {d0['split'][k]['lines']} → {d1['split'][k]['pairs']} / {d1['split'][k]['lines']}" for k in ('src', 'test', 'mixed'))
  + ". The ten largest fragments (`c-duplication.json`):")
P('')
pair = lambda r: (r[0].rsplit(':', 1)[0], r[1].rsplit(':', 1)[0])
base_pairs = {pair(r) for r in d0['top']}
for r in table(['first', 'second', 'lines', 'file pair in the baseline top list'], d1['top'], min(10, d1['clones']),
               lambda r: (r[0], r[1], r[2]), 'top clones'):
    P(f"| `{r[0]}` | `{r[1]}` | {r[2]} | {'standing' if pair(r) in base_pairs else 'new'} |")
P('')
pf = J('c-duplication-perfile.json')
assert pf['pairs'] == d1['clones']
P("**Suspected shape:** the growth concentrates in the driving-verb test files — by duplicated lines a file takes part in (`c-duplication-perfile.json`): "
  + ', '.join(f"`{p_}` {n_}" for p_, n_ in pf['by_file'][:8]) + f"; {pf['same_file_pairs']} of the {d1['clones']} pairs sit inside one file. "
  "The largest cross-file fragments are per-file copies of the same test scaffolding: a child-run-and-wait helper (`cli_answer.rs:134` / `cli_wait_last.rs:132`), "
  "a wrapper boot that waits for `session-start` and an `events()` reader (`cli_answer.rs:56` / `tui_wheel.rs:33` / `cli_wheel.rs:36`).")
P("**Proposal:** a direction for the founder's judgment — lift that scaffolding (the child-run helper, the booted-wrapper-until-session-start helper, the events reader) "
  "into `tests/support/`, and look at the within-file repeats of `tests/cli_verify.rs` and `src/run/send.rs` as candidates for one helper each.")
P('')
P(f"### M2 — complexity-creep · complexity.over_ceiling — +{c1['over_ceiling'] - c0['over_ceiling']}")
P(f"**Movement:** {c0['over_ceiling']} → {c1['over_ceiling']} functions over cognitive 15 ({ep(base)} → {ep(cur)}). Fires at +3 and +25 %.")
P(f"**Evidence:** cyclomatic p90 {c0['cyclomatic_p90']} → {c1['cyclomatic_p90']}, cognitive p90 {c0['cognitive_p90']} → {c1['cognitive_p90']} (unchanged), max {c0['max']['val']} → {c1['max']['val']}. The functions over the ceiling (`c-complexity.json`):")
P('')
base_fn = {(t['fn'], t['file'].rsplit(':', 1)[0]) for t in c0['top'] if t['value'] > 15}
for t in table(['function', 'site', 'cognitive', 'over the ceiling at the baseline'], [t for t in c1['top'] if t['value'] > 15],
               c1['over_ceiling'], lambda t: (t['fn'], t['file']), 'over ceiling'):
    P(f"| `{t['fn']}` | `{t['file']}` | {t['value']} | {'yes' if (t['fn'], t['file'].rsplit(':', 1)[0]) in base_fn else 'no — entered this epoch'} |")
P('')
P("**Suspected shape:** three entrants, each in a file that is also a B2 hotspot this epoch: `dialog_variants` in `ledger.rs` (hotspot 95, the file itself 884 → 2 678 code lines), `record` in `src/cmd/verify.rs` (51), `submit` in `src/bin/viola-fake-agent.rs` (160).")
P("**Proposal:** a direction — split each of the three entrants where its branches already separate; `sgr_attributes` is a test helper that stood over the ceiling at the baseline too.")
P('')
P("### M3 — monotonic · duplication.pct and complexity.over_ceiling — worse at both of the last two diffs")
P(f"**Movement:** `duplication.pct` {mono['duplication.pct']['values'][0]} → {mono['duplication.pct']['values'][1]} → {mono['duplication.pct']['values'][2]}; "
  f"`complexity.over_ceiling` {mono['complexity.over_ceiling']['values'][0]} → {mono['complexity.over_ceiling']['values'][1]} → {mono['complexity.over_ceiling']['values'][2]} "
  f"({ep(base2)} → {ep(base)} → {ep(cur)}; the chain by sha {short(base2['sha'])} → {short(base['sha'])} → {short(cur['sha'])}).")
P("**Evidence:** the closed set of six, each over the same three records: "
  + ' · '.join(f"`{k}` {' → '.join('null' if x is None else str(x) for x in v['values'])} ({'fires' if v['fires'] else 'not evaluable' if not v['evaluable'] else 'no'})" for k, v in mono.items()) + '.')
P("**Suspected shape:** both scalars are M1's and M2's own subjects; the Epoch 2b step was small (+0.06 pt, +1) and this epoch's is the large one.")
P("**Proposal:** a direction — M1 and M2 taken together in one cleanup chunk, as followed each of the three earlier boundaries (`master-route.md`: epoch-1-cleanup, epoch-2-cleanup, epoch-2b-cleanup); no separate action beyond them.")
P('')

# ---------------------------------------------------------------- informational
P('## Informational')
P('')
s0, s1 = base['sizes'], cur['sizes']
P(f"- **Sizes (no single-epoch rule in the table):** file max {s0['file_max']} → {s1['file_max']} (`{s1['top'][0][0]}`), files over 800 code lines {s0['over_800']} → {s1['over_800']}, "
  f"p90 {s0['file_p90']} → {s1['file_p90']}; totals {base['totals']['loc']} → {cur['totals']['loc']} code lines in {base['totals']['files']} → {cur['totals']['files']} files (populations, never a movement). "
  f"Neither size scalar is monotonic ({' → '.join(map(str, mono['sizes.file_max']['values']))}; {' → '.join(map(str, mono['sizes.over_800']['values']))}).")
ent = lambda new, old: [x for x in new if x not in old]
b_sz = [r[0] for r in s0['top']]
P("- **Top-N entrants, sizes:** " + ', '.join(f"`{p}` ({n})" for p, n in s1['top'] if p not in b_sz) + '.')
b_cx = {(t['fn'], t['file'].rsplit(':', 1)[0]) for t in c0['top']}
P("- **Top-N entrants, complexity:** " + ', '.join(f"`{t['fn']}` {t['value']} (`{t['file']}`)" for t in c1['top'] if (t['fn'], t['file'].rsplit(':', 1)[0]) not in b_cx) + '.')
b_hs = [r[0] for r in base['hotspots']]
P("- **Top-N entrants, hotspots:** " + ', '.join(f"`{p}` ({int(v)})" for p, v in cur['hotspots'] if p not in b_hs) + '.')
b_fi = [r[0] for r in base['graph']['fan_in_top']]
strip = lambda s: re.sub(r'^rust-analyzer cargo (\S+) \S+ ', r'\1 ', s)
P("- **Top-N entrants, fan-in:** " + ', '.join(f"`{strip(sym)}` ({n})" for sym, n in cur['graph']['fan_in_top'] if sym not in b_fi) + '.')
P(f"- **Duplication top list:** {sum(1 for r in d1['top'] if pair(r) not in base_pairs)} of the ten largest fragments are new file pairs (M1's table).")
P(f"- **Churn:** {base['churn']['pct']} % → {cur['churn']['pct']} %, files churned {base['churn']['files_churned']} → {cur['churn']['files_churned']} (no rule; 61 commits in the window).")
P(f"- **Coverage:** line {cur['coverage']['line']} % at this record (1 747 of 1 747 tests passed, one run); the baseline record holds null (its test run failed), so `coverage-drop` has no baseline to read. The last scored value is {base2['coverage']['line']} % at {ep(base2)}, named for the eye only.")
P("- **Corrections carried by this record (`corrections[]`, two schema-gap fills on the " + short(base['sha']) + " record):** `mutation.timing` — scope `unit` for its six scored units, read from its own mutation command, wall time UNKNOWN; `recipes` — the A5 classing recipe recovered from that record's evidence twin and pinned inline in this record's `recipes.dead`. "
  "Reading that recipe meant opening five summarizer scripts and two dead-code twins in the baseline's run dir (the files its `commands` name); no judgment or proposal of that run was read.")
P('')
m = cur['mutation']
surv = m['survivors']
n_missed = sum(c['missed'] for c in m['counts'].values())
P(f"**Mutation survivors at this boundary — {n_missed} on `{m['host']}`** (no threshold reads them in trend mode; the project's own boundary gate does: `.andromeda/test-plan.md:1212`). Per unit:")
P('')
for u in table(['unit', 'state', 'caught', 'missed', 'not measured', 'timeout', 'unviable', 'score', 'baseline score', 'wall s', 'cap s'],
               m['scoped_units'], 7, lambda u: u, 'units'):
    c = m['counts'].get(u)
    t = m['timing'].get(u, {})
    b = base['mutation']['scores'].get(u)
    if c:
        P(f"| {u} | {m['unit_states'][u]} | {c['caught']} | {c['missed']} | {c['not_measured']} | {c['timeout']} | {c['unviable']} | {m['scores'][u]} | {b} | {t['wall_s']} | {t['cap_s']} |")
    else:
        P(f"| {u} | {m['unit_states'][u]} | — | — | — | — | — | — | {b if b is not None else '— (baseline-test-failure there too)'} | 115 | 1800 |")
P('')
P(f"Score formula `{m['score_formula']}`; every score at scope `unit`, jobs 1. The survivors, complete:")
P('')
unit_of = lambda site: next(u for u in ('viola-agent-claude', 'viola-state', 'viola-pty', 'viola-channel', 'viola-core', 'viola-e2e') if site.startswith(f'crates/{u}/')) if site.startswith('crates/') else 'viola'
for r in table(['unit', 'site', 'mutation'], surv, n_missed, lambda r: (r[0], r[1]), 'survivors'):
    P(f"| {unit_of(r[0])} | `{r[0]}` | {cell(r[1])} |")
P('')
P("Four of the six `viola` survivors are in the fake agent (`src/bin/viola-fake-agent.rs`, a test-side bin of the root package). The twelve in `viola-state`: a `NotFound` match guard replaced by `true` at four sites (`events.rs:96`, `:149`, `stamps.rs:28`, `strict.rs:34`); "
  "six in `replace_private_with`'s retry loop (`fs.rs:270`–`275`), whose guard reads the const `HOST_IS_WINDOWS = cfg!(windows)` — false on this host, so the arm is never taken here; the recipe proves only `#[cfg]` nodes, so they stay missed by this audit's letter (the project's ruling on such a const is at `.andromeda/test-plan.md:1212`); "
  "`replace_private_shared`'s content guard (`fs.rs:290`); and `LoggedLines::next_line` (`events.rs:209`).")
P('')

# ---------------------------------------------------------------- findings outside the table
e2e = J('mut-done-viola-e2e.json')
win = J('carry-windows.json')
P('## Findings outside the threshold table')
P('')
P("### F1 — viola-e2e cannot pass its unmutated baseline under the `mutants` profile (both hosts)")
P("**Readings (three, the operator's direction):** (1) here, the baseline's one failure is `viola-e2e::harness_lifecycle boot_with_an_unknown_cli_version_is_verify_failed`, `TIMEOUT [30.003s]`, 257 of 258 passed "
  "— the profile's `package(viola-e2e)` override kills at 15 s × 2 (`.config/nextest.toml`); (2) the same test on the same tree in this audit's coverage run, profile `ci`: `PASS [34.477s]`; "
  "(3) the Windows runner's `mutants (viola-e2e)` job, run 37761947926: `TIMEOUT [30.009s]`, 249 of 250 passed, `mutants-exit-4`.")
P("**Host:** backing `tmpfs` behind the `target/e2e-home` link; hostwatch for 2026-10-08T11:00:00Z..11:02:10Z reads QUIET (io some peak 5 %, load peak 4.8). Not re-run.")
P("**Suspected shape:** the test's 34.5 s matches four waits of the gate's 8.5 s maximum — the shape the profile's own comment gives a `verify_window_` test — and is above the kill the profile gives its package (30 s).The profile's `verify_window_` override (45 s) does not reach it: the test carries no such name, and the package override sits first.")
P("**Consequence:** the unit is unscored in the ledger for the second boundary running (a different cause at Epoch 2b), and the Windows job cannot grade its 68 mutants.")
P("**Proposal:** a direction — give this test a kill above its designed wait (a name the `verify_window_` override matches, or an override of its own), the founder's choice of which.")
P('')
wv = win['jobs']['viola']
P("### F2 — the Windows workflow's `viola` job no longer fits its 120-minute ceiling")
P(f"**Readings:** run 37761947926, job `mutants (viola)`: started 10:12:36Z, cancelled 12:13:03Z (`The operation was canceled`), {wv['graded']} of {wv['found']} mutants graded. "
  "Its unmutated baseline took 123 s build + 104 s test, and a graded mutant's test phase a median 89 s (101 of the 125 reached their tests); the first dispatch (run 37174673472, 60c569b) read 88 s + 10 s and finished 131 mutants in 26 m 49 s.")
P(f"**Consequence:** {wv['found'] - wv['graded']} mutants of that job's scope have no Windows grade at this boundary, and the job has no harness document.")
P("**Proposal:** a direction — the root package's test phase on the Windows runner grew about tenfold across the epoch; either the ceiling or the job's split (per file) would bring it back inside, and the growth itself may deserve a look.")
P('')
wo = [r for r in win['rows'] if r['class'] == 'Windows-only body, missed on Windows']
sb = [r for r in win['rows'] if r['class'].startswith('shared body')]
P(f"### F3 — {len(wo) + len(sb)} Windows-side survivors that are not `cfg(unix)` twins")
P(f"**Readings:** {len(wo)} in Windows-only bodies and {len(sb)} in shared bodies (the CARRY 1 table below lists each). Among them the strict-modes entry points: "
  "`check_stamps → Ok(())`, `check_path → Ok(())` and `win::check → Ok(())` in `crates/viola-state/src/strict.rs` all read MISSED on the Windows runner, the first two caught on Linux.")
P("**Suspected shape:** no test that runs on the `windows-2025` runner fails when the Windows strict-modes check answers `Ok` unconditionally; the killing tests are Unix-side.")
P("**Proposal:** a direction — a Windows-side refusal test for the strict-modes path (and for `win::protect` / `win::dacl_of` / `console::is_console`), or an equivalence argument per site; the route entry that owns the Windows DACL check is the natural home.")
P('')

# ---------------------------------------------------------------- CARRY 1
P('## Route CARRYs answered (`working-route.md:105`)')
P('')
P('### CARRY 1 — the Windows-dispatch survivors and their `cfg(unix)` twins')
P("A fresh dispatch at this boundary, report-only and outside the ledger: `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` → run **37761947926** on `e304994`. Per job, from its own log:")
P('')
JOBN = {'pty': 113260249676, 'channel': 113260249908, 'state': 113260250051, 'agent-claude': 113260250053, 'viola': 113260249970, 'e2e': 113260250076}
for j in table(['job (id)', 'found', 'graded', 'caught', 'unviable', 'missed', 'timeout', 'harness document'], list(win['jobs']), 6, lambda j: j, 'win jobs'):
    v = win['jobs'][j]
    t = v['tally']
    doc = 'ok:true' if v['harness_ok'] else f"ok:false, {v['harness_reason']}" if v['harness_reason'] else 'ok:false' if v['harness_ok'] is False else 'none (cancelled at 120 min)'
    P(f"| mutants ({v['unit']}) ({JOBN[j]}) | {v['found']} | {v['graded']} | {t.get('caught', 0)} | {t.get('unviable', 0)} | {t.get('MISSED', 0)} | {t.get('TIMEOUT', 0)} | {doc} |")
P('')
P(f"The {win['missed_total']} MISSED, each classified by two independent readings: the pinned `cover()` recipe under the runner's own cfg set (`rustc --print cfg --target x86_64-pc-windows-msvc`), and this audit's Linux grade of the same mutant (same sha, joined on the tool's name). By class: "
  + ' · '.join(f"{k} **{v}**" for k, v in sorted(win['classes'].items(), key=lambda kv: -kv[1])) + '.')
P('')
for r in table(['job', 'site', 'mutation', 'class', 'Windows cover', 'Linux reading'], win['rows'], win['missed_total'],
               lambda r: (r['site'], r['mutation']), 'windows missed'):
    P(f"| {r['job']} | `{r['site']}` | {cell(r['mutation'])} | {r['class']} | {('`' + r['windows_cover'] + '`') if r['windows_cover'] else '—'} | {cell(r['linux'])} |")
P('')
tw = [r for r in win['rows'] if r['class'].startswith('cfg twin')]
twc = collections.Counter(r['linux'] for r in tw)
P(f"**The answer.** The twins are host-excluded, not survivors: all {len(tw)} sit in a span the Windows build removes, and on Linux they grade "
  + ', '.join(f"{v} {k}" for k, v in twc.most_common()) + " — none missed. The four `sideload_outcome` mutants (`src/cmd/run.rs:385:5`, `cfg(all(windows, not(target_arch = \"x86_64\")))`) are excluded on both measured hosts: no host this project builds on compiles that body.")
P("Against the first dispatch's 25 (run 37174673472, `evidence/windows-dispatch.md` of chunk 2026-10-04-windows-boundary-mutation-workflow), matched by mutation text since the lines moved: "
  "the 4 not measurable are the four above; of the 19 `cfg(unix)` bodies, the 18 in viola-channel (4), viola-pty (5) and `src/panic_frames.rs` (9) are in the table as twins with their Linux grades (viola-state's six `unix::reading` / `unix::euid` twins are new to that job's scope since then); "
  "the Windows-equivalent `restrict → Ok(())` (now `fs.rs:18:5`) is missed on Windows and caught on Linux, as recorded then; "
  "the two viola-e2e coordinates (`cleanup.rs` `unconnectable`'s twin and the shared-body `+ → -` on the kill deadline) have no Windows grade at this dispatch — the job died on F1 — and their Linux reading is in the report-only run below.")
P(f"**What classifying does not do.** The jobs do not turn green by it: beside the twins the run holds {len(wo) + len(sb)} survivors in code the runner does build (F3), one job over its ceiling (F2) and one baseline failure (F1). "
  "A direction, not a verdict: the harness's Windows arm could subtract a mutant whose covering predicate is false on its host before it counts `missed` — the recipe is `cover.py` in this run dir — so the verdict row reads over the measurable set.")
P('')

# ---------------------------------------------------------------- CARRY 2 + the report-only run
ed = J('carry-e2e-deselected.json')
dn = J('mut-done-viola-e2e-deselected.json')
copies = {}
for u in ('viola-core', 'viola-pty', 'viola-channel', 'viola-state', 'viola-agent-claude', 'viola-e2e', 'viola', 'viola-e2e-deselected'):
    dd = J(f'mut-done-{u}.json')
    for k, v in dd['carry']['copies'].items():
        copies[(u, k)] = v
assert len(copies) == 8 and all(v['islink'] and v['readlink'] == '/tmp/viola-e2e-home-1000' for v in copies.values()), copies
P('### CARRY 2 — what a mutation run\'s copied tree does with the `target/e2e-home` link')
P(f"**The link is carried as a link.** In all {len(copies)} invocations of this audit (seven units and the report-only run) cargo-mutants 27.1.0's copy under `TMPDIR` held `target/e2e-home` as a symlink (`lstat` mode `0o120777`) to `/tmp/viola-e2e-home-1000` — never a plain directory, never a copy of the homes. "
  "So a mutation run's test homes land on the same owner-only tmpfs as the working tree's, outside the copy; the copy's removal does not remove them.")
hm = [J(f'mut-done-{u}.json')['carry'] for u in ('viola-core', 'viola-pty', 'viola-channel', 'viola-state', 'viola-agent-claude', 'viola', 'viola-e2e-deselected')]
P(f"**What the tmpfs saw.** Entries under `/tmp/viola-e2e-home-1000` sampled every 15 s: between {min(h['homes_min'] for h in hm if h['homes_min'] is not None)} and {max(h['homes_max'] for h in hm if h['homes_max'] is not None)} across the tier; "
  f"`/tmp` used between {round(min(h['tmp_used_min'] for h in hm if h['tmp_used_min']) / 2**30, 1)} and {round(max(h['tmp_used_max'] for h in hm if h['tmp_used_max']) / 2**30, 1)} GiB of 32. No quota error in any unit's log.")
keeper = [(o, s) for o, s in ed['keeper']]
P(f"**The keeper's mutants** (`Workspace::ensure_e2e_home`, first mutated here; from the report-only run, {len(keeper)} mutants): "
  + '; '.join(f"`{n.split(': ', 1)[0].rsplit('/', 1)[1]}` {cell(n.split(': ', 1)[1])} — {g}" for n, g in keeper) + '.')
bk = dn['carry']['backing']
if bk:
    P("**`backing/`.** " + ' '.join(f"`{k.split('/', 1)[1]}` appeared in the copied tree at {v['seen']} (mode `{v['mode']}`, {v['entries']} entries; the tool's last line then: `{cell(v['log_line'])[:160]}`)." for k, v in bk.items())
      + " It is the relative-target mutant's (`mod.rs:102:20`, `delete !`): the directory's own mtime reads 15:45:03Z, between the last write of the `:94:9` mutant's log (15:45:01Z) and of the `:102:20` mutant's (15:45:08Z) — "
      "with the check inverted, the keeper's refusal test hands it the relative target `backing` and the keeper makes it under the test's working directory, as chunk 2026-10-07-test-homes-off-the-contended-volume measured by hand (`evidence/keeper-control.md`, pair 3). "
      "It was still standing, empty, two minutes later, so every later mutant of that invocation ran with it present; it sat inside the copy only — nothing named `backing` is in the repository, and "
      + ("the copy went when the invocation was interrupted." if not J('scratch-residue.json')['copies'] else "the interrupted invocation left its copy in the scratch (named under Skips), `backing` inside it."))
    dr = J('carry-copy-droppings.json')
    made = [x['path'].rsplit('/', 1)[1] for x in dr['untracked_in_crates_viola_e2e'] if x['mtime'] > '2026-10-08T15:39:59Z' and not x['path'].endswith('/backing')]
    P(f"**Beside it.** `backing/` is not the only thing a mutated harness leaves at its tests' working directory: at {dr['read']} the copy's `crates/viola-e2e/` also held "
      + ', '.join(f'`{x}`' for x in made) + f" ({len(made)} entries made after the copy, `carry-copy-droppings.json`) — all inside the copy. "
      "And the copy carries the repository's untracked, gitignored entries with it: the eleven `.viola-verify-*` probe dirs at the root and `viola-0.2.0-incubator/`.")
    P("**One thing in the repository itself, not made by this run:** `crates/viola-e2e/.viola-verify-2676638-plan/` (mode 0700, dated 2026-10-05T13:18Z, holding one empty `plans/` dir) — a killed verify's probe dir under a member's directory, where the root-anchored `/.viola-verify-*/` ignore does not reach and `git status` shows nothing because it holds no file. Left where it is.")
else:
    P("**`backing/`.** No directory named `backing` was seen in the copied tree at any 15 s sample of the report-only run (`backing`, `crates/viola-e2e/backing`, `target/backing`, `src/backing`, `tests/backing`), and none is in the repository or the scratch after it. "
      "A directory made and gone inside one sample interval would not be seen: the reading is \"none standing\", not \"never made\".")
P('')
ec = ed['counts']
assert not ed['complete'] and ed['report_only']
P('### The report-only viola-e2e run (the operator\'s answers; never a ledger score)')
P(f"`cargo mutants --package viola-e2e … -- -E 'not test(=boot_with_an_unknown_cli_version_is_verify_failed)'`, the same form otherwise; baseline {ed['baseline']['summary']} (257 passed, 1 skipped). "
  f"Stopped on the operator's second answer once `harness/mod.rs` and `harness/cleanup.rs` were graded: **{ed['tested']} of {ed['listed']}** tested in {ed['wall_s']} s "
  f"(about 12 s a mutant; the whole unit would have taken some 2.5 h) — a prefix, not the unit. Of the tested: "
  f"{ec['caught']} caught · {ec['missed']} missed · {ec['not_measured']} not measured · {ec['timeout']} timeout · {ec['unviable']} unviable. "
  f"Files graded whole: {', '.join('`' + p.rsplit('/', 1)[1] + '`' for p in ed['files_whole'])}"
  + (f"; in part: {', '.join('`' + p.rsplit('/', 1)[1] + '`' for p in ed['files_partial'])}" if ed['files_partial'] else '') + '. '
  + ("A mutant only the deselected test kills reads missed here. The missed of the tested prefix:" if ec['missed'] else "No mutant of the tested prefix read missed."))
P('')
if ec['missed']:
    for r in table(['site', 'mutation'], ed['survivors'], ec['missed'], lambda r: (r[0], r[1]), 'e2e survivors'):
        P(f"| `{r[0]}` | {cell(r[1])} |")
    P('')
if ed['timeouts']:
    P("Timeouts: " + '; '.join(f"`{a}` {cell(b)}" for a, b in ed['timeouts']) + '.')
    P('')
P("CARRY 1's two viola-e2e coordinates on Linux, from this run: " + '; '.join(f"`{n.split(': ', 1)[0]}` {cell(n.split(': ', 1)[1])} — **{g}**" for n, g in ed['carry1']) + '.')
P("The project's own ruling on `harness/run/mutants/scratch.rs` `prepare` (behind `HOST_SCRATCH = cfg!(windows)`, a const the recipe cannot prove false, so it stays `missed` by this audit's letter) is registered at `.andromeda/test-plan.md:1212`.")
P('')

# ---------------------------------------------------------------- below threshold
P('## Below threshold — no action')
P('')
for u, s0_, s1_, dlt in mut_rows:
    if dlt is not None:
        P(f"- mutation-drop · {u}: {s0_} → {s1_} ({'+' if dlt >= 0 else ''}{dlt} pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ ({base['mutation']['host']} → {m['host']}).")
    else:
        P(f"- mutation-drop · {u}: no score at either record (`baseline-test-failure` both times) — passed over, never a zero.")
P(f"- dead-growth: zero-ref candidates {base['dead']['zero_ref_candidates']} → {cur['dead']['zero_ref_candidates']} (raw 1 277 → 42 after the tests filter → 5 read by hand, all false-positive families; `c-dead.json`). Unused deps unchanged: the fuzz workspace's `arbitrary`.")
P(f"- new-cycle: {base['graph']['cycles']} → {cur['graph']['cycles']}; cross-unit edges {base['graph']['cross_unit_edges']} → {cur['graph']['cross_unit_edges']}; fan-out unchanged.")
P("- coverage-drop: not evaluable — the baseline's `coverage.line` is null (skip `baseline-test-failure` there).")
P("- monotonic, the other four: `dead.zero_ref_candidates` flat; `sizes.file_max` and `sizes.over_800` improved at the previous diff; `coverage.line` not evaluable this run (null at the baseline record, its skip `baseline-test-failure`).")
P("- count-under-ratio: not applicable — clones and duplicated lines rose and their ratio rose with them (M1).")
P(f"- complexity percentiles: cyclomatic p50 {c0['cyclomatic_p50']} → {c1['cyclomatic_p50']}, p90 {c0['cyclomatic_p90']} → {c1['cyclomatic_p90']}; cognitive p50 {c0['cognitive_p50']} → {c1['cognitive_p50']}, p90 {c0['cognitive_p90']} → {c1['cognitive_p90']}.")
P('')

# ---------------------------------------------------------------- skips
P('## Skips')
P('')
for s in table(['metric', 'reason', 'note'], cur['skips'], len(J('record.json')['skips']), lambda s: s['metric'], 'skips'):
    P(f"| {s['metric']} | {s['reason']} | {cell(s['note'])} |")
P('')
P("Left for the operator's desk, outside this run's write surface: `~/dev/projects/viola-mutants-scratch/e3/` (made for this run, NOCOW) holds what the mutated tests left under `TMPDIR` — "
  f"{J('scratch-residue.json')['entries']} entries, {J('scratch-residue.json')['human']} — and no cargo-mutants copy. Nothing in it is read by any chunk.")
P('')
P("A note on this record's `commands.mutation`: it names that scratch by the host's absolute path, the one path in the record that is not repo-relative. A relative `TMPDIR` resolves differently inside each copied tree, so the as-run form was kept and the command says so itself; whether the ledger should carry `<repo parent>/…` there instead is the founder's to rule, and a ruling would ride the next record's `corrections[]`.")
P('')

# ---------------------------------------------------------------- appendix
nm = m['not_measured']
n_nm = sum(c['not_measured'] for c in m['counts'].values())
P(f"## Appendix — not measured on this host (`{m['host']}`): {n_nm}")
P('')
P("Each a `missed` outcome whose whole span a covering predicate removes from this host's build (the pinned recipe, `cover.py`); never a survivor, never scored. "
  "The project's own union verdict is registered at `.andromeda/test-plan.md:1212` (the Windows leg `windows-mutants.yml` measures these); it is named here, not merged.")
P('')
for r in table(['site', 'mutation', 'predicate'], nm, n_nm, lambda r: (r[0], r[1]), 'not measured'):
    P(f"| `{r[0]}` | {cell(r[1])} | `{r[2]}` |")
P('')
open(f'{R}/proposals.md', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
informational = 10
print(f'proposals.md written: {n_prop} proposals, {informational} informational items, 3 findings outside the table, {len(cur["skips"])} skips, {len(recs)} records ({bad} unparseable)')
