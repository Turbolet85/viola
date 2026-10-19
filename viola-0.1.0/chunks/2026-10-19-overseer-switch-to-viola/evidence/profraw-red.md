# The coverage merge refusal at implement's first `pre-push` (gate entry 19)

Recorded by /implement P2 before any re-run (a red stays open until its cause is known; a later green never closes it).

## The reading

- Run: run dir `.andromeda/runs/2026-10-19T10-56-42-implement`, gate entry 19 (`bash scripts/agent-run.sh pre-push`),
  first firing, log `19.log` (386 613 B, sha8 `5493f7fb`), 2026-10-19 ~11:01–11:03Z, the uncommitted Part A tree on
  `f23586fb`. Verdict `red · exit 0 ✗ (exit 1)`, `contains '"ok":true'` ✗, stage `linux-tests`.
- Suite `coverage`: 2958 tests run, 2958 passed, 0 skipped (nextest summary 68.092 s). The run's `failures` are
  `llvm-cov-exit-1` and `llvm-cov-summary-missing`, the merge after the tests (archived `target/run-archive/1619`).
- First diagnostic line: `warning: target/llvm-cov-target/viola-552803-11199965412932774179_3.profraw: invalid
  instrumentation profile data (file header is corrupt)`, then `error: no profile can be merged`.

## The file (kept outside the tree: the session scratchpad's `profraw-red/`, sha256 `4f3a54de…295b58f76`)

- 118 784 B. Its 7 766 siblings of the same binary signature `11199965412932774179` are all 146 328 B. The raw profile
  is version 10: 1 455 data records, 3 420 counters, names 25 685 B. Its header reproduces 146 328 B exactly. The cut
  falls inside the counters section (they end at 120 640 B), so only the leading counters stand.
- **Which process** (the method of session-learnings 2026-10-04: the surviving counters matched by name hash against
  a complete sibling's `llvm-profdata show --all-functions`): pid 552803 is a `viola hook` delivering a spine event.
  49 functions have nonzero counters, all named:
  - `main`, `role_of`, `cmd::dispatch`, `cmd::instance_of`;
  - `cmd::hook::{hook, handle, decided, deliver, spine_deadline}`, `cmd::hook::seam::{panic_if_asked, panic_on}`;
  - `obs::viola_obs_init` and the role-file open;
  - `viola_state::snapshot::read_snapshot_classified`;
  - `viola_channel::client::{open_by, connect_by, Client::notify}` (`hook.event`).

  It had sent its notification and was at the runtime's exit-time profile write when it was cut off. The event it
  carried is not readable: the counters past the cut, the `viola_agent_claude` normaliser's among them, are gone.
- **Which test** (the file's mtime 13:02:48.140 local, against the archived JUnit windows):
  - Not narrowed to one. More than forty cases of `channel_endpoint`, `cli_answer`, `cli_budget`,
    `cli_controls_not_disableable`, `channel_paste_validation`, `chaos_feed_panic`, `chaos_wait_vanish`,
    `chaos_forward_compat`, `chaos_torn_append` and `chaos_revive` hold the instant.
  - The closest kill-shaped candidate is `chaos_wait_vanish::chaos_wait_parked_wrapper_killed_exits_21` (47.378 +
    0.798 s, ending 36 ms after the mtime). That is a hypothesis, not a reading.
- **Not this chunk's new cases:** both new `hook_events` cases, and the binary's three other cases, ran at 13:03:13,
  25 s after the file was written. This chunk's diff touches no test or helper in the window, and no process
  lifetime: not the fake agent, the hook's exit path or a wrapper kill.

## Standing

- Mechanism class: a `viola hook` killed during its exit-time profile dump. This is the class of chunk
  2026-10-04-running-turn-refusal's `evidence/watch-profraw.md`, whose fix made the fake agent wait for a hook still
  running at `\x03` / EOF. Today's writer was cut off by something that fix does not cover. The killer is NOT known,
  so the red stays open, its cause owed.
- The tree-without-the-chunk control (a clean worktree at `f23586fb`) was not run: a race reading on one pair
  separates nothing on this host (verification-harness.md 2026-09-29).
- WATCH (`send_budget_paused…` case_4): PASS in this run (nextest, coverage suite) and in gate entry 5 (`run --unit`).

## Re-run

- Gate entry 19's second firing (`19.2.log`, 385 592 B) came back `green · exit 0 · 88.5 s`, with 2958 tests and 0
  `header is corrupt` lines. WATCH case_4 PASS. This green does not close the red above.
