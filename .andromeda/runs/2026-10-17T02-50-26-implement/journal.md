# Journal — implement 2026-10-17-cli-machine-contract

One line per long run, written before the wait.

- 2026-10-17T02:54:47Z · Part A step 2 · entry 8 (`stamped_home_concurrent_`) firing 1 of at most 20, on the unchanged
  product tree (only the new concurrent case in tests/cli_verify.rs and the new typed.rs unit case added) — backgrounded.
  Read: green, 7.01 s entry, the case 4.273 s.
- 2026-10-17T02:55:24Z · Part A step 2 · entry 8 firings 2-20 (19 calls of `gate.py run --entry 8`, one backgrounded
  loop), same unchanged product tree; each firing's log kept as `8.<k>.log`. Read: 19 green (8.2-8.20), the case
  4.269-4.285 s; with firing 1, 20 of 20 green, no red.
- 2026-10-17T02:57:43Z · Part A step 4 · entry 8 firings 21-40 over the fix (`count_of` counts only a capture whose
  bytes landed, typed.rs), one backgrounded loop. Read: 20 green (8.21-8.40), the case 4.274-4.285 s. Then the unit
  witness red before / green with / red reverted / green restored (evidence/ci-red.md §5).
- 2026-10-17T03:03:40Z · Part B written (steps 5-8) · entry 10 (`run --integration`, the layer whole), backgrounded,
  to read what the canonical home moves before Part C. Read: red, 337/338, one case
  (`cli_instance_state::run_rewrites_the_settings_override_each_start`, an `absolute` home against the canonical
  `pinned_bin`) — fixed to the resolved home.
- 2026-10-17T03:14:42Z · Parts C-E written (steps 9-17) · entry 9 (the chunk's binaries), backgrounded. Read: green,
  139 tests, 54.11 s. Then three remove-the-guard runs (evidence/guards.md).
- 2026-10-17T03:16:29Z · P2 · the whole block (`gate.py run`, entries 1-18; 19-21 are the operator's), backgrounded;
  entry 18 is `pre-push` (`run --coverage`, timeout 7200 s). Read: green 15, red 3 — 3 (Windows lint: imports used
  only by the cfg(unix) link case), 11 and 18 (one case, `path1_run_start_one_home_in_order`: `resolved` of a home not
  created yet). Both causes fixed in the test code.
- 2026-10-17T03:21:07Z · P2 iteration 2 · `--only 3,11`, then the whole block again, backgrounded. Read: `--only 3,11`
  green; the block green 18, red 0, not-run 3 (the operator's).
- 2026-10-17T03:27:04Z · operator pass · entry 18 (`pre-push`) re-run on the tree the pre-CI commit carries,
  backgrounded.
  Read: green, 75.77 s. Then hygiene clean, the pre-CI commit e248533, the push (entry 20, history moved f6bcce5b→e2485333).
- 2026-10-17T03:28:52Z · operator pass · entry 21 (`ci.py conclusion --sha HEAD --wait 1800`), fired as written, backgrounded.
  Read: verdict red · test (windows-2025) — one unit, canonical_home_resolves_an_existing_home_and_keeps_a_new_one_absolute (src/cmd/mod.rs:347, oracle took the raw \\?\ form); test (ubuntu-latest) green. Fixed in the test; units and lints 1-3 green.
- 2026-10-17T03:38:02Z · operator pass · entry 18 (`pre-push`) before the fix commit, backgrounded.
  Read: green, 74.38 s. Hygiene refused 1 (a runner drive path quoted in evidence/operator-pass.md), respelt by hand, then clean.
- 2026-10-17T03:49:19Z · operator pass · fix commit f738cbc, pushed (entry 20, e2485333→f738cbc0); entry 21 on it: verdict green, 15/15, ci#38999642102.
