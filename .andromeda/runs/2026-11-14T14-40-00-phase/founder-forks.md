# Founder forks — 2026-11-14-budget-gate-removed (phase P4, the away form)

The phase stopped at P4 on the one fork below; nothing is planned past it. The promotion stands (`route.py promote`,
tx 1157), the row read and the research are done (`doc:2026-11-14-budget-gate-removed/research`), and the scope is at
rev 2. Answer here, or in a message; the phase resumes at P4 on the answer.

## Q1 — Does the removal bump `v`?

**What forces the question.** `architecture · decision/protocol-version-bump-rule`: «Adding a field or event kind is
additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.»
`architecture · contract/protocol-version/v`: `v` is carried in every channel `params`, ndjson event line and snapshot.
This chunk removes fields from all three surfaces' contracts: the snapshot loses `budget_paused` and
`budget_override_until`; the channel `release` loses its `budget` param and its reply's `budget_paused` /
`override_until`; the `list` payload (CLI `--json` and MCP `list`) loses `budget` and each item's `budget_paused`; the
event kind `budget-gate` is no longer written.

**Measured at P3** (`doc:2026-11-14-budget-gate-removed/research`, «Measured facts»):
- A new reader reads an old snapshot and an old log: the budget fields are skipped, an old `budget-gate` line is counted
  in `skipped.unknown_kinds` (`crates/viola-state/src/snapshot.rs:37`; the events reader).
- An OLD (0.1.0) reader cannot parse a new snapshot: its `budget_paused` is a required field (`snapshot.rs:49`), so it
  replays the log, gets no endpoint and reads the instance unreachable — it writes no frame (fails closed).
- A bump of the snapshot's `v` gives an old reader the same outcome by another route (unsupported `v` → replay).
- A bump of the channel's `v` makes every call between an old client and a new wrapper, either way, `-32602`
  `unsupported protocol version` until both sides are upgraded.

**Options, in order (recommendation first):**

1. **Keep every `v` at 1 (Recommended).** The fields leave on your ruling «Убрать совсем» (2026-11-08); the wrap
   amends `decision/protocol-version-bump-rule` with an exception naming this retirement. Why it wins: an old reader
   already fails closed on a new snapshot (no frame, no command run), a new reader reads every old file, and nothing
   else in the fleet changes; the upgrade is one `cargo install`, after which a session started under the old binary is
   restarted to be reached by the new CLI. Trade-off: the written rule gets an exception; a mixed fleet during the
   upgrade reads new instances as unreachable from an old CLI.
2. **Bump the snapshot's `v` to 2 only; the channel and the documents stay at 1.** New readers accept 1 and 2. Why
   not first: old readers end where option 1 leaves them (replay, unreachable), so the bump adds a reader branch and a
   `snapshot-unsupported-v` line in old logs for no change of outcome; the channel and `list` removals still need the
   rule's exception.
3. **Bump every `v` the rule names to 2 (snapshot, channel `params`, event line).** Follows the rule as written. Trade-off:
   during the upgrade every old-client/new-wrapper call is refused both ways, the live fleet's CLI and wrapped sessions
   must all move together, and the event line's `v` changes though no event field is removed (only a kind stops being
   written, which the rule calls additive in the other direction).

### The answer to Q1 (appended 2026-11-14; a delegate's answer, PROVISIONAL)

The operator's message: «Answer to founder-forks.md Q1: option 1, keep every `v` at 1 — a delegate answer,
PROVISIONAL, the rule's exception escalated at the wrap for his own word.» The relay it names, read whole and copied
verbatim (`~/dev/projects/additional/viola-overseer/relay-budget-gate-q1.md`):

> # Phase of 2026-11-14-budget-gate-removed — the answer to founder-forks.md Q1 (a delegate's answer, PROVISIONAL)
>
> **Q1: option 1 — keep every `v` at 1.** PROVISIONAL: the driving overseer answers as the delegate while the operator
> is away; his own word ratifies or changes it, and the question is on his list for his next break.
>
> Why this one while he is away: it is the option that stays open. A `v` written as 2 into snapshots, logs and frames
> cannot be taken back without a second break; a bump added later is one more additive change. The measured outcomes
> do not differ for an old reader (it fails closed on a new snapshot either way).
>
> What it binds in the plan:
> - the plan states the measured compatibility facts as measured (new reader / old files; old reader / new snapshot),
>   with a test for each direction the tree can run (a new reader over a 0.1.0-shaped snapshot and log);
> - the exception to `architecture · decision/protocol-version-bump-rule` is an EXPECTED AMENDMENT FOR THE WRAP, marked
>   PROVISIONAL there and escalated on the wrap's card for his own word — it is not settled by this answer;
> - if his word changes it to a bump before implement ends, the plan takes a revision; say on the P5 card which steps
>   a bump would add, so that revision is cheap.
>
> The five leans stand as written (the ledger row leaves, relied 24 → 23; a `release` carrying a `budget` key is
> refused `-32602` after the string-`from` check; WHEEL stays 23; the BAY line keeps `skipped a - b - c`; the MCP
> tools-list fixture stays as recorded). The companion files the research added without a second row read: name them
> on the P5 card with the row each one rests on.

## Leans the plan will take without asking (shown here so the answer can overrule any of them)

- **The `statusline-rate-limits` ledger row leaves in this chunk** (relied 24 → 23): leaned per v2-04's own
  requirement («no `statusline-rate-limits` ledger row») and the census rows routed to v2-04 (C-205, C-286, C-587,
  C-747).
- **A new wrapper refuses a `release` carrying any `budget` key** with `-32602` `invalid-params` (`data: null`), after
  the string-`from` check, which stays first: leaned per the invariant «the human always wins» — an old
  `viola release --budget` meant «leave the wheel», and a wrapper that skipped the key would hand a human-held wheel
  to the driver — and because skipping it would admit an input the wrapper refuses today (the playbook's «Boundary
  widening» pattern, `rul-r74hshif`).
- **The board's WHEEL column stays 23 characters** wide: leaned per your confirmation of WHEEL 23 as your wording
  (2026-10-13) and the board width you ratified (2026-10-20); narrowing it to `unknown` (7) would move every row's bytes
  and the 65-column NAME cap. The row's stated reason («the width of its widest word») is re-worded at wrap.
- **The BAY line keeps only its non-budget cells**: `BAY  skipped a - b - c`, the four budget cells leaving per the
  entry («no … budget board … fields»).
- **`fixtures/claude/2.1.302/Mcp.tools-list-1.json` stays as recorded**: its replay reads no description; a re-record
  needs a live `claude` start, so it waits as a named operator step («Live round on the trimmed product»'s).
