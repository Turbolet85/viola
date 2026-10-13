# Evolve Diagnosis — viola-0.1.0 · Epoch 4 — Session state & governance · 2026-10-12T06:42:44Z

Every proposal here is obligation-free: accept, reject, defer or modify it, and nothing on the mechanism's side
follows. Ids are ledger record ids (`.andromeda/friction-log.ndjson`); the raw query outputs are the `q-*.json`
twins beside this file.

## Mechanism health
- **Records:** 345 in the epoch (187 step / 158 friction), all live (no in-epoch record retracted). Ten chunks,
  plus 39 chunkless step records: 38 `new-session/orientation` and 1 `wrap-session/route-resolve` (the 2026-10-09
  0-pending adaptation wrap).
- **Coverage per chunk vs expected (phase 5 · implement 3 · wrap 5):** no gaps. Every chunk carries all 13
  checkpoints. Over-fires are re-entries: `epoch-3-cleanup` and `inner-cr-and-crlf-in-a-sent-text` (phase 8 ·
  implement 6, after a plan revision), `budget-governor` (phase 8 · implement 5, a revision and an implement
  re-entry at step 16), `windows-mutation-grade` (implement 4, a re-entry at step 11 that fired only `fix-loop`, by
  its own fact 2026-10-10T08:54:01Z-a).
- **Parse:** unparseable 0 · malformed-`ts` 0 · `id` fill 345/345.
- **Untyped rate:** 25 / 158 (15.8 %). By step: `new-session/orientation` 8/13 · `implement/fix-loop` 4/16 ·
  `implement/code` 3/23 · `wrap-session/reconcile` 3/33 · `phase/research` 2/11 · `phase/validate` 2/13 ·
  `phase/plan` 1/4 · `phase/take-up` 1/2 · `wrap-session/gates` 1/2. Orientation's rate is the outlier: four of
  its eight sit inside its own `ambiguity.ladder-uncovered-state` family (see U1).
- **Problem-fact fill:** 89 / 187 step records carry facts; 132 facts (88 workaround · 16 overridden · 12
  unresolved · 9 deferred · 7 removed-cause · 0 prohibition).
- **Signal line:** guard-refused 10 over 181 step records · exact-hit 30 over 10 curation records (from
  2026-10-09; 6 earlier step records are era). No odd signal strings.
- **Non-ok outcomes:** `route-resolve` halted-resolved 7/11 · `reconcile` halted-resolved 5/10 · `report`
  halted-resolved 1 · `fix-loop` halted-resolved 1 + soft-exit 1 · `curation` ok-degraded 1.
- **Retraction pre-pass (whole ledger):** retracted 2 (0 in this epoch) (2 unresolvable) · clause-retracted 3
  (0 in this epoch) · no retraction targeted by a retraction. The two unresolvable, verbatim, for the founder's
  manual discount (both Epoch 2, `id: null` degraded form):
  - 2026-09-27T07:06:36Z-b: "discount the signal carries-pinned-7 and the note '7 CARRYs' in step record
    2026-09-27T06:40:33Z-a: 8 CARRYs were pinned (:40 x4, :43, :65, :67, :73); a step-record signal has no
    record/clause retraction scope"
  - 2026-09-27T12:58:32Z-a: "discount the ts of step records 2026-09-27T13:02:00Z-a and 2026-09-27T13:10:00Z-a the
    same way; their content stands"
- **Calibration boundaries in range:** `guard-refused` / `exact-hit` live from 2026-10-09 (the six 2026-10-08
  orientation records are their era). The report's `new-text-*` words and reconcile's `rejected-for-*` counts
  (2026-10-08), the `operator-pass` entry (2026-10-07), the Universal types and `contract.in-pass-correction` all
  predate the epoch, so they are live throughout it.

## Proposals (typed patterns)

### P1 — */* · `recall.corpus-recurrence` — 14 cases · weight 29
**Pattern:** standing rules that are already written down, mostly in the always-loaded `host-linux.md`, recurred
14 times across 6 chunks, and no guard caught them. The repeated rules: a bare `cd` (4), an inline python heredoc
(3), a time typed ahead of the clock (2), and one each for: a filtered verdict read, a multi-KB line through a
scratch script, a bounded-repetition grep, `gh run list --commit` with a short sha, and an unstamped Windows home.
**Evidence:** ALL 14 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup-ii | three times written from an estimate, one eight minutes ahead of the clock; the stamp hook caught one, two re-read and corrected | iterations 1 · extra_reads 1 | — |
| 2026-10-10-windows-mutation-grade | `gh run list --commit` given an abbreviated sha printed an empty list; the ci.md entry of 2026-10-09 holds the remedy and was not loaded (no file under its path scope touched yet) | retries 1 | — |
| 2026-10-10-self-healing-state | a cd outside a subshell issued four times (implement 2, wrap 2); one moved the cwd into .andromeda | retries 1 | — |
| 2026-10-10-viola-revive | three orientation calls put a bare cd ahead of the command, past the always-loaded host rule; no guard refused | iterations 3 | — |
| 2026-10-10-statusline-pass-through | six findings matched standing entries stated correctly (heredoc with a file target, gate output through a filter, gh sha, fixed-column nextest split, files read after TestHome dropped, bare cd); all to the handoff's deferred list | iterations 6 | .andromeda/runs/2026-10-10T19-55-42-wrap/curation.md |
| 2026-10-10-budget-governor | most Bash calls of reconcile opened with a bare cd; one edit used a python heredoc; no guard refused either | iterations 1 | — |
| 2026-10-10-session-links | inline python heredocs for a source edit (implement) and multi-line reads (wrap), past the Transports entry | — | .andromeda/runs/2026-10-11T00-33-08-wrap/curation.md |
| 2026-10-10-session-links | a time typed from memory ahead of the clock, caught before it was pasted | — | same |
| 2026-10-10-session-links | a cd at the head of a compound in the session's first call | — | same |
| 2026-10-10-session-links | a verdict tool's output read through a line filter once | — | same |
| 2026-10-10-session-links | multi-KB master lines read through a scratch script once more | — | same |
| 2026-10-11-the-board-viola-list | a bounded-repetition grep sent to the embedded ugrep and refused, against the global rule naming it | retries 1 | — |
| 2026-10-11-the-board-viola-list | one master edit run as an inline python heredoc carrying document text | — | — |
| 2026-10-11-the-board-viola-list | four list cases booted from unstamped homes, red on windows-2025; the fact sits only in test-plan and tests-summary, no auto-loaded rule | iterations 1 | viola-0.1.0/chunks/2026-10-11-the-board-viola-list/evidence/operator.md |

**Proposal:** curation already writes each of these into the handoff's deferred-learnings list, and the rules were
correct when they recurred. So the rule text is not what's missing. What's missing is enforcement where the rule is
read. One direction: for each rule that recurs ≥ 2 epochs, either add a PreToolUse guard that rejects the forbidden
form (a heredoc-to-python edit, a bounded repetition sent to grep, `gh run list --commit` with < 40 hex), or reword
the rule to match what the guard already accepts (see L2 for `cd`). A rule that matters before its path scope fires
(the gh sha in `ci.md`) would move to an always-loaded home or a guard. Correlates: L2, L3, L4, U3.

### P2 — wrap-session/reconcile · `contract.in-pass-correction` — 8 cases · weight 25
**Pattern:** in 8 of 10 reconciles, a first write held an unread claim (a count, a heading, a name, a date) that a
re-read corrected before it landed.
**Evidence:** ALL 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | 3 texts corrected after first write: a "since that chunk" pointing at the wrong chunk; an unclear empty-text clause; a fan-out record claiming an entity probe not run | retries 3 | — |
| 2026-10-09-epoch-3-cleanup-ii | a correction said no master carries the reading; it stands as 78 m in one key file | iterations 1 | — |
| 2026-10-09-epoch-3-cleanup-ii | cascade-dispositions said six leaves re-read; one never opened, three read at sweep rows only | iterations 1 | .andromeda/runs/2026-10-10T01-25-38-wrap/cascade-dispositions.md |
| 2026-10-10-self-healing-state | a chaos-suite reason written as bare fact; a re-read added the chunk that measured it | retries 1 | — |
| 2026-10-10-viola-revive | five first writes: three sidecar payloads over the bound, an invented heading cite, two guessed section names, a wireframe using a test id as sample | iterations 5 · extra_reads 3 | — |
| 2026-10-10-statusline-pass-through | a sidecar payload naming a Supersedes heading the sidecar lacks; a doubled option list; "The other" kept after the count became three | iterations 3 | — |
| 2026-10-10-budget-governor | an example override end written with a calendar date ahead of the clock | iterations 1 | — |
| 2026-10-10-session-links | "an eighth root case" with the count not taken; "19 hits" where the listing held 18 | iterations 2 | .andromeda/runs/2026-10-11T00-33-08-wrap/fanout-results.md |

**Proposal:** two of the classes can be checked by a tool. (a) A sidecar entry that cites a heading the sidecar
does not hold (3 cases) could be refused by the sidecar append tool, the same way it already refuses an oversized
payload. (b) A count written without its basis (4 cases) is the CLAUDE.md learning "a count carries the rule that
produced it" in action; a reconcile pre-apply checklist line ("every number and every cited heading in this pass's
text was read in this pass") would turn the re-read into a step instead of a catch.

### P3 — wrap-session/route-resolve · `ambiguity.trajectory-halt` — 3 cases · weight 18
**Pattern:** three route-resolve runs halted to ask where an ownerless follow-up goes: a founder ruling with no
route slot, owed items with no owner, and an in-version follow-up. Each was answered in one round, and two of the
three took the recommended option.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | a founder ruling named a behaviour and no route slot; one question, four armed dispositions; answered: an entry of its own at the head | dialogue_rounds 1 · halted 1 | — |
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | two owed items with no route owner (a ledger row, a build measurement); one card, two questions, both recommended options taken | halted 1 · dialogue_rounds 1 | — |
| 2026-10-09-epoch-3-cleanup-ii | four mutants caught only by the per-test kill, no obvious owner; four priced placements, answered in one round | dialogue_rounds 1 · halted 1 · extra_reads 4 | .andromeda/runs/2026-10-10T01-25-38-wrap/relay-3.md |

**Proposal:** read P3 with P16 (`contract.carry-no-owner`, 4) and P18 (`contract.no-sanctioned-channel`, 5).
Together they are 12 route-resolve frictions over 11 runs, all about where an ownerless item lands. The
Carry-forward rubric could name a default class for an "owed item with no entry": a pinned CARRY on the
nearest-plausible entry, marked provisional and listed on the handoff. The halt would then be kept for items whose
placement changes scope. That matches what 2026-10-10T09:16:55Z-a already did as a workaround.

### P4 — phase/validate · `contract.mechanical-check` — 7 cases · weight 15
**Pattern:** 7 of 13 validate runs found a plan defect that no mechanical predicate reads. In two of them, only the
operator's review caught it, and both involved harm to the host: a nested compositor known to end the desktop
shell, and a kill sent to pid 4. The same two reviews appear as `overridden` facts 2026-10-09T15:43:22Z-a and
2026-10-10T02:42:58Z-a.
**Evidence:** ALL 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | the plan re-used a live rig whose own evidence records its nested compositor ending the desktop shell; carried as a risk with a stop rule; the operator's review caught it | dialogue_rounds 1 | viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup/inputs/andromeda-inputs.json |
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | scope and plan cited the previous chunk's snapshots as inputs#I9/I10; inputs.py verify read them against this chunk's manifest; no P5 check reads citations | iterations 1 | .andromeda/runs/2026-10-09T18-22-18-phase/inputs-2026-10-09-inner-cr-and-crlf-in-a-sent-text.json |
| 2026-10-09-epoch-3-cleanup-ii | a kill case aimed at a public caller whose first statement opens a lock file never reached the guarded function | extra_reads 1 · iterations 1 | viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/research.md |
| 2026-10-09-epoch-3-cleanup-ii | two graded verbs of a new script with no known-verdict control ahead of them | iterations 1 | — |
| 2026-10-10-windows-mutation-grade | a test case sends a kill at a process the test does not own (pid 4); the operator's review caught it | dialogue_rounds 1 | .andromeda/runs/2026-10-10T02-04-59-phase/relay-3.md |
| 2026-10-10-budget-governor | an obs criterion named G1 as proven by pre-push, which does not run it | extra_reads 2 · iterations 1 | — |
| 2026-10-11-the-board-viola-list | a hand-written modify-path re-check stripped hyphens with tr and read two existing paths as missing | extra_reads 1 | .andromeda/runs/2026-10-11T01-14-29-phase/ |

**Proposal:** the project absorbed the pid-4 case as a memory ("tests kill only their own processes"). The
pipeline-level version is a validate predicate. It would read every signal, kill or removal a plan step or test
names and require the target to be one the step itself starts or makes. It would also read whether a procedure
copied from an earlier chunk has evidence of a recorded harm. A second predicate would read `inputs#I{n}`
citations against the chunk's own manifest at P5 (the inputs.py verify call already does this; validate does not
ask for it).

### P5 — implement/code · `input.plan-step-ambiguous` — 12 cases · weight 14
**Pattern:** every code run in the epoch (12 of 12) met at least one plan step that was underdetermined, misnamed
a helper's behaviour, or conflicted with another step or with the gate order.
**Evidence:** ALL 12 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | step 14 ordered after step 13 while the gate block lists its probes before the entry step 12 fires up to | reformulations 1 | — |
| 2026-10-09-epoch-3-cleanup | step 16's private directory named nowhere in a chunk artifact; found by a file search under old scratch dirs | extra_reads 2 | — |
| 2026-10-09-epoch-3-cleanup-ii | step 12 names a block in three tests; a fourth holds it | extra_reads 1 | — |
| 2026-10-09-epoch-3-cleanup-ii | step 3's population rule (git ls-files) leaves two new untracked files out | extra_reads 1 | — |
| 2026-10-10-windows-mutation-grade | four points settled from neighbouring code (an unreadable volume's path, an empty field, job labels, a doubly-declared file) | extra_reads 5 | — |
| 2026-10-10-self-healing-state | step 3 says every new case fails before the product change; 10/43 unit and 37/43 integration read green on the stub tree | — | .andromeda/runs/2026-10-10T10-25-01-implement/gate-2026-10-10-self-healing-state.json |
| 2026-10-10-viola-revive | step 9 names wait_endpoint_gone for a killed wrapper; on Unix it waits for a socket file a killed wrapper leaves | extra_reads 1 | — |
| 2026-10-10-statusline-pass-through | step 7 orders obs init before the instance check, defeating step 10 case 5; step 3's literal None vs the replay assertion | extra_reads 3 · iterations 1 | viola-0.1.0/chunks/2026-10-10-statusline-pass-through/evidence/red-green.md |
| 2026-10-10-budget-governor | step 3 silent on what two bad keys give | extra_reads 1 | — |
| 2026-10-10-session-links | step 4's remove has its only caller in held step 12: dead code under the lint gate | extra_reads 2 | — |
| 2026-10-10-session-links | step 10 adds a gate evaluation to a slot that holds no gate handle | — | — |
| 2026-10-11-the-board-viola-list | step 3 fixes WHEEL at 22 while its own padding rule gives 23 | — | — |

**Proposal:** three of the shapes are mechanical. (a) A step that names an existing function and asserts its
behaviour (wait_endpoint_gone, DialogSlot::answer): research would read that function's body and quote it. (b) A
step order against the gate block's firing order: planlint would read the two orders together. (c) A step whose
only caller sits in a held step: planlint would flag the dead-code lint consequence. Chain X4 shows the plan
reaching fix-loop as `wrong` in 2 chunks. The project-side product-logic workarounds of Pass A (7 facts) are the
same events seen from the code.

### P6 — */* · `contract.jointly-contradictory-instructions` — 3 cases · weight 13
**Pattern:** three plans each held a pair of instructions that no order satisfies. All three pairs were authored
together, and no mechanical check tried them. One cost a soft-exit.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | gate entry 5 needs a test file edited that entry 7 (preservation guard) lists unchanged from base; exposed at the block's first run | dialogue_rounds 1 · soft_exit 1 | viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/inputs/andromeda-inputs.json |
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | step 11 edits a doc comment after the live start; step 12 asks the rebuilt binary's sha256 to equal the live start's | retries 4 · extra_reads 6 | viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/evidence/live-preconditions.md |
| 2026-10-10-budget-governor | a constraint rejects a hand-written budget.json in any test while step 13 lists cases on files no writer can produce | reformulations 1 | — |

**Proposal:** each pair has a readable shape. "Unchanged from base" × "a step edits that file" is a set
intersection. "Hash equal to an earlier build" × "a source edit after that build" is an order check. "No
hand-written X" × "a case on an invalid X" is a constraint-versus-case read. A planlint rule for each of the first
two is cheap. The third could be a P5 checklist line.

### P7 — wrap-session/reconcile · `input.report-insufficient` — 8 cases · weight 12
**Pattern:** in 8 of 10 reconciles, the detectors needed a fact the report did not carry. The orchestrator or a
detector then read the source. In one case, the owners for held parts were on no file in the tree.
**Evidence:** ALL 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | stale bare route numbers listed by site, no entry title; two detectors declined the citation item | extra_reads 1 | — |
| 2026-10-09-epoch-3-cleanup-ii | the report's search line said 1 hit each in two key files; a fixed-string read found 0 (the reading stands as 78 m) | extra_reads 3 | .andromeda/runs/2026-10-10T01-25-38-wrap/fanout-results.md |
| 2026-10-10-windows-mutation-grade | Changes held the sixteen survivors' grades only; the rest stood in the evidence folder and two proposals citing it fell to the opening rule | extra_reads 1 | .andromeda/runs/2026-10-10T08-56-51-wrap/fanout-results.md, rows T14, T15 and O1 |
| 2026-10-10-self-healing-state | Dependencies named viola-state's new dev-dependency, not viola-channel's standing one; four proposals named viola-state alone | extra_reads 1 | .andromeda/runs/2026-10-10T11-03-09-wrap/fanout-results.md |
| 2026-10-10-viola-revive | a help group unnamed, hint texts cited by location, the replay reach unsaid; three detectors flagged | extra_reads 3 · dialogue_rounds 1 | .andromeda/runs/2026-10-10T15-07-22-wrap/fanout-results.md |
| 2026-10-10-statusline-pass-through | the read function the new arm calls; revive launching through run's start | extra_reads 3 | .andromeda/runs/2026-10-10T19-55-42-wrap/fanout-results.md |
| 2026-10-10-session-links | three unstated facts (wait-feed generation, closed-gate stream, restore order vs strict-modes) | extra_reads 3 | .andromeda/runs/2026-10-11T00-33-08-wrap/fanout-results.md |
| 2026-10-11-the-board-viola-list | the review card's owners for held parts on no file in the tree; the operator supplied them from a transcript | dialogue_rounds 1 · extra_reads 4 | .andromeda/runs/2026-10-11T08-23-35-wrap/relay-3.md |

**Proposal:** see chain X1 (the same shape in 8 chunks, the producer `ok` each time). Most of the missing facts
are of three kinds: a new symbol's call and read sites, a fact about a crate beside the changed one, and a held
part's owner. A report-template family for each ("Reach: each changed symbol's product callers", "Neighbours: the
same dependency or rule in sibling crates", "Held: each held part and its owner, with the file that says so")
would put them where the detectors look. The board case also argues for snapping the founder-approved review card
into the chunk's inputs at P5, which is the step where it exists.

### P8 — */* · `contract.narrow-basis-claim` — 8 cases · weight 11
**Pattern:** a count or a site claim was written from a narrower source than the claim (a session memory, one file
of several, an unread listing, a registry's top-level names), then re-derived before anything acted on it.
**Evidence:** ALL 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| — (adaptation wrap) | a card said six mutants need a Windows test from the Linux table alone; win-outcomes.json graded all six caught | extra_reads 2 | .andromeda/runs/2026-10-09T14-44-30-wrap/adaptation-record.md |
| 2026-10-09-epoch-3-cleanup-ii | the sibling research's "every events copy reads through the shared reader" is six of seven | extra_reads 1 | viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/research.md |
| 2026-10-09-epoch-3-cleanup-ii | an evidence file count typed from the session (16); a listing read 15 | extra_reads 1 | — |
| 2026-10-10-viola-revive | key files mapped to masters by top-level registry names read all 30 as unowned; they stand under registries/contracts/<master>/ | retries 1 | — |
| 2026-10-10-viola-revive | a dialog card said a case does not assert the child ended, from a grep of the test file; the assertion is in Wrapper::kill | dialogue_rounds 1 · extra_reads 2 | viola-0.1.0/chunks/2026-10-10-viola-revive/inputs/I8-relay-3.md.txt |
| 2026-10-10-statusline-pass-through | implement's "the flag literal stands nowhere else" was a claim about the tree made from the diff; a tree grep found two | extra_reads 1 | viola-0.1.0/chunks/2026-10-10-statusline-pass-through/report.md |
| 2026-10-10-budget-governor | research stated 96 rows from an unread listing; the trail read 102 | extra_reads 1 | .andromeda/runs/2026-10-10T20-53-16-phase/tree-query-2026-10-10-budget-governor.json |
| 2026-10-10-budget-governor | a grep hit inside a helper written as the call site; the body gave :104 | extra_reads 2 | viola-0.1.0/chunks/2026-10-10-budget-governor/research.md, section Revision |

**Proposal:** all eight were caught, mostly by the author's own re-read. The registry case (key files under
`registries/contracts/<master>/`) is a layout fact a tool could print, for example a `registry where <key>` verb.
For the rest, the existing Tier-1 learning on counts covers them. The pipeline-level option is for tool trails to
print their own row counts on the verdict line, so a writer quotes the trail instead of a memory.

### P9 — implement/code · `tooling.hook-friction` — 5 cases · weight 10
**Pattern:** two PostToolUse hooks cost a re-issue each time. rustfmt reflowed a just-written function, so the next
anchored Edit missed (2 cases). The stamp-ahead hook read a test's synthetic same-day instants as record stamps (3
cases). The type has recurred in four epochs (Epoch 2: 1 · 2b: 4 · 3: 8 · 4: 5) and has never halted
(chronic-degrade).
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-10-self-healing-state | rustfmt hook reflowed a just-written function; the next Edit missed | retries 1 · extra_reads 1 | — |
| 2026-10-10-self-healing-state | stamp-ahead hook blocked a test fixture's literal timestamp | retries 1 | — |
| 2026-10-10-statusline-pass-through | rustfmt hook reflowed a just-written file; the next Edit missed | retries 1 · extra_reads 1 | — |
| 2026-10-10-budget-governor | stamp-ahead hook read rstest literals in src/human.rs as clock stamps | retries 1 | — |
| 2026-10-11-the-board-viola-list | stamp-ahead hook blocked an Edit of src/human.rs holding same-day synthetic instants | retries 1 | — |

**Proposal:** the stamp-ahead hook's purpose is record files (handoff, plan metadata, ledgers, evidence). A path
scope that leaves out `*.rs` (or `src/` and `tests/`) would remove the three false positives without weakening it
where it was built to act. The handoff's carried learning ("date them in the past") is the project absorbing this.
For rustfmt, the hook could print "reflowed N lines" in its output, so the next Edit reads before it anchors.

### P10 — wrap-session/reconcile · `ambiguity.playbook-no-match` — 3 cases · weight 10
**Pattern:** three detector proposals matched no playbook rule. One cost a halt.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup-ii | a proposal held a count until its rule was read; no rule; settled by the operator's recorded direction | extra_reads 3 | .andromeda/runs/2026-10-10T01-25-38-wrap/fanout-results.md |
| 2026-10-10-statusline-pass-through | two start steps with no span; no rule; escalated and settled as by design | dialogue_rounds 1 | .andromeda/runs/2026-10-10T19-55-42-wrap/relay-1.md |
| 2026-10-10-budget-governor | three proposals narrowing a NEVER ban (hand-written budget.json); no rule; the operator scoped the rule | dialogue_rounds 1 · halted 1 | .andromeda/runs/2026-10-10T22-36-31-wrap/fanout-results.md |

**Proposal:** each answer is now a precedent. A playbook entry per answer ("a proposal narrowing a NEVER rule's
scope", "an unspanned start step", "a count held until its rule is read") would let the next occurrence match
instead of halting. These answers came from the operator, so whether each becomes a playbook rule is the
founder's call.

### P11 — wrap-session/route-resolve · `tooling.long-line-edit` — 3 cases · weight 10
**Pattern:** appending to a multi-KB route line has no in-place verb. An anchored Edit's tail matched twice, and
six annotations took 12 splices.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | a 170-char tail of a 4.4 KB line matched twice; a second read found a unique 330-char anchor | extra_reads 1 | — |
| 2026-10-10-self-healing-state | an Edit anchored on a CARRY's last clause matched twice on one line | retries 1 · extra_reads 1 | — |
| 2026-10-11-the-board-viola-list | six CARRY appends, each an append plus a delete (12 splices), scripted to avoid retyping freight | iterations 6 | — |

**Proposal:** see L1, which carries the theme with its 10 Pass-A facts and the chronic count.

### P12 — implement/fix-loop · `contract.spec-reality-gap` — 2 cases · weight 6 (one halt)
**Pattern:** the plan or spec named something reality did not hold: a gate artifact key on a pure reader, and an
obs span list missing a new span.
**Evidence:** ALL 2 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | two gate entries that only read evidence files carry the artifact key; gate.py bounds freshness by the entry's own start, so they read STALE and red every firing, exit 0 | retries 1 · halted 1 · extra_reads 2 | .andromeda/runs/2026-10-09T15-46-10-implement/gate-2026-10-09-epoch-3-cleanup.json |
| 2026-10-10-budget-governor | the start opens run.budget_eval under run.start, which obs-plan §4 Scenario 1's span list does not hold | — | — |

**Proposal:** the first case is a tool contract question. If `gate.py` told a reader of an artifact apart from its
writer (an `artifact: read` vs `artifact: wrote` key), a plan entry that only reads evidence would not go red by
construction. The second case is ordinary wrap reconcile material.

### P13 — phase/plan · `input.research-thin` — 3 cases · weight 6
**Pattern:** at synthesis, research's Files-to-modify list lacked a build-system or dependency consequence: a
`[[test]]` entry in Cargo.toml, a crate without chrono, a harness profile.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | four facts research had not read (verify logs only with VIOLA_NAME; the harness's nextest profile; gate forms; a mutation run's baseline invocation) | extra_reads 4 · iterations 1 | viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup/plan.md |
| 2026-10-10-statusline-pass-through | research did not say chrono is absent from the agent crate and viola-core; two files joined the modify list | iterations 1 · extra_reads 1 | — |
| 2026-10-10-budget-governor | Files to modify omitted Cargo.toml although a new root test binary needs [[test]] with required-features | extra_reads 1 · iterations 1 | — |

**Proposal:** chain X3 (research → plan) shows a formally `ok` research with `unresolved-questions` in all three.
A research-letter closure line could close the gap: "for each new file, test binary or dependency use, the
manifest lines it needs". It would sit beside the existing name sweep.

### P14 — wrap-session/curation · `ambiguity.filter-borderline` — 4 cases · weight 6
**Pattern:** Filter 4's two signals, verified-by-measurement and no-other-home, had an open reach four times. The
open questions were whether a deviation or a falsified dialog card counts as measurement, and whether a single
master instance or a docs leaf counts as a home.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup-ii | verified-by-measurement earned on a deviation and a record-building hazard; reach open | — | .andromeda/runs/2026-10-10T01-25-38-wrap/curation.md |
| 2026-10-10-viola-revive | does falsifying this wrap's own dialog card count as falsifying a spec or plan claim; scored without it | iterations 1 | .andromeda/runs/2026-10-10T15-07-22-wrap/curation.md |
| 2026-10-10-statusline-pass-through | does a master sentence carrying one instance of a finding count as homed; read as homed, rejected at 0.6 | iterations 1 | .andromeda/runs/2026-10-10T19-55-42-wrap/curation.md |
| 2026-10-10-session-links | is a docs leaf (commands.md) a home; not among the filter's listed homes; applied | extra_reads 1 | .andromeda/runs/2026-10-11T00-33-08-wrap/curation.md |

**Proposal:** the curation guide could give each signal one worked edge. For verified-by-measurement: "a deviation
measured in this chunk counts; a falsified card of the same wrap does or does not". For no-other-home: "a docs
leaf is or is not a home; one instance in a master is or is not". The four records already show which way each
call went. Note that curation's own `exact-hit` signal summed 30 over 10 runs, so Filter 4's cutoff is busy.

### P15 — wrap-session/reconcile · `contract.cascade-miss` — 5 cases · weight 6
**Pattern:** standing restatements that no detector proposed stood after the apply. In four of five cases, the
cascade sweep found them in the same pass. In the fifth, a leaf stale for two wraps was found by chance on a line
re-derived for another claim.
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup | security-plan Error Handling named control-character as the one paste refusal; the sweep listed it | extra_reads 1 | — |
| 2026-10-10-viola-revive | two design-system restatements of run as the one passthrough; the sweep printed both | extra_reads 1 | .andromeda/runs/2026-10-10T15-07-22-wrap/cascade-dispositions.md |
| 2026-10-10-statusline-pass-through | two master restatements (a count's third site; hook-stdout) by the sweep, and a rules leaf by the step-3 read | extra_reads 2 · iterations 1 | .andromeda/runs/2026-10-10T19-55-42-wrap/cascade-dispositions.md |
| 2026-10-10-budget-governor | four standing master sentences on a retired claim, found only by the step-2 sweep | extra_reads 4 | .andromeda/runs/2026-10-10T22-36-31-wrap/cascade-dispositions.md |
| 2026-10-10-session-links | services/viola-state.md said the replay has no reader, stale two wraps since revive | — | .andromeda/runs/2026-10-11T00-33-08-wrap/cascade-dispositions.md |

**Proposal:** the sweep is working as designed, and the four catches are a positive. The gap is the leaves, which
the session-links case shows the sweep did not reach. A leaf sweep keyed on "no reader / no caller / not yet"
phrasings would cover the one leaf miss. Giving the detectors the sweep's hits as input would turn the late finds
into proposals.

### P16 — wrap-session/route-resolve · `contract.carry-no-owner` — 4 cases · weight 6
**Pattern:** a CARRY or follow-up had no plain owner entry.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup-ii | a stale security-plan sentence on CI mutation jobs pinned on the next entry touching the workflows | extra_reads 1 | .andromeda/runs/2026-10-10T01-25-38-wrap/fanout-results.md |
| 2026-10-10-windows-mutation-grade | four mutants no host compiles; whether "Windows-only live measurements" is a plausible owner was a judgement | extra_reads 1 | — |
| 2026-10-10-budget-governor | one of four CARRYs left with no next owner by the plan and the lifting word; the owner came with the resume directions | — | .andromeda/runs/2026-10-10T22-36-31-wrap/route-record.md |
| 2026-10-11-the-board-viola-list | the operator minted three entries and placed one; two slots took a trajectory halt | dialogue_rounds 1 | .andromeda/runs/2026-10-11T08-23-35-wrap/relay-4.md |

**Proposal:** see P3.

### P17 — */* · `contract.structural-blind-spot` — 2 cases · weight 6 (one soft-exit)
**Pattern:** a documented sweep or gate could not reach the defect by construction. A name sweep misses a test that
pins behaviour through data. No local gate lints the Windows target.
**Evidence:** ALL 2 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | the companion sweep greps the changed identifier; a test pinning the behaviour through the wrapper names none; a data-shape sweep found it | extra_reads 3 · soft_exit 1 | viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/research.md |
| 2026-10-10-statusline-pass-through | a cfg(unix)-only helper unused on Windows failed the windows-2025 lint; no gate lints the Windows target; a host check-only clippy for x86_64-pc-windows-msvc reproduced it | iterations 1 | viola-0.1.0/chunks/2026-10-10-statusline-pass-through/evidence/operator-pass.md |

**Proposal:** two additions. (a) Research's companion sweep could add a data-shape arm: a literal of the changed
input class, beside the identifier arm. (b) Where CI lints a target the host does not build, the gate block could
carry a check-only cross-target clippy entry. The 2026-10-10 fact shows it works on this host. The 2026-10-10
budget-governor fix-loop fact (2026-10-10T22:01:41Z-a) ran it by hand ahead of the block.

### P18 — wrap-session/route-resolve · `contract.no-sanctioned-channel` — 5 cases · weight 5
**Pattern:** five things had no carrier in the route or input grammar: a citation sweep on the adaptation path, an
input snap with no chunk, an ownerless audit fact, a ratification owed on a master row, and an operator-judgment
fact. Three of the five came from one adaptation wrap.
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| — (adaptation wrap) | a 0-pending adaptation that inserts above a cited line stales path:N cites; Setup step 6 names no sweep; cites.py fired with --marker no-marker | extra_reads 3 | .andromeda/runs/2026-10-09T14-44-30-wrap/citation-dispositions.md |
| — (adaptation wrap) | the relay driving the adaptation has no snap channel (inputs.py snap needs --chunk-dir); recorded by size and sha256 | — | .andromeda/runs/2026-10-09T14-44-30-wrap/adaptation-record.md |
| — (adaptation wrap) | an audit fact on a retired CARRY has no route owner and no disposition; to the record and the handoff | — | same |
| 2026-10-10-viola-revive | a master row marked not ratified; the only cross-session carrier is the handoff; no annotation class holds an owed ratification | — | .andromeda/runs/2026-10-10T15-07-22-wrap/route-record.md |
| 2026-10-10-session-links | a detector's fact for the operator's judgment has no owner entry; written into the master and listed on the handoff | — | .andromeda/runs/2026-10-11T00-33-08-wrap/route-record.md |

**Proposal:** three small grammar additions would give each item a home. (a) A route annotation class `OWED-WORD:`
for a ratification or judgment owed on a landed shape. The handoff's long "For the founder" list is this channel
today. (b) A chunkless `inputs.py snap --run-dir`. (c) The adaptation path's Setup naming the citation sweep.

### P19 — */* · `contract.premise-falsified` — 3 cases · weight 4
**Pattern:** a premise in an authored artifact (a CARRY, a plan note, a plan step) was false on measurement.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-epoch-3-cleanup-ii | the CARRY allows an argued survivor; the test plan's gate forbids "equivalent", so an argued survivor stays missed | extra_reads 1 · deferred 1 | viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/research.md |
| 2026-10-10-self-healing-state | entries 10 and 11 "read the kept test homes"; no file under target/e2e-home holds the chaos case's line (0) | extra_reads 2 | viola-0.1.0/chunks/2026-10-10-self-healing-state/evidence/red-green.md |
| 2026-10-10-budget-governor | step 14's "the child ends at once on a null stdin"; it ran on, the witness read TIMEOUT | retries 1 | chunks/2026-10-10-budget-governor/evidence/red-green.md (a) |

**Proposal:** two of three are about what a test home holds after the test ends. That links to the vacuous-check
record in the appendix (2026-10-10-session-links smoke) and the removed-cause 2026-10-10T10:46:57Z-a. A test-plan
line stating "a root test's home is removed at its end; a schema check reads only kept homes" would close the
premise class at its source.

### P20 — */* · `recall.change-reconstruction` — 3 cases · weight 3
**Pattern:** the report rebuilt its Changes families from the diff and evidence, because the implementing
conversation was compacted or cleared.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-09-inner-cr-and-crlf-in-a-sent-text | compacted between implement and wrap; Files, Symbols, Counts rebuilt | extra_reads 6 | — |
| 2026-10-10-windows-mutation-grade | five families rebuilt; the coding session was cleared before the stop rule was released, no implement report of steps 1–10 on disk | extra_reads 9 | — |
| 2026-10-10-budget-governor | six families rebuilt from a 3191-line diff, plan, research and scope | extra_reads 14 | .andromeda/runs/2026-10-10T22-36-31-wrap/new-text.md |

**Proposal:** see L7 and chain X2.

## Cross-step chains (starting heuristics)

### X1 — wrap-session/report →report→ wrap-session/reconcile — 8 chunks
Consumers `thin` or `wrong` on the report in 8 of 10 chunks (`epoch-3-cleanup`, `epoch-3-cleanup-ii`,
`windows-mutation-grade`, `self-healing-state`, `viola-revive`, `statusline-pass-through`, `session-links`,
`the-board-viola-list`). The producer was `ok` each time (one `halted-resolved`), with signals `new-text-generated`
and in two cases `reconstructed`. The downstream frictions are P7's 8 `input.report-insufficient` records, and
the report was also extended after the fan-out read it in 4 chunks (q-level.json theme). **Hypothesis:** the
report's families describe what changed but not what a detector needs to re-word a master: reach, neighbours,
owners. The producer cannot see the gap because no producer-side check reads the report as a detector would.
**Direction:** P7's three families, or a report self-check that runs one detector prompt against the draft.

### X2 — implement/smoke →conversation→ wrap-session/curation — 7 chunks (and → wrap-session/report, 3 chunks)
Curation consumed the implementing conversation as `thin` in 7 of 10 chunks, and the report did in 3. The producer
signal is `recorded-not-rerun`, and the consumer notes say "conversation gone; read from resume-point.md" each
time. Together with L7's 8 facts and P20's 3 records, this is the epoch's most cross-cutting shape. **Hypothesis:**
curation and the report are designed against a conversation that a real chunk no longer keeps. The project
invented `resume-point.md` as the carrier, and it is now the actual input. **Direction:** see L7.

### X3 — phase/research →research→ phase/plan — 3 chunks
`epoch-3-cleanup`, `statusline-pass-through`, `budget-governor`: research `ok` with `unresolved-questions`; plan
read it `thin` (P13). Direction: P13.

### X4 — phase/plan →plan→ implement/fix-loop — 2 chunks
`epoch-3-cleanup` (entries 22 and 23 red by their own artifact key) and `inner-cr-and-crlf-in-a-sent-text`
(entries 5 and 7; steps 11 and 12). The plan was `ok` with `designed-dialogue` and `authority-resolved` signals,
and validate signalled `baseline-vacuity-caught` in the first. Both carry into P6 and P12. Direction: P6's planlint
pairs.

### X5 — implement/fix-loop and smoke →implement-outcome→ wrap-session/report — 3 chunks
`statusline-pass-through` (one acceptance clause contradicted by a pre-existing site), `budget-governor` (the
first run's report not in the window), `the-board-viola-list` (implement's green superseded; its consumer lists
`operator-pass` as `fixed`: CI red on windows-2025, fixed by acad7af). In the board case the operator pass is what
superseded the producer's signal, so this is not a producer defect. The other two fold into P8 and L7. No new
direction.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid + chronic-degrade — 10 facts
**Facts:** multi-KB single lines in masters and the route are read and edited around the documented readers. Ten
facts in the epoch: 2026-10-09-epoch-3-cleanup reconcile (environment, workaround: a 2600-char bounded-repetition
grep killed after 120 s; offset script) · same, route-resolve (process: splice append with a script-built payload,
not the anchored Edit) · 2026-10-09-epoch-3-cleanup-ii reconcile (scratch windows; cascade.py window takes one
offset per call) · 2026-10-10-self-healing-state take-up (environment: scratch window script; host grep refuses
bounded repetition) · 2026-10-10-budget-governor report (scratch hit counter, window printer, heading finder) ·
same, reconcile (cascade.py window wraps its output and cannot feed an Edit's old_string) ·
2026-10-10-session-links take-up (character windows instead of Read) · same, reconcile (2 000–12 000-char lines) ·
2026-10-11-the-board-viola-list take-up (line past the Read per-line bound) · same, route-resolve (append-then-delete;
no in-place line replace). Typed correlates: P11's 3 `tooling.long-line-edit`, the untyped 2026-10-09T17:32:09Z-e,
recall 2026-10-11T08:47:43Z-c and 2026-10-11T00:57:13Z-g. Recurring: Epoch 3 facts 2026-10-06T22:04:50Z-a,
2026-10-07T06:01:45Z-a, 2026-10-07T13:13:11Z-a, 2026-10-08T09:34:08Z-a. `tooling.long-line-edit` by epoch: 2b 3 ·
3 1 · 4 3.
**Level hypothesis:** the cause appears to live in the pipeline: the document grammar (one entry or paragraph per
multi-KB line) and a reader/writer set with no unwrapped exact-bytes window and no in-place line rewrite. The fixes
so far live in each session as a fresh scratch script.
**Proposal:** two tool verbs. (a) `cascade.py window --raw` (or a `splice.py show`): exact unwrapped bytes, every
hit of a pattern in one call, fit to paste as an Edit `old_string`. (b) `splice.py replace-line` / `annotate`:
rewrite one line from a payload file, asserting every other line byte-identical. That is what the scripted
append-then-delete already does. The deeper alternative, splitting entries across lines, is a grammar change and
is the founder's to weigh.

### L2 — band-aid (rule stricter than its guard) — 12 facts
**Facts:** a bare `cd` outside a subshell, against host-linux.md Paths. Workaround: 2026-10-08T08:44:32Z-a
(orientation; the guard blocked one) · 2026-10-09T16:07:38Z-a (code) · 2026-10-10T11:05:54Z-a (report; the cwd
moved into .andromeda) · 2026-10-10T21:13:45Z-a (research; guard refused) · 2026-10-10T21:56:00Z-a (code; most
calls, the guard did not refuse). Removed-cause: 2026-10-09T15:17:55Z-a (distill) and 2026-10-09T21:37:58Z-a
(distill), each a cd that moved the cwd and a cd back. Unresolved: 2026-10-10T09:30:33Z-a, 2026-10-10T15:15:05Z-a,
2026-10-10T16:05:18Z-a, 2026-10-10T17:33:27Z-a, 2026-10-10T20:52:31Z-a (five orientation or validate calls with a
cd to the root: "nothing refused them"). Related: 2026-10-09T21:14:19Z-a and 2026-10-09T22:01:10Z-a passed an
absolute `--root` for the same reason. Typed correlates: recall 2026-10-10T11:21:03Z-b, 15:46:28Z-b,
20:32:58Z-c, 23:13:01Z-e, 2026-10-11T00:57:13Z-e. Recurring: 2026-10-04T05:06:21Z-a, 2026-10-08T05:12:25Z-a.
**Level hypothesis:** the guard rejects only a `cd` that moves the cwd away from the root. A `cd` to the root passes
and harms nothing, yet the rule counts it as a breach, and that is five of the twelve facts. The two removed-cause
facts are the case the guard was built for. The rule and the guard disagree, and the cost lands in the ledger as
recurrence.
**Proposal:** one of two directions. (a) Align the rule to the guard: "a cd into the repository root is harmless;
any other cd only inside a subshell". (b) Have the pipeline tools default `--root` to `git rev-parse
--show-toplevel` and the letters drop the leading `cd`.

### L3 — band-aid — 7 facts
**Facts:** an inline python heredoc used for a source or document edit, against host-linux.md Transports:
2026-10-09T16:07:38Z-a (three lines of viola-core lib.rs) · 2026-10-10T03:01:16Z-a (a count-asserted replace
after the format hook rewrote the file; a six-item matrix change) · 2026-10-10T14:38:15Z-a (a fake-agent
multi-edit) · 2026-10-10T21:56:00Z-a (two source edits) · 2026-10-10T23:13:01Z-a (a docs leaf) ·
2026-10-11T00:16:16Z-a (viola-core step 1) · 2026-10-11T08:47:43Z-a (multi-site master edits, several anchored
replacements per file). All landed. Typed correlates: recall 2026-10-10T23:13:01Z-e, 2026-10-11T00:57:13Z-c,
2026-10-11T08:47:43Z-d. Epoch 3 holds 12 heredoc facts (keyword count).
**Level hypothesis:** the Edit tool takes one site per call, and the rustfmt hook moves the anchor after each
write. A scripted multi-site replace is the cheaper path, so it keeps being taken. The rule forbids the inline form
and offers no multi-site channel.
**Proposal:** a sanctioned multi-site replace, for example a `splice.py replace --file F --pairs pairs.json`
asserting each anchor matches exactly once. With it, the rule's positive form ("multi-site: the replace verb")
would exist beside its ban.

### L4 — band-aid — 6 facts
**Facts:** a verdict tool's listing redirected or filtered, against the run-bare rule: 2026-10-09T21:27:01Z-a
(route.py pins) · 2026-10-09T21:37:58Z-a (seven cites calls) · 2026-10-10T02:17:42Z-a (cites through tail -4) ·
2026-10-10T09:13:42Z-a (cascade sweep to a scratch file) · 2026-10-10T20:31:15Z-a (sweep, 103 lines 12 KB) ·
2026-10-10T20:56:23Z-a (pins through a filter keeping UNPARSED / INDETERMINATE). Related: 2026-10-10T23:50:54Z-a
(the query tool's indented JSON read through a script). Typed correlate: recall 2026-10-11T00:57:13Z-f.
**Level hypothesis:** the run-bare rule guards against a pipe hiding the exit code and a filter hiding a verdict
line. The listings are long enough (12 KB) that every operator filters anyway. 2026-10-10T20:56:23Z-a built the
safe filter by hand: keep the head's rows plus every UNPARSED / INDETERMINATE line.
**Proposal:** make that filter a tool flag, for example `--rows <selector>` on the listing verbs, which always keeps
the verdict, UNPARSED and INDETERMINATE lines and the exit code. Filtering would then be sanctioned and
verdict-safe.

### L5 — band-aid — 5 facts
**Facts:** distill's history prompts deviated from the verbatim template in 5 of 10 distill runs:
2026-10-09T15:17:55Z-a (a line added naming the root) · 2026-10-10T02:17:42Z-a (same) · 2026-10-10T16:18:03Z-a
(paths substituted absolute: "the agents' Read tool takes absolute paths") · 2026-10-10T21:05:10Z-a (same) ·
2026-10-10T23:43:42Z-a (same). Recurring: 2026-10-04T21:45:48Z-a.
**Level hypothesis:** the template (`fan-out.md`) gives the sidecar and index paths project-relative, while a fresh
agent's Read takes absolute paths. Every run patches the template the same way.
**Proposal:** the template renders the two paths absolute (`{root}/…`). The deviation would then disappear without
a rule change.

### L6 — band-aid — 5 facts
**Facts:** the detectors' hand-backs were taken from transcript files by script, and the scripted entity decode was
not run: 2026-10-09T21:04:49Z-a (decode not run; returns read directly) · 2026-10-10T01:43:56Z-a (same) ·
2026-10-10T11:20:23Z-a (lists from the transcript by a script printing sizes) · 2026-10-10T15:44:22Z-a (same) ·
2026-10-10T20:31:15Z-a (66 proposals assembled by script: "a retyped list is a transcription").
**Level hypothesis:** the letter prescribes retyping plus a decode step. The operators found transcript extraction
more faithful and skip the decode.
**Proposal:** a tool verb that extracts each detector's hand-back from its transcript, decodes entities and prints
counts. This is the workaround made the sanctioned path.

### L7 — band-aid + override — 8 facts
**Facts:** the implementing conversation does not reach the wrap. Overridden (the one-window wrap):
2026-10-10T01:28:39Z-a (P1 only; the Setup refresh deferred to the resume) · 2026-10-10T15:12:30Z-a (context past
60 %) · 2026-10-10T19:59:11Z-a · 2026-10-10T22:45:21Z-a. Workaround: 2026-10-10T08:54:01Z-a (implement re-entered
at step 11 in a second run dir, firing one checkpoint of three) · 2026-10-10T15:46:28Z-a, 2026-10-10T20:32:58Z-a,
2026-10-10T23:14:59Z-a (curation read candidates from resume-point.md). Typed: P20's 3
`recall.change-reconstruction`. Chain X2: 7 chunks. Recurring: 2026-10-04T16:55:25Z-a (P1 only at 86.7 %),
2026-10-05T10:40:17Z-a (61.5 %).
**Level hypothesis:** the wrap and curation are designed as session-local over one continuous conversation. Real
chunks cross a clear or compaction between implement and the wrap, and the wrap itself splits after Phase 1. The
override recurs on the same rule (the one-window wrap), so the rule's assumption, not the operator, is what
diverges.
**Proposal:** promote what the project already does. Make a per-chunk candidates/changes log (today's
`resume-point.md`) a first-class artifact that implement appends at each checkpoint: its deviations, corrections,
curation candidates and Changes facts. Name it as the report's and curation's primary source, with the
conversation secondary. Also consider a sanctioned two-window wrap (P1 | P2–P7) as the default shape rather than an
override.

### L8 — override / band-aid — 3 facts
**Facts:** Phase 5's route placement was asked at the Phase 2 halt: 2026-10-09T21:04:49Z-a (overridden: two P5
questions asked while detectors ran, because the answer named an entry two master sentences cite by title) ·
2026-10-10T11:20:23Z-a (the placement asked with the P2 escalation, so one halt) · 2026-10-10T15:44:22Z-a ("master
text names route entries by title, so the Phase 5 placements were asked at the Phase 2 halt").
**Level hypothesis:** the Tier-1 learning "a master names a working-route entry by its title" makes a master
amendment depend on a route placement. The wrap's order (masters at P2, route at P5) puts the dependency backwards,
and operators reorder it each time.
**Proposal:** the wrap letter could sanction asking the placement at the P2 halt whenever an amendment must name
an entry, with the route edit still landing at P5. That is the shape all three facts took.

### L9 — removed-cause observation (recurring) — 2 facts
**Facts:** absolute host paths in run-dir files tripped the hygiene read: 2026-10-10T01:11:50Z-a (workaround:
mutation-run stderr carries the repository absolute path; moved to scratch) · 2026-10-11T08:55:19Z-a
(removed-cause: a relay copy rewritten repo-relative). Related: 2026-10-09T22:16:23Z-a (deferred, same files),
untyped 2026-10-10T02:42:58Z-d (`gate.py respell` left a bare-root hit for a hand edit). Recurring:
2026-10-05T15:30:13Z-a.
**Observation:** the same cause is removed by hand at each occurrence. Writers that land third-party output
(cargo build lines, relays) in a run dir produce it by construction. A direction: such captures go to scratch by
default, and `gate.py respell` handles a bare-root hit.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — new-session/orientation — 4 cases → proposed type `ambiguity.ladder-uncovered-state` (definition widened)
**Cluster:** 2026-10-08T08:31:34Z-b ("a pending chunk's live state with a wall-clock bound … is carried by no
orientation input") · 2026-10-08T08:41:34Z-b ("the pending rung has no arm for a pending chunk whose implement run
is live in another session") · 2026-10-08T08:44:32Z-b ("no rung covers a pending chunk whose run is in flight
elsewhere … the suggestion became: start neither skill here") · 2026-10-09T16:34:31Z-b ("an implement stopped at
P2 with gate entries red by the plan's own artifact key, whose resolution is the phase letter's Revision section …
not from the ladder"). The first three are one live run seen from three session starts on one morning. Beside
them: the one typed `ambiguity.ladder-uncovered-state` (2026-10-10T07:59:31Z-b, rung 3's discriminator read true on a
pass that ended by design) and two `overridden` facts where the invocation named the act ahead of the ladder
(2026-10-10T07:59:31Z-a, 2026-10-10T22:05:16Z-a). Recurring from Epoch 3: 2026-10-07T13:34:35Z-b (no rung for an
open red on a complete record), 2026-10-03T21:39:51Z-b (host moved under the route head).
→ **draft criteria line:** "`ambiguity.ladder-uncovered-state` — the cursor's state is one no rung names, or the
rung's named skill would refuse it. Examples: a pending chunk whose run is live in another session; an implement
stopped on a plan contradiction that only a phase revision resolves; an open red on a complete record. Record the
state class and what was suggested instead."
**Note:** the type exists, but its current definition ("would refuse the cursor") is narrower than these events,
which is why they went untyped. Two ladder arms in `andromeda-new-session` would also remove the cause: "pending,
run in flight elsewhere → start nothing, name the run" and "pending, implement halted on a plan contradiction →
`/andromeda-phase` revision".

### U2 — phase/take-up · phase/validate (+ implement/fix-loop) — 4 cases → proposed type `tooling.tool-output-incomplete`
**Cluster:** 2026-10-09T15:07:26Z-b (`ci.py conclusion` clipped the run id in stdout and trail, and names only the
first failed check; one `gh api …/check-runs` read supplied them) · 2026-10-10T02:42:58Z-d (`gate.py hygiene`
refused three run-dir files; `gate.py respell` fixed two and left a bare-root hit for a hand edit) ·
2026-10-11T00:02:22Z-b (`gate.py show` read two of three baseline entries as not run, because it reads the last run
record; a second call named the firing) · 2026-10-10T22:01:41Z-d (the project's pre-push document carries no
coverage figure; read from llvm-cov-summary.json after three greps).
→ **draft criteria line:** "`tooling.tool-output-incomplete` — a pipeline or harness tool exited clean but its
printed result or trail lacked a field the step needed (an id clipped, only the first of several failures named, a
per-call view read as the whole), so a second tool or a raw read supplied it. Name the tool, the missing field and
the read that supplied it."

### U3 — any step — 2 in-epoch, recurring from Epoch 3 → proposed type `recall.clock-estimate`
**Cluster:** 2026-10-10T23:13:01Z-f (an input snap's `--origin` carried a clock time typed without a read) ·
2026-10-11T07:58:07Z-c ("four times written ahead of or without the clock: three journal lines and one evidence
time window"). The same event sits typed elsewhere in the epoch: recall 2026-10-10T01:43:56Z-f and
2026-10-11T00:57:13Z-d, and in-pass-correction 2026-10-10T23:13:01Z-d. That makes 5 occurrences in Epoch 4.
Recurring untyped in Epoch 3: 2026-10-05T11:31:17Z-c, 2026-10-05T11:42:17Z-b, 2026-10-07T07:33:13Z-b,
2026-10-07T08:53:01Z-b, 2026-10-07T14:34:57Z-d, 2026-10-07T14:43:23Z-b, 2026-10-08T06:23:46Z-c.
→ **draft criteria line:** "`recall.clock-estimate` — a time was typed into a record, label or document from an
estimate instead of a clock read made in the same call. Record the field, how far it ran ahead and what caught it
(the stamp-ahead hook, evolve.py's REFUSED ts, a re-read)."
**Note:** the handoff carries this as `recurrence-despite-learning` ("four times in implement"). A tool-side
direction would remove the hand-typed slot itself: `inputs.py snap` already stamps `read_at`, and `--origin` could
drop its time.

### U4 — new-session/orientation · implement/code — 2 in-epoch, recurring from Epoch 3 → proposed type `tooling.permission-compound-denial`
**Cluster:** 2026-10-08T08:44:32Z-c (Setup step 1's chained git call failed whole on its ahead-count member,
"refused with 'not part of this test'"; re-fired once, then split into five calls) · 2026-10-09T16:07:38Z-c ("a
compound call holding one rm -r … beside three read-only commands was denied whole by the permission layer, with
no word on which part"). Recurring from Epoch 3: 2026-10-07T14:05:59Z-b (a compound with a removal beside five
reads, denied whole), 2026-10-07T10:30:18Z-g (an rm -r of an own-made dir cost a question).
→ **draft criteria line:** "`tooling.permission-compound-denial` — a compound Bash call was denied whole by the
permission layer or classifier for one member, with no word on which. Record the member that drew it (when found),
the split it took and whether the rule file already names the shape."
**Note:** host-linux.md Compound commands already names the rm-in-a-compound shape. The new-session letter's own
Setup step 1 is a compound, and one of the two in-epoch cases was that compound.

## Below threshold — no action
Typed groups:
- implement/fix-loop · `contract.test-expectation` — 2 (inner-cr: an existing test asserting a lone inner CR; statusline: every-OS cases booting an unstamped Windows home).
- phase/validate · `ambiguity.review-cycles` — 2 (windows-mutation-grade: a dispatch outside a founder ruling put on the card as a confirm item, not a P4 fork; viola-revive: a first yes carrying a stop rule).
- wrap-session/curation · `ambiguity.tier-routing` — 2 (a route-citing convention and an operator-stated route rule with no clean tier home).
- new-session/orientation · `input.handoff-git-mismatch` — 2 (a phase run after the wrap promoted the entry; diagnosis run dirs written after the wrap).
- */* · `contract.skill-reference-drift` — 2 (session-state-contract's expected-transient list lacks the wrap trail modified after its commit, and an untracked evolve-gates.py).
- */* · `contract.token-proxy-check` — 2 (inputs.py verify false positive on a quoted prior-chunk id; a one-line grep missing two split call sites).
- implement/fix-loop · `retry.fix-iterations` — 1 · `tooling.result-not-run-stable` — 1 (a release build's sha256 not restored with its comment bytes) · `tooling.environmental` — 1 (the dangling `target/e2e-home` link after a reboot) · `contract.vacuous-check-found` — 1 (4 of 238 mutants caught by the kill line alone).
- implement/smoke · `contract.vacuous-check-found` — 1 (the schema-check entry cannot read a removed test home).
- wrap-session/reconcile · `contract.false-positive-proposal` — 1 (a relayed founder ruling written into master text).
- wrap-session/gates · `tooling.commit-mechanics` — 1 (route.py flip has no desc rewrite of a complete record).
- wrap-session/report · `input.implement-outcome-unsettled` — 1 (the operator pass ended fixed).
- phase/research · `contract.instrument-validity` — 1 (cargo mutants --list --file with no --package printed 0) · `tooling.graph-symbol-missing` — 1 (cfg(windows) bodies absent from the Linux-built rust plane).
- phase/distill · `contract.binding-contradiction` — 1 · phase/take-up · `input.carry-context-gap` — 1.
- new-session/orientation · `ambiguity.ladder-uncovered-state` — 1 (folded into U1).
- */* · `contract.grammar-irregularity` — 1, site `scope.md:26` (`UNPARSED: inputs#I3 — no entry`), a one-off: the data.
- */* · `tooling.host-shell` — 1 (TMPDIR unset; a redirect landed at the filesystem root) · `tooling.output-cap-overflow` — 1 (a 65.7 KB documentation fetch).

Untyped clusters (emerging, watch next epoch):
- `gh run list --commit` with an abbreviated sha prints an empty list at exit 0 — 2 untyped (2026-10-09T19:47:45Z-b, 2026-10-10T07:59:31Z-c) plus 1 typed recall (2026-10-10T09:15:51Z-b); ci.md has carried the rule since 2026-10-09, path-scoped (P1).
- a bounded-repetition pattern over a multi-KB line — 1 untyped (2026-10-09T17:32:09Z-e, GNU grep past 120 s); Epoch 3 had 1 (ugrep complexity refusal); folded into L1.
- singletons: a scope bullet recording an operator answer before the card was asked · a 16 MiB format width panicking at run time · cargo-mutants keeping one run's per-mutant logs · two detectors returning `proposals: []` with a real stale clause noted outside them · a fork dialog returning no answer within 58 minutes · `JsonFields` recording an f64 as Debug text · `git rev-parse --short` given two revisions · a name sweep walking the fuzz build output · a handoff operator-desk entry naming a directory that does not exist · a run-dir-relative `../../../../` redirect landing above the repository root.

Problem-fact themes under threshold:
- waiting on a backgrounded call by polling its output file — 2 (2026-10-10T10:46:57Z-a, 2026-10-10T22:34:51Z-a).
- implement's no-commit rule overridden on the operator's word — 2 (2026-10-09T19:47:45Z-a, 2026-10-10T22:34:51Z-a).
- a P4 fork not asked because the invocation's word had chosen — 2 (2026-10-10T22:12:51Z-a, 2026-10-10T23:56:04Z-a).
- route-resolve's recommended placement not taken — 2 (2026-10-10T01:49:01Z-a, 2026-10-10T23:17:00Z-a).
- a playbook read ahead of its checkpoint, against the reading order — 2 (2026-10-10T17:33:27Z-a, 2026-10-11T01:03:03Z-a).
- deferred-forever: 0 hits. All 9 deferred facts are closed in a later record or routed by their own note (listed in q-level.json).
