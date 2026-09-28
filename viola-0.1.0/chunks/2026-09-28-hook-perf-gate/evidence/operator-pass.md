# Operator pass — plan entries 29–32

Driven by /implement on the operator's word (2026-09-28: "run the operator pass now, entries 29-32, then fold any
CI red into this chunk"), each entry by hand in its exact `run`. `{tools_dir}` stands for the andromeda-tools
scripts directory the plan's entries name by absolute path.

## 29 — hygiene (`python -X utf8 {tools_dir}/gate.py hygiene`)

The first read refused two files of the phase run dir, `.andromeda/runs/2026-09-28T05-14-43-phase/dryrun.txt` and
`dryrun2.txt` (P1, drive paths): phase P4's `gate.py run --dry-run` listings carried the resolved shell, the gate log
dir and the tools path. On the operator's word (with the overseer's), lines 2, 4, 35 and 38 of each were rewritten
to placeholders, every other byte kept (binary write, one anchor per edit asserted unique, CRLF count and line count
unchanged, both junctions read back):

- line 2 `shell <abs>\bash.exe (from $SHELL; -c, non-login)` → `shell bash.exe (from $SHELL; -c, non-login)`
- line 4 `logs <abs temp>\andromeda-gate\…\run-…Z · bound: GNU timeout` → `logs <OS temp>/andromeda-gate/2026-09-28-hook-perf-gate/run-…Z · bound: GNU timeout`
- lines 35 and 38 `python -X utf8 <abs>/andromeda-too… (N chars)` → `python -X utf8 {tools_dir}/andromeda-too… (N chars)`

Exit 0 on every read; the final verdict line, read after this file was written and immediately before the pre-CI
commit, is recorded under §Final hygiene below.

## 30 — pre-push (`bash scripts/agent-run.sh pre-push`)

On the uncommitted tree (sync head `85ae5aa`, 55 files). Exit 0. Atoms: `"cmd":"pre-push","ok":true` ✓ ·
`"stage":"union"` ✓. Union gate `{"ok":true,"breaches":[]}`; legs `ubuntu-latest` and `windows-2025` both
`verdict:"counted"`, `tested:52`, `ok:true`; Linux and Windows coverage gates `ok:true`, no breach; zero `MISSED`
lines; the WSL VM `terminated:true`.

## Final hygiene

`hygiene: clean — read 35 (runs 31 · evidence 4) · trails 11 not read · binary 0 not read by P1`

Then the operator pre-CI commit `5a693d6` (`chore(2026-09-28-hook-perf-gate): operator pre-CI commit, for the run this
chunk's verdict reads`), `git add -A` over a tree whose hygiene read the line above.

## 31 — push (`git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0`)

Exit 0: `85ae5aa..5a693d6  build/viola-0.1.0 -> build/viola-0.1.0`; 0 ahead of the upstream after it.

## 32 — CI read (`python -X utf8 {tools_dir}/ci.py conclusion --sha HEAD --wait 5400`)

Exit 0. Atoms: `exit 0` ✓ · `contains verdict: green` ✓.

```
ci v1.0 · 15ed584e
repo Turbolet85/viola (the push remote `origin`) · polled 36× over 1087 s
5a693d655913 verdict: green · checks 18/18 · wall 1067 s · runs ci#36390764600 completed/success
runs: ci#36390764600 push completed/success
```

18 check-runs = the plan's forecast (9 jobs). Per job from the run's own step records (`gh run view 36390764600`):
`perf (windows-2025)`, `perf (macos-latest)`, `perf (ubuntu-latest)` success, each with `Perf rows`, `G2 zero panics`
and `Gate verdict` success; `test` on all three OSes success with `G2 zero panics` (probe then check) and
`Gate verdict` success. No red to fold into the chunk.
