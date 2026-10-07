# Curation — 2026-10-07-send-waits-out-the-paste-hint

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a sweep keyed on a moved bound's own values misses the cases past it by a round number"
                                              + testing.md: "a test clock must not act on state guarded by a lock its reader holds"
                                              + verification-harness.md: "the fake agent receipts a submit's Enter as a key line of its own"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 1 deferred (→ handoff)
  No-other-home: all three applied entries (each scored 0.6 before it, 0.8 with it)
```

## Applied
- testing.md — the sweep hazard. Score: verified by a real gate failure +0.4, specific technical detail +0.2,
  no other durable home +0.2 = 0.8.
  Proof: the first whole-block gate run of the implement run `2026-10-07T12-21-46-implement` read red in the unit
  entry, the default selection and `pre-push` on one case, `feed_past_the_frame_cap_poisons_and_settles_without_rows`
  in `src/cmd/verify/typed.rs`, which read a settle at 6 000 ms; research's sweep "cases that turn by the constant"
  and plan step 6 named two other cases of that file (report.md, Spec claims disproved, first bullet).
- testing.md — the clock under the lock. Score: the finding changed the chunk's design +0.4, specific technical
  detail +0.2, no other durable home +0.2 = 0.8.
  Proof: `Gate::wait_ready` reads `clock.now()` inside the model's lock (`src/run/gate.rs`, the loop body), which is
  why plan step 4's three after-the-wait cases make the gate wait on a screen fed 3 s after the base, not on a feed
  injected mid-wait (report.md, Deviations 1).
- verification-harness.md — the Enter receipt. Score: verified by a real test failure +0.4, specific technical
  detail +0.2, no other durable home +0.2 = 0.8.
  Proof: the keystroke case's first run read `["0d", "6b"]` where it expected `["6b"]`, every product assertion
  before it green (`evidence/hint-red-green.md`, the last section; `Input::plain` in the fake agent).

## Filtered
- task-specific: the overseer's five dispositions at this wrap (each is this chunk's own direction and is carried
  where it names: the sidecar entries, the report, the handoff, the route); pre-building and dry-running a one-shot
  live round before its fire (one event, no emphasis).
- deferred (the cap of three): the Bash tool's `find` is an embedded finder that refuses a GNU-style `-newermt`
  timestamp and takes the ISO form (0.8: a real failure, a specific detail, no other home). Its home would be
  `host-win32.md`, an always-loaded file, so it is judged at Tier 1's bar.
- recurrence-despite-learning: `host-win32.md` Exit codes ("read `$?` from the bare command") — the operator
  pass's push was read through `| tail -3`, with `PIPESTATUS` read beside it and the upstream distance read after.
