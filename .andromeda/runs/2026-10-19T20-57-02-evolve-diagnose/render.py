"""Render proposals.md from the q-*.json twins plus the prose below; writes q-level.json and q-untyped-clusters.json.

render.py <run_dir>   (run from the repository root)
"""
import json
import sys

RUN = sys.argv[1]
TITLE = '# Evolve Diagnosis — viola-0.1.0 · Epoch 5 — Driver surface · 2026-10-19T20:57:02Z'


def load(n):
    return json.load(open(f'{RUN}/{n}', encoding='utf-8'))


typed = {g['key']: g for g in load('q-typed.json')}
health = load('q-health.json')
retr = load('q-retractions.json')
chains = load('q-chains.json')
led = {}
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        continue
    if r.get('id'):
        led.setdefault(r['id'], []).append(r)
facts_by_id = {}
for side in ('target', 'previous'):
    for x in load('q-level-facts.json')[side]:
        facts_by_id.setdefault(x['id'], []).append(x)


def cell(s):
    return str(s if s not in (None, '') else '—').replace('|', '\\|').replace('\n', ' ')


def imp(i):
    return ', '.join(f'{k} {v}' for k, v in (i or {}).items() if v) or '—'


def friction_rows(ids):
    out = ['| chunk | skill/step | type | what | impact | evidence |', '|---|---|---|---|---|---|']
    for i in ids:
        for r in [x for x in led[i] if x.get('kind') == 'friction']:
            sk = (r.get('skill') or '').replace('andromeda-', '')
            out.append(f"| {cell(r.get('chunk'))} | {cell(sk + '/' + str(r.get('step')))} | {cell(r.get('type') or 'untyped')} | "
                       f"{cell(r.get('what'))} | {cell(imp(r.get('impact')))} | {cell(r.get('evidence'))} · `{i}` |")
    return '\n'.join(out)


def fact_rows(keys):
    """keys: (step-record id, substring of the fact's note) pairs."""
    out = ['| chunk | skill/step | nature/solution | note | record |', '|---|---|---|---|---|']
    for i, sub in keys:
        hit = [x for x in facts_by_id[i] if sub in (x.get('note') or '')]
        assert len(hit) == 1, (i, sub, len(hit))
        x = hit[0]
        out.append(f"| {cell(x['chunk'])} | {cell(x['skill'] + '/' + str(x['step']))} | {x['nature']}/{x['solution']} | {cell(x['note'])} | `{i}` |")
    return '\n'.join(out)


def typed_section(n, key, pattern, proposal):
    g = typed[key]
    skill_step = key.rsplit('/', 1)[0]
    typ = key.rsplit('/', 1)[1]
    label = 'any step (universal / recall: grouped by type alone)' if skill_step == '*/*' else skill_step
    rows = ['| chunk | what | impact | evidence |', '|---|---|---|---|']
    for c in g['cases']:
        w = c['what'] + (f" [clause retracted: {c['clause_retracted']}]" if c['clause_retracted'] else '')
        where = '' if skill_step != '*/*' else f" ({c['skill']}/{c['step']})"
        rows.append(f"| {cell(c['chunk'])}{where} | {cell(w)} | {cell(imp(c['impact']))} | {cell(c['evidence'])} · `{c['id']}` |")
    halt = f" · {g['with_halt']} with halt" if g['with_halt'] else ''
    rate = f" · rate {g['rate']} step runs" if g['rate'] else ''
    return (f"### P{n} — {label} · `{typ}` — {g['n']} cases · weight {g['weight']}{halt}{rate}\n"
            f"**Pattern:** {pattern}\n\n**Evidence:** ALL {g['n']} cases —\n\n" + '\n'.join(rows) +
            f"\n\n**Proposal:** {proposal}\n")


# ---------------------------------------------------------------- typed proposals (prose)
P = [
    ('phase/validate/contract.mechanical-check',
     "Validate's mechanical checks caught 23 plan or scope defects in 15 of 18 chunks before the review card. Ten of the 23 are one shape: check 10 (`inputs.py verify`) read `UNPARSED` because the take-up fold copied route CARRY freight that cites another chunk's snapshot as a bare `inputs#I<n>`, which reads as this chunk's own (interrupted-verify, paste-newline, owed-ledger, remove-the-browser-leg, unwrapped, end-to-end, cli-machine, cli-output, mcp, optional-plugin). Five more are gate atoms authored at P4 that could never pass or never fail on a right tree (the doctest suite object's `passed 0`, a word the right implementation also writes, an absent path that makes tokei exit, an unpinned advisory database read as seen).",
     "The check works; the repair is repeated. For the ten citation repairs the cause sits upstream of validate: the CARRY text on the route (written by wrap's route-resolve) carries a chunk-relative citation into a chunk where it means something else. One direction is that the route writer spells a carried citation by its chunk-qualified form (the `inputs/` file path), or that phase take-up's fold re-spells bare `inputs#I` ids on copy, which would remove the ten repairs at their source. For the five atom repairs, one direction is a P4 authoring note in the plan template on what `run --unit` always prints (the doctest suite object) and on the word a right tree also writes."),
    ('wrap-session/reconcile/contract.in-pass-correction',
     "Reconcile corrected 12 of its own first writes before the commit in 11 wraps; every one was caught in-pass, by a re-read or a recount, or by a tool (`registry.py check`, `sidecar.py check`, `classify.py check`, the step-2 sweep). Five of the 12 are a hand-typed tally or count that did not match its rows (`two` → three, 28 → 29 dispositions, 44 → 47 amendments, 5 → 4 escalations, `no change 12` → 11).",
     "The catch is working; the cost is the first write. One direction is that the tallies reconcile writes into `fanout-results.md` / `cascade-dispositions.md` / the answer card are printed by the tool that holds the rows (a count line from `cascade.py` or `sidecar.py`) rather than typed, which would take the five tally cases off the first write. The other seven are wording placements a re-read caught; no common cause is evident there."),
    ('*/*/recall.corpus-recurrence',
     "A curated rule stood correct and the same failure recurred 11 times in 8 chunks. Four are one rule: testing.md 2026-10-07 (list a moved bound's cases by the function that decides it), recurring in paste-newline, owed-ledger, resume-and-compact and unwrapped (and, after its 2026-10-16 extension, again in compact as P6's `dialogless_home` case). Three are host-linux.md (the printf transport, a `cd X 2>/dev/null;` prefix, a `viola` census matching the overseer's bridge); one each are testing.md timing windows, the remove-the-guard sed anchor, gate.py hygiene's Windows-path form and events.md's snapshot-before-line order.",
     "A Tier 2 rule that recurs four times after curation is not reaching the step that commits the error, which is research's sweep, not a reader of testing.md at write time. One direction is to carry the moved-bound rule as a mechanism rather than as more text: a research-letter step that lists a moved bound's readers through the code graph (`tree.db` callers of the deciding symbol, e.g. `LedgerRow::ALL` / `relied()`), and on the project side a test helper that derives its pass counts from the deciding function rather than a literal. See chain X1 and P6. The host-linux recurrences overlap level candidate L3."),
    ('implement/code/input.plan-step-ambiguous',
     "Implement met 12 plan steps in 9 chunks it could not follow as written (13 with a step-less record, see Mechanism health). Six left a named choice unbound (a case count, a drop rule's reading, which name, a verdict word, a slice's rows, where shared writers live); six prescribed a shape that would not work (the snapshot written after the line a test waits on, the agents read before `/clear`, a home canonicalised before it exists, rstest rows over a sequence, an rstest in a crate with no rstest dev-dependency under a no-manifest guard, an INTEGRATION layer with no matching binary).",
     "Each was settled and surfaced; none halted. One direction for the unbound half is a validate check over each step for a choice the step names but does not bind (a count, a word, a reading). The unworkable half shows no single check; two of its six (snapshot-after-line, agents-read placement) recur an events.md rule (P3) and one (rstest vs the manifest guard) is the preservation guard of L1."),
    ('*/*/contract.premise-falsified',
     "Verification falsified a premise eight times in five chunks, three with a halt. Four are live-rig premises about undocumented claude CLI behaviour around a rig `proj/` inside the repository (`--scope local` writes the work tree root's settings, a modal before SessionStart with no trust record, a trust row shown unselected, an external-imports modal from the repository CLAUDE.md's import). The others: `/compact`'s post-condition outlasting the 10 s window, a fixture session id off the session-id shape, `cargo tree -i` exiting 0 once the crate is in the lockfile, and `~/.viola` reading 755 after a verify.",
     "The rig-placement four were absorbed mid-epoch: the founder ruled the rig outside the repository (2026-10-18-plugin-precedence-live-readings `inputs#I4`). The remaining pipeline-level direction, if any, is the one CLAUDE.md already states as a learning (an unmeasured CLI behaviour is a ledger row to probe, not an assumption): a phase check that a plan step resting on an undocumented CLI behaviour names its probe or ledger row before the live round spends a start."),
    ('implement/fix-loop/contract.test-expectation',
     "Seven test oracles were wrong on first run in seven chunks. Three are one helper: `tests/support/home.rs` `dialogless_home` pinning a pass-count literal that moved with the ledger row set (resume-and-compact, unwrapped, compact), each found only by pre-push because the plan's integration filter did not select it. The others are oracles written from plan literals (a call bound, an insertion order), a fixture written before the stamp, and test-side expectations in cli-machine.",
     "The three `dialogless_home` cases recur the moved-bound rule (P3) and the research chain (X1). On the project side, one direction is that `dialogless_home` derives its summary from `LedgerRow::relied()` rather than a literal; the ledger does not show whether that has landed. On the pipeline side the direction is X1's."),
    ('wrap-session/reconcile/contract.cascade-miss',
     "Seven times in six wraps a stale master site drew no detector proposal and was caught later in the same pass, five by the step-2 sweep and two by a re-sweep or check 5. The misses are restated counts and literals (cli_verify summaries, the retired browser surface, refusal wording) and passages no detector reads (design-system, see P10).",
     "The sweep is catching what the detectors miss. One direction is to move the recurring sweep patterns (count literals, retired surface names) into detector scope, and to give design-system a detector for its cli wording passages (P10's two structural cases name the same gap)."),
    ('wrap-session/report/input.implement-outcome-unsettled',
     "In five chunks the operator pass's CI read superseded implement's green with a red in a file implement wrote, and the report re-based on the pass. Four of the five are a cross-OS red the Linux dev host does not run: Windows in three chunks (a ledger case, a test oracle, `viola.exe` in a usage line) and macOS in two (a `cli_send` race, a line-before-snapshot order), owed-ledger carrying one of each.",
     "See chain X2: implement's gates run on the Linux host only, and the Windows and macOS legs are first read at the operator pass. One direction is an implement-time cross-OS reading for the cfg- or OS-sensitive cases a chunk touches (e.g. dispatching the CI run on the implement branch head before the outcome is written); the founder weighs that cost against a pass that already catches them."),
    ('implement/fix-loop/contract.spec-reality-gap',
     "Five spec-versus-reality gaps were surfaced at fix-loop in five chunks: the fixture scrub passing a GitHub owner handle, the preservation guard red on a needed companion edit, plan-named E2E binaries outside test-plan's regex, revive's exit-1 codes short of start()'s refusals, and the live transcript delta of 5 against the accepted two.",
     "Each was surfaced and routed through the amendment flow. No common cause is evident across the five; one of them (the guard) is L1's."),
    ('*/*/contract.structural-blind-spot',
     "A documented mechanism could not reach its subject four times in four chunks, one with a halt: design-system's only detector (`D-design-tokens`) cannot see cli wording passages (twice); `agent-run.sh --live-legs --entry N` runs no round step after the named leg, so the trailing census was hand-run (halt); and the E2E rmcp client accepts a `tools/list` result that claude 2.1.287 rejects, so only the live start saw it.",
     "Three directions, one per mechanism: a design-system detector for cli patterns (shared with P7); a round runner form that runs the steps after the last leg; and a contract case that replays a recorded live `tools/list` exchange (a fixture from the live CLI) so the gate sees what only the live start saw."),
    ('phase/plan/input.research-thin',
     "Synthesis found research short four times in four chunks, one with a halt (the resume-and-compact plan revision after implement stopped on the guard): a mutant cause research had the files to find, facts research read before the P4 cut, a literal-keyed sweep that missed `dialogless_home`, and a boundary that missed test supports.",
     "This is chain X1's plan-side end; the direction is X1's."),
    ('implement/fix-loop/tooling.result-not-run-stable',
     "Three times an identical tree read red, then green: a `cli_send` timing case once in seven firings (unwrapped), path7 alone versus in the full layer plus a truncated profile (mcp), and pre-push's coverage merge refusing a truncated `viola hook` `.profraw` (overseer-switch).",
     "See L2: the truncated-profile red recurs across epochs."),
    ('wrap-session/gates/tooling.light-gate-red',
     "The light gate read red twice; one halted. Paste-newline: implement's two reds reproduced exactly, both plan-wording reds (the literal probe, the preservation guard) for which the P7.1 ASSERT has no arm. Overseer-switch: `Disk quota exceeded` at verify's pinned-copy write on the `/tmp` usrquota tmpfs under nextest's parallelism (22 GB of the user's `/tmp` was Claude Code session output), then a truncated `.profraw`.",
     "Two directions: a P7.1 ASSERT arm for a red the plan's own wording causes (L1), and a disk-headroom read before a gate that boots stamped homes (the handoff already records the need). The profraw half is L2."),
    ('*/*/contract.jointly-contradictory-instructions',
     "Three instruction pairs proved jointly unsatisfiable. Two are the plan's preservation guard against a mandatory companion edit (a workflow matrix line; Trigger 3's companion test edits). The third is a plan line saying matrix records are P5's and the wrap's while implement's P2 MUST writes them. No mechanical check tried any of the three.",
     "The first two are L1. For the third, one direction is a phase P4 rule that a plan does not restate or narrow a skill-owned write right."),
    ('*/*/contract.narrow-basis-claim',
     "Three counts were written from a narrower basis than the claim: two research sweep counts typed before their pattern ran (4+2 → 6, 79 → 84 files), and research's literal sweep taken as the full basis for a count move (five shape-pinned readers outside it). A fourth case of the first shape sits untyped (`2026-10-18T03:41:19Z-b`, 74 → 88 hits).",
     "One direction is a research-letter rule that a count in `research.md` is the printed output of the command written beside it, never typed. The third case is X1."),
    ('*/*/contract.grammar-irregularity',
     "Pipeline tools printed a parse verdict four times on the working route: line 156 three times (`CARRY:` after one space, freight or prose, and a `WATCH:` clause with no since/retires/k-of-N form) across two sessions and one take-up, and line 144 once (a 569-char WATCH clause `UNPARSED`). A site at n ≥ 2 is the grammar or its writer.",
     "Line 156's annotation was written off-grammar, and every read after it paid a by-hand fold. One direction is a write-time check: route-resolve runs `route.py pins` on the line it just wrote and refuses an `UNPARSED` / `INDETERMINATE` result, so the irregularity is fixed once at the writer rather than read around at each later read."),
    ('wrap-session/reconcile/input.report-insufficient',
     "The P1 report misled reconcile three times in three wraps: a site table built from four sweep patterns missed differently worded sites, a Counts bullet dispositioned half of a 3.4 KB line, and a Counts row named the stale and the true line the wrong way round.",
     "Each was caught by a detector note or a re-read. The first case shares U1's cause (the sweep patterns); no other common cause is evident."),
]

# ---------------------------------------------------------------- chains
X = [
    ("phase/research →research→ implement/fix-loop · implement/code · phase/plan — 6 + 5 + 6 chunks (12 distinct)",
     [('2026-10-13-interrupted-verify-cleanup', '2026-10-13T18:39:01Z-a', '2026-10-13T19:09:01Z-a'),
      ('2026-10-13-paste-newline-ledger-row', '2026-10-13T20:43:52Z-a', '2026-10-13T21:39:27Z-a'),
      ('2026-10-14-resume-and-compact-ledger-rows', '2026-10-14T07:22:25Z-a', '2026-10-16T07:26:50Z-a · 2026-10-16T07:32:45Z-b'),
      ('2026-10-16-unwrapped-sessions-on-the-board', '2026-10-16T19:05:10Z-a', '2026-10-16T21:09:24Z-a · 2026-10-16T20:44:30Z-a'),
      ('2026-10-18-compact-post-condition-ledger-row', '2026-10-18T03:41:19Z-a', '2026-10-18T21:43:15Z-a'),
      ('2026-10-19-dialog-hold-for-the-driver', '2026-10-19T07:53:34Z-a', '2026-10-19T08:36:28Z-a · 2026-10-19T08:27:44Z-a/-b'),
      ('2026-10-13-epoch-4-survivors', '2026-10-13T14:32:47Z-b', '2026-10-13T16:18:35Z-a · 2026-10-13T15:55:43Z-a/-b'),
      ('2026-10-13-owed-ledger-rows', '2026-10-13T22:42:12Z-a', '2026-10-13T23:18:46Z-a · 2026-10-13T22:59:19Z-a/-b'),
      ('2026-10-16-remove-the-browser-leg', '2026-10-16T15:49:00Z-a', '2026-10-16T16:26:06Z-a/-c'),
      ('2026-10-17-cli-machine-contract', '2026-10-17T02:38:42Z-a', '2026-10-17T02:44:48Z-a/-b'),
      ('2026-10-17-cli-output-discipline', '2026-10-17T04:46:09Z-a', '2026-10-17T04:50:30Z-a · 2026-10-17T07:36:41Z-a'),
      ('2026-10-17-mcp-server-for-drivers', '2026-10-17T09:44:50Z-a', '2026-10-17T11:25:18Z-a')],
     "Every producer record is formally `ok` with the one signal `unresolved-questions` (unwrapped adds `known-positive-control`); every consumer graded `research` `thin`. The recurring miss is a reader research's literal-keyed sweep cannot see: a helper that builds a count from an integer or pins a row set by shape (`tests/support/home.rs` `dialogless_home` in four chunks: paste-newline, resume-and-compact, unwrapped, compact; `merge_stamp`'s envelope; a second verdict vector), and a test-side caller of a changed constructor (`src/run/wheel.rs` in dialog-hold). **Chain hypothesis:** research grades itself on its lists being written, not on their reach, and the cost lands at fix-loop (stale oracles, P6), at implement/code (companions outside the guard, L1) and at synthesis (P11). It is the same cause as P3's four-time recurrence.",
     "A research-letter step that lists a moved bound's or a changed signature's readers through the code graph (`tree.db`: callers and readers of the deciding symbol) beside the literal sweep, and a research producer signal that names which of the two produced its lists, so a consumer's `thin` can be joined to it."),
    ("implement/smoke →implement-outcome→ wrap-session/report — 7 chunks",
     [('2026-10-13-owed-ledger-rows', '2026-10-13T23:30:43Z-d', '2026-10-14T06:41:55Z-a'),
      ('2026-10-14-resume-and-compact-ledger-rows', '2026-10-16T07:38:01Z-b', '2026-10-16T14:10:45Z-a'),
      ('2026-10-16-unwrapped-sessions-on-the-board', '2026-10-16T21:09:24Z-d', '2026-10-16T23:05:15Z-a'),
      ('2026-10-17-cli-machine-contract', '2026-10-17T03:26:06Z-a', '2026-10-17T03:55:28Z-a'),
      ('2026-10-17-cli-output-discipline', '2026-10-17T08:01:35Z-a', '2026-10-17T08:32:01Z-a'),
      ('2026-10-18-compact-post-condition-ledger-row', '2026-10-18T21:43:15Z-f', '2026-10-18T22:04:15Z-a'),
      ('2026-10-19-dialog-hold-for-the-driver', '2026-10-19T08:51:58Z-a', '2026-10-19T09:32:45Z-a')],
     "Producers are `ok` (`green` or `surfaced`); each consumer graded `implement-outcome` `thin` because the operator pass changed a file implement wrote. Five of the seven are CI reds the pass fixed (owed-ledger, unwrapped, cli-machine, cli-output, dialog-hold; four of them on Windows or macOS, P8); two are the pass's hygiene rewrite of an implement-written evidence line (resume-and-compact, compact). Read with the 2026-10-13 calibration boundary: these are fixes to implement's own files, so `thin` is the grade's intended reading, not era. **Chain hypothesis:** implement's gates are the Linux dev host's; the cross-OS legs exist only in CI and are read first at the operator pass. This is P8's evidence seen from the producer's side.",
     "As P8, plus one for the two hygiene rewrites: implement's evidence writes pass through the same hygiene read the operator pass applies, before the outcome is written."),
    ("phase/plan · phase/validate →plan→ implement/fix-loop · wrap-session/gates — 4 + 3 chunks",
     [('2026-10-13-interrupted-verify-cleanup', 'phase/validate', '2026-10-13T19:09:01Z-a · 2026-10-13T20:30:06Z-a'),
      ('2026-10-13-paste-newline-ledger-row', 'phase/validate', '2026-10-13T21:39:27Z-a · 2026-10-13T22:18:46Z-a'),
      ('2026-10-18-compact-post-condition-ledger-row', 'phase/validate', '2026-10-18T21:43:15Z-a · 2026-10-18T22:36:37Z-a'),
      ('2026-10-18-plugin-precedence-live-readings', 'phase/validate', '2026-10-19T06:19:25Z-a')],
     "The plan reaches fix-loop graded `wrong` or `thin` and reaches the light gate `thin`, in the same chunks, for the same entry: the preservation guard (or, in paste-newline, the literal probe beside it) red on a recorded companion edit. Validate graded the plan `ok` each time; its controls did not try a recorded companion (the jointly-contradictory records say so). **Chain hypothesis:** the guard is authored at P4 without an exclusion for the companions fix-loop is obliged to record, so a correct tree reads it red at implement and again at the gate. This is the level candidate L1.",
     "L1's."),
]

# ---------------------------------------------------------------- level candidates
L = [
    ("L1 — override — the plan's preservation guard red on recorded companion edits — 4 overridden + 1 unresolved + 1 workaround facts, 4 chunks",
     [('2026-10-13T19:09:01Z-a', 'operator chose the companion'), ('2026-10-13T20:30:06Z-a', "light gate's entry 9"),
      ('2026-10-13T22:18:46Z-a', 'commit and master flip'), ('2026-10-18T22:36:37Z-a', 'entry 15 red on recorded'),
      ('2026-10-18T21:43:15Z-a', 'preservation guard (entry 15) reads red'), ('2026-10-19T08:27:44Z-a', 'literal table test')],
     ['2026-10-13T19:09:01Z-b', '2026-10-13T21:39:27Z-d', '2026-10-16T07:26:50Z-c', '2026-10-19T08:27:44Z-c', '2026-10-13T22:18:46Z-b'],
     "The operator overrode the same gate four times in three chunks (interrupted-verify, paste-newline, compact), each time because a correct tree carries an edit the fix-loop protocol makes mandatory and records (`scope-record.md`), and the guard, authored at P4, does not exclude recorded companions. The same rule drove 4 of the epoch's 6 fix-loop `ok-degraded` outcomes and both gates `ok-degraded` outcomes, a plan revision (resume-and-compact), and two jointly-contradictory records; the handoff carries it for the operator's word (\"the plan template's preservation guard reads red on any recorded companion edit\"). **Level hypothesis:** the cause lives in the plan template's guard form, which is pipeline-owned. The fixes so far are per-chunk overrides and deviations.",
     "The guard's form reads the chunk's own `scope-record.md` and excludes the companions recorded there, and the P7.1 ASSERT gains an arm for a red the plan's own wording causes. Whether that weakens the guard's purpose is the founder's judgment."),
    ("L2 — chronic-degrade — a truncated `.profraw` (and other not-run-stable reds) at the coverage merge, recurring across epochs — 3 in-epoch facts + 3 in-epoch frictions",
     [('2026-10-19T11:12:09Z-a', 'llvm-cov refused'), ('2026-10-19T20:39:26Z-a', 'pre-push red again'),
      ('2026-10-16T21:09:24Z-a', 'cause not known')],
     ['2026-10-16T21:09:24Z-b', '2026-10-17T12:33:19Z-d', '2026-10-19T11:12:09Z-b', '2026-10-19T20:39:26Z-b'],
     "Pre-push's coverage merge refused a truncated `viola hook` `.profraw` with every test passing in Epoch 3 (`2026-10-04T22:18:54Z-b`: \"the WATCH subject, 0/3 since the wheel chunk\"; `2026-10-06T22:24:12Z-b`: three truncated profiles in three runs), in this epoch's mcp chunk (read as fixed), and twice in overseer-switch (implement and the light gate). `tooling.result-not-run-stable` holds 18 records across all five epochs. It never halted: each instance was deferred, ruled not this chunk's, or re-fired green, and the route now carries it as a WATCH on \"Newer claude CLI rows\", 0/3. **Level hypothesis:** an environment or harness cause (the profile writer killed mid-write; the handoff says the killer is unknown) absorbed one chunk at a time as a re-fire.",
     "A cause-finding chunk of its own (the killer of the `viola hook` profile writer) rather than a per-chunk WATCH, or a coverage-merge form that names and skips a truncated profile with a recorded count. The founder chooses between them."),
    ("L3 — band-aid — a pipeline tool's output filtered or discarded against the run-bare rule — 9 facts, 8 chunks",
     [('2026-10-13T07:15:41Z-a', 'tail -2'), ('2026-10-13T07:23:36Z-a', 'through tail'), ('2026-10-13T15:59:51Z-a', 'tail -n 1'),
      ('2026-10-17T04:42:07Z-a', 'grep -v'), ('2026-10-17T09:37:18Z-a', '/dev/null'), ('2026-10-17T18:06:04Z-a', 'into tail'),
      ('2026-10-18T22:33:38Z-a', '/dev/null'), ('2026-10-19T08:18:14Z-a', 'through grep'), ('2026-10-19T10:52:14Z-a', 'into a filter')],
     [],
     "Nine times in eight chunks (distill ×3, validate ×4, research ×1, reconcile ×1) a pipeline tool's stdout went through `tail`, `grep` or `/dev/null` against the letters' rule (and, from 2026-10-13, the operator's own rule and the user's memory rule). The Bash guard refuses the plain form, but two notes record forms it did not refuse: a `for` loop's combined output piped on (`2026-10-17T04:42:07Z-a`) and a pipe inside a control helper (`2026-10-19T10:52:14Z-a`). `/dev/null` appears twice. The previous epoch holds no such fact, so the theme starts with the rule. The handoff's recurrence-despite-learning list carries it again this wrap. No typed friction correlates; the deviations were smooth. **Level hypothesis:** the obstacle is the tools' output volume against a read-it-whole rule, held by a guard with gaps and by agent recall.",
     "Either the tools print a compact verdict form by default (so a bare read is cheap and the urge to filter goes), or the guard covers the loop-wrapped, helper-internal and `/dev/null` forms. Both live in the pipeline, not the project."),
    ("L4 — band-aid — multi-KB single-line edits through scratch python replace scripts — 3 facts in-epoch, 9 in Epoch 4",
     [('2026-10-16T18:40:25Z-a', 'exact-match python'), ('2026-10-17T04:21:36Z-a', 'python replace scripts'),
      ('2026-10-17T04:25:45Z-a', 'two multi-KB route lines')],
     ['2026-10-17T14:58:42Z-b'],
     "Reconcile (×2) and route-resolve (×1) edited master and route lines of several KB through count-asserted python scripts in the scratchpad, where the letter names the Edit tool or a splice call. Epoch 4 holds nine facts of the same theme (reads and edits of multi-KB lines through scratch window and replace scripts) and three `tooling.long-line-edit` records; this epoch adds one more of that type. **Level hypothesis:** the masters' and the route's line grammar (one passage or one entry per line, measured multi-KB) is the obstacle; each session rebuilds the same scratch tool.",
     "One sanctioned long-line point-edit form, an anchored replace with a single-match assertion as a pipeline tool verb, named by the letters that today name the Edit tool. The other direction is a line grammar that breaks long passages."),
    ("L5 — band-aid (cross-stream) — `cascade.py` reads a backticked `|` on a non-table line as a table cell — 2 facts + 3 frictions, 4 distinct events in 4 chunks",
     [('2026-10-16T23:37:37Z-a', 'reshaped'), ('2026-10-19T09:53:39Z-a', 'reshaped')],
     ['2026-10-17T14:55:18Z-b', '2026-10-19T09:53:39Z-c', '2026-10-19T19:40:54Z-c'],
     "`cascade.py leaves --apply` and `cascade.py replace` refused a finding as `reshaped` because a `|` inside a backtick code span on a list line read as a cell boundary: unwrapped (fact), mcp (friction, retries 3), dialog-hold (fact + friction, one event), overseer-switch (friction). Each was routed around by writing the edit by hand or replacing the pipe with words. Problem facts alone are n = 2, under the F-2 floor. The theme reaches four distinct events only by joining the friction stream, so it is surfaced here labelled cross-stream. The handoff carries it for the operator's word. **Level hypothesis:** a parser defect in a pipeline tool, absorbed by hand in each wrap.",
     "The cell-boundary reading in `cascade.py` skips a `|` inside a code span, or applies only to table lines."),
]

# ---------------------------------------------------------------- playbook-extension candidates
U = [
    ("U1 — wrap-session/report (+ reconcile) — 4 cases → proposed type `retry.sweep-pattern-authoring`",
     ['2026-10-13T21:56:23Z-b', '2026-10-16T14:10:45Z-b', '2026-10-17T03:55:28Z-c', '2026-10-17T14:55:18Z-e'],
     "The site-sweep / cascade-sweep patterns file the step authored was refused or misfired on its first firing (TOML literal quoting, an inline regex flag's position, a doubled backslash through printf, a pattern id over its bound, an unanchored pattern matching the registries' `-plan/` paths) and was re-fired.",
     "a sweep patterns file this step authored is refused by the tool, or its first firing reads mostly noise, and is re-authored and re-fired: record the refusal or noise and the re-fires."),
    ("U2 — wrap-session/gates (+ phase/validate) — 2 in-epoch, recurring from Epoch 4 → proposed type `contract.hygiene-self-refusal`",
     ['2026-10-16T14:51:33Z-c', '2026-10-18T03:15:41Z-b', '2026-10-10T02:42:58Z-d'],
     "The hygiene read refused a file this run wrote because the file quoted a path as prose or example (a hazard glob being recorded, a wireframe's example home, an in-root home path); the in-epoch typed sibling `2026-10-13T11:05:16Z-b` (recall: a quoted Windows drive path) is the same mechanism.",
     "the hygiene read refuses text this run wrote that quotes a path as an example, a hazard or a record (not a leaked real path): record the refused file, the path form and the re-spelling."),
    ("U3 — phase/take-up — 1 in-epoch, recurring from Epoch 4 → proposed type `tooling.output-clipped`",
     ['2026-10-13T07:09:23Z-b', '2026-10-09T15:07:26Z-b'],
     "`ci.py conclusion` printed a red row with its run id clipped (`nightly#38…`, `windows-mutants#…`) and the failing jobs without their run, so a `gh` read was needed to own the red.",
     "a pipeline tool clipped a field the step needs whole (an id, a path, a job name), and an extra read recovered it: record the tool, the field and the recovery read."),
    ("U4 — any step (reconcile ×2 in-epoch, gates in Epoch 4) → proposed universal type `tooling.cwd-drift`",
     ['2026-10-13T11:04:28Z-c', '2026-10-18T03:06:09Z-d', '2026-10-11T08:55:19Z-b'],
     "A path resolved against a working directory other than the one intended: a run-dir-relative `../../../../` redirect landing above the repository (twice, one epoch apart), and a `cd X 2>/dev/null;` prefix that moved the session cwd for later calls (also a `recall.corpus-recurrence`, `2026-10-18T03:06:56Z-b`).",
     "a command read or wrote through a cwd other than the one intended (a relative path climbing out of the tree, a `cd` that persisted): record where it landed and the move back."),
    ("U5 — any step (route-resolve in-epoch, reconcile and fix-loop in Epoch 4) → proposed universal type `contract.unclocked-time`",
     ['2026-10-17T14:58:42Z-c', '2026-10-10T23:13:01Z-f', '2026-10-11T07:58:07Z-c'],
     "A time was written without a clock read (an `inputs.py snap --origin` placeholder `14:3xZ`, journal lines typed ahead of the clock), refused by the tool or the stamp-ahead hook, or caught on re-read. The in-epoch problem fact `2026-10-13T08:56:20Z-a` (validate: a controls record stamped ahead of the clock, the hook refused it) is the same mechanism.",
     "a time was written into a record, trail or argument without a clock read and was refused or corrected: record where, and what caught it."),
    ("U6 — implement/code — 1 in-epoch, recurring from Epoch 4 → proposed type `tooling.removal-denied`",
     ['2026-10-16T16:26:06Z-b', '2026-10-09T16:07:38Z-c'],
     "A removal the step needed was denied by the host's permission layer (a recursive `rm` of a tracked dir and its move-aside, at the floor of F-4; the Epoch 4 case is a compound call denied whole for one `rm -r`). The theme also stands in two in-epoch problem facts (`2026-10-16T16:26:06Z-a`, `2026-10-16T21:09:24Z-a`) and in the handoff's desk list of directories left for the founder.",
     "a removal the plan or the step needs is denied by the host or the classifier: record the target, whether a sanctioned move-aside was tried, and where it was left."),
]

# ---------------------------------------------------------------- appendix
BELOW = [
    "Typed, below F-2 (as recorded): implement/fix-loop `retry.fix-iterations` 2 · universal `contract.token-proxy-check` 2 (one is L5's mcp case) · universal `tooling.host-shell` 2 · implement/code `tooling.hook-friction` 2 (one is a rustfmt-reflow anchor miss, not a hook) · phase/plan `retry.synthesis-rework` 2 · reconcile `ambiguity.playbook-no-match` 2 · `recall.change-reconstruction` 2 · implement/code `input.research-files-wrong` 2 (X1) · new-session/orientation `input.handoff-git-mismatch` 2 (both: post-wrap bookkeeping read as dirty) · phase/distill `input.spec-source-gap` 2 (one chunk) · curation `ambiguity.tier-routing` 2 · reconcile `contract.false-positive-proposal` 2 · reconcile `contract.proposal-format` 2.",
    "**Borderline by a missing `step`:** reconcile `contract.false-positive-proposal` reads n = 3 if the step-less wrap record `2026-10-13T18:17:23Z-b` (a D-tests-framework proposal rejected as registry over-reach, evidently reconcile) is folded in; it is kept as recorded. The step-less implement record `2026-10-13T17:51:07Z-b` would make P4 13.",
    "Typed, n = 1: `recall.curated-rule-not-applied` · validate `ambiguity.review-cycles` · gates `contract.coverage-hold` · route-resolve `contract.carry-no-owner` · fix-loop `ambiguity.scope-pressure`, `contract.instrument-validity`, `contract.vacuous-check-found`, `tooling.gate-deferral` · research `contract.instrument-validity` · reconcile `retry.detector-respawn` · route-resolve `tooling.long-line-edit` (L4), `contract.no-sanctioned-channel` · `tooling.output-cap-overflow` · distill `contract.binding-contradiction`, `contract.extract-format` · take-up `input.carry-context-gap`.",
    "Untyped clusters under F-4: moved-count readers missed at implement/code, 2 (`2026-10-13T23:18:46Z-b`, `2026-10-16T06:59:33Z-c`; both are `input.research-files-wrong` members filed untyped, part of X1) · L5's two untyped `reshaped` records · a pipeline tool's own argument refused on first call, 4 distinct tools (`gate.py show --plan`, `splice append --lines`, `sidecar.py cites` reading a 0-pending wrap heading as `not a marker`, `gh api` without `--allow-escape-sequences`) · the 0-pending wrap's off-letter Phase 1 on the operator's direction, 2 (one wrap) · a hand-summed research count, 1 (a `contract.narrow-basis-claim` filed untyped).",
    "Untyped singles: nextest `test(=tests::name)` selecting nothing outside a crate-root module (retries 19) · the leaf snapshot pair broken by `route.py --run-dir` trails (halt 1, dialogue 1) · claude auto-updating 2.1.296 → 2.1.302 mid-chunk (retries 2, dialogue 2) · fixtures with no non-null preview / background_tasks · a fork's recommended option and its preview disagreeing · 14 leaf comparers for a count-and-wording chunk · Run E's cross-directory resume judged from CLI code, unmeasured · a fix waiting on a hook receipt the booted fake agent never writes.",
    "Problem-fact themes under F-2: `architecture.md` has no primary-language line for `health.py --stack` (2, both orientation, Epoch 4 none; this session's orientation took `rust` the same way) · nextest filters selecting no rstest case (1 fact + its friction) · a `viola` census matching the overseer pair's bridge (1 fact + a recall record) · removed-cause facts (7) form no repeating theme · overridden facts outside L1: operator or founder holds and pauses (4: P4 paused for the founder's machine, a fork held for his own word, the E2E regex widening, the rig outside the repository), the live-start cap raised once, a route-no-marker trail accepted, the overseer-switch profraw ruling (L2).",
    "Deferred-forever: no hit. All 9 `deferred` facts name a destination (implement's first step, the wrap, the report, the operator pass's CI read, the founder's answer). Two later rode the route: compact's `case_4` red is now a CARRY on \"Windows endpoint admission\", and overseer-switch's profraw red is a WATCH on \"Newer claude CLI rows\" (L2).",
]


def section_chain(title, rows, body, proposal, n):
    t = ['| chunk | producer (step record) | consumer (step / friction records) |', '|---|---|---|']
    for c, p, k in rows:
        t.append(f'| {c} | `{p}` | `{k}` |')
    return f'### X{n} — {title}\n' + '\n'.join(t) + f'\n\n{body}\n\n**Proposal:** {proposal}\n'


def section_level(title, fact_keys, fr_ids, body, proposal):
    s = f'### {title}\n**Facts:**\n\n' + fact_rows(fact_keys) + '\n'
    if fr_ids:
        s += '\n**Correlated friction records:**\n\n' + friction_rows(fr_ids) + '\n'
    return s + f'\n**Level hypothesis:** {body}\n\n**Proposal:** {proposal}\n'


def section_u(title, ids, cluster, criteria):
    return f'### {title}\n' + friction_rows(ids) + f'\n\n**Cluster:** {cluster} → **draft criteria line:** {criteria}\n'


h = health
sl = h['signal_line']
ur = h['untyped_rate']
ur_line = ' · '.join(f"{k} {v['untyped']}/{v['friction']}" for k, v in ur.items() if v['friction'])
health_md = f"""- Records: {h['records']} ({h['step']} step / {h['friction']} friction) over 18 chunks, plus {len([1 for c in h['coverage'] if c == '(null)'])} chunk-less group (41 new-session orientations, one operator-requested adaptation wrap). Retractions in range: none. Whole-ledger pre-pass: 2 retracted, 3 clause-retracted (none in this epoch), 0 retraction-of-retraction, 2 unresolvable, reported verbatim below.
- Coverage vs expected (phase 5 · implement 3 · wrap 5): all 18 chunks complete, no checkpoint gap. Above expected, all explained by their own records: resume-and-compact (phase 8: a plan revision; implement 5: a re-entry), mcp (implement 4: a second smoke), cli-output, compact and plugin-precedence (phase 6 each: P4 paused for the founder's word and resumed). The chunk-less wrap carries report · reconcile · curation · route-resolve (an adaptation wrap, legitimate).
- Unparseable lines: 0 (whole ledger, 2251 lines). Malformed `ts`: 0. `id` fill: {h['id_fill'][0]}/{h['id_fill'][1]}.
- Two friction records carry no `step` (`2026-10-13T17:51:07Z-b` implement, `2026-10-13T18:17:23Z-b` wrap-session). They are grouped as recorded; see the appendix for the one this moves to the threshold.
- Untyped rate (untyped / friction per step): {ur_line}. Overall 30/190. Several untyped records fit an existing type (`contract.narrow-basis-claim`, `input.research-files-wrong`), named in the appendix.
- Problem-fact fill: {h['problem_fill'][0]}/{h['problem_fill'][1]} step records carry a fact (74 facts: workaround 43 · overridden 13 · deferred 9 · removed-cause 7 · unresolved 2).
- Non-ok outcomes: reconcile `halted-resolved` 11 · fix-loop `ok-degraded` 6 · plan `soft-exit` 3 · gates `halted-resolved` 2, `ok-degraded` 2 · route-resolve `halted-resolved` 2 · code `ok-degraded` 1 · smoke `ok-degraded` 1.
- guard-refused {sl['guard_refused']} over {sl['K']} step records · exact-hit {sl['exact_hit']} over {sl['J']} curation records. No signal string is off-form.
- Calibration boundaries in range: implement-outcome's operator-pass reading (2026-10-13, the epoch's first day; X2 reads with it). The `input.*` same-artifact grade cap (2026-10-19) means most of this epoch predates it, and an `ok` beside an `input.*` friction is that era's grade. Orientation's widened ladder gloss (2026-10-13). `guard-refused` / `exact-hit` (2026-10-09) cover the whole epoch.
- Unresolvable retractions (whole ledger, Epoch 2, for the founder's manual discount): """ + '; '.join(f"`{u[0]}` {u[1]}: {u[2]}" for u in retr['unresolvable']) + '\n'

doc = [TITLE, '', '## Mechanism health', health_md, '## Proposals (typed patterns)', '']
for n, (key, pat, prop) in enumerate(P, 1):
    doc.append(typed_section(n, key, pat, prop))
doc += ['## Cross-step chains (starting heuristics)', '']
for n, (t, rows, body, prop) in enumerate(X, 1):
    doc.append(section_chain(t, rows, body, prop, n))
doc += ['## Level candidates (systemic-masked-as-project)', '']
for t, fk, fr, body, prop in L:
    doc.append(section_level(t, fk, fr, body, prop))
doc += ['## Playbook-extension candidates (untyped patterns, F-4)', '']
for t, ids, cl, cr in U:
    doc.append(section_u(t, ids, cl, cr))
doc += ['## Below threshold — no action', ''] + [f'- {b}' for b in BELOW] + ['']
open(f'{RUN}/proposals.md', 'w', encoding='utf-8', newline='\n').write('\n'.join(doc))

json.dump({'themes': {t: {'facts': [k for k, _ in fk], 'frictions': fr} for t, fk, fr, _, _ in L},
           'signatures': {'band-aid': ['L3', 'L4', 'L5 (cross-stream)'], 'override': ['L1'],
                          'chronic-degrade': ['L2'], 'deferred-forever': [],
                          'removed-cause-observation': []}},
          open(f'{RUN}/q-level.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
json.dump({t: ids for t, ids, _, _ in U}, open(f'{RUN}/q-untyped-clusters.json', 'w', encoding='utf-8'),
          ensure_ascii=False, indent=1)
missing = [k for k in typed if typed[k]['above'] and k not in {p[0] for p in P}]
print('rendered', len(P), 'proposals ·', len(X), 'chains ·', len(L), 'level ·', len(U), 'extension · missing above:', missing)
