# Curation — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

Source: this session's conversation (the part before the compaction through its summary) and the report's
Decisions & corrections.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A test that starts the wrapper starts its program twice … an uninstrumented host program, never the test binary" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): extension only (below)
  Filters: 0 dup · 0 task-specific · 0 conflict · 4 below threshold · 2 recurrences (→ handoff)
  No-other-home: "A test that starts the wrapper starts its program twice …"
  No-other-home: "the method needs the refused file itself, and a CI run keeps none"
  Extended: T3/session-learnings.md: "2026-10-04 — Naming the process behind a truncated coverage profile" + "a CI run keeps no refused profile, so its writer cannot be named this way"
```

## Applied
- Tier 2, `.claude/rules/testing.md` `## Session Additions`: the wrapper starts a test's program twice (the
  version probe and the PTY child), with its plugin flag first; use an uninstrumented host program that exits by
  itself.
  Proof: `evidence/profraw-red-green.md` — the old child exited 101 on `--plugin-dir` before any test body; the
  must-pass control read one profile a run, not the two the plan predicted, because the probe runs the same
  program; 3 of 4 800 runs on the untouched test left a short third profile. It falsified two plan claims and
  changed the chunk's design (+0.4), with a specific technical detail (+0.2); at exactly 0.6 the no-other-home
  signal fired (+0.2): test-plan §10 carries the corrupt-profile mechanism, no master carries the probe's second
  start.
- Extension, Tier 3, `.claude/docs/session-learnings.md`, the entry "2026-10-04 — Naming the process behind a
  truncated coverage profile": one sentence tagged `Extended 2026-10-07`.
  Proof: `evidence/ci-attempt-1.md` — the `harness-ubuntu-latest` artifact holds ten members and 0 `.profraw`;
  the refused file sat in `target/llvm-cov-target/`, outside the upload, so pid 10799's writer could not be named
  and the closure states that limit. Measured on a real CI red (+0.4), a specific detail (+0.2), and no other
  durable home (+0.2): the masters state the limit, not the reason the method fails.

## Not applied
- Below threshold, 4:
  - "`gh api` on a job's logs writes nothing to a redirect until `--allow-escape-sequences` is passed" — one
    event, a detail only (0.2).
  - "a hit's section is read from the heading above it, never from the hit's wording" — this wrap's own slip at
    `test-plan.md:233`, one event (0.2).
  - "after a compaction a count is measured again before it is reported" — one event (implement's report said
    nine census directories, eleven on disk); the standing Tier 1 and the operator-memory rule on measuring
    before stating already cover the class.
  - "a red where no assertion failed on a value is read against the backing and the host record first" — the
    operator's word with both implement invocations (0.5: "never" language and a detail); it stands in the plan's
    STOP rules for this chunk.
- Recurrences, to the handoff's deferred list:
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` (a `cat` heredoc with a file target, refused by
    the Bash guard at implement; the rule file states it).
  - the deferred "a time written into a record ahead of the clock (read `date -u`)" met again: an inputs origin
    label typed ten minutes ahead.
- Not curated, the masters own them now: the remedy and its authority; the corrupt-profile mechanism; the
  census script.
- The cascade's one curation-home row (`session-learnings.md:12`, the `profile` pattern) is the extension above.

CLAUDE.md size: read at P7 from `health.py check`.
