#!/usr/bin/env python3
"""P5: render proposals.md from the appended record (re-found by ts+sha), its baseline and the baseline's own baseline
(by sha), judge.json / judge-mutation.json and the c-*.json twins; every evidence table's row count asserted against an
independent n and unique on its natural key. usage: proposals.py <run_dir>"""
import json, sys

R = sys.argv[1]
J = lambda n: json.load(open(f'{R}/{n}'))
rec0 = J('record.json')
recs = []
for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try: recs.append(json.loads(l))
    except ValueError: pass
N_RECS = len(recs)
me_i = max(i for i, r in enumerate(recs) if r['ts'] == rec0['ts'] and r['sha'] == rec0['sha'])
rec = recs[me_i]
base = [r for r in recs[:me_i] if r['sha'] == rec['baseline_sha']][-1]
prev = [r for r in recs[:me_i] if r['sha'] == base['baseline_sha']][-1]
short = lambda e: e.split(' — ')[0]
E, B, P = short(rec['epoch']), short(base['epoch']), short(prev['epoch'])
jd, jm = J('judge.json'), J('judge-mutation.json')
over = J('c-sizes-over800.json')
win = J('c-windows-mutants.json')
cx = J('c-complexity.json')
host = rec['mutation']['host']
out, n_prop, n_info = [], 0, 0


def table(head, rows, n, key):
    assert len(rows) == n, (head, len(rows), n)
    keys = [key(r) for r in rows]
    assert len(set(keys)) == len(keys), (head, 'duplicate key')
    out.append('| ' + ' | '.join(head) + ' |')
    out.append('|' + '---|' * len(head))
    for r in rows:
        out.append('| ' + ' | '.join(str(c) for c in r) + ' |')
    out.append('')


ho = rec['head_overshoot']
out += [f"# Code Audit — viola · {rec['epoch']} · {rec['ts']}",
        f"mode {rec['mode']} · HEAD {rec['sha'][:10]} · baseline {rec['baseline_sha'][:10]} ({base['epoch']}) · span {rec['span']}",
        f"overshoot: this run {ho['commits']} commits ({ho['note']}); the baseline's {base['head_overshoot']['commits']} commits",
        "trend-breaks: none — every tool's leading version token equals the baseline's "
        f"({', '.join(f'{k} {v}' for k, v in rec['tool_versions'].items())})",
        f"ledger: {N_RECS} parseable records after this run's append", '']

# ---------------- Proposals
out += ['## Proposals', '']
mono = jd['mono']
a, b, c, fire = mono['sizes.file_max']
assert fire
n_prop += 1
out += [f'### M{n_prop} — monotonic · sizes.file_max — +{c - b} lines',
        f'**Movement:** {a} → {b} → {c} ({P} → {B} → {E}); worse at both diffs.',
        '**Evidence:** the maximum is `crates/viola-agent-claude/src/ledger.rs` at all three boundaries (tokei code lines). '
        'Raw lines 3069 → 4230 (+1161) since the baseline; its `#[cfg(test)]` module starts at line 1205 at the baseline and '
        'line 1514 at HEAD, so the module above it grew 1204 → 1513 raw lines (+309) and the test module 1865 → 2717 (+852). '
        '`git log --numstat` over the window: +1207 / −46 across 6 commits (c-churn: 6 commits for the file). '
        f"Its max per-function cognitive is {cx['per_file_max_cognitive']['crates/viola-agent-claude/src/ledger.rs']:g} (under the ceiling of 15).",
        '**Suspected shape:** three quarters of the growth is the in-file test module: each capability-ledger row this '
        'epoch added (the resume, agents-join and compact rows among them) brought its probe tests into the same file.',
        '**Proposal:** move the `#[cfg(test)]` module of `ledger.rs` into its own file (`ledger/tests.rs`), or split the '
        'row definitions by row family, so the file stops being the size maximum by test mass.', '']

a, b, c, fire = mono['sizes.over_800']
assert fire
n_prop += 1
rows = over['over_800_head']
out += [f'### M{n_prop} — monotonic · sizes.over_800 — +{c - b} files',
        f'**Movement:** {a} → {b} → {c} ({P} → {B} → {E}); worse at both diffs.',
        f'**Evidence:** the {c} files over 800 code lines at HEAD (tokei code lines; `—` = under 800 or absent at the baseline):', '']
table(['file', f'{B} lines', f'{E} lines'],
      [[f'`{f}`', bl if bl and bl > 800 else (f'— ({bl})' if bl else '— (new)'), hl] for f, bl, hl in rows],
      rec['sizes']['over_800'], lambda r: r[0])
entr = [r for r in rows if not (r[1] and r[1] > 800)]
left = over['left_800']
out += [f"Entrants ({len(entr)}): " + '; '.join(f'`{f}` {bl or "new"} → {hl}' for f, bl, hl in entr) + '.',
        f"Left ({len(left)}): " + '; '.join(f'`{f}` (now {n})' for f, n in left) + ' — net +3.',
        '**Suspected shape:** the entrants are the epoch\'s own surfaces — the typed verify probes (`src/cmd/verify/typed.rs`), '
        'the board read model and its human view (`crates/viola-state/src/board.rs`, `src/human/board.rs`), the harness boot '
        'and the test home helper; `src/run/send.rs` left by being split into `src/run/send/*`.',
        '**Proposal:** the same split that took `src/run/send.rs` under 800 (a module directory by concern) for '
        '`src/cmd/verify/typed.rs` (1199) and `src/human/board.rs` (949, new this epoch) first: they are the two largest '
        'entrants, and `typed.rs` is the second hotspot (90).', '']

assert not any(m[1] == 'drop' for m in jm)
out += ['No other check fired: no new cycle, no duplication-up, no complexity-creep, no dead-growth, no coverage-drop, '
        'no mutation-drop at equal scope.', '']

# ---------------- Informational
out += ['## Informational', '']


def info(s):
    global n_info
    n_info += 1
    out.append(f'- {s}')


# mutation: per unit
info('**Mutation, per unit** (`caught/(caught+missed)`, jobs 2, the nextest `mutants` profile; each score against its last '
     'scored value at the same scope — none dropped ≥ 10 pts):')
mrows = []
for m in jm:
    u = m[0]
    cn = rec['mutation']['counts'].get(u)
    if m[1] == 'unscored':
        mrows.append([u, '—', rec['mutation']['unit_states'][u], 'baseline-test-failure (see below)', '—'])
    elif m[1] == 'first':
        mrows.append([u, m[2], f"unit · {cn['mutants']} mutants", 'first score', f"{cn['caught']} / {cn['missed']} / {cn['not_measured']} / {cn['timeout']} / {cn['unviable']}"])
    else:
        d = round(m[2] - m[4], 2)
        mrows.append([u, m[2], f"unit · {cn['mutants']} mutants", f"{m[4]} ({short(m[6])}, unit) · {'+' if d >= 0 else ''}{d}",
                      f"{cn['caught']} / {cn['missed']} / {cn['not_measured']} / {cn['timeout']} / {cn['unviable']}"])
out.append('')
table(['unit', 'score', 'state · scope', 'last scored · delta', 'caught / missed / not measured / timeout / unviable'],
      mrows, len(rec['mutation']['scoped_units']), lambda r: r[0])

# survivors, full
surv = rec['mutation']['survivors']
by = {}
for s in surv:
    f = s[0]
    u = 'viola-e2e' if f.startswith('crates/viola-e2e/') else next(
        (x for x in ('viola-agent-claude', 'viola-state', 'viola-mcp', 'viola-pty', 'viola-channel', 'viola-core') if f.startswith(f'crates/{x}/')), 'viola')
    by.setdefault(u, []).append(s)
info(f"**Survivors, complete** ({len(surv)} = Σ counts.missed; the run at this boundary):")
out.append('')
n_missed = sum(v['missed'] for v in rec['mutation']['counts'].values())
table(['unit', 'site', 'mutation'], [[u, f'`{s}`', f'`{t}`'] for u in sorted(by) for s, t in by[u]], n_missed,
      lambda r: (r[1], r[2]))
for u in sorted(by):
    assert len(by[u]) == rec['mutation']['counts'][u]['missed'], u
out += ['Readings: `viola-state`\'s 10 baseline survivors (the `fs.rs` retry loop and `board.rs`) are all killed now; its two '
        'are new code (`liveness.rs` `live_endpoint`, the Epoch 5 liveness pre-check). `viola-mcp`\'s three (`serve`, '
        '`get_info`, `list_tools`) are most likely killed only by `crates/viola-e2e/tests/mcp_tools.rs`, outside the package '
        'run (an inference, not run). `viola-e2e`\'s eight sit in its harness: the mutants scratch `prepare`, '
        '`HarnessSession`\'s `Drop` and `read_list`\'s offset.', '']
nm = rec['mutation']['not_measured']
n_nm = sum(v['not_measured'] for v in rec['mutation']['counts'].values())
info(f'**Not measured on this host ({host})** ({len(nm)} = Σ counts.not_measured): missed mutants whose whole span the '
     'host\'s build excludes, by the pinned `cover()` reading of `rustc --print cfg` — one row each, then the union.')
out.append('')
table(['site', 'mutation', 'predicate'], [[f'`{s}`', f'`{t}`', f'`{p}`'] for s, t, p in nm], n_nm, lambda r: (r[0], r[1]))
preds = sorted({p for _, _, p in nm})
out += [f'Union: {len(nm)} rows under {len(preds)} predicates (' + ', '.join(f'`{p}`' for p in preds) +
        ') — their grading is the Windows / macOS legs\' (`windows-mutants.yml`).', '']

info('**Root unit `viola` — not scored, `baseline-test-failure`.** The founder\'s scope was CARRY 2\'s two files '
     '(`src/bin/viola-fake-agent.rs`, `src/cmd/verify.rs`: 265 of 1543 listed). Two attempts, both cargo-mutants exit 4 at '
     'the UNMUTATED baseline under the `mutants` profile\'s 10 s kill: 21:50Z on a host at load ~67 (another project\'s sweep: '
     'TIMEOUTs across `cli_revive` / `cli_send` / `cli_verify`, 4 `send_window_*` "wrapper builder not ready"), and 22:51Z '
     'at load ~4: 3 TIMEOUTs at 10.00 s — `contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set`, '
     '`cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` and `..._on_a_verified_cli::case_1_hint`. '
     'In this run\'s green coverage pass (`ci` profile, no kill) the same tests take **10.577 s**, 9.010 s and 9.346 s: '
     'the first cannot pass a 10 s kill on this host at all, the two within a second of it. The `cli_revive` tests take '
     '5.2–6.2 s each (the Epoch 5 preflight\'s bounded agents read). So the root package\'s baseline does not fit the '
     'profile at HEAD — structural, not load. The last scored root value stays E4\'s 97.03 (unit scope); no comparison is made. '
     'A change-direction for the founder: a `mutants`-profile override for these tests (as `verify_window_` / `send_window_` '
     'have), or a faster replay — before CARRY 2 can be graded by a tool run.')
info('**CARRY 2** (`viola-0.1.0/chunks/2026-10-13-paste-newline-ledger-row/evidence/carry2-kills.md`, its four sites c1–c4) — '
     '**still owed**: the root unit above never reached a mutant. Its kills stand on that chunk\'s hand-applied mutants only.')

st = next(j for j in win['jobs'] if j['job'] == 'mutants (viola-state)')
c3 = st['carry3']
assert len(c3) == 3 and all(x[2] == 'caught' for x in c3)
info('**CARRY 3 — graded: all three `viola-state` retry-loop mutants CAUGHT on windows-2025** (dispatch '
     f"[{win['run']}]({win['url']}), HEAD `{win['head_sha'][:10]}`): " +
     '; '.join(f'`{s}` `{t}` — {o} after a {sec} s test' for s, t, o, sec in c3) +
     '. Each test run ends at 11 s with a failing test, not at the kill (a timeout reads `TIMEOUT`). '
     f"The job's {len(st['missed'])} misses (`fs.rs:19` `restrict`, `strict.rs` `unix::reading` ×4 and `unix::euid` ×2) all "
     'sit under `#[cfg(unix)]` — not measurable on Windows.')
wrows = []
for j in win['jobs']:
    why = j['tally'] or ''
    if j['baseline'] and j['baseline'].startswith('FAILED'):
        why = 'baseline FAILED: ' + ('; '.join(x.split(') ', 1)[-1] if ') ' in x else x for x in j['baseline_failures'][:2])
                                     or ' '.join(j['assertion']))
    elif j['harness_verdict'] and '"ok":false' in j['harness_verdict']:
        why += ' — harness verdict `unviable-exceeds-caught`'
    wrows.append([j['job'], j['conclusion'], why.replace('|', '/')])
info(f"**The `windows-mutants.yml` dispatch after `c6e1301`** (the root package's Windows baseline) (the founder's \"Dispatch now\"; run {win['run']}, "
     f"{win['conclusion']}): 4 of 10 jobs green.")
out.append('')
table(['job', 'conclusion', 'reading'], wrows, len(win['jobs']), lambda r: r[0])
out += ['The three root-package jobs that time out at the baseline time out on the same test as this host\'s second root attempt '
        '(`fake_agent_dialog_replay_matches_every_recorded_dialog_set`): one cause on both OSes. `viola-e2e`\'s Windows '
        'baseline fails `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` '
        '(left `Ok(())`, right `Err(Writable)`) — a Windows-only assertion; `viola-cmd-verify-stop`\'s on '
        '`cli_budget path6_the_boundaries_and_the_window_word` at 10 s.', '']
info('**`verify_window_without_screens_fails_every_interactive_row` against the 45 s kill:** 42.952 s alone on an unloaded '
     'host under the `mutants` profile (a one-test nextest run, this audit), and 43.635 s inside the root unit\'s second baseline — '
     '2.0 s and 1.4 s of headroom.')
info('**The Epoch 5 chunks\' code** (end-to-end, machine-contract, output-discipline, mcp, plugin, compact, dialog-hold, the '
     'switch): graded at unit scope by every library unit above (viola-core, -pty, -channel, -state, -agent-claude, -mcp) and '
     'viola-e2e; the part in the root package (`src/`) is not graded this boundary (the root unit\'s baseline above).')
info('**First score `viola-mcp`: 92.68** (76 mutants, 35 unviable); `viola-channel` 100 against its last scored value, '
     'E3\'s 100 (it was not run at E4).')

# top-N entrants
def entrants(name, cur, old, key):
    o = {key(x) for x in old}
    return [x for x in cur if key(x) not in o]


en_sz = entrants('sizes.top', rec['sizes']['top'], base['sizes']['top'], lambda x: x[0])
info('**Top-N entrants — sizes.top (10):** ' + '; '.join(f'`{f}` {n}' for f, n in en_sz) + '.')
en_cx = entrants('complexity.top', rec['complexity']['top'], base['complexity']['top'], lambda x: (x['fn'], x['file'].rsplit(':', 1)[0]))
info('**Top-N entrants — complexity.top (10):** ' + '; '.join(f"`{x['fn']}` `{x['file']}` {x['value']:g}" for x in en_cx) + '.')
en_dp = entrants('duplication.top', rec['duplication']['top'], base['duplication']['top'], lambda x: tuple(x))
info(f'**Top-N entrants — duplication.top (10): {len(en_dp)}** — ' + '; '.join(f'`{a}` ↔ `{b}` {n} L' for a, b, n in en_dp) + '.')
en_ht = entrants('hotspots', rec['hotspots'], base['hotspots'], lambda x: x[0])
info('**Top-N entrants — hotspots (10):** ' + '; '.join(f'`{f}` {v:g}' for f, v in en_ht) + '.')
en_fi = entrants('fan_in_top', rec['graph']['fan_in_top'], base['graph']['fan_in_top'], lambda x: x[0])
info('**Top-N entrants — graph.fan_in_top (20):** ' + ('; '.join(f'`{s}` {n}' for s, n in en_fi) or 'none') + '.')
info('**Unused dependency, new:** `viola-mcp` → `serde` (cargo-machete; `serde` derives may arrive through `schemars` / '
     '`rmcp` re-exports — a candidate, not a verdict). `fuzz` → `arbitrary` stands from E4.')
info('**Coverage run:** 2958/2958 tests passed in 66.6 s; no `.profraw` refused or truncated — this host\'s reading for the '
     'open `pre-push` WATCH on "Newer claude CLI rows" (`evidence/profraw-red.md`), one clean run.')

# ---------------- Below threshold
out += ['', '## Below threshold — no action', '']
bl = []
d_pct, d_rel, _ = jd['dup']
bd, rd = base['duplication'], rec['duplication']
bl.append(f"duplication-up: `duplication.pct` {bd['pct']} → {rd['pct']} (+{d_pct:.2f} pt, +{d_rel:.1f} %) — under the +0.5 pt AND +15 % floor.")
bl.append(f"count-under-ratio: not met — `clones` {bd['clones']} → {rd['clones']} and `duplicated_lines` {bd['duplicated_lines']} → "
          f"{rd['duplicated_lines']} rose with `pct`; population `total_lines` {bd['total_lines']} → {rd['total_lines']} "
          f"(+{(rd['total_lines'] / bd['total_lines'] - 1) * 100:.1f} %); split src {rd['split']['src']['pairs']} · test "
          f"{rd['split']['test']['pairs']} · mixed {rd['split']['mixed']['pairs']} pairs.")
oc_b, oc_c, _ = jd['cx']
bl.append(f'complexity-creep: `over_ceiling` {oc_b} → {oc_c} (+{oc_c - oc_b}) — under +3; the four over 15: ' +
          '; '.join(f"`{x['fn']}` `{x['file']}` {x['value']:g}" for x in rec['complexity']['top'] if x['value'] > 15) + '.')
bl.append(f"dead-growth: `zero_ref_candidates` {base['dead']['zero_ref_candidates']} → {rec['dead']['zero_ref_candidates']} "
          '(13 residuals hand-read, all false positives — c-dead.json).')
bl.append(f"coverage-drop: line {base['coverage']['line']} → {rec['coverage']['line']}.")
bl.append(f"new-cycle: `graph.cycles` {base['graph']['cycles']} → {rec['graph']['cycles']}.")
for p in ('duplication.pct', 'complexity.over_ceiling', 'dead.zero_ref_candidates', 'coverage.line'):
    a, b, c, f = mono[p]
    assert not f
    bl.append(f'monotonic `{p}`: {a} → {b} → {c} ({P} → {B} → {E}) — not worse at both diffs.')
for m in jm:
    if m[1] == 'below':
        bl.append(f'mutation `{m[0]}`: {m[4]} → {m[2]} (unit, {short(m[6])} → {E}).')
bl.append(f"sizes: p50 {base['sizes']['file_p50']} → {rec['sizes']['file_p50']}, p90 {base['sizes']['file_p90']} → "
          f"{rec['sizes']['file_p90']}; totals {base['totals']['loc']} → {rec['totals']['loc']} loc, "
          f"{base['totals']['files']} → {rec['totals']['files']} files (populations, never movements).")
bl.append(f"complexity percentiles: cyclomatic p50/p90 {base['complexity']['cyclomatic_p50']}/{base['complexity']['cyclomatic_p90']} → "
          f"{rec['complexity']['cyclomatic_p50']}/{rec['complexity']['cyclomatic_p90']}, cognitive "
          f"{base['complexity']['cognitive_p50']}/{base['complexity']['cognitive_p90']} → {rec['complexity']['cognitive_p50']}/"
          f"{rec['complexity']['cognitive_p90']}.")
bl.append(f"churn (no threshold): {base['churn']['pct']} → {rec['churn']['pct']} %, files churned "
          f"{base['churn']['files_churned']} → {rec['churn']['files_churned']}.")
bl.append(f"graph (no threshold): cross-unit edges {base['graph']['cross_unit_edges']} → {rec['graph']['cross_unit_edges']}; "
          f"fan-out {dict(map(tuple, base['graph']['fan_out']))} → {dict(map(tuple, rec['graph']['fan_out']))}.")
out += [f'- {x}' for x in bl]

# ---------------- Skips
out += ['', '## Skips', '']
for s in rec['skips']:
    out.append(f"- `{s['metric']}` — {s['reason']}" + (f": {s['note']}" if s.get('note') else ''))
out.append('')
open(f'{R}/proposals.md', 'w', encoding='utf-8').write('\n'.join(out))
print(f"P={n_prop} I={n_info} B={len(bl)} S={len(rec['skips'])}")
