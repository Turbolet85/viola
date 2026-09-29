# Curation — 2026-09-29-fake-agent-drift-contract

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "On this host the local suite grades an identical tree differently run to run, so one subject/control pair never separates two trees: judge a local red against a same-day control as reverse-order standalone pairs of the same harness command, then run the binaries still unmatched alone for a few rounds on BOTH trees — and the red stays open under its owner until its cause is known."
    Proof: the `2d8bc53` control read 44 then 57 failed across two standalone `run --coverage` runs, and 2 then 0 on a
    23-test two-binary run; pair 1 alone showed 12 chunk-only reds that pair 2 cut to 5 and the alone-rounds (control
    2 reds, chunk 1, the same test on the same 1.0 s bound) closed — `viola-0.1.0/chunks/2026-09-29-fake-agent-drift-contract/evidence/host-reds-two-sided.md`.
    Score: measured +0.4 · repeated pattern (entry 7 pair, entry 8 pair, two coverage pairs, alone-rounds) +0.3 ·
    specific technical detail +0.2 = 0.9.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 4 below threshold (`grep -c insta` counts
  "instance" 0.2-0.3; the fake-agent watcher's idle-CPU reading 0.6 exact; the booted harness session writes no
  receipt, one-off; coverage logs carry binary bytes, 0.6 exact) · 1 routed elsewhere (the subagent hand-back framing
  `Another Claude session sent a message:` + `<agent-message from=…>` → the P5 CARRY on "First live test and
  self-drive")
  Recurrence: `recurrence-despite-learning: host-win32.md Session Additions 2026-09-28 (extended 2026-09-29) — "The Bash guard refuses … ANY command carrying [a doubled backslash]"` — met 3× this session (the implement hygiene regex, a drive-path regex, the reconcile evolve JSON), each rerouted through a Write-tool file → handoff Deferred learnings.
