# Operator pass — 2026-10-17-cli-machine-contract

Run on the operator's word in this session's `/andromeda-implement` invocation ("Run the operator pass with the
ci.py conclusion read (leg=operator); stop ready for the wrap" — the operator, 2026-10-17).

## Entry 18 — `pre-push`, re-run before the pre-CI commit
```
 18 probe       green · exit 0 · 75.77s · 356125 B → 18.3.log · bash scripts/agent-run.sh pre-push
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

## Entry 19 — hygiene, fired as written
```
hygiene: clean — read 52 (runs 46 · evidence 2 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit and entry 20 — the push
- `e248533 chore(2026-10-17-cli-machine-contract): operator pre-CI commit, for the run this chunk's verdict reads`
```
operator entry 20 · history tripwire: git
 20 probe       green · exit 0 · 2.43s · 90 B → 20.log · history moved: refs/remotes/origin/HEAD f6bcce5b→e2485333; refs/remotes/origin/build/viola-0.1.0 f6bcce5b→e2485333 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```

## Entry 21 — the CI read of `e2485333`, fired as written
```
e24853337527 verdict: red · checks 15/15 · first-fail +501 s test (windows-2025) · runs ci#38998933306 in_progress/-
failed 1: test (windows-2025) (failure)
running 1: oldest test (macos-latest) 498 s
```
- The one failure (job 117297543004): `cmd::tests::canonical_home_resolves_an_existing_home_and_keeps_a_new_one_absolute`,
  `panicked at src\cmd\mod.rs:347:9` — left: the runner's temp dir in the plain drive form (the product's, correct),
  right: the same dir behind the `\\?\` verbatim prefix (the test's oracle took the raw canonical form). A test-oracle error on
  Windows only; the product returned what architecture asks.
- `test (ubuntu-latest)` — the leg `ci#38994749835` failed on — **completed success** on `e2485333`
  (`gh run view 38998933306 --json jobs`), 13 of 15 checks success, `test (macos-latest)` still running at the read.

## The fix, and entry 18 before its commit
- `src/cmd/mod.rs` test module: the oracle is the plain drive form (`plain_drive`, pinned by its own literal table),
  with an assertion that it holds no `\\?\` prefix. `canonical_home_` / `plain_drive_` units 10/10 green; entries
  1-3 (fmt, clippy, the Windows-target clippy) green.
```
 18 probe       green · exit 0 · 74.38s · 356125 B → 18.4.log · bash scripts/agent-run.sh pre-push
```

## The fix commit, entry 20 again, and entry 21 on the final HEAD
- `f738cbc fix(2026-10-17-cli-machine-contract): the canonical-home unit's oracle is the plain drive form on Windows`
  (hygiene clean before it, after one drive path in this file was respelt by hand).
```
 20 probe       green · exit 0 · 1.55s · 90 B → 20.2.log · history moved: refs/remotes/origin/HEAD e2485333→f738cbc0; refs/remotes/origin/build/viola-0.1.0 e2485333→f738cbc0 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```
```
f738cbc0f307 verdict: green · checks 15/15 · wall 542 s · runs ci#38999642102 completed/success
runs: ci#38999642102 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```
