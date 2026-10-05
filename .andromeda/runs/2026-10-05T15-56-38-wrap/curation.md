# Curation — 2026-10-05-permission-end-to-end

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "nextest pads a status line's duration inside its brackets … an atom over a recorded status line allows the padding"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  Extended: T2/testing.md: "2026-09-28: A timing red is never fixed by raising a timeout or a test bound" + "the designed-floor exception, with its planted-hang control pair"

## Applied
1. **Extended, `testing.md` 2026-09-28 (the timing-red rule): the designed-floor exception.**
   - Confidence 0.9: the operator's explicit direction to curate it (+0.5), verified by measurement (+0.4).
   - Filter 3: the extension narrows the entry it extends, so it is a conflict. The handoff had parked it as "Curation
     conflict (your review)". The operator resolved it at this wrap's invocation (carry 1). The text is the overseer's
     decision, relayed by the operator: inputs#I1 of the chunk, and plan.md's curation note.
   - Proof:
     - the class's designed floor, measured across four CI rounds: `2026-10-05-dialog-rows-and-re-probe/evidence/ci-rounds.md`
       and `verify-window-class.md`;
     - its planted-hang control pairs: the 20 s override (`verify-window-class.md` §Controls: `TIMEOUT [20.003s]` /
       `PASS [43.346s]`) and the 45 s override (`2026-10-05-permission-end-to-end/evidence/verify-window-hang-control.md`:
       `TIMEOUT [  45.004s]` under `--profile ci`, `PASS [  80.480s]` under the default profile, 2026-10-05).
2. **New, `testing.md`: the nextest bracket-padding sweep hazard.**
   - Confidence 1.0: the operator flagged the defect (+0.4), verified by measurement — it falsified the plan's gate atom
     (+0.4), specific technical detail with context (+0.2).
   - Filter 2: a sweep hazard, so the token is kept.
   - Home: `testing.md`, whose `paths:` cover `.config/nextest.toml` and which holds the kill-run-evidence rule.
   - Proof: `plan.md:202`'s atom `TIMEOUT \[4[0-9]\.[0-9]+s\]` against cargo-nextest 0.9.146's verbatim
     `TIMEOUT [  45.004s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row`. It matched
     only the evidence table's collapsed row (report Spec claims disproved 1, the dated plan correction).

## Rejected
- **"A one-off `cargo nextest run` of a root test target needs `--features fake-agent`"** — 0.2 (detail +0.2,
  paraphrased from implicit behaviour −0.2, measured fact +0.4 discounted: the run without the feature was never made,
  so what it does is not measured). Below threshold. Carried in report Spec claims disproved 2.
- **"Raw boot JSON and nextest logs carry host paths; keep them in the scratchpad"** — the wrap's own P7.3c contract
  already states the remedy (`gate-contract.md` §Hygiene: a process capture moves to the session scratchpad). One-off
  (−0.3), 0.1. Below threshold.

CLAUDE.md size: see P7's health row.
