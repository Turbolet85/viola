# Wrap journal — 2026-10-25-windows-endpoint-admission

Run dir `.andromeda/runs/2026-10-26T00-17-51-wrap/`. Operator's word at invocation (2026-10-26T00:17Z, this session):
«wrap 2026-10-25-windows-endpoint-admission whole; at P2 the operator's word is forced sample», with the directive
`~/dev/projects/additional/viola-overseer/wrap-windows-endpoint-directive.md` (snapped at P1).

Setup: 1 pending (this chunk), no prior report, base `b6740d59` (the parent of the pre-CI commit `e2bb59c8`); branch
`build/viola-0.1.0`, 0 ahead of its upstream.

## Long runs
- 2026-10-26T00:18Z · code-graph refresh (Setup 7), backgrounded → done.
- 2026-10-26T00:24Z · P2 stage1 Workflow: 60 agents (doc 7 · task 6 · catdet 47), the sample FORCED (`draw 3 of 10 ·
  FORCED by the operator`), script `prompts/stage1.workflow.js`. Nothing is written between its two snapshots.
  → returned 60, died 0 (wf_a917a754-6ef). `check`, first try:
  `doc: 7 agents · 6 pass · failed SPLIT 1` · `task: 6 agents · 6 pass` · `catdet: 47 agents · 44 pass · failed
  YAML_FORM 2, OWN_FILE 1` · `proposals: architecture 1 · security-plan 0 · design-system 0 · layout-templates 0 ·
  test-plan 2 · obs-plan 0 · a11y-plan 0` · `VERDICT: 56 of 60 pass`. The four failed agents are re-spawned once
  (stage1-r1).
- stage1-r1 (wf_c3bf31e4-b6c): `doc: 1 agents · 1 pass` · `catdet: 3 agents · 3 pass` · `proposals: architecture 1` ·
  `VERDICT: 4 of 4 pass`.
- classify (wf_09bd2b04-249, 62 agents, haiku · max): first try `classify: 62 agents · 62 pass · yes 36 of 745` ·
  `VERDICT: 62 of 62 pass`.
- pick: architecture yes 4 · security-plan yes 14 · design-system yes 0 · test-plan yes 13 · obs-plan yes 5 ·
  a11y-plan yes 0 → 4 strong readers.
- strong (wf_c0bfe24f-053): first try `strong: 4 agents · 3 pass · failed SPLIT 1` (fanout-test-plan--cascade);
  strong-r1 (wf_7aa991dd-962): `strong: 1 agents · 1 pass`.
- facts: architecture UNPLACED occupied-resources F1, cross-cutting-patterns F3 · security-plan UNPLACED
  dependency-security F4, F5 · test-plan UNPLACED test-strategy F9 · design-system, obs-plan, a11y-plan PASS → 4 passes.
- newdetail (wf_ff626ff8-c70): first try `newdetail: 4 agents · 3 pass · failed YAML_FORM 1` —
  `fanout-security-plan--cascade-new-dependency-security · [YAML_FORM] 6 line(s) off the list's shape`. The facts
  re-read: every doc `VERDICT: PASS` (architecture F1 no home, F3 placed by pass; security-plan F4, F5 placed by pass —
  off the failed file; test-plan F9 placed by pass).

## HALT — 2026-10-26T01:00Z, a hard P2 step failure (inputs#I7 item 3)
The one re-spawn the letter owes the failed new-detail pass cannot be rendered:
`fanout.py render --batch newdetail … --respawn fanout-security-plan--cascade-new-dependency-security --failed YAML_FORM`
→ `--respawn: not an agent of batch newdetail — fanout-security-plan--cascade-new-dependency-security` (usage refusal).
Cause, read in `fanout.py` (its newdetail agent list, lines 723–735): the batch's agents are the categories whose facts
read `unplaced` in `detail.fact_state`; that call counts F4 and F5 `placed by pass` from the very file `check` failed
on `YAML_FORM`, so the category leaves the batch and its re-spawn is refused. No retry around it was made (no hand
move-aside of the failed file, no re-render, no scripted fix): the wrap stops before Validate. Nothing in a master,
sidecar, leaf, route or matrix was written by this wrap; the report, the inputs I7–I8 and every P2 product stand in this
run dir.

## Resume — 2026-10-26T01:26Z (Setup 2a, the operator's option 2, inputs#I9)
- fanout v7 · dd446a75 (overseer1's f8827d5b, F350); `fanout.py status`: 6 batches, 0 unchecked. I9 snapped; the
  report's addendum (lines 291–295) inserted, `## New text, by line` now at 296.
- 2026-10-26T01:28Z · newdetail-r1 rendered (1 agent; the failed file and its accounting moved to `.raw-…` by the
  tool); its snapshot pair and one Workflow follow.
- newdetail-r1 (wf_e245f992-999): `newdetail: 1 agents · 1 pass`; security-plan facts `VERDICT: PASS`.
- compare: architecture `sampler: E carried 1 · the cascade lost 0 · beside a placed fact 1
  (stack-and-technologies#1.6)` · security-plan `sampler: E carried 3 · the cascade lost 0 · beside a placed fact 1
  (dependency-security#2.1)` · design-system `sampler: E carried 0 · the cascade lost 0` · test-plan `sampler: E carried
  8 · the cascade lost 1 (mocking-stubbing-discipline#2.4) · beside a placed fact 1 (test-strategy#3.1)` · obs-plan and
  a11y-plan `sampler: E carried 0 · the cascade lost 0`.
- newgroup (wf_3e5404df-780): `newgroup: 10 agents · 10 pass` first try; 22 verdicts, 21 distinct, 1 twin.
- Validate: `fanout-results.md` — 29 proposals → 17 apply · 2 apply-merged · 10 reject · 0 escalate; no halt.
- apply (wf_09ddc859-3b7): `apply: 3 agents · 3 pass` first try; 17 `applied` verdicts, 2 `folded into`; `cascade.py
  replace` wrote 17 blocks, every file read back. Read-backs: `registry.py check` 0 defects; `detail.py check`
  architecture / security-plan / test-plan `VERDICT: 0 defects`; `classify.py check` PASS on all ten edited sections of
  security-plan and test-plan; on the key file `crate-dependency-direction.md`: `VERDICT: FAIL groups` — line 12 a
  malformed label (`<!-- detail: dependency · the \`viola\` root bin -->`: no such kind, a subject outside the
  architecture grammar), line 16 a closer with no open group.
- apply-r1 (wf_87074b60-368, `--failed RULED`, the row: replace line 12 with `<!-- detail: values ·
  infra-crate-dependency-direction -->` and append the subject to `architecture.md:459`): `apply: 1 agents · 0 pass ·
  failed OWN_FILE 1, EDITS_PLAN 1` — `cascade.py replace` reads a label line `preserved`, and `architecture.md` is no
  target of that writer.

## HALT — 2026-10-26T01:39Z, Apply fails hard (inputs#I7 item 3)
The read-back of the architecture key file fails and its one re-spawn fails: no apply writer can correct a label line
in place (`replace` preserves comment lines; a writer's targets are its own files). No hand fix was made. Written and
standing in the tree (uncommitted): the 17 blocks across `.andromeda/test-plan.md`, seven test-plan category files, two
security-plan category files and the architecture key file (its 6 added lines, the bad label among them). Not done:
the step-2 sweep, the sidecar entries (`--batch entry`), the leaves, the tally.

## Resume 2 — 2026-10-26T01:41Z (option 1, inputs#I10 the ruling F351, inputs#I11 the operator's answer)
Batch notes — operator ruling (overseer1, F351): the apply writer's label fixed by the orchestrator; no writer can
correct a label it wrote. Exactly two writes:
1. the Edit tool on `.andromeda/registries/contracts/architecture/crate-dependency-direction.md:12`, the label only:
   `<!-- detail: dependency · the \`viola\` root bin -->` → `<!-- detail: values · infra-crate-dependency-direction -->`;
2. `cascade.py replace --edits replace-1.md` (dry run first, then the write, read back): `.andromeda/architecture.md:459`,
   the subject line, `· infra-crate-dependency-direction` appended.
Read-backs: `classify.py check` on the key file `VERDICT: PASS` (groups 1 · subjects used 1) · `detail.py check
--master architecture` `VERDICT: 0 defects` · `registry.py check` `0 defect(s) — clean`. The wrap continues at the
step-2 sweep.
- I10 (the ruling) and I11 (the answer) snapped; the report's second addendum inserted (`## New text, by line` at 303).
- step-2 sweep (`cascade-dispositions.md`): `total (6 patterns) · 36 rows over 15 files` · `per class · new 14/7 ·
  standing 18/9 · leaf 4/3`; every row dispositioned.
- entry (wf_875c589e-f28): `entry: 3 agents · 3 pass` first try; one entry's Kept line re-worded on main's ruling,
  entry-r1 (wf_2a832def-36b) `entry: 1 agents · 1 pass`; `sidecar.py check` on-form ×8; the three sidecars appended
  (architecture +9, security-plan +18, test-plan +45 lines), each file's last line its last entry's `**Ref:**`.
- leaf (wf_ab490627-6c6): `leaf: 9 agents · 9 pass` first try; 6 findings → 3 applied, 3 declined.
- tally: architecture `entities 88 · yes 4 · carried 1 · false-yes 0.750 · no-fit 0 · facts 3 · placed 2 · by pass 1 ·
  no home 1 · beside fact 1` · security-plan `entities 65 · yes 14 · carried 3 · false-yes 0.786 · facts 5 · placed 5 ·
  by pass 2 · beside fact 1` · test-plan `entities 138 · yes 13 · carried 4 · false-yes 0.692 · facts 10 · placed 10 · by
  pass 1 · beside fact 1` · design-system `entities 181 · yes 0` · obs-plan `entities 258 · yes 5 · carried 0 ·
  false-yes 1.000` · a11y-plan `entities 15 · yes 0`; every one `sampled yes · no-fit 0`.
- P2: 17 amendments applied (+2 main's writes on F351) · 0 escalations · 0 rejected for a source the report does not
  carry · citations 0 re-pointed · 0 changed · 0 stretched.
- P3: T2 1 (testing.md 2026-09-27 extended) · T3 1 · filtered 1 (`curation.md`). P4: done, rust 6605n/36674e.
  P5: the tokio entry inserted at working-route 171; the macOS readiness WATCH retired (3 green runs). P6: state.yaml
  (session 91), the handoff.
- 2026-10-26T01:59Z · P7.1 the light gate, the whole block as /implement ran it (operator entries re-verified from
  `evidence/operator-pass.md`), backgrounded; pre-push ~100 s the longest.
