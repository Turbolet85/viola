# Fan-out results — 2026-11-14-hyprland-seam (wrap 2026-11-14T09-29-36)

architecture: add 14 · set 9 · retire 0 · none 0 · ops-architecture.md (re-spawned once, EVERY_TASK_FACT; the first file kept as .raw-ops-architecture.md)
security-plan: add 5 · set 4 · retire 0 · none 2 · ops-security-plan.md
design-system: add 0 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: none · ops-layout-templates.md — reason: no viola verb, cli wording or user-facing surface changed (fct-woso52s6, fct-wnem37fs, fct-q6e6jeog, fct-y4ojdqlq)
test-plan: add 1 · set 16 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 6 · set 10 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 0 · set 2 · retire 0 · none 0 · ops-a11y-plan.md
main: ops-main-security-plan.md — add 1 (the plan's expected `+ decision/hyprland-recorder-string-allowlist`, the wrap directive's item 3 handle)

## Facts offered more than once (check 2 — one home each)
- the hyprctl child deadline: architecture `decision/hyprctl-deadline` · security-plan `decision/hyprctl-read-deadline` · obs-plan `decision/hyprctl-deadline` → home security-plan `decision/hyprctl-read-deadline`
- reply text dropped at the parse: architecture `decision/no-hyprland-reply-text` · security-plan `decision/hyprland-reply-text-dropped` · obs-plan `decision/compositor-reply-text-never-kept` → home security-plan `decision/hyprland-reply-text-dropped`
- the recorder's closed keep-list: architecture `decision/hyprland-fixture-keep-list` · security-plan `decision/hyprland-fixtures-closed-keep-list` · obs-plan `decision/hyprland-fixtures-keep-list` · test-plan `decision/hyprland-replies-keep-list` → home security-plan `decision/hyprland-recorder-string-allowlist` (main's, the plan's handle)
- hyprctl's direct spawn, its caller-resolved path and the closed Lua dispatch text: architecture `decision/hyprctl-direct-spawn` · security-plan `decision/hyprctl-caller-resolved-program` · security-plan `decision/hyprctl-dispatch-closed-values` → home architecture `decision/hyprctl-direct-spawn`
- the seam's closed error words: architecture `contract/hyprland-seam-errors/*` (5) · obs-plan `decision/hyprland-seam-closed-errors` → home architecture `contract/hyprland-seam-errors/*`
- the reply under the frame cap: architecture `decision/hyprctl-reply-under-frame-cap` · security-plan `decision/max-frame-shared-cap` (standing, «child output») → home security-plan `decision/max-frame-shared-cap`, standing true
- the fake as hyprctl by its file name: architecture `decision/fake-hyprctl-by-file-name` · test-plan `decision/fake-agent-on-test-path` (set) → home test-plan `decision/fake-agent-on-test-path`

## Expected amendments (check 5)
- architecture `+ name/crate/viola-hyprland` · `+ name/binary/viola-hypr-tap` · `+ decision/hyprland-seam-crate` · `+ decision/hyprctl-direct-spawn` — proposed (ops-architecture.md)
- security-plan `~ decision/max-frame-shared-cap` — not carried: the row's «child output» already holds the hyprctl reply (security row_none F2); a set would lengthen a true row
- security-plan `+ decision/hyprctl-resolved-from-path` — superseded: its fact (direct spawn from a caller-resolved path, no PATH lookup) stands once, in architecture `decision/hyprctl-direct-spawn` (check 2); viola reads no new env var, so no row moves
- security-plan `+ decision/hyprland-recorder-string-allowlist` — raised by main (ops-main-security-plan.md)
- obs-plan `~ contract/diag-line-fields/process-start` · `~ contract/diag-line-fields/parse-rejected` · `+ contract/diag-subject/hyprctl-probe` · `~ decision/viola-crates-log-at-info` · `~ decision/os-apis-at-call-sites` · `~ decision/instrument-call-sites-not-os-internals` — proposed (ops-obs-plan.md)
- test-plan `~ decision/fixtures-recorded-by-live-verify` · `~ decision/fixture-hygiene-walk` · `~ decision/fake-agent-on-test-path` · `~ decision/property-tests-on-parsers` — proposed (ops-test-plan.md)
- a11y-plan `~ decision/time-bounds-never-on-human` — proposed (ops-a11y-plan.md)

## Spec claims disproved (check 6)
- the plan's step 2 dispatch argv, step 5 `/bin/sleep` witness, step 14 case form — no master site (report, Spec claims disproved); routed to curation (Decisions & corrections) and P5 route-resolve; DISPOSED.

## Dispositions

ops-architecture.md
decision/nutype-domain-newtypes · reject — check 7: not_master type 5 — an inventory the code holds; the row stays true
decision/nutype-newtypes · reject — check 7: not_master type 5 — an inventory the code holds; the row stays true
decision/tolerant-external-payloads · reject — check 7: not_master type 5 — one more payload in a true list
decision/unparseable-external-values-unknown · apply — check 1: the seam's malformed error is the named exception
decision/fake-agent-feature-on-named-members · reject — check 7: not_master type 5 — an inventory the manifests hold
decision/test-binaries-never-released · reject — check 7: not_master type 5 — an inventory the manifests and the name rows hold
decision/sync-crates-tokio-free · reject — check 7: not_master type 5 — the sync-crate list in the code holds it
decision/root-bin-consumes-normalised-events · reject — check 7: not_master type 4 — a manifest detail; the standing row is false, so main's shorter set (ops-main-architecture.md) follows
decision/modular-monolith-no-daemon · reject — check 7: not_master type 5 — an inventory the workspace manifest holds
name/crate/viola-hyprland · apply — check 5: the plan's expected entry, P5-approved (inputs#I4)
name/binary/viola-hypr-tap · apply — check 5: the plan's expected entry, P5-approved (inputs#I4)
decision/hyprland-seam-crate · apply — check 5: the plan's expected entry; placement the overseer's technical answer (inputs#I2 Q3), no widening
decision/hyprctl-direct-spawn · escalate → Q1 — check 1: boundary widening (rule «Boundary widening the founder answered himself to a P4 fork», rul-bznresq5); the founder's own answer to P4 fork Q1 (inputs#I2): the answer card · resolved: apply (Q1) — relayed verbatim by overseer1, 2026-11-14, inputs#I10: «Подтвердить обе (Recommended)»
decision/hyprctl-reply-under-frame-cap · reject — check 2: the fact's home is security-plan decision/max-frame-shared-cap
decision/hyprctl-deadline · reject — check 2: the fact's home is security-plan decision/hyprctl-read-deadline
decision/no-hyprland-reply-text · reject — check 2: the fact's home is security-plan decision/hyprland-reply-text-dropped
contract/hyprland-seam-errors/unsupported · escalate → Q3 — check 1: a NEW keyed contract boundary (Hyprland seam errors) escalates (inputs#I7 item 6); a technical fork, the overseer's by relay · resolved: apply (Q3) — delegate operator, relayed by the overseer, 2026-11-14, inputs#I9: option 1, the five land as the proposer wrote them (area=standard-contracts, a new boundary; a technical placement, no widening)
contract/hyprland-seam-errors/unreachable · escalate → Q3 — check 1: the same new boundary (inputs#I7 item 6) · resolved: apply (Q3) — delegate operator, relayed by the overseer, 2026-11-14, inputs#I9: option 1, the five land as the proposer wrote them (area=standard-contracts, a new boundary; a technical placement, no widening)
contract/hyprland-seam-errors/oversize · escalate → Q3 — check 1: the same new boundary (inputs#I7 item 6) · resolved: apply (Q3) — delegate operator, relayed by the overseer, 2026-11-14, inputs#I9: option 1, the five land as the proposer wrote them (area=standard-contracts, a new boundary; a technical placement, no widening)
contract/hyprland-seam-errors/malformed · escalate → Q3 — check 1: the same new boundary (inputs#I7 item 6) · resolved: apply (Q3) — delegate operator, relayed by the overseer, 2026-11-14, inputs#I9: option 1, the five land as the proposer wrote them (area=standard-contracts, a new boundary; a technical placement, no widening)
contract/hyprland-seam-errors/refused · escalate → Q3 — check 1: the same new boundary (inputs#I7 item 6) · resolved: apply (Q3) — delegate operator, relayed by the overseer, 2026-11-14, inputs#I9: option 1, the five land as the proposer wrote them (area=standard-contracts, a new boundary; a technical placement, no widening)
decision/hyprland-fixture-keep-list · reject — check 2: the fact's home is security-plan decision/hyprland-recorder-string-allowlist
decision/fake-hyprctl-by-file-name · reject — check 2: the fact's home is test-plan decision/fake-agent-on-test-path

ops-security-plan.md
decision/child-output-tolerant · apply — check 1: the seam's closed failure words (Symbols)
decision/child-output-tolerant-unknown · apply — check 1: the same fact on its twin row
decision/shared-validators-in-core · apply — check 1: Address lives in the seam crate (Symbols)
decision/parser-fuzz-scope · apply — check 1: the hyprctl parsers under fuzz and proptest (Crates / modules)
decision/hyprctl-caller-resolved-program · reject — check 2: the fact's home is architecture decision/hyprctl-direct-spawn
decision/hyprctl-read-deadline · apply — check 1: the deadline kill (Symbols, read.rs), PROVISIONAL as the plan states; the card's Beyond-the-plan row settles it at «viola open»
decision/hyprctl-dispatch-closed-values · reject — check 2: the fact's home is architecture decision/hyprctl-direct-spawn
decision/hyprland-fixtures-closed-keep-list · reject — check 2: the fact's home is security-plan decision/hyprland-recorder-string-allowlist (main's, the plan's handle, inputs#I7 item 3)
decision/hyprland-reply-text-dropped · apply — check 1: no text field, the canary contract (Symbols; Coverage PII)

ops-test-plan.md
decision/fixtures-recorded-by-live-verify · apply — check 5: expected entry
decision/fixture-hygiene-walk · apply — check 5: expected entry
decision/fixtures-walked-for-hygiene · apply — check 1: the Hyprland class joins the walk
decision/fake-agent-on-test-path · apply — check 5: expected entry
decision/fake-agent-argv-only · apply — check 1: the hyprctl personality's mode file is the named exception
contract/fake-agent-cli/argv-only-options · apply — check 1: the same exception on the contract row
decision/fake-agent-as-claude · apply — check 1: hyprctl a second external child served by the fake
decision/fakes-for-external-services · apply — check 1: the real hyprctl replaced in tests
decision/ci-never-real-claude · apply — check 1: no CI image carries hyprctl (Dev-tool versions)
decision/bounded-readers-keep-the-cap · apply — check 1: the hyprctl reader's BOUNDARIES row
decision/property-tests-on-parsers · apply — check 5: expected entry
decision/parser-property-fuzz-coverage · apply — check 1: hyprctl_json in the fuzz replay
decision/property-tests-with-proptest · apply — check 1: the Hyprland suite's committed regressions
decision/no-production-data-in-tests · apply — check 1: a test-only recorder's capture is the second source
decision/recorder-scrub · apply — check 1: viola-hypr-tap runs the shared scrub
decision/dirty-payload-refuses-recording · apply — check 1: viola-hypr-tap refuses a dirty set whole
decision/hyprland-replies-keep-list · reject — check 2: the fact's home is security-plan decision/hyprland-recorder-string-allowlist

ops-obs-plan.md
contract/diag-line-fields/process-start · apply — check 5: expected entry (subject 7 → 8)
contract/diag-line-fields/parse-rejected · apply — check 5: expected entry (parser 10 → 11)
contract/parse-rejected-detail/oversize · apply — check 1: hyprctl-json writes oversize
decision/per-parser-parse-failure-counts · apply — check 1: the hyprctl parsers count under their own word
decision/viola-crates-log-at-info · reject — check 7: not_master type 6 — a retelling of configuration the filter's code holds; the expected entry ends here, the row's per-module rule stays true
decision/os-apis-at-call-sites · apply — check 5: expected entry
decision/instrument-call-sites-not-os-internals · apply — check 5: expected entry
decision/parse-failures-unknown-counter · apply — check 1: the seam's closed parse failure is the named exception
decision/parse-failures-surface-as-unknown · apply — check 1: the same exception on its twin row
decision/span-naming · apply — check 1: the span hyprland.read adds the area hyprland
contract/diag-subject/hyprctl-probe · apply — check 5: expected entry
contract/parse-rejected/hyprctl-json · apply — check 1: the new parser word's contract, sibling of claude-agents-json
decision/compositor-reply-text-never-kept · reject — check 2: the fact's home is security-plan decision/hyprland-reply-text-dropped
decision/hyprland-fixtures-keep-list · reject — check 2: the fact's home is security-plan decision/hyprland-recorder-string-allowlist
decision/hyprland-seam-closed-errors · reject — check 2: the fact's home is architecture contract/hyprland-seam-errors/*
decision/hyprctl-deadline · reject — check 2: the fact's home is security-plan decision/hyprctl-read-deadline

ops-a11y-plan.md
decision/time-bounds-never-on-human · reject — check 7: not_master type 5 — the rule already binds every bound; the expected entry ends here
decision/time-bounds-fall-on-the-driver · reject — check 7: not_master type 5 — the rule already covers every bound

ops-main-security-plan.md
decision/hyprland-recorder-string-allowlist · escalate → Q2 — check 1: boundary widening (rul-bznresq5); the founder's own answer to P4 fork Q2 (inputs#I2), narrowed by the P5 review to the closed keep-list (inputs#I3 item 1): the answer card · resolved: apply (Q2) — relayed verbatim by overseer1, 2026-11-14, inputs#I10: «Подтвердить обе (Recommended)»

ops-main-architecture.md
decision/root-bin-consumes-normalised-events · apply — check 7: the standing row false, fit's not_master on the longer set leaves the shorter set owed
