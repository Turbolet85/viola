# Founder forks — phase P4, chunk 2026-11-16-wait-wake-condition («`wait` wake condition»)

The operator is away (inputs#I1 item 6, inputs#I2): no dialog was raised. Each fork below waits for an answer by
relay. A founder fork is answered by the delegate PROVISIONAL, the most conservative option, his own word owed. A
technical fork is answered by relay at once.

## What P3 measured (research, doc:2026-11-16-wait-wake-condition/research; scope rev 2)

- `wait` wakes on the event KIND alone and reads no `data` (`src/run/wait.rs:196-210`). Every `turn-ended` wakes it,
  whatever its `background_tasks` count and whatever its message says. The CLI's Notification hook (`activity`) and
  a background task's `<task-notification>` harness turn (`prompt-submitted`) are log-only. MCP `wait` reaches the
  same wrapper method.
- So both clauses of the entry already hold in viola as built. The V150 miss is the overseer's own waiter, which
  re-parks on a `turn-ended` whose `background_tasks` is non-zero (`pwait.sh`, inputs#I3; `vwatch.sh` names V150,
  inputs#I4). The «false wake» after a 0-task end is a real `turn-ended`, returned while the CLI still had a wake
  scheduled.
- claude 2.1.325's own Stop hook schema (read statically from the installed build) carries two arrays.
  `background_tasks` is «in-flight background work … lets hooks distinguish "session is done" from "session is
  paused waiting for background work to wake it"». `session_crons` is «session-scoped cron tasks (CronCreate,
  ScheduleWakeup, /loop) that will wake this session later». viola carries the first as a count and drops the second
  at the parse. Every recorded Stop (2.1.287, 2.1.288, 2.1.302) holds both, empty.
- Live `claude` starts: none is asked (inputs#I1 item 3). Under either option of F1, every clause is proved on the
  fake agent: the wake rule reads only the kind, and a count reads only an array's length, never its elements. A live
  reading would add only whether a running Workflow counts in `background_tasks` on 2.1.325. No clause of this chunk
  turns on that.

## F1 — what v2-17 delivers, given that `wait` already wakes on both cases (founder's fork)

Cap v2-17's acceptance: «A waiting driver is woken by a question asked at a turn end that still holds a background
task and is not woken by a notification right after a task-free turn end; the wake condition is documented; each
case has a test that fails before the fix.»

1. **Document and pin `wait` as built (Recommended).**
   - The masters state the wake condition: the six wake kinds, every turn end waking whatever its counts, and the
     log-only kinds (Notification's `activity` and a harness turn's `prompt-submitted` among them).
   - They state what a driver reads to tell «done» from «paused»: the `background_tasks` count and the turn's
     message. They also state what viola does not carry: the CLI's scheduled wakes.
   - Each case is pinned by a unit row and a booted case over the fake agent.
   - «A test that fails before the fix» is concretized as each case's remove-the-guard control:
     - `turn-ended` taken out of the wake set turns the held-task case red;
     - `activity` or `prompt-submitted` put into it turns the no-false-wake case red.
   - No field crosses any boundary, and no escalate pattern is shipped.
   - Trade-off: a driver still cannot learn from viola that a 0-task end has a scheduled wake coming. It re-arms on
     the next turn end, as the measured driver does now. The fix for the V150 miss lands in the overseer's waiter, in
     the overseer's repository, outside this chunk.
2. **Option 1, plus carry `session_crons` as a count beside `background_tasks`.**
   - `turn-ended` data gains `session_crons`: the array's length, `null` when the key is absent, `null` or not an
     array. It is read as `IgnoredAny` elements, never one read, and is null on an API-error end.
   - It crosses into `events.ndjson`, `wait --json` and MCP `wait`, so a driver can read «a wake is scheduled»
     straight from the turn end.
   - **This is a Boundary widening** (escalate patterns `rul-r74hshif` / `rul-p2zl2kgl`): a new upstream field
     crosses the hook boundary into the event log and both driver surfaces.
   - The turn-ended data keys move from 3 to 4, with the architecture, security-plan and obs-plan rows and their
     Decisions Log entries.
   - A delegate's answer leaves it the wrap's full escalation, and it stays PROVISIONAL until the founder's own word.
3. **Change the wake rule itself.** `wait` would hold back a `turn-ended` whose counts are non-zero until a later turn
   end.
   - This puts driver policy in the binary, against the architecture's «viola is mechanism, not policy», and it can
     still never read the message.
   - Not recommended. Listed because the entry's wording could be read this way.

Delegate's answer (PROVISIONAL, the most conservative option), by relay
`../additional/viola-overseer/relay-wait-wake-f1.md` (snapped at P4): **option 1** — document and pin `wait` as
built, no product change. The acceptance's «a test that fails before the fix» is read as each case's
remove-the-guard control, said in so many words on the P5 card and in the cap's proposed wording. Option 2 is not
taken (a boundary widening is the founder's). Option 3 is not taken. The founder's own word is owed.

### Candidate for the founder (option 2, not taken by this chunk; no route entry minted)

- **What a driver gains:** it can see from the turn end itself that the CLI has a wake scheduled (CronCreate,
  ScheduleWakeup, /loop), so it does not treat that turn end as the session's last word.
- **What crosses:** one count, the length of the Stop payload's `session_crons` array (`null` when the key is absent,
  `null` or not an array). It holds no content: the elements are read as `IgnoredAny`, never one read, and it is null
  on an API-error end.
- **Where it would land:**
  - `crates/viola-agent-claude/src/hook.rs`: the Stop parse and the `turn-ended` data.
  - `turn-ended` data keys move from 3 to 4, crossing into `events.ndjson`, `wait --json` and MCP `wait`.
  - The masters' rows: architecture (event data, the hook-to-kind map), security-plan (what crosses the hook
    boundary, decision/stop-background-tasks-count-only's sibling) and obs-plan (event values), each with its
    Decisions Log entry.
  - The fake agent's synthetic Stop variants.

## Technical lean stated, not asked (the carry, inputs#I1 item 2)

Lean, decided by measurement at P3: option (a). One `viola verify` stamp per binary, taken under a file lock by the
first stamped-home test to need it. Every test copies it into its own fresh home under `target/e2e-home` (36–38 ms
against a 4.73 s stamp, measured on this host). Option (b), sizing the `ci` kill, is set aside: `testing.md`
2026-09-28 bars raising a bound for a load-driven excess.

Isolation cost: the tests share one stamp's bytes (one `written_at`, one pinned copy), each in its own copied home.
The test-plan's stamped-home row is amended at the wrap («each home stamped by `viola verify`» becomes «each home a
copy of one `viola verify` stamp»). A relay may overrule this lean before the card.
