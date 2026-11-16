## After implement
- **Report:** 2026-11-16T03-08-18-implement — the newest implement run of 2026-11-16-wait-wake-condition (`rec.py runs
  --kind '*-implement' --file '*-2026-11-16-wait-wake-condition.json'`, its last run line), its outcome `green`
  (`outcome-2026-11-16-wait-wake-condition.md` of that run). The chunk took four implement runs (01-25-19 surfaced,
  01-45-25 surfaced, 02-56-58 surfaced, 03-08-18 green) and plan revisions rev 4 → 10 (phase runs 00-29-34, 01-41-00,
  02-52-20, 03-04-30), each surfaced gap absorbed by the next revision.
- **Commits:** since `last_wrap` 2026-11-15T23:35:53Z (the `state` view): `df3bad61 feat(2026-11-15-api-error-turn-end):
  …` (the previous wrap's own commit, not this chunk's) · `d107a553 chore(2026-11-16-wait-wake-condition): operator
  pre-CI commit, for the run this chunk's verdict reads` (this chunk's; base `df3bad61`, its parent).
- **Operator pass:** run in implement run 03-08-18's window (`evidence/operator-pass.md`):
  - entry 14 `gate.py hygiene`: `hygiene: clean — read 290 (runs 265 · evidence 8 · inputs 17) · trails 45 not read ·
    copies 15 not read by P1 — 0 host paths kept · binary 0 not read by P1` (`evidence/operator-14-hygiene.txt`);
  - entry 15 the push: green, `df3bad61..d107a553  build/viola-0.2.0 -> build/viola-0.2.0`;
  - entry 16 `ci.py conclusion --sha HEAD --wait 1200`: `d107a553c832 verdict: green · checks 7/7 · wall 437 s · runs
    ci#43797036340 completed/success` (fuzz-replay · lint · msrv · perf · release · supply-chain · test).
  - The pass's end: as-left — no fix commit.
- **Changes after implement:** none — no commit after the pre-CI one (`git log df3bad61..HEAD`: `d107a553` alone).
- **Inputs:** `inputs.py verify`: `inputs: 16 entries — unchanged 7 · drifted 1 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 8 · uncited 3 · unparsed 0`.
  - `I3 drifted — the source's bytes moved · copy no-repo · ../additional/viola-overseer/pwait.sh` — DRIFT: the
    overseer's waiter now reads the turn end's last message line and wakes on a turn end that ASKS even with a
    background task running (comment `V150 (2026-11-16)`) — the V150 fix, done outside viola as the report's
    Beyond-the-plan row 2 named; no fact of this report rests on the old bytes beyond what row 2 already states.
  - `I9 UNCITED`, `I12 UNCITED` — both are P5 words on superseded cards; implement's Inputs bullet names them
    (`inputs#I6`, `I9`, `I12`) in a form the citation reader counts for I6 alone. No fact rests on them beyond their
    cards' rows, carried by `inputs#I15`.
  - `I16` (new, `--step wrap`) — inputs#I16, the overseer's wrap directive
    (`../additional/viola-overseer/wrap-wait-wake-directive.md`): F1 PROVISIONAL with the founder away; the carried CI
    red closed on measurement; the hang carry owned by «Version done-check»; `session_crons` a founder candidate with no
    route entry; the V150 fix outside viola; the amendment candidates; two card items.
- **Scope:** none — the wrap's scope read appended no line: `scope: clean — changed 6 · listed 6 · recorded 0 (companion
  0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 273` (record: `listed — tests/cli_verify.rs`).
- **Sweep reach:** `cascade.py sweep --pre-pass`, `.andromeda/runs/2026-11-16T03-24-03-wrap/report-patterns.toml`,
  baseline HEAD `d107a553`:
  - `wake-kinds` (Expected amendment architecture contract/channel-wait/wake-kinds; owner architecture): standing 8 —
    architecture 2 (`ctr-yns6vswp`, `ctr-uafxdsbc` — the wake set; candidates, the turn-end-whatever-its-tasks clause
    is P2's) · test-plan 3 (`ctr-ketfehl3`, `ctr-f5rl62ri` — restatements of the wake set, candidates beside the
    architecture row; `ctr-ic6eivw6` — see `non-waking`) · obs-plan 2 (`ctr-5l5et57y` dialog-expired wakes,
    `ctr-5oj35pdm` outcome over the wake set — true claims sharing the token, no change) · a11y-plan 1 (`ctr-2iuqepef`
    dialog-expired wakes — true claim, no change).
  - `non-waking` (Expected amendment test-plan contract/viola-wait/non-waking-records; owner test-plan): standing 1 —
    test-plan `ctr-ic6eivw6` («Activity and wheel records never wake a parked wait») — candidate: the harness-origin
    `prompt-submitted` is not named there.
  - `stamped-home` (Expected amendments test-plan decision/fresh-home-per-test · rstest-fixture-chain ·
    verify-sole-stamp-writer; owner test-plan): standing 6 — test-plan `dec-dvrimbrq` fresh-home-per-test (candidate:
    one stamp per binary, copied) · `dec-h23ez5ma` guarded-paths-start-stamped (true claim, no change) · `dec-ic4quyea`
    ci-stamped-home-kill-line (candidate: the rationale) · `dec-cxlxjspx` the fixture chain (candidate) ·
    `dec-fciy32om` verify-sole-stamp-writer (candidate: still the sole writer; the copy is no write of a stamp) ·
    `dec-eejgmpdq` rstest-fixture-chain (candidate).
  - `stamp-kill` (Expected amendment test-plan decision/verify-driven-tests-20s-kill · ci-stamped-home-kill-line; owner
    test-plan): standing 2 — `dec-ic4quyea` (as above) · `dec-enalpnf3` verify-driven-tests-20s-kill (candidate: the
    rationale moves to the stamp taken once, the kill unchanged).
  - `park-bound` (`PARK_BOUND`, the directive's candidate): control UNFIRED over the seven masters — the tool's refusal
    (exit 3, `cascade-2026-11-16-wait-wake-condition-pre-pass.json`) is the basis of its master zero; a test-module
    constant, no master site.
  - Spec claims disproved (two plan claims): no master site — each a plan revision's prediction, absorbed by rev 6 and
    rev 10.
  - Counts / qualifiers moved: none (implement's `none — verified` stands).
  - `Sweep reach: 5 patterns · 6 moved facts, 5 with a pattern (the report's own count: the four Expected amendments,
    the non-waking clause and PARK_BOUND; the two disproved plan claims have no master site) · by line 0 lines · 0
    offsets` — the `total` line counts 4 patterns run (`total (4 patterns) · 30 rows over 10 files`), `park-bound`
    refused before it.
- **Outcome, final:**
  - supersedes «(ci) the carried red closed on CI — owed to the operator pass's `ci.py conclusion` (entry 16)»: MET and
    the carried red CLOSED — the final HEAD `d107a553`'s run ci#43797036340 reads `verdict: green · checks 7/7`, its
    `test (ubuntu-latest)` job running the stamped-home class under the unchanged 20 s `ci` kill
    (`evidence/operator-pass.md`). The measurement that closes it: one `viola verify` stamp per test binary under a
    lock, copied into each test's home; `path1_run_start_one_home_in_order` 6.593 → 2.385 s, the class wall 56.069 →
    48.381 s, 321 = 321 tests (`evidence/class-wall.json`). The CI runner's own slowness was never this chunk's to
    prove and is not claimed (inputs#I16 item 2).
  - supersedes «`gate.py hygiene`, the push and `ci.py conclusion --sha HEAD --wait 1200` — `not run — leg operator`»:
    entries 14 · 15 · 16 green, as **Operator pass** above.
  - The F1 reading stands PROVISIONAL (inputs#I5, inputs#I16 item 1): the cap's «a test that fails before the fix» is
    read as each case's remove-the-guard control — the six controls all `holds`; nothing is ratified, the founder's
    word owed.
  - Watches: none folded.
  - Outcome basis: Setup 4's commit list (`d107a553` alone after the base), the final HEAD's CI run ci#43797036340 in
    `evidence/operator-pass.md`, implement's report for the rest.
- **Decisions & corrections after implement:**
  - inputs#I16 item 1 — the founder is AWAY; where a letter halts for his word, the delegate's answer by relay is taken
    PROVISIONAL, the most conservative option; nothing is ratified.
  - inputs#I16 item 3 — the Carry-forward (the two older wait tests,
    `wait_a_line_appended_and_signalled_during_the_park_wakes_it` and `wait_a_signalled_log_only_line_parks_again`,
    hanging past a failure in `thread::scope`'s join of a waiter on a clock that never advances) is pinned to «Version
    done-check», the P5 word of revision 3 (inputs#I15).
  - inputs#I16 item 4 — `session_crons` as a count beside `background_tasks`: a founder candidate, a boundary widening,
    his to give; no route entry is minted. The V150 fix is the overseer's own waiter, done outside viola (I3's drift):
    no row.
  - inputs#I16 item 2 — the carry on «`wait` wake condition» (the CI timing red) is discharged on the measurement
    above.
  - inputs#I16 item 6 (for the card, no edit asked): the code's relied ledger count is 21
    (`crates/viola-agent-claude/src/ledger/test_cases_stamps.rs:66`, `assert_eq!(here.len(), 21)`), while leaves still
    say twenty-four: `.claude/rules/verification-harness.md:28` («every relied row, `LedgerRow::relied()`: all
    twenty-four»), `.claude/docs/commands.md:23` and `:24`, `.claude/docs/tests-summary.md:21` («all 24 on every
    build»); no master states the count (`rec.py find --regex '(twenty-one|twenty-four|21|24) (relied|rows)|all
    twenty'` over architecture and test-plan: 0 hits). And `rec.py put` of plan rev 3 printed `UNPARSED: Expected
    newline or end of document after a statement (at line 17, column 65)` and exit 3, and landed the rev anyway
    (`.andromeda/runs/2026-11-16T00-29-34-phase/p5-card-1.md:134`).
  - The operator pass record: `## Corrections and rulings during the pass` — none.
- **Process hygiene, re-measured:** `gate.py census --comm viola --comm cargo-nextest --comm viola-fake-agent --args
  projects/viola/target` by this wrap's run dir at 2026-11-16T03:25Z: `census: 7 running`, none from this repository's
  `target/` (the product install `~/.local/viola-product`, the lab prototype's builds, the `~/.viola/bin/0.1.0-…` mcp).
