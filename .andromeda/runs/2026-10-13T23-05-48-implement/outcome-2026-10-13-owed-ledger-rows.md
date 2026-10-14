# Outcome — 2026-10-13-owed-ledger-rows (implement run 2026-10-13T23-05-48)

Written 2026-10-13T23:31:25Z.

## Outcome: green + smoke (rewritten 2026-10-14T06:24Z; first written as surfaced)

The gates are green: all 17 non-leg entries on the final full run (`gate-run-6.txt`), and the live round, fired once
(`round: COMPLETE · legs fired 2/2`, `evidence/round-232112Z.txt`). One fixture-hygiene gap was surfaced (below). It
was resolved by the founder's word, accept and push (`inputs#I3`).

The operator pass then ran (`evidence/operator.md`):
- entry 24, hygiene, clean;
- the pre-CI commit `9ee83095ace5`, pushed by entry 25;
- entry 26 read **red** (ci#38519633153), one test each on macOS and Windows, both this chunk's own (deviation 10);
- the fix commit `1c4f2258ef7c`, pushed after a green block, then entry 26 read **green** (ci#38520999442, 15/15,
  wall 550 s).

Next → `/andromeda-wrap-session`. It amends test-plan §7 narrowly: a `workspace.repo` naming this repository's own
remote is kept.

- Record round (entry 8, 5 starts): `stamped 2.1.287  21 pass  0 fail`; census empty.
- Live-suite round (entry 11, 5 starts): `stamped 2.1.287  21 pass  0 fail`, `"suite":"local-live","passed":1,"failed":0`;
  census empty.
- Ten live starts in all, the founder's cap. They were ledgered in `journal.md` before the one call that made them, and no
  round was re-fired.
- Nine row cases: `"suite":"nextest-unit","passed":9,"failed":0` (entry 6).
- Integration entry (entry 14): green in 35.41 s against the P5 baseline's 35.9 s. The two fake-agent statusline firings
  per stamped-home verify cost nothing measurable.
- Smoke: entries 17–22 ran green as P2 gates (boot stamps at 21 rows against the fake agent); P3 recorded, not re-driven.
- Matrix: the chunk claims no capability (`v1-34` stays unclaimed, per the plan), so there was no write.
- A Windows-target clippy (`target/wincheck`, check only) read clean after one fix.

## Surfaced — a spec↔reality gap (nothing authored)

- **Value:** both recorded variants, `fixtures/claude/2.1.287/StatusLine.default-1.json` and
  `StatusLine.rate-limits-1.json`, carry `workspace.repo.owner`. Its value is the repository's GitHub owner handle.
- **Spec:** test-plan §7 Fixture hygiene says fixtures are synthetic probe text only, scrubbed of absolute paths and
  usernames. `testing.md` §Test data says the same.
- **Impl reality:** `ledger::scrub`, `ledger::unclean` and the hygiene walk (`tests/support/hygiene.rs`) know only the OS
  user (the home dir's last component), and only as a whole word. The handle is that name with a two-digit suffix, so
  nothing rewrites or refuses it. The record round's own refusal did not fire, and `contract_fixture_hygiene` reads both
  files green.
- **First occurrence:** no committed file held the handle before (`git grep -i` over `fixtures schemas tests src crates`
  read nothing). The hook payloads carry no `workspace` object; the status line payload is the first fixture class that
  does.
- **Why this run did not fix it:**
  - The plan's rejected approaches bar a hand-written or hand-edited fixture.
  - Widening the scrub is new behaviour, and a re-record would need live starts past the cap.
- The two files are untracked; nothing has left the host. Ways forward are the operator's call:
  - Accept the handle as public: it is the remote's own owner name, and wrap amends test-plan §7 to say so.
  - Or re-plan: a `workspace.repo` scrub and a re-record round, both under a new cap.
- **Resolved — accept and push** (`inputs#I3`, relayed by the operator as viola-overseer): the founder's own word,
  live in the overseer's dialog 2026-10-14T05:57Z. The handle is the public remote's own owner, already committed in
  the architecture registry. The operator pass runs. The wrap amends test-plan §7 narrowly: a `workspace.repo` naming
  this repository's own remote is kept.

## Files changed (18; the scope read: clean, 18 listed, 0 recorded)

- New:
  - `schemas/claude-statusline-fixture.v1.json`;
  - `fixtures/claude/2.1.287/StatusLine.default-1.json`, `StatusLine.rate-limits-1.json` (recorded live, copied alone);
  - `viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/evidence/` (`live-round.md`, `round-232112Z.txt`).
- Modified:
  - `crates/viola-agent-claude/src/ledger.rs`, `statusline.rs`;
  - `src/cmd/hook.rs`, `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`;
  - `src/run/version_gate.rs`, `src/bin/viola-fake-agent.rs`;
  - `tests/hook_fail_open.rs`, `tests/cli_verify.rs`, `tests/contract_ledger_probes.rs`;
  - `tests/contract_fixture_hygiene.rs`, `tests/contract_fake_agent_drift.rs`;
  - `tests/support/home.rs`, `tests/support/verify.rs`;
  - `crates/viola-e2e/src/harness/run.rs`.

## Deviations from the plan, with justifications

1. **Step 2's order.** The arms were written before the nine cases, in one scripted edit, so no red-before-green reading
   of the cases exists. The cases do read each arm's pass, broken shape and no-capture fail, and are green.
2. **Step 2's count.** The plan fixes nine cases but gives the override row no broken payload shape. The third override
   case is "Run B's hooks fired while its status line filed nothing", separate from "no capture at all". The gate's
   `passed 9` atom holds.
3. **A shape-pinned reader research did not list.** `merge_stamp_writes_the_envelope_for_a_first_version`
   (`ledger.rs`, a literal `rows` object) went red at the first unit run and gained the three ids. The file was on the
   list; the reader was not. This recurs testing.md 2026-10-07.
4. **Harness gate spelling.** The 18-row `LEDGER_ROWS` and `LIVE_PASS` in `crates/viola-e2e/src/harness/run.rs` are
   `cfg(not(unix))`, not `cfg(windows)`.
   - `pre-push` (entry 23) went red on `contract_windows_mutation_scope`: a `cfg(windows)` line in product code makes the
     file a Windows-gated source that `windows-mutants.yml` must list.
   - `not(unix)` is the exact complement of `statusline::WRAPPED_HERE = cfg!(unix)`, so it says the rule as the product
     does, and no workflow change is needed. Test files and in-file test modules keep `cfg(windows)` for their Windows
     literals.
5. **Step 3's parser.** `parse_statusline_capture_file_name` reads the `StatusLine.<k>.json` names only. The shell file
   is derived from `k` (`statusline_shell_file_name`), never listed.
6. **Step 6's plumbing.** `named_payloads` now returns owned `(name, Value)` pairs, so the status line variants (bytes,
   not `Capture`s) join the same scrub and refusal path. A payload that does not parse is still left out.
7. **Step 8's relative command.** The `--framing` statusline replay goes through the existing `run_receipted`. A
   relative command is receipted `command_absolute:false, ran:false` and never run, as the `--statusline-stdin` path
   already does.
8. **Cases beyond the plan's list:**
   - `hook.rs` units on `capture_statusline` (k claim, byte-exact word, cap, bad dir);
   - `read_statusline` units and the `ProbeDir` settings case;
   - `verified_reads_a_failed_statusline_row_by_the_relied_set`;
   - the harness's `run_local_live_reads_the_relied_rows` and its statusline row-missing cases;
   - the drift contract's `statusline_drift` synthetic case;
   - the hygiene walk's planted status line cases;
   - `cli_verify`'s identity-strip case asserting the two `statusline` receipts on Unix.
9. **A pre-check outside the block before the live round.**
   - `agent-run.sh run --integration --filter 'binary(cli_verify) | binary(hook_fail_open) | binary(hook_statusline)'`,
     fake agent only, to prove Run B's new statusline path before spending live starts.
   - It read 66 passed and 5 failed, all five at `stamped_home` over the then-unrecorded set. That is the designed red
     until entry 8, and those cases read green in entry 14 and `pre-push` afterwards.

10. **The CI red on the pre-CI commit, fixed in the chunk.** Two of this chunk's cases went red on CI. Neither shows
    on the Linux host.
    - The macOS red, `cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`:
      - The fake agent's first-Stop status line replay ran before the paste-hint clear, so the earlier input box
        stood on screen while `viola hook statusline` spawned.
      - Its cause was read on Linux with a 2 s probe sleep (red at `cli_send.rs:919`). It went green with the replay
        moved after the turn screen, the sleep kept in (80 of 80 over the four replay-driving binaries). The sleep was
        then removed.
    - The Windows red, `ledger::tests::verified_needs_every_row_pass_under_that_version`: the case popped `ALL`'s last
      row, which Windows does not rely on. It now drops `relied()`'s last row.
    - Both files (`src/bin/viola-fake-agent.rs`, `ledger.rs`) are on research's list.

## Process hygiene

Measured at 2026-10-13T23:31Z against the host's process list: every process whose `/proc/<pid>/exe` is under the
repository, and every `claude` whose cwd is a probe dir.

| process | started by | final state |
|---|---|---|
| `claude` 2.1.287 ×5 (record round) | a leg (entry 8) | terminated (census entry 9 empty; none at the final census) |
| `claude` 2.1.287 ×5 (live-suite round) | a leg (entry 11) | terminated (census entry 12 empty; none at the final census) |
| `viola` / `viola-harness` / fake agent (gate entries, smoke session `p-olr-smoke`) | this run | terminated (entry 22 `processes_gone:true`, `endpoint_gone:true`; none at the final census) |
| `viola` / fake agent (the operator pass: two probe runs, `gate-run-6`, smoke session `p-olr-smoke` again) | this run | terminated (re-measured 2026-10-14T06:25Z: no process whose `/proc/<pid>/exe` is under the repository; no `claude` whose cwd is a probe dir) |

## Decisions & corrections

- **The live cap.** The founder's cap is 10 starts (his dialog 2026-10-13T22:52Z, the operator's invocation). Ten were
  made, ledgered before the call, and none was re-fired.
- **Guard refusal.** The Bash guard refused a `cat` heredoc to a scratch script; it was re-issued through the Write tool.
- **Hook false positive.** The stamp-ahead hook blocked a scratch script because a test-data RFC 3339 literal was later
  than the clock. The literal was moved to a past date.
- **Recurrence of a standing learning.** testing.md 2026-10-07: a moved bound's readers are listed by function. A
  literal-row-set oracle (`merge_stamp`'s envelope case) was missed by research's sweep.
- **Held operator pass.** The invocation asked for the operator pass "as usual". It was held on the surfaced gap,
  because the push would publish the handle in two committed fixtures. The founder's answer, accept and push, was
  relayed by the operator (`inputs#I3`, 2026-10-14T05:57Z), and the pass ran then.
- **Context compaction.** The session was compacted at 01:25Z. The run dir's outcome and journal were re-read from
  disk before the pass, at the operator's direction.
- **A cross-OS miss, met in CI.** A case written against `ALL`'s order assumed every row is relied. A fake-agent
  step inserted between a hook and a screen change moved a race window that only the macOS runner's spawn cost opens.
  Both stayed green on the Linux host and its Windows-target clippy.
