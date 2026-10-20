# Operator pass — 2026-10-20-epoch-5-cleanup-ii

The builder session ran this pass on the operator's word, given at the `/andromeda-implement` invocation (`inputs#I4`,
2026-10-20T15:48Z): «Then the operator pass as usual; stop before the wrap.» It follows implement's run
(`.andromeda/runs/2026-10-20T15-48-36-implement/`), whose outcome is green on the dialog-15 answers (`inputs#I5`).

## `pre-push` on the uncommitted tree (entry 27)

It fired in implement's last block firing, on the uncommitted tree, after the last code edit. Only evidence, the outcome
file and two phase-run respells were written after it:
`27 probe       green · exit 0 · 96.78s · 392118 B → 27.2.log · bash scripts/agent-run.sh pre-push`.

## Entry 28 — `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`

Fired as written on the uncommitted tree, before 2026-10-20T18:42:59Z (the `date -u` read right after it).

- **First firing:**
  ```
  P1 .andromeda/runs/2026-10-20T10-26-22-phase/p5-card-1.md:75 ×3 · tmp
  P1 .andromeda/runs/2026-10-20T10-26-22-phase/p5-prepush-red.md:10 ×3 · tmp
  hygiene: refused 2 files — P1 2 · P2 0 · P3 0 · read 63 (runs 48 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
  ```
- **The rewrite (the letter's, §Hygiene):** the 3 + 3 temp-dir paths in the phase run's two P5 records were respelled
  `/tmp/<segment>` → `$TMPDIR/<segment>`, the run-dir spelling of the OS temp dir. Nothing else in those files
  changed. The bare `/tmp` mount mentions do not fire and stay as written.
- **Second firing:**
  `hygiene: clean — read 63 (runs 48 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- **Third firing**, after this file was first written, so that it is read too:
  `hygiene: clean — read 64 (runs 48 · evidence 9 · inputs 7) · …`.

## The pre-CI commit

`2cdfb0dd` — `chore(2026-10-20-epoch-5-cleanup-ii): operator pre-CI commit, for the run this chunk's verdict reads`,
made with `git add -A` on top of `844de976` on `build/viola-0.1.0`. The tree was clean after it.

## Entry 29 — `python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/plan.md --operator 29`

The entry's line and the call's summary, verbatim (the header left out):

```
operator entry 29 · history tripwire: git
 29 probe       green · exit 0 · 2.41s · 92 B → 29.log · history moved: refs/remotes/origin/HEAD 844de976→2cdfb0dd; refs/remotes/origin/build/viola-0.1.0 844de976→2cdfb0dd · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 30 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 29
```

The history move is the push the entry exists to make.

## Entry 30 — `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`

Fired as written, in the background, and finished before 2026-10-20T19:07:00Z (the `date -u` read after it). Its output,
verbatim:

```
ci v1.3 · 49546645
repo Turbolet85/viola (the push remote `origin`) · polled 45× over 1368 s
2cdfb0dd8b1b verdict: red · checks 15/15 · wall 1335 s · runs ci#39477233397 completed/failure
runs: ci#39477233397 push completed/failure
run failed: ci#39477233397 failure
not success: test (macos-latest)=cancelled · release (macos-latest)=cancelled
```

**The red, read to its cause (not this chunk's: no step ran).** Both cancelled jobs were created at 18:43:26Z and
cancelled at 18:58:28Z. Each ran 0 steps and was given no runner (`runner_name` empty). Each job's annotations read
`failure: The job was not acquired by Runner of type hosted even after multiple attempts`, with three `notice: Due to
capacity constraints, jobs targeting macOS arm64 runners may experience longer queue times.` lines. The other 13 checks
are `success`:

- every `test`, `lint`, `perf` and `release` job on `ubuntu-latest` and `windows-2025`;
- `lint (macos-latest)` (which started only at 18:53:37Z) and `perf (macos-latest)`;
- `msrv`, `fuzz-replay` and `supply-chain`.

So the macOS legs of the OS-sensitive cases the plan names under entry 30 (the folded relied count, `prepare(root,
true)`, the serve re-exec, the dropped-session case, the replay) are **not yet read on macOS**: no test ran there. The
red stays open until those two jobs run.

**Not done here:** a re-run of the two jobs (`gh run rerun 39477233397 --failed`). It is an outward CI action outside the
pass's entries, so it waits for the operator's word. The pass stops here, before the wrap (`inputs#I4`).

## Entry 30 — attempt 2, re-read at the wrap

The overseer re-ran the two cancelled jobs (the wrap directive, `inputs#I6` item 2). The wrap's P7 re-read the run with
the entry's own read-only query, `ci.py conclusion --sha 2cdfb0dd` (no `--wait`), at 2026-10-20T20:27:59Z, verbatim:

```
ci v1.3 · 49546645
repo Turbolet85/viola (the push remote `origin`)
2cdfb0dd8b1b verdict: green · checks 15/15 · attempt 2 · wall 2863 s · runs ci#39477233397 completed/success
runs: ci#39477233397 push completed/success
re-run: ci#39477233397 attempt 2 — each check is its latest attempt's; an earlier attempt's result is unread
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

The final HEAD's run reads green on every check; the macOS legs named above are read (`test (macos-latest)` job
118828554674, attempt 2).
