# Fan-out results — 2026-10-09-epoch-3-cleanup-ii

Seven doc-agents, one batch, each sent the letter's prompt verbatim with repo-relative paths; the four keyed masters
(architecture, test-plan, obs-plan, a11y-plan) each with its contracts render in this run dir. Detector counts per
prompt: architecture 2, security-plan 3, design-system 1, layout-templates 1, test-plan 3, obs-plan 3, a11y-plan 2;
sum 15, the drift-base's 15 `doc:` names. Every return read as YAML. No HTML entity was seen in any of them, read
directly from the hand-backs; no scripted decode was run. Four empty returns carried comment lines that stripping
removed; each has its raw twin (`.raw-fanout-{doc}.md`).

Totals: 5 proposals (architecture 1, test-plan 4). Applied 4 (test-plan). Rejected 1 for a source the report does not
carry (architecture), its amendment raised by the orchestrator instead and applied. 1 carried `escalate` (test-plan
T4, the detector's own word), resolved on the operator's recorded direction with no halt. Orchestrator-raised: 1
(architecture, check 5).

## Verdict lines
- architecture — 1 proposal (D-arch-resources 1; D-arch-decisions none).
- test-plan — 4 proposals (D-tests-framework 2, D-tests-coverage 2; D-tests-obs-harness none).
- security-plan — `proposals: []`. Stripped: its three no-drift bases and one note outside its detectors (the
  Threat Model Summary's CI jobs sentence lists a `mutation` job while §Dependency Security says `ci.yml` runs
  none). See "Outside the report" below.
- obs-plan — `proposals: []`. Stripped: its three no-drift bases and two notes outside its detectors (§3 intro
  calls `src/human.rs` `refuse` "called only by `run`"; the session homes a `viola-e2e` mutation run leaves contradict
  no obs-plan claim). See "Outside the report" below.
- design-system — `proposals: []`. Stripped: its no-drift basis. It read `design-system.md` to line 476 of 922; the
  report adds no UI element, so the detector had nothing to sweep.
- layout-templates — `proposals: []`. Stripped: its no-drift basis (`viola verify`'s output entries stand).
- a11y-plan — `proposals: []`. Nothing stripped.

## architecture (1)
| # | detector | section | change, in short | disposition |
|---|---|---|---|---|
| A1 | D-arch-resources | Occupied Resources → Filesystem (the `viola-root-watch` Watch report row) | "the 9 root waits on a child … the ninth" becomes the pattern count of waits on `WITHIN`, 22 sites in 16 files, the two shared support files named | REJECTED before the checks: its rationale rests on the detector's own greps over the tree (`Watch::start` 34 → 26, the sites its pattern misses) and on two earlier chunks' `scope.md` and `evidence/`, none of which the report carried when it read it. The amendment itself is the plan's third expected entry: raised by the orchestrator, O1 below. |

## test-plan (4)
| # | detector | section | change, in short | disposition |
|---|---|---|---|---|
| T1 | D-tests-framework | §2 Test directory + naming conventions (the sync E2E suites bullet) | the helper list gains `cli.rs`; `events.rs` and `cli.rs` are the one shared copies; no root test file defines its own | APPLY — check 1, "Accurate this-chunk addition"; check 5, the plan's first expected entry names this change. |
| T2 | D-tests-coverage | §10 Mutation gate | the first whole-member `viola-e2e` score on the dev host: 718 mutants, 5356 s, 656 / 2 / 0 / 60, the 2 missed Windows-only and owed to "Windows mutation grade" | APPLY — check 1, "Accurate this-chunk addition"; check 5, the plan's second expected entry. Applied without mutant line coordinates (a `path:N` in a master is a citation the sweep would follow). |
| T3 | D-tests-coverage (dependent-of T2) | §3 → Bootstrap phases (derive for route / setup-project) | the new wall beside the `78 m` reading | APPLY with T2 — the plan's second entry says "beside the 78 min reading it carries"; that reading is this key file's line 16, spelled `78 m` (the orchestrator's own read; the report's search line was corrected). |
| T4 | D-tests-framework, `escalate` | §3 → 5-command implementation | hold "all 9 root waits" and "the ninth wait" until the count's rule is read; move it with architecture's in one apply | RESOLVED, no halt — check 1's recorded-direction arm: the operator's word (`inputs#I5`) is that the count is amended after its rule is read. The orchestrator read the rule (the report's Counts bullet now states it) and applied this site in the same pass as O1, with the same number and rule. |

## Orchestrator-raised
| # | check | section | change | disposition |
|---|---|---|---|---|
| O1 | 5 (the plan's third expected entry) | architecture §Occupied Resources → Filesystem (the `viola-root-watch` Watch report row) | "used by the 9 root waits on a child; the ninth is … `wait_endpoint_gone`" becomes the pattern count the report measured (22 sites of `Instant::now() + WITHIN` in 16 files under `tests/`), stated with its rule; `wait_endpoint_gone` stays named, without an ordinal | APPLY — routine: the report substantiates it (Counts / qualifiers moved, with the rule read at the resumed wrap), and the plan's entry names the change. No boundary is involved: a test-only registry row. |

## The six checks
1. Playbook: T1, T2, T3, O1 match "Accurate this-chunk addition". T4 matched no rule and was settled by the
   recorded direction. No boundary widening.
2. Cross-contradiction: none. O1 and T4 edit the same claim in two files, in the same direction.
3. Intent-consistency: the report's seven deviations are each justified in the report and none changes behaviour;
   the scope record is empty (`scope: clean`). No divergence from the frozen route line or the plan's 21 criteria.
4. Absence needs evidence: O1's "no other site" rests on the sweep in `cascade-dispositions.md`. The report's one
   false search line (`78 min`) was corrected in the report before any apply.
5. Expected amendments: three entries. First → T1. Second → T2 and T3. Third → O1 (with T4 as its test-plan twin).
   No citation row was dispositioned `claim false`.
6. Disproved claims: the report lists none in a master. Its plan-level count (three tests named, four found) is a
   deviation, dispositioned there.

## Outside the report — two stale clauses the detectors flagged, not this chunk's
Neither is a change of this chunk and neither is amended here (amendment-flow: a fact no report carries waits for
the wrap whose report carries it; the playbook's "Not this chunk's drift" sends a real one to its owner).
Both were read by the orchestrator and are real:
- `obs-plan.md:556` says `src/human.rs` `refuse` → `write_refusal` is "the root bin's only human-stderr writer,
  called only by `run`". `src/cmd/verify.rs` calls `human::refuse` at four sites, and ten files under `src/` use
  `human::` (`grep` at this wrap, HEAD `632f6a7`). Routed at P5.
- `security-plan.md:165` (Threat Model Summary, CI/CD jobs) lists "mutation (ubuntu and windows legs plus a union
  verdict)" and no perf job; `ci.yml`'s jobs read `test`, `perf`, `msrv`, `fuzz-replay`, `lint`, `release`,
  `supply-chain`, and `security-plan.md:368` says "`ci.yml` runs no mutation job". Routed at P5.
