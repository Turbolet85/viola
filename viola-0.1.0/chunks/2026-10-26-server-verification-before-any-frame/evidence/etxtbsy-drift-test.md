# The pre-push red met at implement P2 — ETXTBSY in the agents drift test, folded and fixed

**The red.** Block run 1, entry 21 (`bash scripts/agent-run.sh pre-push`), red `"ok":true` ✗ at stage `linux-tests`;
its one failing test (gate trail `.andromeda/runs/2026-10-26T08-22-16-implement/gate-2026-10-26-server-verification-before-any-frame.json`, `show --n 21`):

    FAIL [   0.291s] ( 385/3048) viola::contract_fake_agent_drift fake_agent_agents_replay_matches_every_recorded_agents_set
    thread '<unnamed>' panicked at tests/contract_fake_agent_drift.rs:582:14:
    agents --json: Os { code: 26, kind: ExecutableFileBusy, message: "Text file busy" }

**The mechanism (measured, not this chunk's code).** The test runs each recorded set on a thread of its own
(`each_set_at_once`), and each thread planted its `claude` with `support::fake::claude_bin`, a hard link of the fake agent
or, where linking fails, a copy. On the Linux dev host `target/e2e-home` is a link to a tmpfs directory
(`test -L target/e2e-home` true; `findmnt -T target/e2e-home/` tmpfs, `-T target/debug/` btrfs), so the link crosses
filesystems and every plant is a copy. Two sets carry `Agents.recorded-1.json` (2.1.287, 2.1.302), so two threads plant
and spawn at once: a child one thread forks while the other's copy still holds its write handle inherits that handle
until its own exec, and the copy's exec then fails `ETXTBSY`. None of the involved files is in this chunk's diff
(`tests/contract_fake_agent_drift.rs`, `tests/support/fake.rs`, `src/bin/viola-fake-agent.rs`, before the fix).

**Before the fix** (2026-10-26 08:38:05Z–08:38:23Z by the run logs' mtimes, this tree), the test alone, 40 standalone runs:

    cargo nextest run --workspace --features fake-agent -E 'test(=fake_agent_agents_replay_matches_every_recorded_agents_set)'
    red 6 of 40 — runs 3, 10, 15, 26, 38, 39, each log holding "Text file busy"

**The ruling.** Fold the fix in (`inputs#I5`, the overseer's answer, founder-delegated, 2026-10-26 08:39Z): a red met
during a chunk folds into that chunk; show the fix with the 40-run loop at 0 red; the guard's outside-the-plan line for
`tests/contract_fake_agent_drift.rs` accepted as named. The overseer read the host record CONTENDED for 08:24–08:39Z
(15 s IO stalls at 08:34–08:35Z), which can raise the rate; the standalone reading stands on its own.

**The fix** (`tests/contract_fake_agent_drift.rs`, recorded in `scope-record.md`): every set's `claude` is planted
sequentially before any set's thread starts; the threads only spawn. No product file and no support file changed.

**After the fix** (2026-10-26 08:41:43Z–08:42:02Z by the run logs' mtimes, the same command, 40 standalone runs):

    red 0 of 40
    Summary [   0.148s] 1 test run: 1 passed, 3054 skipped   (run 40)
