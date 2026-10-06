# Operator pass — 2026-10-06-local-command-send-outcomes

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual"). The block had read 21 green, 0 red on the final
tree in its one full run (entries 1 to 21; entries 22 to 24 are this pass). No live round exists in this block.

## Before the pass — `pre-push` (entry 21) on the uncommitted tree, 2026-10-06T23:28:06Z
- `bash scripts/agent-run.sh pre-push` → exit 0 after 43 s: `"ok":true`, `"stage":"linux-tests"`; coverage
  1662/1662, playwright 1/1, `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓. Load average at its start: 1.43; no other build was running on the host.
- The watch item (a coverage merge red over green tests): not seen. Two green readings in this chunk, entry 21
  in the block (42.5 s) and this one.

## Entry 22 — hygiene (by hand)
- First read, 2026-10-06T23:28:53Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 47 (runs 40 · evidence 3 · inputs 4)`. The one row:
  `evidence/rewritten-path-warning-control.md`, form `drive`, 2 hits. Both were the test's own literal, the
  Git-for-Windows rewritten form of `/clear`, quoted from `tests/cli_send.rs`: a drive path by shape, no path of
  this host. The record was reworded to describe the literal without spelling it; nothing else changed in it.
- Re-read after the rewording and after this file was written, 2026-10-06T23:29:17Z: exit 0,
  `hygiene: clean — read 48 (runs 40 · evidence 4 · inputs 4)`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓.
  Read once more after this line was added, before the commit: clean, the same counts.

## The pre-CI commit and entry 23 — the push, 2026-10-06T23:29:30Z
- `8e66926` `chore(2026-10-06-local-command-send-outcomes): operator pre-CI commit, for the run this chunk's
  verdict reads` (the whole tree, 66 files: the phase's products, this chunk's ten source and test files, its
  evidence, the two run dirs and the bookkeeping the tree carried).
- Entry 23: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0,
  `11c77f7..8e66926  HEAD -> build/viola-0.1.0`.

## Entry 24 — the CI conclusion: GREEN
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `8e6692679255 verdict: green · checks 15/15 · wall 456 s ·
  runs ci#37546848541 completed/success` (polled 16× over 465 s, 2026-10-06T23:29:36Z to 23:37:21Z). Atoms:
  `exit 0` ✓, `contains verdict: green` ✓. No fix commit was needed.
- The fifteen jobs, each `success`: `test`, `lint`, `perf` and `release` on the three OSes (windows-2025,
  macos-latest, ubuntu-latest), `supply-chain`, `msrv`, `fuzz-replay`.
- The three `test` legs: ubuntu-latest `1662 tests run: 1662 passed (5 slow)`, macos-latest `1658 passed
  (4 slow)`, windows-2025 `1697 passed (4 slow)`.

## The new cases on CI, from the three `test` jobs' logs of ci#37546848541
Seconds. The kill is 20 s for `binary(cli_send)`, `binary(cli_verify)` and `binary(cli_fake_agent)`.

| test | ubuntu-latest | macos-latest | windows-2025 | here |
|---|---|---|---|---|
| `send_under_the_paste_hint_on_a_verified_cli::case_1_hint` (3 000 ms hold) | 7.793 | 7.693 | 7.714 | 7.437 |
| `send_under_the_paste_hint_on_a_verified_cli::case_2_no_hint` | 5.137 | 5.140 | 5.015 | 4.747 |
| `send_clear_on_a_verified_cli_is_confirmed_by_its_new_session` | 4.649 | 4.671 | 4.607 | 4.219 |
| `send_window_local_command_is_not_presumed_delivered` | 0.660 | 0.517 | 0.577 | 0.643 |
| `send_window_slash_text_off_the_list_is_not_delivered` (waits the 10 s window out) | 10.615 | 10.551 | 10.490 | 10.623 |
| `send_rewritten_path_argument_draws_the_warning` | 0.245 | 0.024 | 0.047 | 0.010 |
| `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal` | 3.776 | 3.798 | 4.180 | 3.656 |
| `fake_agent_tag_turn_screen_draws_the_named_screen_after_the_tag_turn_alone` | 0.012 | 0.050 | 0.075 | 0.013 |

- The `hint` case's longest reading is 7.793 s, on the ubuntu coverage leg: under half its 20 s kill, and under
  the 10 s kill it would meet under the nextest `mutants` profile. No timing red was read and no bound moved.
- `send_window_local_command_is_not_presumed_delivered` no longer waits a window out: it reads 0.5 s to 0.7 s
  where it read about 10.6 s before this chunk. It keeps its name, as planned.
- The fake-agent case reads its marker rows off the test's own PTY. It passed on windows-2025 too, where that
  stream comes through ConPTY.

## The fix commit — `/remote-control` read under `--json`, on the operator's word
The implement report named one gap: `verification-matrix.json#v1-29` says `/clear` on an unverified CLI and
`/remote-control` "each exit 0 with `{confirmed:false, detail:"unconfirmable", cursor}`", and
`send_window_local_command_is_not_presumed_delivered` read `/remote-control` in human mode only, its document
pinned at unit level. The operator's word after the report: "close the gap you named, by the test and not by the
wording: … add a /remote-control send under --json on the same wrapper and assert its exit 0 and its one document
with confirmed false, detail unconfirmable and the cursor … Keep the human-mode assertions. Then the named entry,
pre-push, one fix commit, the push and the ci.py conclusion read (leg=operator)" (the operator, 2026-10-06).

- The test now sends `/clear`, then `/remote-control`, each under `--json` on the one wrapper, and asserts for
  each: exit 0, an empty stderr, one document equal to
  `{"v":1,"ok":{"confirmed":false,"detail":"unconfirmable","cursor":L}}`, the two records after `L`, and one more
  prompt in the receipt with that text, `submit` `local-command`. The human-mode `/remote-control` send and its
  line stay, and the role-log read covers the three cursors. `tests/cli_send.rs` only; no product line moved.
- The targeted run, 2026-10-06T23:39:33Z to 23:41:09Z (`gate.py run … --only 1,2,7,21`): entry 1 `cargo fmt`
  green; entry 2 `cargo clippy` green; entry 7, the four named cases, green in 13.18 s (`"passed":4,"failed":0`;
  the extended case 0.535 s); entry 21 `pre-push` green in 79.55 s (`"ok":true`, `"stage":"linux-tests"`,
  coverage 1662/1662, playwright 1/1, no breaches).
- The host was not quiet for that run: load average 107.61 at its start and 100.85 at its end, another build
  running. Nothing read red, so no stalled-start re-read was needed; `pre-push` took 79.55 s where it took 43 s on
  the quiet host. The watch item was not seen: a third green reading.
- Hygiene before the commit, the commit, the push and the CI read: below.

## Not done here
- The wrap: the flip, the amendments and the chunk commit are `/andromeda-wrap-session`'s.
