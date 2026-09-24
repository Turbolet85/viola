# Evolve Diagnosis — viola-0.1.0 · Epoch 1 — Foundation · 2026-09-24T17:37:22Z

Chunk markers below drop their shared `2026-09-24-` prefix. `L{n}` = line n of `.andromeda/friction-log.ndjson`;
`L{n}.{i}` = problem fact i of that step record. Raw query outputs: `q-retractions.json` · `q-health.json` ·
`q-typed.json` · `q-chains.json` · `q-level.json` in this run dir.

## Mechanism health
- **Records:** 190 in epoch (111 step / 79 friction); ledger 191 total, 0 unparseable, 0 malformed `ts`, `id` on 190/190.
- **Retractions:** 0 retracted · 0 clause-retracted · 0 unresolvable.
- **Coverage:** 8/8 chunks carry exactly the expected step records (phase 5 · implement 3 · wrap 5), and new-session has 7 records across the epoch's session starts. 0 checkpoint gaps.
- **Outcomes:** 105 ok · 3 ok-degraded (all implement) · 3 halted-resolved (wrap/gates ×2, wrap/reconcile ×1) · 0 soft-exit / aborted.
- **Untyped rate:** 15/79 = 19 %. By step: reconcile 4/14 · fix-loop 3/12 · research 2/8 · take-up, distill, validate, code, gates, route-resolve 1 each · the rest 0.
- **Problem-fact fill:** 36/111 step records carry 50 problem facts.
- **Calibration:** every record is from 2026-09-24, so all boundaries apply for the whole range: deviation scan, `id`, Universal types and `contract.grammar-irregularity`.
- **Producer-spelling split:** the `skill` field mixes `andromeda-`-prefixed (127) and bare (63) forms, and even within one chunk. The fold was applied before grouping; without it every group splits roughly 2:1.

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 10 cases (8 chunks) · weight 14
**Pattern:** validate's mechanical checks fired a REQUIRED-RESOLUTION in every chunk of the epoch. The same checks recur: check 9 (the research Platform-issues slot reads `none` next to a CI-reading entry) in 4 chunks; check 4(5)/(9)/(6) (an atom with no source, a probe with no `new` mark, a criterion with no gate) in 5; check 3 (unregistered `target/` dir) in 2.

| chunk | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton | check 7 WARN by construction: no tree-query trace on a cold-start chunk (no tree.db) | — | — |
| fake-agent-and-test-data-fixtures | check 9: Platform-issues slot `none` while the fence carries a CI-reading entry; web fetch + slot rewrite | extra_reads 2 | — |
| fake-agent-and-test-data-fixtures | check 8: `--exit-no-eof` criterion vacuous under `run_viola` (Stdio::null); notes amended to require piped stdout | extra_reads 1 | — |
| supply-chain-and-workflow-gates | check 4 ×4 on the P4 fence: (5) operator-leg atoms with no source · (9) two tool probes unmarked new · (4) CI JSON with no producer · check 3 target/ dirs unregistered | iterations 1, extra_reads 3 | runs/2026-09-24T09-16-31-phase/baseline/ |
| diagnostics-plane | check 9: Platform-issues slot `none` ("no CI-reading entry") while the plan lists the operator check-runs read | — | research.md §Scope |
| log-redaction-and-never-log-floor | 4(5) atom with no source · 4(9) new-scope filter lacks `new=true` · 9 Platform-issues `none` beside a runner-only bullet | iterations 1, extra_reads 2 | — |
| observability-gates | 4(9) baseline: shims reject unknown verbs, so the plan's harness subcommands were unreachable; shims added to modify-set | extra_reads 2 | runs/2026-09-24T11-55-47-phase/b17.log |
| observability-gates | check 8: step 8's "boot owns the wrapper mutably" false (home.rs:139); 4(6) deadline criterion with no gate | extra_reads 2 | — |
| quality-gates | 4(6): 4 criteria no entry proved · 4(9): entry 13 atom unpassable twice, corrected to `failed,` | iterations 2, extra_reads 2 | P5 baseline logs (session scratchpad) |
| workspace-tree-and-code-graph-planes | check 3 (target/release-check unregistered) · 4(6) (no-tsconfig criterion, no gate) · 9 (empty Platform-issues slot) | — | — |

**Proposal:** the checks work: each defect was resolved before review. What recurs is that the upstream producers keep leaving the same fields empty. One direction is to move the recurring three upstream as authoring-time requirements. Research would fill the Platform-issues slot whenever scope names a CI or runner read. P4's plan template would state the atom source, the `new` flag and the gate-per-criterion rule next to each entry. `target/` dirs minted at P3/P4 would be registered in Occupied Resources as they are minted. Validate then confirms instead of repairing. The check-7 cold-start WARN is a separate, by-construction case: validate could exempt the first chunk before tree.db exists.

### P2 — implement/fix-loop · `contract.spec-reality-gap` — 4 cases (2 chunks) · weight 9

| chunk | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton | test-plan §3 filterset `kind(test) & !binary(...)` rejected by nextest 0.9.133 when no binary matches; harness uses `kind(test)` | iterations 1 | — |
| three-os-ci-headless-harness-skeleton | test-plan §3 shims build into the same target the running viola-harness.exe lives in: Windows relink os error 5; moved to target/harness | iterations 3 | — |
| supply-chain-and-workflow-gates | cargo-deny 0.20.2 moved `--config` to a global option; plan gate 4 used the 0.19.4 form P3/P5 ran under | iterations 1 | runs/2026-09-24T09-34-05-implement/gate4-corrected.log |
| supply-chain-and-workflow-gates | plan gate 10 declares artifact `mutants.out/` on a no-rust-delta run that never runs cargo-mutants: freshness STALE by construction | iterations 0 | — |

**Proposal:** the project absorbed two of these mid-epoch. Relink became the host-win32 session addition (2026-09-24). Tool version became the Tier-1 learning "probe, baseline and gate at the CI-pinned version". A pipeline-level generalisation would be for P5's baseline to record each tool's `--version` next to its entry and compare it with the CI pin. That turns the 0.19.4-vs-0.20.2 class into a validate finding. For the gate-10 case, the artifact-freshness check could exclude artifacts under a verdict (`no-rust-delta`) that by contract never produces them.

### P3 — wrap-session/reconcile · `input.report-insufficient` — 3 cases (3 chunks) · weight 8 (1 halt)

| chunk | what | impact | evidence |
|---|---|---|---|
| fake-agent-and-test-data-fixtures | report listed the fixture lookup shape but did not flag it against test-plan §2 kebab naming / §3 `--fixtures` dir; the tests detector escalated both | dialogue 1, halted 1 | runs/2026-09-24T08-45-42-wrap/fanout-results.md |
| quality-gates | report's obs expected-amendment placed `mutants.out` at obs §8 point 6; the only hit is §9:1249; the detector corrected the site | extra_reads 1 | runs/2026-09-24T14-48-15-wrap/.raw-fanout-obs-plan.md |
| workspace-tree-and-code-graph-planes | arch proposal 7 rested on a rust-plane coverage fact only research.md carried, and mis-named cargo-modules as the plane | extra_reads 1 | — |

**Proposal:** the three cases fail in three ways: a doc-vs-code shape conflict went unflagged, a site coordinate was wrong, and a research-only fact was missing. They share one cause: report's Changes families are written "from session knowledge". Every report step record says so, and none carries a signal (see X3). One direction is a report-side sweep that diffs implement's new names and shapes against the spec sections that name the same concept. Another is to carry research.md's Measured facts into the report as a family. Both would make the report the fan-out's complete basis instead of leaving the detectors to rediscover facts.

### P4 — wrap-session/curation · `ambiguity.filter-borderline` — 4 cases (4 chunks) · weight 4 (1 deferred)

| chunk | what | impact | evidence |
|---|---|---|---|
| diagnostics-plane | all three new Tier-2 entries scored exactly 0.6 and passed only through the conditional no-other-home +0.2 | — | — |
| log-redaction-and-never-log-floor | both applied learnings at exactly 0.6, passed only via no-other-home +0.2; a third at 0.6 rejected (obs :485 states it) | — | — |
| observability-gates | four at 0.8, two only via a conditional +0.2 on a 0.6 base; the Filter-5 cap chose among equals with no ordering rule | — | — |
| quality-gates | three at 0.6 + no-other-home → tied 0.8; the Filter-5 cap pick underdetermined (git check-ignore deferred) | deferred 1 | runs/2026-09-24T14-48-15-wrap/curation.md |

**Proposal:** every case sits on the same boundary. The signals score 0.6, and the conditional +0.2 decides the outcome. When the Filter-5 cap binds, nothing orders a tie. One direction is a declared tiebreaker in the curation guide, for example measured-on-this-host first, then recurrence count, then recency. Another is to have the conditional +0.2 apply only after a base of more than 0.6, so the conditional stops being the deciding term in most cases.

### P5 — phase/distill · `contract.binding-contradiction` — 3 cases (3 chunks) · weight 3

| chunk | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton | arch: no tokio/anyhow in viola-e2e; test-plan §3 gives viola-e2e tokio test-util; carried to P4 | deferred 1 | — |
| supply-chain-and-workflow-gates | the weekly advisory run's home differs across arch §CI/CD + obs §9 (ci.yml), test-plan §9 (nightly.yml) and security (a separate schedule) | retries 0 | runs/2026-09-24T09-16-31-phase/{arch,tests,security,obs}.md |
| quality-gates | arch + security require rustup install; tests keeps dtolnay/rust-toolchain for MSRV + fuzz jobs (an earlier sweep left those lines) | retries 0 | runs/2026-09-24T13-41-49-phase/{arch,security,tests}.md |

**Proposal:** distill reported each contradiction faithfully, and P4 forked on it. The contradictions themselves were cross-spec inconsistencies that an earlier wrap cascade did not sweep. The third case says so outright: "the harness-skeleton sweep left those test-plan lines unchanged". One direction is for the wrap cascade's sweep to follow an amended mechanism (a toolchain source, a workflow home) into every master that names it, not only the master the amendment targeted. The next chunk's distill would then stop meeting the stale half.

### P6 — phase/research · `contract.premise-falsified` — 3 cases (3 chunks) · weight 3
Also counted inside P8.

| chunk | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton | test-plan mutation base (`merge-base HEAD origin/main`) and PR-only mutation job presume main + PRs; only build/viola-0.1.0 exists | deferred 1 | viola-0.1.0/chunks/…-three-os-ci-headless-harness-skeleton/research.md |
| supply-chain-and-workflow-gates | arch "graph with viola/viola-mcp/viola-ui excluded" read as `cargo deny --exclude` false-fails under feature unification | — | …-supply-chain-and-workflow-gates/research.md §Measured facts |
| quality-gates | test-plan §3: coverage JUnit path wrong; ignore regex cannot match llvm-cov's Windows backslash paths | extra_reads 2 | …-quality-gates/research.md M5, M6 |

**Proposal:** research did its job: each falsification was measured before plan. Two of the three are spec facts about a branching model or a host path form that the specs never measured. That fits the Tier-1 learning "an unmeasured behaviour is a ledger row to probe". One direction is a spec-authoring convention, for the specialists or `/andromeda-route`, that marks unmeasured mechanism claims as `[unmeasured]`. Research could then target them directly instead of finding them by collision.

### P7 — wrap-session/reconcile · `contract.false-positive-proposal` — 3 cases (3 chunks) · weight 3

| chunk | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton | D-security-input/-auth proposed Decisions-Log deferral prose for gaps the markerless route already owns; also cited a location the report lacks | — | — |
| fake-agent-and-test-data-fixtures | D-arch-decisions proposed a Stack Testing row though arch [Deferred] delegates the test framework to test-plan | — | — |
| supply-chain-and-workflow-gates | D-arch-resources proposed registering individual config files in Occupied Resources; only the target/ dirs applied (Registry over-reach) | iterations 0 | runs/2026-09-24T09-41-13-wrap/.raw-fanout-architecture.md |

**Proposal:** all three were rejected at zero cost. They were in the first three chunks and did not recur in chunks 4–8, so the detectors may already have calibrated. If the founder wants to prevent recurrence, the direction is detector-side: tell the security detector that the markerless route owns sequencing gaps, tell the arch-decisions detector about the `[Deferred]` delegation, and set the Registry granularity (dirs and ports, not individual files) in the playbook's Registry row.

### P8 — cross-step · `contract.premise-falsified` (Universal) — 9 cases (6 chunks + 2 session starts) · weight 15

| chunk / step | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton · research | mutation base presumes main + PRs (= P6) | deferred 1 | research.md |
| supply-chain-and-workflow-gates · research | `--exclude` graph premise false-fails (= P6) | — | research.md §Measured facts |
| diagnostics-plane · plan | operator-relayed probe `guard_probe.py` asserts the SHIPPED guard, so it cannot witness the fix; repo-local probe chosen | dialogue 1 | plan.md step 13 |
| (session start) · new-session | handoff expected all-green CI for 809456e; CI mutants failed (obs.rs:193 missed) though local mutants read 0 survived | extra_reads 5 | actions run 35990393334 job 107602836363 |
| (session start) · new-session | handoff expected all-green for 2834e4d; CI mutants 2 TIMEOUT on Linux; local Windows counted 5/5 caught | extra_reads 3 | gh run view 35995290314 --log-failed |
| observability-gates · code | obs-plan §3: the inner `#[allow]` in `obs_event!` lets callers lint clean; false on clippy 1.98.1 (16 sites) | iterations 2, dialogue 1, extra_reads 2 | runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md |
| observability-gates · code | Cargo.lock-unchanged probe red by construction (a new dependency-edge line) | extra_reads 1 | — |
| quality-gates · take-up | prior premise "ubuntu CI mutants kills the 3 cfg(unix) file_mode mutants" falsified by run 36005608858 | deferred 1 | CI run 36005608858 |
| quality-gates · research | coverage JUnit path + Windows backslash ignore regex (= P6) | extra_reads 2 | research.md M5, M6 |

**Proposal:** four of the nine are one sub-class: the **local mutation verdict differs from the CI mutation verdict** (the two new-session cases, quality-gates take-up, and by root cause the file_mode case). The host-vs-CI asymmetry is the recurring premise. See L4 and U1, which point the same way from other streams. The remaining five are measured spec or plan premises and have no shared mechanism.

### P9 — cross-step · `recall.corpus-recurrence` — 3 cases (3 chunks) · weight 5

| chunk / step | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton · curation | host-win32 already rules pipe exit = last stage; the plan still authored `grep … \| grep -Evc …`, caught only by P5 | iterations 1 | — |
| diagnostics-plane · curation | `env = []` for self-setting runs is in plan-template §Test Commands and the prior plan; P4 dropped it when copying the entry | — | — |
| log-redaction-and-never-log-floor · reconcile | cascade amended obs :54 (obs §1, kept verbatim by rule obs :485), a rule the previous sidecar had already applied; reverted before commit | retries 1, extra_reads 2 | — |

**Proposal:** each rule existed in the corpus the author had loaded or copied from, and was not applied. The third case recurs as untyped L91 (the same obs §1 rule, one chunk earlier; see U2), so the obs §1 kept-verbatim rule has now been missed twice. One direction is to turn recall into mechanism where it is cheap. The cascade's sweep could skip obs §1 by rule, a structural exclusion rather than a remembered one. The plan template could lint gate entries whose `run` sets a shell variable for a missing `env = []`. gate.py's dry-run could flag the scavenge (see L3).

### P10 — cross-step · `contract.structural-blind-spot` — 3 cases (3 chunks) · weight 4

| chunk / step | what | impact | evidence |
|---|---|---|---|
| three-os-ci-headless-harness-skeleton · wrap/gates | three runner-only red causes; the only gate that sees them is the post-push CI read, which fires after the wrap commit and the v1-06 flip | iterations 1 | runs/2026-09-24T07-05-59-wrap/ci-failed.log |
| observability-gates · take-up | Setup 5a reads CI for `git log -1`; an out-of-pipeline docs commit (2969198) hid the wrap sha's mutants red | extra_reads 1 | check-runs 2969198 vs 2834e4d |
| quality-gates · research | ubuntu-only mutants cannot kill mutants in another OS's cfg-gated bodies (cargo-mutants limitation) | — | …-quality-gates/research.md M1, M2 |

**Proposal:** two of the three are about **CI-only evidence arriving after the wrap commits** (see L7). The third is the mutation host asymmetry (see L4). For the Setup-5a case, one direction is to read CI for the sha recorded as the wrap's commit (the handoff's `Last Commit` or the master record), not HEAD.

## Cross-step chains (starting heuristics)

### X1 — phase/validate →plan→ implement (code | fix-loop) — 3 chunks
- **supply-chain-and-workflow-gates:** validate L58 `ok` (its L59 check-4 friction fixed 4 items) → fix-loop L61 `ok-degraded`, plan `wrong` (gate 4 used the 0.19.4 form; gate 10 artifact STALE) → L62, L63.
- **diagnostics-plane:** validate L80 `ok` (`review-note-folded`; L81 check 9) → fix-loop L84 plan `thin` (entry 12 lacked `env = []`) → untyped L85, "P5's dry-run parse cannot show an env scavenge".
- **observability-gates:** validate L125 `ok` (`baseline-caught-defect`; L126/L127) → code L129 `ok-degraded`, plan `wrong` → L130, L131; fix-loop L132 `ok-degraded`, plan `thin`.

**Hypothesis:** in each chunk validate was formally `ok` and caught defects, but a defect it could not see survived: a tool version not at the pin, a dry-run that does not execute, a premise only a real clippy run exposes. All three of the epoch's `ok-degraded` outcomes sit at this join.
**Direction:** P5's baseline executes new or changed entries for real at the CI-pinned tool version (see P2). gate.py's dry-run reports the env keys an entry would scavenge.

### X2 — phase/validate →plan→ wrap-session/gates — 2 chunks
- **three-os-ci-headless-harness-skeleton:** validate L8 `ok` → gates L24 `halted-resolved`, plan `thin` (CI witness entries fire only after the wrap commit) → L25.
- **observability-gates:** validate L125 `ok` → gates L143, plan `thin` (2 entries skipped on operator-ratified reasons) → untyped L144.

**Hypothesis:** in both chunks the plan's gate set holds entries whose evidence only CI can produce. Validate cannot classify them, and the wrap's light gate has no arm for them. Joins L7.

### X3 — wrap-session/report →report→ wrap-session/reconcile — 2 chunks
- **fake-agent-and-test-data-fixtures:** report L41 `ok`, signals `[]` → reconcile L42 `thin` → L43 (halted).
- **workspace-tree-and-code-graph-planes:** report L181 `ok`, signals `[]` → reconcile L183 `thin` → L184.

**Hypothesis:** the producer emitted no signal in either chunk, so this chain is a join on artifact plus downstream friction only. The report playbook's signal list may lack one for "Changes written without a research/spec cross-check". Joins P3.

### X4 — route →working-entry→ phase/take-up — 2 chunks (observation)
L1 (three-os-ci-headless-harness-skeleton, "skeleton + five agent-run commands" leaves boot readiness undefined) and L171 (workspace-tree-and-code-graph-planes, four clauses with no mechanism, filled as `[inferred]`). The producer is route authoring, outside the step stream, so there is no producer record to examine.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 5 facts (4 chunks), environment/workaround
**Facts:**
- three-os-ci-headless-harness-skeleton · validate · L8.0 — rm -r of the P5 control dir denied twice; overwrote its Cargo.toml with comments instead
- supply-chain-and-workflow-gates · research · L53.0 — a cat heredoc writing scratch manifests blocked by the PreToolUse hook; Write tool + script by path
- observability-gates · validate · L125.0 — compound mkdir+rm+G2 control denied; re-run as a scratchpad copy
- observability-gates · validate · L125.1 — the PreToolUse hook blocks Write to any path containing `/target/`, scratchpad included; python by path
- observability-gates · code · L129.2 — an rm -rf compound denied; single mv instead

Typed correlate: untyped L128 (2 retries).
**Level hypothesis:** the obstacle is the session's guard configuration: the permission layer, plus a write-guard hook whose path match covers the scratchpad. Each chunk routes around it again. host-win32.md already carries a prohibition ("rm -rf + mkdir compounds get denied"), and L125.0 and L129.2 happened after it existed. So the project-level rule is being recalled, not preventing the obstacle.
**Proposal:** narrow the hook's `/target/` match to the repo's own `target/`, which would exempt the scratchpad. The diagnostics-plane chunk already repaired this write-guard once (L73.0). Alternatively, the pipeline could ship a sanctioned "control dir" recipe (create and dispose through one python-by-path helper) that the validate letter names, instead of each step rediscovering the denial.

### L2 — band-aid — 4 facts (3 chunks), environment/workaround + removed-cause
**Facts:**
- three-os-ci-headless-harness-skeleton · fix-loop · L12.0 — Windows cannot relink a running exe; harness cargo work moved to target/harness
- three-os-ci-headless-harness-skeleton · fix-loop · L12.1 (removed-cause) — a Monitor `tail -F` held mutants.out open and survived TaskStop; stopped by pid
- supply-chain-and-workflow-gates · research · L53.1 — cargo deny / cargo tree probes piped through tail masked exit codes; re-ran with bare `$?`
- (session start) · new-session · L49.0 — check 13's `test -x` via native python → bash printed empty; re-probed in the Bash tool

Typed correlate: P9's L22 (the pipe-exit rule existed in host-win32 and the plan still authored it).
**Level hypothesis:** the Windows/MSYS host semantics live in the environment. The fixes land as host-win32.md session additions: two of these four became additions today. L53.1 recurred after its rule existed.
**Proposal:** where a pipeline tool wraps the command, as gate.py does for gate entries, it could enforce the pipe-exit and bare-`$?` rule mechanically instead of relying on a recalled rule. The new-session check-13 probe is a pipeline-side fix: evaluate `test -x` in the Bash tool, never through a native python subprocess.

### L3 — band-aid — 3 facts (3 chunks), process/workaround
**Facts:**
- supply-chain-and-workflow-gates · fix-loop · L61.0 — plan gate 4 invalid under cargo-deny 0.20.2; plan.md read-only at implement; ran the corrected form by hand and surfaced it
- diagnostics-plane · fix-loop · L84.0 — gate entry 12 not run by gate.py (env var scavenged); ran its exact text by hand; plan.md not edited
- observability-gates · code · L129.1 — the operator asked for a measurement in research.md; implement may not edit it; parked in the run dir for wrap

Correlates: P2 (L62, L63), untyped L85, and all three `ok-degraded` implement outcomes (X1).
**Level hypothesis:** the read-only plan at implement is designed, and it holds. The band-aid is how errata travel: hand-run commands and run-dir notes that wrap must find. The gate tool never learns the corrected entry.
**Proposal:** a sanctioned implement-side erratum channel, such as an `errata.toml` beside the plan that gate.py reads as an override of a named entry, with wrap reconciling it into the plan. That would keep the plan immutable, make the correction machine-run instead of hand-run, and give wrap one place to read.

### L4 — band-aid — 4 facts (4 chunks), process/environment workaround
**Facts:**
- three-os-ci-headless-harness-skeleton · code · L10.1 — harness cargo orchestration made testable against throwaway temp projects so its mutants are killable
- three-os-ci-headless-harness-skeleton · wrap/gates · L24.0 — cargo-mutants 27.1 ignores test_workspace/test_package for `--in-diff` package-scoped baselines; operator chose root prebuild + `--copy-target`
- observability-gates · fix-loop · L132.0 — the `[profile.mutants]` 10 s kill cut a harness test that waits out a 20 s boot deadline; per-package 30 s override
- quality-gates · code · L156.0 — plan's fuzz seam generalised to a Runner closure so coverage/dispatch mutants are killable without nested cargo

Product-logic siblings: L10.0 (three plan steps dropped as unkillable mutants) and L10.2 (a no-op feature for package-scoped runs).
Correlates: U1 (5 untyped), P8's local-vs-CI mutation sub-class (4), P10's L151, and P2's L63.
**Level hypothesis:** the mutation gate is the epoch's heaviest recurring obstacle. It appears in every stream: problem facts, untyped friction, typed premise-falsified and structural-blind-spot. Each chunk adapts code, nextest profiles or harness structure to it. The cause sits in the gate's design: a single-host local run (Windows) witnessing a property whose other half is only observable on ubuntu CI. quality-gates' two-leg union is the project's own mid-epoch absorption.
**Proposal:** a pipeline-level treatment of cfg-gated and host-unreachable mutants. Validate would classify at plan time which new code is host-unreachable, and the local atom would exclude those by construction. The CI leg would be declared as their only witness, turning the operator-ratified `--skip` (L143.0) into a designed arm. The founder may also want to compare this with how the pipeline treats other CI-only gates (L7).

### L5 — removed-cause observation — 3 removed + 2 workaround + 1 deferred (5 chunks), environment
**Facts:**
- quality-gates · research · L150.0 — host lacked cargo-llvm-cov 0.9.1, the 1.96 toolchain and llvm-tools-preview; installed
- quality-gates · code · L156.1 — host lacked the dated fuzz nightly; installed
- workspace-tree-and-code-graph-planes · research · L173.0 — host cargo-modules 0.26.0 behind the CI pin 0.27.0; installed
- workaround: three-os-ci-headless-harness-skeleton · L8.1 — rg absent on the gate PATH, grep -rE form used
- workaround: log-redaction-and-never-log-floor · L102.0 — no Rust on a Linux host; WSL busybox probe
- deferred: supply-chain-and-workflow-gates · L53.2 — zizmor absent, verdict unmeasured

The deferral was later routed: L59, L182.
**Observation:** the same cause keeps returning. Each chunk discovers the host is behind or without the tool CI pins, and removes it for that one tool. The Tier-1 learning (probe at the pinned version) encodes the rule but not the inventory.
**Proposal:** a host-tool manifest derived from the CI pins (`ci.yml` versions), which new-session or phase Setup checks, would list every gap once, before research needs the tool.

### L6 — override — 3 facts (3 chunks), phase/take-up
**Facts:**
- fake-agent-and-test-data-fixtures · L27.0 — Setup 5a red; the letter offers take-up-unowned or mint-a-chunk; operator chose a third course: fold into this chunk
- diagnostics-plane · L73.0 — an overseer note mid-Setup folded an out-of-entry item (settings.json write-guard repair) into scope as `[inferred]`
- quality-gates · L146.0 — the PREREQ's first action was performed by the overseer and delivered mid-turn, with a directive to fold both mutants as `[inferred]`

Correlate: untyped L28 (halted, 1 dialogue round: "the red-disposition ask needed a third arm").
**Level hypothesis:** the operator corrected take-up's scope rule the same way in 3 of 8 chunks, by folding an adjacent defect into the current chunk. The rule's arms (unowned or mint) appear miscalibrated for a CI red that the next chunk's own area can fix.
**Proposal:** add a third take-up arm, "fold into this chunk as `[inferred]` with a no-vacuous-pass constraint". That would be the operator's recurring choice, made explicit and recorded as such.

### L7 — override — 3 facts (2 chunks), wrap light gate / flip precondition
**Facts:**
- observability-gates · reconcile · L137.0 — the P7 light-gate ASSERT has no arm for a plan-probe defect or host-unkillable cfg(unix) mutants; operator ratified a reasoned `--skip` + a witness pin
- observability-gates · gates · L143.0 — `--skip` on 2 entries; witness pinned as a PREREQ on Quality gates
- workspace-tree-and-code-graph-planes · gates · L189.0 — P7.1 halted before the flip: 3 operator-leg entries had no evidence and v1-23 needed the pushed sha; operator chose the pre-CI pass (commit 0c2e0cc + push + CI read) before the flip

Correlates: untyped L138, P10's L25, `halted-resolved` L24 and L189, and the two new-session CI-red discoveries in P8.
**Level hypothesis:** the wrap commits and flips on local evidence. Evidence only CI produces arrives afterwards: after the commit (L25) or at the next session start (L99, L118). The operator's pre-CI commit (L189.0) is a hand-built version of an ordering the wrap letter lacks.
**Proposal:** a designed two-phase wrap for chunks with CI-only witnesses: commit and push, read CI on that sha, then flip. That is what 0c2e0cc → a28f696 did by hand. Alternatively, a light-gate arm "CI-witnessed, pending" that holds the flip and not the commit.

### L8 — chronic-degrade (within epoch) — 3 records (2 chunks)
Every `ok-degraded` outcome of the epoch is at implement (L61 supply-chain-and-workflow-gates fix-loop; L129 code and L132 fix-loop in observability-gates), and each consumed a plan rated `wrong` or `thin`. None halted, so the halt policy never surfaced them. This is the X1 join seen as outcomes. There is no cross-epoch recurrence yet because this is the ledger's first epoch.

### Deferred-forever — 0 hits
Every deferral names its routing or shows a later closure: L53.2 → L59 and L182 · L4 → P4 synthesis · L6 → L7.0 · L147 → folded · L61 and L179 `deferral-open` → the route PREREQ on Security prerequisites and the operator witness condition. One is open at epoch end and routed: L167, the git check-ignore learning, deferred by the curation cap. It sits in the handoff's deferred learnings and was not reproduced in the last session.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — implement/fix-loop (spans take-up, research, reconcile) — 5 cases → proposed type `tooling.gate-host-unreachable`
**Cluster:**
- L28 (fake-agent-and-test-data-fixtures · take-up) — CI red on a docs-only witness commit: the mutation gate reads a Rust-free diff as outcomes-missing
- L32 (fake-agent-and-test-data-fixtures · research) — cargo-mutants leaves a prior mutants.out untouched on a Rust-free diff, so the harness would read stale counts (a host-only face)
- L134 (observability-gates · fix-loop) — a `#[cfg(unix)]`-only body produces mutants the Windows host can never compile or kill; local `survived:0` unreachable by construction
- L138 (observability-gates · reconcile) — the light-gate ASSERT arms cover no red that is the chunk's own yet unfixable on the gate host
- L160 (quality-gates · fix-loop) — a second host-equivalent survivor appeared (`fuzz_host_supported -> false`), killed only by the Linux leg

**Draft criteria line:** "Did any gate entry's pass condition prove unreachable on this host by construction (cfg-gated code, a CI-only artifact, a verdict that never produces its artifact)? Record the entry, the host, and which leg alone can witness it." Because the cluster spans 4 steps, the founder may prefer it as a Universal type.

### U2 — wrap-session/reconcile (plus route-resolve, research) — 5 cases → proposed type `retry.self-apply-corrected`
**Cluster:**
- L91 (diagnostics-plane · reconcile) — the cascade citation fold edited obs :202 in obs §1 (kept verbatim by rule); reverted in-pass
- L139 (observability-gates · reconcile) — an unnumbered "new Decisions Log entry" minted D-27, which already existed; renumbered D-33 after a grep
- L186 (workspace-tree-and-code-graph-planes · reconcile) — three of the orchestrator's own applies wrong on first write (perf path placement, a contradicting sentence left behind, relative leaves), all fixed before the sidecars
- L96 (diagnostics-plane · route-resolve) — an anchored Edit carried bullet newlines, splitting one route entry into four lines; caught by the diff-size read-back
- L103 (log-redaction-and-never-log-floor · research) — a citation line number from a hand count of a cat listing; `grep -n` re-derived it

**Draft criteria line (reconcile playbook):** "Were any of your own applies wrong on first write and corrected in-pass? Count them and name what caught each (read-back, grep, a later sweep)." Three of the five are at reconcile alone. L91, together with P9's L112, is the same obs §1 rule missed twice.

## Below threshold — no action
**Typed per-step groups (n < 3, no halt):**
- implement/code `contract.premise-falsified` 2 (in P8)
- implement/fix-loop `contract.vacuous-check-found` 2
- wrap/curation `recall.corpus-recurrence` 2 (in P9)
- new-session/orientation `contract.premise-falsified` 2 (in P8)
- 1 each:
  - fix-loop: `tooling.result-not-run-stable` · `retry.fix-iterations` · `contract.test-expectation`
  - plan: `retry.synthesis-rework` · `contract.premise-falsified` · `input.research-thin`
  - report: `input.implement-outcome-unsettled` · `contract.narrow-basis-claim`
  - reconcile: `ambiguity.playbook-no-match` · `ambiguity.escalation-rounds` · `recall.corpus-recurrence` · `contract.cascade-miss`
  - gates: `contract.jointly-contradictory-instructions` · `contract.structural-blind-spot`
  - research: `tooling.host-shell` · `contract.structural-blind-spot` · `input.cookbook-gap`
  - take-up: `input.working-entry-thin` · `input.out-of-pipeline-source` · `contract.structural-blind-spot` · `contract.premise-falsified`
  - code: `input.plan-step-ambiguous` · `input.research-files-wrong`
  - distill: `contract.extract-format`
  - route-resolve: `contract.carry-no-owner`
  - smoke: `contract.skill-reference-drift`

**Cross-step Universal groups under threshold:**
- `contract.jointly-contradictory-instructions` 1
- `tooling.host-shell` 1
- `contract.skill-reference-drift` 1
- `contract.narrow-basis-claim` 1

**Untyped singletons / sub-threshold clusters:**
- L76 plus problem fact L29.0 — the Agent return transport prepends a harness warning or neutralization line to distiller returns (2, emerging)
- L83 — first schema draft rewritten whole
- L85 — the env scavenge is invisible to P5's dry-run (in X1)
- L144 — `gate.py --skip` splits its reason on commas

**Pass-A themes under threshold:**
- flip-compaction with no route.py verb, 2 (L97.0, L116.0)
- raw twins written condensed, not byte-complete, 2 (L29.0, L163.0)
- locating the CI run for the shipped sha, 2 (L98.0, L119.0)
- process-global test state admits one setting test, 2 (L82.2, L107.0)
- plan literal vs repo convention, 2 in 1 chunk (L178.0, L178.1)

**Other signatures:**
- Override: a designed escalation resolution, 1 (L89.0)
- Singletons: L7.0, L7.1, L10.0, L10.2, L42.0, L45.0, L50.0, L82.0, L82.1, L86.0
