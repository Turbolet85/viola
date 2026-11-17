# CI red read after the wrap's commit — 2026-11-17-standing-live-rig

- Commit 668854a6 (the wrap: records, renders, harness leaves, handoff; no source file), pushed 1b22388f..668854a6.
- `ci.py conclusion --sha HEAD --wait 1800`: `668854a685c1 verdict: red · checks 7/7 · first-fail +443 s test
  (ubuntu-latest) · wall 443 s` · `runs: ci#44048204364 push completed/failure` · `failed 1: test (ubuntu-latest)`.
- Its parent's source tree is the same: 1b22388f read `verdict: green · checks 7/7`, ci#44040049854.
- The coverage run: `Summary [ 235.886s] 3484 tests run: 3458 passed (4 slow), 26 failed, 0 skipped`. Every failure reads
  the shared stamped fixture home: `stamps written by verify: Os { code: 2, kind: NotFound … }`
  (cli_fake_agent), `pinned: NotFound` (cli_verify stamped_home_copies_share_no_file_with_the_stamp_or_each_other),
  `stamps: NotFound` (stamped_home_reuses_one_stamp_for_every_home), `the fixture home is stamped`
  (cli_instance_state), and booted wrappers refusing `unverified-cli` (exit 12, expected 0: cli_answer, cli_send …).
  The first failure is at 08:38:31Z; the `viola-e2e` tests (rig_rehearsal among them) ran from 08:41:36Z.
- Local, same HEAD: `bash scripts/agent-run.sh run --integration --filter 'binary(cli_answer)'` → `"ok":true`,
  9 passed.
- Cause: not found. Not re-run (a green re-run would not close it). Pinned open as a CARRY on «A booted wrapper does
  not outlive its test», whose own work is the booted tests' shared fixtures.
- Log read through `gh run view 44048204364 --log-failed` (kept in the session scratchpad, not here).
