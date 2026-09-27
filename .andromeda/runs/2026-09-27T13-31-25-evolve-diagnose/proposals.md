# Evolve Diagnosis — viola-0.1.0 · Epoch 2 — Windows slice I: wrapper, events, ledger · 2026-09-27T13:31:25Z

Obligation-free: every item below may be accepted, rejected, deferred or modified; nothing here is applied, queued or remembered. Line refs `L{n}` are `.andromeda/friction-log.ndjson` line numbers; ids are the records' own.

## Mechanism health

- **Records:** 219 in-epoch after the retraction pre-pass (114 step / 105 friction; 220 raw, 1 friction record retracted whole) · unparseable: 0 · malformed-ts: 0 by the ISO check.
- **Hand-authored ts (well-formed, wrong):** 4 records of the wrapper-channel wrap (ids `2026-09-27T13:02:00Z-a/-b`, `2026-09-27T13:10:00Z-a/-b`) carry a ts later than their events; the -b records are clause-retracted and the -a records are named by an `id:null` retraction (below). The ISO check cannot see this class — a well-formed ts that is not a `date -u` reading. Recorded untyped as L412.
- **Coverage:** all 7 chunks carry phase 5/5 and wrap 5/5; implement ≥ 3/3 everywhere (5 · 5 · 6 in security-prerequisites, local-linux-pre-push-gate, instance-state-and-start-order — resumed fix-loop/smoke passes, not gaps). chunk-null: new-session 13 (one per session start), wrap 3 (adaptation / no-op wraps). **No checkpoint gaps.**
- **Outcomes:** {'ok': 101, 'halted-resolved': 7, 'ok-degraded': 3, 'soft-exit': 3}.
- **Problem-fact fill:** 47/114 step records carry at least one fact · **id fill:** 219/219.
- **Untyped rate per step:** wrap-session/reconcile 5/15 · implement/fix-loop 3/29 · implement/code 2/9 · wrap-session/route-resolve 1/7 · phase/distill 1/3 (all other steps 0).
- **Retractions (whole ledger):** retracted 1 record + 1 problem-fact (2 unresolvable) · clause-retracted 3 (kept, notes rendered) · retraction targeted by retraction: none.
  - unresolvable `2026-09-27T07:06:36Z-b` (unknown-id-or-scope), verbatim note for manual discount: “discount the signal carries-pinned-7 and the note '7 CARRYs' in step record 2026-09-27T06:40:33Z-a: 8 CARRYs were pinned (:40 x4, :43, :65, :67, :73); a step-record signal has no record/clause retraction scope”
  - unresolvable `2026-09-27T12:58:32Z-a` (unknown-id-or-scope), verbatim note for manual discount: “discount the ts of step records 2026-09-27T13:02:00Z-a and 2026-09-27T13:10:00Z-a the same way; their content stands”
- **Calibration boundaries in range:** none newly crossed — the deviation scan, ids and the Universal types were all live for the whole epoch.

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 10 cases · weight 18 · rate 10/7 step-runs
**Pattern:** P5's mechanical checks fired on the P4 draft in every one of the 7 chunks (10 cases, 7 step-runs); the same checks recur — 4(9) new-entry marking / vacuity 4×, 4(6) a criterion naming a gate with no entry 3×, check 9 research's Platform-issues slot 2×, plus 4(2) boot-path-without-smoke, 3 (minted names) and 8 (criterion with no witness-writing step) once each. All resolved before the review; the cost is iterations + extra reads, never a halt.

**Evidence:** ALL 10 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| epoch-1-cleanup | validate | check 5 (placeholder leak) and check 4 (9) (new-entry grep) fired on the P4 draft: a brace-shaped output-format description in step 1 prose, and two grep probes aimed at new scopes left unmarked new; both fixed before the review (L203) | iterations 1 | — |
| security-prerequisites | validate | check 4(9) corpus grep found two P4-authored entries without new=true (the deny-sync sole-root form, the deny.toml allow-line grep) and check 9 found research's Platform-issues slot 'none' while the fence reads CI runs; all resolved at P5 before review (L223) | extra_reads 2 | .andromeda/runs/2026-09-25T12-15-33-phase/p5/ |
| pty-wrapper-on-windows | validate | check 4(6) required-resolution: acceptance criteria named secret-scan, G4 schema-check, zizmor and the coverage floors with no gate entry; added the three entries (+ kept homes on the integration run) and reworded coverage to the operator CI read (L251) | iterations 1 | — |
| pty-wrapper-on-windows | validate | check 4(9) vacuity: install-ripgrep.sh --probe (the script ignores its arguments today and exits 0 on the real install) and orphans-check.sh --probe (already green at 1/1) carried exit-only expects; added last-line verdict atoms so both baseline red (L252) | iterations 1 | .andromeda/runs/2026-09-25T13-34-03-phase/p5/18.log |
| pty-wrapper-on-windows | validate | check 2 touchpoint: the secret-scan canary table was listed by description, not path; resolved to crates/viola-e2e/src/harness/secret_scan.rs l.15-20 (scan_patterns.rs no change) (L253) | extra_reads 1 | — |
| ci-chunk-base-and-union-verdict | validate | check 4 (4)/(5)/(6) each required a resolution the P4 authoring self-check does not name: an artifact key on a producer a criterion reads, atom-from sources on operator check-run reads, one-shot guard prose (L293) | iterations 1 | .andromeda/runs/2026-09-26T19-33-34-phase/p5/ |
| local-linux-pre-push-gate | validate | the review caught a premise no check reads: research M3 made build-essential an operator sudo-password step, but wsl -u root is uid 0 with no password (overseer measurement); entry 6 turned into an ordinary build entry (L314) | dialogue_rounds 1 | — |
| local-linux-pre-push-gate | validate | check 9: research's Platform issues slot was first filled with context sources (a REST doc, a runner-image log) under a lead pointing back at check 9, reading as a completed search; the review rewrote it to 'none — no signature search' with the sources as context, and the HIT stands (L315) | dialogue_rounds 1 | — |
| instance-state-and-start-order | validate | checks 4(2) (src/cmd/run.rs boot path, no smoke entry) and 4(6) (criterion named G4, no entry) fired on the P4 fence; the added smoke's 4(9) baseline then showed harness logs has no --kind/events source, adding a step-13 bullet and two touchpoints (L342) | extra_reads 4 | .andromeda/runs/2026-09-27T01-36-12-phase/p5/s4.log |
| wrapper-channel | validate | P5 checks fired on the P4 plan: check 3 (minted names absent from arch Occupied Resources: the Unix viola-<h12>.lock and the test endpoint names), check 8 (three criteria — first-snapshot endpoint, squatted stderr, wrapper role-file conn/corr and the v1-20 debug-redaction half — had no step writing their witness against a real wrapper), check 6 size WARN 468 > ~400; all resolved by plan edits before review, full set re-run (L385) | extra_reads 3 | .andromeda/runs/2026-09-27T07-09-57-phase/p5/ |

**Proposal:** A P4-side pre-run of the recurring checks (4(6) · 4(9) · 9) on the draft before P5 opens — e.g. planlint over the draft fence as P4's last step, or the P4 authoring self-check naming these three resolutions (the ci-chunk-base case says the self-check does not name them) — so P5's review reads a draft these checks already passed. The design question of whether P5 catching them IS the intended division of labour is the founder's.

### P2 — any step (universal / recall — grouped by type) · `contract.premise-falsified` — 9 cases · weight 14
**Pattern:** 9 falsified premises across 6 chunks and 4 steps. Two sub-shapes repeat: (a) a spec names a crate whose licence the repo's deny.toml allowlist rejects — interprocess 2.4.4's 0BSD deps, atomic-write-file 0.3.1's BSD-3-Clause (2 chunks); (b) a CARRY's stated premise does not hold when its owner takes it up — corr-required-everywhere vs obs-plan's documented nulls, the arbiter that cannot reach the replace_private_shared red, CARRY 2's base equality at HEAD==flip (3 cases, 2 chunks). The rest are ConPTY facts measured at research (2) and one mutation premise.

**Evidence:** ALL 9 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| epoch-1-cleanup | fix-loop | scope/plan premise that every moved run.rs mutant was once caught held only for the two-leg union: fuzz_host_supported -> false had passed at authoring via the ubuntu leg alone, so the host-leg entry asserting ok:true went red on it (L206) | iterations 1 | viola-0.1.0/chunks/2026-09-24-epoch-1-cleanup/evidence/metrics-after.txt |
| security-prerequisites | research | security-plan names interprocess 2.4.4 for the channel client, but its transitive deps doctest-file and recvmsg are 0BSD, which the repo deny.toml licence allowlist rejects; the scope premise 'cargo deny stays green' failed on measurement (L220) | dialogue_rounds 1 | viola-0.1.0/chunks/2026-09-25-security-prerequisites/research.md#cargo-deny |
| pty-wrapper-on-windows | research | a11y-plan §3 Windows zero-viola-bytes check (no SGR or cursor control the child did not emit) is unsatisfiable: ConPTY itself emits ?9001h/?1004h/?25l/2J/H, an OSC 0 title with the command line, and re-renders nested output (measured spike) (L246) | extra_reads 1 | viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/research.md §Measured facts 4 |
| pty-wrapper-on-windows | research | test-plan §4/§6 require a literal 14-name list copied from S6, but input.md S6 and the brief name only 5 of the 14; no artifact enumerates them (L247) | extra_reads 3 | viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/research.md §Measured facts 7 |
| pty-wrapper-on-windows | research | the working entry presumed the fake agent and the wrapper can simply move to a PTY; under ConPTY a cooked console line-buffers input and swallows Ctrl-C (0x03) in both child and wrapper, so raw-mode handling enters scope (L248) | extra_reads 2 | viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/research.md §Measured facts 1-3 |
| local-linux-pre-push-gate | plan | CARRY 2's 'the leg's printed base should equal the Windows and CI legs' base' fails before the pre-CI commit when HEAD is the wrap flip: chunk_flip steps to HEAD^ (acd08c7) while CI derives a69c5ef; found at P4 synthesis, missed by the P3 closure (L312) | extra_reads 1, dialogue_rounds 1 | — |
| instance-state-and-start-order | research | architecture.md:22/:50 [Snapshot writer] names atomic-write-file 0.3.1, whose crates.io licence (BSD-3-Clause) the project's deny.toml allow list rejects; no spec records the conflict (L339) | extra_reads 1 | viola-0.1.0/chunks/2026-09-27-instance-state-and-start-order/research.md §Measured facts |
| wrapper-channel | research | CARRY from 2026-09-24-diagnostics-plane: make corr required for every channel-*/dialog-*/hook-*/send-*/release-from-driver line; obs-plan itself documents null corr for hook.event notifications, client-side send-refused and non-dialog hook-invoked, and -32700/-32600 responses carry no id (L381) | extra_reads 2 | viola-0.1.0/chunks/2026-09-27-wrapper-channel/scope.md fold item 2 |
| wrapper-channel | research | CARRY from 2026-09-27-instance-state-and-start-order named this chunk's exclusive-bind arbiter as owner of the replace_private_shared red; the red's boot was two names (builder, overseer) with distinct endpoints and the bind sits after the pin copy, so the arbiter cannot reach it (L382) | extra_reads 1 | viola-0.1.0/chunks/2026-09-27-wrapper-channel/scope.md fold item 11 |

**Proposal:** (a) A licence probe at the point a spec names a crate: the architecture/security amendment path (or research's standard probe list) running `cargo deny check licenses` against a scratch manifest carrying every crate a spec master newly names, so the conflict lands in the spec before a chunk inherits it. (b) The CARRY grammar carrying its premise's measurement pointer (or `HYPOTHESIS` when unmeasured — the session-learning already applied to wraps), so the owner's research re-verifies a labelled premise first. Research did catch all of them — the proposal is about moving the catch earlier, not about a miss.

### P3 — implement/fix-loop · `retry.fix-iterations` — 3 cases · weight 19 · rate 3/11 step-runs
**Pattern:** All 3 high-iteration fix-loops are mutation-leg driven: 4 runs (unviable swamp → 49 → 9 → 5 cfg(unix)), 2 further ~18-min union runs for 5 then 1 survivors, and 6 iterations of which one full pre-push was spent on a false belief that cargo-mutants skips const initializers. 12 iterations + 4 retries in 3 of 11 fix-loop runs.

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| pty-wrapper-on-windows | fix-loop | mutation leg took 4 runs: unviable swamp (new crates lacked the fake-agent feature cargo-mutants passes), then 49 missed (real-PTY paths testable only via root tests), then 9, then 5 cfg(unix)-only (L263) | iterations 4 | — |
| instance-state-and-start-order | fix-loop | the pre-push union named 5 then 1 surviving mutants across two further runs (unobservable umask-only restrict, redundant parent filter, unjoined-thread drop, NotFound guard, non-object event line); each run costs ~18 min (L352) | iterations 2, retries 2 | .andromeda/runs/2026-09-27T02-03-01-implement (gate entry 21) |
| wrapper-channel | fix-loop | 6 iterations to green; two went to mutation survivors, the second after hoisting disjoint-bit flag operators into consts on the false belief that cargo-mutants skips const initializers (27.1.0 mutates them), costing one full pre-push run (L393) | iterations 6, retries 2 | — |

**Proposal:** A cheaper inner loop before the full union: a file-scoped cargo-mutants run (`--file` over the chunk's changed sources, host leg only) as the fix-loop's first mutation pass, the full pre-push union kept as the verdict. Plus a curated cargo-mutants 27.1.0 behaviour sheet (mutates const initializers; runs only the mutated package's tests; unviable without the fake-agent feature) as a testing-rule reference — three of these beliefs cost runs this epoch.

### P4 — wrap-session/route-resolve · `contract.carry-no-owner` — 4 cases · weight 14 · rate 4/9 step-runs
**Pattern:** A recurrence watch or directed hypothesis reached route-resolve naming no route entry ("the next entry that feeds viola run stdin", a route note naming no entry, a report-only owner, a frozen line's watches after the flip) in 3 of 7 chunks; 2 of the 4 halted for an overseer placement.

**Evidence:** ALL 4 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| pty-wrapper-on-windows | route-resolve | the ^Z hypothesis was directed to "the next route entry that feeds viola run stdin", which no entry names; placed on "The wheel" (human-keystroke path) by reading entries 47-57 (L281) | extra_reads 1 | — |
| instance-state-and-start-order | route-resolve | overseer item 4 (local pre-push has no Windows llvm-cov test stage, green twice on trees CI read red) was relayed as a route note naming no entry; placement halted and the overseer chose a CARRY on Wrapper channel with a fold-now direction (L372) | dialogue_rounds 1, halted 1 | .andromeda/runs/2026-09-27T06-12-23-wrap/resume.md |
| instance-state-and-start-order | route-resolve | the replace_private_shared fix-by-reasoning label had a recurrence owner only in the report entry, with no route entry named; placement was asked in the same halt round and the overseer chose Wrapper channel, which lands the exclusive-bind arbiter (L373) | — | viola-0.1.0/chunks/2026-09-27-instance-state-and-start-order/report.md:142 |
| wrapper-channel | route-resolve | Two recurrence watches owned by this chunk's frozen line (viola-pty resize red, harness_lifecycle concurrent-boot red) had no owner after the flip and no report mention; halted, and the overseer directed one move to the head entry with a 3-green-runs expiry, to be taken by the Epoch 2 cleanup chunk not yet minted [clause retracted: ts 2026-09-27T13:10:00Z (hand-authored; true time 12:54:15Z–12:58:22Z)] (L411) | dialogue_rounds 1, halted 1 | — |

**Proposal:** Require an owner at WRITE time: the report template's recurrence/owner field and any relayed direction naming `working-route:<line>` (or the minted cleanup head) rather than a description, with route.py's pins listing flagging a report owner that names no line. The halts were resolved by the same shape of answer each time ("CARRY on entry X"), which suggests the placement could be asked at the report step, where the owner is known.

### P5 — implement/fix-loop · `contract.vacuous-check-found` — 5 cases · weight 10 · rate 5/11 step-runs
**Pattern:** The remove-the-guard and mutation protocols found 5 insensitive checks in 3 chunks — a guard test green without its guard, a restore check true with or without the restore, a clean-step never exercised, a cap used only symbolically, a guard redundant with the bind. Each cost one iteration; each was closed.

**Evidence:** ALL 5 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| pty-wrapper-on-windows | fix-loop | remove-the-guard showed pump_copies_the_output_it_drains_after_exit insensitive to an exit-on-output-EOF guard; added pump_waits_for_the_handle_after_the_output_ends, then red under the neutralised guard (L259) | iterations 1 | viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/evidence/guards/readings.json |
| pty-wrapper-on-windows | fix-loop | the child PTY test checked terminal restore through a fresh HostTerminal::enter(), true with or without the restore; a drop->() mutant survived until the check read ENABLE_LINE_INPUT / ICANON after drop (L260) | iterations 1 | — |
| local-linux-pre-push-gate | fix-loop | guard g8 (git clean in the sync) neutralised stayed green: reset --hard already drops files the clone's own add -A staged, so the test never exercised clean; a stray clone-side file case was added and the re-run pair holds (L320) | iterations 1 | chunks/2026-09-26-local-linux-pre-push-gate/evidence/guards/readings.json |
| local-linux-pre-push-gate | fix-loop | the union's first live run found 7 survivors in this chunk's own new tests: the 40 GiB cap used only symbolically (6 arithmetic mutants) and no pin on skipping git apply for an unchanged tree; literal and call-absence assertions added, killed on the next run (L324) | iterations 1 | implement run dir pp-instr-1.out / pp-instr-2.out |
| wrapper-channel | fix-loop | the mutation protocol found the Unix stale-socket removal's non-NotFound error guard insensitive: any leftover that cannot be removed already fails the bind, so the guard was removed (L396) | iterations 1 | — |

**Proposal:** The protocol is doing its job; the direction is to move the catch into authoring: a short list of the measured insensitivity shapes (fresh-state re-read instead of post-drop read; a constant used only symbolically; a guard another mechanism already enforces; a cleanup a prior reset already performs) as plan-authoring guidance beside the guard-test rule, so P4 writes the killing assertion first.

### P6 — any step (universal / recall — grouped by type) · `recall.corpus-recurrence` — 6 cases · weight 7
**Pattern:** 6 cases, all at curation: 5 rules already curated (host-win32 §Paths MSYS conversion ×2, the append-lock rule in events.md, 'never pipe agent-run.sh boot', stop-by-ExecutablePath) were reproduced anyway, and 1 learning left as a 0.7 handoff deferral (rust-analyzer holding mutants.out) recurred on the next session's first mutation run. Note: 2 of the instance-state cases (pipe boot, substring stop) are the same events the fix-loop also recorded as untyped friction (U-appendix) — the event count is 6, the record count 8.

**Evidence:** ALL 6 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| ci-chunk-base-and-union-verdict | curation | the rust-analyzer mutants.out lock recurred (os error 5) because the prior session left it as a 0.7 deferred learning in the handoff rather than a curated entry; this session reproduced it on the first run --mutants (L302) | retries 1 | .andromeda/runs/2026-09-26T20-59-23-wrap/curation.md |
| local-linux-pre-push-gate | curation | the report's sweep hazard 'from Git Bash a /mnt/c/… argument needs MSYS2_ARG_CONV_EXCL' is already curated in host-win32.md §Paths (leading-/ mangling, scope MSYS2_ARG_CONV_EXCL); the implement work reproduced the failure anyway (L332) | — | viola-0.1.0/chunks/2026-09-26-local-linux-pre-push-gate/report.md §Decisions & corrections |
| instance-state-and-start-order | curation | events.md 'append + exclusive lock fails on Windows' was already curated; the chunk's lock opener was written append-only anyway (L367) | — | .andromeda/runs/2026-09-27T06-12-23-wrap/curation.md |
| instance-state-and-start-order | curation | verification-harness.md 2026-09-25 'Never pipe agent-run.sh boot' was already curated; a subprocess capture of boot hung anyway (L368) | — | .andromeda/runs/2026-09-27T06-12-23-wrap/curation.md |
| instance-state-and-start-order | curation | host-win32.md 2026-09-25 'Stop a process by its exact ExecutablePath' was already curated; a command-line-substring stop killed its own shell anyway (L369) | — | .andromeda/runs/2026-09-27T06-12-23-wrap/curation.md |
| instance-state-and-start-order | curation | host-win32.md §Paths MSYS leading-slash conversion was already curated; a wsl.exe --exec /usr/bin/... call was mangled anyway (L370) | — | .andromeda/runs/2026-09-27T06-12-23-wrap/curation.md |

**Proposal:** Where a curated always-loaded rule is reproduced anyway, the prose tier is not reaching the moment of action; the direction is enforcement over curation for those rules — a PreToolUse hook that flags the exact shapes (`wsl.exe` + a leading-`/` arg without MSYS2_ARG_CONV_EXCL; `agent-run.sh boot` into a pipe; a `Stop-Process`/`taskkill` keyed on a command-line substring), the same mechanism the bash-guard already uses. For the deferral case see P16.

### P7 — implement/fix-loop · `tooling.result-not-run-stable` — 4 cases · weight 9 · rate 4/11 step-runs
**Pattern:** 4 fix-loop cases in 4 chunks (plus 1 at gates): two tests red once then green (harness_lifecycle concurrent boot; tui resize reading a torn line), cargo-mutants grading an identical tree 73/6 then 71/8, and secret-scan reading a stale chunk.diff residue. Twice the note says no artifact froze the first number; once the red run's home was removed by the test itself.

**Evidence:** ALL 4 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| pty-wrapper-on-windows | fix-loop | secret-scan read target/agent-run/chunk.diff left by a prior run --mutants (hit: its own ?t= pattern-table line as diff context of secret_scan.rs); green once the residue was removed (L262) | iterations 1 | — |
| ci-chunk-base-and-union-verdict | fix-loop | two cargo-mutants 27.1.0 windows runs over the identical final tree graded differently: caught 73 / unviable 6, then caught 71 / unviable 8 (79 tested both times); no artifact froze the first number, both green (L297) | — | viola-0.1.0/chunks/2026-09-26-ci-chunk-base-and-union-verdict/evidence/mutants-leg-windows-local.md |
| instance-state-and-start-order | fix-loop | harness_lifecycle harness_session_boots_reports_logs_and_tears_down went red once (concurrent overseer+builder boot, builder exit 1) on a tree it passed on before, and passed 8/8 on reproduction; the red run left no chain (home removed by the test) (L355) | retries 1 | — |
| wrapper-channel | fix-loop | tui_passthrough tui_host_resize_reaches_the_child went red in one Linux pre-push after green in two: the receipt reader parsed a torn final line; no artifact froze the first number; folded as a harness fix, not closed by re-run (L397) | iterations 2, retries 1 | viola-0.1.0/chunks/2026-09-27-wrapper-channel/evidence/red-torn-receipt-read.md |

**Proposal:** Freeze the first red: the harness keeping a failed test's home and the gate runner archiving each run's raw outcome file (outcomes.json, the junit) into the run dir before the next run overwrites it — project tooling. The torn-line case was already closed by the ndjson reader fold; the concurrent-boot case rode a recurrence watch to the Epoch 2 cleanup chunk.

### P8 — wrap-session/reconcile · `ambiguity.playbook-no-match` — 3 cases · weight 10 · rate 3/7 step-runs
**Pattern:** 3 of 7 reconciles met a proposal class no playbook rule covered — verbatim upstream copies outside obs §1, a locked-decision reversal, a predicate widening — and each ended with a rule appended or proposed (1 halt).

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| pty-wrapper-on-windows | reconcile | four proposals landed in verbatim upstream copies (security Threat Model Summary :42 :109, a11y §1 :98 :179) that the obs-§1-only verbatim rule did not cover; escalated E4, rule widened on approval (L276) | dialogue_rounds 1 | — |
| instance-state-and-start-order | reconcile | the [Snapshot writer] reversal of a locked decision matched no playbook rule; the operator P4 ruling plus the wrap directive settled it (apply), the rule proposal goes to the card (L362) | — | — |
| wrapper-channel | reconcile | The E2 fd-witness change (report: 'overseer-widened predicate') matched no playbook rule; escalated by unease under the founder's directive 1 and resolved live by the overseer as a premise fix, not a widening (L405) | dialogue_rounds 1, halted 1 | — |

**Proposal:** Each miss grew the playbook by one rule, which is the designed path. A generalization for the founder: a named default arm for "no rule matches" that classifies the proposal against a short list of the classes seen so far (verbatim-copy, locked-decision reversal, predicate widening/narrowing) before escalating, so the escalation arrives pre-classified.

### P9 — any step (universal / recall — grouped by type) · `contract.narrow-basis-claim` — 5 cases · weight 5
**Pattern:** 5 claims across 5 steps were stated from a narrower basis than the claim: grep counts written before the greps ran (research), a CARRY count from the first of two edit batches (gates), a session-end attributed from timing alone (code), a prior-wrap mention asserted without a search and an unmeasured mechanism stated as fact (curation ×2). All were caught and corrected in-pass.

**Evidence:** ALL 5 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| — | curation | route-resolve's item-4 disposition stated as fact that fuzz_target! reaches arbitrary through libfuzzer-sys's re-export; curation's measurement attempt found libfuzzer-sys absent from the host registry, so the mechanism was unmeasured; the adaptation record and handoff relabel it a hypothesis (the disposition stands on the measured spec pin) (L195) | extra_reads 1 | .andromeda/runs/2026-09-24T18-00-38-wrap/adaptation-record.md |
| security-prerequisites | curation | the recurrence record asserted a prior wrap had named the formatter-reflow hazard without any search establishing it; no such prior entry was read, so the recurrence premise is unsupported (L237) | — | .andromeda/runs/2026-09-25T13-11-43-wrap/curation.md |
| pty-wrapper-on-windows | code | the 16:44:58 session end was attributed to the contract_diag_schema Ctrl-C hang from the timing of an exit-137 call alone; per the operator the window was closed by hand, and a hung test cannot close a window (L266) | — | viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/evidence/ |
| ci-chunk-base-and-union-verdict | research | research.md first stated AGENT_RUN_CHUNK_BASE at 3 hits and an a11y 0-hit grep over e2e-web\|playwright\|--browser before either was run; re-derivation gave 4 hits and confirmed 0 (L290) | extra_reads 1 | viola-0.1.0/chunks/2026-09-26-ci-chunk-base-and-union-verdict/research.md |
| instance-state-and-start-order | gates | the route-resolve CARRY count was taken from the first edit batch and missed the second two-CARRY edit on :40; the route.py pins listing reads 8, and the handoff line was corrected before the commit (L375) | — | .andromeda/runs/2026-09-27T06-12-23-wrap/pins-after.txt |

**Proposal:** A convention that every count/absence/prior-mention claim in research.md, report.md and the handoff carries the command (or tool listing) that produced it — the route.py pins listing already does this for CARRY counts and caught the gates case.

### P10 — implement/fix-loop · `tooling.environmental` — 3 cases · weight 8 · rate 3/11 step-runs
**Pattern:** 2 of 3 cases are rust-analyzer holding mutants.out (os error 5), in consecutive chunks, stopped by hand before every leg because the LSP restarts after edits; the third is a stale apt list 404 in the WSL provision.

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| pty-wrapper-on-windows | fix-loop | cargo-mutants mutants.out rename denied (os error 5) while the session LSP rust-analyzer watched the workspace; renames succeeded only after stopping it (L261) | iterations 1, retries 2 | — |
| ci-chunk-base-and-union-verdict | fix-loop | cargo-mutants could not rename mutants.out: rust-analyzer held it (os error 5); the LSP restarted after edits so the stop was repeated before each of three leg runs (L296) | retries 1 | viola-0.1.0/chunks/2026-09-26-ci-chunk-base-and-union-verdict/evidence/mutants-leg-windows-local.md |
| local-linux-pre-push-gate | fix-loop | build-essential install exit 100: security.ubuntu.com 404 on libc-dev-bin/linux-libc-dev/libc6-dev under stale package lists; apt-get update fixed it (L318) | iterations 1 | chunks/2026-09-26-local-linux-pre-push-gate/evidence/provision.md |

**Proposal:** Remove the cause in tooling rather than by rule: `run --mutants` pointing cargo-mutants' `--output` outside the LSP-watched workspace (or stopping the session's rust-analyzer by exact ExecutablePath itself as its first action), and `scripts/wsl-provision.sh` running `apt-get update` before its installs. See L4 for the level view.

### P11 — any step (universal / recall — grouped by type) · `tooling.host-shell` — 3 cases · weight 7
**Pattern:** 3 host-shell corruptions in 2 chunks: `wsl.exe -- <cmd>` re-parsing argv through the distro login shell, MSYS rewriting `/usr/bin/printenv` inside a `wsl.exe --exec` call, and a python bytes literal failing on an em dash.

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| local-linux-pre-push-gate | research | wsl.exe with `--` joins argv and re-parses it through the distro's login shell, so a quoted probe script lost its variables; --exec passes argv verbatim (L310) | retries 1 | — |
| instance-state-and-start-order | fix-loop | MSYS argument conversion rewrote /usr/bin/printenv in a wsl.exe --exec call into a Windows path; re-run with MSYS2_ARG_CONV_EXCL (L349) | retries 1 | — |
| instance-state-and-start-order | reconcile | a python edit script with bytes literals failed on a non-ASCII em dash; converted to str literals encoded at use (the conversion itself was blocked once by the bash guard for a doubled backslash and went through a file) (L364) | retries 2 | — |

**Proposal:** A project wrapper for the WSL crossing (`scripts/wsl-exec.sh`: `MSYS2_ARG_CONV_EXCL='*' wsl.exe -d <distro> --exec "$@"`) so the two WSL recipes are one tool call instead of two remembered rules — the same cases recur under P6.

### P12 — implement/fix-loop · `contract.spec-reality-gap` — 2 cases · weight 10 · rate 2/11 step-runs
**Pattern:** 2 fix-loop gates could not pass for reasons outside the chunk's scope — a tests-only Rust delta that the harness grades outcomes-missing by ruling (halt), and a per-test-home pinned exe copy that exhausts the Linux mutation tmpfs (soft-exit); both resolved by an operator scope widening.

**Evidence:** ALL 2 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| security-prerequisites | fix-loop | plan step 10 / acceptance forecast mutants verdict counted and green for a tests-only Rust delta; test-plan §10 (operator ruling, test-plan.md:1790) and the harness make a Rust delta with no fresh outcomes.json red (outcomes-missing), so the gate cannot pass without a harness/spec change outside scope (L226) | iterations 1, halted 1 | .andromeda/runs/2026-09-25T12-41-13-implement/gate-run-3.txt |
| instance-state-and-start-order | fix-loop | the per-start pinned copy of the running exe lands in every test home (38 MB debug on Linux); homes kept by AGENT_RUN_KEEP_HOMES or left by fail-fast-killed tests exhaust disk across the mutation legs, so the plan mutation gate cannot pass (L347) | iterations 1, soft_exit 1 | .andromeda/runs/2026-09-27T02-03-01-implement (gate entry 21) |

**Proposal:** A P5 known-answer check of the mutation gate against the diff's SHAPE (tests-only Rust delta → the harness's outcomes-missing arm; a new per-home artifact × mutants × legs → a disk estimate) so the forecast in the plan is checked against the harness's own verdict table before implement.

### P13 — wrap-session/report · `input.implement-outcome-unsettled` — 3 cases · weight 6 · rate 3/7 step-runs
**Pattern:** In 3 chunks the report's outcome basis was not implement's green: operator passes, CI runs and post-implement fix commits superseded it. The chain X2 below finds the same shape in a 4th chunk.

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| epoch-1-cleanup | report | implement reported green, but two CI operator passes after it surfaced a windows stall and a false-green unviable swamp; the outcome basis became operator pass 4 (run 36126924953) after two more harness fixes folded post-implement (L209) | iterations 3 | viola-0.1.0/chunks/2026-09-24-epoch-1-cleanup/evidence/ |
| pty-wrapper-on-windows | report | steps ran after implement: operator pass pushed 17ea8c7, two fix commits (c05e6e2 unix holder group, 7681c73 macOS scoping) and three CI runs; outcome basis = operator-pass.md incl. its uncommitted final sections (L272) | extra_reads 2 | viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/evidence/operator-pass.md |
| local-linux-pre-push-gate | report | implement soft-exited stuck; the operator then widened scope (product viola-pty fix) and ran experiments A/B and the operator pass, so the report's outcome basis is the resumed implement plus the operator-pass evidence (L327) | extra_reads 1 | chunks/2026-09-26-local-linux-pre-push-gate/report.md Outcome basis |

**Proposal:** The operator pre-CI pass is now a standing stage (a `chore({marker}): operator pre-CI commit` exists in 4 of 7 chunks). The direction: the report letter naming the operator-pass evidence (`evidence/operator-pass.md` + the CI run ids) as the canonical outcome basis whenever that commit exists, so the report reads a designed input instead of reconstructing a superseded one.

### P14 — any step (universal / recall — grouped by type) · `contract.structural-blind-spot` — 3 cases · weight 6
**Pattern:** 3 mechanisms missed by construction: no drift-base detector binds a harness-internal contract changing in code; local pre-push had no Windows llvm-cov test stage and a faster runner than CI (green twice on trees CI read red); cargo-mutants runs only the mutated package's tests, so root-level security tests could not kill a crate's mutants.

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| epoch-1-cleanup | reconcile | no drift-base detector covers a harness-internal contract (the run --mutants verdict and invocation) that changed in code while test-plan §3/§10 text lagged; the test-plan agent reused D-tests-obs-harness (whose test-plan↔obs-plan invariant held), and the arch and obs agents could only flag their restatements out of scope (L211) | extra_reads 2 | .andromeda/runs/2026-09-25T11-29-18-wrap/fanout-results.md |
| instance-state-and-start-order | fix-loop | pre-push green twice (1187 s, 1163 s) on trees CI read red: the canary-scan TIMEOUT needs a slower runner than the 32-core WSL leg, and the llvm-cov windows test job is not a pre-push stage; CI's first failure came at +88 s (L357) | iterations 1, retries 1 | viola-0.1.0/chunks/2026-09-27-instance-state-and-start-order/evidence/operator-pass.md |
| wrapper-channel | fix-loop | cargo-mutants runs only the mutated package's tests, so viola-channel mutants in the owner-only DACL and SQOS flags survived although the root security tests would kill them; closed by crate-level DACL read-back and flag pin tests (L395) | iterations 1 | — |

**Proposal:** Each was closed locally (the pre-push `windows-tests` stage landed in wrapper-channel; crate-level DACL/flag tests). The generalization is a CI↔local gate parity table — every CI job named with its pre-push stage or an explicit 'CI-only' reason — that a check can diff against ci.yml, so the next unmirrored job is visible before it greens a red tree.

### P15 — wrap-session/reconcile · `contract.false-positive-proposal` — 3 cases · weight 4 · rate 3/7 step-runs
**Pattern:** 3 reconciles received detector proposals that did not hold: a role widening the row did not need, collateral facts the report did not carry, and a proposal inheriting a false premise from the old spec text. (The untyped L404 record — 7 of 18 architecture proposals citing source lines against the prompt's ban — is the same family; see U1.)

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| security-prerequisites | reconcile | arch detector proposed widening the PTY layer row's windows-sys role to the SQOS open (dependent-of D-arch-decisions); the row's viola-pty kill-fallback claim stays true and the channel's use belongs to the IPC row — rejected (L233) | — | .andromeda/runs/2026-09-25T13-11-43-wrap/.raw-fanout-architecture.md |
| pty-wrapper-on-windows | reconcile | two proposals carried a collateral fact the report does not: test-plan T8 fake-agent cwd receipt as {path} (code: {cwd}); architecture A4 bad claude_env_keep reported as parse-rejected{parser:"config-json"}; both applied with the report fact only (L275) | extra_reads 1 | — |
| instance-state-and-start-order | reconcile | D-arch-resources proposed that a local #[allow(clippy::print_stderr)] sits on the new refuse helper; the root bin carries no print allow (writeln! on the locked stderr) — the old arch text was already false and the proposal inherited its premise; the body was re-derived from the code (L361) | reformulations 1 | .andromeda/runs/2026-09-27T06-12-23-wrap/validation.md |

**Proposal:** The detector prompts already ban re-derivation; the orchestrator's validation caught every case. A direction: a mechanical pre-filter on detector returns that rejects any proposal whose basis cites a path outside the report/spec set, before the orchestrator reads it.

### P16 — wrap-session/curation · `ambiguity.filter-borderline` — 3 cases · weight 3 · rate 3/8 step-runs
**Pattern:** 3 of the 8 curations filled Filter 5's cap of 3 by judgment because candidates tied (5 at 0.8; 2 at 0.7; 4 at 0.8), deferring 6 candidates to the handoff. One deferred learning (rust-analyzer, 0.7) recurred the next session (P6 case 1).

**Evidence:** ALL 3 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| epoch-1-cleanup | curation | five candidates tied at confidence 0.8 (three of them only via the no-other-home +0.2); the Filter 5 cap of 3 was filled by judgment (the operator convention and two measured design-deciding facts first), with no tiebreak rule for equal scores (L213) | deferred 2 | — |
| pty-wrapper-on-windows | curation | Filter 5 cap: two survivors tied at 0.7 (CI-form mutation leg on Windows; rust-analyzer holding mutants.out); the mutation-leg entry was taken as nearer the next chunk and the other deferred to the handoff by judgment (L278) | deferred 1 | — |
| wrapper-channel | curation | Four candidates tied at 0.8 for Filter 5's three slots (all measurement +0.4, detail +0.2, no-other-home +0.2); the sweep-hazard candidate was deferred by judgment, and the founder's boundary-widening ruling (0.7) fell below the cap and was routed to a playbook proposal [clause retracted: ts 2026-09-27T13:02:00Z (hand-authored; true time 12:54:15Z–12:58:22Z)] (L409) | deferred 3 | .andromeda/runs/2026-09-27T12-33-51-wrap/curation.md |

**Proposal:** A tiebreak rule for equal scores (e.g. prefer a candidate whose failure recurred in-chunk, then the nearest next-chunk relevance), or a cap that admits all ties at the cut score; and a handoff deferral that reaches its second session is re-scored first. The founder weighs cap size against the always-loaded tier's cost.

## Cross-step chains (starting heuristics)

### X1 — phase/plan →plan→ implement/fix-loop — 4 chunks
- security-prerequisites: consumer L225 (quality:wrong step 4 carried a mistyped NIST 448-bit message literal; step 10 forecast a green counted mutation verdict for a test-target-only Rust delta, which test-plan §10's operator ruling makes red) ← producer L221 outcome `ok` signals ['designed-dialogue:0bsd-licence-disposition', 'designed-dialogue:mit-copyright-holder', 'authority-resolved:research-measurement-over-security-extract-windows-targets', 'fence-parses-first-try']
- pty-wrapper-on-windows: consumer L258 (quality:thin entry 26 runs cargo-mutants on this Windows host without --leg, so cfg(unix) bodies this chunk adds (termios, ioctl) grade missed by construction; read in the CI form (--leg windows-2025, ok:true) with the union as the verdict) ← producer L249 outcome `ok` signals ['designed-dialogue:r8-strip-set', 'designed-dialogue:unix-raw-tty', 'designed-dialogue:viola-env-ownership', 'authority-resolved:a11y two-space refusal form yielded to design-system cli pattern 2 fixed-message form']
- local-linux-pre-push-gate: consumer L317 (quality:thin entry 6 (root apt install) lacked the apt-get update its P5 note flagged as a hypothesis; measured red exit 100 on 404s) ← producer L311 outcome `ok` signals ['designed-dialogue:base-timing', 'designed-dialogue:verdict-union', 'designed-dialogue:unix-test-form', 'designed-dialogue:wiring-home', 'planlint-caught-platform-slot']
- local-linux-pre-push-gate: consumer L323 (quality:thin widened on the operator's word at implement (scope item 6, steps 10-13); the window-test entry's exact-name filter selected nothing and was corrected to the regex form) ← producer L311 outcome `ok` signals ['designed-dialogue:base-timing', 'designed-dialogue:verdict-union', 'designed-dialogue:unix-test-form', 'designed-dialogue:wiring-home', 'planlint-caught-platform-slot']
- wrapper-channel: consumer L392 (quality:thin Test Commands ran as written; entry 20 acceptance premise for E2 (fds 0,1,2 and at most one other) falsified by measurement; v1-20 acceptance 'dispatch takes no conn input' untrue of the planned Dispatch(method, params) signature until conn was st) ← producer L383 outcome `ok` signals ['authority-resolved:arch Unix endpoint path yielded to security-plan Decisions Log amendment 2', 'authority-resolved:route Epoch 6 SDDL deferral yielded to security-plan listener mandate', 'authority-resolved:arch crate dep direction yielded to obs-plan §3 instrumentation hooks (Expected amendment)', 'plan-oversize']

Plan forecasts falsified at the gate: a mutation gate form that cannot pass with cfg(unix) bodies (pty), a counted-green forecast for a tests-only delta (security-prerequisites), a missing apt-get update the P5 note had flagged as a hypothesis (local-linux), an E2 acceptance premise (wrapper-channel). Producers were formally ok; two carried `plan-oversize` / 492-line signals. Correlates P12 and P2.

### X2 — implement/smoke →implement-outcome→ wrap-session/report — 3 chunks
- epoch-1-cleanup: consumer L208 (quality:thin implement's green was superseded: CI runs 36046091888 (stall) and 36118112104 (false-green windows swamp) led to two post-implement fixes (streaming+flags, cli.rs guard + swamp rule); the final basis is operator pass 4, run 36126924953) ← producer L207 outcome `ok` signals ['green']
- pty-wrapper-on-windows: consumer L270 (quality:thin no P4 report on disk; implement evolve records (12) + gate record stood in; superseded by the operator pass, 2 fix commits and 3 CI runs) ← producer L264 outcome `ok` signals ['green']
- local-linux-pre-push-gate: consumer L326 (quality:thin implement's first P4 report (stuck) was superseded by the operator's scope widening and experiments A/B, then the operator pass; the final green report and the operator-pass evidence stood in) ← producer L325 outcome `ok` signals ['green']

Implement's green superseded by the operator pass / CI (the 4th chunk, instance-state, has the same shape from implement/fix-loop). Correlates P13.

### X3 — phase/research →research→ implement/fix-loop — 3 chunks
- security-prerequisites: consumer L225 (quality:thin closure left the 0-mutant verdict as an unmeasured hypothesis; the harness's outcomes-missing arm and test-plan:1790 were not read) ← producer L219 outcome `ok` signals ['graph-not-applicable', 'unresolved-questions', 'mechanism-measured']
- pty-wrapper-on-windows: consumer L258 (quality:thin missed tests/contract_diag_schema.rs (hung on a swallowed Ctrl-C) and that the new crates need a no-op fake-agent feature for package-scoped mutant builds) ← producer L245 outcome `ok` signals ['unresolved-questions']
- instance-state-and-start-order: consumer L346 (quality:thin two companion oracles outside the lists: tests/cli_fake_agent.rs pins the start receipt; crates/viola-e2e/tests/harness_lifecycle.rs pins the readiness missing list) ← producer L338 outcome `ok` signals ['unresolved-questions']

Same producer, later consumer: the 0-mutant verdict left as an unmeasured hypothesis (security-prerequisites), missed companion tests and the no-op fake-agent feature (pty), companion oracles outside the lists (instance-state). Together with X4: research → implement in 4 of 7 chunks.

### X4 — phase/research →research→ implement/code — 3 chunks
- pty-wrapper-on-windows: consumer L254 (quality:thin companion sweep missed tests/cli_fake_agent.rs and tests/contract_diag_schema.rs, both drive viola run through piped stdin with an immediate Ctrl-C) ← producer L245 outcome `ok` signals ['unresolved-questions']
- instance-state-and-start-order: consumer L344 (quality:thin lists missed two consumers: harness_lifecycle.rs builds a logs Filter literal; viola-agent-claude has no serde_json dev-dep for the plugin JSON asserts) ← producer L338 outcome `ok` signals ['unresolved-questions']
- wrapper-channel: consumer L387 (quality:thin boundary lists held; viola-state already depended on tracing, so its Cargo.toml needed no edit; fuzz/Cargo.lock unlisted but moves with fuzz/Cargo.toml) ← producer L380 outcome `ok` signals ['unresolved-questions']

Research's companion/boundary sweep missed files implement then had to touch (cli_fake_agent.rs, contract_diag_schema.rs, harness_lifecycle.rs, a serde_json dev-dep). Producers all `ok` with `unresolved-questions`. This is the upstream end of level candidate L2.

### X5 — implement/smoke →conversation→ wrap-session/curation — 3 chunks
- local-linux-pre-push-gate: consumer L331 (quality:thin the implement window's conversation is gone (wrap resumed across a session boundary); candidates came from the report's Decisions & corrections) ← producer L325 outcome `ok` signals ['skipped']
- instance-state-and-start-order: consumer L366 (quality:thin checkpoint answered in a resumed window after the context-alarm pause; the curation trace is read from the run dir's curation.md, not a live window) ← producer L353 outcome `ok` signals ['recorded-not-rerun']
- wrapper-channel: consumer L408 (quality:thin the implement/operator-pass conversation was in the prior window; only the resumed wrap session is in this window) ← producer L399 outcome `ok` signals ['fired-boot-path', 'teardown-exact']

Curation reads a conversation that is gone — the wrap ran in a later window in all 3 chunks. Correlates L7.

### X6 — wrap-session/report →report→ wrap-session/reconcile — 2 chunks
- security-prerequisites: consumer L232 (quality:thin the report's Expected-amendments site list for the SHA-256 open-pick claim missed security-plan:265 (Data Protection); the security detector's own sweep found it) ← producer L231 outcome `ok` signals []
- pty-wrapper-on-windows: consumer L273 (quality:thin carried no statement on spans in the run path; obs-plan requires pty.spawn + seam spans and 0 #[instrument] exist workspace-wide, surfaced by the orchestrator at validate; fake-agent receipt field names absent (a detector invented cwd {path})) ← producer L270 outcome `ok` signals ['reconstructed']

The report omitted a master site or a span statement the detectors then needed (2 chunks; input.report-insufficient 2, below threshold).

### X7 — phase/plan →plan→ implement/code — 2 chunks
- pty-wrapper-on-windows: consumer L254 (quality:thin step 2 prescribed a thiserror PtyError while the plan own dependency probe asserts viola-pty deps == [libc, portable-pty, windows-sys]; resolved by a hand-written Display/Error impl) ← producer L249 outcome `ok` signals ['designed-dialogue:r8-strip-set', 'designed-dialogue:unix-raw-tty', 'designed-dialogue:viola-env-ownership', 'authority-resolved:a11y two-space refusal form yielded to design-system cli pattern 2 fixed-message form']
- wrapper-channel: consumer L387 (quality:thin step 12 hook-invoked/hook-decision corr rules contradict obs-plan D-07 and the budget scenario; step 5 names SecurityDescriptor::deserialize (needs a widestring U16CStr, no such dep) and ListenerOptionsExt::mode (Unsupported on macOS)) ← producer L383 outcome `ok` signals ['authority-resolved:arch Unix endpoint path yielded to security-plan Decisions Log amendment 2', 'authority-resolved:route Epoch 6 SDDL deferral yielded to security-plan listener mandate', 'authority-resolved:arch crate dep direction yielded to obs-plan §3 instrumentation hooks (Expected amendment)', 'plan-oversize']

Plan steps contradicting shipped manifests or spec (a thiserror PtyError beside a deps probe excluding it; corr rules contradicting obs-plan D-07).

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 6 in-epoch facts · 2 prior-epoch
**Theme:** The bash-guard / PreToolUse file-target guard blocks doubled backslashes and heredocs with a file target; each session re-routes through a scratchpad script or the Edit tool

**Facts:**
- pty-wrapper-on-windows · phase/research · environment · workaround · bash-guard hook blocked a grep carrying a doubled backslash (the escaped dot of .cmd); re-ran with a [.] character class (L245 2026-09-25T13:55:39Z-a#0)
- pty-wrapper-on-windows · phase/research · environment · workaround · bash-guard hook blocked this checkpoint as a one-line heredoc append (backslash in a note); re-authored as a scratchpad python script run by path (L245 2026-09-25T13:55:39Z-a#1)
- pty-wrapper-on-windows · implement/code · environment · workaround · bash-guard blocked a doubled-backslash python heredoc and a cat heredoc with a file target twice; edits re-done via the Edit tool and scratchpad scripts (L254 2026-09-25T15:19:15Z-a#3)
- instance-state-and-start-order · implement/code · environment · workaround · a cat heredoc append to src/run/mod.rs was blocked by the PreToolUse file-target guard; the Edit tool wrote it (L344 2026-09-27T02:15:13Z-a#2)
- wrapper-channel · implement/code · environment · workaround · bash-guard blocked an inline python carrying a doubled backslash; the FNV oracle ran as a scratchpad script with chr(92) (L387 2026-09-27T08:10:38Z-a#5)
- wrapper-channel · implement/smoke · environment · workaround · bash-guard blocked a pipe-path Test-Path carrying doubled backslashes; dropped it, the harness cleanup's own viola-client connect proves endpoint_gone (L399 2026-09-27T10:16:57Z-a#0)
- *prior epoch (lookback, read-only):* supply-chain-and-workflow-gates phase/research workaround: a cat heredoc writing scratch Cargo manifests was blocked by the PreToolUse hook; the files went through the Write tool and the probe ran as (L53 2026-09-24T09:26:43Z-a#0) · observability-gates phase/validate workaround: a PreToolUse hook blocks Write to any path containing /target/, including a scratchpad target/ tree; the control file was created by a pytho (L125 2026-09-24T12:20:24Z-a#1)

**Level hypothesis:** The obstacle is a standing environment control (the project's own guard hook) meeting Windows paths and escaped patterns; the fixes live in each step as a re-route, 6 times in 5 of 7 chunks and twice the epoch before.

**Proposal:** A sanctioned channel named inside the guard's refusal text ("write it to the scratchpad and run it by path") so the re-route costs no retry, or a guard allowance for a doubled backslash inside a grep/python pattern argument. Which of the two, if either, is the founder's call.

### L2 — band-aid — 7 in-epoch facts · 5 prior-epoch
**Theme:** Implement's file lists (research's modify-set; plan and research read-only at implement) — companion tests and manifests outside the lists are edited anyway, or product code is shaped to stay inside them

**Facts:**
- pty-wrapper-on-windows · implement/code · process · workaround · edited tests/cli_fake_agent.rs and tests/contract_diag_schema.rs outside the research modify-set as companions of step 7 (both raced a Ctrl-C ahead of the fake agent raw mode and hung or died) (L254 2026-09-25T15:19:15Z-a#1)
- instance-state-and-start-order · implement/code · process · workaround · logs --kind: public Filter kept two-field and a Query/query() added, because crates/viola-e2e/tests/harness_lifecycle.rs (outside the research lists) builds a Filter literal (L344 2026-09-27T02:15:13Z-a#0)
- instance-state-and-start-order · implement/code · process · workaround · plugin_files tests assert literal strings instead of parsing JSON: viola-agent-claude has no serde_json dev-dependency and its Cargo.toml is outside the lists (L344 2026-09-27T02:15:13Z-a#1)
- instance-state-and-start-order · implement/fix-loop · process · workaround · tests/cli_fake_agent.rs (outside the lists) updated: it pinned the start receipt shape that plan step 13 grows (L346 2026-09-27T02:48:45Z-a#2)
- instance-state-and-start-order · implement/fix-loop · process · workaround · harness readiness made staged (snapshot/heartbeat only after both process starts) so crates/viola-e2e/tests/harness_lifecycle.rs keeps its two-entry missing oracle unedited (L346 2026-09-27T02:48:45Z-a#3)
- wrapper-channel · implement/code · process · workaround · span-capture helper first added to src/main.rs test_support (not in the file lists), moved into the cmd/run.rs test module and main.rs restored (L387 2026-09-27T08:10:38Z-a#6)
- wrapper-channel · implement/fix-loop · process · workaround · cosmetic name_str edit to refuse_batch_script reverted so the pre-existing, stderr-unasserted function left the mutation diff; its killer would sit in an out-of-scope test file (L392 2026-09-27T10:15:29Z-a#5)
- *prior epoch (lookback, read-only):* diagnostics-plane implement/code workaround: the existing panic_hook_writes_exactly_one_line_without_payload test was merged into the plan-named panic_hook_writes_detail_line_with_paylo (L82 2026-09-24T10:31:39Z-a#2) · log-redaction-and-never-log-floor implement/code workaround: step 6 asked the new test to capture the home-level line for the same failure, but the role line needs ProcessCtx and only one test may set  (L107 2026-09-24T11:34:08Z-a#0) · observability-gates implement/code workaround: the operator asked for the clippy measurement to go into research.md; /implement may not edit research.md, so it went to the run dir (clippy (L129 2026-09-24T12:31:54Z-a#1) · supply-chain-and-workflow-gates implement/fix-loop workaround: plan gate 4's run text is invalid under cargo-deny 0.20.2 and plan.md is read-only at implement: ran the corrected --config form by hand (ex (L61 2026-09-24T09:39:16Z-a#0) · diagnostics-plane implement/fix-loop workaround: gate entry 12 not run by gate.py (not run — env c unset: the entry's own loop variable, no env key); ran its exact run text by hand in the r (L84 2026-09-24T10:38:06Z-a#0)

**Level hypothesis:** The lists are authored at research before companion oracles are known (chains X3/X4: research → implement, 4 of 7 chunks), and implement treats them as a hard fence. Four of the seven facts shaped the product or test to fit the fence (a two-field Filter kept, literal-string asserts because a Cargo.toml was unlisted, staged readiness to keep an oracle unedited, a reverted edit to keep a function out of the mutation diff).

**Proposal:** A sanctioned 'companion' class in the implement letter: a test or manifest that pins a shape the chunk changes may be edited, recorded in the report with its reason, and does not count as scope widening. Typed correlate: input.research-files-wrong (2, below threshold).

### L3 — override — 5 in-epoch facts · 4 prior-epoch
**Theme:** A pending chunk's scope widened or folded mid-implement by the operator/overseer — the rule: phase refuses a pending chunk and implement may not edit plan/research, so no skill path exists for a widening

**Facts:**
- security-prerequisites · implement/fix-loop · process · overridden · operator widened scope (option A) to the harness mutants.rs and its tests to add the explicit test-only-rust-delta verdict; test-plan §10 wording goes to the wrap as an amendment (L229 2026-09-25T13:00:45Z-a#0)
- instance-state-and-start-order · implement/fix-loop · process · overridden · the stuck soft-exit was resolved by an overseer ruling (option 1): research/plan file lists widened by direct edit for mutants.rs, the fixture sweep, a sysinfo dev-dependency and cli_fake_agent.rs (L351 2026-09-27T04:03:46Z-a#0)
- instance-state-and-start-order · implement/fix-loop · process · overridden · second overseer fold before the operator pass: the pre-push Linux mutation scratch moved off the /tmp tmpfs (L354 2026-09-27T04:48:02Z-a#0)
- wrapper-channel · implement/fix-loop · process · overridden · overseer widened the E2 predicate to 'the child holds nothing of viola's' (no socket, no path under the viola home, 0-2 the pty) with a remove-the-guard pair (L392 2026-09-27T10:15:29Z-a#0)
- wrapper-channel · implement/fix-loop · process · overridden · overseer folded the torn receipt read (tests/support/fake.rs, outside the diff) into the chunk, then the whole live-append ndjson reader class (11 sites onto tests/support/ndjson.rs) (L392 2026-09-27T10:15:29Z-a#1)
- *prior epoch (lookback, read-only):* fake-agent-and-test-data-fixtures phase/take-up overridden: Setup 5a red (mutants outcomes-missing on a Rust-free diff, run 35973118026) judged not intersecting; letter offers take-up-unowned or mint- (L27 2026-09-24T08:09:02Z-a#0) · diagnostics-plane phase/take-up overridden: operator pasted an overseer note mid-Setup folding an out-of-entry item (.claude/settings.json write-guard repair) into this chunk's scope a (L73 2026-09-24T10:06:26Z-a#0) · quality-gates phase/take-up overridden: PREREQ first action (read CI ubuntu mutants job) was performed by the overseer and delivered mid-turn (run 36005608858: 2 MISSED file_mode m (L146 2026-09-24T13:43:24Z-a#0) · observability-gates implement/code overridden: clippy 1.98.1 disallowed_macros ignores every allow inside obs_event! (only a crate-level allow in the caller works: throwaway 8-placement p (L129 2026-09-24T12:31:54Z-a#0)

**Level hypothesis:** 5 overrides in 4 chunks, all the same call (widen or fold now); L323#1 records the workaround shape (the fix-loop's 'edit plan + research' path used on the operator's word). The recurrence suggests the rule, not the chunks, is where the correction keeps landing.

**Proposal:** A documented widening verb or letter arm (e.g. `/andromeda-implement --widen` recording the operator's word, the edited lists and an Expected-amendment line), so the override becomes a designed transition with its own record instead of an override fact.

### L4 — band-aid — 3 in-epoch facts · 1 prior-epoch
**Theme:** rust-analyzer (the session LSP) holds mutants.out, so cargo-mutants cannot rename it (os error 5); stopped by hand before each mutation run

**Facts:**
- pty-wrapper-on-windows · implement/fix-loop · environment · workaround · this session's LSP rust-analyzer held a handle on mutants.out, so cargo-mutants' rename to mutants.out.old failed with os error 5; stopped rust-analyzer before each mutation run (it respawns on edits) (L258 2026-09-25T16:59:16Z-a#0)
- ci-chunk-base-and-union-verdict · implement/fix-loop · environment · workaround · rust-analyzer (session LSP) held mutants.out, cargo-mutants rename failed os error 5; stopped by exact ExecutablePath before each leg run (it restarts after edits), per the deferred 0.7 learning (L295 2026-09-26T20:35:27Z-a#0)
- ci-chunk-base-and-union-verdict · wrap-session/gates · environment · workaround · rust-analyzer stop-check run before the light gate's run --mutants (0 found running this time) (L304 2026-09-26T21:25:51Z-a#0)
- *prior epoch (lookback, read-only):* three-os-ci-headless-harness-skeleton implement/fix-loop removed-cause: my own Monitor tail -F held mutants.out open and survived TaskStop, blocking cargo-mutants' rename twice; the three leftover processes were  (L12 2026-09-24T07:04:20Z-a#1)

**Level hypothesis:** Curated into host-win32.md on 2026-09-24 and extended 2026-09-26; still paid in 3 chunks this epoch (+2 typed tooling.environmental, +1 recall.corpus-recurrence). The rule asks each run to remember a stop the tool could perform.

**Proposal:** Remove the cause in the harness (see P10): `run --mutants` performs the exact-ExecutablePath stop itself, or directs cargo-mutants `--output` to a directory the LSP does not watch.

### L5 — band-aid — 5 in-epoch facts
**Theme:** Committed run dirs and evidence pick up content that must not be committed or that enters the chunk diff: absolute host paths, mutants.out copies, a python bytecode cache, .rs control files, a stale chunk.diff

**Facts:**
- epoch-1-cleanup · phase/validate · process · workaround · known-positive control copies (.rs) minted in the committed run dir were deleted after reading and replaced by p5-controls.md, since untracked .rs files would enter the chunk diff that run --mutants classifies (L202 2026-09-24T18:26:11Z-a#0)
- epoch-1-cleanup · wrap-session/gates · process · workaround · the light gate's printed listing (redirected to the wrap run dir) carries absolute host log paths; it was moved to the session scratchpad before the commit so the committed run dir holds no absolute path (L215 2026-09-25T12:07:13Z-a#0)
- pty-wrapper-on-windows · implement/fix-loop · process · workaround · secret-scan hit its own ?t= pattern table inside target/agent-run/chunk.diff, residue of a prior run --mutants whose diff carries secret_scan.rs; removed the residue before the scan; the scan scope is a harness owner question, not narrowed here (L258 2026-09-25T16:59:16Z-a#1)
- ci-chunk-base-and-union-verdict · implement/fix-loop · process · removed-cause · orchestrator copied mutants.out/outcomes.json (absolute argv paths, test output) into evidence; deleted before any commit, counts recorded in a markdown file instead (L295 2026-09-26T20:35:27Z-a#1)
- local-linux-pre-push-gate · wrap-session/report · process · removed-cause · a python bytecode cache from the evidence guard scripts rode the pre-CI commit; deleted at the wrap (L326 2026-09-27T01:05:00Z-a#0)

**Level hypothesis:** 5 facts in 4 chunks, each removed by hand before a commit or a scan; nothing checks the run dir mechanically.

**Proposal:** A pre-commit/pre-CI hygiene check over staged `.andromeda/runs/**` and `evidence/**`: no absolute path, no `.rs`, no `__pycache__`, no raw cargo-mutants output — refusing the commit with the offending path.

### L6 — band-aid — 3 in-epoch facts (+1 friction) · 2 prior-epoch
**Theme:** The verbatim raw-twin discipline for agent returns — twins written as condensed indexes or field-kept transcriptions, saves paraphrased or reflowed

**Facts:**
- pty-wrapper-on-windows · wrap-session/reconcile · process · workaround · raw twins consolidated into fanout-results.md (one table per doc with dispositions) instead of per-doc .raw-fanout-{doc}.md files (L273 2026-09-25T18:03:46Z-a#0)
- ci-chunk-base-and-union-verdict · phase/distill · process · removed-cause · orchestrator folded nested sub-bullets in the arch and tests saves; restored to the raw nesting by anchored Edits so saved == raw and no twin was owed (L288 2026-09-26T19:38:37Z-a#1)
- wrapper-channel · wrap-session/reconcile · process · workaround · raw fan-out twins for architecture and test-plan were written as field-kept transcriptions (sidecar/rationale lines omitted), not byte-verbatim agent returns; fanout-results.md says so (L403 2026-09-27T12:54:15Z-b#0)
- pty-wrapper-on-windows · phase/distill · untyped friction · the orchestrator's first save of layouts.md was a paraphrased reconstruction instead of the verbatim return body; caught before validation and re-saved verbatim via a second whole-content Write (L244 2026-09-25T13:41:00Z-c)
- *prior epoch (lookback, read-only):* fake-agent-and-test-data-fixtures phase/distill workaround: arch return arrived with a harness neutralization line and <\ rewrites in two harness-turn prefixes; saved copy restored the literal < and d (L29 2026-09-24T08:13:24Z-a#0) · quality-gates wrap-session/reconcile workaround: raw fan-out twins for architecture/security/test-plan written as a condensed per-proposal index (detector · section · basis) rather than the (L163 2026-09-24T15:05:18Z-a#0)

**Level hypothesis:** 3 facts + 1 untyped record this epoch, 2 facts the epoch before: the orchestrator transcribes large returns and the transcription drifts from verbatim (size-bounding, nested bullets folded, a paraphrased save).

**Proposal:** The twin produced by the transport rather than transcribed: the agent writes its own return body to `{run_dir}/.raw-*.md` as its last act (or the orchestrator saves the tool result by reference), so byte-verbatim holds by construction.

### L7 — chronic-degrade — 2 in-epoch facts + 2 step records
**Theme:** A wrap does not fit one context window: resumed across a session boundary or paused at the context alarm, its evolve checkpoints degrade to ok-degraded and curation reads a conversation that is gone

**Facts:**
- local-linux-pre-push-gate · wrap-session/reconcile · process · workaround · wrap resumed across a session boundary: the prior window paused at 86% context after 13 of 24 body applies; this window applied the rest from p2-apply-state.md and ran the whole cascade, so the fan-out/validate half of this step ran in a window whose trace is gone (fan-out counts taken from fanout-results.md: arch 10, security 4, test-plan 7, four docs 0; 19 accepted, 2 rejected as registry over-reach, 5 orchestrator-raised) (L329 2026-09-27T01:12:11Z-a#0)
- wrapper-channel · wrap-session/report · resources · deferred · P1 paused at the context alarm: its evolve checkpoint, the drift-base check-field pre-scan against the Changes bullets and the per-master grep counts behind Expected amendments were not done (resume.md); the report says the search basis is owed to P2's detectors (L402 2026-09-27T12:54:15Z-a#0)
- wrapper-channel · wrap-session/curation · outcome `ok-degraded` · conversation: thin — the implement/operator-pass conversation was in the prior window; only the resumed wrap session is in this window (L408)
- instance-state-and-start-order · wrap-session/curation · outcome `ok` · conversation: thin — checkpoint answered in a resumed window after the context-alarm pause; the curation trace is read from the run dir's curation.md, not a live window T2 1 new + 1 extension (testing.md); 6 filtered; 4 recurrences to the handoff (L366)

**Level hypothesis:** 3 of 7 wraps this epoch crossed a window (local-linux-pre-push-gate resumed at 86%; instance-state-and-start-order after a context-alarm pause, L366; wrapper-channel paused at 92%). Outcomes L402 and L408 are ok-degraded; chain X5 shows curation reading 'conversation: thin' in the same 3 chunks. Never halts, so the halt policy never surfaces it.

**Proposal:** Either a wrap split into two designed sessions (P1-P3 · P4-P7 with a resume file as the contract), or each checkpoint's evolve answer written at the step into the run dir so a later window can append it without the trace.

**Signatures with no hit:** deferred-forever — every in-epoch `deferred` fact names its route (L280 → the Local Linux pre-push gate chunk, since complete; L285#1 → surfaced to the operator; L402 → P2's detectors, same wrap).

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — wrap-session/reconcile — 3 cases (+3 prior epoch) → proposed type `contract.in-pass-correction`
**Cluster:** a proposal or an apply was wrong on first write and a later mechanism in the SAME pass caught it —
- instance-state-and-start-order · the fan-out missed four stale restatements that the cascade sweep caught (test-plan:529, :1775, obs-plan:869, security-plan:671) and a wider leaf grep found six leaf lines the sweep patterns did not name (they stated the chunk's NEW facts' predecessors, not retired wording) (L363 2026-09-27T06:33:04Z-d)
- wrapper-channel · The architecture detector cited source/manifest lines for 7 of 18 proposals (endpoint.rs, server.rs, frame.rs, Cargo.toml) against its prompt's re-derivation ban and carried unreported facts (dir fallback, lock mode, unlink order, dependency versions); all 7 were rejected and their report-carried facts re-raised by the orchestrator (L404 2026-09-27T12:54:15Z-c)
- wrapper-channel · An architecture [Session Liveness] edit removed a still-true clause (a name with an answering endpoint refuses) beyond the retired pending-bind claim; caught on re-reading the edit and restored before the sweep (L407 2026-09-27T12:54:15Z-f)
- *prior epoch:* the cascade citation fold edited obs :202, which sits in obs §1 — a verbatim copy of obs-scope whose pending wording is kept by rule (obs :485); rever (L91) · The obs detector proposed 'append a new Decisions Log entry' with no number; the apply first minted D-27, which already existed (the log runs to D-32) (L139) · three of the orchestrator's own applies were wrong on first write and caught in-pass: a target/perf/ line placed perf/*.json under it (spec check: rep (L186)

**Draft criteria line (reconcile playbook):** "Did a later mechanism in this pass (the cascade sweep, a re-read of the edit, the orchestrator's validation) catch a proposal or an apply that was wrong on first write — a restatement the fan-out missed, an edit removing a still-true clause, a proposal citing outside its basis? Record which mechanism caught it and what the first write was." (The detector-side half overlaps `contract.false-positive-proposal`, P15; the founder may prefer to widen that type instead.)

## Below threshold — no action

**Typed groups:**
- `implement/fix-loop/ambiguity.scope-pressure` n=2 w=7 — instance-state-and-start-order: two red oracles sat outside research lists: cli_fake_agent.rs edited as the planned receipt change own oracle; harness_lifecycle.rs avoided by staging readiness …
- `implement/code/input.research-files-wrong` n=2 w=6 — pty-wrapper-on-windows: research companion sweep for Wrapper\|booted_wrapper\|stdin.*x03\|Stdio::piped omitted tests/cli_fake_agent.rs and tests/contract_diag_schema.rs; both needed the start-recei …
- `implement/fix-loop/contract.test-expectation` n=2 w=4 — security-prerequisites: the plan's FIPS 180-2 448-bit message literal dropped 'klmnlmnomn' (…jklmmnopnopq); the digest literal was right, so the KAT went red; fixed to the published message, ver …
- `implement/code/input.plan-step-ambiguous` n=2 w=3 — pty-wrapper-on-windows: plan step 2 (PtyError via thiserror) contradicted plan gate 17 (viola-pty normal deps exactly libc, portable-pty, windows-sys) …
- `wrap-session/route-resolve/ambiguity.trajectory-halt` n=1 w=5 — —: operator direction named the entry's slot but left its item contents to the orchestrator; the proposal went to dialogue, where the first print did not reach the overseer 
- `wrap-session/reconcile/input.report-insufficient` n=2 w=2 — security-prerequisites: report Expected-amendments listed security-plan 608/609/611 + 205/373/375 for the open-pick claim but missed :265, where the Data Protection bullet restated it; the detec …
- `phase/distill/contract.binding-contradiction` n=2 w=2 — pty-wrapper-on-windows: a11y extract states the CLI refusal as `unable  <reason>  <detail>` (a11y-plan §8) while design and layouts fix the exit-1 start refusal as `unable: <cause>` (design-syst …
- `implement/code/tooling.hook-friction` n=1 w=3 — wrapper-channel: the PostToolUse formatter reflowed a region between edits, so two Edit anchors written against the pre-format text matched nothing
- `implement/smoke/tooling.harness-friction` n=1 w=2 — pty-wrapper-on-windows: agent-run boot leaves supervise holding the stdout it inherited, so boot \| <pipe> never reaches EOF while the session runs (pre-existing: boot.rs untouched by this chunk)
- `*/*/contract.token-proxy-check` n=1 w=2 — —: hand-written section-marker extractor mismatched the CLAUDE.md start/end marker grammar: checks 1, 2 and 14 read region-not-found / 0 markers / 0 pointer rows (false red 
- `implement/fix-loop/contract.instrument-validity` n=1 w=2 — local-linux-pre-push-gate: the resize-red instrument dumped only from the test's own 10 s deadline assert; the one instrumented recurrence ended as a nextest TIMEOUT at 10.006 s in the mutants base
- `phase/research/input.extract-signal-gap` n=1 w=1 — epoch-1-cleanup: a11y extract stated run --browser must still exit 2 browser-linux-only / browser-missing after the split; at HEAD --browser is unbuilt and clap refuses it as usage (cli.r
- `new-session/orientation/input.handoff-git-mismatch` n=1 w=1 — —: handoff Status clean / Position Next /andromeda-phase vs measured: chunk promoted (master pending), 3 chunk commits on HEAD, operator-pass.md uncommitted with an OPEN uni
- `*/*/recall.change-reconstruction` n=1 w=1 — pty-wrapper-on-windows: Files/Symbols/Crates/Dependencies/Schema/Coverage all reconstructed by git diff 0253507 + pub-item grep + research.md facts; no implement conversation in the window
- `wrap-session/curation/ambiguity.tier-routing` n=1 w=1 — pty-wrapper-on-windows: the relayed-claim-stays-hypothesis learning fit Tier 1 (universal directive) or Tier 3 (pipeline process context); routed Tier 1 as a one-sentence directive under 600 B
- `wrap-session/route-resolve/contract.no-sanctioned-channel` n=1 w=1 — pty-wrapper-on-windows: secret-scan reading target/agent-run/chunk.diff residue (implement friction 16:59:16Z-e, "a harness owner question") has no route owner and no directive disposition; reco
- `wrap-session/gates/tooling.result-not-run-stable` n=1 w=1 — ci-chunk-base-and-union-verdict: the light gate's windows leg over the same Rust tree read caught 74 / unviable 5, after implement's 73/6 and 71/8; verdict green each time, no artifact froze a count
- `implement/code/input.conventions-gap` n=1 w=1 — wrapper-channel: research measured the listener controls but not their preconditions: SecurityDescriptor::deserialize takes a widestring type outside the deps, and mode() is Unsupported o

**Untyped clusters (emerging — watch next epoch):**
- cascade.py pattern-id length refusal (L330, L406; + problem fact L300#0) — see the themes list below.
- an already-curated rule reproduced at fix-loop (L356 pipe-captured boot, L358 substring stop) — the same events P6 counts at curation; `recall.corpus-recurrence` is not in the fix-loop playbook's list, so the fix-loop records them untyped (a double record of one event).
- singletons: L244 distill paraphrased save (folded into L6) · L257 a mis-diagnosed mechanism coded with a measured-claim comment · L319 a red reproducible only through the gate under test · L391 a scoped tracing subscriber caching a callsite as disabled · L412 hand-authored ts (see Mechanism health).

**Pass-A themes below threshold / observations:**
- MSYS / WSL argument crossing (environment; L309#0, L309#1) — 2 facts; the same crossing appears as typed tooling.host-shell (P11) and recall.corpus-recurrence (P6) — covered there.
- code-graph query read through | tail against the cookbook (process; L289#0, L338#0) — 2 facts, 2 chunks, no prior-epoch recurrence.
- cascade.py 16-char pattern-id limit not stated in the reference (process; L300#0, L330, L406) — 1 problem fact + 2 untyped friction records = 3 occurrences in 3 chunks; each stream alone is below its threshold. Cheapest fix on this page: state the limit in the cascade section, or print it in the refusal.
- light-gate --skip overriding the source-delta arm (process; L283#0) — 1 in-epoch; 2 the epoch before (L137, L143) — recurring across epochs at n=1 in-epoch.
- mutation equivalence shaping product code (fn -> const, literal consts, no join, visitor coverage) (product-logic; L205#0, L351#1, L392#4, L294#0) — At the count threshold, but nature product-logic sits outside the band-aid signature — an observation: the mutation gate is a design force on product shape. Correlates: P3, P14.

**Other override / deferral facts (single, no recurrence on the same rule):** L217 take-up fold (licence item) · L285#0 epoch split placed below the valve's position · L313#0 planlint check-9 HIT left standing by ruling · L360 two escalate-class items settled by recorded directions · L285#1 the Epoch 2 header still names “events, ledger”, left unrenamed because the ledger keys epochs byte-exact on header text.
