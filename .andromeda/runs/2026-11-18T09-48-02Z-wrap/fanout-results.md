# Fan-out results — 2026-11-18-trust-record-row-read-live (wrap run 2026-11-18T09-48-02Z-wrap)

architecture: add 3 · set 9 · retire 0 · none 1 · ops-architecture.md
security-plan: add 2 · set 3 · retire 0 · none 0 · ops-security-plan.md
design-system: add 1 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 2 · set 1 · retire 1 · none 1 · ops-test-plan.md
obs-plan: add 0 · set 2 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 0 · set 2 · retire 0 · none 0 · ops-a11y-plan.md
security-plan (main): add 1 · ops-main-security-plan.md

## Dispositions

ops-architecture.md
contract/ledger-stamps/measured · reject — check 2: the fact's home is architecture contract/ledger-stamps/first-screen, contract/ledger-stamps/first-settle-ms and contract/ledger-stamps/first-facts (one row an item, added in this file); the measured row stays true
decision/verify-record-only-clean · escalate → Q1 — check 1: boundary widening (a failed verify now writes a first-screen record of whole scrubbed rows), the founder's own answers inputs#I11 item 1 and inputs#I13 cover it — the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/recorded-fixtures-scrubbed · escalate → Q1 — check 1: boundary widening, the same first-screen record; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/screen-fixtures-signature-rows-only · escalate → Q1 — check 1: boundary widening, the same first-screen record; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
contract/viola-cli-arguments/verify · escalate → Q1 — check 1: boundary widening, the hidden option that carries the same first-screen record; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/run-never-reads-screen-content · escalate → Q1 — check 1: boundary widening, row text now also leaves through the first-screen record; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/claude-first-no-adapter-trait · apply — check 1: routine (accurate this-chunk addition: the rig's private-config seed of bundle keys, report Symbols / APIs rig isolate.rs; technical per inputs#I20 item 3)
decision/claude-shapes-in-one-crate · apply — check 1: routine (accurate this-chunk addition, the same seed)
decision/claude-shapes-isolated · apply — check 1: routine (accurate this-chunk addition, the same seed)
contract/ledger-stamps/first-screen · apply — check 1: routine (accurate this-chunk addition; the plan's expected amendment to the measured block)
contract/ledger-stamps/first-settle-ms · apply — check 1: routine (accurate this-chunk addition; the plan's expected amendment)
contract/ledger-stamps/first-facts · reject — check 7: not_master type 4 — the basis retells how the first-facts reading is built, no promise or reason stated; the code holds it

ops-security-plan.md
decision/screen-fixture-signature-rows-only · escalate → Q1 — check 1: boundary widening, the first-screen record of whole scrubbed rows; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/verify-signature-only · escalate → Q1 — check 1: boundary widening, the plan's expected except; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/verify-screen-read-against-literals · escalate → Q1 — check 1: boundary widening, the plan's expected except; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/verify-stamp-first-screen-closed-words · apply — check 1: routine (accurate this-chunk addition: the stamp's measured first-screen words, walls and fact rows hold no screen text)
decision/live-rig-seed-from-bundle-logic · apply — check 1: routine (the plan's expected amendment to the rig's isolation, Revision 4; technical per inputs#I20 item 3)

ops-main-security-plan.md
decision/live-rig-git-root-as-files · reject — check 7: not_master type 4 — how the rig builds its child's git root as plain files is one mechanism, no reason stated; the rig's code holds it

ops-design-system.md
decision/verify-hidden-first-screen-record · reject — check 2: the fact's home is security-plan decision/verify-signature-only (its except) and architecture contract/viola-cli-arguments/verify

ops-layout-templates.md
contract/viola-verify-output/record-first-screens · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/verify

ops-test-plan.md
decision/record-only-at-zero-failing-rows · escalate → Q1 — check 1: boundary widening, the first-screen record written on a failed verify; the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-18, inputs#I22: «Confirm» — inside the founder's own answers inputs#I11 item 1 and inputs#I13
decision/verify-window-60s-kill · apply — check 1: routine (the class kill moved 60 s → 80 s, report Counts / qualifiers moved; the key is made from the value, so retired with its heir)
decision/verify-window-80s-kill · apply — check 1: routine (the heir of the retired 60 s row)
decision/instrumented-probe-runs-in-scratch-cwd · reject — check 2: the fact's home is curation (a fix's lesson, the wrap directive inputs#I20 item 5 names it a curation candidate), never a master row

ops-obs-plan.md
decision/fixtures-not-telemetry · reject — check 2: the row stays true (the first-screen record is fixture-side data); the fact's home is security-plan decision/verify-signature-only
decision/ledger-contents-never-logged · reject — check 2: the row stays true; the fact's home is security-plan decision/verify-stamp-first-screen-closed-words

ops-a11y-plan.md
decision/no-screen-content-parsing · reject — check 2: the row stays true (a whole-row copy is no parse) and the set drops its standing «raw frames kept local and gitignored»; the fact's home is security-plan decision/verify-signature-only
contract/cli-help/served-verbs-only · reject — check 2: the row stays true (a hidden option is not listed); the fact's home is architecture contract/viola-cli-arguments/verify

## Check 5 — the plan's expected amendments
- architecture `contract/ledger-stamps/measured` add-to: carried by the three item adds (ops-architecture.md).
- test-plan `decision/verify-window-60s-kill`: carried by the retire and its 80 s heir (ops-test-plan.md).
- test-plan `decision/standing-live-rig` add-to: not carried as a row — what it lists (the V arm's summary lines, the VC arm, the labels, `kept:`) is inventory (master-rows type 5); the git root and the seed are carried in security-plan (main's add and the proposer's seed add), the local first-screen copy by the Q1 rows.
- security-plan `decision/live-rig-isolated-child` add-to: carried by decision/live-rig-seed-from-bundle-logic and main's decision/live-rig-git-root-as-files.
- security-plan `decision/verify-signature-only`, `decision/verify-screen-read-against-literals`: carried, Q1. `decision/fixture-scrub-and-refusal`: not carried — the row stays true: the first-screen record scrubs before any write and refuses the whole set by the same rule.

## Check 6 — disproved claims
- The route CARRY «ONE common cause, not trust» — confirmed, not disproved: the route's (P5).
- Revision 1's first-run-config cause (inputs#I4) — measured false by S0R-2: no master site (the sweep's `first-run-cfg` control unfired); routed to the route (F2, no carry — P5).
- Step 15's fake-oracle premise — a plan acceptance, absorbed by the Revision 2 phase run: no master site.
