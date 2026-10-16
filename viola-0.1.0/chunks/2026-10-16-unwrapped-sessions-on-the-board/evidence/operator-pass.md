# Operator pass — 2026-10-16-unwrapped-sessions-on-the-board

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator); stop ready for the wrap"). Times are `date -u` or the gate
trail's own; the pass ran on 2026-10-16. No mutation run and no workflow dispatch is part of it. The eight live
`claude` starts of the chunk were made before it (`round-204738Z.txt`, `live-p4.md`); the pass starts none.

## Before it — the block

Every entry of the block read green in its latest firing on HEAD `92264a7f254c` plus the chunk's uncommitted edits
(the implement run's trail): entries 1, 2 and 11 at 21:08Z after the last test edit (`tests/cross_list_rows.rs`,
the Windows terminal case), entries 3–6, 10 and 12–20 in firing 5, entries 7–9 in the live round. One red in the
run stays open with its two-sided readings: `cli_send
send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, once in firing 5's entry 11, reproduced on
neither tree in 20 runs after it (the implement outcome, `outcome-{marker}.md` in the run dir).

## Step 1 — `pre-push` (entry 21) on the uncommitted tree, 21:15:14Z to 21:16:37Z

```
 21 probe       green · exit 0 · 82.67s · 344801 B → 21.4.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2645 of 2645 (`Summary … 2645 tests run: 2645
passed (8 slow), 0 skipped`), doctest 0 of 0, `gate` `"ok":true`, no breaches. No source or test file changed
after it.

## Step 2 — entry 22, hygiene (fired as written), 21:16:51Z

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
`hygiene: clean — read 58 (runs 47 · evidence 7 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0
host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓.

## Before the pre-CI commit, 21:17:07Z

- Hygiene re-read with this file among the evidence: `hygiene: clean — read 59 (runs 47 · evidence 8 · inputs 4)`,
  exit 0. 0 ahead of the upstream; the remote branch head read live (`git ls-remote`) at `92264a7f254c`.
- This section and everything below it was written after the commit; it rides the next commit.

## Step 3 and step 4 — the pre-CI commit and entry 23, the push, 21:17:21Z to 21:17:40Z

- `de9ee3f` `chore(2026-10-16-unwrapped-sessions-on-the-board): operator pre-CI commit, for the run this chunk's
  verdict reads` at 21:17:21Z (the whole tree, 121 files). `git status --short` read empty after it.
- Entry 23, fired once through the gate tool with no run dir (`--operator 23`), 21:17:37Z to 21:17:40Z; a
  `--dry-run` of the same call fired nothing and printed the same tripwire line:

  ```
  operator entry 23 · history tripwire: git
   23 probe       green · exit 0 · 2.54s · 90 B → 23.log · history moved: refs/remotes/origin/HEAD 92264a7f→de9ee3f9; refs/remotes/origin/build/viola-0.1.0 92264a7f→de9ee3f9 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
  entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
  ```

  A fast-forward of the two remote-tracking refs; no local branch, tag or stash moved; no force push.

## Step 5 — entry 24, the CI read (fired as written), 21:17:45Z: red

`ci.py conclusion --sha HEAD --wait 1800` → exit 0: `de9ee3f96eeb verdict: red · checks 15/15 · first-fail +520 s
test (windows-2025) · wall 520 s`, run `ci#38968360744` push completed/failure, polled 18 times over 529 s; one job
failed, `test (windows-2025)`. Atoms: `exit 0` ✓, `contains verdict: green` ✗.

The job's log (`gh api …/jobs/117200345162/logs`): `2633 tests run: 2626 passed (6 slow), 7 failed`, all seven in
`viola::cli_verify` — `verify_dispatch_error_keeps_the_chain_in_the_detail_file`, the four
`verify_record_refuses_a_dirty_kept_row` cases, `verify_record_screens_keep_only_signature_rows`,
`verify_record_writes_the_scrubbed_spine_variants_and_screens` — each reading `unable: a recorded fixture is not
clean: Agents.recorded-1.json absolute-path`.

**Cause (this chunk's):** the fake agent's registry row carried its real working directory; `verify --record` keeps
Run B's own row of its agents read, and on `windows-2025` the checkout (`D:\a\…`) lies outside the user home, so
the scrub leaves it absolute and `--record` refuses it — correctly. On the Unix runners and the Linux dev host the
checkout lies under the home, so the scrub rewrote it and nothing failed.

**Fix:** the fake's row replays the launch SessionStart fixture's recorded `cwd` (already scrubbed), `~` when the
recording holds none — as every other value the fake hands out is a recorded one. Guard:
`contract_fake_agent_drift` asserts the row's `cwd` equals the recorded launch `cwd` (with the real `current_dir`
put back at the call site the case read red: `the row replays the recorded cwd, never this host's directory`;
restored, green), and the fake agent's unit case `launch_session_replays_the_recorded_id_and_cwd`.

## The fix, before its commit

- The whole block on the fix (21:26Z–21:31Z): `entries 24 · green 18 · red 0 · recorded 0 · timeout 0 ·
  not-run 6`; entry 21, `pre-push`: `"ok":true`, coverage 2646 of 2646, `gate` no breaches. The Windows target's
  clippy, check only: exit 0.

## The fix commit and its CI read, 21:32:35Z to 21:39Z: red on `ubuntu-latest`

- `3c1c54d` `fix(2026-10-16-unwrapped-sessions-on-the-board): the fake agent's registry row replays the recorded cwd,
  never the host's` (6 files). Entry 23 through `--operator 23`: `history moved: refs/remotes/origin/HEAD
  de9ee3f9→3c1c54df; refs/remotes/origin/build/viola-0.1.0 de9ee3f9→3c1c54df`, a fast-forward.
- Entry 24: `3c1c54df925b verdict: red · checks 15/15 · first-fail +394 s test (ubuntu-latest)`, `ci#38970086070`.
  `test (windows-2025)` and `test (macos-latest)` green — the first fix held. `test (ubuntu-latest)`, the
  `run --coverage` job: `2646 tests run: 2644 passed, 2 failed` — `cli_list list_without_a_name_logs_no_agents_read`
  and `hook_statusline hook_statusline_without_a_recorded_command_prints_nothing_and_records_the_reading`, both at
  `tests/support/home.rs:362`, `viola verify did not stamp the home: exit Some(1), summary line matched false` —
  and the coverage merge refused a profile: `viola-4627-…_3.profraw: invalid instrumentation profile data (file
  header is corrupt)` → `llvm-cov-summary-missing`.

**Cause (this chunk's; inferred, not read):** the failing row is not in the logs (the stamp's `verify` logs nothing
without an instance, and CI uploads no stamp). Measured locally: the fake agent's `agents` listing paid one sysinfo
process lookup per registry row, about 1 ms each, and every fake killed by `verify` (Runs C, D, E) leaves its row
behind the shared binary: 398 rows read in 386–459 ms, 1 860 rows (the coverage build's dir after a few
`pre-push` runs) in 1 836–1 933 ms, under load 31. Under coverage instrumentation and CI's parallel load the
stamp's agents read can pass its 5 s bound; the read then kills the fake — the one kill in `verify` that can land
on a child that has finished its work and is writing its profile at exit, which fits the corrupt profile header —
and Run B carries no read, so `agents-join` fails and the stamp reads `1 fail`. Whether `rust-cache` carried rows
from the first CI run into this one is not measured.

**Fix:** one process-table read per listing — `viola_state::liveness::start_times()` (unit cases: this process at
its own start time; a dead pid absent) — and the fake checks each row against it: 398 rows 182–197 ms, 1 860 rows
214–243 ms. `stamped_home`'s message now names the failing rows' ids, so the next such red reads its row.

## The second fix, before its commit

- The whole block (21:45Z): green 17 · red 1 (2, clippy: `unnecessary_get_then_check` in the new unit case) →
  `contains_key` → `--only 1,2,5` green. `pre-push` in that firing (21.6): `"ok":true`, coverage 2647 of 2647, `gate`
  no breaches. The Windows target's clippy, check only: exit 0.
