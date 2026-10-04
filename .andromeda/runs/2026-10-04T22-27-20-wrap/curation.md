# Curation — 2026-10-04-running-turn-refusal

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Naming the process behind a truncated coverage profile" (confidence 0.8)
    Proof: the chunk's pre-push run 1 refused `viola-673807-…_15.profraw` (77 744 B against 81 128 B siblings); the
    header decode reproduced 81 128 B exactly at 64-byte records, the counters named a `viola hook` on
    `UserPromptSubmit`, the JUnit window named `hook_events::hook_prompts_arrive_normalised_with_their_origin`, and the
    fix that cause implied held for 4 consecutive green pre-push runs (`evidence/watch-profraw.md`).
  Filters: 1 dup (the fake agent's session-leader SIGHUP fact — this wrap's P2 landed it in test-plan §7 and the
    verification-harness.md body) · 1 task-specific (this wrap's operator directives) · 0 conflict · 0 deferred ·
    2 recurrence-despite-learning (→ handoff)
  No-other-home: "Naming the process behind a truncated coverage profile" (other signals 0.6: measured +0.4,
    specific technical detail +0.2; not on the route, not in a master, not a ledger note)
  Recurrences (Filter 1, defect entries; never dropped — handoff Deferred learnings):
    - host-win32.md body "A zero-is-healthy count probe … exits non-zero on no matches" — the plan's `:82` probe
      (`git grep … | wc -l`, `expect exit 0`) reproduced it under the gate shell's pipefail (gate entry 8 red on its
      satisfied subject).
    - testing.md 2026-09-24 "A test wait that a mutant can reach must … be bounded" (+ 2026-09-25 extended
      2026-10-04, the remove-the-guard control on a fixed test clock) — a new unit test parked a wrongly accepted send
      on a fixed-clock confirm window and its control read TIMEOUT (120 s) before the rework.
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B (health `check 1`).
