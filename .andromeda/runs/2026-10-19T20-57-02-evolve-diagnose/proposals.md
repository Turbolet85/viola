# Evolve Diagnosis — viola-0.1.0 · Epoch 5 — Driver surface · 2026-10-19T20:57:02Z

## Mechanism health
- Records: 478 (288 step / 190 friction) over 18 chunks, plus 1 chunk-less group (41 new-session orientations, one operator-requested adaptation wrap). Retractions in range: none. Whole-ledger pre-pass: 2 retracted, 3 clause-retracted (none in this epoch), 0 retraction-of-retraction, 2 unresolvable, reported verbatim below.
- Coverage vs expected (phase 5 · implement 3 · wrap 5): all 18 chunks complete, no checkpoint gap. Above expected, all explained by their own records: resume-and-compact (phase 8: a plan revision; implement 5: a re-entry), mcp (implement 4: a second smoke), cli-output, compact and plugin-precedence (phase 6 each: P4 paused for the founder's word and resumed). The chunk-less wrap carries report · reconcile · curation · route-resolve (an adaptation wrap, legitimate).
- Unparseable lines: 0 (whole ledger, 2251 lines). Malformed `ts`: 0. `id` fill: 478/478.
- Two friction records carry no `step` (`2026-10-13T17:51:07Z-b` implement, `2026-10-13T18:17:23Z-b` wrap-session). They are grouped as recorded; see the appendix for the one this moves to the threshold.
- Untyped rate (untyped / friction per step): implement/code 5/23 · implement/fix-loop 1/33 · implement/smoke 1/1 · implement/None 0/1 · new-session/orientation 0/6 · phase/distill 1/6 · phase/plan 1/7 · phase/research 2/6 · phase/take-up 1/3 · phase/validate 2/27 · wrap-session/curation 0/14 · wrap-session/gates 2/6 · wrap-session/reconcile 8/40 · wrap-session/report 5/12 · wrap-session/route-resolve 1/4 · wrap-session/None 0/1. Overall 30/190. Several untyped records fit an existing type (`contract.narrow-basis-claim`, `input.research-files-wrong`), named in the appendix.
- Problem-fact fill: 61/288 step records carry a fact (74 facts: workaround 43 · overridden 13 · deferred 9 · removed-cause 7 · unresolved 2).
- Non-ok outcomes: reconcile `halted-resolved` 11 · fix-loop `ok-degraded` 6 · plan `soft-exit` 3 · gates `halted-resolved` 2, `ok-degraded` 2 · route-resolve `halted-resolved` 2 · code `ok-degraded` 1 · smoke `ok-degraded` 1.
- guard-refused 77 over 288 step records · exact-hit 35 over 19 curation records. No signal string is off-form.
- Calibration boundaries in range: implement-outcome's operator-pass reading (2026-10-13, the epoch's first day; X2 reads with it). The `input.*` same-artifact grade cap (2026-10-19) means most of this epoch predates it, and an `ok` beside an `input.*` friction is that era's grade. Orientation's widened ladder gloss (2026-10-13). `guard-refused` / `exact-hit` (2026-10-09) cover the whole epoch.
- Unresolvable retractions (whole ledger, Epoch 2, for the founder's manual discount): `2026-09-27T07:06:36Z-b` unknown-id-or-scope: {"id": null, "note": "discount the signal carries-pinned-7 and the note '7 CARRYs' in step record 2026-09-27T06:40:33Z-a: 8 CARRYs were pinned (:40 x4, :43, :65; `2026-09-27T12:58:32Z-a` unknown-id-or-scope: {"id": null, "note": "discount the ts of step records 2026-09-27T13:02:00Z-a and 2026-09-27T13:10:00Z-a the same way; their content stands"}

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 23 cases · weight 44 · rate 23/19 step runs
**Pattern:** Validate's mechanical checks caught 23 plan or scope defects in 15 of 18 chunks before the review card. Ten of the 23 are one shape: check 10 (`inputs.py verify`) read `UNPARSED` because the take-up fold copied route CARRY freight that cites another chunk's snapshot as a bare `inputs#I<n>`, which reads as this chunk's own (interrupted-verify, paste-newline, owed-ledger, remove-the-browser-leg, unwrapped, end-to-end, cli-machine, cli-output, mcp, optional-plugin). Five more are gate atoms authored at P4 that could never pass or never fail on a right tree (the doctest suite object's `passed 0`, a word the right implementation also writes, an absent path that makes tokei exit, an unpinned advisory database read as seen).

**Evidence:** ALL 23 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-epoch-4-cleanup | check 9 read REQUIRED-RESOLUTION: research's runner-only Windows-baseline bullet stood beside a 'Platform issues consulted: none' slot; two web searches were run and the slot filled (no runner-images source found, the ci#37214281447 measurement stands) | extra_reads 2 | viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/research.md · `2026-10-13T08:56:20Z-b` |
| 2026-10-13-epoch-4-cleanup | check 4 (9): entry 10 (the tokei sizes read) baselined red because tokei exits with an error and no JSON on a path that does not exist yet (the split dirs), so its red on the untouched tree is absent input, not the subject; a must-fail and a must-pass control in a scratch clone were run to show it reads the subject | extra_reads 1 | .andromeda/runs/2026-10-13T07-07-28-phase/p5-controls.md · `2026-10-13T08:56:20Z-c` |
| 2026-10-13-epoch-4-survivors-ii | check 4 (9): the cargo deny check entry, copied byte-equal from the previous plan, reads an advisory database no commit pins and so is new every chunk; the baseline listing read it seen green, and P5 set new = true and baselined it by the letter's rule | — | — · `2026-10-13T17:45:43Z-b` |
| 2026-10-13-interrupted-verify-cleanup | check 10 (inputs.py verify) read UNPARSED inputs#I4 and inputs#I5 at scope.md:25 and :74 (and the same I4 at plan.md:364, which verify did not list): another chunk's snapshot written in this chunk's citation form while folding CARRY text; re-worded to those chunks' inputs/ file paths and the set re-run | retries 1 | — · `2026-10-13T18:47:17Z-b` |
| 2026-10-13-paste-newline-ledger-row | check 10 (inputs.py verify) printed UNPARSED for inputs#I6 and inputs#I8 in scope.md: two other chunks' snapshot ids copied verbatim from the folded CARRY freight at P1; reworded to those chunks' inputs/ paths | iterations 1 | .andromeda/runs/2026-10-13T20-31-55-phase/ · `2026-10-13T21:20:33Z-b` |
| 2026-10-13-paste-newline-ledger-row | check 11 (a) host reach: the live entry's stop and census patterns, copied from the 2026-10-06 precedent round, matched any .viola-verify-* or viola-verify-* cwd (an older run's leftover included); narrowed to the repo root and /tmp before the prompt | iterations 1 | .andromeda/runs/2026-10-13T20-31-55-phase/p5-card-1.md · `2026-10-13T21:20:33Z-c` |
| 2026-10-13-owed-ledger-rows | check 10 (inputs.py verify) read UNPARSED for inputs#I2, I6 and I8 in scope.md: P1 copied earlier chunks' bare inputs#I citations from the CARRY text, which read as this chunk's own | retries 3 | .andromeda/runs/2026-10-13T22-31-54-phase · `2026-10-13T23:04:13Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows | check 11 (a): both live stop forms carried a $TMPDIR/viola-verify-* arm, a kill by name over a dir pattern every viola verify on the host shares (another tree's bridge included); arm dropped, the round's pid census left to end a Run A survivor | iterations 1 | .andromeda/runs/2026-10-14T07-11-44-phase/p5-card-1.md · `2026-10-16T06:45:00Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows | check 12 (a): step 12 read the integration entry's timings before and after with no anchor, while the entry reads the fixture step 13's record round lands; re-anchored to the P5 baseline and a firing after step 13 | iterations 1 | .andromeda/runs/2026-10-14T07-11-44-phase/p5-card-1.md · `2026-10-16T06:45:00Z-c` |
| 2026-10-14-resume-and-compact-ledger-rows | check 8 read: step 5 told implement to make read_captures pub(super), but typed is verify's child module and already reaches the private fn; the visibility change was dropped | iterations 1 | .andromeda/runs/2026-10-14T07-11-44-phase/p5-card-1.md · `2026-10-16T06:45:00Z-d` |
| 2026-10-16-remove-the-browser-leg | check 10 (inputs.py verify) listed scope.md:11 citing inputs#I1 where the text meant the adaptation wrap's I1, not this chunk's P4 snapshot; reworded to that wrap's path before the review | iterations 1 | .andromeda/runs/2026-10-16T15-37-36-phase/inputs-2026-10-16-remove-the-browser-leg.json · `2026-10-16T15:59:57Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board | check 4 (9): entry 6's atom lacks "passed":0, could never pass because the run --unit document always carries the doctest suite object with passed 0; found by reading the baseline log, re-spelt as lacks "suite":"nextest-unit","passed":0, | iterations 1 | .andromeda/runs/2026-10-16T18-50-06-phase/p5-controls.md · `2026-10-16T20:05:29Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board | check 10: inputs.py verify read UNPARSED inputs#I5 at scope.md:126, a citation of the revive chunk's snapshot written in this chunk's own form at P1; re-pointed to that chunk's inputs/I5-relay-1.md.txt after reading it | extra_reads 1 | — · `2026-10-16T20:05:29Z-c` |
| 2026-10-16-unwrapped-sessions-on-the-board | check 11 (a): plan step 13 let the fake agent skip dead registry entries without saying how liveness is read, leaving kill(pid, 0) open; the step now reads the process table and signals nothing | iterations 1 | — · `2026-10-16T20:05:29Z-d` |
| 2026-10-17-end-to-end-path-binary-and-harness-legs | check 10 (inputs.py verify) read UNPARSED inputs#I2 at scope.md:142: the folded CARRY 6 text cited the previous chunk's own inputs#I1 / inputs#I2 as bare ids, which read as this chunk's; re-pointed to that chunk's inputs/ file paths | retries 1 | .andromeda/runs/2026-10-17T00-19-29-phase/ (inputs trail) · `2026-10-17T00:48:14Z-b` |
| 2026-10-17-cli-machine-contract | check 10 (inputs.py verify) printed UNPARSED for inputs#I3 and inputs#I8 in scope.md: verbatim CARRY freight citing two earlier chunks' snapshots in the this-chunk form; the take-up fold copied them as they stood | retries 1, extra_reads 1 | .andromeda/runs/2026-10-17T02-25-51-phase/ · `2026-10-17T02:49:06Z-b` |
| 2026-10-17-cli-output-discipline | check 3 fired: the plan read NO_COLOR, TERM and COLORTERM without naming the architecture env-var registries (Occupied Resources Environment variables, Naming :104, Conventions :157) in its expected amendments; one plan edit added them and the whole set re-ran | iterations 1 | — · `2026-10-17T07:42:49Z-b` |
| 2026-10-17-cli-output-discipline | check 10 fired: scope.md carried four bare inputs#I3/I7/I8/I9 citations copied verbatim from the CARRY freight at P1, which name chunk 2026-10-11-the-board-viola-list's snapshots; re-spelt by that chunk's inputs paths and inputs.py verify re-run | iterations 1 | — · `2026-10-17T07:42:49Z-c` |
| 2026-10-17-mcp-server-for-drivers | check 10 (inputs.py verify) printed 4 UNPARSED citations: scope.md's CARRY 5 fold cited the board chunk's snapshots as inputs#I3/I7/I8/I9, this chunk's own form; re-worded to that chunk's inputs/ paths | iterations 1 | — · `2026-10-17T11:31:27Z-b` |
| 2026-10-17-mcp-server-for-drivers | check 6 (size) warned: 461 lines of plan prose against ~400 (642 total, fence 181), under the >50% band; check 4 (9) also set new on 3 NOT SEEN entries (cargo deny check, the sole-root loop, deny-probes) the P4 author had not marked | iterations 1 | — · `2026-10-17T11:31:27Z-c` |
| 2026-10-17-optional-plugin-install | check 10 (inputs.py verify) printed UNPARSED for inputs#I3 (scope.md:26) and inputs#I4 (scope.md:30): P1's scope cited other chunks' snapshots in this chunk's form; re-worded to those chunks' inputs/ file paths and re-verified 0 unparsed | iterations 1 | .andromeda/runs/2026-10-17T15-14-51-phase/inputs-2026-10-17-optional-plugin-install.json · `2026-10-17T18:06:04Z-b` |
| 2026-10-18-compact-post-condition-ledger-row | check 4 (9)'s baseline reads corrected two P4-authored entries: the literal probe counted the word twenty-three and version_gate.rs, which step 9 writes correctly, so it could never read 0 after a right implementation; the new-cases filter carried a contains "failed":0 atom the doctest suite always satisfies; both re-spelt and the whole set re-run before the card | iterations 1 | .andromeda/runs/2026-10-18T03-26-43-phase/p5-controls.md · `2026-10-18T20:51:05Z-b` |
| 2026-10-19-overseer-switch-to-viola | check 4 (6) and check 8 read as REQUIRED-RESOLUTION: a security criterion cited a step-10 read the step did not make, and an a11y criterion rested on a live terminal read no step made; step 10 gained the command-path count and the a11y criterion was re-worded to the existing passthrough cases, then the whole set re-ran | iterations 1 | — · `2026-10-19T10:52:14Z-b` |

**Proposal:** The check works; the repair is repeated. For the ten citation repairs the cause sits upstream of validate: the CARRY text on the route (written by wrap's route-resolve) carries a chunk-relative citation into a chunk where it means something else. One direction is that the route writer spells a carried citation by its chunk-qualified form (the `inputs/` file path), or that phase take-up's fold re-spells bare `inputs#I` ids on copy, which would remove the ten repairs at their source. For the five atom repairs, one direction is a P4 authoring note in the plan template on what `run --unit` always prints (the doctest suite object) and on the word a right tree also writes.

### P2 — wrap-session/reconcile · `contract.in-pass-correction` — 12 cases · weight 31 · rate 12/19 step runs
**Pattern:** Reconcile corrected 12 of its own first writes before the commit in 11 wraps; every one was caught in-pass, by a re-read or a recount, or by a tool (`registry.py check`, `sidecar.py check`, `classify.py check`, the step-2 sweep). Five of the 12 are a hand-typed tally or count that did not match its rows (`two` → three, 28 → 29 dispositions, 44 → 47 amendments, 5 → 4 escalations, `no change 12` → 11).

**Evidence:** ALL 12 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| — | three first writes corrected before commit: the new playbook rule said 'two' card-confirm entries (three; caught by re-read); security-plan-entry-1 carried a Supersedes of a partly standing entry (caught by re-reading sidecar-contract, moved into Change); the Leaves table carried a backticked pipe that split a cell (caught by re-read) | iterations 3 | — · `2026-10-13T07:00:53Z-c` |
| 2026-10-13-epoch-4-cleanup | the test-plan §9 Lint row addition first said a chunk whose cfg(windows) code the host does not build carries the Windows-target clippy (a prescription past the measurement); a re-read before the sweep rewrote it to: a chunk plan may list it, first listed by this chunk | retries 1 | .andromeda/runs/2026-10-13T10-47-05-wrap/fanout-results.md · `2026-10-13T11:04:28Z-b` |
| 2026-10-13-paste-newline-ledger-row | architecture.md:49 first write changed the earlier chunk's measured clause 'every text was under 100 bytes' to 'under 120 bytes' as if the old measurement had moved; caught on re-reading the edit and rewritten to state the union (the probe's 113-byte text included) | iterations 1 | — · `2026-10-13T22:12:27Z-b` |
| 2026-10-13-owed-ledger-rows | fanout-results.md's architecture section was first written with 28 numbered dispositions and a skipped 19; a count of the file's `- detector:` items (29) caught it and the section was rewritten in the file's order before the halt | iterations 1 | — · `2026-10-14T07:02:56Z-d` |
| 2026-10-16-remove-the-browser-leg | 5 first writes wrong, caught before commit: obs-plan proposals 16/17 (§1 run\|hook\|mcp\|ui at L138, L209) and test-plan proposal 9 (§7 cookie file, L972) dispositioned apply but not edited — caught by the step-2 sweep; a deleted Log format block left its index label — caught by registry.py check STALE; the new corr rule cited 'obs-plan D-02 … D-05' — caught by re-reading the copy | iterations 2 | .andromeda/runs/2026-10-16T17-15-05-wrap/cascade-dispositions.md · `2026-10-16T18:40:25Z-c` |
| 2026-10-17-cli-machine-contract | 3 first writes corrected before commit: security-plan.md:203's home resolution first sat under the list-arguments label (re-read; relabelled Home resolution); obs-plan-entry.md had Kept before Why (sidecar.py check OFF-FORM; reordered); fanout-results.md's summary said 44 amendments (re-count; 47) | iterations 3 | .andromeda/runs/2026-10-17T03-50-43-wrap/ · `2026-10-17T04:21:36Z-e` |
| 2026-10-17-cli-output-discipline | architecture.md §Conventions → Environment variables: the apply added 'A third ratified exception' after the standing 'The two ratified exceptions are the test seams', a self-contradiction; the cascade sweep's edited row (env-exhaustive, architecture.md:157) caught it, re-worded 'Two of the ratified exceptions are' | retries 1 | .andromeda/runs/2026-10-17T08-28-14-wrap/cascade-dispositions.md · `2026-10-17T08:47:35Z-b` |
| 2026-10-17-mcp-server-for-drivers | two first writes corrected before the commit: the viola-state liveness line inserted inside the phrase 'the events / reader' in project-directory-structure.md (caught by a re-read), and fanout-results.md's tally reading 5 escalations and 5 raised for 4 and 4 (caught by a re-read) | retries 2 | .andromeda/registries/contracts/architecture/project-directory-structure.md · `2026-10-17T14:55:18Z-d` |
| 2026-10-17-optional-plugin-install | the project-directory-structure key edit first placed the marketplace module between the statusline module and its own parenthetical; the read-back caught it and the line was reordered before the sweep | retries 1 | — · `2026-10-18T03:06:09Z-c` |
| 2026-10-18-plugin-precedence-live-readings | answer-card.md first listed the inputs#I4 rig place as a card item; the playbook rule's note (a word that answered no P4 fork is no card item) caught it on a read before the halt, and I4 became its own plain question | iterations 1 | — · `2026-10-19T07:18:49Z-b` |
| 2026-10-18-plugin-precedence-live-readings | cascade-dispositions.md first wrote 'no change 12'; a recount of the 22 rows by class caught it (11) before any sidecar entry landed | iterations 1 | — · `2026-10-19T07:18:49Z-c` |
| 2026-10-19-dialog-hold-for-the-driver | two lines this pass wrote failed classify.py's provenance arm on first write: architecture.md:77 said the ceiling stays PROVISIONAL 'until then' (rewritten to 'is PROVISIONAL'), and otel-sdk-init.md:17's rewritten clause kept the standing stamp '(chunk 2026-10-10-budget-governor)' (dropped); both caught by classify.py check --mode standing before the sidecar entries | retries 2 | — · `2026-10-19T09:53:39Z-b` |

**Proposal:** The catch is working; the cost is the first write. One direction is that the tallies reconcile writes into `fanout-results.md` / `cascade-dispositions.md` / the answer card are printed by the tool that holds the rows (a count line from `cascade.py` or `sidecar.py`) rather than typed, which would take the five tally cases off the first write. The other seven are wording placements a re-read caught; no common cause is evident there.

### P3 — any step (universal / recall: grouped by type alone) · `recall.corpus-recurrence` — 11 cases · weight 21
**Pattern:** A curated rule stood correct and the same failure recurred 11 times in 8 chunks. Four are one rule: testing.md 2026-10-07 (list a moved bound's cases by the function that decides it), recurring in paste-newline, owed-ledger, resume-and-compact and unwrapped (and, after its 2026-10-16 extension, again in compact as P6's `dialogless_home` case). Three are host-linux.md (the printf transport, a `cd X 2>/dev/null;` prefix, a `viola` census matching the overseer's bridge); one each are testing.md timing windows, the remove-the-guard sed anchor, gate.py hygiene's Windows-path form and events.md's snapshot-before-line order.

**Evidence:** ALL 11 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-epoch-4-cleanup (wrap-session/curation) | the phase run quoted the CI runner's scratch as a Windows drive path in two run-dir notes and the operator pass's hygiene read refused both; the Tier 3 entry on gate.py hygiene already says the read takes a quoted Windows drive path for a host path (its 2026-10-06 extension) | retries 1 | viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/evidence/operator.md · `2026-10-13T11:05:16Z-b` |
| 2026-10-13-paste-newline-ledger-row (wrap-session/curation) | testing.md 2026-10-07 (a moved bound: list the cases by the function the bound decides, never by its literals) was already curated; research swept the row count's literals anyway and five readers of LedgerRow::ALL and the fixture set read red at the unit, integration and pre-push gates | iterations 2 | — · `2026-10-13T22:13:47Z-b` |
| 2026-10-13-owed-ledger-rows (wrap-session/curation) | testing.md 2026-10-07 (list a moved bound's cases by the function it decides) did not prevent research missing merge_stamp's literal row-set envelope oracle when LedgerRow::ALL grew; the same entry also recurred in the 2026-10-13 paste-newline chunk | iterations 1 | viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/report.md (Deviations 3) · `2026-10-14T07:04:34Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows (wrap-session/curation) | testing.md 2026-10-07 (a moved bound: list the cases by the function it decides) recurred a third time: Run E moved verify's run count and its readers were swept by literal, ten readers outside the plan's list, one (an integer-built summary line) reaching pre-push and a plan revision | retries 1 | viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/report.md · `2026-10-16T14:44:40Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board (wrap-session/curation) | a moved count again escaped the chunk's planned test selection (the integration filter missed cli_answer, whose shared helper pinned the summary) despite testing.md's 2026-10-07 moved-bound entry; landed as that entry's extension, its fourth recurrence | extra_reads 1 | .andromeda/runs/2026-10-16T22-30-02-wrap/curation.md · `2026-10-16T23:39:19Z-b` |
| 2026-10-17-cli-machine-contract (wrap-session/curation) | the plan's Part A step 2 sampled a timing window over 20 firings although testing.md 2026-09-27 says a timing window is forced, never sampled; the forced unit case found it | iterations 1 | .andromeda/runs/2026-10-17T03-50-43-wrap/curation.md · `2026-10-17T04:24:20Z-c` |
| 2026-10-17-cli-machine-contract (wrap-session/curation) | a remove-the-guard sed anchored on pre-rustfmt text matched nothing and its green proved nothing, past testing.md 2026-09-25's extension | retries 1 | viola-0.1.0/chunks/2026-10-17-cli-machine-contract/evidence/guards.md · `2026-10-17T04:24:20Z-d` |
| 2026-10-17-cli-machine-contract (wrap-session/curation) | a TOML pattern written through printf landed a doubled backslash, past host-linux.md Transports (never build a document through shell quoting) and its 2026-09-28 addition; fixed through Edit | retries 1 | viola-0.1.0/chunks/2026-10-17-cli-machine-contract/report.md · `2026-10-17T04:24:20Z-e` |
| 2026-10-17-optional-plugin-install (wrap-session/curation) | host-linux.md Paths (anchor cross-call paths absolutely, cd only in a subshell) stood correct, and a cd X 2>/dev/null; prefix still moved the session cwd for one call this window | retries 1 | — · `2026-10-18T03:06:56Z-b` |
| 2026-10-19-dialog-hold-for-the-driver (wrap-session/curation) | host-linux.md 2026-10-07 (a process named viola is not this repository's by its name; pgrep -x reads the overseer bridge) stood correct, and the implement run's first census by gate.py census --args target/debug/viola listed the bridge's 11 processes anyway; extended with the census form | extra_reads 1 | — · `2026-10-19T09:54:55Z-b` |
| 2026-10-19-dialog-hold-for-the-driver (wrap-session/curation) | events.md 2026-10-16 (a snapshot field a reader expects once a line is visible is written before the line) stood correct, and DialogSlot::register wrote pending_dialog after the dialog line; met as a macOS-only CI red at 3def5b5b, fixed at d146c50c | iterations 1 | ci#39226239371 · `2026-10-19T09:54:55Z-c` |

**Proposal:** A Tier 2 rule that recurs four times after curation is not reaching the step that commits the error, which is research's sweep, not a reader of testing.md at write time. One direction is to carry the moved-bound rule as a mechanism rather than as more text: a research-letter step that lists a moved bound's readers through the code graph (`tree.db` callers of the deciding symbol, e.g. `LedgerRow::ALL` / `relied()`), and on the project side a test helper that derives its pass counts from the deciding function rather than a literal. See chain X1 and P6. The host-linux recurrences overlap level candidate L3.

### P4 — implement/code · `input.plan-step-ambiguous` — 12 cases · weight 17 · rate 12/18 step runs
**Pattern:** Implement met 12 plan steps in 9 chunks it could not follow as written (13 with a step-less record, see Mechanism health). Six left a named choice unbound (a case count, a drop rule's reading, which name, a verdict word, a slice's rows, where shared writers live); six prescribed a shape that would not work (the snapshot written after the line a test waits on, the agents read before `/clear`, a home canonicalised before it exists, rstest rows over a sequence, an rstest in a crate with no rstest dev-dependency under a no-manifest guard, an INTEGRATION layer with no matching binary).

**Evidence:** ALL 12 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-epoch-4-cleanup | step 4 did not place the human tests' shared writers (Recorder, Broken, one_write), and step 5 listed a PasteFn re-export no caller outside the module uses (an unused pub(crate) use fails -D warnings); settled from the scratch.rs super::super::tests precedent and by leaving PasteFn un-re-exported, both reported | extra_reads 2 | — · `2026-10-13T09:05:51Z-b` |
| 2026-10-13-owed-ledger-rows | plan step 2 fixes the row-case count at nine (the gate atom passed 9) while naming no broken post-condition for the override row; a ninth case was chosen and is surfaced in the report | reformulations 1 | — · `2026-10-13T23:18:46Z-d` |
| 2026-10-16-unwrapped-sessions-on-the-board | plan step 4 said the snapshot write lands after the session-start line is appended; every test helper that waits on that line then read the snapshot mid-rewrite (cli_list path1 and e5 red) | iterations 1 | — · `2026-10-16T20:44:30Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board | plan step 11 pinned the agents read before the /clear paste, so a run that stops pasting early failed the agents row as well; the read moved to right after the first turn settles | iterations 1 | — · `2026-10-16T20:44:30Z-c` |
| 2026-10-17-end-to-end-path-binary-and-harness-legs | the re-cut INTEGRATION layer would fail to parse in run.rs's throwaway mini workspace (no E2E-prefixed binary): measured exit 94 'operator didn't match any binary names' on a scratch crate, fixed by planting tests/path_it.rs in the two mini cases whose integration layer runs | extra_reads 1 | — · `2026-10-17T01:32:56Z-b` |
| 2026-10-17-end-to-end-path-binary-and-harness-legs | step 7's drop rule ('cleanup keeping homes under AGENT_RUN_KEEP_HOMES=1, then removes the home parent unless KEEP_FAILED and panicking') reads two ways; implemented as keep = KEEP_HOMES or (KEEP_FAILED and panicking), surfaced in the report | — | — · `2026-10-17T01:32:56Z-c` |
| 2026-10-17-cli-machine-contract | plan step 12 says revive --list opens cli-<name>.ndjson under a parsed VIOLA_NAME without saying whether <name> is the caller's or the revived instance's; settled by obs-plan §4's cli row (list and verify log under the caller's name) and pinned by a case using VIOLA_NAME=driver | extra_reads 1 | — · `2026-10-17T03:15:06Z-c` |
| 2026-10-17-cli-machine-contract | plan step 16 asks one rstest row per outcome on a stamped home; outcomes in sequence over one live wrapper (pause then refused send then release, then the stopped instance) cannot be independent rstest rows, so they became a labelled case table per world, each assertion naming its label | — | — · `2026-10-17T03:15:06Z-d` |
| 2026-10-17-optional-plugin-install | plan step 2 names the home argument as the resolved canonical home, but resolved_home canonicalises before the home exists, so a fresh home under the linked target/e2e-home would be spelt through the link and miss the resolved(home) oracle; settled by re-canonicalising after the dirs are created (run's own pattern) and surfaced for the P4 report | extra_reads 1 | — · `2026-10-17T18:14:26Z-b` |
| 2026-10-18-compact-post-condition-ledger-row | plan step 10 'the statusline slice LEDGER_ROWS[18..] reads the same three rows only if it is bounded to them — bound it so' does not say which three rows the slice read (it read five on Unix, two on Windows) | reformulations 1 | — · `2026-10-18T21:22:56Z-b` |
| 2026-10-18-plugin-precedence-live-readings | plan step 1's key-rule verdicts (selected · pointer-above/below · unreached · other-modal · absent) name no word for a trust row with zero or several pointer rows; the rig reads it `unselected`, no key and red, per inputs#I2 F1 'anything else no key and red' — surfaced in the report | — | — · `2026-10-19T05:27:21Z-b` |
| 2026-10-19-dialog-hold-for-the-driver | plan step 1 asks for an rstest in crates/viola-core, which has no rstest dev-dependency while the plan's preservation guard forbids a manifest edit; written as a literal table test under the planned name | reformulations 1 | — · `2026-10-19T08:27:44Z-c` |

**Proposal:** Each was settled and surfaced; none halted. One direction for the unbound half is a validate check over each step for a choice the step names but does not bind (a count, a word, a reading). The unworkable half shows no single check; two of its six (snapshot-after-line, agents-read placement) recur an events.md rule (P3) and one (rstest vs the manifest guard) is the preservation guard of L1.

### P5 — any step (universal / recall: grouped by type alone) · `contract.premise-falsified` — 8 cases · weight 20 · 3 with halt
**Pattern:** Verification falsified a premise eight times in five chunks, three with a halt. Four are live-rig premises about undocumented claude CLI behaviour around a rig `proj/` inside the repository (`--scope local` writes the work tree root's settings, a modal before SessionStart with no trust record, a trust row shown unselected, an external-imports modal from the repository CLAUDE.md's import). The others: `/compact`'s post-condition outlasting the 10 s window, a fixture session id off the session-id shape, `cargo tree -i` exiting 0 once the crate is in the lockfile, and `~/.viola` reading 755 after a verify.

**Evidence:** ALL 8 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-14-resume-and-compact-ledger-rows (phase/research) | the folded CARRY priced /compact as a ledger row plus one Run B paste; research found the send's single 10 s confirm window ends before the measured 18.8 s compaction post-condition, so a window of its own is also needed | — | viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/research.md · `2026-10-14T07:22:25Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows (implement/code) | plan step touchpoints said tests/support/verify.rs changes only if the stamped-home fixture names the run count or dirs; its synthetic session id s-verify-1 is off the session-id shape, so Run E would never start under it and the id had to take the shape | iterations 1 | viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/plan.md · `2026-10-16T06:59:33Z-b` |
| 2026-10-17-mcp-server-for-drivers (implement/code) | plan step 3 / research: cargo tree -p viola-channel -i tokio exits 101 'did not match'; once rmcp/tokio enter Cargo.lock it exits 0 'nothing to print', so the script reads presence by a 'tokio v' line and accepts both absent forms | extra_reads 1 | viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/instrument-controls.md · `2026-10-17T11:55:13Z-b` |
| 2026-10-17-optional-plugin-install (implement/fix-loop) | plan step 7 premised that claude plugin marketplace add --scope local run from target/plugin-live-*/proj/ writes proj/.claude/settings.local.json; on 2.1.287 it wrote the git work tree root's .claude/settings.local.json; the plan's guard caught it after command 1, the removal ran, no start was made | halted 1 | viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/live-round.md · `2026-10-17T18:23:32Z-b` |
| 2026-10-17-optional-plugin-install (implement/fix-loop) | rig 2 (inputs#I5 Q1, proj/ its own git repo) passed the local-scope guard, but start 1 from proj/ was held by a modal before SessionStart (no session-start, no viola mcp, child exit 1 at the stop); proj/ became its own project root with no trust record in ~/.claude.json; start 1 red, start 2 held for the founder | halted 1 | viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/live-sessions.ndjson · `2026-10-18T01:49:04Z-a` |
| 2026-10-17-optional-plugin-install (implement/fix-loop) | the one-Enter trust answer (inputs#I6 Q1) premised that the folder-trust dialog shows its confirm row selected; the rig read the row on screen without the selection pointer (unselected), typed no key, start 2 red; start 3 held for the founder | halted 1 | viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/live-round.md · `2026-10-18T02:20:52Z-a` |
| 2026-10-18-plugin-precedence-live-readings (implement/fix-loop) | the plan's premise that a start passes once the trust dialog is accepted was falsified: from proj/ inside the work tree the CLI showed the external-imports modal next (inferred cause: the repository CLAUDE.md's @.claude/session-handoff.md import outside proj/'s git root), costing start 1 of the cap 3 and one dialogue round | dialogue_rounds 1 | viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/live-sessions.ndjson · `2026-10-19T06:19:25Z-b` |
| 2026-10-19-overseer-switch-to-viola (implement/fix-loop) | plan step 8 expects stat ~/.viola to read 700 after the verify because the product's log open narrows it; it read 755: viola verify opens no home log without a VIOLA_NAME, so the narrowing comes only with the first run start | extra_reads 1 | viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-round.md · `2026-10-19T11:12:09Z-c` |

**Proposal:** The rig-placement four were absorbed mid-epoch: the founder ruled the rig outside the repository (2026-10-18-plugin-precedence-live-readings `inputs#I4`). The remaining pipeline-level direction, if any, is the one CLAUDE.md already states as a learning (an unmeasured CLI behaviour is a ledger row to probe, not an assumption): a phase check that a plan step resting on an undocumented CLI behaviour names its probe or ledger row before the live round spends a start.

### P6 — implement/fix-loop · `contract.test-expectation` — 7 cases · weight 17 · rate 7/19 step runs
**Pattern:** Seven test oracles were wrong on first run in seven chunks. Three are one helper: `tests/support/home.rs` `dialogless_home` pinning a pass-count literal that moved with the ledger row set (resume-and-compact, unwrapped, compact), each found only by pre-push because the plan's integration filter did not select it. The others are oracles written from plan literals (a call bound, an insertion order), a fixture written before the stamp, and test-side expectations in cli-machine.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-owed-ledger-rows | the new step_lines_count_to_the_relied_total case asserted every row fails an empty probe; largest-hook-payload passes over no capture by design, so the case's own oracle was wrong and was corrected | iterations 1 | — · `2026-10-13T23:30:43Z-c` |
| 2026-10-14-resume-and-compact-ledger-rows | three iterations fixed stale count oracles the plan's sweep did not list: a second [true; 21] verdict vector (ledger.rs), env 5 / hooks 37 receipts (cli_verify.rs), dialogless_home 17/14 (tests/support/home.rs, outside the integration filter, red only in pre-push) | iterations 3 | — · `2026-10-16T07:26:50Z-b` |
| 2026-10-16-remove-the-browser-leg | two test oracles written at P1 from the plan's literals were wrong for reality: pre_push_every_native_call_carries_only_home_and_path kept a > 10 call bound (the pass now makes 9) and the linux key list is insertion-ordered [run, gate], not the plan's [gate, run]; both fixed in one iteration | iterations 1 | .andromeda/runs/2026-10-16T16-17-45-implement/gate-2026-10-16-remove-the-browser-leg.json · `2026-10-16T17:04:06Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board | tests/support/home.rs dialogless_home asserted 18 pass  4 fail; with the agents row relied the stamp reads 19 pass  4 fail; the plan's integration filter did not select its binary, so only pre-push found it | iterations 1 | — · `2026-10-16T21:09:24Z-c` |
| 2026-10-17-end-to-end-path-binary-and-harness-legs | boot --agents-mode malformed read verify-failed: the agents-mode file written beside the session claude before the stamp made verify's Run B agents read malformed; fixed by writing it after the stamp (absent file answers recorded) | iterations 1 | .andromeda/runs/2026-10-17T01-21-28-implement/gate-2026-10-17-end-to-end-path-binary-and-harness-legs.json · `2026-10-17T01:42:23Z-c` |
| 2026-10-17-cli-machine-contract | three test-side oracles were wrong on first run: an absolute home compared with the canonical pinned_bin (cli_instance_state), list under a VIOLA_NAME read as creating no home, and viola --help's command descriptions read as options (--file); each fixed in the test, no product change | iterations 2 | — · `2026-10-17T03:25:37Z-b` |
| 2026-10-18-compact-post-condition-ledger-row | tests/support/home.rs dialogless_home pinned '19 pass  4 fail' / '16 pass  4 fail'; the 24-row set reads 20 / 17; neither research's lists nor the literal probe's five files named it, so only pre-push's cli_answer read it | iterations 1 | — · `2026-10-18T21:43:15Z-b` |

**Proposal:** The three `dialogless_home` cases recur the moved-bound rule (P3) and the research chain (X1). On the project side, one direction is that `dialogless_home` derives its summary from `LedgerRow::relied()` rather than a literal; the ledger does not show whether that has landed. On the pipeline side the direction is X1's.

### P7 — wrap-session/reconcile · `contract.cascade-miss` — 7 cases · weight 9 · rate 7/19 step runs
**Pattern:** Seven times in six wraps a stale master site drew no detector proposal and was caught later in the same pass, five by the step-2 sweep and two by a re-sweep or check 5. The misses are restated counts and literals (cli_verify summaries, the retired browser surface, refusal wording) and passages no detector reads (design-system, see P10).

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-owed-ledger-rows | security-plan.md:291 (accepted risk of verify's trusted runs) restated that Runs B, C and D run the user's own status line; no detector proposed it; the cascade step-2 sweep pattern user-statusline caught it and it was folded as architecture 7's duplicate | iterations 1 | .andromeda/runs/2026-10-14T06-37-54-wrap/cascade-dispositions.md · `2026-10-14T07:02:56Z-c` |
| 2026-10-16-remove-the-browser-leg | no detector proposed 16 sites the plan's expected amendments named or that stated the retired leg: design-system Typography (L243, L261, L269, L271), a11y-plan §1/§3/§9/§10/§11 beyond the obs-schema bind, obs-plan §1 tier basis / enum item / gap (b) / Last-Event-ID, test-plan pid-file and the two index labels; main raised them at check 5 | extra_reads 6 | .andromeda/runs/2026-10-16T17-15-05-wrap/fanout-results.md · `2026-10-16T18:40:25Z-b` |
| 2026-10-16-remove-the-browser-leg | the architecture-2 comparer left stack.md lines no master states (getrandom/constant_time_eq GUI token, npm-audit.sh, tower-http in viola-ui, axum-test/tower/reqwest/eventsource-client, the Node line, the browser a11y libs, DejaVu as the CI render); the post-apply re-sweep found them and they were rewritten by hand | extra_reads 2 | .andromeda/runs/2026-10-16T17-15-05-wrap/cascade-dispositions.md · `2026-10-16T18:40:25Z-d` |
| 2026-10-16-unwrapped-sessions-on-the-board | test-plan:586's cli_verify summary literals (18 pass 4 fail / 18 pass 1 fail, 20 pass 2 fail / 17 pass 2 fail) were stale before the pass; no detector proposed them; the step-2 sweep's counts pattern caught them and they were folded in from the chunk's own test literals | extra_reads 2 | .andromeda/runs/2026-10-16T22-30-02-wrap/cascade-dispositions.md · `2026-10-16T23:37:37Z-b` |
| 2026-10-17-end-to-end-path-binary-and-harness-legs | obs-plan heartbeat-ticks.md:11 restated the status derivation ('degraded when any item is stale'); the test-plan detector's note judged obs-plan §3 still agreeing and no obs proposal came; the step-2 status-interim sweep caught it and the bind fix was folded in | — | — · `2026-10-17T02:16:45Z-c` |
| 2026-10-17-cli-machine-contract | the step-2 sweep found 4 stale restatements no detector proposed: design-system.md:482 x2 (no exit-1 --json document; the stderr pair only), security-plan.md:214 (resolve_home's home_dir read), obs-plan.md:754 (revive's strict-modes refusal is the stderr pair) | iterations 1 | .andromeda/runs/2026-10-17T03-50-43-wrap/cascade-dispositions.md · `2026-10-17T04:21:36Z-b` |
| 2026-10-17-optional-plugin-install | obs-plan.md:466 (span naming) still gave cli.plugin_install as a dispatch-span example the pass retired; no detector proposed it; the cascade sweep caught it and it was amended in the pass | extra_reads 1 | .andromeda/runs/2026-10-18T02-36-56-wrap/cascade-dispositions.md · `2026-10-18T03:06:09Z-b` |

**Proposal:** The sweep is catching what the detectors miss. One direction is to move the recurring sweep patterns (count literals, retired surface names) into detector scope, and to give design-system a detector for its cli wording passages (P10's two structural cases name the same gap).

### P8 — wrap-session/report · `input.implement-outcome-unsettled` — 5 cases · weight 10 · rate 5/19 step runs
**Pattern:** In five chunks the operator pass's CI read superseded implement's green with a red in a file implement wrote, and the report re-based on the pass. Four of the five are a cross-OS red the Linux dev host does not run: Windows in three chunks (a ledger case, a test oracle, `viola.exe` in a usage line) and macOS in two (a `cli_send` race, a line-before-snapshot order), owed-ledger carrying one of each.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-owed-ledger-rows | implement's outcome (surfaced, then green+smoke) was superseded by the operator pass: ci.py read red on the pre-CI commit (macOS cli_send race opened by the fake agent's statusline replay; Windows ledger case popping a non-relied row); a fix commit made it green; the basis is the pass's commits and evidence/operator.md | iterations 1 | .andromeda/runs/2026-10-13T23-05-48-implement/outcome-2026-10-13-owed-ledger-rows.md · `2026-10-14T06:41:55Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board | the operator pass ended fixed (two CI reds, two fix commits to files implement wrote); the outcome basis was the updated outcome file plus evidence/operator-pass.md | iterations 2 | — · `2026-10-16T23:05:15Z-c` |
| 2026-10-17-cli-machine-contract | implement's outcome (green block, surfaced gap) was moved by the operator pass: CI on the pre-CI commit read red on test (windows-2025), a test oracle implement wrote, fixed by the pass's commit f738cbc; the report's basis is the pass's final HEAD CI run ci#38999642102 with implement's outcome file for what only it holds | — | ci#38998933306 job 117297543004; ci#38999642102 · `2026-10-17T03:55:28Z-b` |
| 2026-10-17-cli-output-discipline | implement's green was superseded by the operator pass: ci#39015978414 read test (windows-2025) red on help_lists_only_served_verbs (the per-verb usage line names viola.exe on Windows); fixed by 4a8a0ca, ci#39016750772 green 15/15 stood in as the outcome basis | retries 1 | viola-0.1.0/chunks/2026-10-17-cli-output-discipline/evidence/operator-pass.md · `2026-10-17T08:32:01Z-b` |
| 2026-10-19-dialog-hold-for-the-driver | implement's green was not the final word: the operator pass read CI red on macOS at 3def5b5b (a pre-existing line-before-snapshot order in register) and a fix commit d146c50c changed src/run/dialog.rs; the report's basis is the pass's final HEAD and its evidence | iterations 1 | ci#39226239371, ci#39228538322 · `2026-10-19T09:32:45Z-b` |

**Proposal:** See chain X2: implement's gates run on the Linux host only, and the Windows and macOS legs are first read at the operator pass. One direction is an implement-time cross-OS reading for the cfg- or OS-sensitive cases a chunk touches (e.g. dispatching the CI run on the implement branch head before the outcome is written); the founder weighs that cost against a pass that already catches them.

### P9 — implement/fix-loop · `contract.spec-reality-gap` — 5 cases · weight 9 · rate 5/19 step runs
**Pattern:** Five spec-versus-reality gaps were surfaced at fix-loop in five chunks: the fixture scrub passing a GitHub owner handle, the preservation guard red on a needed companion edit, plan-named E2E binaries outside test-plan's regex, revive's exit-1 codes short of start()'s refusals, and the live transcript delta of 5 against the accepted two.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-owed-ledger-rows | both recorded StatusLine variants carry workspace.repo.owner (the GitHub owner handle); the record scrub and the hygiene walk read only the OS user as a whole word, so the handle passes; test-plan §7 Fixture hygiene says fixtures are scrubbed of usernames | — | viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/evidence/live-round.md · `2026-10-13T23:30:43Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows | plan entry 15 (preservation guard) reads red on a needed companion edit, tests/support/home.rs, which its exclusion list does not name; surfaced, not authored | iterations 1 | — · `2026-10-16T07:26:50Z-c` |
| 2026-10-17-end-to-end-path-binary-and-harness-legs | plan-named E2E binaries path1_run_start and path6_budget_harness fall outside test-plan §3's regex ^(path\|tui\|mcp\|cross\|chaos\|contract)_ (the digit precedes the underscore); run --e2e read 18 binaries and run --integration ran path1; resolved by widening the regex on the overseer's word | iterations 1, dialogue_rounds 1 | .andromeda/runs/2026-10-17T01-21-28-implement/gate-2026-10-17-end-to-end-path-binary-and-harness-legs.json · `2026-10-17T01:42:23Z-b` |
| 2026-10-17-cli-machine-contract | v1-39 / plan step 10 close revive --json's exit-1 codes at five; start()'s own refusals reached after a passed preflight (batch-script-child, pinned-hash-mismatch, squatted-name, internal-error for a missing claude) print run's human pair or nothing under --json, with no code in the closed set; surfaced, not authored | — | — · `2026-10-17T03:25:37Z-d` |
| 2026-10-18-compact-post-condition-ledger-row | the live round's transcript delta was 5 (a -backup- file written at the compaction, two Run B session transcripts, Runs C and D), against security-plan §Data Protection's accepted 'two per real-CLI verify' | — | — · `2026-10-18T21:43:15Z-e` |

**Proposal:** Each was surfaced and routed through the amendment flow. No common cause is evident across the five; one of them (the guard) is L1's.

### P10 — any step (universal / recall: grouped by type alone) · `contract.structural-blind-spot` — 4 cases · weight 10 · 1 with halt
**Pattern:** A documented mechanism could not reach its subject four times in four chunks, one with a halt: design-system's only detector (`D-design-tokens`) cannot see cli wording passages (twice); `agent-run.sh --live-legs --entry N` runs no round step after the named leg, so the trailing census was hand-run (halt); and the E2E rmcp client accepts a `tools/list` result that claude 2.1.287 rejects, so only the live start saw it.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-14-resume-and-compact-ledger-rows (wrap-session/gates) | round entry 12 (the census after the last live leg) had no round-listing line: implement fired the round per leg with --live-legs --entry 11, which runs the round steps before the named leg and none after it, so the trailing census could only be run by hand; the P7.1 ASSERT halted and the overseer accepted the hand record (inputs#I8) | dialogue_rounds 1, halted 1 | .andromeda/runs/2026-10-16T14-05-30-wrap/relay-4.md · `2026-10-16T14:51:33Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board (wrap-session/reconcile) | the design-system detector set holds only D-design-tokens, so an As-built 'not built / owed' clause and a new refusal pair in cli patterns 1 and 2 drew 0 proposals; the plan's expected-amendments list carried them and Validate check 5 raised four items | extra_reads 3 | .andromeda/runs/2026-10-16T22-30-02-wrap/fanout-results.md · `2026-10-16T23:37:37Z-c` |
| 2026-10-17-cli-machine-contract (wrap-session/reconcile) | design-system's only detector (D-design-tokens) reads Coverage tokens flags, so revive's cli output wording (design-system.md:506, :513) was reachable only by check 5 from the plan's expected amendments | extra_reads 1 | .andromeda/runs/2026-10-17T03-50-43-wrap/fanout-results.md · `2026-10-17T04:21:36Z-d` |
| 2026-10-17-mcp-server-for-drivers (implement/fix-loop) | the E2E rmcp client accepts a tools/list result without ttlMs/cacheScope, so every gate read green while claude 2.1.287 (protocol 2026-07-28) dropped every tool; only the one live start could see it | iterations 1 | viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/live-mcp.md · `2026-10-17T12:33:19Z-c` |

**Proposal:** Three directions, one per mechanism: a design-system detector for cli patterns (shared with P7); a round runner form that runs the steps after the last leg; and a contract case that replays a recorded live `tools/list` exchange (a fixture from the live CLI) so the gate sees what only the live start saw.

### P11 — phase/plan · `input.research-thin` — 4 cases · weight 9 · 1 with halt · rate 4/22 step runs
**Pattern:** Synthesis found research short four times in four chunks, one with a halt (the resume-and-compact plan revision after implement stopped on the guard): a mutant cause research had the files to find, facts research read before the P4 cut, a literal-keyed sweep that missed `dialogless_home`, and a boundary that missed test supports.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-epoch-4-survivors | P3 recorded the PortablePty::resize mutant's pass as cause unknown; writing step 2 at P4 led to reading report_paths and the audit's mut-driver.py (jobs 2, one TMPDIR), which names a likely cause P3 had the files to find | extra_reads 2 | — · `2026-10-13T15:55:43Z-b` |
| 2026-10-13-owed-ledger-rows | research ran before the P4 cut was answered, so three facts the decided shape turned on (the fake agent's --settings takes a file path only, the fixture schema requires hook_event_name, the status line is not wrapped on Windows) were read at synthesis | extra_reads 4 | .andromeda/runs/2026-10-13T22-31-54-phase/relay-1.md · `2026-10-13T22:59:19Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows | the take-up's research sweep keyed on pass-count literals in tests/cli_verify.rs missed tests/support/home.rs dialogless_home, which builds its summary from an integer; the plan's preservation guard did not exclude it, implement stopped on entry 15 and this plan revision followed | halted 1, iterations 1 | .andromeda/runs/2026-10-16T06-46-40-implement/outcome-2026-10-14-resume-and-compact-ledger-rows.md · `2026-10-16T07:32:45Z-b` |
| 2026-10-17-cli-machine-contract | synthesis found research's boundary short: the VIOLA_DIR step reaches test-started children whose env carries VIOLA_DIR on this host, canonicalisation reaches tests/support/start.rs's endpoint hash and run.rs's home creation, and test-plan's 'config schema' had no file; research.md lists, sweeps and scope were amended | extra_reads 5, iterations 1 | .andromeda/runs/2026-10-17T02-25-51-phase/ · `2026-10-17T02:44:48Z-b` |

**Proposal:** This is chain X1's plan-side end; the direction is X1's.

### P12 — implement/fix-loop · `tooling.result-not-run-stable` — 3 cases · weight 10 · rate 3/19 step runs
**Pattern:** Three times an identical tree read red, then green: a `cli_send` timing case once in seven firings (unwrapped), path7 alone versus in the full layer plus a truncated profile (mcp), and pre-push's coverage merge refusing a truncated `viola hook` `.profraw` (overseer-switch).

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-16-unwrapped-sessions-on-the-board | gate entry 11 over an identical tree read green (firing 3), red on one cli_send timing case (firing 5), then green again (firings 6 and 7); 20 two-sided reruns after the red never reproduced it; host load 37 to 41 in every window | retries 4 | .andromeda/runs/2026-10-16T20-14-16-implement/journal.md · `2026-10-16T21:09:24Z-b` |
| 2026-10-17-mcp-server-for-drivers | over one tree path7 passed alone and timed out in the full E2E layer; pre-push passed then failed on a truncated profile; both causes read and fixed, the first two-sided (6/24 vs 0/24) | iterations 2 | viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/unstamped-boot-race.md · `2026-10-17T12:33:19Z-d` |
| 2026-10-19-overseer-switch-to-viola | pre-push over an identical tree read red (coverage merge refused one truncated viola hook .profraw, 2958/2958 tests passed) then green on the next firing; no artifact froze the first number | retries 1, extra_reads 6 | viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/profraw-red.md · `2026-10-19T11:12:09Z-b` |

**Proposal:** See L2: the truncated-profile red recurs across epochs.

### P13 — wrap-session/gates · `tooling.light-gate-red` — 2 cases · weight 13 · 1 with halt · rate 2/18 step runs
**Pattern:** The light gate read red twice; one halted. Paste-newline: implement's two reds reproduced exactly, both plan-wording reds (the literal probe, the preservation guard) for which the P7.1 ASSERT has no arm. Overseer-switch: `Disk quota exceeded` at verify's pinned-copy write on the `/tmp` usrquota tmpfs under nextest's parallelism (22 GB of the user's `/tmp` was Claude Code session output), then a truncated `.profraw`.

**Evidence:** ALL 2 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-paste-newline-ledger-row | the light gate reproduced implement's two reds exactly (12 last line 1, 13 exit 1); neither is a tree defect and the P7.1 ASSERT names no arm for a red the plan's own wording causes, so the wrap proceeded on the operator's direction rather than an ASSERT arm | — | .andromeda/runs/2026-10-13T21-52-39-wrap/gate-2026-10-13-paste-newline-ledger-row.json · `2026-10-13T22:18:46Z-b` |
| 2026-10-19-overseer-switch-to-viola | the light gate re-ran the plan block that implement ran green and read integration, e2e and pre-push red: every stamped-home test failed at viola verify's pinned-copy write with Disk quota exceeded (os error 122) under nextest's parallelism on the /tmp usrquota tmpfs, passing with -j1; then, quota freed, pre-push red on a truncated .profraw (the chunk's open red recurring) | retries 1, dialogue_rounds 2, halted 2, extra_reads 14 | .andromeda/runs/2026-10-19T19-16-36-wrap/gate-2026-10-19-overseer-switch-to-viola.json · `2026-10-19T20:39:26Z-b` |

**Proposal:** Two directions: a P7.1 ASSERT arm for a red the plan's own wording causes (L1), and a disk-headroom read before a gate that boots stamped homes (the handoff already records the need). The profraw half is L2.

### P14 — any step (universal / recall: grouped by type alone) · `contract.jointly-contradictory-instructions` — 3 cases · weight 7
**Pattern:** Three instruction pairs proved jointly unsatisfiable. Two are the plan's preservation guard against a mandatory companion edit (a workflow matrix line; Trigger 3's companion test edits). The third is a plan line saying matrix records are P5's and the wrap's while implement's P2 MUST writes them. No mechanical check tried any of the three.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-interrupted-verify-cleanup (implement/fix-loop) | plan entry 9's preservation guard (no workflow edit) and the contract test in entry 17's pre-push (every cfg(windows) product file listed in windows-mutants.yml) were jointly unsatisfiable once stop.rs carried its Windows console routine; P5's guard control minted stop.rs without running the contract test; the guard was authored at P4 against a standing test | iterations 1, dialogue_rounds 1 | .andromeda/runs/2026-10-13T18-48-52-implement/gate-2026-10-13-interrupted-verify-cleanup.json · `2026-10-13T19:09:01Z-b` |
| 2026-10-13-paste-newline-ledger-row (implement/fix-loop) | the preservation guard (plan entry 13) refuses any test file outside research's lists, while the fix-loop protocol's Trigger 3 makes a companion test edit (a test pinning the changed data) mandatory and recorded; the integration and pre-push entries needed two such edits, so 13 cannot read green on a tree where they are; authored together in the plan; P5's controls did not try a recorded companion | iterations 1 | .andromeda/runs/2026-10-13T21-22-15-implement/gate-2026-10-13-paste-newline-ledger-row.json · `2026-10-13T21:39:27Z-d` |
| 2026-10-18-plugin-precedence-live-readings (implement/fix-loop) | plan step 12 says matrix records are P5's and the wrap's, never implement's, while the implement skill's P2 MUST sets ref + implemented for the chunk's caps on green; the skill's ledger write was made and the conflict surfaced in the report; no mechanical check attempted it; the plan line was authored at phase P4, the skill rule earlier | extra_reads 1 | — · `2026-10-19T06:19:25Z-c` |

**Proposal:** The first two are L1. For the third, one direction is a phase P4 rule that a plan does not restate or narrow a skill-owned write right.

### P15 — any step (universal / recall: grouped by type alone) · `contract.narrow-basis-claim` — 3 cases · weight 6
**Pattern:** Three counts were written from a narrower basis than the claim: two research sweep counts typed before their pattern ran (4+2 → 6, 79 → 84 files), and research's literal sweep taken as the full basis for a count move (five shape-pinned readers outside it). A fourth case of the first shape sits untyped (`2026-10-18T03:41:19Z-b`, 74 → 88 hits).

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-13-epoch-4-survivors-ii (phase/research) | research.md's first draft wrote a sweep as 4 hits plus 2 more before its stated pattern was run; the run of that pattern read 6 hits, and the line was corrected | extra_reads 1 | — · `2026-10-13T17:38:06Z-b` |
| 2026-10-13-paste-newline-ledger-row (implement/fix-loop) | research's sweep (75 hits of the seventeen-row literals, 'all six files on the modify list') was the basis for the count move; five sites pinning the row set by shape (a verdict vector, a stamp's rows object, an older-stamp oracle, a dialogless summary `13 pass  4 fail`, a four-variant list) were outside it and read red only at the unit, integration and pre-push gates | iterations 2 | .andromeda/runs/2026-10-13T21-22-15-implement/gate-2026-10-13-paste-newline-ledger-row.json · `2026-10-13T21:39:27Z-c` |
| 2026-10-16-unwrapped-sessions-on-the-board (phase/research) | research.md's sweep row first carried 79 files, a figure taken from an earlier listing's length rather than from the stated pattern and filters; re-deriving with the command written beside it gave 84 | retries 1 | — · `2026-10-16T19:05:10Z-b` |

**Proposal:** One direction is a research-letter rule that a count in `research.md` is the printed output of the command written beside it, never typed. The third case is X1.

### P16 — any step (universal / recall: grouped by type alone) · `contract.grammar-irregularity` — 4 cases · weight 4
**Pattern:** Pipeline tools printed a parse verdict four times on the working route: line 156 three times (`CARRY:` after one space, freight or prose, and a `WATCH:` clause with no since/retires/k-of-N form) across two sessions and one take-up, and line 144 once (a 569-char WATCH clause `UNPARSED`). A site at n ≥ 2 is the grammar or its writer.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| — (new-session/orientation) | route.py pins: 144 · WATCH: · 569 chars · clause UNPARSED | — | route.py pins --root . (new-session, no run dir) · `2026-10-17T00:18:55Z-b` |
| — (new-session/orientation) | INDETERMINATE: working-route.md:156 — after ONE space — freight or prose, no structural split: 'CARRY:' (pins also printed the line's WATCH: clause UNPARSED) | — | route.py cursor / markerless / pins / epoch output, this session · `2026-10-18T22:38:51Z-b` |
| 2026-10-18-plugin-precedence-live-readings (phase/take-up) | INDETERMINATE: working-route.md:156 — after ONE space — freight or prose, no structural split: 'CARRY:'; pins also read the line's WATCH: clause UNPARSED (no since/retires/k-of-N form); both folded by hand from the line | — | .andromeda/runs/2026-10-18T22-39-54-phase/route-2026-10-18-plugin-precedence-live-readings.json · `2026-10-18T22:41:58Z-b` |
| — (new-session/orientation) | INDETERMINATE: working-route.md:156 — after ONE space — freight or prose, no structural split: 'CARRY:' | — | route.py cursor / markerless / epoch output, this session (no run dir) · `2026-10-19T05:20:21Z-b` |

**Proposal:** Line 156's annotation was written off-grammar, and every read after it paid a by-hand fold. One direction is a write-time check: route-resolve runs `route.py pins` on the line it just wrote and refuses an `UNPARSED` / `INDETERMINATE` result, so the irregularity is fixed once at the writer rather than read around at each later read.

### P17 — wrap-session/reconcile · `input.report-insufficient` — 3 cases · weight 5 · rate 3/19 step runs
**Pattern:** The P1 report misled reconcile three times in three wraps: a site table built from four sweep patterns missed differently worded sites, a Counts bullet dispositioned half of a 3.4 KB line, and a Counts row named the stale and the true line the wrong way round.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| — | the P1 report's site table, built from four sweep patterns, omitted batch sites worded 'accepted by the operator', 'the operator's answer' outside its patterns, and a third card-confirm site worded 'confirmed the one card' | extra_reads 6, dialogue_rounds 1 | .andromeda/runs/2026-10-13T06-38-35-wrap/cascade-dispositions.md · `2026-10-13T07:00:53Z-b` |
| 2026-10-13-interrupted-verify-cleanup | the report's Counts bullet dispositioned registries/contracts/architecture/ci-cd-approach.md:2 as a past run's «all nine jobs» only, but the same 3.4 KB line also stated the matrix's present-tense nine-item / root-four count; the architecture detector proposed it | extra_reads 1 | .andromeda/runs/2026-10-13T20-00-28-wrap/fanout-architecture.md · `2026-10-13T20:22:54Z-b` |
| 2026-10-17-mcp-server-for-drivers | the report's Counts row named test-plan.md:399 stale and :1021 true for the sync-crates list; the reverse held (:399 names no crate, :1021 listed viola-channel) — caught by the test-plan detector's note and a re-read | extra_reads 1 | .andromeda/runs/2026-10-17T14-19-45-wrap/fanout-results.md · `2026-10-17T14:55:18Z-c` |

**Proposal:** Each was caught by a detector note or a re-read. The first case shares U1's cause (the sweep patterns); no other common cause is evident.

## Cross-step chains (starting heuristics)

### X1 — phase/research →research→ implement/fix-loop · implement/code · phase/plan — 6 + 5 + 6 chunks (12 distinct)
| chunk | producer (step record) | consumer (step / friction records) |
|---|---|---|
| 2026-10-13-interrupted-verify-cleanup | `2026-10-13T18:39:01Z-a` | `2026-10-13T19:09:01Z-a` |
| 2026-10-13-paste-newline-ledger-row | `2026-10-13T20:43:52Z-a` | `2026-10-13T21:39:27Z-a` |
| 2026-10-14-resume-and-compact-ledger-rows | `2026-10-14T07:22:25Z-a` | `2026-10-16T07:26:50Z-a · 2026-10-16T07:32:45Z-b` |
| 2026-10-16-unwrapped-sessions-on-the-board | `2026-10-16T19:05:10Z-a` | `2026-10-16T21:09:24Z-a · 2026-10-16T20:44:30Z-a` |
| 2026-10-18-compact-post-condition-ledger-row | `2026-10-18T03:41:19Z-a` | `2026-10-18T21:43:15Z-a` |
| 2026-10-19-dialog-hold-for-the-driver | `2026-10-19T07:53:34Z-a` | `2026-10-19T08:36:28Z-a · 2026-10-19T08:27:44Z-a/-b` |
| 2026-10-13-epoch-4-survivors | `2026-10-13T14:32:47Z-b` | `2026-10-13T16:18:35Z-a · 2026-10-13T15:55:43Z-a/-b` |
| 2026-10-13-owed-ledger-rows | `2026-10-13T22:42:12Z-a` | `2026-10-13T23:18:46Z-a · 2026-10-13T22:59:19Z-a/-b` |
| 2026-10-16-remove-the-browser-leg | `2026-10-16T15:49:00Z-a` | `2026-10-16T16:26:06Z-a/-c` |
| 2026-10-17-cli-machine-contract | `2026-10-17T02:38:42Z-a` | `2026-10-17T02:44:48Z-a/-b` |
| 2026-10-17-cli-output-discipline | `2026-10-17T04:46:09Z-a` | `2026-10-17T04:50:30Z-a · 2026-10-17T07:36:41Z-a` |
| 2026-10-17-mcp-server-for-drivers | `2026-10-17T09:44:50Z-a` | `2026-10-17T11:25:18Z-a` |

Every producer record is formally `ok` with the one signal `unresolved-questions` (unwrapped adds `known-positive-control`); every consumer graded `research` `thin`. The recurring miss is a reader research's literal-keyed sweep cannot see: a helper that builds a count from an integer or pins a row set by shape (`tests/support/home.rs` `dialogless_home` in four chunks: paste-newline, resume-and-compact, unwrapped, compact; `merge_stamp`'s envelope; a second verdict vector), and a test-side caller of a changed constructor (`src/run/wheel.rs` in dialog-hold). **Chain hypothesis:** research grades itself on its lists being written, not on their reach, and the cost lands at fix-loop (stale oracles, P6), at implement/code (companions outside the guard, L1) and at synthesis (P11). It is the same cause as P3's four-time recurrence.

**Proposal:** A research-letter step that lists a moved bound's or a changed signature's readers through the code graph (`tree.db`: callers and readers of the deciding symbol) beside the literal sweep, and a research producer signal that names which of the two produced its lists, so a consumer's `thin` can be joined to it.

### X2 — implement/smoke →implement-outcome→ wrap-session/report — 7 chunks
| chunk | producer (step record) | consumer (step / friction records) |
|---|---|---|
| 2026-10-13-owed-ledger-rows | `2026-10-13T23:30:43Z-d` | `2026-10-14T06:41:55Z-a` |
| 2026-10-14-resume-and-compact-ledger-rows | `2026-10-16T07:38:01Z-b` | `2026-10-16T14:10:45Z-a` |
| 2026-10-16-unwrapped-sessions-on-the-board | `2026-10-16T21:09:24Z-d` | `2026-10-16T23:05:15Z-a` |
| 2026-10-17-cli-machine-contract | `2026-10-17T03:26:06Z-a` | `2026-10-17T03:55:28Z-a` |
| 2026-10-17-cli-output-discipline | `2026-10-17T08:01:35Z-a` | `2026-10-17T08:32:01Z-a` |
| 2026-10-18-compact-post-condition-ledger-row | `2026-10-18T21:43:15Z-f` | `2026-10-18T22:04:15Z-a` |
| 2026-10-19-dialog-hold-for-the-driver | `2026-10-19T08:51:58Z-a` | `2026-10-19T09:32:45Z-a` |

Producers are `ok` (`green` or `surfaced`); each consumer graded `implement-outcome` `thin` because the operator pass changed a file implement wrote. Five of the seven are CI reds the pass fixed (owed-ledger, unwrapped, cli-machine, cli-output, dialog-hold; four of them on Windows or macOS, P8); two are the pass's hygiene rewrite of an implement-written evidence line (resume-and-compact, compact). Read with the 2026-10-13 calibration boundary: these are fixes to implement's own files, so `thin` is the grade's intended reading, not era. **Chain hypothesis:** implement's gates are the Linux dev host's; the cross-OS legs exist only in CI and are read first at the operator pass. This is P8's evidence seen from the producer's side.

**Proposal:** As P8, plus one for the two hygiene rewrites: implement's evidence writes pass through the same hygiene read the operator pass applies, before the outcome is written.

### X3 — phase/plan · phase/validate →plan→ implement/fix-loop · wrap-session/gates — 4 + 3 chunks
| chunk | producer (step record) | consumer (step / friction records) |
|---|---|---|
| 2026-10-13-interrupted-verify-cleanup | `phase/validate` | `2026-10-13T19:09:01Z-a · 2026-10-13T20:30:06Z-a` |
| 2026-10-13-paste-newline-ledger-row | `phase/validate` | `2026-10-13T21:39:27Z-a · 2026-10-13T22:18:46Z-a` |
| 2026-10-18-compact-post-condition-ledger-row | `phase/validate` | `2026-10-18T21:43:15Z-a · 2026-10-18T22:36:37Z-a` |
| 2026-10-18-plugin-precedence-live-readings | `phase/validate` | `2026-10-19T06:19:25Z-a` |

The plan reaches fix-loop graded `wrong` or `thin` and reaches the light gate `thin`, in the same chunks, for the same entry: the preservation guard (or, in paste-newline, the literal probe beside it) red on a recorded companion edit. Validate graded the plan `ok` each time; its controls did not try a recorded companion (the jointly-contradictory records say so). **Chain hypothesis:** the guard is authored at P4 without an exclusion for the companions fix-loop is obliged to record, so a correct tree reads it red at implement and again at the gate. This is the level candidate L1.

**Proposal:** L1's.

## Level candidates (systemic-masked-as-project)

### L1 — override — the plan's preservation guard red on recorded companion edits — 4 overridden + 1 unresolved + 1 workaround facts, 4 chunks
**Facts:**

| chunk | skill/step | nature/solution | note | record |
|---|---|---|---|---|
| 2026-10-13-interrupted-verify-cleanup | implement/fix-loop | process/overridden | pre-push red on an unlisted src/cmd/verify/stop.rs; the operator chose the companion windows-mutants.yml matrix line (recorded in scope-record.md), leaving the plan's preservation guard red by that file, surfaced for the wrap | `2026-10-13T19:09:01Z-a` |
| 2026-10-13-interrupted-verify-cleanup | wrap-session/gates | process/overridden | the light gate's entry 9 read red, this chunk's own (no not-this-chunk's basis); the commit and the flip proceeded on the operator's wrap direction recording it as a deviation, the guard left unedited | `2026-10-13T20:30:06Z-a` |
| 2026-10-13-paste-newline-ledger-row | wrap-session/gates | process/overridden | commit and master flip with plan entries 12 (literal probe) and 13 (preservation guard) red: the ASSERT has no arm for a plan-wording red, and the operator's word (inputs#I4) recorded both as deviations, oracle and guard unchanged | `2026-10-13T22:18:46Z-a` |
| 2026-10-18-compact-post-condition-ledger-row | wrap-session/gates | process/overridden | entry 15 red on recorded companion/in-intent edits passed the light gate on the operator's direction (inputs#I7), not on a not-this-chunk's basis | `2026-10-18T22:36:37Z-a` |
| 2026-10-18-compact-post-condition-ledger-row | implement/fix-loop | process/unresolved | the preservation guard (entry 15) reads red on tests/support/home.rs and tests/tui_channel_fds.rs, both recorded in scope-record.md; surfaced to the operator | `2026-10-18T21:43:15Z-a` |
| 2026-10-19-dialog-hold-for-the-driver | implement/code | process/workaround | the viola-core dialog_hold_ case is a literal table test, not an rstest: viola-core has no rstest dev-dependency and the preservation guard refuses a Cargo.toml edit | `2026-10-19T08:27:44Z-a` |

**Correlated friction records:**

| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-13-interrupted-verify-cleanup | implement/fix-loop | contract.jointly-contradictory-instructions | plan entry 9's preservation guard (no workflow edit) and the contract test in entry 17's pre-push (every cfg(windows) product file listed in windows-mutants.yml) were jointly unsatisfiable once stop.rs carried its Windows console routine; P5's guard control minted stop.rs without running the contract test; the guard was authored at P4 against a standing test | iterations 1, dialogue_rounds 1 | .andromeda/runs/2026-10-13T18-48-52-implement/gate-2026-10-13-interrupted-verify-cleanup.json · `2026-10-13T19:09:01Z-b` |
| 2026-10-13-paste-newline-ledger-row | implement/fix-loop | contract.jointly-contradictory-instructions | the preservation guard (plan entry 13) refuses any test file outside research's lists, while the fix-loop protocol's Trigger 3 makes a companion test edit (a test pinning the changed data) mandatory and recorded; the integration and pre-push entries needed two such edits, so 13 cannot read green on a tree where they are; authored together in the plan; P5's controls did not try a recorded companion | iterations 1 | .andromeda/runs/2026-10-13T21-22-15-implement/gate-2026-10-13-paste-newline-ledger-row.json · `2026-10-13T21:39:27Z-d` |
| 2026-10-14-resume-and-compact-ledger-rows | implement/fix-loop | contract.spec-reality-gap | plan entry 15 (preservation guard) reads red on a needed companion edit, tests/support/home.rs, which its exclusion list does not name; surfaced, not authored | iterations 1 | — · `2026-10-16T07:26:50Z-c` |
| 2026-10-19-dialog-hold-for-the-driver | implement/code | input.plan-step-ambiguous | plan step 1 asks for an rstest in crates/viola-core, which has no rstest dev-dependency while the plan's preservation guard forbids a manifest edit; written as a literal table test under the planned name | reformulations 1 | — · `2026-10-19T08:27:44Z-c` |
| 2026-10-13-paste-newline-ledger-row | wrap-session/gates | tooling.light-gate-red | the light gate reproduced implement's two reds exactly (12 last line 1, 13 exit 1); neither is a tree defect and the P7.1 ASSERT names no arm for a red the plan's own wording causes, so the wrap proceeded on the operator's direction rather than an ASSERT arm | — | .andromeda/runs/2026-10-13T21-52-39-wrap/gate-2026-10-13-paste-newline-ledger-row.json · `2026-10-13T22:18:46Z-b` |

**Level hypothesis:** The operator overrode the same gate four times in three chunks (interrupted-verify, paste-newline, compact), each time because a correct tree carries an edit the fix-loop protocol makes mandatory and records (`scope-record.md`), and the guard, authored at P4, does not exclude recorded companions. The same rule drove 4 of the epoch's 6 fix-loop `ok-degraded` outcomes and both gates `ok-degraded` outcomes, a plan revision (resume-and-compact), and two jointly-contradictory records; the handoff carries it for the operator's word ("the plan template's preservation guard reads red on any recorded companion edit"). **Level hypothesis:** the cause lives in the plan template's guard form, which is pipeline-owned. The fixes so far are per-chunk overrides and deviations.

**Proposal:** The guard's form reads the chunk's own `scope-record.md` and excludes the companions recorded there, and the P7.1 ASSERT gains an arm for a red the plan's own wording causes. Whether that weakens the guard's purpose is the founder's judgment.

### L2 — chronic-degrade — a truncated `.profraw` (and other not-run-stable reds) at the coverage merge, recurring across epochs — 3 in-epoch facts + 3 in-epoch frictions
**Facts:**

| chunk | skill/step | nature/solution | note | record |
|---|---|---|---|---|
| 2026-10-19-overseer-switch-to-viola | implement/fix-loop | process/deferred | pre-push entry 19's first firing: llvm-cov refused a viola hook's truncated .profraw (its writer named by counters, its killer not known); recorded in evidence/profraw-red.md and left open with its cause owed, the second firing green | `2026-10-19T11:12:09Z-a` |
| 2026-10-19-overseer-switch-to-viola | wrap-session/gates | environment/overridden | pre-push red again on a truncated viola .profraw at the coverage merge (2958/2958 passed), no clean-tree basis run; the overseer's technical answer ruled it not this chunk's with a re-watch as owner | `2026-10-19T20:39:26Z-a` |
| 2026-10-16-unwrapped-sessions-on-the-board | implement/fix-loop | process/deferred | cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins read exit 0 (10 asserted) once in firing 5's entry 11; green in 3 other firings of this tree and in 20 two-sided runs after it (5+5 standalone, 3+3 binary, 2+2 entry filter on this tree and on git archive HEAD); cause not known, left open for the wrap | `2026-10-16T21:09:24Z-a` |

**Correlated friction records:**

| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-16-unwrapped-sessions-on-the-board | implement/fix-loop | tooling.result-not-run-stable | gate entry 11 over an identical tree read green (firing 3), red on one cli_send timing case (firing 5), then green again (firings 6 and 7); 20 two-sided reruns after the red never reproduced it; host load 37 to 41 in every window | retries 4 | .andromeda/runs/2026-10-16T20-14-16-implement/journal.md · `2026-10-16T21:09:24Z-b` |
| 2026-10-17-mcp-server-for-drivers | implement/fix-loop | tooling.result-not-run-stable | over one tree path7 passed alone and timed out in the full E2E layer; pre-push passed then failed on a truncated profile; both causes read and fixed, the first two-sided (6/24 vs 0/24) | iterations 2 | viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/unstamped-boot-race.md · `2026-10-17T12:33:19Z-d` |
| 2026-10-19-overseer-switch-to-viola | implement/fix-loop | tooling.result-not-run-stable | pre-push over an identical tree read red (coverage merge refused one truncated viola hook .profraw, 2958/2958 tests passed) then green on the next firing; no artifact froze the first number | retries 1, extra_reads 6 | viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/profraw-red.md · `2026-10-19T11:12:09Z-b` |
| 2026-10-19-overseer-switch-to-viola | wrap-session/gates | tooling.light-gate-red | the light gate re-ran the plan block that implement ran green and read integration, e2e and pre-push red: every stamped-home test failed at viola verify's pinned-copy write with Disk quota exceeded (os error 122) under nextest's parallelism on the /tmp usrquota tmpfs, passing with -j1; then, quota freed, pre-push red on a truncated .profraw (the chunk's open red recurring) | retries 1, dialogue_rounds 2, halted 2, extra_reads 14 | .andromeda/runs/2026-10-19T19-16-36-wrap/gate-2026-10-19-overseer-switch-to-viola.json · `2026-10-19T20:39:26Z-b` |

**Level hypothesis:** Pre-push's coverage merge refused a truncated `viola hook` `.profraw` with every test passing in Epoch 3 (`2026-10-04T22:18:54Z-b`: "the WATCH subject, 0/3 since the wheel chunk"; `2026-10-06T22:24:12Z-b`: three truncated profiles in three runs), in this epoch's mcp chunk (read as fixed), and twice in overseer-switch (implement and the light gate). `tooling.result-not-run-stable` holds 18 records across all five epochs. It never halted: each instance was deferred, ruled not this chunk's, or re-fired green, and the route now carries it as a WATCH on "Newer claude CLI rows", 0/3. **Level hypothesis:** an environment or harness cause (the profile writer killed mid-write; the handoff says the killer is unknown) absorbed one chunk at a time as a re-fire.

**Proposal:** A cause-finding chunk of its own (the killer of the `viola hook` profile writer) rather than a per-chunk WATCH, or a coverage-merge form that names and skips a truncated profile with a recorded count. The founder chooses between them.

### L3 — band-aid — a pipeline tool's output filtered or discarded against the run-bare rule — 9 facts, 8 chunks
**Facts:**

| chunk | skill/step | nature/solution | note | record |
|---|---|---|---|---|
| 2026-10-13-epoch-4-cleanup | phase/distill | process/workaround | the four registry.py contracts render calls ran piped through tail -2 where the letter says every tool call runs bare; each printed its wrote line and the renders landed | `2026-10-13T07:15:41Z-a` |
| 2026-10-13-epoch-4-cleanup | phase/research | process/workaround | the first graph query's stdout was read through tail where the cookbook says read the trace or the whole stdout; a scratchpad reader of the trace's last record was written and used for both queries | `2026-10-13T07:23:36Z-a` |
| 2026-10-13-epoch-4-survivors | phase/validate | process/workaround | after the baseline writes, three saved tool outputs were first read by their last line (tail -n 1 on the files) against the operator's new rule to read a pipeline tool's output whole; re-read whole in the next call | `2026-10-13T15:59:51Z-a` |
| 2026-10-17-cli-output-discipline | phase/distill | process/workaround | the four registry.py contracts renders ran in a for-loop whose combined output was piped into grep -v to drop indented rows, against the skill's never-filter rule; the guard did not refuse that loop-wrapped pipe, so the renders landed and their head and summary lines were read filtered | `2026-10-17T04:42:07Z-a` |
| 2026-10-17-mcp-server-for-drivers | phase/distill | process/workaround | registry.py contracts was run with its stdout sent to /dev/null in a loop over the four keyed masters, against the letter's rule that a tool's stdout is never redirected; the renders and trails were written and the exits read 0 | `2026-10-17T09:37:18Z-a` |
| 2026-10-17-optional-plugin-install | phase/validate | process/workaround | piped gate.py show into tail once to read entry 8's red cause, against the skill's no-filter rule; the cause was then read from the entry's own log file | `2026-10-17T18:06:04Z-a` |
| 2026-10-18-compact-post-condition-ledger-row | wrap-session/reconcile | process/workaround | the leaf apply's stdout went to /dev/null against the run-bare rule; its record is the cascade trail, the landing read by git diff --stat | `2026-10-18T22:33:38Z-a` |
| 2026-10-19-dialog-hold-for-the-driver | phase/validate | process/workaround | the first clone firing of entries 10 and 11 piped gate.py's output through grep, which the phase Setup letter forbids (a tool's stdout never through a filter); the filter kept each entry's verdict row, every later control was fired and read unfiltered, and the slip was named on the review card | `2026-10-19T08:18:14Z-a` |
| 2026-10-19-overseer-switch-to-viola | phase/validate | process/workaround | twice a pipeline tool's output was piped into a filter (gate.py into grep inside a control helper, the re-run set into tail) instead of read bare as the letter and the user's memory rule say; the guard did not refuse those forms | `2026-10-19T10:52:14Z-a` |

**Level hypothesis:** Nine times in eight chunks (distill ×3, validate ×4, research ×1, reconcile ×1) a pipeline tool's stdout went through `tail`, `grep` or `/dev/null` against the letters' rule (and, from 2026-10-13, the operator's own rule and the user's memory rule). The Bash guard refuses the plain form, but two notes record forms it did not refuse: a `for` loop's combined output piped on (`2026-10-17T04:42:07Z-a`) and a pipe inside a control helper (`2026-10-19T10:52:14Z-a`). `/dev/null` appears twice. The previous epoch holds no such fact, so the theme starts with the rule. The handoff's recurrence-despite-learning list carries it again this wrap. No typed friction correlates; the deviations were smooth. **Level hypothesis:** the obstacle is the tools' output volume against a read-it-whole rule, held by a guard with gaps and by agent recall.

**Proposal:** Either the tools print a compact verdict form by default (so a bare read is cheap and the urge to filter goes), or the guard covers the loop-wrapped, helper-internal and `/dev/null` forms. Both live in the pipeline, not the project.

### L4 — band-aid — multi-KB single-line edits through scratch python replace scripts — 3 facts in-epoch, 9 in Epoch 4
**Facts:**

| chunk | skill/step | nature/solution | note | record |
|---|---|---|---|---|
| 2026-10-16-remove-the-browser-leg | wrap-session/reconcile | process/workaround | master-body edits on multi-KB lines went through exact-match python replacement scripts (scratchpad repl.py / dellines.py and inline python heredocs carrying the replacement text) rather than one Edit-tool call per site; each asserted a single match and wrote nothing on a miss | `2026-10-16T18:40:25Z-a` |
| 2026-10-17-cli-machine-contract | wrap-session/reconcile | process/workaround | spec-body and run-dir point edits on multi-KB lines went through python replace scripts (a scratchpad file run by path, and short inline heredoc scripts asserting one match each) instead of one Edit-tool call per site; the skill names the Edit tool for a point change | `2026-10-17T04:21:36Z-a` |
| 2026-10-17-cli-machine-contract | wrap-session/route-resolve | process/workaround | the two multi-KB route lines were edited by one scratchpad python script asserting each match once, not an anchored Edit or splice call | `2026-10-17T04:25:45Z-a` |

**Correlated friction records:**

| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-17-mcp-server-for-drivers | wrap-session/route-resolve | tooling.long-line-edit | the anchored Edit appending a CARRY to 'Optional plugin install' matched twice: its last clause is repeated verbatim on 'Explicit session links'; re-anchored with a longer prefix | retries 1 | — · `2026-10-17T14:58:42Z-b` |

**Level hypothesis:** Reconcile (×2) and route-resolve (×1) edited master and route lines of several KB through count-asserted python scripts in the scratchpad, where the letter names the Edit tool or a splice call. Epoch 4 holds nine facts of the same theme (reads and edits of multi-KB lines through scratch window and replace scripts) and three `tooling.long-line-edit` records; this epoch adds one more of that type. **Level hypothesis:** the masters' and the route's line grammar (one passage or one entry per line, measured multi-KB) is the obstacle; each session rebuilds the same scratch tool.

**Proposal:** One sanctioned long-line point-edit form, an anchored replace with a single-match assertion as a pipeline tool verb, named by the letters that today name the Edit tool. The other direction is a line grammar that breaks long passages.

### L5 — band-aid (cross-stream) — `cascade.py` reads a backticked `|` on a non-table line as a table cell — 2 facts + 3 frictions, 4 distinct events in 4 chunks
**Facts:**

| chunk | skill/step | nature/solution | note | record |
|---|---|---|---|---|
| 2026-10-16-unwrapped-sessions-on-the-board | wrap-session/reconcile | process/workaround | cascade.py leaves refused obs-summary.md:67 as reshaped (a backticked pipe on a bullet line read as a table cell); the finding was written by hand | `2026-10-16T23:37:37Z-a` |
| 2026-10-19-dialog-hold-for-the-driver | wrap-session/reconcile | process/workaround | cascade.py replace refused obs-plan.md:593 and :596 as reshaped (a `\|` inside a code span read as a table cell boundary); both short lines were edited with the Edit tool instead | `2026-10-19T09:53:39Z-a` |

**Correlated friction records:**

| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-17-mcp-server-for-drivers | wrap-session/reconcile | contract.token-proxy-check | cascade.py leaves --apply read three findings as reshaped ('a table cell boundary added or removed') because their replace-with held \| inside backticks on non-table lines (api.md:35, :36, security.md:40); false positive, written by hand without the pipes | retries 3 | .andromeda/runs/2026-10-17T14-19-45-wrap/cascade-dispositions.md · `2026-10-17T14:55:18Z-b` |
| 2026-10-19-dialog-hold-for-the-driver | wrap-session/reconcile | untyped | cascade.py replace reads a `\|` inside a backtick code span on a non-table list line as a table cell boundary and refuses the block as reshaped (obs-plan.md:593 and :596, wake-kind lists written `turn-ended\|question\|...`); the whole call wrote nothing until the two blocks were moved to the Edit tool | retries 1 | .andromeda/runs/2026-10-19T09-28-14-wrap/replace-4.md · `2026-10-19T09:53:39Z-c` |
| 2026-10-19-overseer-switch-to-viola | wrap-session/reconcile | untyped | cascade.py leaves --apply read a list item's replace-with holding a backticked `a\|b` as a table cell boundary and printed reshaped; the finding was written by hand with words in place of the pipe | extra_reads 1 | .andromeda/runs/2026-10-19T19-16-36-wrap/cascade-dispositions.md · `2026-10-19T19:40:54Z-c` |

**Level hypothesis:** `cascade.py leaves --apply` and `cascade.py replace` refused a finding as `reshaped` because a `|` inside a backtick code span on a list line read as a cell boundary: unwrapped (fact), mcp (friction, retries 3), dialog-hold (fact + friction, one event), overseer-switch (friction). Each was routed around by writing the edit by hand or replacing the pipe with words. Problem facts alone are n = 2, under the F-2 floor. The theme reaches four distinct events only by joining the friction stream, so it is surfaced here labelled cross-stream. The handoff carries it for the operator's word. **Level hypothesis:** a parser defect in a pipeline tool, absorbed by hand in each wrap.

**Proposal:** The cell-boundary reading in `cascade.py` skips a `|` inside a code span, or applies only to table lines.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — wrap-session/report (+ reconcile) — 4 cases → proposed type `retry.sweep-pattern-authoring`
| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-13-paste-newline-ledger-row | wrap-session/report | untyped | the site-sweep patterns file was refused twice before it ran (exit 2): a doubled apostrophe inside a TOML literal string (not an escape in TOML), then an inline (?i) flag placed after the first alternative (Python re requires global flags at the start); fixed by a triple-quoted literal and a leading flag | retries 2 | — · `2026-10-13T21:56:23Z-b` |
| 2026-10-14-resume-and-compact-ledger-rows | wrap-session/report | untyped | the site-sweep pattern probe-dirs (-plan/) matched the registries' contracts/{doc}-plan/ paths, 28 of 35 standing rows noise; re-fired anchored on .viola-verify-<pid> | retries 1 | .andromeda/runs/2026-10-16T14-05-30-wrap/cascade-2026-10-14-resume-and-compact-ledger-rows-pre-pass.json · `2026-10-16T14:10:45Z-b` |
| 2026-10-17-cli-machine-contract | wrap-session/report | untyped | a TOML sweep pattern appended through printf landed a doubled backslash (a literal backslash in a TOML literal string, not the regex word boundary); caught by reading the file back and fixed through Edit before the sweep fired | retries 1 | — · `2026-10-17T03:55:28Z-c` |
| 2026-10-17-mcp-server-for-drivers | wrap-session/reconcile | untyped | cascade.py sweep refused the patterns file (exit 2) for a 17-char pattern id over its 16-char bound; renamed and re-fired | retries 1 | .andromeda/runs/2026-10-17T14-19-45-wrap/cascade-patterns.toml · `2026-10-17T14:55:18Z-e` |

**Cluster:** The site-sweep / cascade-sweep patterns file the step authored was refused or misfired on its first firing (TOML literal quoting, an inline regex flag's position, a doubled backslash through printf, a pattern id over its bound, an unanchored pattern matching the registries' `-plan/` paths) and was re-fired. → **draft criteria line:** a sweep patterns file this step authored is refused by the tool, or its first firing reads mostly noise, and is re-authored and re-fired: record the refusal or noise and the re-fires.

### U2 — wrap-session/gates (+ phase/validate) — 2 in-epoch, recurring from Epoch 4 → proposed type `contract.hygiene-self-refusal`
| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-14-resume-and-compact-ledger-rows | wrap-session/gates | untyped | the P7.3c hygiene read refused this wrap's own curation.md, whose log quoted the prose glob $TMPDIR/viola-verify-* it was recording as a hazard; the token was rewritten to a <placeholder> and the re-read was clean | retries 1 | .andromeda/runs/2026-10-16T14-05-30-wrap/curation.md · `2026-10-16T14:51:33Z-c` |
| 2026-10-17-optional-plugin-install | wrap-session/gates | untyped | hygiene P1 refused a leaf-delta diff carrying an example path written into the new layout wireframe (~/.viola, then /h/.viola read as an msys drive); the wireframe took the <home>/marketplace placeholder in the master and the diff | retries 2 | — · `2026-10-18T03:15:41Z-b` |
| 2026-10-10-windows-mutation-grade | phase/validate | untyped | gate.py hygiene refused three files this run had written into its run dir for an in-root home path; gate.py respell fixed two and left the third, whose hit was the bare root path with nothing after it, so it was edited by hand | retries 1 | — · `2026-10-10T02:42:58Z-d` |

**Cluster:** The hygiene read refused a file this run wrote because the file quoted a path as prose or example (a hazard glob being recorded, a wireframe's example home, an in-root home path); the in-epoch typed sibling `2026-10-13T11:05:16Z-b` (recall: a quoted Windows drive path) is the same mechanism. → **draft criteria line:** the hygiene read refuses text this run wrote that quotes a path as an example, a hazard or a record (not a leaked real path): record the refused file, the path form and the re-spelling.

### U3 — phase/take-up — 1 in-epoch, recurring from Epoch 4 → proposed type `tooling.output-clipped`
| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-13-epoch-4-cleanup | phase/take-up | untyped | ci.py conclusion's red row for dffad3e printed its runs field clipped to `nightly#38…` beside red-by jobs `mutants (viola-*)` named without their run, so which run owned the four failed jobs took a gh run view read (they were windows-mutants#38215104120's root shards) | extra_reads 1 | .andromeda/runs/2026-10-13T07-07-28-phase/ci-no-marker.json · `2026-10-13T07:09:23Z-b` |
| 2026-10-09-epoch-3-cleanup | phase/take-up | untyped | ci.py conclusion printed the red row as runs windows-mutants#… with the run id clipped, in stdout and in its trail alike, and names only the first failed check; the fold needs the run id and each failing job verbatim, so one gh api commits/{full sha}/check-runs read supplied them | extra_reads 1 | .andromeda/runs/2026-10-09T15-03-42-phase/ci-no-marker.json · `2026-10-09T15:07:26Z-b` |

**Cluster:** `ci.py conclusion` printed a red row with its run id clipped (`nightly#38…`, `windows-mutants#…`) and the failing jobs without their run, so a `gh` read was needed to own the red. → **draft criteria line:** a pipeline tool clipped a field the step needs whole (an id, a path, a job name), and an extra read recovered it: record the tool, the field and the recovery read.

### U4 — any step (reconcile ×2 in-epoch, gates in Epoch 4) → proposed universal type `tooling.cwd-drift`
| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-13-epoch-4-cleanup | wrap-session/reconcile | untyped | the citation sweep's dry-run output was redirected through a relative path climbing four levels from the run dir, landing one directory above the repository; noticed at once and moved into the session scratchpad | retries 1 | .andromeda/runs/2026-10-13T10-47-05-wrap/citation-dispositions.md · `2026-10-13T11:04:28Z-c` |
| 2026-10-17-optional-plugin-install | wrap-session/reconcile | untyped | a cd into the run dir prefixed a read-only listing command (cd X 2>/dev/null; …) and moved the session cwd for the next calls; the next call cd-ed back to the root | retries 1 | — · `2026-10-18T03:06:09Z-d` |
| 2026-10-11-the-board-viola-list | wrap-session/gates | untyped | the compaction dry-run's output was redirected through a run-dir-relative ../../../../ path that resolved above the repository root, writing a file outside the working tree; it was moved into the scratchpad before the commit | — | — · `2026-10-11T08:55:19Z-b` |

**Cluster:** A path resolved against a working directory other than the one intended: a run-dir-relative `../../../../` redirect landing above the repository (twice, one epoch apart), and a `cd X 2>/dev/null;` prefix that moved the session cwd for later calls (also a `recall.corpus-recurrence`, `2026-10-18T03:06:56Z-b`). → **draft criteria line:** a command read or wrote through a cwd other than the one intended (a relative path climbing out of the tree, a `cd` that persisted): record where it landed and the move back.

### U5 — any step (route-resolve in-epoch, reconcile and fix-loop in Epoch 4) → proposed universal type `contract.unclocked-time`
| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-17-mcp-server-for-drivers | wrap-session/route-resolve | untyped | inputs.py snap refused the resume relay's --origin for a placeholder time (14:3xZ); the invocation's arrival was never clock-read, so the origin was re-worded to name the first clock read after it | retries 1 | viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/inputs/andromeda-inputs.json · `2026-10-17T14:58:42Z-c` |
| 2026-10-10-budget-governor | wrap-session/reconcile | untyped | an input snap's --origin text carried a clock time typed without a read of the clock (the minutes written as a placeholder); the clock read at the snap is in the same call's output, 23:03:18Z | — | viola-0.1.0/chunks/2026-10-10-budget-governor/inputs/andromeda-inputs.json · `2026-10-10T23:13:01Z-f` |
| 2026-10-11-the-board-viola-list | implement/fix-loop | untyped | four times written ahead of or without the clock: three journal lines and one evidence time window were typed, then measured and corrected (one caught by the stamp-ahead hook, three by re-reading the trail and log mtimes) | retries 4 | — · `2026-10-11T07:58:07Z-c` |

**Cluster:** A time was written without a clock read (an `inputs.py snap --origin` placeholder `14:3xZ`, journal lines typed ahead of the clock), refused by the tool or the stamp-ahead hook, or caught on re-read. The in-epoch problem fact `2026-10-13T08:56:20Z-a` (validate: a controls record stamped ahead of the clock, the hook refused it) is the same mechanism. → **draft criteria line:** a time was written into a record, trail or argument without a clock read and was refused or corrected: record where, and what caught it.

### U6 — implement/code — 1 in-epoch, recurring from Epoch 4 → proposed type `tooling.removal-denied`
| chunk | skill/step | type | what | impact | evidence |
|---|---|---|---|---|---|
| 2026-10-16-remove-the-browser-leg | implement/code | untyped | a plan step deleting a whole tracked directory could not be executed in-session: the host denies a recursive rm and the classifier denied the sanctioned move-aside; the deletion waits on the operator | retries 1, deferred 1 | .andromeda/runs/2026-10-16T16-17-45-implement/journal.md · `2026-10-16T16:26:06Z-b` |
| 2026-10-09-epoch-3-cleanup | implement/code | untyped | a compound call holding one rm -r of a just-made bytecode cache dir beside three read-only commands was denied whole by the permission layer, with no word on which part; it was re-issued as separate read calls and the removal replaced by a move to the scratchpad | retries 1 | — · `2026-10-09T16:07:38Z-c` |

**Cluster:** A removal the step needed was denied by the host's permission layer (a recursive `rm` of a tracked dir and its move-aside, at the floor of F-4; the Epoch 4 case is a compound call denied whole for one `rm -r`). The theme also stands in two in-epoch problem facts (`2026-10-16T16:26:06Z-a`, `2026-10-16T21:09:24Z-a`) and in the handoff's desk list of directories left for the founder. → **draft criteria line:** a removal the plan or the step needs is denied by the host or the classifier: record the target, whether a sanctioned move-aside was tried, and where it was left.

## Below threshold — no action

- Typed, below F-2 (as recorded): implement/fix-loop `retry.fix-iterations` 2 · universal `contract.token-proxy-check` 2 (one is L5's mcp case) · universal `tooling.host-shell` 2 · implement/code `tooling.hook-friction` 2 (one is a rustfmt-reflow anchor miss, not a hook) · phase/plan `retry.synthesis-rework` 2 · reconcile `ambiguity.playbook-no-match` 2 · `recall.change-reconstruction` 2 · implement/code `input.research-files-wrong` 2 (X1) · new-session/orientation `input.handoff-git-mismatch` 2 (both: post-wrap bookkeeping read as dirty) · phase/distill `input.spec-source-gap` 2 (one chunk) · curation `ambiguity.tier-routing` 2 · reconcile `contract.false-positive-proposal` 2 · reconcile `contract.proposal-format` 2.
- **Borderline by a missing `step`:** reconcile `contract.false-positive-proposal` reads n = 3 if the step-less wrap record `2026-10-13T18:17:23Z-b` (a D-tests-framework proposal rejected as registry over-reach, evidently reconcile) is folded in; it is kept as recorded. The step-less implement record `2026-10-13T17:51:07Z-b` would make P4 13.
- Typed, n = 1: `recall.curated-rule-not-applied` · validate `ambiguity.review-cycles` · gates `contract.coverage-hold` · route-resolve `contract.carry-no-owner` · fix-loop `ambiguity.scope-pressure`, `contract.instrument-validity`, `contract.vacuous-check-found`, `tooling.gate-deferral` · research `contract.instrument-validity` · reconcile `retry.detector-respawn` · route-resolve `tooling.long-line-edit` (L4), `contract.no-sanctioned-channel` · `tooling.output-cap-overflow` · distill `contract.binding-contradiction`, `contract.extract-format` · take-up `input.carry-context-gap`.
- Untyped clusters under F-4: moved-count readers missed at implement/code, 2 (`2026-10-13T23:18:46Z-b`, `2026-10-16T06:59:33Z-c`; both are `input.research-files-wrong` members filed untyped, part of X1) · L5's two untyped `reshaped` records · a pipeline tool's own argument refused on first call, 4 distinct tools (`gate.py show --plan`, `splice append --lines`, `sidecar.py cites` reading a 0-pending wrap heading as `not a marker`, `gh api` without `--allow-escape-sequences`) · the 0-pending wrap's off-letter Phase 1 on the operator's direction, 2 (one wrap) · a hand-summed research count, 1 (a `contract.narrow-basis-claim` filed untyped).
- Untyped singles: nextest `test(=tests::name)` selecting nothing outside a crate-root module (retries 19) · the leaf snapshot pair broken by `route.py --run-dir` trails (halt 1, dialogue 1) · claude auto-updating 2.1.296 → 2.1.302 mid-chunk (retries 2, dialogue 2) · fixtures with no non-null preview / background_tasks · a fork's recommended option and its preview disagreeing · 14 leaf comparers for a count-and-wording chunk · Run E's cross-directory resume judged from CLI code, unmeasured · a fix waiting on a hook receipt the booted fake agent never writes.
- Problem-fact themes under F-2: `architecture.md` has no primary-language line for `health.py --stack` (2, both orientation, Epoch 4 none; this session's orientation took `rust` the same way) · nextest filters selecting no rstest case (1 fact + its friction) · a `viola` census matching the overseer pair's bridge (1 fact + a recall record) · removed-cause facts (7) form no repeating theme · overridden facts outside L1: operator or founder holds and pauses (4: P4 paused for the founder's machine, a fork held for his own word, the E2E regex widening, the rig outside the repository), the live-start cap raised once, a route-no-marker trail accepted, the overseer-switch profraw ruling (L2).
- Deferred-forever: no hit. All 9 `deferred` facts name a destination (implement's first step, the wrap, the report, the operator pass's CI read, the founder's answer). Two later rode the route: compact's `case_4` red is now a CARRY on "Windows endpoint admission", and overseer-switch's profraw red is a WATCH on "Newer claude CLI rows" (L2).
