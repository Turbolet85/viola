# Evolve Diagnosis — viola-0.1.0 · Epoch 3 — Windows slice II: driving verbs and live proof · 2026-10-08T09:45:26Z

Obligation-free: every item below may be accepted, rejected, deferred or modified with no consequence on the mechanism's side. Nothing here is applied, queued or remembered. Evidence twins: `q-health.json`, `q-retractions.json`, `q-typed.json`, `q-untyped.json`, `q-chains.json`, `q-level.json` beside this file.

## Mechanism health

- **Population:** 681 records carry this epoch's label (345 step / 336 friction), one spelling of the label. The population is selected by the epoch label alone, so both spellings of `version` are read: `viola-0.1.0` 609 and `0.1.0` 72 (ledger lines 1236 to 1387, written by all four skills). Both spellings of `skill` (`andromeda-{name}` and `{name}`) are folded. A filter on either `version` spelling would read a part.
- **Ledger:** 1438 records · unparseable 0 · malformed `ts` 0 · id fill 681/681.
- **Duplicate ids in the epoch:** 2 (`2026-10-06T22:04:50Z-a`, `2026-10-06T22:04:50Z-b`; one step record of the epoch names the cause, two checkpoints appended within one second). A retraction aimed at a duplicated id would resolve to the later record; none is.
- **Retractions:** retracted 2 ledger-wide, 1 in this epoch (`2026-10-05T13:08:03Z-d`, re-recorded untyped by its retractor); problem-fact targets 1 ledger-wide, 0 here; clause-retracted 0; retraction targeted by retraction 0. Stages 1 to 4 ran over 335 friction records after the filter.
- **Unresolvable retractions (2, never honored, verbatim for a manual discount; both targets sit in an earlier epoch):**
  - retractor `2026-09-27T07:06:36Z-b` · unknown-id-or-scope · {"id": null, "note": "discount the signal carries-pinned-7 and the note '7 CARRYs' in step record 2026-09-27T06:40:33Z-a: 8 CARRYs were pinned (:40 x4, :43, :65, :67, :73); a step-record signal has no record/clause retraction scope"}
  - retractor `2026-09-27T12:58:32Z-a` · unknown-id-or-scope · {"id": null, "note": "discount the ts of step records 2026-09-27T13:02:00Z-a and 2026-09-27T13:10:00Z-a the same way; their content stands"}
- **Untyped:** 57 of 335 friction records (17 %). One record spells the absence as the literal type `untyped` with no flag (`2026-10-04T18:40:24Z-c`); it is counted as untyped.
- **Problem-fact fill:** 139 of 345 step records carry facts (217 facts).
- **Calibration boundaries inside the range:** the report record's `operator-pass` entry lives from 2026-10-07; wrap's `new-text-*` words and the reconcile record's `rejected-for-source` / `rejected-for-coordinate` counts live from 2026-10-08. Earlier records without them are era. `contract.in-pass-correction` (2026-09-27) and the universal types are live for the whole range.

**Coverage** (step records per chunk against the expected phase 5 · implement 3 · wrap 5):

| chunk | phase | implement | wrap | beyond the expected count |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | 5 | 3 | 5 | — |
| 2026-10-03-mutation-scoring-completion | 5 | 3 | 5 | — |
| 2026-10-04-windows-boundary-mutation-workflow | 5 | 3 | 5 | — |
| 2026-10-04-readiness-gate-and-timing-constants | 5 | 3 | 5 | — |
| 2026-10-04-confirmed-send-with-cl-1-records | 5 | 3 | 5 | — |
| 2026-10-04-wait-and-last | 5 | 3 | 5 | — |
| 2026-10-04-dialog-answers-by-dialog-id | 8 | 4 | 5 | research ×2 · plan ×2 · validate ×2 · code ×2 |
| 2026-10-04-the-wheel | 5 | 3 | 6 | gates ×2 |
| 2026-10-04-running-turn-refusal | 5 | 3 | 6 | gates ×2 |
| 2026-10-05-real-cli-verify-probes | 11 | 6 | 5 | research ×3 · plan ×3 · validate ×3 · code ×3 · fix-loop ×2 |
| 2026-10-05-dialog-rows-and-re-probe | 5 | 4 | 5 | code ×2 |
| 2026-10-05-permission-end-to-end | 5 | 3 | 5 | — |
| 2026-10-06-local-command-and-paste-framing-rows | 10 | 7 | 5 | research ×2 · plan ×3 · validate ×3 · code ×3 · fix-loop ×2 · smoke ×2 |
| 2026-10-06-local-command-send-outcomes | 5 | 3 | 5 | — |
| 2026-10-07-test-homes-off-the-contended-volume | 5 | 3 | 5 | — |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | 5 | 3 | 5 | — |
| 2026-10-07-send-waits-out-the-paste-hint | 5 | 3 | 5 | — |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | 8 | 7 | 5 | research ×2 · plan ×2 · validate ×2 · code ×2 · fix-loop ×3 · smoke ×2 |
| 2026-10-08-first-live-test-and-self-drive | 11 | 8 | 5 | research ×3 · plan ×3 · validate ×3 · code ×3 · fix-loop ×3 · smoke ×2 |

Checkpoints that did not fire: 0. The counts beyond the expected are revision runs of a pending chunk (phase and implement more than once in 5 chunks) and resumed wrap gates. Without a chunk: 52 new-session orientation records (one per session start) and 3 route-resolve records of 0-pending adaptation wraps. Two friction records carry step `u35-door`, which has no step record, so that step has no denominator.

**Per step** (step runs · friction · untyped):

| skill/step | runs | friction | untyped | not `ok` outcomes |
|---|---|---|---|---|
| implement/code | 28 | 61 | 15 | halted-resolved 3 · soft-exit 5 · ok-degraded 1 |
| implement/fix-loop | 25 | 44 | 9 | ok-degraded 1 · halted-resolved 2 · soft-exit 5 |
| implement/smoke | 22 | 1 | 0 | soft-exit 1 |
| new-session/orientation | 52 | 13 | 4 | — |
| phase/distill | 19 | 7 | 0 | — |
| phase/plan | 27 | 17 | 4 | — |
| phase/research | 26 | 22 | 10 | ok-degraded 2 |
| phase/take-up | 19 | 10 | 3 | — |
| phase/validate | 27 | 34 | 2 | ok-degraded 1 |
| wrap-session/curation | 19 | 32 | 1 | ok-degraded 1 |
| wrap-session/gates | 21 | 8 | 3 | halted-resolved 4 |
| wrap-session/reconcile | 19 | 51 | 3 | halted-resolved 9 |
| wrap-session/report | 19 | 11 | 0 | — |
| wrap-session/route-resolve | 22 | 23 | 3 | halted-resolved 4 |
| wrap-session/u35-door | 0 | 2 | 0 | — |

**Observations on the capture itself** (never proposals):

- **Classification scatter.** One event is filed under several types depending on the step that met it. A keyword read of this epoch's friction `what` lines finds the Bash guard's refusal of a heredoc with a file target in 22 records under 8 (skill, step, type) keys (`recall.corpus-recurrence` 11, `tooling.hook-friction` 6, `tooling.host-shell` 2, untyped 3), and a time written ahead of the clock in 11 records under 8 keys (untyped 7, `tooling.hook-friction` 2, `retry.synthesis-rework` 1, `recall.corpus-recurrence` 1). Each typed group below therefore under-counts these two events, and the per-step thresholds see them in pieces.
- **Reading order.** Three problem-facts and one untyped record say a checkpoint's playbook was read before its step had completed (take-up once, wrap gates twice, the read made while the light gate ran in the background; the untyped record is the same event as one of the gates facts). The records state the step's outcome was already determined by the tool.
- **`version` spelling.** The second spelling (`0.1.0`) appears from ledger line 1236 and stops at 1387; the envelope's wording (active project version) does not say which form is meant.

## Proposals (typed patterns)

278 typed friction records in 41 (skill, step, type) groups plus 10 cross-step groups (universal and `recall.*` types, grouped by type alone). Threshold F-2: n ≥ 3, or n ≥ 2 where at least two cases carry a halt or soft exit (the reading applied to the plural in the rule). 31 groups are above it. Weight is the sum over the group of 1 + iterations + retries + reformulations + 2·dialogue rounds + 3·halted + 3·soft exit; the order below is by weight.

| # | group | n | weight | chunks | halting cases |
|---|---|---|---|---|---|
| P1 | `phase/validate/contract.mechanical-check` | 30 | 64 | 16/19 | 0 |
| P2 | `*/*/contract.premise-falsified` | 16 | 50 | 10/19 +2 no-chunk | 5 |
| P3 | `*/*/recall.corpus-recurrence` | 19 | 41 | 15/19 | 0 |
| P4 | `implement/code/input.plan-step-ambiguous` | 19 | 37 | 15/19 | 1 |
| P5 | `wrap-session/reconcile/contract.in-pass-correction` | 16 | 36 | 15/19 | 0 |
| P6 | `wrap-session/reconcile/contract.proposal-format` | 4 | 24 | 4/19 | 0 |
| P7 | `*/*/contract.skill-reference-drift` | 8 | 21 | 5/19 +3 no-chunk | 0 |
| P8 | `implement/fix-loop/contract.spec-reality-gap` | 7 | 21 | 6/19 | 3 |
| P9 | `*/*/tooling.host-shell` | 10 | 20 | 8/19 +2 no-chunk | 0 |
| P10 | `implement/fix-loop/contract.test-expectation` | 8 | 20 | 7/19 | 0 |
| P11 | `wrap-session/reconcile/contract.false-positive-proposal` | 7 | 20 | 7/19 | 0 |
| P12 | `*/*/contract.narrow-basis-claim` | 9 | 17 | 8/19 | 0 |
| P13 | `implement/code/tooling.hook-friction` | 8 | 17 | 7/19 | 0 |
| P14 | `wrap-session/gates/tooling.result-not-run-stable` | 3 | 17 | 3/19 | 3 |
| P15 | `phase/plan/retry.synthesis-rework` | 7 | 16 | 7/19 | 0 |
| P16 | `implement/fix-loop/tooling.result-not-run-stable` | 4 | 16 | 4/19 | 0 |
| P17 | `wrap-session/curation/ambiguity.filter-borderline` | 10 | 12 | 10/19 | 0 |
| P18 | `wrap-session/reconcile/contract.cascade-miss` | 9 | 12 | 7/19 | 0 |
| P19 | `wrap-session/route-resolve/ambiguity.trajectory-halt` | 3 | 12 | 2/19 +1 no-chunk | 1 |
| P20 | `*/*/contract.structural-blind-spot` | 3 | 10 | 2/19 +1 no-chunk | 1 |
| P21 | `phase/plan/input.research-thin` | 5 | 9 | 5/19 | 0 |
| P22 | `implement/fix-loop/tooling.environmental` | 4 | 9 | 3/19 | 0 |
| P23 | `wrap-session/reconcile/input.report-insufficient` | 7 | 8 | 7/19 | 0 |
| P24 | `wrap-session/route-resolve/contract.carry-no-owner` | 5 | 8 | 4/19 +1 no-chunk | 0 |
| P25 | `wrap-session/report/input.implement-outcome-unsettled` | 6 | 7 | 6/19 | 0 |
| P26 | `wrap-session/route-resolve/contract.no-sanctioned-channel` | 5 | 7 | 3/19 +2 no-chunk | 0 |
| P27 | `*/*/contract.token-proxy-check` | 3 | 7 | 3/19 | 0 |
| P28 | `new-session/orientation/input.handoff-git-mismatch` | 4 | 4 | 0/19 +4 no-chunk | 0 |
| P29 | `implement/code/input.research-files-wrong` | 3 | 4 | 3/19 | 0 |
| P30 | `implement/fix-loop/contract.instrument-validity` | 3 | 4 | 3/19 | 0 |
| P31 | `wrap-session/curation/ambiguity.tier-routing` | 3 | 3 | 3/19 | 0 |

### P1 — phase/validate · `contract.mechanical-check` — 30 cases · weight 64
**Rate:** 30 cases over 27 runs of the step

**Impact (summed):** iterations 16 · retries 4 · dialogue_rounds 7 · extra_reads 11

**Pattern:** P5's checks fired on the plan as P4 wrote it in 16 of 19 chunks. By reading, the 30 cases fall in four shapes: 14 where a check fired as designed on a P4 authoring omission (check 4 (4), a criterion citing an evidence record no entry produces, 5 times; check 4 (6) 5 times); 9 where the record says no mechanical predicate attempts the defect, or the check reads the form only, and the operator's review or P5's own read caught it (a mutation command in the gate fence twice, a new filter entry green at its baseline with none of the chunk's cases, a guard or check arm with no failing counterpart, a claimed acceptance wider than what the steps prove, a ledger-note line for an unverified capability); 4 size or check-7 warnings that fire by construction on a revised plan; 3 corrections of P5's own first-form guards.

**Evidence:** ALL 30 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | the plan listed three direct cargo-mutants runs as [[gate]] entries although testing.md and test-plan 10 state no chunk mutation gate; no mechanical check reads the fence for mutation commands; the operator review caught it | dialogue_rounds 1 · iterations 1 | `2026-10-02T19:34:01Z-b` |
| 2026-10-02-epoch-2b-cleanup | size check warned: plan 505 lines against the ~150-400 guide (under +50%) | — | `2026-10-02T19:34:01Z-d` |
| 2026-10-03-mutation-scoring-completion | the plan listed 4 run --mutants scoring runs as [[gate]] entries against the founder's 2026-09-28 rule that a chunk gate block holds no mutation entries; no P5 predicate checks for it, the operator's review caught it, and the operator named it a recurrence of the clean… | dialogue_rounds 1 | `2026-10-03T22:50:28Z-b` |
| 2026-10-03-mutation-scoring-completion | REQUIRED-RESOLUTION fired on checks 4 (4) (M3 criterion's outcomes.json had no artifact producer), 4 (6) (remove-the-guard one-shot unnamed; coverage-regex criterion unguarded) and 9 (runner-only scope bullet beside a platform slot reading none); all resolved in plan/r… | extra_reads 3 | `2026-10-03T22:50:28Z-c` |
| 2026-10-04-windows-boundary-mutation-workflow | check 4 (9): the P4-authored uses-pinned probe (grep \| grep -cv) baselined green on an absent workflow file (grep -c prints 0, exit 1 = the passing reading); resolved by a test -f prefix, re-baselined red | iterations 1 | `2026-10-04T02:01:02Z-b` · .andromeda/runs/2026-10-04T01-32-13-phase/p5-baseline.sh |
| 2026-10-04-windows-boundary-mutation-workflow | check 5: a {p} placeholder in an Implementation-notes command (cargo mutants --list --package {p}) leaked from research's derivation; replaced by <package> | iterations 1 | `2026-10-04T02:01:02Z-c` |
| 2026-10-04-readiness-gate-and-timing-constants | check 4 (2) REQUIRED-RESOLUTION: plan touched src/cmd/run.rs (boot path by shape, test-plan declares no list) with no smoke entry; P4 authoring omitted it, P5 added the 4-entry harness smoke and baselined it green | iterations 1 · extra_reads 2 | `2026-10-04T04:57:44Z-b` |
| 2026-10-04-readiness-gate-and-timing-constants | plan carried an Expected-amendments matrix#v1-21 ledger-note line for a not-verified cap whose note P5 itself writes; check 2 reads only that the form names matrix#, not the cap's status, so it passed pre-word and matrix.py audit surfaced it after the yes | iterations 1 | `2026-10-04T04:57:44Z-c` |
| 2026-10-04-confirmed-send-with-cl-1-records | check 2 required three fixes to the P4 plan: two Expected-amendments entries used the matrix ledger-note form for unverified caps (v1-29, v1-10), fuzz/fuzz_targets/vt100_feed.rs was changed by step 6 but absent from the touchpoints, and two unchanged files sat under Fi… | iterations 1 | `2026-10-04T06:09:12Z-b` · viola-0.1.0/chunks/2026-10-04-confirmed-send-with-cl-1-records/plan.md |
| 2026-10-04-confirmed-send-with-cl-1-records | the operator's review sharpened two Expected-amendments lines (test-plan §7, obs-plan §7) that named only G2 while the F4 home sits outside G2, G4 and the secret scan; no predicate reads amendment wording against the held-widening section; the edit re-ran the full chec… | dialogue_rounds 1 · iterations 1 | `2026-10-04T06:09:12Z-c` · viola-0.1.0/chunks/2026-10-04-confirmed-send-with-cl-1-records/plan.md |
| 2026-10-04-dialog-answers-by-dialog-id | check 4 (0) fired correctly on a self-inflicted break: a sed edit put an apostrophe inside a TOML literal-string note and the baseline run exited 3 UNPARSED at line 34; the quote was removed and the set re-run | retries 1 | `2026-10-04T14:20:02Z-b` |
| 2026-10-04-the-wheel | check 4 (9)'s baseline run caught a plan entry using `agent-run.sh run --e2e`, a selector the harness rejects (exit 2 usage); the plan copied the form from .claude/rules/testing.md, which documents `run --e2e --filter` as the one-test recipe though the harness has only… | iterations 1 · extra_reads 2 | `2026-10-04T18:09:05Z-b` · .andromeda/runs/2026-10-04T17-30-10-phase/ (baseline 7.log) |
| 2026-10-04-the-wheel | a unit gate filter `test(/viola_core\|viola_channel\|viola_pty/)` keyed test NAMES on crate names and would select nothing; caught by reading at P5 (no predicate attempts filter vacuity on a green baseline), rewritten to package() before its baseline | iterations 1 | `2026-10-04T18:09:05Z-c` · .claude/rules/verification-harness.md 2026-09-27 addition |
| 2026-10-05-real-cli-verify-probes | check 8 (mechanism reach) fired REQUIRED-RESOLUTION: step 7 named src/cmd/run.rs:487 passing SIGNATURES on cli_verified, but Launched (run.rs:156-164) carries no cli_verified into pump_child; step 7 rewritten to thread it | extra_reads 2 | `2026-10-05T06:22:20Z-b` |
| 2026-10-05-real-cli-verify-probes | check 4 (4) fired REQUIRED-RESOLUTION: the budget criterion asserts evidence/live-sessions.md, which no gate entry produces; resolved by Test Commands prose naming it a hand-kept counter | — | `2026-10-05T06:22:20Z-c` |
| 2026-10-05-real-cli-verify-probes | check 7 (code-graph consulted) WARNed by construction on a revision run: the chunk's tree-query trace lives in the take-up run dir and the revision re-queried nothing | — | `2026-10-05T08:33:46Z-b` · .andromeda/runs/2026-10-05T00-16-17-phase/tree-query-2026-10-05-real-cli-verify-probes.json |
| 2026-10-05-real-cli-verify-probes | check 7 WARN: no tree-query trace in the revision run dir; the take-up run's trace (00-16-17-phase) stands, and revision 3 cites no new symbol | — | `2026-10-05T09:16:30Z-b` |
| 2026-10-05-dialog-rows-and-re-probe | check 2 FAILed: plan's Expected amendments (wrap) named CLAUDE.md's generated line and a route minting, neither a spec master; moved under 'Not amendments, owned by other wrap channels' (cascade, route-resolve) | iterations 1 | `2026-10-05T11:46:53Z-b` |
| 2026-10-05-dialog-rows-and-re-probe | check 4 (4) REQUIRED-RESOLUTION: an acceptance criterion cites evidence/live-sessions.md, which no entry produces; resolved by prose naming it the hand-written session ledger | iterations 1 | `2026-10-05T11:46:53Z-c` |
| 2026-10-05-dialog-rows-and-re-probe | a plan defect no mechanical predicate attempts, caught at the operator's review: the free_k capture-race fix (a new guard) carried no red-before-green control (testing.md 2026-09-25 rule); added as step 3's forced-race unit test with both readings in evidence/capture-r… | iterations 1 · dialogue_rounds 1 | `2026-10-05T11:46:53Z-d` |
| 2026-10-06-local-command-and-paste-framing-rows | the operator review caught that two of the three new check arms had no failing case: without the fake agent replay option both paste rows pass on the echo, so no listed test showed the arms read their capture; no mechanical predicate reads an arm for a failing counterp… | dialogue_rounds 1 | `2026-10-06T19:50:32Z-b` · .andromeda/runs/2026-10-06T19-18-12-phase/relay-3.md |
| 2026-10-06-local-command-and-paste-framing-rows | an inline guard authored at P4 matched env::var as a bare prefix, so an added line holding the existing vars_os call would have tripped it; found while minting its controls, corrected to the call form before the baseline was written, and a must-pass control line was ad… | retries 1 | `2026-10-06T19:50:32Z-c` · .andromeda/runs/2026-10-06T19-18-12-phase/p5-guard-controls.sh |
| 2026-10-06-local-command-send-outcomes | two checks needed resolution on the plan as P4 wrote it: check 4 (6), three criteria named CI jobs and one named a grep that were not gate entries; check 4 (9), new = true stood on five entries whose scope tokens earlier plans already name (the p-send-smoke session of… | iterations 1 · extra_reads 2 | `2026-10-06T23:04:21Z-b` · .andromeda/runs/2026-10-06T22-33-26-phase/planlint-2026-10-06-local-command-send-outcomes.json |
| 2026-10-07-test-homes-off-the-contended-volume | the plan as first written owed four resolutions that only P5's own read found, the two tools reading clean: check 4 (2) a harness boot file in the modify-set with no smoke entry, check 4 (6) two gates a criterion named and the fence did not list, check 4 (4) evidence r… | iterations 1 | `2026-10-07T07:38:33Z-b` · .andromeda/runs/2026-10-07T06-14-00-phase/dry-run-2.out |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | Checks 4 (4) and 4 (6) each needed a resolution on the first pass: the criteria name evidence records no entry produces, and a red-before-green control that mutates source; the section's prose gained the no-artifact sentence and the one-shot control sentence. Check 3 m… | iterations 1 | `2026-10-07T09:55:23Z-b` |
| 2026-10-07-send-waits-out-the-paste-hint | check 4 (6) required a resolution: the criterion on the two moved nextest kill lines named a file fact no listed entry read; a probe entry was added, and its first form (one count of either line, expecting 2) could pass with the wrong line moved, so it was rewritten as… | retries 1 | `2026-10-07T12:19:07Z-b` · .andromeda/runs/2026-10-07T11-51-17-phase/ |
| 2026-10-07-send-waits-out-the-paste-hint | check 4 (7)'s ignore read, run on the live entry's output path under target/e2e-home, exited 128 (pathspec beyond a symbolic link: the base is a link to tmpfs on this host) where the check knows only exits 0 and 1; the read was repeated on the link itself and exited 0 | retries 1 | `2026-10-07T12:19:07Z-c` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | a new filter entry read green at its baseline on one existing case of the same name part while none of the chunk's cases existed; check 4 (9) records such a green and asks for no count atom, so the entry went to the review asserting only exit and ok, and the operator's… | dialogue_rounds 1 | `2026-10-07T14:12:33Z-b` · .andromeda/runs/2026-10-07T13-36-31-phase/p5-baseline-unit.out |
| 2026-10-08-first-live-test-and-self-drive | the plan claimed a capability whose acceptance names three dialog kinds while its steps and its reader proved the never-renders clause for one, and listed the other two as owed in its own notes; no mechanical predicate compares a claimed acceptance's enumerated cases w… | dialogue_rounds 1 · iterations 1 | `2026-10-08T06:23:46Z-b` · .andromeda/runs/2026-10-08T04-59-25-phase/p5-review.md |
| 2026-10-08-first-live-test-and-self-drive | check 6 (size) WARNED: a twice-revised plan carries three plans' done-and-standing steps and reads 668 prose lines against the 150 to 400 band; the review went on with the warning named | — | `2026-10-08T08:30:01Z-b` · .andromeda/runs/2026-10-08T07-43-15-phase/planlint-2026-10-08-first-live-test-and-self-drive.json |

**Proposal:** Three directions, each for the founder's judgment. (1) The recurring check-4 and check-2 resolutions are stated at P5 and absent at P4: carrying those same sentences into the plan template or P4's self-check would move the correction one step earlier. (2) The nine cases no predicate reaches are candidate planlint predicates where the shape is mechanical: a mutation command inside a `[[gate]]` entry; a `new = true` filter entry whose baseline is green with no count atom; a new guard with no known-failing control. (3) Check 7 and the size band read a revision run as a first run: reading the take-up run's trace and counting only changed lines would remove the by-construction warnings. Absorbed mid-epoch: the mutation-gate rule was curated into Session Additions on 2026-10-04 and does not recur after; the count atom was added by review on 2026-10-07.

### P2 — cross-step · `contract.premise-falsified` — 16 cases · weight 50
**Steps:** implement/code 8 · wrap-session/route-resolve 2 · implement/fix-loop 2 · phase/research 2 · implement/smoke 1 · phase/plan 1

**Impact (summed):** iterations 2 · reformulations 1 · dialogue_rounds 2 · extra_reads 33 · halted 4 · soft_exit 5

**Pattern:** A premise an authored artifact states was falsified at verification, in 10 chunks and at two 0-pending wraps. Seven of the 16 are premises about the live `claude` CLI (print-mode dialogs, the trust dialog's key, the external-imports dialog, the plan-approve shape, the wrapped prompt's newlines, `plansDirectory`, the child's argv); three are a relay's or a CARRY's count or coordinate; the rest are leak and timing predictions. Five cases carry a halt or soft exit, each a stop on a live reading, most at the plan's own step-0 STOP clause.

**Evidence:** ALL 16 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) (wrap-session/route-resolve) | the relay's item A named '13 cfg(unix) mutants' for a WSL leg; the code-audit table lists 12 cfg(unix) and 1 cfg(all(windows, not(target_arch = x86_64))) that WSL cannot measure; the cleanup CARRY carries the measured split | — | `2026-10-01T12:36:15Z-c` · .andromeda/runs/2026-10-01T12-19-55-wrap/ |
| 2026-10-02-epoch-2b-cleanup (implement/code) | plan step 4 premised that git read-only object files make TempDir's drop leave the repo; the guard-removed test read green and a 225-test self-test run leaked 0 dirs | iterations 1 | `2026-10-03T04:04:41Z-b` · viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/leaks.md |
| (no chunk) (wrap-session/route-resolve) | the relay cited :76 for First live test (it is :80) and asked to retire a host-reds CARRY already absent from the route; both caught by re-reading the tail before editing | extra_reads 1 | `2026-10-03T22:17:30Z-c` · .andromeda/runs/2026-10-03T22-10-25-wrap/adaptation-record.md |
| 2026-10-04-windows-boundary-mutation-workflow (implement/smoke) | the plan's leak acceptance predicted 0 .tmp* after one full viola-e2e run under terminate=wait; it read 38 — 17 from nextest TIMEOUT/SIGKILL kills (the path wait cannot drain, which the acceptance's 0 did not allow for) and 21 base.rs Pass-fixture repos left half-remov… | extra_reads 12 | `2026-10-04T03:31:03Z-b` · viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/leak.md |
| 2026-10-04-dialog-answers-by-dialog-id (implement/code) | the plan's dialog probe (step 7) and four ledger rows (step 5) assume a print-mode turn can raise PreToolUse AskUserQuestion/ExitPlanMode and PermissionRequest; three measured print-mode runs on 2.1.287 (default, plan mode, explicit --tools) had neither tool available… | soft_exit 1 | `2026-10-04T11:54:44Z-b` · viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/print-mode-dialog-probe.md |
| 2026-10-05-real-cli-verify-probes (phase/plan) | the held widening (readiness-gate plan.md:388-390) specified --no-session-persistence for the interactive probe and no trust dialog; the 2.1.288 --help says the flag is print-only and an interactive session shows the workspace trust dialog, which became a live founder… | dialogue_rounds 1 · extra_reads 2 | `2026-10-05T06:18:34Z-b` |
| 2026-10-05-real-cli-verify-probes (implement/code) | plan premises falsified at step 0: one accept key (Enter selects 'No, exit' on 2.1.288), SessionStart before trust (no hook fires while the dialog is up), and a fresh probe dir showing the dialog (trust is inherited from the trusted repo, which the record and local-liv… | halted 1 · soft_exit 1 | `2026-10-05T06:32:48Z-b` · viola-0.1.0/chunks/2026-10-05-real-cli-verify-probes/evidence/screen-probe-2.1.288.md |
| 2026-10-05-real-cli-verify-probes (implement/code) | plan revision 2 premised Run B under a trusted parent starts with no modal; live session 3 showed 'Allow external CLAUDE.md file imports?' because the repo CLAUDE.md @-imports a file outside the subdir cwd, keyed on the git root's project config | soft_exit 1 · halted 1 | `2026-10-05T08:42:29Z-b` · viola-0.1.0/chunks/2026-10-05-real-cli-verify-probes/evidence/screen-probe-2.1.288.md |
| 2026-10-05-dialog-rows-and-re-probe (implement/code) | live 2.1.288 plan mode: dialog::decision_body's bare plan approve (PreToolUse allow, no updatedInput) was followed by a second ExitPlanMode PermissionRequest and no PostToolUse; the dialog.rs doc and the plan_approved snapshot state it as the S7 approve | halted 1 · soft_exit 1 | `2026-10-05T11:59:55Z-b` · viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/step0-dialog-shapes.md |
| 2026-10-05-dialog-rows-and-re-probe (implement/fix-loop) | the operator-pass hypothesis that hook-process count drove the coverage slowdown was falsified by a timing-only CI round (17-hook verifies no faster than 29-hook; processes alive at start did not move wall time); the decision's contention-removal step was not taken | iterations 1 | `2026-10-05T14:26:27Z-b` · viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/ci-rounds.md |
| 2026-10-05-permission-end-to-end (implement/code) | plan's F1 probe gate greps TIMEOUT \[4x.xxxs\] unpadded, but nextest 0.9.146 prints TIMEOUT [ 45.004s] with column padding; evidence records verbatim lines plus a padding-collapsed table (the :86 record's form), which the gate reads | reformulations 1 | `2026-10-05T15:27:29Z-b` · .andromeda/runs/2026-10-05T15-22-03-implement/f1-ci.log |
| 2026-10-06-local-command-and-paste-framing-rows (implement/code) | step 0 measured the 2.1.287 wrapped prompt as two newlines, the pasted_content pair, one newline; HEAD prompt_text keeps the three newlines, so the long-paste arm as planned (equality with the compiled text, normalisation unchanged) cannot pass; plan STOP 3 fired | halted 1 · soft_exit 1 | `2026-10-06T20:02:31Z-b` · viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/step0-shapes.md |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host (phase/research) | The folded CARRY and the prior research it cites infer from the R8 strip that a verify child cannot be addressed by another session; the code verifies the strip only, and this session's peer listing shows wrapped builder sessions listed as peers, so the inference is un… | extra_reads 2 | `2026-10-07T09:07:21Z-b` · research.md M4 |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (phase/research) | scope's folded-red bullet and the directive rested on naming the dead process from the refused profile; the run's harness artifact holds 10 members and no profile, because the upload takes target/agent-run/ and the profile sat in target/llvm-cov-target/ | extra_reads 8 | `2026-10-07T13:56:05Z-b` · viola-0.1.0/chunks/2026-10-07-a-send-ending-in-a-newline-is-confirmed/research.md |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (implement/code) | Plan step 7 and research's P4 note state that a child's arguments are the test's to give; child_launch prepends --plugin-dir <dir>, libtest refuses it and exits 101 with nothing listed, so no test body of a child started through start ever runs. Found by reading child_… | extra_reads 6 · dialogue_rounds 1 | `2026-10-07T14:34:57Z-b` · viola-0.1.0/chunks/2026-10-07-a-send-ending-in-a-newline-is-confirmed/evidence/profraw-red-green.md |
| 2026-10-08-first-live-test-and-self-drive (implement/fix-loop) | plan step 6 states that the launcher's plansDirectory key keeps the CLI's plan file out of the user's CLI directory; the live run left the named directory empty and one plan file in the CLI's default plans directory; surfaced in the report, the file left in place | extra_reads 2 | `2026-10-08T08:56:11Z-f` · viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-run.md |

**Proposal:** The STOP clauses stopped the work as designed; the cost the records show is what follows a STOP: five chunks ran phase two or three times (see Mechanism health, coverage). A direction: for a chunk whose scope names undocumented behaviour of an external program, let research take the bounded live measurement the plan would otherwise place at step 0, so the plan is written on the measured shape and the revision loop is not entered. The relay and CARRY cases are a second, smaller shape: a count or `:NN` coordinate copied from a relay was stale on arrival; a re-read of the tail before the edit caught each.

### P3 — cross-step · `recall.corpus-recurrence` — 19 cases · weight 41
**Steps:** wrap-session/curation 18 · wrap-session/reconcile 1

**Impact (summed):** iterations 1 · retries 16 · reformulations 1 · dialogue_rounds 2 · extra_reads 1 · deferred 3

**Pattern:** A rule the curated corpus already states was met again, in 15 of 19 chunks. Eleven of the 19 records name one event: the project's Bash guard refusing a heredoc with a file target, a rule the always-loaded host rule file has stated since 2026-09-28. The others: `rm` inside a compound (2), an exit or listing read through a pipe (2), a `cd` the guard refuses, a zero-is-healthy count probe, a bounded mutant wait, the no-mutation-gate rule, the hygiene check on a path through a directory named home, a clipped long-line view, a time written ahead of the clock, `pkill -f`.

**Evidence:** ALL 19 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion (wrap-session/curation) | testing.md's and verification-harness.md's bodies already said no chunk/pre-push/CI mutation gate, yet phase put cargo-mutants runs into the gate block at two consecutive chunks (28->25, 28->24), each caught at P5 review; the founder rule is now a Session Additions ent… | dialogue_rounds 2 | `2026-10-04T01:22:46Z-d` |
| 2026-10-04-windows-boundary-mutation-workflow (wrap-session/curation) | host-win32.md 2026-09-28 (extended 2026-09-29) already says the guard refuses a heredoc whose target is a file; this session still sent a heredoc append to an evidence file and the PreToolUse hook blocked it (and once more blocked a telemetry payload that merely quoted… | retries 2 | `2026-10-04T04:22:08Z-b` |
| 2026-10-04-confirmed-send-with-cl-1-records (wrap-session/curation) | host-win32.md 2026-09-28 (the Bash guard refuses a heredoc redirected to a file) states the rule correctly and the session reproduced it twice at implement (an append heredoc to fuzz/Cargo.toml, a heredoc writing a scratchpad script) and once more at this wrap, when a… | retries 3 | `2026-10-04T08:56:53Z-b` |
| 2026-10-04-dialog-answers-by-dialog-id (wrap-session/curation) | the host-win32.md entry (the Bash guard refuses a heredoc redirected to a file) was curated and the wrap still issued a cat heredoc append to a run-dir file; the guard refused it | retries 1 | `2026-10-04T17:17:58Z-b` |
| 2026-10-04-the-wheel (wrap-session/reconcile) | host-win32.md 2026-09-28/29 Session Addition (a heredoc redirected to a file is refused) recurred: a cat heredoc append was refused by the guard on the Linux host | retries 1 | `2026-10-04T20:59:31Z-c` |
| 2026-10-04-running-turn-refusal (wrap-session/curation) | host-win32.md's body rule that a zero-is-healthy count probe exits non-zero on no matches was correct, yet the phase-authored `git grep \| wc -l` gate with an `exit 0` atom reproduced it under the gate shell's pipefail (entry 8 red on its satisfied subject) | extra_reads 1 | `2026-10-04T22:37:37Z-b` |
| 2026-10-04-running-turn-refusal (wrap-session/curation) | testing.md's bounded-mutant-wait entries (2026-09-24; 2026-09-25 extended 2026-10-04 on fixed-clock remove-the-guard controls) were correct, yet a new unit test parked a wrongly accepted send on a fixed-clock confirm window and its control read a 120 s TIMEOUT before t… | iterations 1 | `2026-10-04T22:37:37Z-c` |
| 2026-10-05-real-cli-verify-probes (wrap-session/curation) | the gate hygiene P1 form again refused committed evidence spelling a path through a dir named home; session-learnings 2026-09-29 already records that hazard and its remedy | retries 1 | `2026-10-05T11:01:20Z-b` · viola-0.1.0/chunks/2026-10-05-real-cli-verify-probes/evidence/operator-pass.md |
| 2026-10-05-dialog-rows-and-re-probe (wrap-session/curation) | two curated host defects recurred in the chunk: pkill -f self-match (host-win32.md 2026-09-25) and a heredoc to a file refused by the Bash guard (host-win32.md 2026-09-28/29) | deferred 2 | `2026-10-05T14:47:29Z-b` · .andromeda/runs/2026-10-05T14-27-34-wrap/curation.md |
| 2026-10-06-local-command-and-paste-framing-rows (wrap-session/curation) | two host-shell recurrences past entries that state the rule: an rm -rf chained with a listing was refused by the permission layer (host-win32.md, compound commands), and a cat heredoc with a file target was refused by the Bash guard (host-win32.md 2026-09-28/29); neith… | — | `2026-10-06T22:04:50Z-b` |
| 2026-10-06-local-command-send-outcomes (wrap-session/curation) | a cat heredoc redirected to a scratchpad file was refused by the Bash guard at implement; host-win32.md's 2026-09-28 entry and its 2026-09-29 extension state the rule, and the handoff already carried this recurrence from the previous wrap | retries 1 | `2026-10-07T00:05:14Z-c` |
| 2026-10-06-local-command-send-outcomes (wrap-session/curation) | the report stated that a test-plan line carried no count from a cut-limited grep row of a 3692-char line; host-win32.md's long-single-line section says such a view never answers a membership question; the test-plan detector read the line whole and caught it | reformulations 1 | `2026-10-07T00:05:14Z-d` |
| 2026-10-07-test-homes-off-the-contended-volume (wrap-session/curation) | A scratchpad script was written through a cat heredoc with a file target and refused by the Bash guard, past the host rule file's 2026-09-28/29 entry that states the rule; the handoff already carried this recurrence from earlier sessions. | retries 1 | `2026-10-07T08:35:54Z-c` |
| 2026-10-07-test-homes-off-the-contended-volume (wrap-session/curation) | A compound call carrying rm -rf beside other steps was denied whole, past the host rule file's Compound commands rule; the handoff already carried this recurrence too. | retries 1 | `2026-10-07T08:35:54Z-d` |
| 2026-10-07-test-homes-off-the-contended-volume (wrap-session/curation) | A time was written into an evidence file two minutes ahead of the clock (estimated, not read); the stamp hook refused the write and the time was read from date -u. The hook exists for this defect. | retries 1 | `2026-10-07T08:35:54Z-e` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host (wrap-session/curation) | the curated host rule on the Bash guard refusing a heredoc redirected to a file states the rule correctly and the session reproduced the refusal once | retries 1 | `2026-10-07T11:11:38Z-b` |
| 2026-10-07-send-waits-out-the-paste-hint (wrap-session/curation) | the always-loaded host rule says to read an exit from the bare command, and the operator pass read its push through a tail pipe, with PIPESTATUS read beside it and the upstream distance read after; logged for the handoff as recurrence-despite-learning | deferred 1 | `2026-10-07T13:14:51Z-c` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (wrap-session/curation) | A cat heredoc with a file target was written at implement and refused by the Bash guard; the host rule file's 2026-09-28 entry states that refusal. Logged to the handoff as a recurrence, the third wrap to carry it. | retries 1 | `2026-10-07T19:32:55Z-d` |
| 2026-10-08-first-live-test-and-self-drive (wrap-session/curation) | three shell rules the host leaf already states were met again in this session: a cd into a subdirectory refused by the guard, a heredoc with a file target refused by the guard, and a tool listing filtered through a pipe instead of run bare | retries 2 | `2026-10-08T09:31:57Z-c` |

**Proposal:** The records show the rule's presence in an always-loaded file does not stop the act, while the hook that refuses it bounds the cost at one retry. Directions: (1) the guard's refusal text could carry the remedy itself (the Write tool, run by path), so the retry needs no recall; (2) the capture side currently files one hook-caught refusal under several types (Mechanism health, classification scatter): one signal for a refusal a hook caught as designed would keep the count honest without a friction record per occurrence; (3) whether the heredoc habit is fed by the pipeline's own recipes (the evolve append is a heredoc to stdin) is a hypothesis the records do not settle. See level candidate L1 for the problem-fact side.

### P4 — implement/code · `input.plan-step-ambiguous` — 19 cases · weight 37
**Rate:** 19 cases over 28 runs of the step

**Impact (summed):** retries 1 · reformulations 12 · dialogue_rounds 1 · extra_reads 7 · soft_exit 1

**Pattern:** A plan step left a case open or conflicted with another step, an acceptance line or an existing oracle at the moment of coding, in 15 of 19 chunks. The implementer chose and carried the choice to the report (12 reformulations in total); one case stopped as a question to the operator. By reading: a step against an existing test, contract, gate entry or script the plan did not name (4); two passages of the same plan against each other (4); a step silent on a case (11).

**Evidence:** ALL 19 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | plan step 5 asks that every runner call be /usr/bin/env -i HOME=… PATH=…, but the passwd probes that produce HOME cannot carry it; the probes carry PATH (system dirs) only | — | `2026-10-03T23:02:17Z-b` |
| 2026-10-04-windows-boundary-mutation-workflow | plan step 2's mod-line rule did not say whether modules declared inside a whole-file Windows module are gated too; implemented transitive propagation with a synthetic known positive and carried it to the report as a deviation | — | `2026-10-04T02:06:22Z-b` |
| 2026-10-04-readiness-gate-and-timing-constants | step 11 named init_repo and Pass::commit; run.rs test_support::mini carries two more throwaway-repo git calls with the same identity list; the edit script's count assertion caught it | retries 1 | `2026-10-04T05:06:21Z-b` |
| 2026-10-04-confirmed-send-with-cl-1-records | three plan details underdetermined the change and were settled in the impl, to surface in the report: the -32602 fault for bad send params (ProtocolError::InvalidParams added), the Git Bash warning placed in clap value parsers (main.rs frozen), and the claim/settle two… | extra_reads 3 | `2026-10-04T06:27:45Z-c` |
| 2026-10-04-wait-and-last | plan step 8 says role_of returns Cli for every other first word, but the existing literal row --help -> Other would then flip; kept flags (a leading '-') as Other so the existing oracle stands, surfaced as a deviation | reformulations 1 | `2026-10-04T10:08:48Z-b` |
| 2026-10-04-the-wheel | step 5.2 lists release -> driver/release without saying whether a release on a wheel the driver already holds records a cause change; implemented as no change, to surface in the report | — | `2026-10-04T18:27:23Z-c` |
| 2026-10-04-running-turn-refusal | plan step 5 and the turn's-life acceptance require a human-origin prompt to make the next send refuse turn-running, unreachable because the human-typing rung precedes it and the only wheel return clears the turn | reformulations 1 | `2026-10-04T22:05:32Z-b` |
| 2026-10-05-real-cli-verify-probes | plan steps underdetermined or conflicted at 6 points: &dyn Clock vs a pump-thread sink, the no-screens case vs the 7 s bound, the Stop-less case vs a PROBE_DEADLINE turn wait, row text out of Screen, screen_is_clean 'after scrub' semantics, merge_stamp literals vs type… | reformulations 6 | `2026-10-05T09:39:17Z-b` |
| 2026-10-06-local-command-and-paste-framing-rows | step 0 names two forms for the session end (the earlier chunk step 0 ended by kill, Run B ends by Ctrl-C) and no line count for the long text; chose Ctrl-C into the settled input box and one line, both named in the report | extra_reads 2 | `2026-10-06T20:02:31Z-c` · viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/step0-shapes.md |
| 2026-10-06-local-command-and-paste-framing-rows | step 8 says a set without the framing variants is not compared, and step 7 moves 2.1.288 to the drift-only list, but the existing dialog-replay contract runs verify over every dialog set and asserts exit 0: with seventeen rows 2.1.288 can no longer exit 0, and with the… | extra_reads 1 | `2026-10-06T20:35:24Z-c` |
| 2026-10-06-local-command-and-paste-framing-rows | step 7 asks for the unit literals of src/cmd/verify.rs at seventeen rows, and gate entry 10 greps that file for a local-command literal: the seventeenth step line's words hold one, so the unit test asserts that line by its counter, id and verdict and leaves its words t… | — | `2026-10-06T20:35:24Z-d` |
| 2026-10-06-local-command-send-outcomes | step 7 fixed the hint case's hold at 6000 ms and sized it against the 20 s CI kill only; the case has no window prefix, so the mutants profile kills it at 10 s and it measured 10.549 s; the same step's wait (the turn-ended record alone) left the order of the Stop hook'… | reformulations 2 | `2026-10-06T23:23:25Z-b` · viola-0.1.0/chunks/2026-10-06-local-command-send-outcomes/evidence/paste-hint-send.md |
| 2026-10-07-test-homes-off-the-contended-volume | Plan step 3 (replace the keeper call at its call site with the bare create_dir_all, run case 2 red) conflicts with steps 1 and 2 (each case over its own temp dir with a link made by the test, so it calls the keeper and never the call site): the call-site swap cannot tu… | reformulations 1 | `2026-10-07T07:48:25Z-b` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | the STOP list's header (a STOP ends the live work before another start) against STOP 5, which the plan evaluates at step 7 after every live round; the falsifying shapes were known after start 1, and the live rounds went on in plan order | — | `2026-10-07T10:30:18Z-b` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | step 4 says every verb with --json while the layouts acceptance asks for the live refusal's lines as printed; resolved by one added human-mode send in the window | — | `2026-10-07T10:30:18Z-c` |
| 2026-10-07-send-waits-out-the-paste-hint | plan step 4 asks for cases where the wheel or a turn moves while a verified gate waits, without saying how the wait is produced; a feed during the wait cannot be joined because wait_ready reads the clock under the model lock, so the cases wait on the not-quiet arm with… | reformulations 1 | `2026-10-07T12:28:55Z-b` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | Step 6's rule for the census directory's name is a character class that the words dot and dot-dot satisfy; the step does not say whether they are names. They were refused (exit 2) and the choice is carried to the P4 report as a detail the plan did not state. | — | `2026-10-07T15:12:49Z-b` |
| 2026-10-08-first-live-test-and-self-drive | step 6 read the live start by lines 1 to 3 of the event log and had no arm for a wheel record of cause human-input between budget-gate and session-start; the stop was put to the operator as a question | dialogue_rounds 1 · soft_exit 1 | `2026-10-08T07:41:13Z-b` · viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/terminal-replies.md |
| 2026-10-08-first-live-test-and-self-drive | plan step 5c gives evidence/key-probe-own.sh the key as a second argument; the standing script took the font size alone and typed a fixed key, so it was extended before the step could run | extra_reads 1 | `2026-10-08T08:56:11Z-b` |

**Proposal:** The surfacing channel worked in every case (each choice reached the report). A direction for the share that is a step against an existing oracle: together with `contract.test-expectation` (existing tests pinning the changed behaviour) and `input.research-files-wrong` (files with such tests missing from the lists), these are one shape seen at three steps: research's sweep did not list the tests that pin what the chunk changes. A named pin sweep at research (tests asserting the changed literal, code or method name; the code graph does not see a test pinning a method name as a call) would feed all three. See chains X1, X2 and X4.

### P5 — wrap-session/reconcile · `contract.in-pass-correction` — 16 cases · weight 36
**Rate:** 16 cases over 19 runs of the step

**Impact (summed):** iterations 6 · retries 12 · reformulations 2 · extra_reads 1

**Pattern:** A first write inside reconcile was wrong and the pass's own re-read, sweep or tool check caught it before the commit, in 15 of 19 chunks. By reading: a sidecar payload off form (two blocks, a trailing blank, over 3 000 B, a wrong section name) 5 times; a hand-typed count in the fan-out record 3 times; a raw twin saved with a host path respelled twice; a cascade pattern id over 16 characters once here and once untyped; a sentence wrong on first write in most of the rest.

**Evidence:** ALL 16 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | the test-data-bootstrap remnants sentence was first written without its as-measured pointer (caught on re-reading the apply rule's epistemic-status clause); the obs-plan sidecar entry first named its section Pipeline integration (caught by a header grep: the line sits… | iterations 2 | `2026-10-03T07:54:56Z-c` · .andromeda/runs/2026-10-03T07-46-03-wrap/fanout-results.md |
| 2026-10-03-mutation-scoring-completion | security-plan.md: a two-step Edit of the 2770-char line 445 consumed the newline before the next bullet (caught by the sweep window showing the joined text, restored, diff re-read at 4 lines); and lines 337/445 first carried retirement-history clauses in the body, trim… | iterations 2 | `2026-10-04T01:20:42Z-e` |
| 2026-10-04-windows-boundary-mutation-workflow | bootstrap-phases key file: the new history sentence first said 'the plain fail-fast = true this replaced' (wait replaced immediate, not fail-fast=true); caught on the cascade sweep's re-read of the timeout-misgrade row and rewritten | iterations 1 | `2026-10-04T04:20:25Z-c` |
| 2026-10-04-windows-boundary-mutation-workflow | the test-plan sidecar payload first carried two entries in one file; sidecar.py check refused it OFF-FORM (2 blocks), split into two payloads (and a trailing blank trimmed) before append | retries 1 | `2026-10-04T04:20:25Z-d` |
| 2026-10-04-readiness-gate-and-timing-constants | the first architecture sidecar entry landed with a trailing blank line (a split of a two-block payload kept the separator's newline); the second append's --expect-last-prefix '**Ref:**' refused, the one byte was trimmed with an exact-tail assertion and the append re-run | retries 1 | `2026-10-04T05:35:33Z-b` · .andromeda/runs/2026-10-04T05-25-03-wrap/ |
| 2026-10-04-confirmed-send-with-cl-1-records | test-plan.md:434 first write dropped the comma before `vt100_feed` (the anchor replaced 'and `vt100_feed`' without restoring the list separator); the sweep row's text caught it and a second apply restored it | iterations 1 | `2026-10-04T08:55:41Z-d` |
| 2026-10-04-dialog-answers-by-dialog-id | cascade-patterns.toml first carried two ids over 16 chars (cascade.py refused, exit 2); test-plan:989 first said the RUNNER_TEMP case 'stays owed' with no owner, a re-read added ':111' | retries 1 | `2026-10-04T17:16:27Z-e` |
| 2026-10-04-running-turn-refusal | cascade-dispositions.md first said 'Four patterns read 0 rows' while listing five; corrected on re-read before any sidecar entry landed | retries 1 | `2026-10-04T22:36:36Z-b` |
| 2026-10-05-real-cli-verify-probes | the first edit of security-plan's Universal dialog-stamp rule rewrote its head to the ten-row stamp but left the same sentence's tail reading 'that six-row stamp' and 'until `:84`'; a full-line re-read caught it and a second edit fixed it | retries 1 | `2026-10-05T11:00:00Z-d` |
| 2026-10-05-dialog-rows-and-re-probe | fanout-results.md first recorded the architecture return as 16 proposals; a recount of the parsed list found 17 and the record was corrected before any apply | reformulations 1 | `2026-10-05T14:46:12Z-d` · .andromeda/runs/2026-10-05T14-27-34-wrap/fanout-results.md |
| 2026-10-06-local-command-and-paste-framing-rows | three corrections before the commit: an architecture sidecar entry was appended at 3040 B after its check printed OFF-FORM, because the append loop did not gate on the check, and was trimmed to 2997 B in the sidecar and its payload; the fan-out record counted test-plan… | retries 3 | `2026-10-06T22:04:50Z-d` |
| 2026-10-06-local-command-send-outcomes | the design-system sidecar payload first said the CLI component pattern already carried the post-condition trigger; the detector's notes say that pattern states no trigger; the sentence was corrected before the form check and the append | reformulations 1 | `2026-10-07T00:03:32Z-c` |
| 2026-10-07-test-homes-off-the-contended-volume | One site: report.md, the Harness / gate surface bullet, first said CI's inline form of G2 in ci.yml is unchanged; ci.yml carries no inline form, it runs the script in two jobs. Caught by a grep of the workflow taken while the detectors ran, corrected after the fan-out… | extra_reads 1 | `2026-10-07T08:34:21Z-c` |
| 2026-10-07-send-waits-out-the-paste-hint | two first writes corrected before the sweep: an architecture sentence attributed the refusal at the bound to the fake agent where it is proved on the injected clock in unit cases (caught by a re-read of the applied line), and two raw fan-out twins were written with a h… | retries 2 | `2026-10-07T13:13:11Z-d` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | Two run-dir artifacts were wrong on first write: a raw twin was saved with one host path respelled repo-relative and restored verbatim on a re-read of the twin rule; a sweep pattern was written under an id naming another subject and is explained in the dispositions, th… | retries 1 | `2026-10-07T19:30:10Z-d` |
| 2026-10-08-first-live-test-and-self-drive | one sidecar entry payload named its section by the wrong number (2 for 1); a heading read caught it before the append and the payload was corrected and re-checked | retries 1 | `2026-10-08T09:30:39Z-d` |

**Proposal:** Every case was caught in-pass, so the type measures a correction working at the cost of a retry. Directions on the tool side: make the sidecar append refuse when its own check printed OFF-FORM (one entry landed at 3 040 B because the append loop did not gate on the check); state the 16-character id bound where the pattern file's form is given (see U7); derive the fan-out record's counts from the parsed returns instead of typing them.

### P6 — wrap-session/reconcile · `contract.proposal-format` — 4 cases · weight 24
**Rate:** 4 cases over 19 runs of the step

**Impact (summed):** reformulations 20

**Pattern:** Detector returns carried a basis citing source lines the report does not carry, against the prompt's own ban, in three of the four cases (12, 8 and 3 proposals); each was rejected as proposed and its fact re-raised by the orchestrator from the report. The fourth case is the harness neutralising tag-like text in three returns. All four fall in the epoch's last seven chunks (2026-10-06 on).

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-local-command-and-paste-framing-rows | 12 of 52 proposals carried a basis citing a source file location (ledger.rs, boot.rs, tests/*) that the report does not carry, against the prompt's own ban; each was rejected as proposed and raised by the orchestrator from the report, the applied text unchanged by it | reformulations 12 | `2026-10-06T22:04:50Z-b` · .andromeda/runs/2026-10-06T21-43-53-wrap/fanout-results.md |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | three returns arrived with the harness's control-tag neutralisation applied (a backslash after the angle bracket of tag-like text), which on this subject reads as the CLI's own escaped form; no change line was pasted, every body was re-derived from the report | — | `2026-10-07T11:10:34Z-e` |
| 2026-10-07-send-waits-out-the-paste-hint | three primaries (two of test-plan, one of obs-plan) carried a basis naming source lines the report does not carry; they and five dependents were rejected on the re-derivation tell, and each of the eight facts, all in the report, was raised by the orchestrator from the… | reformulations 8 | `2026-10-07T13:13:11Z-b` · .andromeda/runs/2026-10-07T12-57-41-wrap/fanout-results.md |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | Three of the test-plan detector's six proposals carried a basis naming source lines the report does not carry, against the prompt's ban on re-deriving from the codebase; each was rejected as a proposal and its fact, present in the report, raised by the orchestrator. | — | `2026-10-07T19:30:10Z-b` |

**Proposal:** Read with `contract.false-positive-proposal` below: 23 proposals here and about ten there were rejected and re-raised, and one record states the applied text was unchanged by it. A direction: give the detectors the report's facts in a citable form (numbered Changes lines), so a basis can point at the report; or apply the tell only where the fact is absent from the report. Calibration: the `rejected-for-source:{n}` count on the reconcile record lives from 2026-10-08, so only the last chunk could carry it.

### P7 — cross-step · `contract.skill-reference-drift` — 8 cases · weight 21
**Steps:** wrap-session/u35-door 2 · phase/take-up 2 · phase/validate 1 · implement/fix-loop 1 · new-session/orientation 1 · implement/code 1

**Impact (summed):** retries 7 · dialogue_rounds 3 · extra_reads 2

**Pattern:** A letter or reference states a form the deployed tool refuses, or a stamp it no longer prints. Four sites: `promotion.md` §External inputs and the implement letter give the relay snap without `--origin` (3 typed cases here and one untyped, three of them on three consecutive take-ups of 2026-10-05); `gate-contract.md` §Tool states a version behind the one `gate.py` prints, twice, once changing within a session; `registry-contract.md`'s U35 door (the marker form and the lift target), twice; `session-state-contract.md`'s expected-transient list omits the wrap run's own evolve trail.

**Evidence:** ALL 8 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) (wrap-session/u35-door) | registry-contract.md §The U35 door substitutes {marker} = the run dir's name (2026-09-29T15-07-57-wrap), but registry.py RECORD_RE needs a lowercase YYYY-MM-DD-slug marker: verify failed 6x on the record headings, then again 6x on the lowercased 2026-09-29t15-..., pass… | retries 2 · dialogue_rounds 2 | `2026-09-29T15:26:07Z-a` · .andromeda/runs/2026-09-29T15-07-57-wrap/adaptation-record.md |
| (no chunk) (wrap-session/u35-door) | the U35 lift-rewriter prompt says a lift lands under a body heading 'outside the log', but registry.py refuses any target inside a mapped section, K included: 6 lifts (a11y 3, obs 1, test 2) aimed at §3 keys failed verify; the operator routed them to hand-landed key-fi… | retries 1 · dialogue_rounds 1 | `2026-09-29T15:26:07Z-b` · .andromeda/runs/2026-09-29T15-07-57-wrap/u35/k-lifts.toml |
| 2026-10-04-dialog-answers-by-dialog-id (phase/validate) | gate.py prints gate v1.7 while gate-contract.md §Tool states gate.py v1.6; matrix.py prints v1.3 beside the v1.2 it printed at 10:52 | — | `2026-10-04T14:20:02Z-c` |
| 2026-10-04-running-turn-refusal (implement/fix-loop) | gate.py printed `gate v1.8 · af374e71` on run and `gate v1.9 · fb1f9172` on show within the same session; gate-contract.md §Tool states v1.8 | — | `2026-10-04T22:18:54Z-e` |
| 2026-10-05-dialog-rows-and-re-probe (phase/take-up) | promotion.md §External inputs gives the relay snap as `--message-file` in place of `--source` with no `--origin`; inputs.py refused it (exit 2, '--message-file needs --origin') and the call was re-fired with --origin | retries 1 | `2026-10-05T11:18:10Z-b` |
| 2026-10-05-permission-end-to-end (phase/take-up) | promotion.md §External inputs gives the inputs.py snap form for a relay with --message-file but omits --origin; the tool refused exit 2 '--message-file needs --origin', re-fired with --origin | retries 1 | `2026-10-05T15:02:49Z-b` |
| (no chunk) (new-session/orientation) | session-state-contract's expected-transient list names the handoff, state.yaml and the two telemetry ledgers; the tree at session start also held a tracked wrap run-dir trail (evolve-{marker}.json, +40 lines) written by the wrap's post-commit evolve append, a member th… | extra_reads 2 | `2026-10-06T22:32:34Z-b` · git diff --stat at session start: 3 files, 45 insertions |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host (implement/code) | the implement letter names only inputs.py snap --source for a fact from outside the repository; an answer that arrives in the session is refused under --source from the run dir (in-repository) and from the scratchpad (temp dir), and lands only with --message-file --ori… | retries 2 | `2026-10-07T10:30:18Z-d` |

**Proposal:** Each site is a one-line reference edit: add `--origin` to both snap forms; settle whether a tool stamp may move during a session without its contract line; restate the U35 door's marker and lift target as `registry.py` enforces them; name the evolve trail in the expected-transient list or commit it with the wrap. The `--origin` omission cost a refused call on each of three take-ups in one day: a friction record does not reach the reference between diagnoses.

### P8 — implement/fix-loop · `contract.spec-reality-gap` — 7 cases · weight 21
**Rate:** 7 cases over 25 runs of the step

**Impact (summed):** iterations 1 · dialogue_rounds 2 · soft_exit 3 · deferred 1

**Pattern:** A live reading contradicted a spec sentence or a plan acceptance and was surfaced with nothing authored, as the letter asks, in 6 chunks; three cases ended the run as a soft exit. The gaps: a mutant not measurable on the host, an envelope code, the paste hint, the dropped trailing newline, the census expectation, the wheel's reply list on a real terminal, a trailing CR.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | the M3 acceptance asked the confirming run for missed==0 over the whole viola-e2e unit, but two mutants in the Windows-only prepare body (HOST_SCRATCH = cfg!(windows)) cannot be measured on Linux; the overseer ruled missed==0 over the measurable set, the pair owed to :… | dialogue_rounds 1 | `2026-10-04T00:32:34Z-d` · viola-0.1.0/chunks/2026-10-03-mutation-scoring-completion/evidence/m3.md |
| 2026-10-04-wait-and-last | channel_wait_last_invalid_params: last with from:7 answered -32600 (envelope Params.from: Option<String>) where security-plan Channel frames row and the plan acceptance say -32602; closed in the impl by removing the unread envelope field | iterations 1 | `2026-10-04T10:13:15Z-b` · gate entry 8 first run, tests/cli_wait_last.rs:527 |
| 2026-10-06-local-command-and-paste-framing-rows | after a long paste the 2.1.287 footer shows a paste hint in place of the compiled input-box literal for about 8 s from the paste; Run B's settle gives up 5 s after the Stop capture, so the guard that pastes only into a settled input box pasted neither the tag-like text… | soft_exit 1 | `2026-10-06T20:41:30Z-b` · viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/record-round-red.md |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | a send whose text ends in a newline is delivered and runs a turn yet is never claimed: the CLI drops a pasted text's last newline before the hook, and the claim compares exactly; surfaced under the plan's STOP 5, nothing authored | — | `2026-10-07T10:41:20Z-b` · viola-0.1.0/chunks/2026-10-07-live-rows-and-paste-shapes-on-the-dev-host/evidence/live-shape-red-green.md |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | Surfaced: the plan's census expectation (two whole profiles a run, steps 6 to 8, entry 16's atom) against the built fix, which leaves one. Pointer only; the operator revises the plan through phase. | soft_exit 1 | `2026-10-07T14:38:43Z-c` · viola-0.1.0/chunks/2026-10-07-a-send-ending-in-a-newline-is-confirmed/evidence/profraw-red-green.md |
| 2026-10-08-first-live-test-and-self-drive | the wheel's closed list of terminal replies (F-W2) against a real terminal: on foot 1.28.0 the replies CSI 0 n, CSI ? 997;1 n, CSI 4/6/8;..t, CSI 48;..t and CSI > 4;1 m are read as typing, and a live claude 2.1.287 took the wheel from the driver at its start; surfaced,… | soft_exit 1 · dialogue_rounds 1 | `2026-10-08T07:41:13Z-e` · viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/reply-probe.ndjson |
| 2026-10-08-first-live-test-and-self-drive | a live reading the plan left open falsified a compiled behaviour: a driver text ending in one CR is delivered and answered, filed as a human prompt, takes the wheel and is reported not delivered; surfaced in the report, nothing authored | deferred 1 | `2026-10-08T08:56:11Z-e` · viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/live-readings.ndjson |

**Proposal:** The records show the designed surface path operating: each gap named here was given an owner (a later chunk, a ruling, a CARRY). No pipeline change is evident from them. They are above threshold by count and listed whole for the founder's reading of what live measurement found.

### P9 — cross-step · `tooling.host-shell` — 10 cases · weight 20
**Steps:** implement/code 3 · implement/fix-loop 2 · wrap-session/reconcile 1 · phase/research 1 · new-session/orientation 1 · wrap-session/route-resolve 1 · phase/distill 1

**Impact (summed):** retries 10 · extra_reads 1

**Pattern:** The host shell or the Bash tool's embedded tools changed or refused a command, at seven different steps. The host `grep` is ugrep and refuses a bounded-repeat context pattern (3 cases here, 1 untyped, 7 problem-facts: L5); a backticked word inside double quotes or an unquoted heredoc ran as command substitution (2); an apostrophe ended a single-quoted JSON argument; the embedded `find` refuses a GNU-style `-newermt`; PowerShell 5 read a BOM-less file as ANSI; the guard refused a heredoc with a file target and a `cd`.

**Evidence:** ALL 10 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup (implement/code) | Windows PowerShell 5 read a BOM-less UTF-8 em dash in a scratch .ps1 as ANSI and failed to parse the script; replaced with ASCII | retries 1 | `2026-10-03T04:04:41Z-c` |
| 2026-10-04-running-turn-refusal (wrap-session/reconcile) | an UNQUOTED evolve heredoc (used to interpolate the ts) ran a backticked token inside a record value as command substitution: record 2026-10-04T22:36:36Z-c lost the id turn-running-cause (reads 'an 18-char id (limit 16)'); the record landed otherwise valid | retries 1 | `2026-10-04T22:36:42Z-a` |
| 2026-10-05-dialog-rows-and-re-probe (phase/research) | the host's grep is ugrep, which refused -o patterns with bounded repeats ('.{0,160}…' exceeds complexity limits) twice; re-done with a python scanner | retries 2 | `2026-10-05T11:31:17Z-b` |
| (no chunk) (new-session/orientation) | the Linux host's grep is ugrep: an -o -i -E pattern with two bounded any-char repeats around an alternation exited with an exceeds-complexity-limits error and printed no match; the fact was re-read from a small evidence file | retries 1 · extra_reads 1 | `2026-10-06T20:18:55Z-b` |
| 2026-10-06-local-command-send-outcomes (implement/code) | a grep pattern held a backticked word inside double quotes, so bash command-substituted it and ran the installed viola with no arguments; its help text landed in the probe's output and the pattern lost that alternative; nothing was written | — | `2026-10-06T23:23:25Z-d` |
| (no chunk) (wrap-session/route-resolve) | grep on this host is ugrep, and a bounded-context extraction with two counted wildcard runs around a literal was refused as exceeding complexity limits, printing nothing; the same read was redone by a scratchpad python extractor | retries 1 | `2026-10-07T06:01:45Z-d` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host (implement/code) | an apostrophe inside a single-quoted JSON argument ended the quote and the call failed with unexpected EOF; reworded without the apostrophe | retries 1 | `2026-10-07T10:30:18Z-f` |
| 2026-10-07-send-waits-out-the-paste-hint (implement/fix-loop) | the Bash tool's find is an embedded finder that refuses a GNU-style -newermt timestamp (date, space, time, UTC) as an invalid timestamp; the transcript count for the round's census was re-run with the ISO form | retries 1 | `2026-10-07T12:46:39Z-c` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (implement/fix-loop) | An append to an evidence file was written as a cat heredoc redirected to the file; the Bash guard refused it (a cat/tee heredoc with a file target) and the same text went in through the Edit tool on a short anchor. The host rule file states this refusal; it was not re-… | retries 1 | `2026-10-07T15:39:29Z-b` |
| 2026-10-08-first-live-test-and-self-drive (phase/distill) | a per-extract probe written as cd into the run dir then a loop was refused by the Bash guard, which denies a cd that moves the session cwd; the always-loaded host rule file states the rule and it was met again; one call re-run with absolute paths | retries 1 | `2026-10-08T05:12:25Z-b` |

**Proposal:** The ugrep and `find` facts are properties of the harness's shell, not of this project: the host rule template that setup renders is a home for them that every project would inherit. The two command-substitution cases are the documented reason the evolve recipe uses a quoted heredoc; one of them corrupted a friction record's text (the id of the record it describes).

### P10 — implement/fix-loop · `contract.test-expectation` — 8 cases · weight 20
**Rate:** 8 cases over 25 runs of the step

**Impact (summed):** iterations 12

**Pattern:** A red traced to a test's expectation, not to product code, in 7 chunks (12 iterations). Half are existing tests pinning behaviour the chunk changes and absent from research's lists and the plan's steps; half are the chunk's own new tests wrongly built on first write.

**Evidence:** ALL 8 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-confirmed-send-with-cl-1-records | tests/channel_endpoint.rs channel_debug_level_keeps_request_content_out_of_the_role_files asserted send -> -32601, the behaviour this chunk removes; plan.md said to expect no change in that file. Rewritten to a confirmed send at debug, with the canary scan kept and the… | iterations 1 | `2026-10-04T06:34:51Z-b` |
| 2026-10-04-wait-and-last | last_survives_a_wrapper_restart rebooted with the gated path3 script while the control file still held two releases, so the restarted fake agent replayed the turn and last changed legitimately; the reboot now runs without the script | iterations 1 | `2026-10-04T10:13:15Z-c` · gate entry 8 first run, tests/cli_wait_last.rs:382 |
| 2026-10-04-dialog-answers-by-dialog-id | two existing tests pinned data this chunk changed: contract_windows_mutation_scope (a new Windows-gated source unlisted in windows-mutants.yml) and channel_endpoint (answer used as the unknown-method probe, now served, -32602 not -32601) | iterations 1 | `2026-10-04T15:36:14Z-b` |
| 2026-10-04-the-wheel | tui_focus_mouse_and_resize_never_take_the_wheel used a driver send as its wheel probe, but the reports sit in the fake agent's prompt line so the paste is never read back and the CLI outlived the test bound; probe changed to an answer to no pending dialog | iterations 1 | `2026-10-04T18:40:24Z-b` |
| 2026-10-05-real-cli-verify-probes | two cli_verify cases asserted the wrong thing for the new drive (a Stop-less set with a trusted turn; Run A's temp root inside the fake's trusted root) and were rewired before the live round | iterations 2 | `2026-10-05T09:45:14Z-c` |
| 2026-10-05-real-cli-verify-probes | two more test-construction reds against the recorded fixtures: a Stop-less stamp in cli_version_gate (trusted turn waits PROBE_DEADLINE) and a named 20 s test deadline past the mutants kill and floor (contract_lints); fixed by the trust-root move and the send_window_ n… | iterations 1 | `2026-10-05T10:27:54Z-b` |
| 2026-10-05-dialog-rows-and-re-probe | four reds traced to this chunk's own new tests, none to product code: a hook test whose earlier call consumed ordinal 1, a record test's expected name order, a cwd assertion over dialog payloads that carry none, and three screen names dropped in the fix | iterations 4 | `2026-10-05T13:20:49Z-b` |
| 2026-10-07-send-waits-out-the-paste-hint | one unit case pinned to the old 5 s bound (a settle read at 6000 ms) reddened the unit entry, the default selection and pre-push in the first full block run; research's sweep and the plan's step 6 did not list it; its instant moved to 9000 ms and the block read green o… | iterations 1 | `2026-10-07T12:46:39Z-b` |

**Proposal:** For the existing-test half see the pin-sweep direction under `input.plan-step-ambiguous`. The own-test half was closed inside the fix loop, which is its job.

### P11 — wrap-session/reconcile · `contract.false-positive-proposal` — 7 cases · weight 20
**Rate:** 7 cases over 19 runs of the step

**Impact (summed):** iterations 1 · reformulations 12

**Pattern:** A detector proposal was wrong or wider than the report supports and was rejected or narrowed at apply, in 7 chunks (12 reformulations). Three of the seven are the source-coordinate shape `contract.proposal-format` also counts; the others are a condition carried over from another verb, a registry over-reach, a ban wider than the masters' own registries, a clause the code keeps.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-windows-boundary-mutation-workflow | test-plan T1's change line said only TIMEOUT/SIGKILL kills still leave dirs; leak.md also has 21 fixture git repos — narrowed by re-derivation at apply | — | `2026-10-04T04:20:25Z-b` |
| 2026-10-04-wait-and-last | the architecture detector's diagnostics/ proposal (A3) said send/wait/last write cli-<name>.ndjson only when VIOLA_NAME holds a valid name, carrying verify's condition over; the report says they init obs with the target <name> argument; applied re-derived from the repo… | reformulations 1 | `2026-10-04T10:41:11Z-b` · .andromeda/runs/2026-10-04T10-29-04-wrap/fanout-results.md |
| 2026-10-04-dialog-answers-by-dialog-id | D-arch-decisions proposed insta for arch Stack, which carries no test-library row; rejected under Registry over-reach | iterations 1 | `2026-10-04T17:16:27Z-b` · .andromeda/runs/2026-10-04T16-53-44-wrap/fanout-results.md |
| 2026-10-04-the-wheel | four proposals carried facts the report does not: T2's basis and still-to-join list read from the test source (rejected, re-raised from the report), T1's two cli_wheel/tui_wheel case names, A10's feature name, A8's wait timeout_ms recovery clause (all narrowed) | reformulations 4 | `2026-10-04T20:59:31Z-b` · .andromeda/runs/2026-10-04T20-44-01-wrap/fanout-results.md |
| 2026-10-05-dialog-rows-and-re-probe | 4 proposals rejected by the re-derivation tell: layout L1 (+2 dependents) cited ledger.rs lines and arch A5 cited verify.rs lines the report does not carry; their facts were re-raised by the orchestrator from the report | reformulations 2 | `2026-10-05T14:46:12Z-b` · .andromeda/runs/2026-10-05T14-27-34-wrap/fanout-results.md |
| 2026-10-07-test-homes-off-the-contended-volume | Two proposals were narrowed at apply. D-security-input's change line opened with a ban on any test-side creation or removal outside the working directory except through the link, which the masters' own registries contradict (the chaos home, the watch reports, verify's… | reformulations 2 | `2026-10-07T08:34:21Z-b` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | three returns leaned on source coordinates the report does not carry (two test-plan bases, one obs-plan adjacent remark) and one architecture change line retired a stay-clause the code keeps; the two test-plan proposals were rejected as proposed and raised by the orche… | reformulations 3 | `2026-10-07T11:10:34Z-c` |

**Proposal:** See `contract.proposal-format`. The re-derivation at apply caught every case; the remaining direction is the one stated there.

### P12 — cross-step · `contract.narrow-basis-claim` — 9 cases · weight 17
**Steps:** phase/research 3 · wrap-session/report 2 · implement/fix-loop 2 · phase/distill 1 · implement/code 1

**Impact (summed):** retries 3 · reformulations 1 · dialogue_rounds 2 · extra_reads 15

**Pattern:** A count, an absence or a location was written from a basis narrower than the claim and corrected on re-derivation, in 8 chunks. Counts set down from recall of output read earlier (3, all in research.md); a location named from a grep hit with no heading read (2, both report drafts); a claim made to the operator from reasoning, on which the first answer rested; a lock state read from two probes that cannot see it, which nine reads and four probe runs then stood on; a record that left out a reading its own log held; a history agent reading a sidecar alone.

**Evidence:** ALL 9 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-readiness-gate-and-timing-constants (wrap-session/report) | the first report draft named 'test-plan §10 Mutation-gate prose' as the body site of the half-removed-repo class without a grep; the re-derivation pass found 0 hits in test-plan.md and 1 in registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.… | reformulations 1 | `2026-10-04T05:27:22Z-b` |
| 2026-10-05-permission-end-to-end (phase/distill) | tests history agent wrote 'No planted-hang control was recorded with the override' from the sidecar alone; the prior chunk's evidence/verify-window-class.md records one for the 20 s binary override (TIMEOUT 20.003s), none for the 45 s verify_window_ override | extra_reads 1 | `2026-10-05T15:08:44Z-b` |
| 2026-10-06-local-command-and-paste-framing-rows (implement/fix-loop) | step 0's shape record states four settled screens each showing the input box, and the plan was revised on that record; step 0's own drive log holds the input box settling 5.8 s after the long-paste turn against 1.3 s after the others, a reading the record left out and… | extra_reads 3 | `2026-10-06T20:41:30Z-c` · viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/record-round-red.md |
| 2026-10-06-local-command-send-outcomes (phase/research) | a call-site count for SendSlot::new was written into research.md from the graph's 23 rows for name new with callee_file send.rs, which also hold the test helper Pastes::new defined in the same file; a grep of the qualified call gave 15 in-file sites and the two counts… | retries 1 · extra_reads 1 | `2026-10-06T22:51:16Z-b` · .andromeda/runs/2026-10-06T22-33-26-phase/tree-query-2026-10-06-local-command-send-outcomes.json |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host (phase/research) | research.md was first written with three counts not yet derived (the labelled-case count with an absence claim, and the two rule files' addition counts); the re-derive before the closure read 44, 8 and 31, corrected two of the three and reworded the absence to what the… | retries 1 | `2026-10-07T09:07:21Z-c` |
| 2026-10-07-send-waits-out-the-paste-hint (phase/research) | four sweep counts and two caller counts in research.md's first write were set down from recall of grep output read earlier in the window; a recount against that output moved five of them (changed sites 4 to 2, hits 6 to 7, changed 4 to 3, hits 9 to 10, cases nine to ei… | retries 1 | `2026-10-07T12:08:01Z-b` · .andromeda/runs/2026-10-07T11-51-17-phase/ |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (implement/fix-loop) | The implementer told the operator on the step 7 card that the census gate would read as planned with a host child, from reasoning about the wrapper's child alone. The version probe runs the same program, so its profile goes too: the control read one profile a run, not… | dialogue_rounds 1 | `2026-10-07T14:38:43Z-b` · viola-0.1.0/chunks/2026-10-07-a-send-ending-in-a-newline-is-confirmed/evidence/profraw-red-green.md |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (wrap-session/report) | The report's first draft named test-plan.md:233 as section 6 Scenario Path 2 from a grep hit on the path's wording; a heading read placed line 233 in section 1 and the scenario at 733 to 754. Corrected before the fan-out. | extra_reads 1 | `2026-10-07T19:17:53Z-c` |
| 2026-10-08-first-live-test-and-self-drive (implement/code) | The session was read as not locked from two probes that cannot see a shell-held lock (no hyprlock process, logind LockedHint=no) in research M1, inputs#I6 and inputs#I10; four phase probe runs, a DPMS test and the overseer focus measurements followed on that basis. hyp… | extra_reads 9 · dialogue_rounds 1 | `2026-10-08T06:36:48Z-b` · viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/key-probe.md |

**Proposal:** The letter's re-derive pass caught the research and report cases before they left the step. A direction for the form: a count in research.md or a report written beside the command that produced it, so the re-derive is a re-run. The lock-state case was the costliest and is now a host rule.

### P13 — implement/code · `tooling.hook-friction` — 8 cases · weight 17
**Rate:** 8 cases over 28 runs of the step

**Impact (summed):** retries 9

**Pattern:** A project hook refused a call at implement, in 7 chunks. Six of the eight are the Bash guard on a heredoc with a file target; the other two records hold the `cd` guard (twice in one record) and the stamp-ahead hook returning a blocking error for a planned future time its own text says may stand (once in each).

**Evidence:** ALL 8 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-confirmed-send-with-cl-1-records | the PreToolUse Bash guard refused a cat >> heredoc and a cat > heredoc to a scratchpad script; both edits were re-made through the Edit tool | retries 2 | `2026-10-04T06:27:45Z-b` |
| 2026-10-04-the-wheel | the PreToolUse Bash guard refused a cat heredoc redirected to a scratchpad script file; the script was re-written through the Write tool and run by path (host-win32.md 2026-09-29 recurrence) | retries 1 | `2026-10-04T18:27:23Z-b` |
| 2026-10-05-real-cli-verify-probes | a compound Bash call carrying a heredoc to a file was refused whole by the PreToolUse guard; re-split into two calls | retries 1 | `2026-10-05T09:39:17Z-c` |
| 2026-10-06-local-command-send-outcomes | the PreToolUse Bash guard blocked a cat heredoc with a file target on this Linux host (the rule file names it for the Windows host); the call was re-made through the Write tool | retries 1 | `2026-10-06T23:23:25Z-c` |
| 2026-10-07-test-homes-off-the-contended-volume | The Bash guard refused a scratchpad script written through a cat heredoc with a file target (the rule host-win32.md already carries, on a Linux host); rewritten through the Write tool and run by path. | retries 1 | `2026-10-07T07:48:25Z-d` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | the Bash guard refused a compound that wrote a scratch script through a cat heredoc with a file target; the file went through the Write tool | retries 1 | `2026-10-07T10:30:18Z-e` |
| 2026-10-08-first-live-test-and-self-drive | two Bash calls opening with a cd into a subdirectory were refused by the cd guard and redone through the Read tool; one PostToolUse stamp-ahead hook flagged a planned future time (the 3 hour bound) in an evidence document, which stood as written | retries 2 | `2026-10-08T07:41:13Z-c` |
| 2026-10-08-first-live-test-and-self-drive | the stamp-ahead Write hook returned a blocking error for a planned future time (the 10:32Z bound of the plan) written as prose into a record tail; its own text says a planned time stands, and nothing was changed | — | `2026-10-08T08:56:11Z-c` |

**Proposal:** For the heredoc refusals see `recall.corpus-recurrence` and L1. For the stamp hook: a planned bound written as prose is not the defect it exists for; a non-blocking return for a time the text marks as planned would remove the two cases.

### P14 — wrap-session/gates · `tooling.result-not-run-stable` — 3 cases · weight 17
**Rate:** 3 cases over 21 runs of the step

**Impact (summed):** retries 3 · dialogue_rounds 1 · halted 3

**Pattern:** The wrap's light gate graded red a tree that had read green at implement, three times, each a halt. Causes as measured: the Windows volume's latency; a test that counted a receipt before the fake agent wrote it (a real defect); another project's cargo writes on the shared volume.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | on an identical-code tree the frozen-stale case and the viola-e2e cli entry went green at /implement and red at the wrap light gate on D: (stamped-home verify never exited; boot readiness-timeout); run --coverage took 5054 s vs 2196 s, and pre-push timed out at its 540… | halted 1 | `2026-10-03T11:45:42Z-b` · .andromeda/runs/2026-10-03T07-46-03-wrap/gate-2026-10-02-epoch-2b-cleanup.json |
| 2026-10-04-running-turn-refusal | the full-suite entry `agent-run.sh run` went red at the light gate on an identical tree that read green at implement (twice) and in 5 pre-push coverage runs: the new cli_send driver-turn test counts receipt prompts right after the first send confirms, but the fake agen… | halted 1 | `2026-10-04T22:41:04Z-b` |
| 2026-10-06-local-command-and-paste-framing-rows | the light gate graded an identical tree red on three different entries in three consecutive whole-block runs (pre-push on three truncated raw coverage profiles of 4709 with every test passing; the default run on 39 stalled starts; one named case on a boot not ready) an… | retries 3 · dialogue_rounds 1 · halted 1 | `2026-10-06T22:24:12Z-b` · .andromeda/runs/2026-10-06T21-43-53-wrap/light-gate-red.md |

**Proposal:** Read with the fix-loop group below. Absorbed mid-epoch: the test homes moved off the contended volume on 2026-10-07 and no record of this type follows 2026-10-06. One direction remains from two records that say no artifact had frozen a red: the gate tool files one log per entry number per run dir, so a targeted re-run overwrites the red log; keeping the first red beside the re-run would leave the cause hunt something to read.

### P15 — phase/plan · `retry.synthesis-rework` — 7 cases · weight 16
**Rate:** 7 cases over 27 runs of the step

**Impact (summed):** iterations 8 · retries 1

**Pattern:** Point corrections inside synthesis after the plan's first write, in 7 chunks (8 iterations). Two carried a stale fact adapted from the previous chunk's plan (a path list with two crate dirs that do not exist; a source coordinate that had moved); one is an apostrophe escaped as a doubled quote in a TOML literal string; one a stamp ahead of the clock; one a ledger note for an unverified capability; two are collateral of a revision edit.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-windows-boundary-mutation-workflow | step 8 first named ci.py conclusion --wait as the dispatch waiter; ci.py stops at the first failed check, so a package with survivors would end the wait while the other jobs ran; rewritten to gh run watch then one ci.py row | iterations 1 | `2026-10-04T01:57:32Z-c` |
| 2026-10-05-real-cli-verify-probes | the gate fence failed to parse: two TOML literal-string notes escaped an apostrophe as '' (not valid in a literal string); rewritten as basic strings, dry-run then parsed 24 entries | iterations 1 | `2026-10-05T06:18:34Z-d` |
| 2026-10-06-local-command-and-paste-framing-rows | two rewrites inside synthesis: a Revised stamp written ten minutes ahead of the clock was refused by the stamp hook and re-read from the clock; an added acceptance line named a git guard and a byte-identity claim no listed entry proved, so it was reworded and two entri… | iterations 2 · retries 1 | `2026-10-06T20:57:18Z-b` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | The preservation guard's path list was adapted from the previous chunk's guard entry and carried two crate dirs that do not exist in the tree (viola-mcp, viola-ui); an existence loop before the dry-run named them. The dry-run then read the jq variable of the first read… | iterations 1 | `2026-10-07T09:51:37Z-b` |
| 2026-10-07-send-waits-out-the-paste-hint | three point corrections after the plan's first write: a probe note's source coordinate copied from the previous plan's entry named a line the function no longer stands at (ledger.rs 316, now 388), a step stated a count of wait steps with no derivation and was reworded,… | iterations 1 | `2026-10-07T12:15:34Z-b` · .andromeda/runs/2026-10-07T11-51-17-phase/planlint-2026-10-07-send-waits-out-the-paste-hint.json |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | The first write of the revised plan moved one acceptance line the directive did not name (the pre-CI commit's CI run became the final commit's) and left a gate note describing the falsified picture; the line was put back and the note reworded before validation. | iterations 1 | `2026-10-07T14:46:13Z-b` |
| 2026-10-08-first-live-test-and-self-drive | three passages written in the first edit pass were rewritten in the same pass: the retried session's instance (a reused name, then a fresh one), a step heading that read as done, and a touchpoint line left holding a file from the replaced list | iterations 1 | `2026-10-08T08:28:44Z-b` |

**Proposal:** Each was caught before validation. Two shapes recur outside this group and may be worth a template line: the TOML literal string (L16) and text adapted from the previous chunk's plan, whose paths and coordinates need an existence read before the dry-run.

### P16 — implement/fix-loop · `tooling.result-not-run-stable` — 4 cases · weight 16
**Rate:** 4 cases over 25 runs of the step

**Impact (summed):** iterations 5 · retries 5 · dialogue_rounds 1 · extra_reads 15

**Pattern:** An identical tree read red then green inside the fix loop, in 4 chunks (15 extra reads). Causes as measured: host load on the Windows volume; a truncated coverage profile from the fake agent's exit hanging up a hook (a real cause, fixed); CI runner slow tails read from five runs' JUnit; another session's cargo link writes on the same volume.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | root M1 direct-file baseline red once (run_is_silent_on_stdout_and_stderr at the 10 s kill under 371 parallel tests) then green on retake; the HEAD copy showed one load stall of its own in 3 full rounds | retries 1 | `2026-10-03T06:03:49Z-c` · viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/m1.md |
| 2026-10-04-running-turn-refusal | pre-push coverage merge refused a truncated viola-hook .profraw (the WATCH subject, 0/3 since the wheel chunk) on a tree whose 1472 tests passed; cause measured from the raw profile's counters and JUnit timing | iterations 1 · dialogue_rounds 1 · extra_reads 6 | `2026-10-04T22:18:54Z-b` · viola-0.1.0/chunks/2026-10-04-running-turn-refusal/evidence/watch-profraw.md |
| 2026-10-05-dialog-rows-and-re-probe | the ubuntu coverage leg graded identical code green (round 1, 4179973) and red (round 3, 74e719b) on the 7 s test bound; the reds coincided with runner-wide slow tails (untouched tests' p90 ratio 2.46 and 3.09 vs 1.08 and 1.31), read from five runs' JUnit | iterations 4 · retries 1 | `2026-10-05T14:26:27Z-a` · viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/ci-rounds.md |
| 2026-10-06-local-command-and-paste-framing-rows | gate entries 16 and 18 graded an identical tree red in the first full block (22 timeouts and 2 boot failures; 1 boot failure) and green in three later runs; the delta is a host stall of process starts, timed against another session's cargo link writes on the same volum… | retries 3 · extra_reads 9 | `2026-10-06T21:30:37Z-b` · .andromeda/runs/2026-10-06T21-01-22-implement/gate-2026-10-06-local-command-and-paste-framing-rows.json |

**Proposal:** See the gates group above; the same absorption and the same remaining direction apply.

### P17 — wrap-session/curation · `ambiguity.filter-borderline` — 10 cases · weight 12
**Rate:** 10 cases over 19 runs of the step

**Impact (summed):** iterations 1 · reformulations 1 · deferred 6

**Pattern:** Curation candidates scored exactly 0.6 on filter 4, or tied at 0.8 for the cap's last slot, in 10 of 19 chunks; the conditional no-other-home signal or a judgment decided them, and 6 candidates were deferred by the cap of three. One record states the open point: whether a fact in a chunk evidence file counts as a durable home is not stated by filter 4.

**Evidence:** ALL 10 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | four candidates tied at 0.8 for the three Filter-5 slots; the PowerShell-5 encoding candidate was deferred by judgment, no rule orders a tie | deferred 1 | `2026-10-03T07:56:53Z-b` |
| 2026-10-03-mutation-scoring-completion | three candidates tied at 0.8 (the workflow-scope push, the PID 1 deadline target, the PTY master-close hangup) for the cap's last slot; the tiebreak was a judgment (the one touching every operator pass), the other two deferred | deferred 2 | `2026-10-04T01:22:46Z-b` |
| 2026-10-04-dialog-answers-by-dialog-id | the Windows DACL test-home candidate scored exactly 0.6 and rejected because P2 had already amended the fact into test-plan, which also barred the no-other-home signal | iterations 1 | `2026-10-04T17:17:58Z-c` |
| 2026-10-05-real-cli-verify-probes | the send_window_/verify_window_ no-test-deadline precedent scored exactly 0.6 and rejected because this wrap amended its fact into the test-plan nextest contract | — | `2026-10-05T11:01:20Z-c` |
| 2026-10-05-dialog-rows-and-re-probe | the 300 ms settle candidate scored exactly 0.6 (an exact-0.6 reject) while the same fact landed in the architecture master this wrap | — | `2026-10-05T14:47:29Z-c` · .andromeda/runs/2026-10-05T14-27-34-wrap/curation.md |
| 2026-10-06-local-command-send-outcomes | three candidates scored exactly 0.6 and each took the no-other-home signal; for the one built on a single operator directive the could-be-task-specific negative was weighed and not applied, which is what kept it at 0.6 and let the conditional signal fire | reformulations 1 | `2026-10-07T00:05:14Z-b` |
| 2026-10-07-test-homes-off-the-contended-volume | Three candidates sat exactly at Filter 4's 0.6 (measured + specific detail): the process-name hazard took the next-entry signal, the direct-call remove-the-guard facet took the no-other-home signal, and the trailing-slash find hazard took neither because this wrap amen… | — | `2026-10-07T08:35:54Z-b` |
| 2026-10-07-send-waits-out-the-paste-hint | four candidates each scored exactly 0.6 on Filter 4 (a real failure or a design change plus a specific detail) and passed only on the conditional no-other-home signal; three were applied and the fourth deferred by the cap of three | deferred 1 | `2026-10-07T13:14:51Z-b` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | Both applied candidates scored exactly 0.6 on a measurement and a detail and passed only on the no-other-home signal; whether a fact in a chunk evidence file counts as a durable home is not stated by filter 4. | — | `2026-10-07T19:32:55Z-b` |
| 2026-10-08-first-live-test-and-self-drive | five candidates scored exactly 0.6 on the two measurement signals, so the conditional no-other-home signal decided four of them and the fifth was rejected for having a home in the plan; with four at 0.8 the cap of three was cut by judgment, not by score | deferred 2 | `2026-10-08T09:31:57Z-b` · .andromeda/runs/2026-10-08T09-10-03-wrap/curation.md |

**Proposal:** Two rule lines would remove most of these: an order for ties at the cap, and a definition of a home for the conditional signal (three cases were rejected because the same wrap had amended the fact into a master, which reads as a rule already applied but not written).

### P18 — wrap-session/reconcile · `contract.cascade-miss` — 9 cases · weight 12
**Rate:** 9 cases over 19 runs of the step

**Impact (summed):** iterations 3 · extra_reads 12

**Pattern:** A stale restatement no detector proposed was found by the cascade sweep, by the orchestrator's own read or grep, or by both, in 7 chunks. Four of the missed sites are plan key files (`5-command-implementation.md` twice, `ci-cd-approach.md`, `test-data-bootstrap.md`; one record gives the path under `registries/contracts/`); one leaf line stood more than 80 characters from the words its proximity pattern keyed on; one leaf restated retired wording with no swept token.

**Evidence:** ALL 9 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | two stale statements of the removal mechanism no detector proposed: obs-plan.md:1044 named an rstest TempDir drop as the deleter (the sweep's tempdir-drop pattern caught it), and test-data-bootstrap.md:15 stated the keep path as TempDir::keep() calls (caught on the orc… | iterations 2 | `2026-10-03T07:54:56Z-b` · .andromeda/runs/2026-10-03T07-46-03-wrap/cascade-dispositions.md |
| 2026-10-03-mutation-scoring-completion | two stale restatements no detector proposed: ci-cd-approach.md:5 ("the WSL provisioning carry no perf step", on a 3320-char line) and 5-command-implementation.md:56 (outcomes read only on counted and scoped verdicts); both found by the cascade sweep rows, amended in-pa… | extra_reads 2 | `2026-10-04T01:20:42Z-c` · .andromeda/runs/2026-10-04T01-02-04-wrap/cascade-dispositions.md |
| 2026-10-03-mutation-scoring-completion | obs-summary.md:51 restated obs-plan §8 item 6's retired clone wording (Windows repo/home path, Linux clone) without any swept token; found by reading the obs leaf, not by the sweep | extra_reads 1 | `2026-10-04T01:20:42Z-d` · .andromeda/runs/2026-10-04T01-02-04-wrap/cascade-dispositions.md |
| 2026-10-04-confirmed-send-with-cl-1-records | registries/contracts/test-plan/5-command-implementation.md:52 restated the per-target fuzz seed list without paste_text; no detector proposed it; the cascade sweep's fuzz-four row caught it and the pass folded it | iterations 1 | `2026-10-04T08:55:41Z-c` |
| 2026-10-04-dialog-answers-by-dialog-id | two pre-pass restatements no detector proposed: obs-plan:737 (CI verify at 2.1.283) found by the orchestrator's expected-amendment grep, test-plan:408 (contract suite vs verify fixtures only) found by the sweep | extra_reads 2 | `2026-10-04T17:16:27Z-c` · .andromeda/runs/2026-10-04T16-53-44-wrap/cascade-dispositions.md |
| 2026-10-05-real-cli-verify-probes | the retired fixed --record refusal text stood at architecture:139, design-system:815 and security-plan:278; no detector proposed them (two docs returned proposals: []), the orchestrator's grep and the cascade sweep caught them | extra_reads 3 | `2026-10-05T11:00:00Z-b` · .andromeda/runs/2026-10-05T10-37-44-wrap/cascade-dispositions.md |
| 2026-10-05-real-cli-verify-probes | seven test-plan sites still named `:84` as the owner of local-command / dialog / permission work after the split; the test-plan detector proposed none, the route84 sweep pattern caught all seven | extra_reads 1 | `2026-10-05T11:00:00Z-c` · .andromeda/runs/2026-10-05T10-37-44-wrap/sweep-1.txt |
| 2026-10-05-dialog-rows-and-re-probe | two restatements no detector proposed: arch:75 (the S7 approve form, disproved claim 1) and test-plan:757 (the permission wake owed to this entry); caught by the orchestrator's site scan and folded | extra_reads 2 | `2026-10-05T14:46:12Z-c` · .andromeda/runs/2026-10-05T14-27-34-wrap/cascade-dispositions.md |
| 2026-10-07-send-waits-out-the-paste-hint | one stale leaf line (gotchas.md, a partial gate refusing a screen not quiet within 5 s) drew no row from the 17-pattern sweep: the value stood more than 80 chars from the words the proximity pattern keyed on; a separate regex read of every leaf body found it and it was… | extra_reads 1 | `2026-10-07T13:13:11Z-c` · .andromeda/runs/2026-10-07T12-57-41-wrap/cascade-dispositions.md |

**Proposal:** The sweep is doing its job as the backstop. Directions: whether the detectors read the plans' key files at all (four misses sit there); a leaf read by section for the restatements a wording-keyed sweep cannot reach.

### P19 — wrap-session/route-resolve · `ambiguity.trajectory-halt` — 3 cases · weight 12
**Rate:** 3 cases over 22 runs of the step

**Impact (summed):** dialogue_rounds 3 · halted 1

**Pattern:** Route-resolve halted for a trajectory answer three times; each was one halt, answered in one round.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | relay items A-D asked for disposition, not pre-placed; one halt with four questions (C2 placement, C3+D placement, :80 live host, the auto set) answered in one round by the overseer | dialogue_rounds 1 · halted 1 | `2026-10-03T22:17:30Z-b` · .andromeda/runs/2026-10-03T22-10-25-wrap/adaptation-record.md |
| 2026-10-04-the-wheel | the running-turn state had no route owner (plan-surfaced); one recommended-first card, the founder ruled live through the operator: mint an entry ahead of First live test | dialogue_rounds 1 | `2026-10-04T21:11:43Z-b` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | four findings needed a trajectory answer; one halt, four cards, recommended-first, answered in one round (two by the overseer, two as the founder's live rulings) | dialogue_rounds 1 | `2026-10-07T11:39:56Z-b` |

**Proposal:** This is the designed dialogue. No change is evident from the records; they are above threshold by count only.

### P20 — cross-step · `contract.structural-blind-spot` — 3 cases · weight 10
**Steps:** wrap-session/route-resolve 1 · phase/take-up 1 · wrap-session/reconcile 1

**Impact (summed):** dialogue_rounds 2 · extra_reads 7 · halted 1

**Pattern:** A documented mechanism could not reach its subject by construction, three times: 38 citations of working-route line numbers stood behind their entries because the cascade sweep keys on amended wording and an insertion moves no wording; `ci.py conclusion` printed green for a sha whose first attempt failed; the first citation sweep classed a planted literal of a probe script as a moved citation.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) (wrap-session/route-resolve) | 38 live citations of working-route line numbers in masters, registries, rules, docs and CLAUDE.md were 4 lines behind their entries at HEAD: entries inserted into the route after the last renumber shifted every later line, and the cascade sweep keys on amended wording,… | dialogue_rounds 1 · extra_reads 2 | `2026-10-07T06:01:45Z-c` · .andromeda/runs/2026-10-07T05-47-07-wrap/renumber-manifest.md |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed (phase/take-up) | ci.py conclusion printed green 15/15 for a sha whose run failed on attempt 1 and passed on a re-run of the same commit; a red attempt behind a green re-run cannot reach Setup 5a's disposition by any correct execution of the call | extra_reads 2 | `2026-10-07T13:39:16Z-b` · .andromeda/runs/2026-10-07T13-36-31-phase/ |
| 2026-10-08-first-live-test-and-self-drive (wrap-session/reconcile) | the first citation sweep classed a planted literal of a probe script (a real path and line written in a master as the probe's synthetic case) as a moved citation: written, and listed in no row; it was seen only by reading the map's trail, and the operator ruled a rewor… | dialogue_rounds 1 · halted 1 · extra_reads 3 | `2026-10-08T09:30:39Z-b` · .andromeda/runs/2026-10-08T09-10-03-wrap/citation-dispositions.md |

**Proposal:** Directions: `ci.py conclusion` reading a run's attempts and naming a red attempt behind a green re-run (the project's own rule keeps such a red open); an exemption marker the citation sweep honours for a literal written as a synthetic case. The route-coordinate case was met by the citation sweep introduced at the epoch's last wrap.

### P21 — phase/plan · `input.research-thin` — 5 cases · weight 9
**Rate:** 5 cases over 27 runs of the step

**Impact (summed):** iterations 2 · dialogue_rounds 1 · extra_reads 11

**Pattern:** Synthesis needed a file or a fact research had not read, in 5 chunks (11 extra reads): a nextest profile's kill line, a test support file, an enum's variants, the CLI's `--help`, the installed skills.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-windows-boundary-mutation-workflow | research's Windows-gate inventory took every gate occurrence, including gates only inside #[cfg(test)] modules (run/mutants.rs, viola-channel lib.rs, viola-state snapshot.rs); P4 refined the rule and recounted 508 mutants | iterations 1 · extra_reads 2 | `2026-10-04T01:57:32Z-b` |
| 2026-10-04-confirmed-send-with-cl-1-records | synthesis found two files the steps need outside research's lists: .config/nextest.toml (the mutants profile kills a root test at 10 s, equal to CONFIRM_WINDOW_FALLBACK) and tests/support/home.rs (a sized boot and the F4 out-of-scan home); research.md lists amended and… | iterations 1 · extra_reads 2 | `2026-10-04T06:05:10Z-b` · viola-0.1.0/chunks/2026-10-04-confirmed-send-with-cl-1-records/research.md |
| 2026-10-04-wait-and-last | synthesis needed two facts research had not read: EventKind has no dialog variants (crates/viola-core/src/lib.rs:44-55) and the cross-OS wrapper-kill precedent (tests/run_cli.rs:606, tests/cli_instance_state.rs:208); research.md amended at P4 with two touchpoints | extra_reads 3 | `2026-10-04T09:50:37Z-b` · viola-0.1.0/chunks/2026-10-04-wait-and-last/research.md |
| 2026-10-05-real-cli-verify-probes | P3 research did not read how contract_ledger_probes walks recorded sets nor the installed claude versions; at P4 the grown ledger proved unstampable on the 2.1.283/2.1.287 sets, costing a fork round and 3 reads | dialogue_rounds 1 · extra_reads 3 | `2026-10-05T06:18:34Z-c` |
| 2026-10-08-first-live-test-and-self-drive | the plan needed a second read-only pipeline skill for the clears-between-skills step; research had not listed the installed skills, and the documentation skill the project CLAUDE.md names is not installed on this host, found by one directory listing at synthesis; the s… | extra_reads 1 | `2026-10-08T06:14:27Z-b` |

**Proposal:** The nextest mutants profile's 10 s kill appears four times across types in this epoch (here, `input.plan-step-ambiguous`, `contract.test-expectation`, one problem-fact): a project gotcha a rule entry would carry. For the general shape see chain X8.

### P22 — implement/fix-loop · `tooling.environmental` — 4 cases · weight 9
**Rate:** 4 cases over 25 runs of the step

**Impact (summed):** retries 3 · dialogue_rounds 1

**Pattern:** The host filesystem broke a mutation run's scratch copy, in the three mutation chunks of 2026-10-02 to 2026-10-04: per-operation latency on the Windows volume, a tmpfs quota, a btrfs reflink dropping the exec bit, a socket path past `sun_path`.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | D: per-op filesystem latency (75-500 ms) keeps the in-repo suite, coverage and pre-push windows-tests red; full in-repo unit run took 1404 s | — | `2026-10-03T06:03:49Z-d` · viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/m2-diagnosis.md |
| 2026-10-03-mutation-scoring-completion | the M3 witness copied a 14.3 GB target/ into /tmp (32 GB tmpfs, usrquota) and died at 460/709 with Disk quota exceeded; 25 of its 77 unviable came from the cap | retries 1 · dialogue_rounds 1 | `2026-10-04T00:32:34Z-b` · viola-0.1.0/chunks/2026-10-03-mutation-scoring-completion/evidence/m3.md |
| 2026-10-03-mutation-scoring-completion | on btrfs cargo-mutants 27.1.0 reflinks the copy via reflink 0.1.3, which creates the clone 0644, so the prebuilt viola-fake-agent lost its exec bit and the baseline read 32 EACCES failures; a NOCOW scratch forces the fs::copy fallback | retries 1 | `2026-10-04T00:32:34Z-c` · viola-0.1.0/chunks/2026-10-03-mutation-scoring-completion/evidence/m3.md |
| 2026-10-04-windows-boundary-mutation-workflow | the plan's 'new subdir under viola-mutants-scratch' for the leak witness has an implicit length ceiling: a 24-char subdir pushed a viola-e2e unit test's Unix socket path past sun_path (107 B), so cargo-mutants' unmutated baseline failed (mutants-exit-4) after 26 s | retries 1 | `2026-10-04T02:10:29Z-b` · viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/leak.md |

**Proposal:** Confined to those chunks and curated into the host rule since. No pipeline change is evident beyond U4 (the type exists in fix-loop's list only).

### P23 — wrap-session/reconcile · `input.report-insufficient` — 7 cases · weight 8
**Rate:** 7 cases over 19 runs of the step

**Impact (summed):** reformulations 1 · extra_reads 9

**Pattern:** The report lacked a count, a name or a site the detectors needed, in 7 chunks; a detector then inferred it from source or the orchestrator re-derived it. Most are a count or a name the Counts family did not carry; two are a site list keyed on wording that missed a restatement; one is a disproved claim listed without the route entry that owns building it, which drew seven proposals to drop it.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | the report said sync was removed but did not list the closed pre-push reasons and details retired with it; a detector inferred them from code-shaped reasoning, the orchestrator confirmed them by grep and added the list to the report's Changes | extra_reads 1 | `2026-10-04T01:20:42Z-b` |
| 2026-10-04-dialog-answers-by-dialog-id | report Counts moved perf rows 4->5 but not the perf suite's green count (6 passed -> 7) the test-plan key states; re-derived from perf.rs:357 | extra_reads 1 | `2026-10-04T17:16:27Z-d` |
| 2026-10-06-local-command-and-paste-framing-rows | the report's Counts missed that verify's quiet waits moved from seven to ten and gave the layout sites as a line range holding 8 of the 9; both surfaced in detector commentary outside any proposal and were corrected in the report after the fan-out | extra_reads 2 | `2026-10-06T22:04:50Z-c` |
| 2026-10-06-local-command-send-outcomes | the report's Counts bullet said test-plan 1077 names the fake agent's options without a count, from a grep hit read as a clipped line; the test-plan detector read the line and it says seven argv options; and the report located expected amendment 9 by its section name,… | extra_reads 2 · reformulations 1 | `2026-10-07T00:03:32Z-b` · .andromeda/runs/2026-10-06T23-49-33-wrap/fanout-results.md |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | the report's sweep for the cross-session claim was keyed on its wording and missed a restatement at architecture.md:70 ('the escaped form is the one the CLI injects'); the detector's own read found it | extra_reads 1 | `2026-10-07T11:10:34Z-b` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | The report named the sixteen unit cases by filter and count and only two of the seven send cases by name; the test-plan detector's section 4 proposal needed the names and took them from source. | — | `2026-10-07T19:30:10Z-c` |
| 2026-10-08-first-live-test-and-self-drive | the report listed the VIOLA_DIR home-resolution step as a claim disproved by a code read without naming the standing route CARRY that owns building it; three detectors returned seven proposals to drop or escalate the step, five were rejected and two applied in part as… | extra_reads 2 | `2026-10-08T09:30:39Z-c` · .andromeda/runs/2026-10-08T09-10-03-wrap/fanout-results.md |

**Proposal:** Directions for the report's form: each Counts line names the master line it moves; a disproved-claim bullet names the route owner when a CARRY owns the claim. Three of the seven reports were reconstructed after the implementing window was gone (chain X5).

### P24 — wrap-session/route-resolve · `contract.carry-no-owner` — 5 cases · weight 8
**Rate:** 5 cases over 22 runs of the step

**Impact (summed):** reformulations 1 · dialogue_rounds 1 · extra_reads 2

**Pattern:** A finding or CARRY had no route entry as its owner, in 5 wraps, and was pinned on the nearest plausible entry: once the addressee was the epoch boundary, which is no route entry; once the prototype switch, which no markerless entry owns; three had no owner named at all.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | the first draft of the cleanup CARRY left the 13th not-measurable mutant 'named, unrouted' (an in-version follow-up with no disposition); re-worded before commit to an owned disposition on the same entry (recorded not-measurable, M1's exemption form) | reformulations 1 | `2026-10-01T12:36:15Z-d` · .andromeda/runs/2026-10-01T12-19-55-wrap/ |
| 2026-10-03-mutation-scoring-completion | the mutation temp-dir leak's owner was left to the dialogue by the directive (:70 or a minted corrective chunk); one halt, the overseer chose :70 with the acceptance as proposed | dialogue_rounds 1 | `2026-10-04T01:25:23Z-b` |
| 2026-10-04-windows-boundary-mutation-workflow | pre-direction (3) placed the audit-classification note 'on the Epoch 3 boundary', which is no route entry; pinned it as a CARRY on :82, the last Epoch 3 entry, naming the boundary audit that follows it | — | `2026-10-04T04:24:02Z-b` |
| 2026-10-04-confirmed-send-with-cl-1-records | the -32602 cause split (ProtocolError::InvalidParams sharing the unsupported-version code, the metric over-counting) had no obviously named owner; pinned to :115 Sanitised error surfaces as the nearest plausible entry, not asked | extra_reads 1 | `2026-10-04T08:58:39Z-b` |
| 2026-10-08-first-live-test-and-self-drive | the directive named a pin for the prototype switch and its gap list but no entry; no markerless entry is the switch's, so it was pinned on the one entry the gap list names for one of its five stop groups, with the three ownerless groups stated as the founder's question… | extra_reads 1 | `2026-10-08T09:34:08Z-b` |

**Proposal:** The epoch boundary is a recurring addressee with no place to hold freight (a second record sits under `contract.no-sanctioned-channel`). A direction: a route form for boundary-addressed freight, on the epoch header or a boundary line, so flip-compaction does not archive it as spent.

### P25 — wrap-session/report · `input.implement-outcome-unsettled` — 6 cases · weight 7
**Rate:** 6 cases over 19 runs of the step

**Impact (summed):** iterations 1 · extra_reads 3

**Pattern:** The operator pass ran after implement's report and folded CI reds, so the outcome basis was the pass's evidence, in 6 chunks; all six fall on or before 2026-10-05.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | the operator pass ran after implement: CI on the pre-CI commit read lint red (an unused cfg(windows)-only import), folded as a fix commit; the final HEAD's green CI run is the outcome basis | iterations 1 | `2026-10-03T07:48:59Z-b` · viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/operator-pass.md |
| 2026-10-04-windows-boundary-mutation-workflow | implement's surfaced report was followed by the overseer's disposition (leak accepted, :72 owner) and the operator pass (dispatch grading the 34); the outcome basis is the operator pass's evidence files | — | `2026-10-04T04:10:35Z-b` · viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/windows-dispatch.md |
| 2026-10-04-confirmed-send-with-cl-1-records | a step ran after implement: the operator pass met and folded a red (text=auto stripped the multi-line fuzz seed's CRLF) in fix commit 306c4ae; the outcome basis is the pass's final HEAD and ci#37183365088 | extra_reads 1 | `2026-10-04T06:47:16Z-b` |
| 2026-10-04-wait-and-last | implement reported green; the operator pass's first CI run went red on macOS (last read the previous turn) and its fold 3de125f changed src/run/wait.rs and send.rs, so the final HEAD's CI run and evidence/operator-pass.md stood in as the outcome basis | extra_reads 2 | `2026-10-04T10:31:42Z-b` |
| 2026-10-04-the-wheel | the operator pass ran after implement's green report and folded four CI reds; the outcome basis is evidence/operator-pass.md and ci#37232840791 on 79ec57c, not implement's report | — | `2026-10-04T20:46:04Z-b` |
| 2026-10-05-dialog-rows-and-re-probe | implement's P4 outcome (CI red, stuck on a fix choice) was superseded by the operator pass's four CI rounds (the settle fix, a timing-only measurement, its revert, the verify_window_ class); the outcome basis is evidence/ci-rounds.md and operator-pass.md at c914216 | — | `2026-10-05T14:30:10Z-b` · viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/ci-rounds.md |

**Proposal:** Calibration: the report record's `operator-pass` entry lives from 2026-10-07, and none of the seven chunks after 2026-10-05 carries this type. What the six records show beside that: implement's green is the local host's, and reds on the other two CI operating systems arrive after it. See chain X3.

### P26 — wrap-session/route-resolve · `contract.no-sanctioned-channel` — 5 cases · weight 7
**Rate:** 5 cases over 22 runs of the step

**Impact (summed):** dialogue_rounds 1 · extra_reads 7

**Pattern:** The wrap needed a write no step on its path owns, five times: `intent.md` and a verified capability's observed gap on a 0-pending wrap; a renumber of 51 route-coordinate citations after an insertion; the re-wording of an unclaimed capability's acceptance on a founder ruling; a P5 ruling that re-opens sentences P2 wrote; two CARRY blocks addressed to the boundary audit.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | relay item B1 listed viola-0.1.0/intent.md:118 and verification-matrix v1-08 observed_gap (status verified) as toolkit sites; the 0-pending path has no writer for intent.md and both are dated route-time observations - left as history by the stated default, the operator… | — | `2026-10-01T12:36:15Z-e` · .andromeda/runs/2026-10-01T12-19-55-wrap/ |
| 2026-10-05-real-cli-verify-probes | inserting two route entries shifted 51 `:NN` coordinate citations across masters, leaves, rules and the matrix; no route.py verb renumbers them, so a scratch listing and an offset-asserted script carried the write (second such renumber on 2026-10-05) | extra_reads 3 | `2026-10-05T11:04:44Z-b` · .andromeda/runs/2026-10-05T10-37-44-wrap/renumber-manifest.md |
| (no chunk) | a founder ruling re-words the acceptance of an unclaimed capability; matrix.py refine refuses a capability no chunk owns and the 0-pending path has no marker, so the ruling landed as a dated note and a CARRY telling the claiming entry to write the text at claim; the ti… | dialogue_rounds 1 · extra_reads 2 | `2026-10-07T06:01:45Z-b` · .andromeda/runs/2026-10-07T05-47-07-wrap/note-v1-33.md |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | a P5 card answer that is a ruling on a spec sentence written at P2 has no named return path: the letter orders reconcile before route-resolve and names no step that re-opens a body; the bodies, the sidecar entries and the leaves were corrected in place and the sweep re… | extra_reads 1 | `2026-10-07T11:39:56Z-c` |
| 2026-10-08-first-live-test-and-self-drive | two CARRY blocks on the chunk's frozen line are addressed to the epoch boundary audit, which is not a route entry: the flip-compaction would archive them as spent; they were re-pinned verbatim on the next markerless entry and named in the handoff, the only surfaces the… | extra_reads 1 | `2026-10-08T09:34:08Z-c` |

**Proposal:** Each is its own gap: a refine path for an unclaimed capability on a ruling; a named return from P5 to P2's bodies; boundary-addressed freight (see `contract.carry-no-owner`). The renumber was met by the citation sweep at the epoch's last wrap. Six untyped records at other steps are the same class (U5).

### P27 — cross-step · `contract.token-proxy-check` — 3 cases · weight 7
**Steps:** phase/distill 1 · implement/code 1 · implement/fix-loop 1

**Impact (summed):** iterations 1 · retries 2 · reformulations 1 · extra_reads 1

**Pattern:** A check tested a token where the property is semantic, three times: an H2 probe word-splitting bold cites (103 false UNRESOLVED); a plan step testing the process name `viola`, which on this host is also another tree's bridge (a false positive); the gate's artifact atom reading freshness by mtime, red on a re-entered run where cargo rebuilt nothing (a false red).

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion (phase/distill) | the orchestrator's H2 probe word-split bold cites carrying a parenthetical qualifier, printing 103 false UNRESOLVED; a scratchpad script keyed on the cite's first token read 79 cites / 0 unresolved | retries 1 | `2026-10-03T22:28:46Z-b` · .andromeda/runs/2026-10-03T22-20-19-phase/*-history.md |
| 2026-10-07-test-homes-off-the-contended-volume (implement/code) | Plan step 5 tests a process NAME (pgrep -x viola) where the intended property is no test or harness process of this repository alive; on this host the name is also the overseer pair's bridge from another tree, 9 alive, one hosting this session. Direction: false positiv… | extra_reads 1 · reformulations 1 | `2026-10-07T07:48:25Z-c` |
| 2026-10-08-first-live-test-and-self-drive (implement/fix-loop) | the gate's artifact atom reads freshness by mtime: on a re-entered run cargo rebuilt nothing, so entry 15 read red (a false red) while its exit and last line held; the truth came from reading the entry's readings with gate.py show | iterations 1 · retries 1 | `2026-10-08T07:41:13Z-f` · .andromeda/runs/2026-10-08T07-13-39-implement/gate-2026-10-08-first-live-test-and-self-drive.json |

**Proposal:** The artifact atom is the pipeline's: freshness by mtime cannot tell a stale artifact from one that needed no rebuild. It recurs in this epoch as two problem-facts and one plan grade besides this record. A direction: an atom that accepts an unchanged artifact when the entry's own command exited 0.

### P28 — new-session/orientation · `input.handoff-git-mismatch` — 4 cases · weight 4
**Rate:** 4 cases over 52 runs of the step

**Impact (summed):** extra_reads 7

**Pattern:** The handoff's Position or Status disagreed with git and the derived cursor at 4 of 52 session starts. Three of the four are one shape: the wrap wrote the handoff, a phase (and once an implement) ran after it, the session was cleared, and the handoff stood one promotion behind with the promotion uncommitted in the tree. The fourth is the session-end hook's three appended lines.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | handoff Status reads clean while git shows the handoff itself modified: a 3-line 'Session End Status: Completed normally at 2026-10-02 14:54:39' block appended after the wrap commit (a session-end hook write, not a wrap bookkeeping rewrite) | — | `2026-10-02T12:56:10Z-b` · git diff --stat at session start: 1 file, 3 insertions |
| (no chunk) | The handoff's Position half (Next: working-route.md:94 to /andromeda-phase) disagreed with the derived route (that entry promoted, one master pending record, plan.md present): the wrap-owned handoff was written at 06:03Z, the phase ran after it and does not rewrite the… | — | `2026-10-07T07:40:09Z-b` · .andromeda/runs/2026-10-07T06-14-00-phase/master-record.txt |
| (no chunk) | The handoff Position (43 complete, 0 pending, Next :100 to phase) disagreed with the derived cursor (pending 1, the :100 chunk; next markerless :102): a phase ran after the wrap and the handoff is the wrap's to write. The tree carried the promotion uncommitted (master-… | extra_reads 1 | `2026-10-07T14:15:11Z-b` · .andromeda/runs/2026-10-07T13-36-31-phase/ |
| (no chunk) | The handoff (written at the 13:15Z wrap) reads Status clean, 0 pending and Next → /andromeda-phase; the master holds 1 pending record for that entry and the tree holds four modified source files, one untracked script and three untracked run dirs of a phase, an implemen… | extra_reads 6 | `2026-10-07T15:10:14Z-b` · .andromeda/runs/2026-10-07T14-40-18-phase/relay-1.md |

**Proposal:** The cursor gave the true position each time. A direction for the contract: name the post-phase tree (an uncommitted promotion, a chunk folder, a run dir) as an expected state beside the bookkeeping list, since the handoff is the wrap's to write and phase does not touch it. Orientation graded the handoff thin or wrong at 8 of the 52 starts in all.

### P29 — implement/code · `input.research-files-wrong` — 3 cases · weight 4
**Rate:** 3 cases over 28 runs of the step

**Impact (summed):** retries 1 · extra_reads 5

**Pattern:** Research's file lists omitted a file the change needed, three times; in each the missing file holds a test that pins the changed behaviour (`tests/hook_fail_open.rs` twice).

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | research lists omitted five files the change needed (viola-channel lib.rs for ChannelError::Deadline, two manifests, liveness.rs snapshot literal, hook_fail_open.rs pinning pre-tool-use as unknown) and named tests/hook_events.rs for a re-pin it did not need | extra_reads 3 | `2026-10-04T14:49:33Z-b` |
| 2026-10-05-dialog-rows-and-re-probe | tests/hook_fail_open.rs is gated by entry 9's binary filter and cited by an acceptance (the capture arm fail-open) but absent from research's Files to modify; recorded in-intent | extra_reads 1 | `2026-10-05T13:08:03Z-b` |
| 2026-10-06-local-command-and-paste-framing-rows | research's Files to modify omitted src/run/version_gate.rs: its unit test stamps_verdict_needs_the_dialog_rows expects a stamp of the fourteen older rows to read verified, so it went red on the first unit run after the row set grew; edited and recorded as a companion | retries 1 · extra_reads 1 | `2026-10-06T20:35:24Z-b` |

**Proposal:** See the pin-sweep direction under `input.plan-step-ambiguous`.

### P30 — implement/fix-loop · `contract.instrument-validity` — 3 cases · weight 4
**Rate:** 3 cases over 25 runs of the step

**Impact (summed):** retries 1 · extra_reads 3

**Pattern:** An instrument could not give the verdict asked of it, three times here and twice at research: a plan probe (`git grep | wc -l` with an `exit 0` atom) red on its satisfied subject under the gate shell's pipefail, its baseline read only on the red side; a refusal with one fixed message that cannot attribute a live red; a control that could not discriminate.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-04-running-turn-refusal | plan probe entry 8 (`git grep ... \| wc -l`, expect exit 0 + last line 0) is red on its satisfied subject: the gate shell runs pipefail and git grep exits 1 on zero matches; its baseline was only ever read on the red (matches) side | extra_reads 1 | `2026-10-04T22:18:54Z-c` |
| 2026-10-05-real-cli-verify-probes | the record path's refusal is one fixed message for four payloads and three screens and keeps no trace, so the live red (entry 7) cannot be attributed; an offline re-run of the product's checks over sessions 2/4 screens and session 4 payloads reads all clean | extra_reads 2 | `2026-10-05T09:45:14Z-b` · viola-0.1.0/chunks/2026-10-05-real-cli-verify-probes/evidence/screen-probe-2.1.288.md |
| 2026-10-05-dialog-rows-and-re-probe | a red-before-green control for the unbounded verify call (a 1 000 ms Stop-receipt hold) could not discriminate: a hold past the 300 ms quiet period defeats the settle it would ride, reading left 4 right 6 under both forms; recorded as not forced, the hang control (plan… | retries 1 | `2026-10-05T14:26:27Z-c` · viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/verify-window-class.md |

**Proposal:** The pipefail probe is recorded at least five times across types in its chunk. A direction for P5's check 4 (9): a new probe's baseline read on its satisfied state as well as its unsatisfied one, the mirror of the known-positive control it already asks for.

### P31 — wrap-session/curation · `ambiguity.tier-routing` — 3 cases · weight 3
**Rate:** 3 cases over 19 runs of the step

**Impact (summed):** —

**Pattern:** A learning had no rule file that loads where it applies, three times: a Linux-host fact routed to a host rule file still named for the retired Windows host; facts about reading a workflow dispatch with no rule covering `.github/` or the operator pass; a unit-test learning for the root bin routed to a testing rule whose paths cover the tests directories only.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | the directive sends a Linux-host fact (btrfs reflink exec-bit loss) to the host rule, whose only rendered file is host-win32.md (always loaded, describing the retired Windows host); written there as a labelled extension of the cargo-mutants TMP entry pending setup's ho… | — | `2026-10-04T01:22:46Z-c` |
| 2026-10-04-windows-boundary-mutation-workflow | the dispatch-reading facts (gh run view blocking, ci.py read after dispatch) have no path-scoped rule home (no rule file covers .github/ or the operator pass), so they went to Tier 3 rather than verification-harness.md | — | `2026-10-04T04:22:08Z-c` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | The test-authoring learning concerns a unit test inside the root bin's source; the registry's home testing.md is path-scoped to the tests directories and does not load for that file. It was routed there by the nearest-scope fallback. | — | `2026-10-07T19:32:55Z-c` |

**Proposal:** The first was closed by setup's host-rule upgrade. For the other two: a rule home for the CI and operator-pass surface, and the testing rule's `paths:` reaching test modules inside `src/`.

## Cross-step chains (starting heuristics)

102 anchors (a step record grading an input thin, wrong or missing, or an `input.*` friction whose artifact is the one its type names); 82 joined to the latest earlier step record of the same chunk that produced the artifact; 20 not joined (no producer record: the handoff at orientation 9, the working route at take-up 4, sidecars and the drift base 5, one CI verdict, one with no artifact). A shape seen in two or more chunks is listed: 12 shapes. No transitive chains, no scoring. `via-input-type` marks an anchor whose step record graded nothing but whose `input.*` friction names the artifact.

### X1 — phase/plan →plan→ implement/code — 12 chunks
13 anchors; the producer closed `ok` with at least one signal at 13 of them. The producer closed `ok` with planlint at 0 hits and its fence parsed in every case; its signals are designed dialogue and authority resolutions, none a quality warning. The consumer paid in reformulations, and at two anchors ended as a halt or soft exit on a step that could not be built. Hypothesis: nothing at P4 measures whether a step is determinate against the code it changes; the first reader that can tell is the implementer.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-readiness-gate-and-timing-constants | `2026-10-04T05:06:21Z-a` · via-input-type · ended ok | step 11 named init_repo and Pass::commit; run.rs test_support::mini carries two more throwaway-repo git calls with the same identity list; the edit script's count assertion caught it | `2026-10-04T04:54:04Z-a` · ok · designed-dialogue:carry1-signature-rows-held, designed-dialogue:carry2-in-process-witness, authority-resolved:extract-seam-advice-yielded-to-measured… |
| 2026-10-04-confirmed-send-with-cl-1-records | `2026-10-04T06:27:45Z-a` · via-input-type · ended ok | three plan details underdetermined the change and were settled in the impl, to surface in the report: the -32602 fault for bad send params (ProtocolError::InvalidParams added), the Git Bash warning placed in c… | `2026-10-04T06:05:10Z-a` · ok · designed-dialogue:F1 partial gate, designed-dialogue:F2 local-command hold, designed-dialogue:F3 verification gap, designed-dialogue:F4 G2 home, auth… |
| 2026-10-04-wait-and-last | `2026-10-04T10:08:48Z-a` · via-input-type · ended ok | plan step 8 says role_of returns Cli for every other first word, but the existing literal row --help -> Other would then flip; kept flags (a leading '-') as Other so the existing oracle stands, surfaced as a d… | `2026-10-04T09:50:37Z-a` · ok · designed-dialogue:fifth-interim-gap-founder-live, designed-dialogue:cli-panic-witness-form, authority-resolved:layouts exit-21 line over design's nam… |
| 2026-10-04-the-wheel | `2026-10-04T18:27:23Z-a` · via-input-type · ended ok | step 5.2 lists release -> driver/release without saying whether a release on a wheel the driver already holds records a cause change; implemented as no change, to surface in the report | `2026-10-04T18:04:42Z-a` · ok · designed-dialogue:pause-release-dated-gap, designed-dialogue:terminal-replies-editing, authority-resolved:obs-plan §4 human-key yielded to arch §Stan… |
| 2026-10-04-running-turn-refusal | `2026-10-04T22:05:32Z-a` · via-input-type · ended ok | plan step 5 and the turn's-life acceptance require a human-origin prompt to make the next send refuse turn-running, unreachable because the human-typing rung precedes it and the only wheel return clears the tu… | `2026-10-04T21:55:48Z-a` · ok · authority-resolved:extract-open-question->architecture.md:70 text |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T09:39:17Z-a` · via-input-type · ended ok | plan steps underdetermined or conflicted at 6 points: &dyn Clock vs a pump-thread sink, the no-screens case vs the 7 s bound, the Stop-less case vs a PROBE_DEADLINE turn wait, row text out of Screen, screen_is… | `2026-10-05T09:14:05Z-a` · ok · fence-unchanged, rulings-folded |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T20:35:24Z-a` · thin · ended ok | 4 points settled by reading: settles between Run B's added pastes; the fake agent's receipt word and Stop for a lacking variant; the dialog-drift contract over a set now drift-only; a step-7 unit literal that… | `2026-10-06T20:15:32Z-a` · ok · authority-resolved:architecture long-paste bullet (keeps the ends byte for byte) yielded to Delivery Confirmation (text as sent) on inputs#I4, gates-… |
| 2026-10-06-local-command-send-outcomes | `2026-10-06T23:23:25Z-a` · via-input-type · ended ok | step 7 fixed the hint case's hold at 6000 ms and sized it against the 20 s CI kill only; the case has no window prefix, so the mutants profile kills it at 10 s and it measured 10.549 s; the same step's wait (t… | `2026-10-06T22:58:47Z-a` · ok · designed-dialogue:readiness-gate-card, authority-resolved:events-record-form-to-matrix-v1-10-and-obs-plan, authority-resolved:missed-post-condition-d… |
| 2026-10-07-test-homes-off-the-contended-volume | `2026-10-07T07:48:25Z-a` · thin · ended ok | steps 0 to 5 executed; step 3 names a call-site swap its own cases cannot reach, step 5 a process-name check that cannot read empty on this host | `2026-10-07T07:33:13Z-a` · ok · designed-dialogue:backing, designed-dialogue:proof-red-side, authority-resolved:the security extract's canonical-home pricing yielded to research's r… |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | `2026-10-07T10:30:18Z-a` · via-input-type · ended ok | the STOP list's header (a STOP ends the live work before another start) against STOP 5, which the plan evaluates at step 7 after every live round; the falsifying shapes were known after start 1, and the live r… | `2026-10-07T09:51:37Z-a` · ok · designed-dialogue:probe-paths, designed-dialogue:chunk-shape, designed-dialogue:hint-home, authority-resolved:the arch extract's no-mechanism reading… |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T14:34:57Z-a` · wrong · ended halted-resolved | step 7's child-entry design cannot be entered through start; its rejected-approach reason describes the standalone binary, not the wrapper's child | `2026-10-07T14:05:59Z-a` · ok · designed-dialogue:remedy-shape, designed-dialogue:strip-width, designed-dialogue:red-closure |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T07:41:13Z-a` · thin · ended soft-exit | steps 0 to 5 ran as written; step 6 assumed a live CLI in a real terminal leaves the wheel with the driver, and no stop rule covered a wheel taken at the start by a terminal reply | `2026-10-08T07:09:42Z-a` · ok · designed-dialogue:compositor-form, revision-by-anchored-edits, size-over-band |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T08:56:11Z-a` · via-input-type · ended ok | plan step 5c gives evidence/key-probe-own.sh the key as a second argument; the standing script took the font size alone and typed a fixed key, so it was extended before the step could run | `2026-10-08T08:28:44Z-a` · ok · designed-dialogue:the closed list's widening, seven shapes against the two this CLI draws |

### X2 — phase/validate →plan→ implement/code — 8 chunks
8 anchors; the producer closed `ok` with at least one signal at 7 of them. The same artifact with validate as its last writer. P5's baselines and controls read as designed at every producer; three of the eight consumers ended as a soft exit on a premise about the live CLI that no baseline can read. Hypothesis: P5 verifies the gate block's instruments, not the steps' premises.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | `2026-10-03T04:04:41Z-a` · thin · ended halted-resolved | step 4 premise (git read-only objects keep TempDir dirs) falsified by measurement; step 5's held-file reading not reproducible | `2026-10-02T19:34:01Z-a` · ok-degraded · baseline-m2-reproduced, baseline-audit-grading-not-reproduced |
| 2026-10-03-mutation-scoring-completion | `2026-10-03T23:02:17Z-a` · via-input-type · ended ok | plan step 5 asks that every runner call be /usr/bin/env -i HOME=… PATH=…, but the passwd probes that produce HOME cannot carry it; the probes carry PATH (system dirs) only | `2026-10-03T22:50:28Z-a` · ok · baseline-caught-token-proxy |
| 2026-10-04-windows-boundary-mutation-workflow | `2026-10-04T02:06:22Z-a` · via-input-type · ended ok | plan step 2's mod-line rule did not say whether modules declared inside a whole-file Windows module are gated too; implemented transitive propagation with a synthetic known positive and carried it to the repor… | `2026-10-04T02:01:02Z-a` · ok · baseline-control-caught-vacuity |
| 2026-10-04-dialog-answers-by-dialog-id | `2026-10-04T11:54:44Z-a` · wrong · ended soft-exit | steps 5/7 rest on a print-mode dialog probe; measured: 2.1.287 print mode exposes neither AskUserQuestion nor ExitPlanMode, so the plan's own STOP clause fired after steps 1-2 | `2026-10-04T11:47:58Z-a` · ok · mechanism-reach-caught-wording, baseline-controls-fired |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T06:32:48Z-a` · wrong · ended soft-exit | step 0 STOP 2 fired: the 2.1.288 trust dialog focuses 'No, exit', so one accept key exits; step 3 phase 1 and the modal-signature check rest on premises the live run and the CLI bundle falsify | `2026-10-05T06:22:20Z-a` · ok · baseline-controls-fired |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T20:02:31Z-a` · via-input-type · ended soft-exit | step 0 names two forms for the session end (the earlier chunk step 0 ended by kill, Run B ends by Ctrl-C) and no line count for the long text; chose Ctrl-C into the settled input box and one line, both named i… | `2026-10-06T19:50:32Z-a` · ok · baseline-controls-reproduced |
| 2026-10-07-send-waits-out-the-paste-hint | `2026-10-07T12:28:55Z-a` · via-input-type · ended ok | plan step 4 asks for cases where the wheel or a turn moves while a verified gate waits, without saying how the wait is produced; a feed during the wait cannot be joined because wait_ready reads the clock under… | `2026-10-07T12:19:07Z-a` · ok · baselines-written |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T15:12:49Z-a` · via-input-type · ended ok | Step 6's rule for the census directory's name is a character class that the words dot and dot-dot satisfy; the step does not say whether they are names. They were refused (exit 2) and the choice is carried to… | `2026-10-07T14:51:54Z-a` · ok · review-added-control, checks-rerun-after-fold, review-prompt-as-text |

### X3 — implement/smoke →implement-outcome→ wrap-session/report — 8 chunks
8 anchors; the producer closed `ok` with at least one signal at 8 of them. Every producer signalled `green` or `surfaced`; every consumer graded the outcome thin because the operator pass ran between the two and folded CI reds. All eight anchors fall on or before 2026-10-05; the report record's `operator-pass` entry lives from 2026-10-07, so none of these consumers could carry it. Hypothesis: implement's outcome is a local-host verdict and the pass that supersedes it wrote no record of its own in this era.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | `2026-10-03T07:48:59Z-a` · thin · ended ok | superseded by the operator pass: CI read red on lint, folded as 9e3b850, then green | `2026-10-03T06:04:15Z-a` · ok · surfaced |
| 2026-10-04-windows-boundary-mutation-workflow | `2026-10-04T04:10:35Z-a` · thin · ended ok | superseded by the operator pass (dispatch grades) and the overseer's leak disposition | `2026-10-04T03:31:03Z-a` · ok · surfaced |
| 2026-10-04-confirmed-send-with-cl-1-records | `2026-10-04T06:47:16Z-a` · thin · ended ok | superseded by the operator pass, which folded a red (the CRLF fuzz seed) after implement's green | `2026-10-04T06:35:29Z-a` · ok · green |
| 2026-10-04-wait-and-last | `2026-10-04T10:31:42Z-a` · thin · ended ok | superseded by the operator pass: a macOS CI red folded as 3de125f after implement's green report | `2026-10-04T10:14:02Z-a` · ok · green |
| 2026-10-04-dialog-answers-by-dialog-id | `2026-10-04T16:55:25Z-a` · thin · ended ok | implement P4 reported green before the operator pass; the pass then found a Windows red and landed three fix commits, one founder-ruled widening | `2026-10-04T15:36:33Z-a` · ok · green |
| 2026-10-04-the-wheel | `2026-10-04T20:46:04Z-a` · thin · ended ok | implement's green P4 report was superseded by the operator pass: four CI reds measured and folded (a product race, a test-side child reader, F-W3's platform fact) | `2026-10-04T18:40:53Z-a` · ok · green |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T10:40:17Z-a` · thin · ended ok | implement's first P4 outcome (stuck at entry 7) was superseded by two operator rulings, the named-refusal widening and the vhome plan correction; the final green + operator pass is the basis | `2026-10-05T10:27:54Z-c` · ok · green |
| 2026-10-05-dialog-rows-and-re-probe | `2026-10-05T14:30:10Z-a` · thin · ended ok | implement's P4 reported CI red and stuck; four operator CI rounds and two overseer decisions followed, the final basis is evidence/ci-rounds.md at c914216 green twice | `2026-10-05T13:21:27Z-a` · ok · green |

### X4 — phase/research →research→ implement/code — 7 chunks
10 anchors; the producer closed `ok` with at least one signal at 9 of them. The producer's one recurring signal is `unresolved-questions`, which names plan decisions. The consumer's grades name something else: a file with a pinning test missing from the lists (3), or a premise about the CLI or the host not measured (6). Hypothesis: research's closure reads that the lists parse, not that they are complete, and its signal vocabulary has no word for a premise left unmeasured (one producer did write `measurement-not-taken`).

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | `2026-10-04T14:49:33Z-a` · thin · ended ok | lists missed the channel lib.rs error variant, viola-state/agent-claude manifests, liveness.rs and hook_fail_open.rs pins; all recorded as companions | `2026-10-04T14:12:39Z-a` · ok · graph-not-applicable |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T06:32:48Z-a` · thin · ended soft-exit | no measurement of the trust dialog's key handling or of trust inheritance from a trusted ancestor (the repo); the record and local-live homes sit under it | `2026-10-05T00:29:31Z-a` · ok · unresolved-questions |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T08:42:29Z-a` · wrong · ended soft-exit | M13/M15: 'every dir under the repo starts with no dialog' — true for the trust dialog only; a subdir cwd shows the external CLAUDE.md imports dialog | `2026-10-05T08:26:36Z-a` · ok · revision, lists-rewritten |
| 2026-10-05-dialog-rows-and-re-probe | `2026-10-05T13:08:03Z-a` · thin · ended ok | Files to modify omitted tests/hook_fail_open.rs, which the plan's gate filter and an acceptance name | `2026-10-05T11:31:17Z-a` · ok · unresolved-questions |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T20:02:31Z-a` · thin · ended soft-exit | M3 states the rows add probes and checks, not normalisation code; the wrapped prompt shape that claim rests on was not measured before step 0 | `2026-10-06T19:34:51Z-a` · ok · unresolved-questions |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T20:35:24Z-a` · thin · ended ok | the two lists missed src/run/version_gate.rs, whose unit test pins the row set | `2026-10-06T20:09:42Z-a` · ok · revision-section-added, lists-parsed |
| 2026-10-07-send-waits-out-the-paste-hint | `2026-10-07T12:28:55Z-a` · via-input-type · ended ok | the fake agent receipts the Enter that submits a prompt as a key line of its own; neither plan step 10 nor research said so, and the keystroke case's first run read red on its receipt oracle while every produc… | `2026-10-07T12:08:01Z-a` · ok · unresolved-questions |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T14:34:57Z-a` · wrong · ended halted-resolved | both lists exact (4 modified, 2 new, 0 outside); the P4 line 'a child's arguments are the test's to give' misses the wrapper's plugin flag ahead of them | `2026-10-07T13:56:05Z-a` · ok · unresolved-questions |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T06:36:48Z-a` · thin · ended ok-degraded | both lists right (src/human.rs exists, evidence/ new); M1 recorded no locker process and never read the compositor lock state | `2026-10-08T05:28:57Z-a` · ok-degraded · unresolved-questions, measurement-not-taken, scope-lists-parsed |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T07:41:13Z-a` · thin · ended soft-exit | both lists right (scope read clean, 0 record lines); M1r measured the key under the fake agent only, which sends no terminal query; the 25-item gap list of M9 was not kept item by item and was re-derived | `2026-10-08T07:04:06Z-a` · ok · revision-premise-closure, measured-under-fake-agent |

### X5 — wrap-session/report →report→ wrap-session/reconcile — 7 chunks
7 anchors; the producer closed `ok` with at least one signal at 3 of them. Three of the seven producers carry `reconstructed`: the report was rebuilt from the diff after the implementing window was gone. The consumer then lacked a count, a name or a site. Hypothesis: a reconstructed report is thinner in its Counts family, and the detectors fill the gap from source (which the proposal-format ban then rejects).

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | `2026-10-04T01:20:42Z-a` · thin · ended ok | the pre-push closed reasons/details retired with sync were absent; the test-plan detector inferred them, and the orchestrator confirmed them in code and extended the report before validation | `2026-10-04T01:04:49Z-a` · ok · — |
| 2026-10-04-dialog-answers-by-dialog-id | `2026-10-04T17:16:27Z-a` · thin · ended halted-resolved | carried every Change the 68 proposals rested on; did not carry the perf suite's green count (6->7) or name RECORDED_CLI_VERSION, so the orchestrator re-read perf.rs:357 and the plan | `2026-10-04T16:55:25Z-a` · ok · — |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T22:04:50Z-a` · thin · ended halted-resolved | two facts were corrected from detector notes: the layout site list (9 sites on 9 lines, not a 471-484 range) and the missing count of verify's quiet waits, seven to ten | `2026-10-06T21:47:49Z-a` · ok · reconstructed |
| 2026-10-06-local-command-send-outcomes | `2026-10-07T00:03:32Z-a` · thin · ended ok | one Counts bullet misstated a master line (test-plan 1077 carries the option count) and expected amendment 9 named a section whose claim sites were three other lines; both corrected in the pass | `2026-10-06T23:52:31Z-a` · ok · — |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | `2026-10-07T11:10:34Z-a` · thin · ended halted-resolved | its site list for the cross-session claim missed architecture.md:70, a restatement with none of the swept wording; the architecture detector found it | `2026-10-07T10:57:21Z-a` · ok · — |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T19:30:10Z-a` · via-input-type · ended ok | The report named the sixteen unit cases by filter and count and only two of the seven send cases by name; the test-plan detector's section 4 proposal needed the names and took them from source. | `2026-10-07T19:17:53Z-a` · ok · reconstructed |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T09:30:39Z-a` · thin · ended halted-resolved | its disproved-claims bullet on the VIOLA_DIR step did not name the route entry that owns the step, and seven proposals in three docs proposed dropping or escalating it | `2026-10-08T09:13:58Z-a` · ok · reconstructed, new-text-generated, new-text-rows:1 |

### X6 — phase/plan →plan→ implement/fix-loop — 5 chunks
6 anchors; the producer closed `ok` with at least one signal at 6 of them. The grades name gate entries that cannot hold as written: an `exit 0` atom under pipefail, a `--home` path the hygiene check refuses, an artifact atom stale on a re-fired block, a census count for a design that cannot be built. Hypothesis: a gate entry authored at P4 is baselined on one side only (see the proposal on `contract.instrument-validity`).

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-running-turn-refusal | `2026-10-04T22:18:54Z-a` · thin · ended halted-resolved | entry 8's `exit 0` atom cannot hold on the green state: git grep exits 1 on zero matches under the gate shell's pipefail | `2026-10-04T21:55:48Z-a` · ok · authority-resolved:extract-open-question->architecture.md:70 text |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T09:45:14Z-a` · thin · ended soft-exit | the record entry refuses with one fixed message for any payload or screen, so a red names no class; the plan's 'fire after the non-live gates are green' cannot hold (the fixture-reading gates need the recordin… | `2026-10-05T09:14:05Z-a` · ok · fence-unchanged, rulings-folded |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T10:27:54Z-a` · wrong · ended halted-resolved | record entries 7/8 passed --home "$h/home"; the /home/ component survives the scrub and the hygiene check refuses it (product as designed); corrected by the overseer, driven by hand with $h/vhome | `2026-10-05T09:14:05Z-a` · ok · fence-unchanged, rulings-folded |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T20:41:30Z-a` · thin · ended soft-exit | the Test Commands ran as written for the 12 entries fired; step 3 says the run pastes only into a settled input box and step 0's record did not carry the 5.8 s the box took to settle after the long-paste turn | `2026-10-06T20:15:32Z-a` · ok · authority-resolved:architecture long-paste bullet (keeps the ends byte for byte) yielded to Delivery Confirmation (text as sent) on inputs#I4, gates-… |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T14:38:43Z-a` · wrong · ended soft-exit | entries 1 to 15 ran as written and read green; entry 16's atom (profiles 9600) and the script's two-a-run rule describe the test binary as the program, the design that cannot be built | `2026-10-07T14:05:59Z-a` · ok · designed-dialogue:remedy-shape, designed-dialogue:strip-width, designed-dialogue:red-closure |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T07:41:13Z-d` · thin · ended soft-exit | 23 entries ran as written; entry 15's artifact key reads STALE on a re-fired block when cargo has nothing to rebuild; the three reader entries are red by design until the live work writes their records | `2026-10-08T07:09:42Z-a` · ok · designed-dialogue:compositor-form, revision-by-anchored-edits, size-over-band |

### X7 — phase/research →research→ implement/fix-loop — 5 chunks
5 anchors; the producer closed `ok` with at least one signal at 5 of them. Each consumer names a companion the red run surfaced that research's sweep had not listed. The producers signalled `unresolved-questions` only. Hypothesis as at research to code: the sweep does not reach tests and helpers that pin the changed behaviour.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | `2026-10-04T00:32:34Z-a` · thin · ended ok | the measured M3 B form ran its copy on /tmp tmpfs, which hid both the quota and the btrfs reflink exec-bit loss | `2026-10-03T22:38:49Z-a` · ok · unresolved-questions |
| 2026-10-04-wait-and-last | `2026-10-04T10:13:15Z-a` · thin · ended ok | did not name the frame.rs envelope typing of from, which pre-empts the method's -32602 | `2026-10-04T09:42:00Z-a` · ok · unresolved-questions |
| 2026-10-04-dialog-answers-by-dialog-id | `2026-10-04T15:36:14Z-a` · thin · ended ok | two companions the red run surfaced were in neither list: the windows-mutants workflow scope and the channel_endpoint unknown-method probe | `2026-10-04T14:12:39Z-a` · ok · graph-not-applicable |
| 2026-10-04-the-wheel | `2026-10-04T18:40:24Z-a` · thin · ended ok | the 25-writer sweep covered keys before a send; the stop's Ctrl-C taking the wheel in tests/hook_events.rs was outside it | `2026-10-04T17:46:16Z-a` · ok · unresolved-questions |
| 2026-10-07-send-waits-out-the-paste-hint | `2026-10-07T12:46:39Z-a` · thin · ended ok | the sweep of cases that turn by the constant missed one in a listed file (typed.rs, a settle read at 6000 ms on a poisoned screen) | `2026-10-07T12:08:01Z-a` · ok · unresolved-questions |

### X8 — phase/research →research→ phase/plan — 5 chunks
5 anchors; the producer closed `ok` with at least one signal at 3 of them. Synthesis read what research had not (11 extra reads over five chunks) and amended research.md in place. Two producers ran without the code graph or with a measurement not taken and said so.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-windows-boundary-mutation-workflow | `2026-10-04T01:57:32Z-a` · thin · ended ok | its gated-file inventory counted Windows gates that sit only inside #[cfg(test)] modules (562 vs 508); refined at P4 and research.md amended | `2026-10-04T01:50:29Z-a` · ok-degraded · derived-without-graph, unresolved-questions |
| 2026-10-04-confirmed-send-with-cl-1-records | `2026-10-04T06:05:10Z-a` · thin · ended ok | missed .config/nextest.toml (mutants 10 s kill vs the 10 s window) and tests/support/home.rs (sized boot, out-of-scan home); both added at synthesis | `2026-10-04T05:56:59Z-a` · ok · unresolved-questions |
| 2026-10-04-wait-and-last | `2026-10-04T09:50:37Z-a` · thin · ended ok | missed that viola-core EventKind lacks question/permission/plan and the per-OS wrapper-kill precedent; both found at P4 and folded back into research.md (E9, E10) | `2026-10-04T09:42:00Z-a` · ok · unresolved-questions |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T06:18:34Z-a` · thin · ended ok | missed the 2.1.288 --help facts (trust dialog interactive-only, --no-session-persistence print-only) and that contract_ledger_probes stamps every recorded set; found by 9 P4 reads | `2026-10-05T00:29:31Z-a` · ok · unresolved-questions |
| 2026-10-08-first-live-test-and-self-drive | `2026-10-08T06:14:27Z-a` · via-input-type · ended ok | the plan needed a second read-only pipeline skill for the clears-between-skills step; research had not listed the installed skills, and the documentation skill the project CLAUDE.md names is not installed on t… | `2026-10-08T05:28:57Z-a` · ok-degraded · unresolved-questions, measurement-not-taken, scope-lists-parsed |

### X9 — implement/smoke →conversation→ wrap-session/curation — 5 chunks
5 anchors; the producer closed `ok` with at least one signal at 5 of them. In five chunks the wrap ran in a resumed or compacted window; the conversation that curation scans was gone and candidates came from the report and a resume file. The producer's signal (`recorded-not-rerun`) is unrelated. Hypothesis: a wrap split across windows loses curation's source; one chunk's resume file carried candidates across, which is the shape of a remedy.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | `2026-10-04T17:17:58Z-a` · thin · ended ok | resumed window: the P1 window's conversation is gone, so only this window and the report's Decisions & corrections were scanned | `2026-10-04T15:36:33Z-a` · ok · recorded-not-rerun |
| 2026-10-04-the-wheel | `2026-10-04T21:01:48Z-a` · thin · ended ok | a resumed window: the implement and operator-pass conversation is gone; resume.md carried its candidates | `2026-10-04T18:40:53Z-a` · ok · recorded-not-rerun |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T11:01:20Z-a` · thin · ended ok-degraded | resumed wrap: the implementing window is gone; candidates came from report Decisions & corrections and resume-point.md only | `2026-10-05T10:27:54Z-c` · ok · recorded-not-rerun |
| 2026-10-05-dialog-rows-and-re-probe | `2026-10-05T14:47:29Z-a` · thin · ended ok | the window that ran implement and P1 is gone; this window held the P2 work only | `2026-10-05T13:21:27Z-a` · ok · recorded-not-rerun |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T19:32:55Z-a` · thin · ended ok | the part before the compaction was read through its summary | `2026-10-07T15:19:01Z-a` · ok · recorded-not-rerun |

### X10 — phase/plan →plan→ phase/validate — 4 chunks
4 anchors; the producer closed `ok` with at least one signal at 4 of them. The plan passed P4's self-check and owed resolutions at P5; see the proposal on `contract.mechanical-check`.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | `2026-10-02T19:34:01Z-a` · thin · ended ok-degraded | shipped three cargo-mutants [[gate]] entries against the 2026-09-28 no-mutation-in-chunk-gates ruling; removed at review (28 -> 25 entries) | `2026-10-02T13:21:40Z-a` · ok · unresolved-questions, designed-dialogue:m2-rename-aside, designed-dialogue:leftover-removal-owner, authority-resolved:P6 lean to never-planned flags… |
| 2026-10-03-mutation-scoring-completion | `2026-10-03T22:50:28Z-a` · thin · ended ok | passed P4 self-check but owed 4 (4) artifact, 4 (6) one-shot prose and a coverage-regex guard, check 9 platform search, and carried 4 mutation gate entries the founder rule bans | `2026-10-03T22:44:27Z-a` · ok · authority-resolved:tests-extract rstest yielded to test-plan history (viola-e2e has no dev-deps), designed-dialogue:M3 form (harness arm vs recipe),… |
| 2026-10-04-the-wheel | `2026-10-04T18:09:05Z-a` · thin · ended ok | two gate entries were defects found at P5: a run --e2e selector the harness lacks (baseline exit 2) and a test() filter over crate names that selects nothing; one probe dropped as a false-match risk; obs G4/se… | `2026-10-04T18:04:42Z-a` · ok · designed-dialogue:pause-release-dated-gap, designed-dialogue:terminal-replies-editing, authority-resolved:obs-plan §4 human-key yielded to arch §Stan… |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | `2026-10-07T14:12:33Z-a` · thin · ended ok | no check failed or warned, but check 4 (9)'s name grep found an existing unit case the remedy turns, which research's sweep and the plan had missed; step 3 was amended before the prompt | `2026-10-07T14:05:59Z-a` · ok · designed-dialogue:remedy-shape, designed-dialogue:strip-width, designed-dialogue:red-closure |

### X11 — phase/distill →extracts→ phase/research — 2 chunks
2 anchors; the producer closed `ok` with at least one signal at 2 of them. Two chunks: extracts assigned out-of-scope cases to the chunk, or listed as an existing pattern something the source does not hold. At the floor of the heuristic.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-the-wheel | `2026-10-04T17:46:16Z-a` · thin · ended ok | a11y/design/layouts assigned the outer-PTY cases (1)(2) and --help grouping to this chunk though :100 owns them; arch read CL-1's send-refused wheel as an event-data amendment, it is a process-log field the sc… | `2026-10-04T17:37:35Z-a` · ok · binding-unilateral, path-check-exact |
| 2026-10-06-local-command-and-paste-framing-rows | `2026-10-06T19:34:51Z-a` · via-input-type · ended ok | the tests extract listed the fake agent emitting the paste-wrap form past the fixture threshold as an existing pattern to follow; the fake agent source holds no paste wrapper at all, so the signal pointed at p… | `2026-10-06T19:28:26Z-a` · ok · binding-unilateral, receipt-count-mismatch |

### X12 — phase/plan →plan→ wrap-session/report — 2 chunks
2 anchors; the producer closed `ok` with at least one signal at 2 of them. The same two plans graded thin at fix-loop, read again at the report; no new fact.

| chunk | consumer record · grade | the consumer's note | producer record · outcome · signals |
|---|---|---|---|
| 2026-10-04-running-turn-refusal | `2026-10-04T22:29:48Z-a` · thin · ended ok | step-5 human-origin oracle unreachable; entry 8 expect atom unreachable on green | `2026-10-04T21:55:48Z-a` · ok · authority-resolved:extract-open-question->architecture.md:70 text |
| 2026-10-05-real-cli-verify-probes | `2026-10-05T10:40:17Z-a` · thin · ended ok | record entries 7/8's --home "$h/home" disproved by measurement; the live-before-gates order could not hold | `2026-10-05T09:14:05Z-a` · ok · fence-unchanged, rulings-folded |

## Level candidates (systemic-masked-as-project)

217 problem-facts on 139 step records: workaround 142 · overridden 41 · deferred 12 · unresolved 9 · removed-cause 9 · prohibition 4. Pass A read every workaround, prohibition and removed-cause fact and clustered them by the obstacle each routes around, across steps and skills; a fact sits in at most one theme, and 32 fit none (appendix). Threshold: n ≥ 3 in the epoch, or n ≥ 2 recurring from an earlier epoch. The level of a problem is the founder's to judge; each entry states facts, a hypothesis and a direction.

| # | signature | theme | facts | chunks | natures |
|---|---|---|---|---|---|
| L1 | band-aid | the project's Bash guard refuses a command shape and the content is rerouted | 15 | 11 | environment 13 · process 2 |
| L2 | band-aid | multi-site edits go through an asserted replace script instead of the Edit tool | 12 | 6 | process 12 |
| L3 | band-aid | a tool's listing read through a pipe or a filter, against the run-bare rule | 12 | 8 | process 12 |
| L4 | band-aid | `inputs.py snap` has no form for the input met | 8 | 7 | product-logic 2 · process 5 · resources 1 |
| L5 | band-aid | the host grep refuses or outruns a bounded-context read; a scratch python scanner stands in | 7 | 5 | environment 6 · resources 1 |
| L6 | band-aid | the wait on P2's background batches is filled with reads that belong to P3 | 6 | 6 | process 6 |
| L7 | band-aid | the code graph's host tools were off the default path on the new Linux host | 6 | 4 | environment 6 |
| L8 | band-aid | the host filesystem under mutation and suite runs | 6 | 3 | environment 5 · resources 1 |
| L9 | band-aid | the permission layer denies a removal of the session's own scratch | 4 | 4 | environment 3 · process 1 |
| L10 | band-aid | the approval word arrives carrying directions, and the plan is edited after it | 4 | 4 | process 4 |
| L11 | band-aid | raw outputs carrying host paths are moved out of the run dir before the commit | 3 | 3 | process 3 |
| L12 | band-aid | a call or a child process outlives its bound | 3 | 2 | environment 2 · process 1 |
| L13 | band-aid | a plan step is replaced by another implementation at implement | 15 | 13 | product-logic 8 · process 6 · environment 1 |
| L14 | removed-cause | a time typed ahead of the clock (the cause removed, and back again) | 3 | 3 | process 3 |
| L15 | removed-cause | the cause removed sat in a file outside research's lists | 3 | 3 | product-logic 2 · process 1 |
| L16 | band-aid | an apostrophe inside a TOML literal string breaks a gate fence or a pattern file | 2 | 1 | process 2 |
| L17 | override | phase Setup's pending guard, against an operator-directed revision of the pending chunk | 4 | 3 | process 4 |
| L18 | chronic-degrade | four `tooling.*` types recurring across epochs | — | — | — |

### L1 — band-aid — 15 facts — the project's Bash guard refuses a command shape and the content is rerouted
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | phase/research | environment · workaround | two read-only probe commands refused by the Bash guard for a doubled backslash (a path rewrite and a PowerShell regex); rerun without it / moved into Write-tool script files | `2026-10-02T13:16:18Z-a#0` |
| (no chunk) | wrap-session/route-resolve | environment · workaround | a file-targeted cat heredoc for a scratch sweep script was refused by the project Bash guard; the script went through the Write tool | `2026-10-03T22:17:30Z-a#0` |
| 2026-10-04-readiness-gate-and-timing-constants | implement/code | environment · workaround | a Bash guard refused `cd <cargo registry>/vt100-0.16.2/src; grep parser.rs` (deny rule ./secrets/** names that relative file); re-ran with absolute paths | `2026-10-04T05:06:21Z-a#0` |
| 2026-10-04-confirmed-send-with-cl-1-records | implement/code | process · workaround | the Bash guard refused two heredoc-to-file commands (fuzz/Cargo.toml append, a scratchpad edit script); the edits went through the Edit tool instead | `2026-10-04T06:27:45Z-a#0` |
| 2026-10-04-dialog-answers-by-dialog-id | implement/code | environment · workaround | the Bash guard refused a cat heredoc to a file for the scratch plugin; the files were written with the Write tool | `2026-10-04T11:54:44Z-a#2` |
| 2026-10-04-dialog-answers-by-dialog-id | wrap-session/reconcile | environment · workaround | a cat heredoc appending to fanout-results.md was blocked by the project's Bash guard; the text went in through the Edit tool | `2026-10-04T17:16:27Z-a#0` |
| 2026-10-04-the-wheel | wrap-session/reconcile | environment · workaround | the Bash guard refused a cat heredoc appending E1's resolution to fanout-results.md; the append went through the Edit tool | `2026-10-04T20:59:31Z-a#0` |
| 2026-10-05-real-cli-verify-probes | implement/code | environment · workaround | the Bash guard refused a cat heredoc to a scratch file; the script went through the Write tool | `2026-10-05T08:42:29Z-a#0` |
| 2026-10-05-real-cli-verify-probes | implement/code | environment · workaround | Bash guard refused a heredoc to a schema file inside a compound call; split into a python edit and a Write-tool file | `2026-10-05T09:39:17Z-a#2` |
| 2026-10-06-local-command-and-paste-framing-rows | implement/code | environment · workaround | the Bash guard refused a cat heredoc with a file target for a scratch script; the script was written with the Write tool and run by path | `2026-10-06T20:35:24Z-a#1` |
| 2026-10-06-local-command-send-outcomes | phase/validate | environment · workaround | a cat heredoc redirected to a scratchpad file was refused by the Bash guard hook; the patch scripts were written with the Write tool and run by path | `2026-10-06T23:04:21Z-a#0` |
| 2026-10-06-local-command-send-outcomes | implement/code | environment · workaround | the Bash guard refused a heredoc redirected to a scratchpad file; the script was written with the Write tool and run by path | `2026-10-06T23:23:25Z-a#1` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/research | environment · workaround | a cat heredoc with a file target was refused by the Bash guard; the scratch scripts went through the Write tool | `2026-10-07T13:56:05Z-a#2` |
| 2026-10-08-first-live-test-and-self-drive | phase/distill | environment · workaround | the validation probe opened with a cd into the run dir and the Bash guard refused it as moving the session cwd; the probe was re-run with absolute paths | `2026-10-08T05:12:25Z-a#0` |
| 2026-10-05-dialog-rows-and-re-probe | implement/code | process · prohibition | the Bash guard refused a heredoc to an evidence file; appended through the Edit tool | `2026-10-05T13:08:03Z-a#1` |

**Level hypothesis:** The refused shapes (a heredoc with a file target, a doubled backslash, a `cd` that moves the cwd) are written by habit in every skill; the rule lives in a project rule file and the refusal in a project hook. The cause appears to sit above the project, in how the agent writes files from the shell; the fixes so far sit in this project. Keyword counts by epoch for the heredoc alone: friction 0, 0, 8, 22; problem-facts 1, 3, 4, 12.

**Proposal:** A pipeline-level home: the remedy inside the guard's refusal text; the fact in setup's host rule template so a new project starts with it; and a capture rule that a hook-caught refusal is one signal, not a friction record per type (see the proposal on `recall.corpus-recurrence`).

### L2 — band-aid — 12 facts — multi-site edits go through an asserted replace script instead of the Edit tool
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-the-wheel | implement/code | process · workaround | multi-hunk edits to dialog.rs, send.rs, cmd/run.rs, human.rs applied by asserted python replace scripts run from the scratchpad instead of the Edit tool; every target was read first | `2026-10-04T18:27:23Z-a#0` |
| 2026-10-05-real-cli-verify-probes | wrap-session/reconcile | process · workaround | multi-site body, contract and leaf edits applied by python byte-mode replace scripts (assert count==1 per anchor) instead of one Edit-tool call per anchor, to batch ~60 anchored replacements | `2026-10-05T11:00:00Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | implement/code | process · workaround | bulk anchored replacements across test files went through scratchpad python scripts (each anchor asserted to match exactly once, newline preserved) instead of one Edit call per site; two scripts stopped on an anchor of mine that did not match and we… | `2026-10-06T20:35:24Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/plan | process · workaround | the plan's point edits went through one scratchpad python script with each anchor asserted to match once, instead of one Edit call per site | `2026-10-06T20:57:18Z-a#1` |
| 2026-10-06-local-command-and-paste-framing-rows | implement/code | process · workaround | the fake agent's eight edits went in as one python write by path with unique-anchor asserts and newline kept, not through the Edit tool; no obstacle forced it, and the read-back was git diff --stat and cargo fmt --check only | `2026-10-06T21:12:21Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | wrap-session/reconcile | process · workaround | the body edits went in as scripted batches of anchored replacements (each anchor asserted to match once, terminators kept) instead of one Edit per site: 21 sites sit on multi-KB lines of one master | `2026-10-06T22:04:50Z-a#0` |
| (no chunk) | wrap-session/route-resolve | process · workaround | the write table names an anchored Edit for inserting an entry; the three head lines and the blocked entry were composed from the old line's own bytes by a scratchpad script and landed with splice delete and append, because the moved freight is multi… | `2026-10-07T06:01:45Z-a#0` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | phase/plan | process · workaround | four same-shaped fence fixes were made by one python script with asserted unique anchors in binary mode, not by anchored Edits; the P4 answers were snapshotted with --step phase:P3, the tool taking no P4 value | `2026-10-07T09:51:37Z-a#0` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | wrap-session/curation | process · workaround | the Tier 2 entry was appended by a binary-mode python append with a byte read-back, not by an anchored Edit as the guide names; the append point was the file's last line | `2026-10-07T19:32:55Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | wrap-session/report | process · workaround | five figures of the report were corrected right after the write by one scripted replace run from the shell, not by the Edit tool; the paste of the generated section followed the corrections | `2026-10-08T09:13:58Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | wrap-session/reconcile | process · workaround | several single-site body and leaf edits were applied by a scripted replace that asserts one match per site, not by the Edit tool; the detectors' returns were taken from their transcripts by script rather than copied by hand | `2026-10-08T09:30:39Z-a#1` |
| 2026-10-08-first-live-test-and-self-drive | wrap-session/route-resolve | process · workaround | the five blocks were appended to three multi-KB entry lines by one script that asserts every other line byte-identical, not by anchored Edits; the two audit blocks were copied from the frozen line by that script, not retyped | `2026-10-08T09:34:08Z-a#0` |

**Level hypothesis:** The letters name an anchored Edit per site. On multi-KB single-line files and on batches of tens of anchors the sessions wrote their own scripts (each anchor asserted to match once, terminators kept) at six different steps. The convention appears not to fit the artifact shape the pipeline itself produces (long entry lines, long master lines); the fix is re-authored per session. A keyword read of problem-facts by epoch: 0, 0, 1, 13.

**Proposal:** A sanctioned batch form: a `splice.py` verb taking a list of (anchor, replacement) pairs, each asserted once, with the read-back the scripts already do by hand.

### L3 — band-aid — 12 facts — a tool's listing read through a pipe or a filter, against the run-bare rule
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-wait-and-last | phase/distill | process · workaround | sidecar.py summary output piped through head -3 against the bare-call rule; the within/OVER verdict line was inside the 3 lines shown | `2026-10-04T09:34:59Z-a#0` |
| 2026-10-04-dialog-answers-by-dialog-id | phase/distill | process · workaround | two tool calls ran through a pipe against the 'every call runs bare' rule: sidecar.py summary piped to head -3 and sidecar.py cites piped to tail -n +2 to trim the stamp line; verdict lines were fully visible, nothing hidden, but the documented bare… | `2026-10-04T11:00:38Z-a#0` |
| 2026-10-04-dialog-answers-by-dialog-id | phase/research | process · workaround | the graph query's JSON stdout was filtered with grep -v \| head rather than read whole from the trace; the authoritative row count (99) was then read from the trace file | `2026-10-04T11:07:02Z-a#1` |
| 2026-10-04-the-wheel | phase/research | process · workaround | piped the code-graph query's stdout through tail -80 against the cookbook's never-clip rule; recovered the full 136-row result from the trace file tree-query-2026-10-04-the-wheel.json, the authority | `2026-10-04T17:46:16Z-a#0` |
| 2026-10-04-running-turn-refusal | phase/distill | process · workaround | sidecar.py summary piped through \| head -3 against the run-bare rule; verdict lines intact | `2026-10-04T21:45:48Z-a#0` |
| 2026-10-04-running-turn-refusal | phase/validate | process · workaround | planlint piped through \| tail -1 once against the run-bare rule; the full listing was re-read bare after the review edit | `2026-10-04T21:58:57Z-a#1` |
| 2026-10-05-real-cli-verify-probes | phase/distill | process · workaround | piped the seven sidecar.py summary calls through \| head -3 despite the every-call-runs-bare rule; the within/OVER verdict line was still visible, no re-run | `2026-10-05T00:26:25Z-a#0` |
| 2026-10-06-local-command-send-outcomes | phase/take-up | process · workaround | route.py pins was read through a grep filter keeping the header, line 92's rows and abstention words, where the letter says every call runs bare; the 58-row listing had already been read bare in this session | `2026-10-06T22:35:47Z-a#1` |
| 2026-10-06-local-command-send-outcomes | phase/distill | process · workaround | the seven sidecar.py cites calls were read through a grep keeping the stamp, the UNRESOLVED and not-a-marker rows and the summary line, where the tools are meant to run bare; the trails hold the full listings | `2026-10-06T22:44:09Z-a#0` |
| 2026-10-07-send-waits-out-the-paste-hint | phase/take-up | process · workaround | the completeness listing route.py pins was run through a grep for the entry's own rows, against the run-bare rule; the full listing had been read bare at orientation and the trail holds it | `2026-10-07T11:53:34Z-a#0` |
| 2026-10-07-send-waits-out-the-paste-hint | phase/distill | process · workaround | the four registry renders and the seven sidecar cites calls had their output redirected to dot-files in the run dir and only their verdict lines read back through grep, against the run-bare rule; done to keep seven listings out of the window, every… | `2026-10-07T12:00:48Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | implement/fix-loop | process · workaround | two targeted gate calls had their listing filtered through grep or a file instead of being run bare; the trail holds each listing whole | `2026-10-08T08:56:11Z-d#0` |

**Level hypothesis:** The rule says every call runs bare; the sessions clipped or filtered `sidecar.py summary` and `cites`, `route.py pins`, planlint, the code-graph query and gate listings across the epoch, one record giving the reason: to keep seven listings out of the window. The rule protects the verdict line; what is routed around is the listing's size. A keyword read of problem-facts by epoch: 1, 2, 3, 11.

**Proposal:** A verdict-only mode on the listing tools (the trail already holds the listing whole), so the bare call and the short call are the same call.

### L4 — band-aid — 8 facts — `inputs.py snap` has no form for the input met
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | phase/research | product-logic · workaround | read the external prototype capture store (~/.viola/sessions) through two scratch readers kept in the run dir, to measure pairing and choose relayed fixtures | `2026-10-04T14:12:39Z-a#1` |
| 2026-10-05-real-cli-verify-probes | phase/plan | process · workaround | inputs.py snap refuses --step phase:P4, so the founder's live split ruling arriving at P4 is cited from the run dir relay-2.md and the plan Provenance instead of an inputs#I3 snapshot | `2026-10-05T06:18:34Z-a#0` |
| 2026-10-05-dialog-rows-and-re-probe | phase/research | process · workaround | the prototype's hook logs (~/.viola/sessions) hold private content of other repositories, so inputs.py snap was given a structure-only census document via --message-file instead of --source on the logs (a verbatim copy into this public repo is barre… | `2026-10-05T11:31:17Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/plan | process · workaround | the card answers were snapped with step phase:P3 because inputs.py has no P4 step value | `2026-10-06T19:45:38Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/plan | product-logic · workaround | the relayed driver log outside the repository was not snapped as a source because it holds prompt text; its claim was re-derived by a counts-only scratchpad script and cited through the relay snapshot | `2026-10-06T19:45:38Z-a#1` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | phase/research | process · workaround | the driver log read at P3 is outside the repository and was not snapshotted into inputs/: it holds prompt text, so the reading is counts only with the script's logic stated, re-run at implement | `2026-10-07T09:07:21Z-a#0` |
| 2026-10-07-send-waits-out-the-paste-hint | phase/plan | process · workaround | the fork answers were snapshotted as an input with --step phase:P3 although they arrived at P4: the snap tool's closed step list has no P4 value | `2026-10-07T12:15:34Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | phase/research | resources · workaround | the operator's hypothesis source (fixtures) held no query bytes, so the CLI's own reply parser was read from its 244 MB binary by a regex scan; inputs.py snap refuses a binary source over 1 MiB, so the read is recorded in the run dir with the binary… | `2026-10-08T07:50:08Z-a#0` |

**Level hypothesis:** Two obstacles under one tool. An answer that arrives at P4 has no step value (`phase:P4` is refused), so it is snapped as P3 or cited from the run dir: 3 facts in 3 chunks, a fourth inside an L2 fact. A source outside the repository that holds private text, or a binary over 1 MiB, cannot be snapshotted, so a counts-only or structure-only document stands in: 5 facts. A third obstacle, no amend verb for a mistyped origin label, sits in L14.

**Proposal:** A `phase:P4` step value; and a stated form for a source that may not be copied (a hash-and-extract record), which the sessions have been inventing each time.

### L5 — band-aid — 7 facts — the host grep refuses or outruns a bounded-context read; a scratch python scanner stands in
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | wrap-session/reconcile | environment · workaround | grep -o with a 250-char context window failed on the host grep (ugrep complexity limit); used a scratchpad python context printer | `2026-10-04T17:16:27Z-a#1` |
| 2026-10-05-real-cli-verify-probes | implement/code | resources · workaround | grep -a with a 300-char window over the 2.1.288 binary exceeded the 120 s call bound and was backgrounded; replaced by an mmap find script | `2026-10-05T08:42:29Z-a#1` |
| 2026-10-05-dialog-rows-and-re-probe | implement/code | environment · workaround | ugrep over the 245 MB CLI binary with a 700-char prefix regex timed out at 120 s; read the same site with a python byte slice | `2026-10-05T11:59:55Z-a#0` |
| 2026-10-05-dialog-rows-and-re-probe | wrap-session/reconcile | environment · workaround | the host grep is ugrep and refused a bounded-context regex (complexity limit); the site scan ran through a scratchpad python script instead | `2026-10-05T14:46:12Z-a#0` |
| 2026-10-05-permission-end-to-end | phase/research | environment · workaround | grep on this host is ugrep, which refused a bounded-context regex over the 245 MB CLI binary ('exceeds complexity limits'); the static bundle read went through a scratchpad python byte-search script instead | `2026-10-05T15:13:17Z-a#0` |
| (no chunk) | new-session/orientation | environment · workaround | a bounded-repeat context grep over plan.md for the live-session cap was refused by the host grep (ugrep, exceeds complexity limits); the cap was read from the chunk's evidence/live-sessions.md instead | `2026-10-06T20:18:55Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | wrap-session/report | environment · workaround | grep -E on this host is ugrep and refused the context-window pattern as exceeding complexity limits; sites were located by a scratch python script run by path | `2026-10-06T21:47:49Z-a#0` |

**Level hypothesis:** The Bash tool's `grep` is an embedded ugrep that refuses a bounded repetition and prints nothing. Every case is new to this epoch (the host changed on 2026-10-03) and each was met by a fresh scratch script. An environment fact of the harness, absorbed one session at a time.

**Proposal:** The fact in setup's host rule template, with the remedy (`command grep`, or a scanner by path); see the proposal on `tooling.host-shell`.

### L6 — band-aid — 6 facts — the wait on P2's background batches is filled with reads that belong to P3
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-06-local-command-and-paste-framing-rows | phase/distill | process · workaround | the two batch waits were filled with read-only code reads that belong to P3 (verify.rs, ledger.rs, hook.rs, two name greps), made before codebase-research.md was read; nothing was written between either snapshot pair | `2026-10-06T19:28:26Z-a#0` |
| 2026-10-07-test-homes-off-the-contended-volume | phase/distill | process · workaround | the waits on the two background batches were filled with read-only code and evidence reads that belong to P3; nothing was written between either snapshot pair | `2026-10-07T06:24:41Z-a#0` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | phase/distill | process · workaround | the two batch waits were filled with read-only research reads that belong to P3 and P4 (product sources, the earlier evidence, a static read of the 2.1.287 binary, the peer listing, the P3 and P4 references); nothing was written between the snapshot… | `2026-10-07T09:01:15Z-a#0` |
| 2026-10-07-send-waits-out-the-paste-hint | phase/research | process · workaround | the read-only file reads research needed were started while P2's distillers and history agents ran, ahead of the extracts, to use the wait; the graph queries, the premise closure and research.md came after P2's validation | `2026-10-07T12:08:01Z-a#0` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/distill | process · workaround | the two batch waits were filled with research-class read-only work outside the tree, ahead of the research step: the folded red's three CI artifacts fetched to the scratchpad and joined by pid and JUnit window | `2026-10-07T13:47:54Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | phase/research | process · workaround | P3 is orchestrator-direct by the letter; two read-only Explore agents were given the repository-facts and gap-list reads to keep the window for synthesis, and read-only code reads began during the stage 2 wait | `2026-10-08T05:28:57Z-a#3` |

**Level hypothesis:** In 6 chunks, all from 2026-10-06 on, the session read code and evidence while the distillers ran, ahead of the extracts the letter orders first; nothing was written between the snapshots. The letter's order is routed around for wall time. One fact adds two read-only sub-agents inside a step the letter calls orchestrator-direct.

**Proposal:** Say in the phase letter whether read-only P3 reads may run during the P2 wait, and under what bound; today each session records it as a deviation.

### L7 — band-aid — 6 facts — the code graph's host tools were off the default path on the new Linux host
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | phase/research | environment · workaround | code-graph host tools (scip-typescript, duckdb, protobuf) sit off the default PATH on the new Linux host; refresh ran with ~/.local/viola-node/bin and ~/.local/viola-venv/bin prepended, per the operator | `2026-10-03T22:38:49Z-a#0` |
| 2026-10-04-windows-boundary-mutation-workflow | phase/research | environment · workaround | code-graph unavailable: python duckdb not importable on this host; impact derived from targeted reads and git grep instead of code-graph.py queries | `2026-10-04T01:50:29Z-a#0` |
| 2026-10-04-windows-boundary-mutation-workflow | phase/plan | environment · workaround | code-graph queried at P4 through ~/.local/viola-venv on the overseer's direction after P3 had closed derived-without-graph; research.md graph section amended | `2026-10-04T01:57:32Z-a#0` |
| 2026-10-04-readiness-gate-and-timing-constants | phase/research | environment · workaround | system python has no duckdb; the code-graph query ran under ~/.local/viola-venv/bin/python, as the previous chunk did | `2026-10-04T04:46:40Z-a#0` |
| 2026-10-04-confirmed-send-with-cl-1-records | phase/research | environment · workaround | code-graph.py failed: duckdb not importable on the host python (health check 11 WARN); installed scripts/requirements.txt into a session-scratchpad venv and ran the queries through it | `2026-10-04T05:56:59Z-a#0` |
| (no chunk) | new-session/orientation | environment · workaround | check 12 parse probe: python yaml module absent on the host; state.yaml judged from its verbatim Setup read (schema_version 3 + the three lean keys only) | `2026-10-05T06:23:28Z-a#0` |

**Level hypothesis:** From 2026-10-03 to 2026-10-05 the python modules the code graph needs were not importable on the host python; each session found a venv, built one in its scratchpad, or derived without the graph. No fact follows 2026-10-05.

**Proposal:** None further is evident: the records stop. The window between a host change and the next setup run is where the cost fell (see also `ambiguity.tier-routing`).

### L8 — band-aid — 6 facts — the host filesystem under mutation and suite runs
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | implement/code | environment · workaround | D: metadata ops cost 75-500 ms; code drafted, compiled and unit-run in a C: copy of the tree, then ported byte-exact to the repo | `2026-10-03T04:04:41Z-a#1` |
| 2026-10-02-epoch-2b-cleanup | implement/code | environment · workaround | step 11 M1 runs ran from C: copies with --in-place: in-repo baselines fail on D: (M2) and the plan form hit LNK1104 (path length) in the deep scratch copy | `2026-10-03T04:04:41Z-a#2` |
| 2026-10-02-epoch-2b-cleanup | implement/fix-loop | environment · workaround | in-repo coverage and suite red on M2; the same commands taken on a byte-equal C: copy for the audit correction numbers (97.59/97.67/97.50), the D: reds recorded with basis | `2026-10-03T06:03:49Z-a#0` |
| 2026-10-03-mutation-scoring-completion | implement/fix-loop | environment · workaround | cargo-mutants 27.1.0 reflink path (reflink 0.1.3 create_new, no mode copy) drops the exec bit on btrfs, so the baseline broke with EACCES on the copied viola-fake-agent; scratch marked NOCOW (chattr +C) so reflink fails and fs::copy keeps modes | `2026-10-04T00:32:34Z-a#1` |
| 2026-10-04-windows-boundary-mutation-workflow | implement/fix-loop | environment · workaround | step-5 leak witness baseline failed (unconnectable_is_true_only_once_nothing_listens: socket path 116 B over sun_path) because the chosen TMPDIR subdir name was long; re-ran in a short-named fresh NOCOW subdir (lw), baseline ok | `2026-10-04T02:10:29Z-a#0` |
| 2026-10-02-epoch-2b-cleanup | implement/code | resources · removed-cause | D: fell below the operator's 40 GB guard mid-M2; stopped and reported; the operator ran cargo clean (151 GB free) and the step resumed | `2026-10-03T04:04:41Z-a#0` |

**Level hypothesis:** Three chunks, 2026-10-02 to 2026-10-04: latency and free space on the Windows volume, then reflink and path-length limits on btrfs. Each was worked around in the chunk (a copy on another volume, a NOCOW scratch, a short directory name).

**Proposal:** None at the pipeline level is evident: the host was replaced and the facts were curated. Listed because the count is at threshold.

### L9 — band-aid — 4 facts — the permission layer denies a removal of the session's own scratch
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | phase/research | environment · workaround | a compound measurement call carrying rm -rf of scratch output dirs was refused by the permission layer; re-run as a scratchpad script writing fresh dirs, no rm | `2026-10-03T22:38:49Z-a#1` |
| 2026-10-04-readiness-gate-and-timing-constants | phase/validate | environment · workaround | a compound control command with rm -r was denied by the permission layer; the controls re-ran as granular calls in the scratchpad with no removal | `2026-10-04T04:57:44Z-a#0` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/plan | environment · workaround | a compound call carrying a removal of a scratch directory was denied whole by the permission layer; the removal was dropped, not rerouted, and the reads re-fired as two calls | `2026-10-07T14:05:59Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | implement/fix-loop | process · workaround | a check of the product binary outside the plan (a send to a name that does not exist, exit 21) created a scratch home on the tmpfs; its removal was denied by the permission layer and the dir was moved by name into target/e2e-home.disk, per the stand… | `2026-10-08T06:44:48Z-a#1` |

Related facts with another solution (not counted in the theme's n):

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-03-mutation-scoring-completion | implement/fix-loop | environment · deferred | rm -r of the 21 leaked /tmp/cargo-mutants-ws-*.tmp dirs (325 MB) was refused by the permission layer; left for the operator | `2026-10-04T00:32:34Z-a#2` |
| 2026-10-04-dialog-answers-by-dialog-id | implement/code | environment · deferred | rm of fixtures/claude/2.1.288 and the stray recording home was refused by the permission layer; removal handed to the operator | `2026-10-04T14:49:33Z-a#3` |
| 2026-10-06-local-command-and-paste-framing-rows | implement/fix-loop | environment · deferred | a test home kept by this run's AGENT_RUN_KEEP_FAILED red reading stays under target/e2e-home: its rm was refused by the permission layer and was not retried; reported for the operator | `2026-10-06T21:30:37Z-a#2` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | implement/code | environment · overridden | a plain rm -r of an own-made scratch home under the target/e2e-home link was denied by the permission layer; asked, and the overseer directed a mv of the three homes into target/e2e-home.disk instead of the plan's removal | `2026-10-07T10:30:18Z-a#0` |

**Level hypothesis:** A compound call carrying `rm` is denied whole, and a plain `rm -r` of an own-made directory is denied too; the sessions dropped the removal, left the directory for the operator (3 deferred facts) or moved it aside on the overseer's word (1 overridden). The cause is the permission configuration; the handling is per chunk and the leftovers accumulate on the operator's desk. No record in the ledger closes any of the three deferrals.

**Proposal:** One sanctioned removal path: the harness's own cleanup verb scoped to directories the run created, or a permission rule for named scratch roots. Which is the founder's choice; the records only show that neither exists.

### L10 — band-aid — 4 facts — the approval word arrives carrying directions, and the plan is edited after it
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-readiness-gate-and-timing-constants | phase/validate | process · workaround | after the yes, matrix.py audit listed the plan's matrix#v1-21 ledger-note line as wrap input although P5 had written the note; the line was reworded post-word and the checks re-run, not re-presented | `2026-10-04T04:57:44Z-a#1` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/validate | process · workaround | the review word was snapshotted as an input and cited by one provenance line added to the plan after the word, as the first revision did; the fence dry-run and the lint were re-run after it, the baselines were not | `2026-10-06T20:58:39Z-a#0` |
| 2026-10-07-test-homes-off-the-contended-volume | phase/validate | process · workaround | the approval word arrived with two directions in one message; they were folded into plan.md after the word, the whole mechanical set re-run on the folded plan, and only then the first matrix call with the run dir fired | `2026-10-07T07:38:33Z-a#0` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/validate | process · workaround | after the review added a count atom to the unit filter entry, its baseline was rewritten by reading the new atom over the recorded output of the baseline run already made, not by a second run of the entry | `2026-10-07T14:12:33Z-a#0` |

Related facts with another solution (not counted in the theme's n):

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-running-turn-refusal | phase/validate | process · overridden | the operator's yes carried a directive: record lean 2's readiness-gate window (up to 5 s) as the wrap residual; folded into plan.md Expected amendments, then the mechanical set was re-run | `2026-10-04T21:58:57Z-a#2` |
| 2026-10-05-real-cli-verify-probes | phase/validate | process · overridden | the word arrived with an amended founder ruling (the overseer set the external-imports flags instead of a hand answer); plan, scope and research edited at the review and the whole mechanical set re-run before the word was taken | `2026-10-05T09:16:30Z-a#0` |

**Level hypothesis:** P5 models the word as a yes. In 6 chunks the yes came with a direction, a ruling or an added control; the plan was edited after the word and the mechanical set re-run, with the edited plan not presented again. The letter's shape is routed around each time.

**Proposal:** A designed arm for a yes with folds: what is re-run, what is re-presented, and where the fold is recorded.

### L11 — band-aid — 3 facts — raw outputs carrying host paths are moved out of the run dir before the commit
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | wrap-session/gates | process · workaround | hygiene refused a run-dir helper script carrying a host path outside the repo; it moved to the session scratchpad before the commit | `2026-10-04T17:23:50Z-a#0` |
| 2026-10-05-permission-end-to-end | implement/smoke | process · workaround | raw boot JSON and F1 nextest logs written into the run dir carried absolute host paths; moved to the session scratchpad before hygiene, the facts kept in evidence and the report | `2026-10-05T15:30:13Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | implement/fix-loop | process · removed-cause | raw outputs of one-shot runs (the pin-first unit run, probe logs, a reader listing) were first written into the run dir, then moved to the private directory outside the tree so the committed run dir holds no raw report | `2026-10-08T08:56:11Z-d#1` |

**Level hypothesis:** Boot JSON, test logs and helper scripts written into a run dir carry absolute host paths, which the hygiene gate refuses at the commit; each session moved them to the scratchpad. Typed correlates: two reconcile corrections of raw twins, one recall record, one plan entry whose `--home` path the check refused.

**Proposal:** Name in the letters where a raw capture goes (outside the committed run dir by default), so the move is not discovered at the gate.

### L12 — band-aid — 3 facts — a call or a child process outlives its bound
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | phase/validate | environment · workaround | gate.py timeout on a 3600 s cargo-mutants entry left its process running; killed by exact pid with taskkill, entries re-narrowed with --re | `2026-10-02T19:34:01Z-a#2` |
| (no chunk) | wrap-session/route-resolve | environment · workaround | an untimed gh run list inside a compound probe held the call past 120 s; re-run alone under timeout 40, returned at once | `2026-10-03T22:17:30Z-a#1` |
| 2026-10-04-dialog-answers-by-dialog-id | implement/fix-loop | process · workaround | the first remove-the-guard run used the default nextest profile; a neutralised continuation held a dialog forever, the run was killed at its time bound before the script restored the file; the leftover line was restored by hand and the controls re-r… | `2026-10-04T15:36:14Z-a#0` |

**Level hypothesis:** A gate entry's timeout left its process running; an untimed call held a compound past the bound; a control run killed at its bound left a mutated line in the source. The untyped cluster U3 holds eight more.

**Proposal:** See U3. For the gate tool specifically: killing the entry's process group on timeout.

### L13 — band-aid — 15 facts — a plan step is replaced by another implementation at implement
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | implement/code | product-logic · workaround | plan step 4's Throwaway guard omitted after its premise was falsified; witness test kept over plain TempDir; surfaced for the wrap | `2026-10-03T04:04:41Z-a#3` |
| 2026-10-03-mutation-scoring-completion | implement/code | product-logic · workaround | the passwd probes (id -u, getent passwd) run as env -i PATH=/usr/bin:/bin with no HOME, because HOME is what they read; narrower than the launcher's HOME+PATH, asserted by the every-call test, surfaced in the report | `2026-10-03T23:02:17Z-a#0` |
| 2026-10-04-readiness-gate-and-timing-constants | implement/code | process · workaround | step 11's two named sites widened to all three run.rs throwaway git arg lists through one GIT_CONFIG const (mini() commits too), keeping the run.rs:1 probe | `2026-10-04T05:06:21Z-a#1` |
| 2026-10-04-confirmed-send-with-cl-1-records | implement/code | product-logic · workaround | src/main.rs role_of files send as Role::Other, so main prints no cli internal-error line for it; with main.rs under the held-boundary guard, the send dispatch arm prints that line itself | `2026-10-04T06:27:45Z-a#1` |
| 2026-10-04-dialog-answers-by-dialog-id | implement/code | product-logic · workaround | red B basis: the plan named target/run-archive plus the reproduction; the archive held no outcomes.json and the reproduction read 0 s, so the committed evidence of earlier workspace mutation runs (max 78 s baseline) was read too and N=400 set from it | `2026-10-04T11:54:44Z-a#0` |
| 2026-10-04-dialog-answers-by-dialog-id | implement/code | product-logic · workaround | plan named parse-rejected detail strict-modes; the closed schema enum admits it nowhere, so the catalog spelling strict-modes-failed was added to parse-rejected's detail enum (companion, expected obs-plan amendment) | `2026-10-04T14:49:33Z-a#2` |
| 2026-10-04-running-turn-refusal | implement/code | product-logic · workaround | plan step 5 says a human-origin prompt-submitted makes the next send refuse turn-running; an unsent human prompt moves the wheel first, so the send reads human-typing and only release (which clears the turn) returns it: the case asserts the turn sta… | `2026-10-04T22:05:32Z-a#0` |
| 2026-10-05-real-cli-verify-probes | implement/code | process · workaround | the plan's 'without --screens' verify case needs ~11 s against the 7 s test bound: made a verify_window_ test with a 20 s bound and a mutants-profile override, the send_window_ precedent | `2026-10-05T09:39:17Z-a#1` |
| 2026-10-05-dialog-rows-and-re-probe | implement/code | product-logic · workaround | Run D waits for its turn's Stop, not the plan's PostToolUse as step 5 words it: ending on the PostToolUse raced the fake agent's receipt line, which would make the receipt counts and the dialog drift contract flaky | `2026-10-05T13:08:03Z-a#0` |
| 2026-10-05-permission-end-to-end | implement/code | process · workaround | F1's planned nextest command lacks --features fake-agent, which the cli_verify target requires; ran it with the feature added and recorded the form in evidence | `2026-10-05T15:27:29Z-a#0` |
| 2026-10-06-local-command-send-outcomes | implement/code | process · workaround | the plan's --paste-hint-ms 6000 made the hint case 10.549 s, over the 10 s kill of the nextest mutants profile it runs under without a window prefix; the hold was shortened to 3000 ms (7.437 s), no bound moved | `2026-10-06T23:23:25Z-a#0` |
| 2026-10-07-test-homes-off-the-contended-volume | implement/code | process · workaround | step 3 asks for the keeper call swapped at its call site and case 2 read red, but the cases call the keeper directly over their own temp dirs; the guard was removed inside each keeper (link arm off) instead and that is recorded in evidence/keeper-co… | `2026-10-07T07:48:25Z-a#0` |
| 2026-10-07-test-homes-off-the-contended-volume | implement/code | environment · workaround | step 5's quiet check (pgrep -x viola empty) read 9 processes, all another tree's viola build the overseer pair runs on; read by executable path instead (this repo 0, other trees 9) and recorded in evidence/backing.md | `2026-10-07T07:48:25Z-a#1` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | implement/code | process · workaround | the plan's hint sequence runs every verb with --json while its layouts acceptance needs the refusal's human lines as printed; one human-mode send was added right after the refused one on the verified home | `2026-10-07T10:30:18Z-a#1` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | implement/code | product-logic · prohibition | the name rule of step 6 (one path segment of letters, digits, dot, underscore, hyphen) admits the two words dot and dot-dot by its letter, which would aim the census directory at target/profraw-census itself or at target; the script refuses both as… | `2026-10-07T15:12:49Z-a#0` |

**Level hypothesis:** A step could not be built or measured as written and the implementer built something else, recorded in evidence or the report. Seven of the 15 facts carry nature process or environment, the rest product-logic. This is the problem-fact side of `input.plan-step-ambiguous` and `contract.premise-falsified`: the cause is in the plan, the fix lands at implement.

**Proposal:** See those two proposals and chains X1 and X2.

### L14 — removed-cause — 3 facts — a time typed ahead of the clock (the cause removed, and back again)
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | implement/code | process · workaround | an inputs origin label was typed ten minutes ahead of the clock; inputs.py has no amend verb, so the label was corrected by an anchored edit of the manifest and inputs verify read clean | `2026-10-07T14:34:57Z-a#1` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/plan | process · removed-cause | the Revised stamp was typed as an estimated time five minutes ahead of the clock; the stamp-ahead hook refused the write result and the stamp was replaced with a date -u reading | `2026-10-06T20:15:32Z-a#0` |
| 2026-10-08-first-live-test-and-self-drive | phase/validate | process · removed-cause | two baseline strings were written with a time four minutes ahead of the clock, for a control re-run not yet made; the stamp hook refused them, the controls were re-run and the read time written | `2026-10-08T06:23:46Z-a#0` |

**Level hypothesis:** The stamp hook refuses the write and the time is re-read from the clock; the same act returns in the next session. With U2's seven untyped records and four typed ones, 11 friction records in 6 chunks, none before this epoch.

**Proposal:** An observation, per the signature: the hook removes each instance and the habit persists. See U2.

### L15 — removed-cause — 3 facts — the cause removed sat in a file outside research's lists
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-wait-and-last | implement/fix-loop | product-logic · removed-cause | crates/viola-channel/src/frame.rs (outside research's lists): the envelope Params typed from as Option<String>, so a non-string from was -32600 before any method ran, against security-plan §Input Validation (Channel frames row: -32602); dropped the… | `2026-10-04T10:13:15Z-a#0` |
| 2026-10-04-the-wheel | implement/fix-loop | product-logic · removed-cause | tests/hook_events.rs (outside research's lists, companion line recorded) pinned event counts the wheel now changes: a human-filed prompt and the stop's Ctrl-C each take the wheel; the Ctrl-C record could be lost at exit, so the wrapper now flushes t… | `2026-10-04T18:40:24Z-a#0` |
| 2026-10-04-running-turn-refusal | implement/fix-loop | process · removed-cause | the folded .profraw WATCH recurred in pre-push 1; cause traced to the fake agent's session-leader exit hanging up an in-flight hook; src/bin/viola-fake-agent.rs fixed outside research's lists on the overseer's founder-delegated word (T3 widening), t… | `2026-10-04T22:18:54Z-a#0` |

**Level hypothesis:** Three fix-loop runs closed a red by changing a file research had not listed (an envelope type, a wrapper flush, the fake agent's exit).

**Proposal:** An observation: see chain X7 (research to fix-loop) and the pin-sweep direction.

### L16 — band-aid — 2 facts — an apostrophe inside a TOML literal string breaks a gate fence or a pattern file
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-running-turn-refusal | phase/plan | process · workaround | six run strings first written as TOML basic strings, converted to literal strings by a Write-tool python script after the template's always-literal rule was re-read | `2026-10-04T21:55:48Z-a#0` |
| 2026-10-04-running-turn-refusal | phase/validate | process · workaround | an apostrophe in a TOML literal baseline string (CARRY's) broke the fence; caught by a grep before the dry-run, reworded | `2026-10-04T21:58:57Z-a#0` |

**Level hypothesis:** n = 2 here, recurring from earlier epochs (a removed-cause fact at 2026-09-26T19:50:03Z-a and an untyped record at 2026-09-29T12:40:29Z-e); two typed records of this epoch carry it as well (`retry.synthesis-rework`, `contract.mechanical-check`). The form is the pipeline's (the gate fence, the cascade pattern file); the breakage is met per chunk.

**Proposal:** One line in the gate-fence and pattern-file templates on quoting; or a dry-run message that names the apostrophe.

### L17 — override — 4 facts — phase Setup's pending guard, against an operator-directed revision of the pending chunk
**Facts:**

| chunk | step | nature · solution | note | record |
|---|---|---|---|---|
| 2026-10-04-dialog-answers-by-dialog-id | phase/research | process · overridden | phase Setup's pending guard would HALT (matrix trail present); the operator directed a revision of the pending chunk, run as P3-P5 on the intact promotion, P1/P2 not re-run | `2026-10-04T14:12:39Z-a#0` |
| 2026-10-05-real-cli-verify-probes | phase/research | process · overridden | operator directive revised the already-pending chunk: the Setup pending-guard HALT was not applied, P1 promote and P2 fan-out were skipped (promotion and extracts kept), and P3-P5 re-run in a new run dir | `2026-10-05T08:26:36Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/research | process · overridden | Setup 3 pending guard reads HALT for a promoted unwrapped chunk; the operator arguments (overseer, founder-delegated, inputs#I4) directed a revision of that chunk, so P1 was skipped, the prior phase run dir reused and the work entered at P3 | `2026-10-06T20:09:42Z-a#0` |
| 2026-10-06-local-command-and-paste-framing-rows | phase/plan | process · overridden | the letter's Setup guard halts on a pending chunk unless P5 is resumable; on the operator's direction this run revised the pending chunk instead, as a P4 re-synthesis in the chunk's existing phase run dir with no promotion and no P2 fan-out, as the… | `2026-10-06T20:57:18Z-a#0` |

**Level hypothesis:** Four overridden facts in three chunks (2026-10-04 to 2026-10-06): the letter halts on a promoted, unwrapped chunk; the operator directed a revision instead, run as P3 to P5 on the intact promotion. Five chunks of the epoch ran phase more than once (coverage); the two latest carry no override fact, which reads as the letter having gained the arm or as the fact not being recorded, and the ledger does not say which.

**Proposal:** The rule halts on a state the operator keeps entering on purpose. If the letter now has a revision arm, nothing remains; if not, the arm is the direction.

### L18 — chronic-degrade — four `tooling.*` types recurring across epochs
**Facts** (cases per epoch, halting cases in brackets, then cases per 100 step records; ledger order of epochs: Epoch 1 · Epoch 2 · Epoch 2b · Epoch 3 · Epoch 4; the last holds six step records so far and is outside the reading):

| type | cases (halting) | per 100 step records | reading |
|---|---|---|---|
| `tooling.hook-friction` | 0 (0) · 1 (0) · 4 (0) · 8 (0) · 0 (0) | 0.0 · 0.9 · 2.3 · 2.3 · 0.0 | three epochs running, no halt in any; the rate per 100 step records rose from 0.9 to 2.3 and held |
| `tooling.host-shell` | 1 (0) · 3 (0) · 1 (0) · 10 (0) · 0 (0) | 0.9 · 2.6 · 0.6 · 2.9 · 0.0 | all four epochs, no halt in any; 10 cases in this epoch after 1, 3 and 1 |
| `tooling.environmental` | 0 (0) · 3 (0) · 2 (0) · 4 (0) · 0 (0) | 0.0 · 2.6 · 1.2 · 1.2 · 0.0 | three epochs, no halt in any |
| `tooling.result-not-run-stable` | 1 (0) · 5 (0) · 2 (0) · 7 (3) · 0 (0) | 0.9 · 4.4 · 1.2 · 2.0 · 0.0 | all four epochs; the first halts are this epoch's three at the wrap gate; none recorded after 2026-10-06 |

`ok-degraded` step records by epoch: 3 of 111 · 3 of 114 · 4 of 171 · 6 of 345 · 0 of 6.

**Level hypothesis:** Three of the four never halt, so the halt policy does not surface them; they are paid as one retry at a time. Their in-epoch evidence is in the proposals on `tooling.hook-friction`, `tooling.host-shell`, `tooling.environmental` and the two `tooling.result-not-run-stable` groups.

**Proposal:** See those proposals and L1, L5 and L8. The epoch-over-epoch counts are the reason they are listed here as well.

**Deferred-forever:** not fired. 12 deferred facts; each note names where the deferral went (a later step, a plan revision, the operator pass, a route WATCH, the operator's desk). Three removals left for the operator have no closing record in the ledger (L9).

**Other overrides:** 37 further overridden facts were read; apart from L17 no rule is overridden three times. Two same-rule pairs are in the appendix (the mutation entries removed from the gate block at review, twice; an escalate-class proposal not halted because the founder's answer was already on record, twice).

## Playbook-extension candidates (untyped patterns, F-4)

57 untyped records, each read and placed in exactly one cluster: 7 clusters at the F-4 threshold (40 records), one pair below it and 15 single records (appendix). Extending a playbook or the universal list is an Andromeda change only the founder applies.

### U1 — phase/research 2 · implement/code 2 · implement/fix-loop 1 · new-session/orientation 1 · phase/plan 1 — 7 cases → proposed type `recall.corpus-recurrence`
**Cluster:** a rule the loaded corpus states, met again outside curation. The same event is typed `tooling.hook-friction` at implement/code and `recall.corpus-recurrence` at the wrap; at research, plan, fix-loop and orientation no listed type fits.

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | phase/research | the doubled-backslash Bash-guard refusal recurred twice in one research step despite host-win32.md Session Additions 2026-09-28 (extended twice) stating it; both rerouted to script files | retries 2 | `2026-10-02T13:16:18Z-b` |
| 2026-10-05-real-cli-verify-probes | implement/code | a cat heredoc to a scratch file was refused by the project's Bash guard (the recorded 2026-09-28 learning recurred); redone through the Write tool | retries 1 | `2026-10-05T08:42:29Z-c` |
| 2026-10-05-dialog-rows-and-re-probe | implement/code | pkill -f with a pattern from the target's command line matched the calling shell's own command line and killed it (exit 144): a recurrence of host-win32.md 2026-09-25 (stop by exact identity, never a command-line substring) | retries 1 | `2026-10-05T11:59:55Z-c` |
| 2026-10-05-dialog-rows-and-re-probe | implement/fix-loop | a heredoc redirected to an evidence file was refused by the Bash guard (recurrence of host-win32.md 2026-09-28/29); appended through the Edit tool | retries 1 | `2026-10-05T13:20:49Z-c` |
| (no chunk) | new-session/orientation | a read-only probe chain ended on an ls over a glob with no match, so the whole Bash call returned exit 2 with its output intact; the host rule for a zero-is-healthy probe (suffix it, or end the chain elsewhere) was in context | — | `2026-10-07T05:46:11Z-b` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/research | a scratch script written with a cat heredoc redirected to a file was refused by the Bash guard, a refusal the project's host rule file already records; one call was lost and the script was re-authored through the Write tool | retries 1 | `2026-10-07T13:56:05Z-c` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/plan | a compound research call that included a removal beside five reads and a timed measurement was denied whole by the permission layer, a shape the project's host rule file already records; one call was lost | retries 1 | `2026-10-07T14:05:59Z-b` |

**Proposed:** recall.corpus-recurrence, valid at every step (it is in the curation and reconcile lists only)

**Draft criteria line:** A rule a loaded rule file already states was met again at this step: name the file and the entry's date, and what refused or caught the act; one record per rule per step.

### U2 — phase/research 2 · phase/plan 2 · phase/take-up 1 · implement/code 1 · phase/validate 1 — 7 cases → proposed type `contract.estimated-stamp`
**Cluster:** a time written from estimate, ahead of the clock. Eleven friction records of the epoch name this event under eight (skill, step, type) keys; none in earlier epochs. Three problem-facts carry it too (L14).

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-05-dialog-rows-and-re-probe | phase/research | two hand-typed times ran ahead of the clock: the inputs#I2 snapshot's --origin says '~11:40Z' though the census ran ~11:28Z (snapshots are never edited in place, so it stands), and this checkpoint's first append was refused REFUSED ts (later than the clock) and re-fire… | retries 1 | `2026-10-05T11:31:17Z-c` |
| 2026-10-05-dialog-rows-and-re-probe | phase/plan | a third hand-typed time ran ahead of the clock: plan.md Metadata 'Generated 2026-10-05T11:50Z' written at ~11:41Z, corrected in place before P5 | iterations 1 | `2026-10-05T11:42:17Z-b` |
| 2026-10-07-test-homes-off-the-contended-volume | phase/plan | plan.md's Generated stamp was written as an estimate 7 minutes ahead of the clock; the stamp-ahead PostToolUse hook refused the write's result and the stamp was corrected from date -u | iterations 1 | `2026-10-07T07:33:13Z-b` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | phase/take-up | scope.md was written with a measurement time estimated two minutes ahead of the clock and a 5/5 split of ten probe dirs counted by eye; the stamp hook refused the time, the re-run read the clock and counted 4 and 6, and both were corrected before the promotion. The han… | retries 1 | `2026-10-07T08:53:01Z-b` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | implement/code | A time typed into an inputs origin label read ten minutes ahead of date -u (the label is hand-typed, the tool stamps read_at beside it). Corrected in the manifest by hand. | retries 1 | `2026-10-07T14:34:57Z-d` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | phase/research | A time was typed into research.md from estimate, two minutes ahead of date -u; the PostToolUse stamp-ahead hook refused the write and the stamp was corrected from the clock. The second estimated time of this session (the first was an inputs origin label at implement). | retries 1 | `2026-10-07T14:43:23Z-b` |
| 2026-10-08-first-live-test-and-self-drive | phase/validate | a time was written into two baseline strings ahead of the clock, estimated for a re-run not yet made; the handoff lists this as a deferred learning met twice before, and it was met again; a hook caught it at the edit | retries 1 | `2026-10-08T06:23:46Z-c` |

**Proposed:** contract.estimated-stamp (cross-step)

**Draft criteria line:** A time was written into an artifact (a stamp, a baseline string, an origin label, a record's `ts`) from estimate instead of `date -u`: record the artifact, the lead in minutes and what caught it (the stamp hook, a tool refusal, a re-read).

### U3 — phase/validate 1 · wrap-session/route-resolve 1 · implement/code 4 · implement/fix-loop 2 — 8 cases → proposed type `tooling.call-lifecycle`
**Cluster:** a Bash call that outlived its bound or its caller. The costliest case left a cargo-mutants process beside later gate entries for about two hours. Three problem-facts are the same class (L12).

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | phase/validate | a gate.py entry timeout (3600 s) left the cargo-mutants process running; the next entries ran beside the orphan, one unmutated baseline failed at the 10 s kill, and about 2 h of baseline time was spent before the overseer measured it | retries 1 · dialogue_rounds 1 | `2026-10-02T19:34:01Z-c` · .andromeda/runs/2026-10-02T12-57-04-phase/ |
| (no chunk) | wrap-session/route-resolve | a compound probe with an untimed gh run list ran past the 120 s Bash limit and was backgrounded and stopped; the same gh call alone under timeout 40 returned at once (cause not established) | retries 1 | `2026-10-03T22:17:30Z-d` |
| 2026-10-04-dialog-answers-by-dialog-id | implement/code | a trailing cat >> /dev/null in an edit command read stdin forever; the call was backgrounded at its 120 s bound and stopped by TaskStop; the python edits before it had landed | retries 1 | `2026-10-04T14:49:33Z-c` |
| 2026-10-05-real-cli-verify-probes | implement/code | a grep -a context scan over the ~200 MB claude binary ran past the Bash call bound and was backgrounded; an mmap find script answered in seconds | retries 1 | `2026-10-05T08:42:29Z-d` |
| 2026-10-05-dialog-rows-and-re-probe | implement/code | pgrep -f \| head -1 named the backgrounded call's bash wrapper, not the python driver; SIGINT went to the wrapper and left driver and child running until a second, pid-exact SIGINT | retries 1 | `2026-10-05T11:59:55Z-d` |
| 2026-10-05-dialog-rows-and-re-probe | implement/code | a backgrounded ugrep that the Bash tool moved to the background at its 120 s bound kept running after the call was abandoned; found by the census and stopped by its exact pid | — | `2026-10-05T11:59:55Z-e` |
| 2026-10-05-dialog-rows-and-re-probe | implement/fix-loop | a backgrounded wait loop picked its target with ls -t \| head -1 over the task dir, which can name its own output file and never end; stopped with TaskStop before it read anything | retries 1 | `2026-10-05T13:20:49Z-d` |
| 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host | implement/fix-loop | a bounded foreground wait on the backgrounded gate block watched for the summary as the output file's last line; the harness appends its own exit line after it, so the wait ran its full 570 s over a block that had ended in about 95 s | — | `2026-10-07T10:41:20Z-c` |

**Proposed:** tooling.call-lifecycle (cross-step)

**Draft criteria line:** A call ran past the Bash bound and was moved to the background, left a child running after a timeout, was stopped through the wrong pid, or was waited on by a loop that could not end: record the mechanism and how the process was found and stopped.

### U4 — implement/code 2 · wrap-session/gates 1 · phase/research 4 — 7 cases → proposed type `tooling.environmental`
**Cluster:** a host or third-party tool behaviour, outside the fix loop. Four of the seven are at research.

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-02-epoch-2b-cleanup | implement/code | cargo-mutants' tree copy under a deep TMP path failed every baseline build with LNK1104 (Windows path length) in 5-8 s; five runs re-taken with --in-place in the disposable copy | retries 1 | `2026-10-03T04:04:41Z-d` |
| 2026-10-02-epoch-2b-cleanup | wrap-session/gates | gate.py hygiene exited 3 once at P7.3c with a git ls-files read timed out after 60 s on the D: volume; the immediate re-run read clean | retries 1 | `2026-10-03T11:45:42Z-c` |
| 2026-10-03-mutation-scoring-completion | phase/research | a cargo mutants --list over member-crate files without --workspace listed only the root package's files, silently dropping viola-pty and viola-channel; re-run with --workspace | retries 1 | `2026-10-03T22:38:49Z-c` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | implement/code | gh api on a job's logs refused to write the response, redirected to a file, until --allow-escape-sequences was passed: exit 1 and a 0-byte file twice. | retries 2 | `2026-10-07T14:34:57Z-e` |
| 2026-10-08-first-live-test-and-self-drive | phase/research | a measurement a founder ruling assigned to this step (does a compositor-typed key move the wheel) was attempted four times over the fake agent and never taken: the classic dispatch form was refused, then a new window on an empty workspace never took keyboard focus, wit… | retries 3 · dialogue_rounds 1 · deferred 1 | `2026-10-08T05:28:57Z-b` · research.md M1 |
| 2026-10-08-first-live-test-and-self-drive | phase/research | a grep of the installed compositor API stub file was refused by the permission classifier with no reason given, in the middle of a probe the operator had authorised; the signatures were taken from the default config and from the dispatcher's own error text instead | extra_reads 2 | `2026-10-08T05:28:57Z-d` · research.md M1 |
| 2026-10-08-first-live-test-and-self-drive | phase/research | A second compositor instance started with no parent aborted at CBackend::create twice with no reason in its output or log; the only working form opens a client window on the locked desktop, whose idle monitor reads it as activity and wakes the screens, after which the… | retries 1 · dialogue_rounds 1 | `2026-10-08T07:04:06Z-b` · .andromeda/runs/2026-10-08T06-46-31-phase/comp-probe-2.log |

**Proposed:** tooling.environmental, valid at research, code and gates (it is in fix-loop's list only)

**Draft criteria line:** A host or third-party tool's behaviour (a filesystem, a path-length limit, a CLI's flag or default, a compositor build, a permission classifier) broke a command that was correct as written: name the mechanism, not the command.

### U5 — wrap-session/reconcile 1 · wrap-session/gates 1 · implement/fix-loop 1 · new-session/orientation 1 · wrap-session/route-resolve 1 · phase/research 1 — 6 cases → proposed type `contract.no-sanctioned-channel`
**Cluster:** the letter or its tool has no arm for the case met. Five typed cases at route-resolve are the same class (the proposal on that group).

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-04-windows-boundary-mutation-workflow | wrap-session/reconcile | security-plan and obs-plan detectors (input/auth/deps; instrumentation/stack/pii) have no invariant covering CI workflow counts or the mutation-in-CI wording, so both returned [] while 5 plan-listed expected amendments sat in their masters; Validate check 5 raised all 5 | extra_reads 2 | `2026-10-04T04:20:25Z-e` |
| 2026-10-04-the-wheel | wrap-session/gates | a resumed P7 has no step that re-writes the P6 handoff: the halt wrote tests-failing, the resumed commit carried it, and the true clean state could only land as an uncommitted post-commit rewrite | retries 1 | `2026-10-04T21:31:09Z-b` |
| 2026-10-06-local-command-and-paste-framing-rows | implement/fix-loop | the gate tool files one log per entry number per run dir, so the targeted re-run of the two red entries overwrote their red logs; the failing blocks survive only as what was read through show and grep before the re-run, and the cause hunt that followed had to work from… | extra_reads 2 | `2026-10-06T21:30:37Z-c` · viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/block-reds-host-contention.md |
| (no chunk) | new-session/orientation | the invocation args reported the wrapped commit's CI run red on attempt 1 and green on a re-run of the same commit; the priority ladder has no rung for an open red on a complete record with 0 pending, so the suggestion stayed rung 4 with the red surfaced as open under… | extra_reads 5 | `2026-10-07T13:34:35Z-b` |
| 2026-10-07-a-send-ending-in-a-newline-is-confirmed | wrap-session/route-resolve | The watch row's trigger names a red with no cause found or a fix by reasoning; this chunk's red has a measured mechanism and an unprovable writer, a case between the two. A watch was pinned on the reading that the link from the mechanism to that run is inference. | — | `2026-10-07T19:32:55Z-f` |
| 2026-10-08-first-live-test-and-self-drive | phase/research | a fact the plan turns on lives in an installed binary the inputs tool cannot snapshot (binary, over 1 MiB): the external-input rule had no form for it, and the record is a run-dir extract with a hash | extra_reads 2 | `2026-10-08T07:50:08Z-b` |

**Proposed:** contract.no-sanctioned-channel, valid at every step (it is in route-resolve's list only)

**Draft criteria line:** The step needed a write, a rung, a form or a verdict the letter or its tool has no arm for (no writer on this path, no rung for this state, no form for this source): record the state class and what was done instead.

### U6 — implement/fix-loop 1 · implement/code 2 — 3 cases → proposed type `tooling.path-anchoring`
**Cluster:** a command failed on where it ran, not on what it ran. At the F-4 floor (n = 3), two of the three in one checkpoint; the universal `tooling.host-shell` may already be read to cover it.

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-04-windows-boundary-mutation-workflow | implement/fix-loop | first backgrounded gate call died before the tool ran: the scratch redirect was written as a relative path climbing out of the run dir, which bash could not open; re-fired with the absolute path | retries 1 | `2026-10-04T02:10:29Z-c` |
| 2026-10-04-readiness-gate-and-timing-constants | implement/code | a cargo check redirect targeted $CLAUDE_SCRATCH, which is unset in the Bash tool's shell, so it wrote to /chk.log and was denied; re-ran with the literal scratchpad path | retries 1 | `2026-10-04T05:06:21Z-c` |
| 2026-10-04-readiness-gate-and-timing-constants | implement/code | the Bash permission guard refused a read-only grep of vt100's parser.rs after a cd into the cargo registry, matching it against the deny rule ./secrets/**; absolute paths passed | retries 1 | `2026-10-04T05:06:21Z-d` |

**Proposed:** tooling.path-anchoring, or a named arm of the universal tooling.host-shell

**Draft criteria line:** A command failed on its anchor: a relative path after the cwd moved, an unset variable inside a path, a deny rule matched by a relative spelling; record the anchor that was missing.

### U7 — wrap-session/reconcile 1 · phase/take-up 1 — 2 cases → proposed type `contract.skill-reference-drift`
**Cluster:** a pipeline tool refused a call its reference gives, or enforced a bound its reference omits. n = 2 in the epoch, recurring from earlier epochs: `cascade.py`'s 16-character pattern id was recorded untyped at 2026-09-27T01:12:11Z-b, 2026-09-27T12:54:15Z-e and 2026-09-29T12:40:29Z-e, and once more here inside a typed `contract.in-pass-correction` record.

| chunk | step | what | impact | record |
|---|---|---|---|---|
| 2026-10-04-running-turn-refusal | wrap-session/reconcile | cascade.py sweep refused the pattern file (exit 2) for an 18-char id (limit 16), then (exit 3) for a regex pattern whose control never fired; renamed and dropped, re-run clean | retries 2 | `2026-10-04T22:36:36Z-c` |
| 2026-10-05-real-cli-verify-probes | phase/take-up | inputs.py snap --message-file exited 2 '--message-file needs --origin'; promotion.md §External inputs names --message-file but not --origin, re-fired with it | retries 1 | `2026-10-05T00:20:30Z-b` |

**Proposed:** contract.skill-reference-drift, with the refusal named in its criteria line

**Draft criteria line:** Also this type: a pipeline tool refuses the call form its reference gives, or enforces a bound its reference does not state (a pattern id's length, a required companion flag); record the reference site and the tool's printed refusal.

## Below threshold — no action

**Typed groups below F-2** (one line each; a per-step group of a universal type is counted in its cross-step group above and not repeated here):

- `new-session/orientation/contract.schema-assumed` — n 2 · weight 4 — the gate count of plan.md Test Commands was first probed with a numbered-list pattern and read 0; the block is a TOML fence of [[gate]] tables, and t… (`2026-10-06T21:00:32Z-b`) ‖ to confirm the operator's stated gate count, the plan's Test Commands entries were counted with a guessed table-row pattern, which read 0; the sectio… (`2026-10-06T23:05:53Z-b`)
- `phase/distill/retry.distiller-respawn` — n 2 · weight 5 — layouts history agent cited three archived Decisions Log entries (2026-09-24 initial generation, T3, T4) from layout-templates-amendments-archive.md,… (`2026-10-04T05:50:49Z-b`) ‖ design and layouts history agents cited 2026-09-24 Decisions Log entries read from the *-amendments-archive.md files (5 and 3 cites), which the WHOLE… (`2026-10-04T09:34:59Z-b`)
- `phase/research/contract.instrument-validity` — n 2 · weight 4 — the first two-sided M3 baseline probe ran under the host git config, whose diff.mnemonicprefix reddened the C3 tests first, so both forms read baseli… (`2026-10-03T22:38:49Z-b`) ‖ the probe script's is-the-workspace-empty check asked the compositor for the active window, which keeps naming the last focused window on an empty wo… (`2026-10-08T05:28:57Z-c`)
- `phase/research/input.extract-signal-gap` — n 2 · weight 2 — the tests extract listed the fake agent emitting the paste-wrap form past the fixture threshold as an existing pattern to follow; the fake agent sour… (`2026-10-06T19:34:51Z-b`) ‖ the security history named a census gate that keeps the cross-session tag literal in hook.rs alone and warned a new literal elsewhere turns it red; a… (`2026-10-06T19:34:51Z-c`)
- `phase/research/tooling.graph-unavailable` — n 2 · weight 3 — python duckdb/protobuf not importable on the Linux dev host (new-session check 11 WARN), so no code-graph query could run; callers derived by git grep (`2026-10-04T01:50:29Z-b`) ‖ code-graph.py query raised ModuleNotFoundError duckdb on python, python3 and /usr/bin/python3, though the prior chunk queried the same DB hours earli… (`2026-10-04T05:56:59Z-b`)
- `phase/take-up/input.carry-context-gap` — n 2 · weight 2 — CARRY 4 cites research M10 of chunk 2026-10-05-real-cli-verify-probes as the measurement that a cross-session prompt needs a second session; that res… (`2026-10-06T19:20:37Z-b`) ‖ the first CARRY's last clause says this run stands as the Linux live confirmation that checks the fake agent's Unix fidelity, without naming what is… (`2026-10-08T05:04:00Z-b`)
- `phase/take-up/input.out-of-pipeline-source` — n 2 · weight 2 — three CARRYs on :68 cite relay linux-route-adaptation.md for measured witnesses (25/13 vs 38/0; Linux TUI 5/0) but the relay is not in the repo; fold… (`2026-10-03T22:22:25Z-b`) ‖ CARRY 1 names the relay `linux-route-adaptation.md` by bare filename; it lives outside the repo (../additional/viola-overseer/), found by a find over… (`2026-10-04T01:33:58Z-b`)
- `wrap-session/gates/tooling.light-gate-red` — n 2 · weight 9 · 1 halting — entry 16 (the no-new-env-read guard over git diff eb53a582c8dc) was green at implement and red at the wrap: the operator pass's later fix commit 8495… (`2026-10-04T21:19:16Z-b`) ‖ probe entry `git grep -F -e '`:82`' -- tests src \| wc -l` red on exit atom (exit 1) with last line 0 holding: a plan-authored atom unreachable on the… (`2026-10-04T22:41:04Z-c`)
- `wrap-session/reconcile/ambiguity.playbook-no-match` — n 2 · weight 6 — A9/A10 (CLI --json wrapper-fault shape vs the arch Standard Contract) matched no playbook rule and escalated by unease; resolved by fixing the code t… (`2026-10-04T08:55:41Z-b`) ‖ two escalations had no playbook rule: a locked decision's measured exception recorded unfixed, and a compiled CLI shape with no verify probe; both re… (`2026-10-07T11:10:34Z-d`)
- `wrap-session/report/contract.detector-fact-gap` — n 2 · weight 2 — The chunk's main fact is a host environment fact set outside code (a link under target to a tmpfs directory, with creation and removal reaching outsi… (`2026-10-07T08:23:43Z-b`) ‖ a measuring chunk's readings (shapes, names, hint timings, the founder's answer) have no stock Changes bullet; a measurements bullet was added so the… (`2026-10-07T10:57:21Z-b`)
- `*/*/contract.grammar-irregularity` — n 1 · weight 2 · sites: viola-0.1.0/working-route.md:66 ×1 — route.py cursor/epoch printed: INDETERMINATE: working-route.md:66 - after ONE space - freight or prose, no structural split: 'FIRST:' (this wrap's ow… (`2026-10-01T12:36:15Z-b`)
- `*/*/recall.change-reconstruction` — n 1 · weight 1 — The session was compacted between implement and the wrap, so Files, Symbols, Counts, Harness surface and Coverage were rebuilt from git diff against… (`2026-10-07T19:17:53Z-b`)
- `*/*/tooling.output-cap-overflow` — n 1 · weight 1 — one cat of health-criteria.md plus evolve-system.md exceeded the tool-result cap (31.5 KB) and was persisted; recovered by a heading grep and two off… (`2026-10-04T18:10:21Z-b`)
- `implement/code/input.conventions-gap` — n 1 · weight 2 — the fake agent receipts the Enter that submits a prompt as a key line of its own; neither plan step 10 nor research said so, and the keystroke case's… (`2026-10-07T12:28:55Z-c`)
- `implement/fix-loop/contract.vacuous-check-found` — n 1 · weight 3 — two new guard tests first read green with their guard removed (throwaway repo: std removes read-only files; remove_owned: held file sorted before own… (`2026-10-03T06:03:49Z-b`)
- `phase/distill/contract.binding-contradiction` — n 1 · weight 1 — check C fired on arch vs obs: arch says recorded screen text goes only to detail-*, obs says diag-detail.v1.json admits no screen text; noted to synt… (`2026-10-05T00:26:25Z-b`)
- `phase/distill/input.spec-source-gap` — n 1 · weight 1 — four extracts (arch, tests, design, layouts) report the masters silent on three verdicts this chunk must produce: the events.ndjson record of an unco… (`2026-10-06T22:44:09Z-b`)
- `phase/validate/contract.matrix-claim` — n 1 · weight 2 — the premise check of v1-29 before its preview found one clause of the cap with no witness: a rewritten path draws a warning is tested as a predicate… (`2026-10-06T23:04:21Z-c`)
- `wrap-session/route-resolve/input.outcome-unclear` — n 1 · weight 1 — The chunk's plan folds a watch (the pre-push coverage merge) and its report records three green runs of it, but no WATCH annotation stands anywhere o… (`2026-10-07T08:37:13Z-b`)
- `wrap-session/route-resolve/tooling.long-line-edit` — n 1 · weight 2 — the two new route entry lines (5.4 KB together) exceeded splice append's 4 KB payload cap; split into two appends of one entry + its arrow line each (`2026-10-05T11:04:44Z-c`)

**Untyped below F-4:**

- cluster, n 2: the driving bridge delivered a stale prompt or left a dialog unanswered. n = 2, no earlier untyped record found by a keyword lookback; emerging, watch next epoch.
  - implement/fix-loop · 2026-10-03-mutation-scoring-completion — a stale /andromeda-phase P3 prompt arrived twice mid-implement as UserPromptSubmit 230 ms after a Stop hook with a background task running and no viola:typed before it (overseer measured from viola e… (`2026-10-04T00:32:34Z-f`)
  - phase/plan · 2026-10-05-real-cli-verify-probes — the P4 AskUserQuestion (two boundary-widening placements), held by viola's dialog hook, got no driver answer within 58 min and returned an error; the turn ended and the operator relayed the founder's… (`2026-10-05T08:31:09Z-b`)
  - wrap-session/reconcile · 2026-10-02-epoch-2b-cleanup — tests-summary.md's header says wrap-session does not modify it while the cascade table names a plan's summary as a re-derived leaf; resolved by the file's history (earlier chunk commits re-derive it) (`2026-10-03T07:54:56Z-d`)
  - wrap-session/curation · 2026-10-02-epoch-2b-cleanup — the first dedup probe (a for-loop of grep -l -i -F piped to tr) printed no file for any term, a known positive included; a single grep -c on a known positive exposed it and a direct grep -o re-ran th… (`2026-10-03T07:56:53Z-c`)
  - new-session/orientation · (no chunk) — Host moved Windows->Linux between wrap and session start: the route head (Mutation scoring completion, WSL leg), the M2 open red (D: volume) and the founder pause on D: all rest on a host that no lon… (`2026-10-03T21:39:51Z-b`)
  - phase/take-up · 2026-10-03-mutation-scoring-completion — the CARRY's re-judge target was cited by line (:131) in the adaptation record; the same wrap's :70 insert shifted it to :133, so the line read a different entry and a title grep re-located it (`2026-10-03T22:22:25Z-c`)
  - implement/fix-loop · 2026-10-03-mutation-scoring-completion — a host-unmeasurable mutant was first recorded as equivalent; the overseer corrected the vocabulary to not measured here, owed to the named route entry (:70) (`2026-10-04T00:32:34Z-e`)
  - implement/fix-loop · 2026-10-04-the-wheel — the default suite and pre-push went red on three tests/hook_events.rs cases (event counts 10 vs 9, 3 vs 2, a count wait reached before the last prompt) that the plan's gate selection for the chunk di… (`2026-10-04T18:40:24Z-c`)
  - wrap-session/route-resolve · 2026-10-04-the-wheel — the minted entry's splice payload carried a 4-space arrow line where the route's 56 arrows are 3-space; caught by an arrow-variant count after the insert and fixed with one anchored Edit (`2026-10-04T21:11:43Z-c`)
  - implement/fix-loop · 2026-10-04-running-turn-refusal — the remove-the-guard control turned a new unit test into a 120 s nextest TIMEOUT (a wrongly pasted send parked on its fixed-clock confirm window), a mutant graded timeout not caught; reworked with a… (`2026-10-04T22:18:54Z-d`)
  - implement/code · 2026-10-05-dialog-rows-and-re-probe — a scripted test edit asserted 7 merge_stamp call sites where 6 took the pattern (a 7th is the definition); the script writes only at its end, so nothing landed and it re-ran with the corrected count (`2026-10-05T13:08:03Z-c`)
  - phase/research · 2026-10-05-permission-end-to-end — the host grep (ugrep) refused two context-window regexes over the CLI binary with 'exceeds complexity limits'; recovered by a python byte-search script in the scratchpad (`2026-10-05T15:13:17Z-b`)
  - implement/code · 2026-10-06-local-command-and-paste-framing-rows — step 11 asks for a scratch probe in step 0's form, but the form on disk is prose in evidence; the driver, hook script and helper themselves sat in an earlier session's scratch dir under the OS temp d… (`2026-10-06T21:12:21Z-b`)
  - implement/code · 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — a permission denial of rm -r on an own-made dir cost one question to the overseer before the gates; the plan's step 5 names the removal and no way to make it (`2026-10-07T10:30:18Z-g`)
  - implement/code · 2026-10-07-a-send-ending-in-a-newline-is-confirmed — The plan names the new function typed_text; hook.rs's test module already holds a proptest strategy of that name, which shadows it there. The first compile of the table failed on it; the table calls… (`2026-10-07T14:34:57Z-c`)
  - wrap-session/gates · 2026-10-07-a-send-ending-in-a-newline-is-confirmed — The gates playbook was opened in the same call that fired the backgrounded light gate, so it was read before the step's outcome existed; the wait had no other read-only work queued. (`2026-10-07T19:39:31Z-b`)
  - new-session/orientation · (no chunk) — the handoff's R-S3 note still schedules Upgrade U02 and the host-win32.md regenerate for the Epoch 3 boundary, while the post-wrap setup-project commit 0dafa09 already ran U02 and U04 and replaced ho… (`2026-10-08T04:58:30Z-b`)

**Note themes below threshold or listed without a hypothesis:**

- the compositor build on the dev host (one chunk) — n 4, 1 chunk(s). Four environment workarounds inside the first live test: a dispatch form refused, a stub file's read denied, a keyword verb refused, a headless start aborting. One chunk, one subject. Facts: `2026-10-08T05:28:57Z-a#1`, `2026-10-08T05:28:57Z-a#2`, `2026-10-08T06:36:48Z-a#2`, `2026-10-08T07:04:06Z-a#0`
- a plan's step or gate order adjusted at run time — n 4, 4 chunk(s). Four facts in four chunks, each for its own reason (a parallel review, a count needed early, a control's timing, compositor uptime); read as not one obstacle. Facts: `2026-10-04T14:49:33Z-a#0`, `2026-10-07T07:33:13Z-a#0`, `2026-10-07T14:38:43Z-a#1`, `2026-10-08T07:41:13Z-a#0`
- route line citations renumbered by a scratch script after an insertion — n 2, 1 chunk(s). n = 2, no earlier fact; three typed or untyped records of the epoch name the same gap, and the epoch's last wrap ran a citation sweep. Facts: `2026-10-05T11:04:44Z-a#0`, `2026-10-07T06:01:45Z-a#1`
- `tools_dir` carried as a shell variable, not the literal substitution — n 2, 1 chunk(s). n = 2. Facts: `2026-10-06T22:32:34Z-a#0`, `2026-10-06T22:35:47Z-a#0`
- the gate's artifact atom read STALE on a re-entered run — n 2, 1 chunk(s). n = 2; see the proposal on `contract.token-proxy-check`. Facts: `2026-10-08T07:41:13Z-d#0`, `2026-10-08T08:28:44Z-a#0`

**Workaround, prohibition and removed-cause facts in no theme (32):**

- wrap-session/route-resolve · (no chunk) · process · prohibition — operator direction wrote a guard into :68 freight: the native pre-push stage keeps env -i HOME+PATH, anything wider halts for the founder live (`2026-10-03T22:17:30Z-a#2`)
- phase/distill · 2026-10-03-mutation-scoring-completion · process · workaround — the operator's CI-red fold (scope item 7, viola-channel macOS close race) arrived while stage 2 ran; scope was amended after both batches, so no extract saw item 7 — carried into P4 straigh… (`2026-10-03T22:28:46Z-a#0`)
- phase/research · 2026-10-04-wait-and-last · environment · workaround — target/debug/viola predates send (built 00:31, exit 2 unrecognized subcommand); the E4 probe used target/harness/debug/viola built 09:00Z from the bcff692 tree instead (`2026-10-04T09:42:00Z-a#0`)
- phase/research · 2026-10-04-dialog-answers-by-dialog-id · environment · workaround — gh api …/jobs/{id}/logs refused to write the log (terminal escape sequences); used gh run view --job {id} --log to a scratchpad file instead (`2026-10-04T11:07:02Z-a#0`)
- implement/code · 2026-10-04-dialog-answers-by-dialog-id · process · workaround — ran three throwaway print-mode claude probes with a scratch capture plugin to measure the plan's STOP premise before writing the dialog tier; no product probe or recording leg was fired (`2026-10-04T11:54:44Z-a#1`)
- wrap-session/reconcile · 2026-10-04-dialog-answers-by-dialog-id · process · workaround — three counts the report does not carry (perf suite 7 passed, PERF_ROWS re-export, MUTANTS_PROGRESS arms) confirmed by reading crates/viola-e2e source before applying (`2026-10-04T17:16:27Z-a#2`)
- wrap-session/curation · 2026-10-04-the-wheel · process · workaround — the --e2e correction was a master claim, so a P2 amendment was raised after P2 closed; the cascade sweep was re-run after it so the sweep still postdates the last amendment (`2026-10-04T21:01:48Z-a#0`)
- wrap-session/gates · 2026-10-04-the-wheel · process · workaround — the committed handoff carried the P7.1 halt's tests-failing status (P6 ran before the halt); rewritten after the commit as uncommitted expected-transient bookkeeping rather than an amend pa… (`2026-10-04T21:31:09Z-a#0`)
- phase/distill · 2026-10-04-running-turn-refusal · process · workaround — history-agent prompts carried one appended line naming the project root (relative sidecar path) — not sent strictly verbatim (`2026-10-04T21:45:48Z-a#1`)
- phase/research · 2026-10-04-running-turn-refusal · process · workaround — 14 file:line citations in research.md were off by 1-5 lines from memory of sed offsets; corrected by a scripted anchored replace after a grep -n re-check (`2026-10-04T21:51:33Z-a#0`)
- wrap-session/gates · 2026-10-04-running-turn-refusal · process · workaround — the operator fix 334ee7f was pushed with plain `git push origin HEAD` instead of plan entry 19's guarded form: its `git diff --quiet` refuses while the wrap's own artifacts sit uncommitted… (`2026-10-04T22:52:46Z-a#0`)
- implement/code · 2026-10-05-real-cli-verify-probes · process · prohibition — step-0 scratch probe: the no-key kill widened from 'trust' text to any trust/external-imports/dialog-footer text before session 4 (probe1 read 'trust' only) (`2026-10-05T09:39:17Z-a#0`)
- implement/fix-loop · 2026-10-05-real-cli-verify-probes · process · workaround — two pre-check test cases rewired (a Stop-less set's trusted turn waits PROBE_DEADLINE; a TMPDIR inside the workspace is trusted by the fake) before the live round (`2026-10-05T09:45:14Z-a#1`)
- phase/research · 2026-10-05-dialog-rows-and-re-probe · environment · workaround — no live claude session is allowed before the founder's cap, so 2.1.288 behaviour was read statically from the installed binary with a scratch byte scanner; every such fact is labelled stati… (`2026-10-05T11:31:17Z-a#1`)
- implement/code · 2026-10-05-dialog-rows-and-re-probe · process · workaround — ended Run D at the STOP by SIGINT to the python driver (its cleanup killed the child and removed the dir) instead of waiting out the driver's 240 s bound with the plan dialog up (`2026-10-05T11:59:55Z-a#1`)
- implement/fix-loop · 2026-10-05-dialog-rows-and-re-probe · process · workaround — a cli_answer.rs case for v1-15 (a plan first raised by PermissionRequest) was added while the full block ran; the buffered run was stopped and the whole block re-run on the final tree (`2026-10-05T13:20:49Z-a#0`)
- phase/research · 2026-10-06-local-command-and-paste-framing-rows · process · workaround — P2 was not re-run for the revision: the seven extracts of the first pass were kept and the tests extract plus four architecture lines and the test-plan Path 2 steps were read directly (`2026-10-06T20:09:42Z-a#1`)
- implement/code · 2026-10-06-local-command-and-paste-framing-rows · process · workaround — the red was run a second time with AGENT_RUN_KEEP_FAILED=1: the case fails at its first assertion, the exit code, so the receipt that shows what the trusted run typed is gone with the test… (`2026-10-06T21:12:21Z-a#1`)
- implement/fix-loop · 2026-10-06-local-command-and-paste-framing-rows · process · removed-cause — the 20 s kills of entry 16 left 21 empty verify probe dirs at the repository root, found by the P4 census and removed with rmdir; the operator-desk dir was not touched (`2026-10-06T21:30:37Z-a#1`)
- wrap-session/route-resolve · 2026-10-06-local-command-and-paste-framing-rows · process · workaround — the route card of this step was put to the operator in the same question as the reconcile escalation, before this phase's own analysis ran, to save a second halt; the earlier reconcile and… (`2026-10-06T22:06:42Z-a#0`)
- phase/research · 2026-10-06-local-command-send-outcomes · process · workaround — a one-line python probe of the diag-line schema's top-level properties printed an empty object because the send fields sit under allOf; the fields were read from a grep of the schema file i… (`2026-10-06T22:51:16Z-a#0`)
- wrap-session/reconcile · 2026-10-06-local-command-send-outcomes · process · workaround — the hook-side log line a detector could not base on the report was to be read from a kept test home; none is kept on the host, so its basis was read from src/cmd/hook.rs and the body says a… (`2026-10-07T00:03:32Z-a#0`)
- wrap-session/route-resolve · 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host · process · removed-cause — two of the four route cards were founder rulings that settled sentences P2 had written as open; the two masters, four sentences of this pass's own sidecar entries and two leaves were brough… (`2026-10-07T11:39:56Z-a#0`)
- phase/validate · 2026-10-07-send-waits-out-the-paste-hint · process · workaround — the kill-line probe's passing control was a scratch copy of the nextest config made in the session scratchpad, not in the run dir; only its verdict is in the plan's baseline line (`2026-10-07T12:19:07Z-a#0`)
- wrap-session/reconcile · 2026-10-07-send-waits-out-the-paste-hint · process · workaround — the offset reads for check 4 went through two scratch scripts (a per-line slice and a regex window lister) written for the report sweep, not through cascade.py window and splice.py summary;… (`2026-10-07T13:13:11Z-a#0`)
- phase/take-up · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process · workaround — Setup 5a's tool reads a run's latest attempt, so the red attempt was read with the provider CLI's per-attempt view beside the documented call (`2026-10-07T13:39:16Z-a#0`)
- phase/research · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · resources · workaround — the refused profile was not uploaded, so the method the directive named could not run; the writer was narrowed by a pid join over the diagnostics artifact and a JUnit window join, then the… (`2026-10-07T13:56:05Z-a#0`)
- phase/research · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process · workaround — research ran the last pre-push's instrumented test binary on one test 1 240 times with the profile path pointed at the scratchpad, a measurement beyond reads, to verify the mechanism's equa… (`2026-10-07T13:56:05Z-a#1`)
- wrap-session/gates · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process · workaround — this checkpoint's playbook was read while the light gate was still running in the background, before the step had completed, against the reading-order rule; the gate's run was already fired… (`2026-10-07T19:39:31Z-a#0`)
- new-session/orientation · (no chunk) · process · workaround — route.py epoch prints the epoch header clipped with an ellipsis, so the byte-exact epoch text this record needs was read from working-route.md line 65 directly (`2026-10-08T04:58:30Z-a#0`)
- implement/code · 2026-10-08-first-live-test-and-self-drive · process · workaround — start 6's step lines went to live-run-start6.ndjson, a file of its own, and live-run.ndjson was not created, so a retried run's reader ledger starts clean (`2026-10-08T07:41:13Z-a#3`)
- phase/research · 2026-10-08-first-live-test-and-self-drive · process · workaround — research ran a three-window reply probe on the standing compositor (no live start, no key) for the one parser type the implement run's battery had not sent; its scripts live in the phase ru… (`2026-10-08T07:50:08Z-a#1`)

**Overridden facts outside L17 (37):**

- phase/take-up · 2026-10-02-epoch-2b-cleanup · process — the operator took the overseer-pre-named split of M3 + the WSL leg out of the taken-up entry at P1; owed to a new entry minted at this chunk's wrap (`2026-10-02T12:59:46Z-a#0`)
- phase/validate · 2026-10-02-epoch-2b-cleanup · resources — overseer stopped the whole-file mutation baselines (2.5 h and 83 min in-tree); viola-state and root baselines recorded blocked-by-M2, taken at implement after the M2 fix (`2026-10-02T19:34:01Z-a#0`)
- phase/validate · 2026-10-02-epoch-2b-cleanup · process — overseer review removed the three mutation entries from the gate block (founder 2026-09-28 ruling); step 11 became a one-off witness (`2026-10-02T19:34:01Z-a#1`)
- wrap-session/gates · 2026-10-02-epoch-2b-cleanup · environment — light gate red on D: (M2 class) incl. two entries green at implement, pre-push timed out at 5400 s; the overseer ruled them M2 class with owner the :68 CARRY and no HEAD-on-D: control (the… (`2026-10-03T11:45:42Z-a#0`)
- phase/validate · 2026-10-03-mutation-scoring-completion · process — the operator's review removed the 4 mutation [[gate]] entries (founder rule 2026-09-28: no mutation entries in a chunk gate block), the same correction as at the epoch-2b-cleanup chunk; the… (`2026-10-03T22:50:28Z-a#0`)
- implement/fix-loop · 2026-10-03-mutation-scoring-completion · environment — M3 run 1 died at 460/709 on the /tmp tmpfs usrquota; overseer directed TMPDIR on btrfs for every mutation run, the died unit re-run in full, partial outcomes never merged (`2026-10-04T00:32:34Z-a#0`)
- implement/fix-loop · 2026-10-03-mutation-scoring-completion · process — the two Windows-only scratch.rs prepare mutants, first recorded equivalent, were re-dispositioned by the overseer as not measured here, owed to :70; missed==0 reads over the measurable set (`2026-10-04T00:32:34Z-a#3`)
- implement/fix-loop · 2026-10-04-readiness-gate-and-timing-constants · process — plan step 12 placed the CARRY 4 two-sided witness in the operator pass; the operator's invocation directed mutation-run TMPDIR handling, so implement ran the economy witness itself in two w… (`2026-10-04T05:15:54Z-a#0`)
- wrap-session/reconcile · 2026-10-04-confirmed-send-with-cl-1-records · process — the wrap folded a source change (src/cmd/send.rs wrapper-fault --json to the arch contract's detail object) on the overseer's founder-delegated E3 resolution; the wrap otherwise never touch… (`2026-10-04T08:55:41Z-a#0`)
- phase/plan · 2026-10-04-dialog-answers-by-dialog-id · process — the plan's Constraints rejected hand-sourced tool-bearing fixtures; the founder ruled live that relayed prototype captures replace the dialog probe, so the Constraint line was rewritten to… (`2026-10-04T14:17:01Z-a#0`)
- phase/validate · 2026-10-04-dialog-answers-by-dialog-id · process — unclaim is the wrap valve per the matrix contract; run at phase P5 on the operator's yes carrying the founder ruling that v1-15 is claimed at :82 (`2026-10-04T14:20:02Z-a#0`)
- implement/code · 2026-10-04-dialog-answers-by-dialog-id · environment — mise moved claude to 2.1.288 mid-chunk; the spine recording read stamped 2.1.288; operator ruled option 1: re-record against the pinned 2.1.287 binary (run text gains -- <path>) (`2026-10-04T14:49:33Z-a#1`)
- implement/code · 2026-10-04-dialog-answers-by-dialog-id · process — overseer review rejected the plan pair as private text in a public repo; the relay script gained an in-script redaction of tool_input.plan and was re-fired (`2026-10-04T14:49:33Z-a#4`)
- wrap-session/reconcile · 2026-10-04-the-wheel · process — the plan's expected amendment asked for a security-plan Decisions Log entry; amendment-flow Apply step 1 forbids a new Decisions Log entry, so the sidecar entry carries the record (`2026-10-04T20:59:31Z-a#1`)
- phase/validate · 2026-10-04-running-turn-refusal · process — the operator's yes carried a directive: record lean 2's readiness-gate window (up to 5 s) as the wrap residual; folded into plan.md Expected amendments, then the mechanical set was re-run (`2026-10-04T21:58:57Z-a#2`)
- phase/plan · 2026-10-05-real-cli-verify-probes · environment — the first AskUserQuestion (the split card) was released unanswered after 58 min by the viola hook this session's driver answers through; the operator relayed the founder's answer as text an… (`2026-10-05T06:18:34Z-a#1`)
- phase/validate · 2026-10-05-real-cli-verify-probes · process — the word arrived with an amended founder ruling (the overseer set the external-imports flags instead of a hand answer); plan, scope and research edited at the review and the whole mechanica… (`2026-10-05T09:16:30Z-a#0`)
- implement/fix-loop · 2026-10-05-real-cli-verify-probes · process — the operator approved a named --record refusal (file + check code) after an unattributable live red; it named SessionStart.default.json absolute-path on the next round (`2026-10-05T10:27:54Z-a#0`)
- implement/fix-loop · 2026-10-05-real-cli-verify-probes · process — entries 7 and 8 driven by hand with --home "$h/vhome" (the overseer's dated plan correction) instead of the plan's $h/home (`2026-10-05T10:27:54Z-a#1`)
- implement/fix-loop · 2026-10-05-real-cli-verify-probes · resources — the live cap raised twice by the founder (12→15→18) after two STOPs (`2026-10-05T10:27:54Z-a#2`)
- wrap-session/report · 2026-10-05-real-cli-verify-probes · process — the operator directed a P1-only wrap at 61.5 % context with a resume point naming P2 (`2026-10-05T10:40:17Z-a#0`)
- wrap-session/gates · 2026-10-06-local-command-and-paste-framing-rows · environment — three whole-block runs each read one red on a different entry while another builder session's cargo wrote 10.8, 23.0 and 11.6 GB to the same volume; the wrap halted for the operator, who ha… (`2026-10-06T22:24:12Z-a#0`)
- new-session/orientation · (no chunk) · process — the ladder's rung 4 names /andromeda-phase for working-route.md:94; the operator's invocation stated the next act as a 0-pending /andromeda-wrap-session adapting the route on four founder r… (`2026-10-07T05:46:11Z-a#0`)
- wrap-session/reconcile · 2026-10-07-test-homes-off-the-contended-volume · process — four proposals carry the never-routine boundary-widening class, which escalates and halts; no halt was taken at this wrap because the founder's answer was given live after this widening was… (`2026-10-07T08:34:21Z-a#0`)
- implement/code · 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host · environment — a plain rm -r of an own-made scratch home under the target/e2e-home link was denied by the permission layer; asked, and the overseer directed a mv of the three homes into target/e2e-home.di… (`2026-10-07T10:30:18Z-a#0`)
- implement/code · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process — step 7's planned child (the test binary at a child-entry test) is unbuildable: the wrapper's --plugin-dir comes first and libtest exits 101. Asked on one card; the overseer chose an uninstr… (`2026-10-07T14:34:57Z-a#0`)
- implement/fix-loop · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process — the operator stopped the run for a plan revision before entry 16 and the operator pass: no push goes out with a gate entry red by its own letter (`2026-10-07T14:38:43Z-a#0`)
- implement/fix-loop · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process — an operator note asking for a rewording before the pre-CI commit arrived after that commit was pushed; the rewording went in as a fix commit on top, its push held until the first run's Wind… (`2026-10-07T15:39:29Z-a#0`)
- implement/fix-loop · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process — the note named the plan's expected-amendment lines; plan.md is a file implement's letter says it never modifies, and four lines of it were edited on the operator's word, recorded in the pas… (`2026-10-07T15:39:29Z-a#1`)
- wrap-session/reconcile · 2026-10-07-a-send-ending-in-a-newline-is-confirmed · process — three security-plan proposals carried escalate severity; the pass did not halt on them because the operator's wrap directive said the amendment is not held and the founder's ruling was on r… (`2026-10-07T19:30:10Z-a#0`)
- phase/take-up · 2026-10-08-first-live-test-and-self-drive · process — a founder ruling relayed in an operator note arrived after the promote call had exited 0: founder not at the keyboard, the wheel takeover by a compositor-typed key, v1-33 re-worded on the t… (`2026-10-08T05:04:00Z-a#0`)
- phase/research · 2026-10-08-first-live-test-and-self-drive · process — the operator set the probe guard mid-phase and, after the block, directed the DPMS test and the step 0 fallback through one dialogue round (`2026-10-08T05:28:57Z-a#4`)
- implement/code · 2026-10-08-first-live-test-and-self-drive · process — the operator answered the step 0 question (inputs#I11): steps 1 to 3 now, stop before step 4, the lock question is with the founder; the plan orders step 0 first (`2026-10-08T06:36:48Z-a#1`)
- phase/research · 2026-10-08-first-live-test-and-self-drive · environment — the nested start woke the locked desktop and was followed by the lock-holding shell exiting and relaunching; surfaced as a Boundary widening fork, the founder accepted it for one start and… (`2026-10-08T07:04:06Z-a#1`)
- implement/code · 2026-10-08-first-live-test-and-self-drive · process — the operator ruled at the stop (inputs#I15): close the session, measure which reply is read as typing, keep the own compositor up with no window for a plan revision, a 3 hour bound; the pla… (`2026-10-08T07:41:13Z-a#2`)
- implement/fix-loop · 2026-10-08-first-live-test-and-self-drive · process — the operator added a measurement the plan did not hold: 23 reply-probe windows on the own compositor, the product build over a script child, no live start and no key (`2026-10-08T07:41:13Z-d#2`)
- wrap-session/reconcile · 2026-10-08-first-live-test-and-self-drive · process — the first citation sweep: on the operator's word a master sentence was reworded before the sweep's write, which reorders the step, so that a planted literal was not re-pointed (`2026-10-08T09:30:39Z-a#0`)

**Deferred facts (12):**

- wrap-session/route-resolve · (no chunk) · process — spec bodies naming WSL/pre-push stages left to :68's wrap (0-pending path applies no drift-derived amendment); host-win32.md routed to an /andromeda-setup-project re-run via the handoff (`2026-10-03T22:17:30Z-a#3`)
- implement/fix-loop · 2026-10-03-mutation-scoring-completion · environment — rm -r of the 21 leaked /tmp/cargo-mutants-ws-*.tmp dirs (325 MB) was refused by the permission layer; left for the operator (`2026-10-04T00:32:34Z-a#2`)
- implement/code · 2026-10-04-dialog-answers-by-dialog-id · environment — rm of fixtures/claude/2.1.288 and the stray recording home was refused by the permission layer; removal handed to the operator (`2026-10-04T14:49:33Z-a#3`)
- wrap-session/report · 2026-10-04-dialog-answers-by-dialog-id · resources — context at 86.7%: the operator directed P1 only, P2-P7 resume in a fresh window from the run dir's resume point (`2026-10-04T16:55:25Z-a#0`)
- implement/code · 2026-10-04-the-wheel · product-logic — source read predicts the Windows ^Z case red at the fake agent (its stdin is std's console read, which strips 0x1A); left for the windows-2025 CI measurement per the operator's measure-firs… (`2026-10-04T18:27:23Z-a#1`)
- wrap-session/gates · 2026-10-04-the-wheel · product-logic — light gate entry 16 red: the operator pass's 849588b added a std::env::var(CHILD_MODE) read in viola-pty's cfg(test) self-exec child; remedy (argv, guard unchanged) handed to an operator pa… (`2026-10-04T21:19:16Z-a#0`)
- implement/fix-loop · 2026-10-06-local-command-and-paste-framing-rows · process — gate entries 12-19 and 21 were not fired: each boots a home stamped over the committed 2.1.287 set, which lacks the framing variants; they wait on the founder's decision about a second round (`2026-10-06T20:41:30Z-a#1`)
- implement/fix-loop · 2026-10-06-local-command-and-paste-framing-rows · environment — a test home kept by this run's AGENT_RUN_KEEP_FAILED red reading stays under target/e2e-home: its rm was refused by the permission layer and was not retried; reported for the operator (`2026-10-06T21:30:37Z-a#2`)
- wrap-session/gates · 2026-10-06-local-command-and-paste-framing-rows · process — the truncated-profile red of pre-push is pinned as a WATCH on the next markerless entry, 0 green runs so far; its mechanism is a hypothesis (`2026-10-06T22:24:12Z-a#1`)
- phase/research · 2026-10-08-first-live-test-and-self-drive · environment — the fake-agent key probe could not move keyboard focus off the overseer window in four guarded runs, DPMS on and the window mapped in the last; on the operator's word it becomes implement s… (`2026-10-08T05:28:57Z-a#0`)
- implement/fix-loop · 2026-10-08-first-live-test-and-self-drive · product-logic — the fix of the stdin classifier is in src/run/wheel.rs, under the chunk's preservation guard: on the operator's word it is a plan revision through phase, in this chunk, with its own red-gre… (`2026-10-08T07:41:13Z-d#1`)
- phase/plan · 2026-10-08-first-live-test-and-self-drive · process — the founder's answer to the one question took about 32 minutes of a 3 hour bound on the standing compositor; the plan carries the bound as a stop rule (S9) (`2026-10-08T08:28:44Z-a#1`)

**Unresolved facts (9):**

- phase/take-up · 2026-10-02-epoch-2b-cleanup · process — the take-up evolve playbook was read in the same parallel batch as the master-record write, before the promote call ran, against the checkpoint reading order; the promote itself was mechani… (`2026-10-02T12:59:46Z-a#1`)
- implement/fix-loop · 2026-10-04-running-turn-refusal · process — plan entry 8 reads red on its green subject (pipefail + git grep exit 1); recorded for the wrap, plan not edited (`2026-10-04T22:18:54Z-a#1`)
- implement/fix-loop · 2026-10-05-real-cli-verify-probes · resources — the live cap (12) leaves 5 after entry 7; entry 7's re-run (3) plus entry 8 (3) is 13: STOP surfaced for the operator, entry 8 not fired (`2026-10-05T09:45:14Z-a#0`)
- implement/fix-loop · 2026-10-06-local-command-and-paste-framing-rows · process — the record round on 2.1.287 was red: Run B pasted the probe prompt and the long text and then nothing, because its settle after the long-paste turn returned rows without the input-box liter… (`2026-10-06T20:41:30Z-a#0`)
- implement/fix-loop · 2026-10-06-local-command-and-paste-framing-rows · environment — entries 16 and 18 read red in the first full block: every process-spawning test stalled about 18 s while another builder session's project linked 10.6 GB and 5.2 GB onto the same btrfs volu… (`2026-10-06T21:30:37Z-a#0`)
- wrap-session/gates · 2026-10-07-test-homes-off-the-contended-volume · process — this checkpoint's playbook was read while the light gate was still running, before the step it observes had completed, against the reading-order rule; the gate, the flip and the commit were… (`2026-10-07T08:40:12Z-a#0`)
- implement/code · 2026-10-08-first-live-test-and-self-drive · environment — step 0: the desktop session is locked (hyprctl locked true, the shell lock held since 2026-10-07T19:39:15Z), so no window takes keyboard focus; no lever was taken past the lock, no key type… (`2026-10-08T06:36:48Z-a#0`)
- implement/fix-loop · 2026-10-08-first-live-test-and-self-drive · environment — the live round and the live run were not fired: step 0 cannot pass while the session is locked, and the lock question is with the founder (`2026-10-08T06:44:48Z-a#0`)
- implement/code · 2026-10-08-first-live-test-and-self-drive · product-logic — start 6 lost the wheel 237 ms after the child started (wheel human/human-input, no key typed): foot answers the CLI's queries on stdin and seven reply shapes are outside the classifier's cl… (`2026-10-08T07:41:13Z-a#1`)
