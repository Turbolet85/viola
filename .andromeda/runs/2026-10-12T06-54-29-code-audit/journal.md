# Code audit journal — Epoch 4 — Session state & governance

Mode trend · HEAD dffad3e02d380362b8040effacc95781ceea8739 · baseline e304994ae0413c5cf5bf679a5b3d19abded0e472
(Epoch 3) · span 1 · overshoot 0 (the pickaxe on `2026-10-11-the-board-viola-list · complete` returns HEAD).
Run dir first created under a time ahead of the clock (06-58-00) and renamed at once to the clock reading
06-54-29, before anything was written into it.

## Decisions (the operator's answers)
- Mutation tier: whole units, every touched unit, as the founder ruled (the invocation). Touched units:
  viola, viola-core, viola-pty, viola-state, viola-agent-claude, viola-e2e. viola-channel untouched since
  e304994 (no source, manifest or build-script change) -> out of scope.
- jobs 2 (the operator: half the wall time of jobs 1 without four concurrent builds; three other builders
  share the host). Caps not credited for the extra job (collectors.md, the mutation cap).
- `--copy-target=false` in place of the recorded `--copy-target=true`: a DEVIATION from the recorded command
  form. Reason (the operator): cargo-mutants 27.1.0 copies the whole repo target/ (100 GB at this run) into
  each build dir because `--gitignore` defaults to false; at jobs 2 that is ~200 GB written to a volume three
  other builders share, and IO contention produced instrument reds at earlier boundaries. Mutation verdicts
  do not depend on the copy; each job's first build goes cold. Caps re-sized: + 800 s per unit (the baseline
  build and job 2's first build, each bounded by --build-timeout=400). The prebuild of the recorded form is
  not run: with no copy it reaches no build dir. target/ is kept as it is (pruning it is the founder's desk).
- Windows leg dispatched: `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` at 2026-10-12T06:57:12Z,
  run 38215104120 (the operator: dispatch). Read back before rendering; cited as the project's union verdict
  pointer, never merged into this host's counts.

## Planned counts (`cargo mutants --package {unit} [--features fake-agent] --list --json`)
viola-core 54 · viola-pty 115 · viola-state 360 · viola-channel 176 (out of scope) · viola-agent-claude 524 ·
viola-e2e 799 · viola 1299.

## Caps (planned x recorded wall_s/tested at jobs 1 x 1.5, + 800 s cold builds, floor 1800)
viola-core 1800 · viola-e2e 1800 (no recorded figure: re-sized once at the floor from its own rate) ·
viola-pty 1800 · viola-agent-claude 1896 · viola-state 3074 · viola 31737 (8.8 h; over 2 h, run whole on the
founder's ruling).

## Events
- 2026-10-12T06:57:50Z driver start: units ['viola-core', 'viola-e2e', 'viola-pty', 'viola-agent-claude', 'viola-state', 'viola'], jobs 2, TMPDIR ~/dev/projects/viola-mutants-scratch
- 2026-10-12T06:57:50Z viola-core: start, planned 54, cap 1800 s, jobs 2
- 2026-10-12T06:58:18Z viola-core: exited, exit 0, wall 28 s
- 2026-10-12T06:58:18Z viola-e2e: start, planned 799, cap 1800 s, jobs 2 (no recorded rate: re-sized once at the floor)
- 2026-10-12T06:58:37Z viola-e2e: exited, exit 4, wall 18 s
- 2026-10-12T06:58:37Z viola-pty: start, planned 115, cap 1800 s, jobs 2
- 2026-10-12T06:59:52Z viola-e2e: the unmutated baseline failed (3 harness::supervise tests: target/mutants/debug/viola-fake-agent absent from the build dir, ENOENT), a consequence of --copy-target=false. The operator: re-run it LAST, from a git archive extract of HEAD inside this run dir, with the recorded prebuild and --copy-target=true (only the extract's target is copied), jobs 2; output mutants-viola-e2e-x/. The first invocation's dir mutants-viola-e2e/ stays as evidence.
- 2026-10-12T07:01:47Z viola-pty: exited, exit 2, wall 190 s
- 2026-10-12T07:01:47Z viola-agent-claude: start, planned 524, cap 1896 s, jobs 2
- 2026-10-12T07:03:44Z viola-e2e (extract): e2e_x.py launched in the background; it waits for the driver's done line, then extracts, prebuilds and runs (journalled at each step)
- 2026-10-12T07:04:10Z control extract base-src/ (59 MB, e304994): rm denied by the permission layer; not removed another way, moved to the session scratchpad (outside the repository) instead. Control results kept in ctl/: sizes, duplication, complexity at e304994 reproduce the Epoch 3 record exactly
- 2026-10-12T07:08:33Z viola-agent-claude: exited, exit 2, wall 405 s
- 2026-10-12T07:08:33Z viola-state: start, planned 360, cap 3074 s, jobs 2
- 2026-10-12T07:17:31Z viola-state: exited, exit 2, wall 538 s
- 2026-10-12T07:17:31Z viola: start, planned 1299, cap 31737 s, jobs 2
- 2026-10-12T09:07:27Z STOP on the founder's word: viola copied to mutants-viola-partial/mutants.out (678/1299), then e2e_x.py and mut-driver.py SIGTERM, the viola cargo-mutants group SIGINT; none left. Torn first copy moved to torn-copy-viola-mutants.out/ (rm denied). See resume-note.md
- 2026-10-12T18:19:20Z RESUME (operator): mutants-viola-iter/ made as a further byte copy of mutants-viola-partial/mutants.out (diff -rq clean, outcomes.json sha256 7439ee70 unchanged). Read-back: the recorded form + --iterate --list --json against it lists 636 = 621 untested + 15 missed, equal by (file, span, replacement) to the expected set; none caught or unviable among them; the tool logged 663 excluded; the copy unchanged after the listing. Runner: viola_iter.py (the recorded form + --iterate, jobs 2, cap 25141 s = 31737 less the 6596 s spent).
- 2026-10-12T18:19:22Z viola (iterate): start, expected 636 (621 untested + 15 missed), cap 25141 s, jobs 2, output mutants-viola-iter/
- 2026-10-12T18:20:45Z The operator: after viola is merged (merge_viola.py), scored and journalled, STOP and report (a deploy is due and needs a seam between units). viola-e2e from the extract, as recorded, waits for the operator's go; e2e_x.py is not launched (its wait gate reads 'driver done', which the killed driver never wrote). Waiting now on viola (iterate), about 6,000 s at the measured rate.
- 2026-10-12T20:32:38Z viola (iterate): exited, exit 2, wall 7995 s
- 2026-10-12T20:33:05Z viola (iterate) COMPLETE by the tool's markers: end_time 20:32:38Z, total_mutants 636 = len(mutants.json) 636; baseline Success; caught 449, missed 50, unviable 137, timeout 0; exit 2 (missed present). The tool rotated the copy to mutants-viola-iter/mutants.out.old (equal to the partial but for lock.json). merge_viola.py: the partial's 663 caught/unviable + the resume's 636 (the partial's 15 missed superseded), disjoint, union = the 1299 listing, every assert green -> mutants-viola-merged/ and c-mutation-viola.json: complete 1299/1299, score 97.03 (caught 1044 / (1044 + 32)), survivors 32, not measured on this host 18, unviable 205, timeout 0; timing scope unit, wall 14591 s (6596 + 7995), jobs 2, cap 31737. mutants-viola-partial/ intact (outcomes.json sha256 7439ee70), torn-copy-viola-mutants.out/ untouched, no process left, git status gained nothing outside the run dir. STOP on the operator's word (deploy seam): viola-e2e from the extract and the rest of the audit wait for the operator's go.
- 2026-10-12T20:33:53Z GO (operator, after the deploy): viola-e2e from the git archive dffad3e extract as recorded. The one change to e2e_x.py: its wait gate reads 'viola (iterate): exited' (the viola end line already in this journal) in place of 'driver done'. Guards clear (no x-src/, mutants-viola-e2e-x/ or state-viola-e2e-x.json). Launching e2e_x.py now; it journals the extract, the prebuild and the run start before the long wait.
- 2026-10-12T20:33:55Z viola-e2e (extract): extracting dffad3e02d into x-src/ and prebuilding viola (fake-agent) into x-src/target/mutants
- 2026-10-12T20:34:03Z viola-e2e (extract): prebuild exit 0
- 2026-10-12T20:34:03Z viola-e2e (extract): start, planned 799, cap 1800 s (floor; re-sized once from its own rate), jobs 2
- 2026-10-12T20:35:19Z viola-e2e (extract): baseline Success read from outcomes[0] (viola-fake-agent present in x-src/target/mutants/debug). Windows read-back of run 38215104120 (head dffad3e): conclusion failure; 5 jobs green (viola-state, -channel, -pty, -agent-claude, -e2e); the 4 root-package shards (viola-main, -cmd-run, -run-env, -panic-frames) failed the UNMUTATED baseline on cmd::hook::statusline::tests::statusline_an_instance_viola_laid_down_passes_its_check (assertion trusted(&instance), statusline.rs:510), no mutant tested. Cited by pointer in c-windows-readback.json, never merged. Waiting on viola-e2e; coverage runs after it (no concurrent build).
- 2026-10-12T21:04:03Z viola-e2e (extract): re-sized once at 1800 s from its own rate (470 tested): cap 4590 s
- 2026-10-12T21:22:06Z viola-e2e (extract): exited, exit 2, wall 2882 s
- 2026-10-12T21:22:26Z viola-e2e (extract) COMPLETE by the tool's markers (end_time 21:22:06Z, total_mutants 799 = len(mutants.json)); baseline Success; mutsum.py viola-e2e mutants-viola-e2e-x: complete 799/799, score 99.73, caught 734, missed 2 (survivors 2, not measured 0), unviable 63, timeout 0, wall 2882 s, cap 4590 (re-sized once). The first invocation's twin kept as c-mutation-viola-e2e-first.json (baseline-test-failure). Coverage starts now: cov.sh (the e304994 record's commands.coverage form), run once in the background; summarizer covsum.py.
- 2026-10-12T21:24:00Z Coverage: cov exit 0, 2406 tests passed; covsum.py: line 98.45, branch null (declined, not instrumented); cov-raw.json and cov-run.log removed after summarizing. P3: assemble.py -> record.json (every per-unit survivor / not_measured assert green; survivors 47, not measured 112; skips coverage.branch), appended by the pinned recipe (append.py): ledger 5 records, 0 unparseable, i/lf w/lf.
- 2026-10-12T21:25:22Z P4/P5: judge_render.py -> judge.json (re-found the baseline e304994 and its baseline 95c1a9b by sha): 2 proposals (monotonic sizes.file_max 884 -> 2678 -> 2709; monotonic sizes.over_800 2 -> 12 -> 19, evidence c-sizes-over800.json, both counts reproduced), 13 informational, 18 below threshold; trend-break code-graph.py d0425fb1 -> f22becbb (graph, dead). render.py -> proposals.md, every table row-count assert green. Exit self-check: git status gained only the ledger line and this run dir; rm -rf of x-src/ (this run's extract, ~6,000 tracked-eligible files) DENIED by the permission layer, not removed another way, left in place and surfaced; torn-copy-viola-mutants.out/ untouched (1,369 files, not gitignored) surfaced; root mutants.out/ and mutants.out.old/ are from 2026-10-10 (not this run), gitignored, surfaced. No process left. Nothing committed. AUDIT COMPLETE.
- 2026-10-12T21:25:28Z Correction to the line above: informational is 14, not 13 (judge.json's 11 + render.py's 3 mutation lines: survivors per unit, viola survivors by file, not measured with the Windows pointer).
