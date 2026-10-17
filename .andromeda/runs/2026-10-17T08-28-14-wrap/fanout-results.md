# Fan-out results — 2026-10-17-cli-output-discipline

Batch: one snapshot pair, before `tree f7169baa457a10ce4dbe06dff8fb9d8de2382bd2 index 7d15df5edccda497e3f427941d2c72c0beac5f2e head 4a8a0ca52bfe9be086af8a7917edb3362a054573`,
after `tree 6be158b5f83e9dd577f048598ef8c56734760dce index 7d15df5e… head 4a8a0ca5…`; `git diff-tree` printed exactly the seven
`A .andromeda/runs/2026-10-17T08-28-14-wrap/fanout-{doc}.md` lines; index and head unchanged. No file needed a fix, so no raw twin.
Every receipt count equals its file's item count. Rejected at the opening rule (a coordinate the report does not carry): 0.

## architecture: 5 proposals · fanout-architecture.md
note: the "Project directory structure" key lacks `src/human/style.rs` and `tests/support/sgr.rs` and its `board.rs` gloss omits the cut and spans; its `tests/cmd/*.toml` row is a [trycmd] site; §Stack → CLI parser stays true.
1. §Occupied Resources → Environment variables (the colour group) — ESCALATE (check 1: boundary widening) → answer card (the founder's own answer to P4 fork Q1, `inputs#I2`).
2. §Established Decisions [Naming] (dependent of 1) — ESCALATE → answer card.
3. §Conventions → Environment variables (dependent of 1) — ESCALATE → answer card.
4. §Cross-cutting Patterns → Config management (dependent of 1) — ESCALATE → answer card.
5. §Infrastructure Patterns → Crate dependency direction (the root bin's `Win32_System_Console` use gains the stdout VT enable) — APPLY (check 1: Accurate this-chunk addition; Dependencies none).
6. (raised by main, from the note) §Infrastructure Patterns → Project directory structure — register `src/human/style.rs` (the colour layer) under `src/human/`, the board's width cut and spans in its `board.rs` gloss, `tests/support/sgr.rs` under `tests/support/` — APPLY (check 1: Accurate this-chunk addition).
7. (raised by main, check 6) §Infrastructure Patterns → Project directory structure, the `tests/cmd/*.toml` "trycmd cases" row — part of the [trycmd] family → ESCALATE (the family's disposition, question 3 of the halt).

## security-plan: 3 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps no drift; the Threat Model vector lists entry points without claiming closure (no occurrence); :211 holds.
1. §Input Validation (a new row for the colour facts) — ESCALATE (check 1: boundary widening) → answer card (`inputs#I2`).
2. §Secret Management → Storage (dependent of 1) — ESCALATE → answer card.
3. §Security Anti-Patterns → Universal (dependent of 1) — ESCALATE → answer card.

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; the expected amendments are left to main.
1. (raised by main, check 5: plan's Expected amendments) §Component Patterns pattern 1 As built (design-system.md:441) — the colour layer built (bold NAME, amber `DIALOG`, dim stale row, the composition), unwrapped NAMEs cut on a narrow terminal; "uncoloured … owed to 'CLI output discipline'" and "never truncated" retired — APPLY (routine; the P5-approved entry names the change).
2. (raised by main, check 5 + check 6 [width]) §Surface: cli Width (design-system.md:420) — the width read by `viola_pty::host_size()`, not `COLUMNS` — APPLY, marked PROVISIONAL (`inputs#I1` Q2, the overseer's technical answer, a delegate's).
3. (raised by main, check 5) §Navigation Pattern (design-system.md:526) — `viola --help` groups as built: board / traffic / wheel / setup; `handoff:` and `plugin install` arrive with "Explicit session links" and "Optional plugin install" — APPLY, PROVISIONAL (`inputs#I1` Q3).
4. (raised by main, check 5) §Tokens (platform-specific) — `NO_COLOR` set read as present, an empty value included (:404); on a stale row NAME is dim and bold (:394) — ESCALATE → answer card for the `NO_COLOR` read clause (inside `inputs#I2`); the stale-NAME composition APPLY.

## layout-templates: 5 proposals · fanout-layout-templates.md
note: two primaries; :23, :29, :42, :71, :79, :89 no-change.
1. §Output structure — `viola --help` (the As built note under the wireframe) — APPLY (check 1: Accurate this-chunk addition; check 5 entry), PROVISIONAL rows per `inputs#I1` Q3.
2. §Primary screens (:59, dependent of 1) — APPLY.
3. §Component — Primary navigation, Discoverability (dependent of 1) — APPLY.
4. §Output structure — `viola list` As built (:91) — APPLY (width source PROVISIONAL, `inputs#I1` Q2).
5. §Component — Primary content block 1 (:298, dependent of 4) — APPLY.

## test-plan: 7 proposals · fanout-test-plan.md
note: D-tests-obs-harness no drift; the pre-existing [trycmd] sites (:383, :403, :586 tail, :666, :850, :1211, the `viola-send` half of :585) left to Validate.
1. §5 CLI bullet (:585) — `viola-help` pinned by the `HELP` literal, no trycmd case — APPLY (check 5 entry; check 6 [trycmd] for this site).
2. §5 Human-mode layouts (:542, dependent of 1) — APPLY.
3. §6 surface table cli row tool cell (:614, dependent of 1) — APPLY.
4. §7 Human-mode expected output row (:931, dependent of 1) — APPLY.
5. §5 CLI bullet, `tests/cli_output_plain.rs` (:589) — the grouped-help and served-verbs cases, the Unix CR reading, the parser in `tests/support/sgr.rs` — APPLY (check 1: Accurate this-chunk addition).
6. §6 surface table cli row verdict cell — dim `stale` proven at the unit tier only, its outer-PTY witness owed to the owner P5 names — ESCALATE (the detector's severity) → the placement is asked in this halt (question 2); applied naming the owner the operator gives.
7. §5 CLI bullet, `tests/cli_controls_not_disableable.rs` — the three colour-variable cases and the helper's removal of inherited values — APPLY (check 1: Accurate this-chunk addition).
8. (raised by main, check 6) the remaining [trycmd] sites in test-plan — ESCALATE (question 3).

## obs-plan: 2 proposals · fanout-obs-plan.md
note: D-obs-instrumentation / D-obs-pii no drift; §11's CI ban "The product never reads env as configuration", §1 and the service-identity key read env absolutely in a logging / identity context — left to the cascade sweep (`env-exceptions`).
1. §3 intro (:505) — CARRY 1: `refuse` → `write_refusal` called by run 5 · verify 5 · list 2 · revive 1; the colour layer adds no stderr writer — APPLY (check 5 entry).
2. §11 Logs (:1071, dependent of 1) — APPLY.

## a11y-plan: 6 proposals · fanout-a11y-plan.md
note: D-a11y-obs-schema no drift; the other [trycmd] hits left to Validate; §1 :69 left (target set, owned by "Explicit session links").
1. §6 Visual Design Verification — CLI equivalent (:258) — APPLY for the built spans, cut and checks; its clause recording that `viola list` reads `NO_COLOR` / `TERM` / `COLORTERM` → answer card (inside `inputs#I2`); the integration-tier dim witness's owner named after question 2.
2. §4 P6 (:234, dependent of 1) — APPLY, the owner named after question 2.
3. §1 cli Notes (:73, dependent of 1) — APPLY.
4. §4 landmark inventory cli row (:209, dependent of 1) — APPLY, PROVISIONAL rows (`inputs#I1` Q3).
5. §1 Critical path 6 (:134, dependent of 1) — APPLY.
6. §8 Plain language, Verification (:303, dependent of 1) — APPLY (the help's literal pin).
7. (raised by main, check 6) the remaining [trycmd] sites in a11y-plan and its key files — ESCALATE (question 3).

## Escalations resolved (the P2 halt, `inputs#I5`)
- Answer card (`answer-card.md`): "Confirm (Recommended)" — architecture 1–4, security-plan 1–3, design-system 4 and the
  reads clause of a11y-plan 1 APPLIED; ratification the founder's own answer (`inputs#I2`); the confirm a delegate's (the Viola
  overseer), PROVISIONAL, so written in each owning sidecar.
- Placement: "CARRY: Linux and macOS parity (Recommended)" — test-plan 6 APPLIED naming that entry; a11y-plan 1 and 2 name it
  too; P5 pins the CARRY.
- [trycmd] family: "CARRY: Version done-check (Recommended)" — architecture 7, test-plan 8 and a11y-plan 7 dispositioned by a
  CARRY on that entry (P5); no body edit.
Open escalations: 0. Applied: architecture 7 (5 + 2 raised; 1 of the raised as a CARRY), security-plan 3, design-system 4,
layout-templates 5, test-plan 7 (+1 raised, a CARRY), obs-plan 2, a11y-plan 6 (+1 raised, a CARRY). Rejected: 0.

## Checks 2–6
- Check 2 (cross-contradiction): none — test 3 and test 6 edit two different cells of the §6 cli row.
- Check 3 (intent-consistency): the report's seven deviations are justified in-intent; scope record none (`gate.py scope` clean). No divergence.
- Check 4 (absence needs evidence): the report's [win-mutants] "no master lists a label's files" rests on its pattern rows (0 `viola-main` master hits), not a view; [trycmd] "no manifest holds trycmd" rests on research.md's sweep (0 hits over the manifests).
- Check 5 (expected amendments): every entry of the plan's list matched by a proposal or raised above (arch 1–4, security 1–3, design 1–4, layout 1–5, a11y 1–6, test 1–5, obs 1–2).
- Check 6 (disproved claims): [width] → design 2; [trycmd] → test 1–4 and a11y 6 for the `viola-help` sites, the rest of the family → question 3.
