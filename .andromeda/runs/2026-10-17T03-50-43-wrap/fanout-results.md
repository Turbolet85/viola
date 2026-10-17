# Fan-out results — 2026-10-17-cli-machine-contract (wrap 2026-10-17T03-50-43, a 2a resume re-fanned at P2)

Batch: one parallel batch of 7, inside one snapshot pair. Before: `tree 00998ca961c6e7a85fd19b10f638162de460d42e index
6395a6710c94ff2d91c15b0a428c5c32c658b8a1 head f738cbc0f307ea6c4bfc931dd1cb9096998ef0d1`. After: `tree
6898cedfb41c397e3b8c7a08b17a2fd43ab005c5`, index and head unchanged; `git diff-tree` printed exactly the seven `A
.andromeda/runs/2026-10-17T03-50-43-wrap/fanout-{doc}.md` lines. No file needed a fix: no twin. Every receipt's count
matches its file.

Validate's opening rule: 0 proposals rejected for a source the report does not carry (every `basis` is a report
coordinate or a `## New text, by line` row: revive.rs 269-275 · 244-246, client.rs 120-124, mod.rs 208-219 · 221-239 ·
241-249, main.rs 100-107, config.v1.json 1-18, obs.rs 335-353, replay.rs 174-181; the master coordinates are the
report's site sweep's).

## architecture: 11 proposals · fanout-architecture.md
note: no state-recovered site in architecture; the revive --json post-preflight gap stated as landed, its owner P2/P7's.
1. apply — §Established Decisions [CLI Conventions] revive takes `--json` · check 1 "Accurate this-chunk addition" + the plan's Expected amendments entry; its gap clause per Escalate Q2.
2. apply — §Occupied Resources subcommands row (:300), dependent of 1 · as 1.
3. apply — §Conventions CLI exit codes, revive's exit-1 documents, dependent of 1 · as 1.
4. apply — §Occupied Resources Filesystem `instances/` row, revive's strict-modes refusal under `--json`, dependent of 1 · check 1 accurate addition.
5. apply — §Standard Contracts CLI `--json` output, revive's documents registered · check 1 + Expected amendments; its gap clause per Escalate Q2.
6. escalate (Q1) — §Occupied Resources Environment variables `VIOLA_DIR` row (:321), every CLI verb a reader · check 1: a validated surface (the CLI home) admits a new input class (an environment variable) — the boundary-widening class judged by subject; the plan's "leaned, not asked" is a direction, never a ratification.
7. escalate (Q1) — §Cross-cutting Config management (:398), the `VIOLA_DIR` step + canonicalisation as landed, dependent of 6 · as 6.
8. apply — §Occupied Resources Filesystem `diagnostics/` row (:341), `revive --list` a `cli` producer · check 1 accurate addition + Expected amendments; rides Escalate Q3 (security 1's class).
9. apply — §Conventions CLI exit codes (:129), `role_of` files `revive … --list` under `cli`, dependent of 8 · as 8.
10. apply — §Occupied Resources Repository, `schemas/config.v1.json` · check 1 accurate addition (the registry lists each `schemas/*.v1.json`, architecture.md:358-362 — not over-reach) + Expected amendments.
11. apply — §Infrastructure Patterns → Project directory structure, `schemas/` row lists `config.v1.json`, dependent of 10 · as 10.

## security-plan: 3 proposals · fanout-security-plan.md
note: auth / deps clean; the Home path sentence ("`--home` and `VIOLA_DIR` are canonicalised") now-true, to be tightened with 2 ("when it exists", plain drive form); 1 worded "a preflight refusal" on purpose (the gap).
1. escalate (Q3) — §Input Validation CLI row, revive's `--json` on both arms · detector severity `escalate` (D-security-input); main's reading: output of closed codes only, no new input class; the `--list` log write lands after the strict-modes check.
2. escalate (Q1) — §Input Validation CLI row, the CLI home resolution (`VIOLA_DIR` step) + canonicalisation as landed · detector severity `escalate` + check 1 boundary-widening class (as architecture 6).
3. escalate (Q3) — §Authentication & Authorization `~/.viola/` row, revive's strict-modes refusal's `--json` document, dependent of 1 · as 1.

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; the §Surface: cli sites lie outside this detector's invariant.
1. raised by main (check 5, the plan's Expected amendments entry) — §Surface: cli → Output structure per verb, `revive` (design-system.md:506): retire "The verb takes no `--json`"; name its `--json` documents · routine (check 1 accurate addition); gap clause per Escalate Q2.
2. raised by main (check 5) — §Surface: cli → Exit-code phraseology, exit-1 row (design-system.md:513): its `--json` cell "— (no `--json` document until arch amends it)" gains revive's `{"v":1,"error":"<code>","detail":null}` document · routine; gap clause per Escalate Q2.

## layout-templates: 3 proposals · fanout-layout-templates.md
note: layout-templates.md:58 asserts no absence of `--json`, left unproposed.
1. apply — §Output structure `viola revive` (:243) · check 1 accurate addition + Expected amendments; its gap clause per Escalate Q2.
2. apply — §Component — Primary navigation (:266), the owed clause retired, dependent of 1 · as 1.
3. apply — §Component — Primary content block 2, `revive --list` joins the `cli` catch site; revive's `--json` refusal document · check 1 accurate addition (report: `role_of`).

## test-plan: 9 proposals · fanout-test-plan.md
note: framework / harness hold; Part A's "reads red with the fix reverted" for the concurrent case left to the wrap (no test-plan site); the gap's owner left to P2/P7.
1. apply — §1 CLI verbs entity (:67) · check 1 accurate addition + Expected amendments.
2. apply — §1 Surface cli, dependent of 1 · as 1.
3. apply — §5 On-disk strict-modes refusals bullet, dependent of 1 · as 1.
4. apply — §6 Exit-cause matrix exit 1 revive (:862) · check 1 + Expected amendments; its gap clause per Escalate Q2.
5. apply — §6 Exit-cause matrix `revive --list` sub-bullet, dependent of 1 · as 1 (rides Q3).
6. apply — §5 CLI `cli_controls_not_disableable` bullet (:593), completeness case landed · check 1 + Expected amendments (functions named, no count — Tier 1 2026-10-16).
7. apply — §5 Wrapper channel out-of-range `Percent` bullet, the "three rows" count retired, dependent of 6 · check 1 + playbook "A count in a master that carries no rule" (the count retired, its cases named).
8. apply — §5 CLI completeness sub-bullet (:595), its sources named · check 1 accurate addition.
9. apply — §5 CLI, a new bullet for `tests/cli_machine_contract.rs` and the home-resolution cases · check 1 accurate addition; the resolution order it names lands with Escalate Q1's disposition.

## obs-plan: 14 proposals · fanout-obs-plan.md
note: stack / PII clean; no `cli.revive` dispatch span is named by the report (not measurable from it — not proposed); exemption 6's retirement either renumbers §10 or keeps a retired marker.
1. apply — §4 cli row (:530), `revive --list` logs as `cli` · check 1 accurate addition + Expected amendments (rides Q3).
2. apply — §1 cli Exporter (:143), dependent of 1 · check 1 + playbook "Verbatim upstream copy kept current".
3. apply — §2 zero-unlogged-panics bullet (:462), dependent of 1 · as 1.
4. apply — §7 Panic hooks catch site (:822), `role_of`, dependent of 1 · as 1.
5. apply — §10 exemption 6 retired (:988), dependent of 1 · as 1; main's form: item 6 kept as a retired marker so exemption 7 and the citations of 2, 3 and 5 keep their numbers (`.claude/rules/observability.md:39` cites 7).
6. apply — §10 exemption 7 (:989), both arms, the "named in exemption 6" reference restated in place, dependent of 1 · as 1.
7. apply — §10 exemption 1, the pre-init window names `--list`'s check, dependent of 1 · as 1.
8. reject (no site) — §7 pre-init window bullet, dependent of 1 · obs-plan §7 holds no separate pre-init bullet (read at apply: §7 Panic hooks, lines 817-830); the fact is carried by exemption 1 (item 7).
9. apply — §3 → OTel SDK init (registry `otel-sdk-init.md:20`), dependent of 1 · as 1.
10. apply — §6 `state-recovered` row, `scanned` · check 1 accurate addition + Expected amendments.
11. apply — §4 E5 (:656), `scanned` beside the caller's `instance`, dependent of 10 · as 10.
12. apply — §6 `process-exit` exit 1 from revive, its `--json` document · check 1 accurate addition.
13. apply — §6 Filesystem refusals, revive's refused check under `--json`, dependent of 12 · as 12.
14. apply — §1 exit-cause bullet, revive's strict-mode fail under `--json`, dependent of 12 · as 12 (+ verbatim-copy-kept-current rule).

## a11y-plan: 2 proposals · fanout-a11y-plan.md
note: D-a11y-obs-schema clean; §1 still lists `link`, `unlink` — not this chunk's.
1. apply — §1 CLI verbs entity (:27) · check 1 accurate addition + Expected amendments + verbatim-copy-kept-current rule; its gap clause per Escalate Q2.
2. apply — §8 Error recovery CLI bullet, scoped to the human form, dependent of 1 · as 1.

## Checks 2-6
- Check 2 (cross-contradiction): none — every revive proposal states the same shape; the gap wording waits on Q2 for all of them at once.
- Check 3 (intent-consistency): the scope record's one line (`in-intent tests/support/outer_pty.rs`, serves step 8) holds. Part A's "reads red on the tree with the fix reverted" for `stamped_home_concurrent_…` — a justified divergence (testing.md 2026-09-27: a timing window is forced, never sampled; the unit witness `captured_reads_no_capture_while_the_claim_is_empty` reads red-reverted / green); no master carries the criterion — recorded here, no amendment. The revive gap diverges from v1-39 and the plan's "every exit-1 refusal" → Escalate Q2.
- Check 4 (absence): the report's site sweep basis carries each count; no proposal claims caught-ALL beyond it.
- Check 5 (expected amendments): every plan entry has a proposal except design-system's two, raised above; `matrix#v1-39` → P7.3.
- Check 6 (disproved claims): (1) Part A's premise — a plan/scope claim with no master site, disposed to curation (the report's two candidates) and this record; (2) the revive `--json` gap → Escalate Q2 (owner and v1-39's disposition).

## Escalate — resolved (the dialog's answers, `relay-1.md`, snapped `inputs#I3`, cited in the report's addendum)
- Q1 → "Landed, not ratified" (the Viola overseer, 2026-10-17T04:05:25Z — a delegate, so no ratification): security-plan 2,
  architecture 6 and 7, test-plan 9 apply as built, each body and sidecar marking the CLI verbs' `VIOLA_DIR` home step
  and the canonicalisation NOT ratified, the founder's own word owed at his next batch (the handoff carries it).
- Q2 → "Un-claim v1-39" (the overseer, a ledger call): P7.3 runs `matrix.py unclaim` on `v1-39` with a dated note naming
  the gap; P5 pins the post-preflight `revive --json` documents as a CARRY on "Exit-cause code catalogue". Every gap
  clause above lands as: a `run::start` refusal after a passed preflight prints no document, owed to that entry.
- Q3 → "Apply as routine" (the overseer, technical): security-plan 1 and 3 apply, with every dependent above.
- Escalations open: 0.

## Phase 2 summary
47 amendments applied (42 proposals less 1 with no site, plus 2 raised by main, plus 4 sweep folds: design-system.md:482
×2, security-plan.md:214, obs-plan.md:754) · 3 escalations resolved (Q1-Q3, `inputs#I3`) · 0 proposals rejected for a
source the report does not carry · 1 rejected for no site (obs-plan 8) · citations 0 re-pointed · 0 changed · 0 stretched.
Sidecar entries: architecture 2, security-plan 1, design-system 1, layout-templates 1, test-plan 1, obs-plan 1,
a11y-plan 1. Leaves: 25 written by the tool, 2 by hand, 1 declined (`cascade-dispositions.md` §Leaves).
