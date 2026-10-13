# Operator pass — 2026-10-13-epoch-4-cleanup

On the operator's word in this session's `/andromeda-implement` arguments: "Run the operator pass with the ci.py
conclusion read (leg=operator) as usual." Lines are copied verbatim, the tools' headers left out.

## Entry 21 re-fired before the pre-CI commit

`gate.py run … --only 21` (firing 3, journal line 3) on the final uncommitted tree:

```
 21 probe       green · exit 0 · 67.13s · 311675 B → 21.3.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2406 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 22 — hygiene, fired as written before the pre-CI commit

The first read refused two files of the phase's run dir, each quoting the CI runner's scratch path as a Windows drive
path:

```
P1 .andromeda/runs/2026-10-13T07-07-28-phase/obs-history.md:9 ×1 · drive
P1 .andromeda/runs/2026-10-13T07-07-28-phase/tests-history.md:5 ×1 · drive
hygiene: refused 2 files — P1 2 · P2 0 · P3 0 · read 46 (runs 40 · evidence 2 · inputs 4) · trails 12 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Each quoted path was rewritten as `<repo parent>/viola-mutants-scratch` on the runner (the spelling plan.md and
scope.md use), nothing else in either line changed. The re-read:

```
hygiene: clean — read 46 (runs 40 · evidence 2 · inputs 4) · trails 12 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-epoch-4-cleanup): operator pre-CI commit, for the run this chunk's verdict reads`, the whole tree
(`git add -A`), on `build/viola-0.1.0`. The record above rides it; the sections below are written after it and ride
the wrap commit.

`3b75dc729a10`. Its body names the two rewritten lines.

## Entry 23 — the push, fired through `gate.py run --operator 23`

```
operator entry 23 · history tripwire: git
 23 probe       green · exit 0 · 2.24s · 90 B → 23.log · history moved: refs/remotes/origin/HEAD d9ad9f1f→3b75dc72; refs/remotes/origin/build/viola-0.1.0 d9ad9f1f→3b75dc72 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

After it, `git rev-list --count @{u}..HEAD` read 0.

## Entry 24 — the CI read for the pre-CI commit: green

`ci.py conclusion --sha HEAD --wait 1800`, fired as written:

```
repo Turbolet85/viola (the push remote `origin`) · polled 27× over 809 s
3b75dc729a10 verdict: green · checks 15/15 · wall 806 s · runs ci#38375200217 completed/success
runs: ci#38375200217 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

The Windows witness, read from the `test (windows-2025)` job log of ci#38375200217 (job 115221715737):

```
PASS [   0.035s] (1993/2403) viola-e2e harness::run::mutants::scratch::tests::wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not
Summary [ 232.934s] 2403 tests run: 2403 passed (5 slow), 0 skipped
```

Both of its assertions held on the runner: the tree inside the scratch `wipe` laid down is accepted by
`check_instance`, and the same tree in a plain-created sibling on the same drive is refused `Writable` — so the
accepted reading is not vacuous there. The root package's Windows mutation baseline itself is read only at the next
boundary's `windows-mutants.yml` dispatch (no dispatch inside this chunk, `inputs#I1`).
