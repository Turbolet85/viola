# Fan-out results — 2026-10-14-resume-and-compact-ledger-rows

**Snapshot pair:**
- before: `tree 246688d9026a7e57c8375abd4978ce0af400fddf index 6e47f96bc0820d662d4e1a53e47559d481552268 head
  784d9f9937d63efbcdc1a7bf19564e9e430bceee`
- after: `tree 8b3322181afe58d06d5da76475fa878bb2d64a14`, index and head unchanged.

`git diff-tree` printed exactly seven `A` lines, one per `fanout-{doc}.md`. No file needed a fix (entities 0, every
file opens with `proposals:`), so there are no twins.

**Receipt mismatch:** the architecture agent's receipt said `proposals: 21`, but its file holds 22 items. The file is
the truth.

## architecture: 22 proposals · fanout-architecture.md
note: the `local-cmd` sites :91 and `project-directory-structure.md:37` still read true; the `/compact` row's owner is
the operator's word at this wrap; no stack drift; the killed-exit leaves are the cascade's.
1. [CLI Version Compatibility] Ledger rows, the `resume-session` sub-bullet → apply (playbook: Accurate this-chunk addition).
2. [CLI Version Compatibility] ledger paragraph, 22 rows, relied 22/19, older 18/21-id stamps unverified, a 23rd row → apply (Accurate this-chunk addition; a count with its rule, `LedgerRow::relied()`).
3. [CLI Version Compatibility] verify run description, Run E → escalate (Boundary widening the founder answered himself to a P4 fork: answer card).
4. [CLI Version Compatibility] `--record` writes `SessionStart.resume-1.json` → apply (Accurate this-chunk addition).
5. §Stack Screen model, four → five interactive runs → apply (Accurate this-chunk addition).
6. [Plugin Scope] verify's children gain Run E → escalate (answer card).
7. [Delivery Confirmation] `LOCAL_COMMANDS` 2 → 3, `/compact` → apply (Accurate this-chunk addition; the founder's fork 1 answer `inputs#I1` names it).
8. [CLI Version Compatibility] Local commands sub-bullet, `/compact` "none", its post-condition row owed → apply (Accurate this-chunk addition; the owner from the halt, question 3).
9. Capability ledger as the single gate: the "no row yet" claim retired; `/compact` owed; 22/19 → apply (Accurate this-chunk addition; the owner from the halt, question 3).
10. Occupied Resources Filesystem, Run E's `-resume` dir → escalate (answer card).
11. Occupied Resources Filesystem, probe dir `resume/` → escalate (answer card).
12. Occupied Resources Claude Code integration names, Run E's argv → escalate (answer card).
13. Occupied Resources diagnostics, spawn pairs 6 → 7 → apply (Accurate this-chunk addition).
14. Occupied Resources diagnostics, killed-at-stop pair carries `child_exit_status` → escalate (no playbook rule; it changes a log-field contract, so it goes to the halt as question 2 with obs-plan 6 and 7).
15. Repository: new fixture stem, the 2.1.287 set 31 → 32 → apply (Accurate this-chunk addition; count with rule `ls fixtures/claude/2.1.287 | wc -l`).
16. Repository: the 2.1.287 stamp at 22/19 → apply (Accurate this-chunk addition).
17. Binary, subcommands and exit codes: the fake agent's `--resume` on the recorded variant → apply (Accurate this-chunk addition).
18. Standard Contracts, Ledger stamps envelope: relied 22/19 → apply (Accurate this-chunk addition).
19. Infrastructure Patterns → CI/CD approach: five runs, 22/19 → apply (Accurate this-chunk addition).
20. Infrastructure Patterns → Crate dependency direction: five runs, 22 rows → apply (Accurate this-chunk addition).
21. Infrastructure Patterns → Crate dependency direction: `resume_target`, `resume_variants`, `Probes::resumed` → apply, if the key enumerates the ledger's public API (read at apply); else reject (Registry over-reach).
22. Infrastructure Patterns → Project directory structure: `verify/typed.rs` five runs → apply (Accurate this-chunk addition).

## security-plan: 8 proposals · fanout-security-plan.md
note: D-security-deps had no proposal (no dependency). Line :600 states the local-command list generically and stays
true. Line :288's scrub already covers the new fixture.
1. §Input Validation, Session id as a child argv value: the third source → escalate (answer card).
2. §Threat Model Summary, the revive entry: `resume_args`' second caller → escalate (answer card).
3. §Threat Model Summary, the verify entry: five children, Run E → escalate (answer card).
4. §Input Validation, Constants MAX_FRAME: five runs A–E → escalate (answer card).
5. §Data Protection, Probe captures `resume/` → escalate (answer card).
6. §Data Protection, Interactive probe dirs `-resume` → escalate (answer card).
7. §Data Protection, Accepted risk (verify's trusted runs): Run E named, no transcript → escalate (answer card).
8. §Security Anti-Patterns Universal: ledger 22 rows, relied 22/19, older stamps → apply (Accurate this-chunk addition).

## design-system: 0 proposals · fanout-design-system.md
note: the tokens invariant holds; pattern 5's counts and pattern 2's `/compact` wording are left to the orchestrator.
1. (raised by main, check 5) §Surface: cli pattern 5 `verify` (design-system.md:811): `[01/22]`, MM 22 on Unix and 19 on Windows (the 22nd row `resume-session` relied on every OS), the ledger-order list ending `· resume-session --resume <id> reopens the session under that id`, summary `22 pass` → apply (Accurate this-chunk addition).
   - Pattern 2 (:758) words the outcome generically ("local command, no measured post-condition") and covers `/compact`: no change, read.

## layout-templates: 3 proposals · fanout-layout-templates.md
note: Signature placement covers `/compact` generically; `local-cmd` read 0 rows.
1. Output structure — `viola verify` example block: 22 relied rows, `[22/22] resume-session …`, `22 pass` → apply (Accurate this-chunk addition).
2. Output structure — `viola verify` Usage paragraph: the ledger order ends `resume-session`; MM 22/19, the Windows set ending at `resume-session` → apply (Accurate this-chunk addition).
3. Output structure — `viola verify` failing-row example re-summed to 22 → apply (A count in a master that carries no rule: its rule is the Unix relied count).

## test-plan: 13 proposals · fanout-test-plan.md
note: D-tests-framework found no drift; :233 stays true; the killed-exit claim has no test-plan site.
1. §5 cli_verify: five runs, 22/19, summaries, seven spawn pairs, the resume-run stop case → apply (Accurate this-chunk addition).
2. §4 unit: the `resume-session` arm and its unit cases, relied 22/19, the older-stamp oracle → apply (Accurate this-chunk addition).
3. §5 `contract_ledger_probes`: `ROW_IDS` 22, n 22/19 → apply (Accurate this-chunk addition).
4. §1: Run E refreshes `SessionStart.resume-1.json`; the drift contract replays the resume sets → apply (Accurate this-chunk addition).
5. §7: the fake agent's `--resume` launch on the recorded variant → apply (Accurate this-chunk addition).
6. §7: the fake-agent options paragraph; the owed resume payload retired → apply (Accurate this-chunk addition).
7. §7: the hygiene walk covers the resume variant → apply (Accurate this-chunk addition).
8. §7: Recorded hook payloads row, `SessionStart.resume-<n>.json`, 32 files → apply (Accurate this-chunk addition).
9. §6 Path 2: `/compact` unconfirmable at once → apply (Accurate this-chunk addition).
10. §4 local-command decision: three entries, the `classify` rows → apply (Accurate this-chunk addition). The case count is re-read at apply; the proposal's `classify.rs:250-270` span is not carried into the body.
11. §3 → Bootstrap phases key: the no-screen `verify_window_` case at 42.98 s → apply only where the key already states the class's timing (read at apply); else reject (not that key's subject).
12. §3 → 5-command implementation: boot's stamp runs Run E, relied 22/19 → apply (Accurate this-chunk addition).
13. §3 → 5-command implementation: `--local-live` `LEDGER_ROWS` 22/19, `LIVE_PASS`, the 22-row firing → apply (Accurate this-chunk addition).

## obs-plan: 7 proposals · fanout-obs-plan.md
note: D-obs-stack and D-obs-pii found no drift; :312, :313, :631, :820 and :836 stay true; proposals 1, 5 and 6 edit
the :739 bullet, and 4 and 7 the :872 sub-bullet.
1. §4 Capability ledger → `verify`: seven spawn pairs, Run E → apply (Accurate this-chunk addition).
2. §4 `verify` CI bullet: five runs → apply (Accurate this-chunk addition).
3. §6 Child / shell spawns: five runs, seven spawns → apply (Accurate this-chunk addition).
4. §6 subject sub-bullet: seven pairs, no new subject → apply (Accurate this-chunk addition).
5. §4 `verify` step-line count 22/19 → apply (Accurate this-chunk addition).
6. §4 `verify` stop sentence: a killed child carries `child_exit_status` → escalate (question 2, with architecture 14).
7. §6 subject sub-bullet: the killed-at-stop exception retired → escalate (question 2).

## a11y-plan: 0 proposals · fanout-a11y-plan.md
note: no web element; the step line matches the generic `[NN/NN]`; the schemas are unchanged; P6 states no count.

## Halt
Question 1 is the answer card (`answer-card.md`: architecture 3, 6, 10, 11, 12; security-plan 1–7). Question 2 is the
killed-exit claim (architecture 14; obs-plan 6, 7). Question 3 is the `/compact` post-condition row's owner (the P5
review card's row 1, `inputs#I2`; architecture 8, 9 name it).

## Resolved at the halt (`inputs#I6`, `relay-2.md`)
- Question 1, the answer card: confirmed. The confirm is the overseer's as delegate, so it is PROVISIONAL until the
  founder's Epoch 5 batch. Architecture 3, 6, 10, 11, 12 and security-plan 1–7 → apply. Their sidecars name the
  ratification as the founder's own answer to fork 2 (`inputs#I1`, relayed verbatim by the operator), with the card
  confirm marked `delegate viola-overseer`.
- Question 2, the killed-exit claim: amend to measured (the overseer, a founder-delegated technical fork).
  Architecture 14 and obs-plan 6, 7 → apply, scoped to verify's interactive PTY runs. A non-PTY probe killed at the
  stop is recorded as not measured.
- Question 3, the `/compact` post-condition row: a new markerless entry at the end of Epoch 5, after "Optional plugin
  install" (the founder's own word «В конец Эпохи 5», relayed verbatim by the overseer). Architecture 8 and 9 name
  it; P5 mints it as a recorded direction.

## Applied
- **Test-plan proposal 10:** applied as the second halt question ruled (`inputs#I7`): the count dropped and the eight
  functions named.
- **Test-plan proposal 11:** applied. The key states the class's timing.
- **Architecture proposal 21:** applied. The `crate-dependency-direction` key enumerates the pure ledger module's API.
- **Phase 2 summary:**
  - 54 amendments applied: architecture 22, security-plan 8, layout-templates 3, test-plan 13, obs-plan 7, and
    design-system 1 raised by main.
  - Every one was applied; 0 proposals were rejected for a source the report does not carry.
  - 4 escalations were resolved over 2 halts (`inputs#I6`, `inputs#I7`).
  - Citations: 0 re-pointed · 0 changed · 0 stretched.
- **Masters edited:** architecture, security-plan, design-system, layout-templates, test-plan and obs-plan, plus key
  files.
  - Architecture's key files: `ci-cd-approach`, `crate-dependency-direction`, `project-directory-structure`.
  - Test-plan's key files: `5-command-implementation`, `bootstrap-phases-derive-for-route-setup-project`.
  - `registry.py check`: 0 defects on architecture, test-plan and obs-plan.
- **Sidecar entries:** seven, each checked on-form, appended, and read back as its file's last line (architecture ×2,
  security-plan, test-plan, obs-plan, design-system, layout-templates).
- **Cascade:** `cascade-dispositions.md`; 49 leaf findings applied over 12 leaves.
