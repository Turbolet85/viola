# Evolve Diagnosis — viola-0.1.0 · Epoch 2b — Windows slice I b: events and ledger · 2026-10-01T09:05:15Z

## Mechanism health
- **Records:** 335 in the epoch (171 step / 164 friction), version `viola-0.1.0`, 11 chunks (2026-09-27-epoch-2-cleanup → 2026-09-29-fake-agent-drift-contract) + 26 new-session orientations + 1 chunk-less wrap record (`route-resolve` `2026-09-27T14:03:12Z-a`, the 0-pending adaptation wrap that minted the epoch's head entries). Ledger total 750 lines.
- **Coverage vs expected (phase 5 · implement 3 · wrap 5):** 11/11 chunks complete on every skill; no checkpoint gap. `2026-09-27-hooks-to-normalised-events` carries 6 wrap step records — its reconcile ran twice (halted at E1 on 2026-09-27T23:50:59Z, resumed 2026-09-28T04:37:42Z), designed for a resumed wrap.
- **Unparseable:** 0 · **malformed `ts`:** 0 (whole ledger) · **id fill:** 335/335.
- **Skill-field fold:** 84 of 335 epoch records carry the `andromeda-` prefixed skill name (mixed producers); all were folded to the bare form before grouping, so no phantom chunks.
- **Outcomes:** ok 158 · halted-resolved 9 · ok-degraded 4. 8 of the 9 halts are `wrap-session/reconcile` (7 of 11 wraps; hooks-to-normalised-events halted twice), all `escalation-shaped` / `escalation-open` and resolved in 1–2 dialogue rounds; the 9th is the sideloaded-conpty `route-resolve` (owner-less follow-ups).
- **Retraction pre-pass (whole ledger):** retracted 1 record (`2026-09-25T13:23:53Z-b`, Epoch 2) · 1 problem-fact (`2026-09-25T15:19:15Z-a` index 4, Epoch 2) · clause-retracted 3 (kept, all Epoch 2 targets) · 0 retraction-of-retraction · 0 in-epoch records excluded. Unresolvable (`id: null`, reported verbatim for manual discount, both Epoch 2):
  - `2026-09-27T07:06:36Z-b`: "discount the signal carries-pinned-7 and the note '7 CARRYs' in step record 2026-09-27T06:40:33Z-a: 8 CARRYs were pinned (:40 x4, :43, :65, :67, :73); a step-record signal has no…"
  - `2026-09-27T12:58:32Z-a`: "discount the ts of step records 2026-09-27T13:02:00Z-a and 2026-09-27T13:10:00Z-a the same way; their content stands"
- **Untyped rate per step (untyped / friction / step-runs):** implement/fix-loop 5/22/11 · implement/code 3/20/11 · wrap/reconcile 3/31/12 · phase/research 3/9/11 · phase/take-up 2/7/11 · wrap/gates 2/7/11 · phase/distill 1/12/11 · new-session/orientation 1/5/26 · every other step 0. Overall 20/164 (12 %). **Typing gap:** 6 of the 20 untyped records fit a type their own step already lists (467 and 500 → `tooling.hook-friction`; 512 and 645 → `contract.cascade-miss`; 726 → `contract.token-proxy-check`; 416 → `ambiguity.ladder-uncovered-state`).
- **Problem-fact fill:** 42/171 step records carry facts (55 facts: workaround 36 · overridden 9 · deferred 6 · removed-cause 2 · prohibition 1 · unresolved 1).
- **Calibration boundaries in range:** none bite. The epoch starts on 2026-09-27, so the deviation scan, the `id`/`retracts` schema, every Universal type and reconcile's `contract.in-pass-correction` (deployed 2026-09-27) are all live across the whole range.

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 13 cases · weight 28
**Pattern:** in 10 of 11 chunks the P4 plan reached P5 with mechanical defects that P5's checks then forced resolved. Check 4 (6) (an acceptance criterion naming a probe, census or gate that no entry runs) fired in 7 chunks, 4 (9) (an entry with no prior plan, not marked `new`) in 5, and 4 (4) (artifact keys) in 2. Check 6 (plan size) warned twice and check 5 (placeholders) fired twice.
**Evidence:** ALL 13 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | P5 REQUIRED-RESOLUTIONs on the P4 plan: check 9 (research Platform issues slot read none beside a scope runner-only bullet; planlint's CI-read arm exempts operator reads and leaves the bullet to the letter, so the authoring self-check printed 0 hits), check 4 (9) (2 filter entries aimed at never-named scopes lacked new = true), check 4 (6) (4 criteria named a TEMP find, cargo check, git ls-files and a grep census with no entry), check 5 (a {feature}/{target} placeholder in step 14) · `2026-09-27T14:40:04Z-b` | extra_reads 3, reformulations 6 | `.andromeda/runs/2026-09-27T14-05-04-phase/` |
| 2026-09-27-browser-verdict-reachability | check 4 (6) REQUIRED-RESOLUTION: 7 acceptance criteria named gates or content absent from the P4 fence (health check 11, config/spec/stub/tsconfig greps, ci.yml gate line, wsl-exec census); check 4 (4) needed artifact keys on run --browser and code-graph refresh; check 6 WARN at 631 then 665 lines · `2026-09-27T18:50:05Z-b` | iterations 1 | — |
| 2026-09-27-hooks-to-normalised-events | check 6 size warned: plan grew from 567 to 625 lines (over the 600 line) through the review rulings and carried items on the head of a six-CARRY split entry; operator accepted · `2026-09-27T21:46:36Z-c` | — | — |
| 2026-09-28-hook-perf-gate | check 5 placeholder probe hit a code span split across two plan lines (a BootOptions struct literal), not a template placeholder; reworded to single-line spans, re-probed 0 · `2026-09-28T05:40:30Z-b` | iterations 1 | — |
| 2026-09-28-cli-output-tokens | check 4 (6) fired twice (a criterion named a grep with no entry; the step-6 guard runs lacked one-shot prose) and check 4 (9) once (run --mutants --file src/cmd/run.rs unmarked new, 0 prior plans name it); all resolved in place before review · `2026-09-28T09:03:42Z-b` | extra_reads 1 | `viola-0.1.0/chunks/2026-09-28-cli-output-tokens/plan.md` |
| 2026-09-28-capability-ledger-and-viola-verify | check 2 fired on an Expected-amendments line in the matrix#{id} ledger-note form aimed at a planned (not verified) cap (v1-34), and check 4 (9) found entry 10 (binary(=contract_fixture_hygiene)) named by no prior plan yet unmarked new; both resolved in place before review (note moved to P5.4 matrix.py note; new=true + green baseline) · `2026-09-28T10:52:09Z-b` | extra_reads 1 | — |
| 2026-09-28-mutation-testing-to-the-epoch-boundary | check 4 (6) fired on the P4 plan: two acceptance criteria (nextest bounds unchanged, no product change) named properties no gate entry proved; resolved by adding git diff --quiet probes before the review · `2026-09-28T20:14:35Z-b` | iterations 1 | — |
| 2026-09-29-h2-conpty-resize-probe | P5 checks 4(2), 4(6) and 4(9) required resolutions the P4 authoring missed: no boot smoke beside pump.rs in the modify-set, two arch criteria (ci.yml byte-restore, Cargo.toml unchanged) with no gate entry, and the package-filtered unit entry not flagged new; all resolved by plan edits before the review · `2026-09-29T05:22:08Z-b` | iterations 1 | — |
| 2026-09-29-h2-conpty-resize-probe | the operator's review added a condition no predicate attempts: on the document branch the gotchas entry must state that the product window (a human key right after a resize) remains with its measured rate, so the reshaped test does not hide the product risk; folded into step 12 and its criterion, full check set re-run · `2026-09-29T05:22:08Z-c` | dialogue_rounds 1, iterations 1 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | check 4 (9) baselines showed three run --filter entries could never pass: with no layer selector the harness runs the filter through the unit layer (0 selected, nextest-exit-4), and binary(contract_ledger_probes) is E2E-only while run --e2e is unbuilt; fixed to --integration and the default run · `2026-09-29T07:13:56Z-b` | retries 1 | `.andromeda/runs/2026-09-29T06-43-45-phase/baseline/` |
| 2026-09-29-verify-stamped-test-homes-and-harness | check 4 (6): three acceptance criteria named censuses (stamps.json writers, --local-live in ci.yml, StampError) no gate entry ran; added as probe entries with baselines and known-positive controls · `2026-09-29T07:13:56Z-c` | extra_reads 1 | `.andromeda/runs/2026-09-29T06-43-45-phase/baseline/control/` |
| 2026-09-29-verify-stamped-test-homes-and-harness | check 8 mechanism reach: plan step 3 credited Wrapper::boot's --cli-version with cli_verified, but run's version gate calls the child with a bare --version (version_gate.rs:107-122), so the fake agent's default decides; step 3 reworded, verify-over-committed-set control recorded · `2026-09-29T07:13:56Z-d` | extra_reads 2 | `.andromeda/runs/2026-09-29T06-43-45-phase/` |
| 2026-09-29-sideloaded-conpty | check 4 (4) and 4 (6) required resolution on the first P5 pass: a criterion named evidence/h2-with-without.md with no producer prose, and criteria named G4, the coverage ignore regex, the viola-pty dep set, the portable-pty pin and the list captions with no entry; resolved by prose, three probe entries and a by-construction rewording before the review · `2026-09-29T08:50:00Z-b` | iterations 1 | — |
**Proposal:** the epoch-2-cleanup case names the mechanism. The P4 authoring self-check printed 0 hits because planlint's arm exempts what P5's letter then reads. One direction is to run the same 4 (6) / 4 (9) / 4 (4) arms as part of P4's authoring close, so these resolve before P5 instead of costing a P5 reformulation round in nearly every chunk. Another is a tool arm that pairs each acceptance criterion's named probe with a gate entry. P5 keeps the judgment checks (8, the review); only the mechanical classes would move left.

### P2 — wrap-session/curation · `recall.corpus-recurrence` — 8 cases · weight 19
**Pattern:** a rule already in the always-loaded or path-scoped corpus recurred anyway, in 8 of 11 chunks. 5 of the 8 are the same rule family in the always-loaded `host-win32.md`: the Bash guard refuses doubled backslashes and file-target heredocs, and documents go through the Write tool. The other three are testing.md's deadline-below-kill-line rule, the orphaned-exe lock, and OnceLock isolation. Grouped by type alone, this is identical to the per-step group, since every case was recorded at curation.
**Evidence:** ALL 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | (wrap-session/curation) testing.md's 2026-09-24 entry (extended 2026-09-27: a test deadline must sit below the nextest kill line) already stated the rule; the watched viola-pty test kept CHILD_WITHIN 10 s equal to the mutants profile 10 s kill and lost its dump in the 16:02Z operator pre-push red · `2026-09-27T17:40:03Z-b` | iterations 1 | `viola-0.1.0/chunks/2026-09-27-epoch-2-cleanup/evidence/pty-watch-recorder.md` |
| 2026-09-27-browser-verdict-reachability | (wrap-session/curation) an orphaned viola-fake-agent.exe from a red integration test locked target/harness and failed the boot smoke's relink (os error 5), the class host-win32.md 2026-09-24 and verification-harness.md's Drop-guard rule already record · `2026-09-27T20:12:54Z-b` | iterations 1 | — |
| 2026-09-27-hooks-to-normalised-events | (wrap-session/curation) the report's cargo test vs nextest capture_global finding deduped against testing.md's 2026-09-24 OnceLock isolation entry, which already stated the rule; the implement work reproduced the failure anyway · `2026-09-28T04:38:48Z-b` | — | `.andromeda/runs/2026-09-27T23-42-19-wrap/curation.md` |
| 2026-09-28-hook-perf-gate | (wrap-session/curation) host-win32.md already routes documents through the Write tool, yet a cat >> heredoc document write was attempted at implement's operator pass and refused by the Bash guard; the Edit tool carried it · `2026-09-28T08:02:00Z-b` | retries 1 | — |
| 2026-09-29-h2-conpty-resize-probe | (wrap-session/curation) host-win32.md Transports states documents and scripts go through the Write tool; a python classifier script was sent as a cat > file heredoc anyway, and the Bash guard blocked it · `2026-09-29T06:29:19Z-c` | retries 1 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | (wrap-session/curation) host-win32.md's 2026-09-28 Bash-guard entry (a heredoc carrying a doubled backslash is refused) stood; this session was refused three times, once on exactly that case (an evolve-record heredoc) and twice on non-heredoc commands (a sed, a regex argument) · `2026-09-29T08:08:31Z-b` | retries 3 | — |
| 2026-09-29-sideloaded-conpty | (wrap-session/curation) the report's host mechanics met the doubled-backslash Bash-guard refusal again (any command carrying one), past host-win32.md Session Additions 2026-09-28 which states the rule correctly; logged to the handoff as recurrence-despite-learning · `2026-09-29T12:42:23Z-b` | retries 1 | `.andromeda/runs/2026-09-29T12-17-33-wrap/curation.md` |
| 2026-09-29-fake-agent-drift-contract | (wrap-session/curation) host-win32.md Session Additions 2026-09-28 (extended 2026-09-29) states that the Bash guard refuses any command carrying a doubled backslash; this session still composed three such commands (an implement hygiene path regex, a hygiene drive-path grep, the reconcile evolve JSON) and each was refused and rerouted through a Write-tool file · `2026-09-29T14:51:22Z-b` | retries 3 | `.andromeda/runs/2026-09-29T14-38-17-wrap/curation.md` |
**Proposal:** the Tier-2 entry for the guard was extended twice during the epoch (2026-09-28, 2026-09-29) and recurrence continued after each extension. That suggests a written rule is the wrong carrier for this class. The direction is mechanism over memory: carry the reroute in the guard's own refusal text (see L1). For testing.md's deadline rule, a project-side lint or test is one option: assert each test deadline constant sits below the nextest kill line. This would move the rule from recall to a check.

### P3 — wrap-session/reconcile · `contract.false-positive-proposal` — 8 cases · weight 16
**Pattern:** detector proposals were rejected or narrowed at apply in 7 chunks. Three mechanisms recur, and one record carries two:
- edits inside verbatim upstream copies (security Threat Model, obs-plan §1): 2 cases.
- claims the report does not carry (a ratification time, a mutation-ruling date, ConPTY mechanism claims, `src/` line numbers): 4 cases.
- registry over-reach (submodules or per-session files as their own rows): 2 cases.

One more case named the audit instrument (`cargo-machete`) in the architecture §Stack.
**Evidence:** ALL 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | D-arch-decisions proposed naming cargo-machete in arch §Stack; rejected: it is the code audit instrument (code-metrics commands.dead), run only inside the audit, the chunk only annotated a false positive · `2026-09-27T17:37:57Z-b` | — | — |
| 2026-09-27-epoch-2-cleanup | D-arch-resources proposed listing the split submodules in the arch project tree, which tracks crates at one comment per crate; the submodule half rejected as registry over-reach, the wsl-exec.sh half applied · `2026-09-27T17:37:57Z-c` | — | — |
| 2026-09-27-browser-verdict-reachability | D-security-deps proposed two amendments inside security-plan's Threat Model Summary, a verbatim threat-assessment copy the playbook excludes; rejected, facts land in §Dependency Security · `2026-09-27T20:12:05Z-b` | reformulations 2 | — |
| 2026-09-28-hook-perf-gate | two proposals narrowed at apply: the security Decisions Log entry named only the 06:21 ratification (re-derived to 06:21 for the seam and 09:52:07 for seam plus G2 exemption), and arch A6 registered per-session payload files as their own row (folded as one clause, registry over-reach) · `2026-09-28T08:01:06Z-b` | reformulations 2 | `.andromeda/runs/2026-09-28T07-37-52-wrap/fanout-results.md` |
| 2026-09-28-mutation-testing-to-the-epoch-boundary | D-tests-framework's §1 and §12 proposals dated the mutation ruling 'founder ratification 2026-09-27'; the ruling is the founder's 2026-09-28 17:59 (scope.md CARRY 1) — narrowed on apply · `2026-09-28T21:21:11Z-c` | — | — |
| 2026-09-29-h2-conpty-resize-probe | D-arch-decisions' [PTY] proposal carried two claims the report does not ('does not trigger the own-ConPTY swap', 'viola has no DSR reply to send'); narrowed at apply, and D-tests-coverage's trailing no-retry clause dropped as a §10 restatement · `2026-09-29T06:27:43Z-b` | extra_reads 1 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | D-obs-instrumentation proposed two edits inside obs-plan section 1 (lines 120 and 400), the verbatim obs-scope copy; both rejected under the playbook Verbatim scope copy rule, the fact carried in sections 4, 6 and 12 · `2026-09-29T08:07:03Z-b` | reformulations 2 | — |
| 2026-09-29-sideloaded-conpty | two detector primaries (D-arch-decisions [Session Liveness], D-obs-instrumentation §4) cited src/cmd/run.rs line numbers the report does not carry - rejected as the re-derivation tell with their dependent groups (arch 2, obs 6), their facts re-raised by the orchestrator from the report as R1/R2 · `2026-09-29T12:40:29Z-b` | reformulations 2 | `.andromeda/runs/2026-09-29T12-17-33-wrap/fanout-results.md` |
**Proposal:** two of the three mechanisms are mechanically detectable after the fan-out returns:
- A proposal whose target line falls inside a declared verbatim-copy range, or one that cites a `src/` path or line absent from `report.md`, could be auto-flagged as `rejected-by-rule` before the orchestrator reads it. The `re-derivation tell` is already named in the sideloaded case.
- Handing the detectors the protected line ranges in their prompt is the upstream half.
Registry over-reach stays a judgment.

### P4 — wrap-session/reconcile · `contract.in-pass-correction` — 6 cases · weight 14
**Pattern:** reconcile's own first writes were corrected before commit in 5 chunks, on 3 recurring mechanisms:
- writes into a protected section: obs-plan §1 verbatim copy, test-plan §12 Decisions Log history.
- claims beyond the report: the security log's SQOS claim, a layout wireframe line, a conpty refusal lean, a CI step order.
- a count or grep result stated before it was run (30 vs 31; a "0-hit" grep that returned 2).
**Evidence:** ALL 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-browser-verdict-reachability | two first writes corrected before commit: fanout-results said test-plan 30 proposals (31; caught by re-reading the list) and the lazy-created design-system sidecar carried a preamble paragraph (sidecar.py check OFF-FORM: 2 blocks) · `2026-09-27T20:12:05Z-d` | retries 2 | — |
| 2026-09-27-hooks-to-normalised-events | two first writes corrected before commit: obs-plan.md:419 edited inside the §1 verbatim scope copy (caught while writing the sidecar Section line, reverted); the security Decisions Log entry claimed the hook opens over the SQOS client open, a fact the report does not carry (caught on re-read, rewritten to Client::connect_by + notify) · `2026-09-28T04:37:42Z-c` | reformulations 2 | `.andromeda/runs/2026-09-27T23-42-19-wrap/cascade-dispositions.md` |
| 2026-09-28-capability-ledger-and-viola-verify | the layout-templates verify wireframe was first written with an invented [02/06] spine-hooks step line whose row words the report never carried; a re-read of the edit caught it and the line was removed · `2026-09-28T18:35:39Z-d` | reformulations 1 | — |
| 2026-09-29-h2-conpty-resize-probe | fanout-results.md check 6 first stated a 0-hit grep for the disproved-claim wording; the grep, run after writing, returned 2 a11y-plan hits (:530, :757, an unrelated UI state), and the record was corrected to list and disposition them · `2026-09-29T06:27:43Z-c` | extra_reads 2 | — |
| 2026-09-29-sideloaded-conpty | the seeded-home carve-out was first also written into test-plan:1735, a line inside the 2026-09-24 entry of the section-12 Test Decisions Log (history); caught by the section lookup while writing the sidecar Section field; reverted and landed as a new 2026-09-29 Log entry · `2026-09-29T12:40:29Z-c` | iterations 1, extra_reads 2 | `.andromeda/runs/2026-09-29T12-17-33-wrap/cascade-dispositions.md` |
| 2026-09-29-sideloaded-conpty | two first writes narrowed on re-read: security-plan Code-bearing artefacts said conpty.dll is refused unless OpenConsole.exe verified (a plan lean, not in the report) - removed; architecture CI/CD said the vendor step runs first in the test job - corrected to before the coverage run · `2026-09-29T12:40:29Z-d` | iterations 2 | `.andromeda/runs/2026-09-29T12-17-33-wrap/fanout-results.md` |
**Proposal:** the protected-section half shares P3's cause. A pre-write check of the target line against the protected ranges would catch it at the write instead of at the re-read. The 2026-09-29 U35 registry migration moved the Decisions Logs out of the master bodies, so that half may already be absorbed; next epoch's records would show it. The "stated before run" half is a write-order rule: run the probe, then write the record line.

### P5 — implement/code · `input.plan-step-ambiguous` — 6 cases · weight 13
**Pattern:** in 6 chunks, plan steps were underdetermined or self-inconsistent, and implement settled them as reportable deviations:
- a file-size target the listed moves could not reach.
- code with no observable effect, against testing.md 2026-09-24.
- a feature spelling differing from its sibling build.
- capture-file and error-type shape left open.
- a loop command selecting fewer tests than the step's prose.
- test-home placement contradicting the neighbouring tests.
**Evidence:** ALL 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | step 6 moved exactly the listed base-resolution items, leaving run/mutants.rs at 733 tokei lines vs its <=700 target; after steps 7/9/11 tests it read 820, so the diff classifiers went to base.rs and the per-leg verdict to a new mutants/leg.rs (a file the plan does not list) · `2026-09-27T15:08:04Z-b` | iterations 2, extra_reads 1 | — |
| 2026-09-27-hooks-to-normalised-events | plan step 9.3 (a new hook-safe init fn beside viola_obs_init) and step 6 (HookEvent::is_spine) each ask for code with no observable effect in this chunk, against testing.md 2026-09-24; both settled as deviations for the report · `2026-09-27T22:07:16Z-c` | reformulations 2 | — |
| 2026-09-28-hook-perf-gate | step 10.2's feature spelling (fake-agent over --workspace) differs from the sibling boot build's viola/fake-agent; took the sibling's and surfaced it for the P4 report · `2026-09-28T05:50:39Z-b` | reformulations 1 | — |
| 2026-09-28-capability-ledger-and-viola-verify | two plan letters were underdetermined and settled in the impl, to be surfaced in the report: capture file k per event vs across events (taken across events, so the spine-hooks order check can read arrival order), and StampError as 'a thiserror variant of AgentError' (taken as its own thiserror enum in ledger.rs, AgentError untouched) · `2026-09-28T11:13:10Z-c` | reformulations 2 | — |
| 2026-09-29-h2-conpty-resize-probe | plan step 8 says the loop runs 'the viola-pty resize tests' but its exact command selects only the red test; followed the exact command, and the step left the job's JUnit overwrite and the runner temp mapping unaddressed (both added to the loop) · `2026-09-29T05:30:47Z-b` | extra_reads 3 | — |
| 2026-09-29-sideloaded-conpty | plan step 8 puts every Windows case in real homes under target/e2e-home, while the neighbouring pin_exe unit tests use tempfile::tempdir; the viola-state and viola-pty unit cases followed the neighbours, the root integration cases use target/e2e-home; surfaced in the report · `2026-09-29T09:03:34Z-c` | extra_reads 1 | — |
**Proposal:** two classes look checkable at P5:
- a step whose deliverable has no consumer or test in the chunk (the hooks case is a testing.md rule the plan itself violated).
- a step whose prose scope and exact command select different sets (h2).
Checks for these would sit beside P5's check 8. The rest are authoring precision that the report already carries.

### P6 — implement/fix-loop · `contract.test-expectation` — 6 cases · weight 13
**Pattern:** in 6 chunks an existing test pinned the behaviour the chunk changes and went red in the fix-loop, then got updated as a companion. 3 of the 6 pin a harness selector as "unbuilt usage": `run --browser`, `run --perf`, and h2's `run --e2e` neighbour. The other three pin role-line counts and indices, `cli_verified false`, and `pty_backend "conpty"`.
**Evidence:** ALL 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | two existing whole-document assertions on the mutants part (stale-outcomes tests) went red on the Windows host once scratch_bytes joined the counted document; expectations now add it per host · `2026-09-27T15:48:15Z-c` | iterations 1 | — |
| 2026-09-27-browser-verdict-reachability | viola-e2e::cli unbuilt_selectors_and_unknown_commands_are_usage asserted run --browser exits 2 usage; the chunk builds that selector, red in integration and in the scoped mutants baseline · `2026-09-27T19:29:07Z-b` | iterations 1 | — |
| 2026-09-28-hook-perf-gate | crates/viola-e2e/tests/cli.rs pinned run --perf as an unbuilt usage selector; once built, the test ran a real perf build, boot and hyperfine inside nextest and read exit 0; swapped to the still-unbuilt --e2e and recorded as a companion · `2026-09-28T06:59:44Z-b` | iterations 1 | `gate 11, first run` |
| 2026-09-28-capability-ledger-and-viola-verify | adding the version-probe start/exit pair to run's role file broke two pre-existing tests that pin role-line counts and indices (tests/run_cli.rs 7 cases, tests/contract_diag_schema.rs 1 sum); the diag-schema one also aborted four mutants baselines and the pre-push · `2026-09-28T13:10:21Z-b` | iterations 2 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | cli_instance_state path1_start_writes_state_before_the_spawn asserted cli_verified false on booted_wrapper, which plan step 6 listed as keeping its assertions; the now-stamped fixture reads true (red on the Windows run and the pre-push Linux leg), oracle moved to true · `2026-09-29T07:41:36Z-b` | iterations 1 | — |
| 2026-09-29-sideloaded-conpty | tests/run_cli.rs pinned pty_backend "conpty" on Windows; the chunk changes the value to conpty-sideload, updated as a companion · `2026-09-29T11:38:38Z-c` | iterations 1 | — |
**Proposal:** this is the downstream end of chain X1 and level theme L2. Research's Files-to-modify does not sweep tests for literals pinning the surface being changed. The direction is in L2. Project-side, the "unbuilt selector" usage test in `crates/viola-e2e/tests/cli.rs` turns red every time a selector gets built (3 times this epoch). A project note could name it as a standing companion for any chunk that builds a selector.

### P7 — wrap-session/reconcile · `contract.cascade-miss` — 6 cases · weight 8
**Pattern:** in 6 chunks, stale restatements that no detector proposed were caught late: by the step-2 sweep, a hand read, a leftover-token grep, or while applying another amendment. Two more untyped records of the same class sit in the epoch (512: an obs doc-agent returned `[]` while naming four stale sites; 645). The previous epoch carried the same shape untyped ("the fan-out missed four stale restatements").
**Evidence:** ALL 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | the first A3 body text claimed both operator pre-pushes went green with rust-analyzer running; the first was red (the viola-pty watch baseline); caught on re-read before the sweep and restated as three pre-pushes and two scoped runs with no stage failing on it · `2026-09-27T17:37:57Z-e` | iterations 1 | — |
| 2026-09-27-browser-verdict-reachability | three standing restatements no detector proposed were caught by the step-2 sweep and amended in the pass: a11y-plan §9 layer-table gate and npx invocation, security-plan 'One other launcher exists' beside the new root launch · `2026-09-27T20:12:05Z-c` | extra_reads 2 | `.andromeda/runs/2026-09-27T19-50-23-wrap/cascade-dispositions.md` |
| 2026-09-28-cli-output-tokens | obs-plan §3 :556 and §11 :1392 named the .cmd/.bat refusal as run's only pre-spawn stderr site (with a local #[allow]) although earlier chunks had added the live/stale/tampered/squatted refusals through one allow-free helper; arch :426 had been updated, obs had not — a prior pass's cross-master restatement survived until this chunk re-sited the writer · `2026-09-28T09:55:16Z-b` | — | `.andromeda/runs/2026-09-28T09-46-16-wrap/fanout-results.md` |
| 2026-09-28-capability-ledger-and-viola-verify | architecture.md VIOLA_NAME env line (its absence makes every hook a silent exit 0) restated the retired hook-contract claim; the name-absent pattern's 50-char window missed it and a hand read beside the sweep caught it after cascade-dispositions first said no change · `2026-09-28T18:35:39Z-c` | reformulations 1 | — |
| 2026-09-28-mutation-testing-to-the-epoch-boundary | two stale restatements no detector proposed stood in test-plan: :645 'A leg name outside the session-id charset is usage invalid-leg' and :1273 (§6) 'except the mutation leg's chunk.diff'; the leftover-token grep caught :645, the cascade sweep's mutleg pattern caught :1273 · `2026-09-28T21:21:11Z-b` | extra_reads 2 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | two test-plan restatements no detector proposed: the section 5 H2 row naming the real-CLI verify entry (raised at Validate check 5 from the report's hit table) and the gate --require 'nine closed suite values' line (found while applying T7) · `2026-09-29T08:07:03Z-c` | extra_reads 2 | — |
**Proposal:** the misses are restatements of a claim the chunk retired. One direction derives the cascade sweep's leftover-token patterns mechanically from the report's retired or changed claims, instead of hand-authoring them per wrap. The capability case names the instrument limit: the name-absent pattern's 50-char window was too narrow.

### P8 — implement/code · `tooling.hook-friction` — 4 cases · weight 11
**Pattern:** the PreToolUse Bash guard refused edit commands in 4 chunks: doubled backslashes in heredoc or sed payloads, and file-target heredocs. Each was redone through the Edit tool or a Write-tool file.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | PreToolUse bash-guard blocked 4 commands (1 heredoc with a file target, 3 carrying doubled backslashes); each reformulated · `2026-09-27T15:08:04Z-d` | reformulations 4 | — |
| 2026-09-28-capability-ledger-and-viola-verify | the PreToolUse bash guard blocked a python edit heredoc whose Rust payload held an escaped backslash; redone as seven Edit-tool calls · `2026-09-28T11:13:10Z-d` | retries 1 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | bash-guard.py refused a multi-file sed whose run_cli.rs expression carried a doubled backslash; the batch was re-issued without it and that file edited by Edit · `2026-09-29T07:29:54Z-b` | retries 1 | — |
| 2026-09-29-sideloaded-conpty | PreToolUse Bash guard refused a cat heredoc with a file target carrying an edit script; host-win32.md already routes documents and scripts through the Write tool · `2026-09-29T09:03:34Z-b` | retries 1 | — |
**Proposal:** the same obstacle as L1 (which carries 8 problem-facts across 7 chunks), P2's recurrence, and extension candidate U1. See L1 for the direction.

### P9 — wrap-session/curation · `ambiguity.filter-borderline` — 5 cases · weight 7
**Pattern:** in 5 chunks, curation candidates scored exactly at the 0.6 boundary. The conditional "no-other-home" +0.2 alone decided pass or fail, and in one case Filter 1's Jaccard was judged by eye rather than computed.
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | C3 (rg is a function in the interactive Git Bash) scored exactly 0.6 (measured +0.4, detail +0.2) and passed only through the no-other-home conditional +0.2 · `2026-09-27T17:40:03Z-c` | — | — |
| 2026-09-27-browser-verdict-reachability | two candidates landed at exactly 0.6 (the recorder's read order; the install-deps dry-run exit) and rejected by the lean default; each already had a home this wrap (a master body, a route CARRY) · `2026-09-27T20:12:54Z-c` | reformulations 2 | — |
| 2026-09-27-hooks-to-normalised-events | Filter 4 on the Windows ExitCode PartialEq-not-Eq gotcha: 0.3 with the one-off negative, 0.8 without it and with no-other-home; the report line alone could not say whether it was an emphasised finding · `2026-09-28T04:38:48Z-c` | — | `.andromeda/runs/2026-09-27T23-42-19-wrap/curation.md` |
| 2026-09-29-h2-conpty-resize-probe | three candidates scored exactly 0.6 (measured +0.4, technical +0.2) and rejected because this wrap's P2 had amended each into a master, which bars both conditional +0.2 signals; the two survivors reached 0.8 only through the no-other-home signal · `2026-09-29T06:29:19Z-b` | — | — |
| 2026-09-29-fake-agent-drift-contract | the surviving candidate (the local suite grades an identical tree differently, so judge by reverse-order pairs plus alone-rounds) sits beside verification-harness.md's 2026-09-26 mutation entry of the same shape; judged a new entry below Filter 1's 0.7 bar by eye, not by a computed Jaccard · `2026-09-29T14:51:22Z-c` | — | `.andromeda/runs/2026-09-29T14-38-17-wrap/curation.md` |
**Proposal:** the guide could state the boundary explicitly (`≥ 0.6` vs `> 0.6`). Filter 1's similarity could be computed by a tool rather than estimated. The h2 case also shows an interaction: a P2 amendment into a master bars both conditional +0.2 signals. That makes the filter systematically reject what reconcile just landed, which may be the intent but is worth stating.

### P10 — implement/fix-loop · `contract.spec-reality-gap` — 4 cases · weight 8
**Pattern:** in 4 chunks, plan or spec text met tool or product reality in the fix-loop:
- Playwright's `install-deps --dry-run` exits 1.
- a Windows-host scoped mutants entry expecting 0 missed while the killer is `#[cfg(unix)]`.
- `run --e2e`, a selector the harness never built.
- the sideloaded OpenConsole DA1 stall, a product finding.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-browser-verdict-reachability | plan step 11 has --install-deps print the dry-run then install; Playwright 1.63.0 install-deps --dry-run exits 1 while packages are missing, so set -e stopped before the install; fixed in the script with \|\| true · `2026-09-27T19:29:07Z-d` | iterations 1 | `viola-0.1.0/chunks/2026-09-27-browser-verdict-reachability/evidence/wsl-chromium-deps.md` |
| 2026-09-28-cli-output-tokens | plan entry 17 expects a Windows-host scoped run --mutants --file src/cmd/run.rs to read 0 missed, but the diff regenerates refuse_stale whose only killer is #[cfg(unix)] (verification-harness.md 2026-09-25 class); surfaced, operator ruled the pre-push union is the verdict · `2026-09-28T09:31:23Z-b` | dialogue_rounds 1 | `viola-0.1.0/chunks/2026-09-28-cli-output-tokens/evidence/gate-17-surfaced.md` |
| 2026-09-29-h2-conpty-resize-probe | plan gate `bash scripts/agent-run.sh run --e2e` follows test-plan §3's run command body, but the harness never built --e2e (nextest.rs:10-11 defers it to the first E2E binary); exit 2 usage, reproduced on a clean HEAD worktree · `2026-09-29T05:36:30Z-b` | extra_reads 3 | `.andromeda/runs/2026-09-29T05-24-12-implement` |
| 2026-09-29-sideloaded-conpty | the sideloaded OpenConsole.exe sends a DA1 query at start and holds the child ~3 s when nothing answers (piped runs); surfaced as a product finding for the wrap, owner the entry that first runs viola headless · `2026-09-29T11:38:38Z-e` | iterations 1 | `viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/evidence/da1-stall.md` |
**Proposal:** three of the four are plan gate entries that cannot pass as written (see X3). One direction is a P5 dry-run of every entry naming a harness selector against the harness's actual CLI surface. That would have caught `--e2e`. A second is a host-class verdict attribute on entries whose killer is platform-gated (L5).

### P11 — wrap-session/route-resolve · `contract.carry-no-owner` — 4 cases · weight 8
**Pattern:** in 4 chunks, follow-ups had no statically named owner entry:
- the install-deps rework, pinned as a CARRY that moves with the head.
- a controls-table case moved between entries.
- a plan CARRY list addressing owners by working-route line number, where `:62` had become a different entry.
- the DA1 stall and the host reds, settled by an armed question.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-browser-verdict-reachability | the overseer placed the --install-deps rework on 'the next entry that meets a re-provision', an owner no route entry names statically; pinned to the first markerless entry as a carry that moves on until a chunk re-provisions the distro · `2026-09-27T20:14:24Z-b` | reformulations 1 | — |
| 2026-09-28-hook-perf-gate | the controls table's completeness case was first pinned to the dialog entry beside the answer negative, then moved to the home-integrity entry once the wheel and --home negatives were read as owned by later entries · `2026-09-28T08:03:30Z-b` | reformulations 1 | — |
| 2026-09-28-capability-ledger-and-viola-verify | the plan's route CARRY list addressed owners by working-route line number (:62 for the dialog rows) and :62 is wait and last; the owner was placed by subject (Dialog answers by dialog_id) and the pin notes the stale number · `2026-09-28T18:39:37Z-b` | extra_reads 1 | — |
| 2026-09-29-sideloaded-conpty | two in-version follow-ups had no owner by title (the DA1 headless stall; the host's shared local integration reds) - one armed question with leans: DA1 -> First live test and self-drive, host reds -> a CARRY moving with the head; both answered as leaned, the overseer agreeing · `2026-09-29T12:44:36Z-b` | dialogue_rounds 1 | `viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/report.md` |
**Proposal:** two directions:
- Address CARRY owners by entry title, never by `:line`. Line numbers shift with every flip and compaction, as the capability case shows.
- Sanctioning the "moves with the head" CARRY form in the route contract, which this epoch used twice (install-deps, host reds), would make it a designed shape rather than a per-wrap improvisation.

### P12 — wrap-session/route-resolve · `tooling.long-line-edit` — 3 cases · weight 7
**Pattern:** anchored Edits on multi-KB working-route lines failed three times in three chunks:
- a misquoted anchor, whose python fallback's read-back carried the same misquote.
- an anchor matching 3 occurrences while a line-grep counted 1 line.
- an anchor matching twice because CARRY freight is repeated verbatim on another entry.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-29-h2-conpty-resize-probe | retiring the last CARRY block of the 3.2 KB :59 line: the Edit anchor retyped '(overseer ruling' where the line reads '; overseer ruling', so 0 matched; the python fallback wrote correctly but its own read-back assertion carried the same misquote and failed after the write · `2026-09-29T06:31:10Z-b` | retries 2, extra_reads 2 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | an anchored Edit on the end of the Server verification line ('which closes that dated exception') matched 3 occurrences in the route file although a line-end grep counted 1 line; re-anchored with the following separator and entry start · `2026-09-29T08:10:41Z-b` | retries 1 | — |
| 2026-09-29-fake-agent-drift-contract | the anchored Edit appending to :66 matched twice: its line-final sentence (the fuzz-pipeline CARRY) is freight repeated verbatim on another entry; recovered by probing a longer anchor with grep -c = 1 and re-firing · `2026-09-29T14:53:01Z-b` | retries 1 | `.andromeda/runs/2026-09-29T14-38-17-wrap` |
**Proposal:** CARRY appends and retirements are structured operations on a documented grammar. A `route.py` verb that appends or retires a CARRY by entry title plus CARRY text, with its own uniqueness assertion, would replace hand-anchored Edits on the longest lines in the repo. The fake-agent case also shows freight duplicated across entries, which is a route-hygiene observation.

### P13 — wrap-session/report · `input.implement-outcome-unsettled` — 4 cases · weight 4
**Pattern:** in 4 chunks, implement's green P4 report was superseded by the operator pass: pre-push reds, CI reds folded, more commits. The report's outcome basis became `evidence/operator-pass.md` plus the final CI run, not implement's P4.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | implement reported green and handed off to the operator pass; that pass's first pre-push went red on the watched viola-pty test, a recorder fold (test code) followed before the green re-run and the pre-CI commit, so the outcome basis is op-35b + ci#36333711860, not implement's P4 · `2026-09-27T17:24:01Z-b` | — | `viola-0.1.0/chunks/2026-09-27-epoch-2-cleanup/evidence/operator-pass.md` |
| 2026-09-28-capability-ledger-and-viola-verify | implement's green was superseded by the operator pass: CI reds after the first push were folded (a private CARGO_HOME for the harness mutants tests; raw panic frames) across 5 more commits; the outcome basis is the final HEAD's CI run ci#36460408121 recorded in evidence/operator-pass.md · `2026-09-28T18:13:40Z-b` | extra_reads 2 | `viola-0.1.0/chunks/2026-09-28-capability-ledger-and-viola-verify/evidence/operator-pass.md` |
| 2026-09-28-mutation-testing-to-the-epoch-boundary | implement's P4 report was not the final word: the operator pass ran after it (measurement push, macOS runner-side arm, ENOTCONN fold); the outcome basis is the pass's final HEAD 17b93c7 and ci#36483042659 · `2026-09-28T21:07:44Z-b` | — | `viola-0.1.0/chunks/2026-09-28-mutation-testing-to-the-epoch-boundary/evidence/operator-pass.md` |
| 2026-09-29-sideloaded-conpty | implement's P4 report predates the operator pass: a CI red (ci#36563179341), its fix commit 224efc4 and the H2 removal 8f643f2 came after it; the report's outcome basis is the pass's evidence files and ci#36566391084 · `2026-09-29T12:20:27Z-b` | extra_reads 1 | — |
**Proposal:** see X2, where the same shape appears in 5 chunks. The operator pass is a recurring post-implement stage, and it produces no step record. One direction is to make it a named artifact in the contract: `evidence/operator-pass.md` with a fixed head (final HEAD, CI run id, folds), which report reads first by contract. Another is to give it its own evolve checkpoint.

### P14 — phase/take-up · `input.carry-context-gap` — 4 cases · weight 4
**Pattern:** in 3 chunks, CARRY text was stale or incomplete at take-up:
- a platform constraint went unnamed (linux-only browser step).
- a CARRY named "a row" in a test file that does not exist.
- a consumer grep missed the red's own test.
- a function was placed in the wrong file.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-browser-verdict-reachability | the W125 CARRY says Playwright on three CI OSes but does not name that test-plan run step 3 makes the browser step linux-only (browser-linux-only exit 2) and the ubuntu-only E2E job; the conflict surfaced only by a test-plan grep · `2026-09-27T18:23:05Z-c` | extra_reads 1 | — |
| 2026-09-28-hook-perf-gate | CARRY 2 asks for 'a cli_controls_not_disableable.rs row' but no such test file exists at HEAD; test-plan specifies it as the whole Vector 6 table, so create-vs-row is left an inferred premise for P3 · `2026-09-28T05:16:37Z-b` | extra_reads 2 | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | CARRY 1's consumer grep (stamped_home\|booted_wrapper, '5 root test files') lists 4 root files + 2 support modules; StampedHome\|Wrapper::boot adds cli_version_gate, hook_events, hook_fail_open — the red's own test was outside the named set · `2026-09-29T06:46:21Z-b` | extra_reads 1 | `viola-0.1.0/chunks/2026-09-29-verify-stamped-test-homes-and-harness/scope.md item 1` |
| 2026-09-29-verify-stamped-test-homes-and-harness | CARRY 5 places run_bounded in src/cmd/verify.rs; it is defined at src/run/version_gate.rs:39 and shared with run's version gate · `2026-09-29T06:46:21Z-c` | extra_reads 1 | `viola-0.1.0/chunks/2026-09-29-verify-stamped-test-homes-and-harness/scope.md item 7` |
**Proposal:** CARRYs could name their subjects by symbol, or carry the probe that located them (the grep or graph query), rather than by a path or a file presumed to exist. Take-up's re-grounding then becomes re-running the probe. All 4 were caught at take-up at a cost of 1–2 extra reads each.

### P15 — phase/distill · `contract.binding-contradiction` — 4 cases · weight 4
**Pattern:** in 4 chunks, distill's check C found the plans themselves contradicting each other, with every extract faithful to its source:
- `jq -e` vs the harness gate for the perf verdict.
- the perf job shape and artifact name.
- the output escape set (CR/DEL).
- whether the verify summary goes to stdout or stderr.
Each was carried to synthesis and resolved there for that chunk.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-hooks-to-normalised-events | obs extract requires the perf max asserted by `jq -e` (obs-plan §10) while test-plan §9 says the perf verdict comes from viola-harness gate, never jq, and CI neither installs nor pins jq; a spec-source contradiction between two plans, carried to synthesis, no retry · `2026-09-27T21:22:45Z-b` | — | — |
| 2026-09-28-hook-perf-gate | check C: the test-plan and obs-plan extracts faithfully carry contradictory spec text — perf job shape (own per-OS job vs step inside test) and perf artifact name (target/agent-run/artifacts/perf-<hook>.json vs perf/hook-<event>.json, arch registry perf/*.json); spec-level, noted to synthesis, no retry · `2026-09-28T05:22:41Z-b` | — | — |
| 2026-09-28-cli-output-tokens | check C soft disagreement: tests extract states the output escape set as C0 except LF/CR/TAB (the paste-input set), design/layouts/security keep only LF and TAB (CR escaped); a11y names DEL escaped, security leaves DEL/CR open; noted to synthesis, no retry · `2026-09-28T08:50:44Z-b` | — | `.andromeda/runs/2026-09-28T08-40-00-phase/` |
| 2026-09-28-capability-ledger-and-viola-verify | check C: obs extract (obs-plan §4 Edge flows verify) puts the `stamped <ver> N pass N fail` summary on stderr; design, layouts and test-plan :518 put it on stdout — a spec-level conflict, carried to P4 synthesis, no retry (the extracts report their plans faithfully) · `2026-09-28T10:30:34Z-b` | — | — |
**Proposal:** a contradiction resolved only in a plan's synthesis stays in the masters for the next chunk's distill to meet again. One direction routes each distill-found cross-plan contradiction to the chunk's report as an Expected amendment, so reconcile converges the masters. The P4 `authority-resolved:` signals already record which side won.

### P16 — Universal · `contract.token-proxy-check` — 5 cases · weight 5 (per-step: phase/distill 3 · phase/validate 1 · wrap-session/report 1)
**Pattern:** probes matched a token where the property was semantic, in 4 chunks:
- 3 orchestrator-authored H2 cite probes keyed on the whole bold span, so cites with a parenthetical or em-dash disambiguator read unresolved.
- planlint 4 (5) passed on an atom-from token whose cited lines had moved.
- a bare `union` hit counted as a mutation-union site.
All were resolved by reading the hits.
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | (phase/distill) two self-written validation probes false-positived: check 3 flagged the obs logging-bans item whose anchors sit on nested sub-items, and H2 keyed on the whole bold title so 6 tests items with shortened titles read unresolved; both cleared by reading the hits (anchors present; every marker occurs in a sidecar heading) · `2026-09-27T14:14:07Z-b` | extra_reads 1 | `.andromeda/runs/2026-09-27T14-05-04-phase/` |
| 2026-09-27-browser-verdict-reachability | (phase/distill) orchestrator H2 probe fed each bold cite verbatim into grep -E; cites carrying a parenthesised or em-dash disambiguator read UNRESOLVED (false negative, 3 cites) until the sidecar headings were read directly · `2026-09-27T18:28:59Z-b` | extra_reads 1 | — |
| 2026-09-27-browser-verdict-reachability | (phase/validate) the pre-push leg atoms copied from the prior plan cited pre_push.rs:193 and :286; planlint 4 (5) passed on the atom-from token while the lines no longer held the tokens (now :104 and :203); false negative, found by check 8's read of the cited lines · `2026-09-27T18:50:05Z-d` | extra_reads 1 | — |
| 2026-09-28-capability-ledger-and-viola-verify | (phase/distill) H2 probe matched the whole bold text against sidecar headings; 8 cites (arch 6, obs 2) carried a parenthetical entry qualifier inside the bold and read unresolved — false positive, truth from grepping the marker heads (all resolve; the markers key several entries each) · `2026-09-28T10:30:34Z-c` | extra_reads 1 | — |
| 2026-09-28-mutation-testing-to-the-epoch-boundary | (wrap-session/report) the expected-amendments site sweep listed a11y-plan :1145 as a mutation-union site from a bare 'union' token hit; reading the line showed 'the union of the per-state axe verdicts' (false positive), corrected before the fan-out · `2026-09-28T21:07:44Z-c` | extra_reads 2 | — |
**Proposal:** the H2 cite check is re-authored by the orchestrator in each phase. Counting P17 and the untyped 726, it misfired in 6 of 11 chunks. One direction is a tool, or an arm of an existing one, that resolves H2 cites by the bare marker token. Each phase's fresh probe has rediscovered that the bold span is not the key. For planlint 4 (5), the arm could re-read the cited line for the token instead of trusting the atom.

### P17 — phase/distill · `contract.extract-format` — 3 cases · weight 4
**Pattern:** in 3 chunks, history agents shortened heading titles or put a parenthetical label inside the bold marker, and check 3 did not say whether a nested `- ` sub-bullet is an item. This is the producer side of P16's H2 misreads.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-hooks-to-normalised-events | security history agent shortened the heading title after the marker on 5 of 16 items; H2 resolves by marker so all passed, but a full-heading match failed on those 5; receipt counts also differed from file (tests receipt 19, file 20) · `2026-09-27T21:22:45Z-c` | — | — |
| 2026-09-29-verify-stamped-test-homes-and-harness | arch and obs history agents put a parenthetical label inside the bold marker (**<marker> (label)**) on 11 items; the H2 cite check keyed on the whole bold span read 11 false unresolved until re-keyed on the marker token · `2026-09-29T06:53:11Z-b` | retries 1 | `.andromeda/runs/2026-09-29T06-43-45-phase/` |
| 2026-09-29-fake-agent-drift-contract | Extracts (arch, a11y, obs, security) carry nested sub-bullets under anchored Constraints/Acceptance items; check 3 does not say whether a nested '- ' is an item, so it was judged a continuation of its anchored parent · `2026-09-29T13:08:32Z-b` | — | — |
**Proposal:** either pin the extract grammar so the marker sits alone in the bold and labels go after it, or key the check on the marker token, as in P16. One of the two is enough. Separately, stating whether a nested sub-bullet under an anchored item is an item or a continuation would remove a recurring judgment call.

### P18 — Universal · `contract.premise-falsified` — 4 cases · weight 6 (per-step: phase/research 2 · implement/code 1 · wrap-session/gates 1)
**Pattern:** verification falsified authored premises in 4 chunks:
- a route CARRY's premise of 11 Windows-only survivors needing kills.
- a plan step's premise that an empty `#[files]` glob compiles.
- a relayed source read that withdrew an already-applied ConPTY mechanism after P2.
- a take-up hypothesis of a pipe-name collision.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | (phase/research) the route CARRY and triage premise of 11 Windows-only survivors needing killing tests was falsified: the code audit's per-crate cargo-mutants ran on the Windows host with no union; 9 of 11 are cfg(unix) code and CI's ubuntu-leg verdict artifacts judged all 11 caught or unviable · `2026-09-27T14:25:31Z-b` | extra_reads 6 | `CI runs 36318398739, 36298052174, 36165685381 mutants-verdict-ubuntu-latest; research.md fact 5` |
| 2026-09-28-capability-ledger-and-viola-verify | (implement/code) plan step 8 states the #[files("fixtures/claude/*/*.json")] glob matching no file is expected at /implement time; rstest_macros 0.27.0 files.rs:635 errors 'No file found' at compile time · `2026-09-28T11:13:10Z-b` | reformulations 1, extra_reads 1 | — |
| 2026-09-29-h2-conpty-resize-probe | (wrap-session/gates) after P2 applied them, the overseer's relayed source read (microsoft/terminal resize path flushes no input buffer; no reported resize input loss) withdrew the 'loss is inside ConPTY / below the seam' mechanism and the stated product window for claude; arch [PTY], test-plan §5, both sidecar entries, gotchas.md, viola-pty.md, the report and the handoff were re-worded before the commit, and an owner CARRY pinned on :74 · `2026-09-29T06:37:38Z-b` | extra_reads 6, retries 1 | `.andromeda/runs/2026-09-29T06-18-44-wrap/fanout-results.md` |
| 2026-09-29-verify-stamped-test-homes-and-harness | (phase/research) the take-up directive's hypothesis (cross-test pipe-name collision) was falsified: the endpoint hashes the home, and the red's kept home in the diag-windows-2025 artifact shows the hook reached its own stopped wrapper's pipe 25 ms after that wrapper's exit line · `2026-09-29T07:01:58Z-b` | extra_reads 1 | `.andromeda/runs/2026-09-29T06-43-45-phase/ci/diag-windows-2025/viola-test-B3yvcs/` |
**Proposal:** the two research cases are research doing its job. The h2 gates case is the costly one: 6 extra reads, 1 retry, and 8 artifacts re-worded before commit. It was absorbed mid-epoch, since CLAUDE.md now carries the Tier-1 learning that a relayed claim is a labelled HYPOTHESIS until an artifact backs it. Whether a pipeline-level generalization exists, such as reconcile refusing to land a relayed mechanism claim without an on-disk artifact, is the founder's call.

### P19 — new-session/orientation · `input.handoff-git-mismatch` — 3 cases · weight 3
**Pattern:** in 3 of 26 orientations, the handoff's Status and Position disagreed with git and the route:
- two wraps stopped after P1 for context (one at 92 %) without rewriting the handoff; the truth was in the run dir's `resume.md`.
- a phase ran after the wrap and did not rewrite the handoff.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| (session) | The handoff's Position Next (phase Wrapper channel) and Status clean disagree with git: the chunk has 4 fix commits, an operator pre-CI commit and untracked wrap artifacts; a wrap paused at P1 at 92% context without rewriting the handoff, and only resume.md in the wrap run dir carries the true position · `2026-09-27T12:36:36Z-b` | extra_reads 2 | — |
| (session) | handoff Status clean + Next '/andromeda-phase' for the capability-ledger chunk disagreed with git (dirty tree, operator pass commits, untracked wrap run dir) and the route (chunk pending); a mid-wrap stop after P1 leaves the handoff unwritten until P7, and the truth came from the wrap run dir's resume.md · `2026-09-28T18:16:19Z-b` | extra_reads 2 | — |
| (session) | handoff Status clean / Position Next /andromeda-phase disagreed with git (7 modified + 2 untracked phase outputs) and route.py cursor (1 pending, promoted): a phase ran after the wrap wrote the handoff and did not rewrite it · `2026-09-29T08:51:29Z-b` | — | — |
**Proposal:** a wrap that pauses could write a one-line interim handoff Status (`wrap paused at P{n} — resume {run_dir}/resume.md`) at its pause point. Alternatively, new-session could read any wrap run dir carrying `resume.md` and no completion marker as the position source. The phase-after-wrap case needs phase to touch the handoff's Position line, or the dashboard to treat that line as advisory beside `route.py cursor`, which is already authoritative. The pause cause itself (context) is listed under Level observations.

### P20 — implement/code · `input.research-files-wrong` — 3 cases · weight 3
**Pattern:** in 3 chunks, research's Files-to-modify omitted files implement had to edit: shared pre-push test fixtures, a test that pins a hooks body, a second caller, and a newly dead helper.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-browser-verdict-reachability | the Files-to-modify lists omitted pre_push.rs's test module, whose CI_LINE and Fake::green fixtures every linux.rs pre-push test drives; proceeded as an in-scope helper · `2026-09-27T19:05:39Z-b` | extra_reads 2 | — |
| 2026-09-27-hooks-to-normalised-events | plan/research companion sweep said tests/cli_instance_state.rs:344-355 asserts the rewrite not the body (no change); it asserted the literal empty hooks body and needed an edit; research's graph impact omitted client::open's second caller in server/win.rs · `2026-09-27T22:07:16Z-b` | extra_reads 1 | — |
| 2026-09-28-mutation-testing-to-the-epoch-boundary | research lists no edit for run/mutants/scratch.rs although host_scratch_bytes keeps no production caller after pre-push loses its two reads (a dead_code error under -D warnings); made as a recorded companion edit · `2026-09-28T20:24:00Z-c` | extra_reads 1 | — |
**Proposal:** the same cause as P6, X1 and L2; the direction is under L2.

### P21 — wrap-session/gates · `tooling.light-gate-red` — 3 cases · weight 3
**Pattern:** in 3 chunks, the light gate re-ran entries red on the Windows host:
- a scoped mutants entry whose killer is `#[cfg(unix)]`.
- the host's shared local integration reds, twice, with counts varying across identical trees (58 → 53 of 235; 61 → 52 of 985).
Each was accepted on an operator ruling or an ASSERT with a two-sided basis, and the host reds were routed to the :66 CARRY.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-28-cli-output-tokens | the light gate re-ran `run --mutants --file src/cmd/run.rs` red (1 missed, refuse_stale, same as implement): a plan entry red by construction on the Windows host, accepted on the operator's E1 ruling with the pre-push/CI union as the verdict; the authoring rule is pinned on working-route :53 · `2026-09-28T10:17:02Z-b` | — | `.andromeda/runs/2026-09-28T09-46-16-wrap/fanout-results.md` |
| 2026-09-29-sideloaded-conpty | entries 6 (agent-run run) and 18 (pre-push windows-tests) re-red on this Windows host as forecast: integration 58/230 and windows coverage 49/975 in the binaries the fb78ddc control fails, plus conpty_sideload 5 at the stamped_home fixture's viola verify (tests/support/verify.rs:120); recorded red - not this chunk's with the implement-day two-sided basis and the P5 CARRY at working-route :63; linux-tests green · `2026-09-29T12:58:23Z-b` | extra_reads 4 | `.andromeda/runs/2026-09-29T12-17-33-wrap/light-gate.md` |
| 2026-09-29-fake-agent-drift-contract | the light gate reproduced implement's three host-class reds on an unchanged source tree with different counts (run: 58 then 53 of 235 red; pre-push windows coverage: 61 then 52 of 985) — every failing test inside the union of the four same-day 2d8bc53 control runs (62 tests), the Linux leg green 948/948; passed the ASSERT as red — not this chunk's → the host-reds CARRY on :66 · `2026-09-29T15:03:46Z-b` | — | `.andromeda/runs/2026-09-29T14-38-17-wrap/assert_basis.py` |
**Proposal:** this is the gates side of L5 (chronic-degrade). The project absorbed part of it: a plan-authoring rule pinned on working-route :53 and the moving host-reds CARRY. See L5 for the pipeline-level direction.

### P22 — Universal · `contract.narrow-basis-claim` — 3 cases · weight 3 (per-step: implement/code 1 · phase/validate 1 · phase/research 1)
**Pattern:** count or absence claims rested on too narrow a basis in 3 chunks:
- a clones control predicted from a top-10 list.
- an awk sweep that knew 4 of 5 spellings.
- "the tests plane is not indexed", stated without a query.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-09-27-epoch-2-cleanup | (implement/code) the plan predicted clones control = 3 from research fact 10 (jscpd top-10 by size); the control read 4, the fourth a 6-line fragment beside the canonical_sddl pair outside the top 10 · `2026-09-27T15:08:04Z-e` | — | `viola-0.1.0/chunks/2026-09-27-epoch-2-cleanup/evidence/metrics-before.md` |
| 2026-09-27-browser-verdict-reachability | (phase/validate) research's companion sweep called mutants.rs:581 a full Selection literal; its awk knew only four spread spellings and missed ..flags(...); check 8's read of the literal corrected plan step 10 and research.md · `2026-09-27T18:50:05Z-c` | extra_reads 1 | `viola-0.1.0/chunks/2026-09-27-browser-verdict-reachability/research.md` |
| 2026-09-29-verify-stamped-test-homes-and-harness | (phase/research) research.md first stated 'the tests plane is not indexed' without a query; a symbol query found tests/support/home.rs indexed and the refs query replaced the grep-only basis · `2026-09-29T07:01:58Z-c` | extra_reads 2 | `.andromeda/runs/2026-09-29T06-43-45-phase/tree-query-2026-09-29-verify-stamped-test-homes-and-harness.json` |
**Proposal:** research facts that are counts or absences could carry their basis (the query, its scope) beside the claim. P5's check 8 caught one, a symbol query caught one, and the control caught the third. Each surfaced, but at a later step than necessary.

## Cross-step chains (starting heuristics)

### X1 — phase/research →research→ implement/code + implement/fix-loop — 7 chunks
Producer: every `phase/research` step in the chain is `ok`, signalling only `unresolved-questions`. Consumers: `implement/code` rated `research` **thin** in 5 chunks (browser-verdict-reachability, hooks-to-normalised-events, mutation-testing-to-the-epoch-boundary, sideloaded-conpty, fake-agent-drift-contract). `implement/fix-loop` rated it thin in 4 (browser-verdict-reachability, capability-ledger-and-viola-verify, hook-perf-gate, sideloaded-conpty). 7 distinct chunks across both.
**Hypothesis:** research's Files-to-modify is formally complete by its own check, but the tests that pin the changed surface fall outside it. Every consumer note names a test file or fixture: `tests/cli.rs` unbuilt-selector cases, `tests/run_cli.rs` and `contract_diag_schema.rs` role counts, `pre_push.rs` fixtures, `cli_instance_state.rs`, `schemas/diag-line.v1.json`, `tui_pty_seam.rs`. The downstream frictions are P6 (6), P20 (3) and the L2 facts. **Direction:** see L2.

### X2 — implement/smoke →implement-outcome→ wrap-session/report — 5 chunks
Producer: implement/smoke `ok`, signal `green`, in all 5. Consumer: wrap-session/report rated the outcome **thin** in all 5 (epoch-2-cleanup, capability-ledger-and-viola-verify, mutation-testing-to-the-epoch-boundary, h2-conpty-resize-probe, sideloaded-conpty), each time "superseded by the operator pass". A sixth symptom (untyped 625): at mutation-testing's light gate, the plan's `git diff --quiet HEAD -- …` probes read vacuously, because HEAD already held the operator pass's commits.
**Hypothesis:** the operator pass, made up of pre-push, push, CI folds and the pre-CI commit, is a real pipeline stage between implement and wrap in 5 of 11 chunks. It has no checkpoint and no contracted output, so wrap reconstructs it each time. **Direction:** contract its output (the named `evidence/operator-pass.md` head, as in P13). Separately, plan probes written as `git diff HEAD` would read against the recorded pre-CI base rather than HEAD.

### X3 — phase/plan →plan→ implement/fix-loop · wrap-session/gates · wrap-session/report — 4 · 2 · 3 chunks (validate 3, code 2)
Producer: phase/plan `ok` throughout, its signals carrying the designed dialogues. Downstream consumers rated `plan` **thin** or **wrong** after P5 had passed it:
- `implement/fix-loop`: cli-output-tokens (entry 17 red by construction, cfg(unix) killer), h2 (`run --e2e`, a selector not in the harness: **wrong**), mutation-testing (probe 13 contradicts step 6's own test), verify-stamped (gate 6's artifact key names a directory whose mtime never moves).
- `wrap-session/gates`: cli-output-tokens and mutation-testing, the same entries.
- `wrap-session/report`: capability, mutation-testing, verify-stamped.
**Hypothesis:** P5's checks resolve the classes P1 lists, but four shapes pass P5 and are unsatisfiable at run:
- a gate entry whose verdict is host-class.
- an entry naming a CLI surface that does not exist.
- an absence-probe contradicted by a literal the plan's own test creates.
- an artifact key on a directory.
**Direction:** P5 arms for each: a host-class marker, a dry parse of harness selectors, a pairwise probe-vs-plan-literal check, and artifact keys restricted to files. Each would turn a fix-loop or gates ruling into a P5 resolution.

### X4 — implement/smoke →conversation→ wrap-session/curation — 3 chunks
Producer: implement/smoke `ok` (`recorded-from-P2` / `recorded-not-rerun`). Consumer: curation rated `conversation` **thin** in hooks-to-normalised-events, capability-ledger-and-viola-verify and sideloaded-conpty. In each, a resumed wrap had lost the implement and operator-pass windows, so candidates came from the report's Decisions & corrections. The hooks-to-normalised-events curation step ended `ok-degraded` on exactly this.
**Hypothesis:** curation's primary input, the conversation, does not survive the wrap running in a fresh or resumed window. That happened in 3 chunks: one wrap resumed after an escalation halt, one after a stop following P1, one after a context stop (see the Level observations). The report's Decisions & corrections section is the de-facto carrier. **Direction:** name that section as curation's contracted input when the window is not the implement window, or have implement P4 append a corrections ledger that curation reads.

### X5 — wrap-session/report →report→ wrap-session/reconcile — 2 chunks
Producer: report `ok`. Consumer: reconcile rated `report` **thin** in capability-ledger-and-viola-verify and mutation-testing-to-the-epoch-boundary. In the first, Symbols omitted the logging and strict-modes status of verify's spawns, so reconcile read source (fact `2026-09-28T18:35:39Z-a`). In the second, a section label was wrong and an obs site went unnamed. This matches the below-threshold `input.report-insufficient` (n=2). **Hypothesis:** the report's Symbols list states what changed but not the security and obs properties of new spawns and reads, so detectors ask. A starting signal only; no direction beyond watching next epoch.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 8 facts (in-epoch) + 6 prior-epoch
**Facts:** the Bash PreToolUse guard (`bash-guard.py`) refused a command, and the step routed around it. All 8 are `nature: environment`, `solution: workaround`:
- epoch-2-cleanup · implement/code: a heredoc-to-file and 3 doubled-backslash commands → Edit tool and `chr(92)`.
- cli-output-tokens · phase/take-up: a grep regex escaping a bracket; the record's own first append was refused too.
- capability-ledger-and-viola-verify · implement/code: a python edit heredoc → Edit tool.
- mutation-testing-to-the-epoch-boundary · implement/code: a 7.3 KB heredoc hit the size guard → scratchpad files.
- verify-stamped-test-homes-and-harness · phase/distill: a sed with a doubled backslash → Edit tool.
- verify-stamped-test-homes-and-harness · implement/code: a sed with an escaped newline → Edit tool.
- sideloaded-conpty · implement/code: a file-target cat heredoc → Edit tool.
- fake-agent-drift-contract · phase/research: two probes → Grep tool.

Typed correlates: P8 (4) and 5 of P2's 8. Untyped correlates: 467, 500, 728. Prior epoch: 6 more facts of the same theme, at research, implement/code and smoke.
**Level hypothesis:** the obstacle is the guard itself, which the records attribute to a PreToolUse hook. The fixes landed in the project: three host-win32.md Session Additions entries (2026-09-28, extended twice on 2026-09-29) and a CLAUDE.md learning. Every refusal still costs a retry, at 7 different steps. The rule is always loaded (host-win32.md is unconditional) and recurred 5 times at curation's count, so the carrier is not the problem.
**Proposal:** if the guard is pipeline-shipped, as setup's settings.json hooks suggest, the change belongs at its source. One option has each refusal message name the sanctioned reroute for its shape: Grep tool for a regex, Edit tool for a source literal, a Write-tool file run by path for a script, `chr(92)` for an inline probe. That would make every refusal one retry with no rule recall needed. A second is to reconsider whether a doubled backslash in a non-file-target command, such as a grep regex, needs refusing at all; the hazard host-win32.md names is collapse into a written file. The founder judges both.

### L2 — band-aid — 4 facts (in-epoch) + 7 prior-epoch
**Facts:** a companion edit went outside the research and plan file lists, or was avoided because the file was outside them:
- browser-verdict-reachability · implement/code · process · workaround: the `pre_push.rs` test module.
- browser-verdict-reachability · implement/fix-loop · process · workaround: `crates/viola-e2e/tests/cli.rs`.
- hooks-to-normalised-events · implement/code · process · workaround: a `cfg(all(windows,test))` wrapper was kept rather than editing `server/win.rs`.
- cli-output-tokens · implement/code · product-logic · workaround: the test inherits the runner env because `tests/support` is out of the lists.

Typed correlates: P6 (6), P20 (3), and chain X1 across 7 chunks. Prior epoch: 7 facts of the same theme, including `tests/cli_fake_agent.rs`, `harness_lifecycle.rs`, a `Filter` literal, and missing `serde_json` / `sysinfo` dev-dependencies.
**Level hypothesis:** the cause appears to live in the research step's file-list derivation, which does not sweep the test trees for literals pinning the surface being changed. Each chunk absorbs it as a reportable "companion outside the lists", and one case narrowed a test rather than cross the list boundary. Across both epochs, the fixes landed in implement's deviation notes.
**Proposal:** add a pinned-literal companion sweep to research. For each changed selector, constant, enum value or output literal, run one query over `tests/`, `crates/*/tests` and the in-file test modules (the code-graph refs for the tests plane), and list the hits as companions. Or, if the founder prefers, formally admit "test companions of a changed literal" as an in-scope class for implement, so they stop counting as deviations.

### L3 — band-aid — 4 facts (in-epoch) + 2 prior-epoch
**Facts:** a pipeline tool's output was piped (`| tail`, `| head`, `| grep`) against the run-bare contract, then re-run bare. All 4 are `nature: process`, `solution: workaround`:
- cli-output-tokens · wrap-session/curation: health.py check 1.
- capability-ledger-and-viola-verify · wrap-session/gates: the P7.3c hygiene and scope re-reads.
- verify-stamped-test-homes-and-harness · phase/validate: a gate.py dry-run.
- verify-stamped-test-homes-and-harness · wrap-session/report: gate.py scope.

Prior epoch: 2 facts (code-graph queries through `| tail`). No typed correlate. These are smooth self-corrections, the kind only the deviation scan sees.
**Level hypothesis:** the contract lives in letters across several skills, while the habit of piping sits in the agent. The fixes are per-instance re-runs.
**Proposal:** a mechanical prohibition at the same PreToolUse hook as L1, refusing `andromeda-tools/scripts/*.py … | head|tail|grep`, would replace the per-letter "run bare" clauses with one control. Its refusal text would name the bare form.

### L4 — band-aid — 2 facts (in-epoch) + 1 prior-epoch (threshold: recurring)
**Facts:** both are `nature: environment`, `solution: workaround`:
- cli-output-tokens · phase/research: the cargo registry was not at `~/.cargo`; it was located via `CARGO_HOME`.
- sideloaded-conpty · phase/take-up: `~/.cargo` was absent and a `find /` fallback timed out at 120 s; it was resolved through `cargo metadata`.

Untyped friction 688 is the second fact's friction twin (2 retries). Prior epoch: the same CARGO_HOME discovery while reading the interprocess source.
**Level hypothesis:** a host fact (`CARGO_HOME` is not `~/.cargo`) is re-discovered per session. Its one-call answer (`cargo metadata --format-version 1` → `manifest_path`) was found the second time and is not recorded anywhere a research step reads.
**Proposal:** carry the recipe in the host rule or gotchas: locate crate sources through `cargo metadata`, never an assumed registry path. If setup's host-win32 template is the carrier, it would reach every Windows-host project, not just this one.

### L5 — chronic-degrade — 3 ok-degraded outcomes + 3 typed + 1 fact
**Facts:** the Windows host cannot render certain plan entries green:
- implement/fix-loop ended `ok-degraded` in cli-output-tokens (entry 17, `cfg(unix)` killer), h2-conpty-resize-probe (entry 6, unbuilt `--e2e`) and sideloaded-conpty (entries 6 and 18, host reds).
- the light gate re-red the same class 3 times (P21).
- fact `2026-09-29T11:38:38Z-a`: environment · overridden, "the overseer ruled them not this chunk's and made CI the acceptance leg".
- P10's cfg(unix) case was settled by an operator ruling ("the pre-push union is the verdict").

Never halted. Each chunk absorbed it through an operator ruling, an ASSERT, or a moving CARRY (now `:66`, carried into Epoch 3's head).
**Level hypothesis:** the gate model assumes the implementing host can produce every entry's verdict. On this host some verdicts are CI-only, by platform gating or by shared host-state reds whose counts vary across identical trees. So the degradation recurs per chunk and gets ruled each time. The project absorbed part of it: the authoring rule on working-route `:53` and the host-reds CARRY.
**Proposal:** a first-class entry attribute in the gate grammar (e.g. `verdict: ci-leg <job>`) would make a host-class entry designed rather than ruled. gate.py would then record it as delegated, not red. The host-reds CARRY on `:66` owns the project side.

### Signature results with no hit
- **Deferred-forever:** 0. All 6 `deferred` facts name their destination or close in a later record:
  - the `rm -rf mutants.out` leftover was removed by the operator (untyped 436).
  - CARRY 2's fact → the handoff.
  - E1 → ratified at the resumed reconcile `2026-09-28T04:37:42Z-a`.
  - E3/E4 → CARRYs on the Verify-stamped entry, since completed.
  - `run --e2e` → the report and wrap.
  - the size-watcher cost → the report.
- **Override:** 9 `overridden` facts across 7 steps; no rule is overridden ≥ 3 times. The closest pair is the host-class verdict ruling (L5). The rest are designed review rulings (validate ×2, plan fork ×1, take-up ×1), operator directions in fix-loop (×2), a plan.md edit after phase (×1), and a context stop (×1).

### Level observations (not signature hits; facts for the founder's eye)
- **Reconcile escalation halts: 8 halted runs in 7 of 11 wraps.** All were `escalation-shaped`, mostly security-plan boundary widenings ratified live; security.md now carries several dated founder exceptions. Related singleton: `contract.structural-blind-spot` (hooks-to-normalised-events). A boundary widening ruled at phase P4 by a founder-delegated overseer reached implement, the operator pass and green CI before the wrap halted on it (halted 1). A direction for the founder, if the pattern holds: surface boundary-widening rulings to the founder at the phase review where they are made, not at the wrap.
- **Wraps split across windows.** 3 wraps were split across windows:
  - hooks-to-normalised-events resumed after its E1 escalation halt.
  - capability-ledger-and-viola-verify stopped after P1.
  - sideloaded-conpty: fact `2026-09-29T12:20:27Z-a`, resources · overridden, "the operator stopped the wrap after P1 (context at 63 %, over the 60 % line)".

  P19 records a further pause at 92 % context. Downstream: P19's two mid-wrap mismatches and X4's three thin-conversation curations. The prior epoch carried the same pause shape twice (P1 at the context alarm; reconcile resumed at 86 %). Not a band-aid by the facts' own nature, so no signature fires.
- **`removed-cause`, n=2:** an orphaned `viola-fake-agent.exe` locking `target/harness`, and cargo-mutants copying a stale root test binary from `target/debug`. Both are stale artefacts in `target/` corrupting a gate. Below threshold; prior-epoch neighbours (rust-analyzer holding `mutants.out`) are workarounds, not removals.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/research · implement/code — 3 cases → proposed type `tooling.hook-friction` as a **Universal** type
**Cluster:**
- 467 (browser-verdict-reachability, implement/code): "the Bash PreToolUse guards refused two commands (a doubled backslash in a test path; a cat heredoc appending to a test file) and each was re-issued in the sanctioned form".
- 500 (hooks-to-normalised-events, implement/code): "the bash-guard PreToolUse hook blocked two python heredoc edit scripts carrying Rust string escapes (a doubled backslash); both edits were redone with the Edit tool".
- 728 (fake-agent-drift-contract, phase/research): "The doubled-backslash Bash guard fired twice in research although host-win32.md's 2026-09-28 addition naming it was loaded; both probes were re-run through the Grep tool".

The L1 facts place the same event at phase/take-up, phase/distill, phase/research, implement/code and wrap-session/curation. Only implement/code's playbook lists the type, and 2 of the 3 untyped records sit at that very step.
**Draft criteria line:** "`tooling.hook-friction` — a harness hook (PreToolUse guard, permission or safety classifier) refused or blocked a command the step needed; record the hook, the refused shape, and the reroute taken."

### U2 — wrap-session/reconcile — 1 in-epoch + 2 prior-epoch (recurring) → proposed type `tooling.pattern-file-refused`
**Cluster:**
- 713 (sideloaded-conpty): "cascade.py refused the first three pattern files: a doubled single quote inside a TOML literal string, an id over 16 chars, and PTY_BACKEND whose control never fires (no master ever named the const) - fixed, dropped and hand-controlled by grep".
- Prior epoch, 2 cases: "cascade.py sweep refused the pattern file (exit 2): pattern ids are limited to 16 chars, the reference's cascade section does not state the limit; 12 ids renamed and the sweep re-run", and "cascade.py sweep refused the first pattern file (exit 2): pattern ids over 16 chars; ids shortened and the sweep re-run".

A prior-epoch problem-fact repeats it as well: "cascade.py refused a 14-char+ pattern id and an unfired control".
**Draft criteria line:** "`tooling.pattern-file-refused` — cascade.py refused an authored pattern file; record each refused constraint (id length, TOML string form, unfired control) and whether the reconcile reference states that constraint." The recurring id-length refusal itself suggests stating the 16-char limit in the reference, which would also be a `contract.skill-reference-drift` fix.

## Below threshold — no action
**Typed groups (per step, n below F-2):**
- `implement/fix-loop/tooling.result-not-run-stable` n=2 · weight 16 · hook-perf-gate, fake-agent-drift-contract — on one tree the host scoped loop over perf.rs graded 'delete field artifact' and 'replace time_rows with ()' unviable while both pre-push legs graded them missed and c…
- `wrap-session/reconcile/ambiguity.escalation-rounds` n=2 · weight 8 · epoch-2-cleanup, hook-perf-gate — E1 (wsl-exec.sh boundary widening) took a second round: the invariant first written as absolute contradicted this chunk plan own wsl-exec.sh --probe gate entry, found …
- `implement/fix-loop/tooling.environmental` n=2 · weight 7 · epoch-2-cleanup, sideloaded-conpty — gate entry 22 (G1 rg) exited 127: the gate tool non-login bash has no rg; the interactive shell rg is a function; the pinned binary sat in target/tools/ripgrep/bin
- `implement/fix-loop/retry.fix-iterations` n=1 · weight 11 · sideloaded-conpty — seven fix iterations: companion sharing violation, a pinned pty_backend literal, test-driver leak, first-start write cost (concurrency tried and reverted, then a test-…
- `wrap-session/reconcile/retry.detector-respawn` n=1 · weight 8 · hooks-to-normalised-events — the halted window's fanout-results.md held dispositions but no parsed proposal lists, so the Setup 2a resume could not reuse the fan-out and all seven doc-agents re-ran
- `phase/plan/retry.synthesis-rework` n=2 · weight 4 · hook-perf-gate, fake-agent-drift-contract — step 2 had hook.rs read FAKE_AGENT_HOOK_PANIC while the one-file grep gate required the name only in seam.rs; steps 1-2 and the gate note were re-written so seam.rs ho…
- `implement/fix-loop/contract.instrument-validity` n=2 · weight 3 · hooks-to-normalised-events, verify-stamped-test-homes-and-harness — the local windows-2025 mutation leg could not kill any mutant only a root integration test catches while target/debug held a root test binary (copy-target carried its …
- `wrap-session/reconcile/contract.structural-blind-spot` n=1 · weight 6 · hooks-to-normalised-events — a boundary widening ruled at phase P4 by a founder-delegated overseer reached implement, the operator pass and a green CI before any step showed it to the founder; the…
- `phase/validate/input.out-of-pipeline-source` n=1 · weight 5 · hooks-to-normalised-events — the long-paste wrapper's closing form (id repeated, unescaped CLI pair vs escaped typed forms) lived in the live session log ~/.viola/sessions/viola-builder/events.ndj…
- `phase/research/contract.premise-falsified` n=2 · weight 2 · epoch-2-cleanup, verify-stamped-test-homes-and-harness — the route CARRY and triage premise of 11 Windows-only survivors needing killing tests was falsified: the code audit's per-crate cargo-mutants ran on the Windows host w…
- `wrap-session/route-resolve/contract.no-sanctioned-channel` n=2 · weight 2 · epoch-2-cleanup, h2-conpty-resize-probe — the operator directed the 8 root-package tests with a 10 s bound equal to the mutants kill to fold into the NEXT chunk at its phase P1, explicitly not as a CARRY; the …
- `phase/research/ambiguity.scope-boundary` n=2 · weight 2 · capability-ledger-and-viola-verify, hook-perf-gate — the job-shape fork decides whether pre_push/linux.rs and the WSL root-install fix join the file lists, so research held those branch files out and the lists get re-wri…
- `wrap-session/reconcile/input.report-insufficient` n=2 · weight 2 · capability-ledger-and-viola-verify, mutation-testing-to-the-epoch-boundary — report Symbols for viola verify omitted whether its --version read and probe child log process lines and whether its update_stamps read runs strict-modes; obs and secu…
- `phase/plan/input.research-thin` n=2 · weight 2 · mutation-testing-to-the-epoch-boundary, verify-stamped-test-homes-and-harness — research listed run.rs only for the leg parameter; synthesis found Selection::from_flags still puts mutants in the default and --all selection (the per-chunk gate's ot…
- `phase/validate/contract.structural-blind-spot` n=1 · weight 3 · hook-perf-gate — the letter's P3 form gate.py scope --lists-only --research carries no --run-dir, so neither parse (9 items at P3, 11 after the P4 fork update) left a trail in the phas…
- `wrap-session/route-resolve/ambiguity.trajectory-halt` n=1 · weight 3 · sideloaded-conpty — the epoch-growth valve surfaced Epoch 2b at 11 chunks once more; the operator relayed that the founder already ruled no split at ~21:50 on 2026-09-28, a ruling not on …
- `wrap-session/route-resolve/contract.grammar-irregularity` n=1 · weight 2 ·  — INDETERMINATE: working-route.md:43 — after ONE space — freight or prose, no structural split: 'P11:'
- `implement/code/input.conventions-gap` n=1 · weight 2 · epoch-2-cleanup — plan step 7 names rstest cases for the scratch guard; viola-e2e carries no [dev-dependencies] at all, measured on read of its Cargo.toml after the first compile failed
- `phase/research/tooling.host-shell` n=1 · weight 2 · browser-verdict-reachability — Git Bash converted the leading-slash argv of wsl.exe --exec /usr/bin/env into C:/Program Files/Git/usr/bin/env; both WSL calls failed execvpe until MSYS2_ARG_CONV_EXCL…
- `wrap-session/reconcile/ambiguity.playbook-no-match` n=1 · weight 2 · browser-verdict-reachability — T17 (annotate the test-plan §12 initial Decisions Log entry as superseded) matched no rule; rejected by unease as a history edit, the new dated entry records the super…
- `wrap-session/gates/tooling.commit-mechanics` n=1 · weight 2 · browser-verdict-reachability — route.py compact --dry-run was fired before route.py flip and refused ('its record is pending -- compact runs AFTER the flip', exit 3); re-fired in order
- `implement/code/contract.premise-falsified` n=1 · weight 2 · capability-ledger-and-viola-verify — plan step 8 states the #[files("fixtures/claude/*/*.json")] glob matching no file is expected at /implement time; rstest_macros 0.27.0 files.rs:635 errors 'No file fou…
- `wrap-session/gates/contract.premise-falsified` n=1 · weight 2 · h2-conpty-resize-probe — after P2 applied them, the overseer's relayed source read (microsoft/terminal resize path flushes no input buffer; no reported resize input loss) withdrew the 'loss is…
- `implement/code/contract.narrow-basis-claim` n=1 · weight 1 · epoch-2-cleanup — the plan predicted clones control = 3 from research fact 10 (jscpd top-10 by size); the control read 4, the fourth a 6-line fragment beside the canonical_sddl pair out…
- `phase/validate/contract.narrow-basis-claim` n=1 · weight 1 · browser-verdict-reachability — research's companion sweep called mutants.rs:581 a full Selection literal; its awk knew only four spread spellings and missed ..flags(...); check 8's read of the liter…
- `phase/validate/contract.token-proxy-check` n=1 · weight 1 · browser-verdict-reachability — the pre-push leg atoms copied from the prior plan cited pre_push.rs:193 and :286; planlint 4 (5) passed on the atom-from token while the lines no longer held the token…
- `new-session/orientation/contract.jointly-contradictory-instructions` n=1 · weight 1 ·  — the evolve-nudge clause needs friction-log records per epoch, while evolve-system says the agent never reads the friction file; no tool answers the per-epoch count, so…
- `phase/distill/input.spec-source-gap` n=1 · weight 1 · hook-perf-gate — obs-plan §9 G2 / §10 zero-unlogged-panics carries no exemption for a deliberate test panic, so the ratified forced-panic case would turn G2 red under the plan as writt…
- `implement/code/contract.jointly-contradictory-instructions` n=1 · weight 1 · mutation-testing-to-the-epoch-boundary — plan step 6 + acceptance (tests/cli.rs pins `gate --require mutants --mutants-legs a` exit 2) and the plan's grep -rlE 'mutants-legs|...' crates probe (expect no outpu…
- `wrap-session/report/contract.token-proxy-check` n=1 · weight 1 · mutation-testing-to-the-epoch-boundary — the expected-amendments site sweep listed a11y-plan :1145 as a mutation-union site from a bare 'union' token hit; reading the line showed 'the union of the per-state a…
- `wrap-session/curation/ambiguity.tier-routing` n=1 · weight 1 · mutation-testing-to-the-epoch-boundary — the probe-narrowing ruling is a directive about plan authoring; no rule file's paths scope plan.md, so it demoted to Tier 3 under the fallback chain
- `phase/research/contract.narrow-basis-claim` n=1 · weight 1 · verify-stamped-test-homes-and-harness — research.md first stated 'the tests plane is not indexed' without a query; a symbol query found tests/support/home.rs indexed and the refs query replaced the grep-only…
- `phase/take-up/input.out-of-pipeline-source` n=1 · weight 1 · fake-agent-drift-contract — The overseer-directed harness-origin item rests on a measurement on andromeda-worker and a fix in the prototype's hook.rs harness_injected, both outside this repositor…
- `wrap-session/reconcile/contract.proposal-format` n=1 · weight 1 · fake-agent-drift-contract — the architecture doc-agent's return quoted the spec's harness-prefix tag literals and the harness flagged it as a harness-envelope-tag, neutralising every quoted tag's…
- Universal by type, below: `contract.structural-blind-spot` n=2 (hooks-to-normalised-events reconcile, halted 1; hook-perf-gate validate, a `gate.py scope --lists-only` call with no `--run-dir`, so no trail). Two unrelated mechanisms with one halt between them, so no shared cause. `contract.jointly-contradictory-instructions` n=2 (new-session evolve-nudge needs per-epoch friction counts the agent may not read; a mutation-testing plan pin vs its own grep probe). `contract.grammar-irregularity` n=1 (`working-route.md:43` — `INDETERMINATE: … freight or prose, no structural split: 'P11:'`). `tooling.host-shell` n=1.

**Untyped clusters below F-4 (watch next epoch):**
- auto-mode classifier with no verdict: 557 (cli-output-tokens, 2 retries), beside problem-facts `2026-09-28T07:41:04Z-a` and `2026-09-28T08:36:07Z-a` (hook-perf-gate, report and gates). Already absorbed as a CLAUDE.md Tier-1 learning.
- permission classifier refused a remove-the-guard run: 704 (sideloaded-conpty, halted 1, recorded not run).
- cargo-mutants mechanics: 536 (a `cfg(all(test, feature))` module is mutated) · 586 (an `mpsc::Receiver` return gives 10 unviable replacements; the operator asked for a fn).
- hygiene refusing the wrap's own artifacts for a host path: 685 (curation.md) · problem-fact `2026-09-29T06:37:38Z-a` (operator-pass.md). Prior-epoch neighbours: absolute paths in run-dir listings and `outcomes.json`.
- vacuous `git diff HEAD` probes after an operator pass: 625. Folded into X2.
- singletons:
  - 416: the ladder has no rung for a post-diagnosis cleanup chunk. Mistyped; `ambiguity.ladder-uncovered-state` fits.
  - 453: the dashboard rendered a direction-less handoff fold as "raise".
  - 436: a gate entry ran 1 minute before the operator's removal.
  - 470: the recorder's snapshot read raced the wrapper. A product fix.
  - 575: an overseer size yardstick exists on no artifact on disk.
  - 630: a relative redirect wrote outside the repo.
  - 512, 645: mistyped; `contract.cascade-miss` fits, as P7 notes.
  - 726: mistyped; `contract.token-proxy-check` fits, as P16 notes.

**Problem-fact themes below threshold:**
- plan-letter vs implementation, 5 facts, mostly product-logic: the rstest table, the duplicate init guard, an unused `is_spine`, a feature spelling, the fixture walk. P5 covers the typed side.
- MSYS argument conversion: 1 in-epoch fact + 2 prior.
- anchored long-line Edit: 1 fact. P12 covers it.
- a playbook rule collision resolved by precedent without the escalation the collision clause asks for (mutation-testing reconcile, S7/S8 Verbatim copy vs Accurate addition): 1 fact.
- Git Bash has no `rev`: 1.
- measured on an existing debug binary rather than rebuilding: 1.
- an out-of-repo prototype read to close a premise: 1, plus the take-up `input.out-of-pipeline-source` singleton.
- a backgrounded gate block stopped and re-fired whole: 1.
- light gate prohibited from running a root WSL launch: 1 prohibition.
- remove-the-guard run refused: 1 unresolved.
