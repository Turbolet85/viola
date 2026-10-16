# relay-4 — the P7.1 ASSERT question of the 2026-10-14-resume-and-compact-ledger-rows wrap

Asked by the wrap session as one AskUserQuestion. The selected option and its note are verbatim.

Question: P7.1 ASSERT: round entry 12 (the census after the `--local-live` round) has no line in a round listing under
evidence/. The round was fired with `--entry 11`, which runs no step after its leg, so the census was run by hand with
the plan's exact command at 07:19:50Z (no pid printed), recorded in evidence/live-round.md. The contract halts on a
round entry with no listing line, and `--live-legs` is forbidden at the wrap (the cap is spent anyway). How is entry 12
treated?
Options, in order: Accept the hand record (Recommended) · Re-run the census by hand now · Hold the flip.
Answer: "Accept the hand record (Recommended)"
Notes: Overseer, a technical fork (founder-delegated): the census ran with the plan's exact command at 07:19:50Z, right after the round, and is recorded in evidence/live-round.md; a re-run today proves only today. Name the treatment in the report's Outcome and the commit body.
