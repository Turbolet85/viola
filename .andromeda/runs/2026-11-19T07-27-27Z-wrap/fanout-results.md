# Fan-out results — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

architecture: add 7 · set 4 · retire 0 · none 2 · ops-architecture.md
security-plan: add 1 · set 4 · retire 0 · none 0 · ops-security-plan.md
design-system: add 2 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 1 · set 1 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 1 · set 2 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 0 · retire 0 · none 0 · ops-a11y-plan.md
security-plan (main): set 1 · ops-main-security-plan.md

## Dispositions

ops-architecture.md
decision/claude-first-no-adapter-trait · escalate → Q1 — check 1: a boundary widening (the VH arm reads the real Claude config's key names), the founder's own answer «Разрешить ветку VH» covers it · resolved: apply (Q1) — delegate the overseer, 2026-11-19: «land it, PROVISIONAL — with ONE change» (Q3, after Q1 «Name rows outside» and Q2 «Shorten, keep every name») · as: except reads «Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.»
decision/claude-shapes-in-one-crate · escalate → Q1 — check 1: a boundary widening (the VH arm reads the real Claude config's key names), the founder's own answer «Разрешить ветку VH» covers it · resolved: apply (Q1) — delegate the overseer, 2026-11-19: «land it, PROVISIONAL — with ONE change» (Q3, after Q1 «Name rows outside» and Q2 «Shorten, keep every name») · as: except reads «Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.»
decision/claude-shapes-isolated · escalate → Q1 — check 1: a boundary widening (the VH arm reads the real Claude config's key names), the founder's own answer «Разрешить ветку VH» covers it · resolved: apply (Q1) — delegate the overseer, 2026-11-19: «land it, PROVISIONAL — with ONE change» (Q3, after Q1 «Name rows outside» and Q2 «Shorten, keep every name») · as: except reads «Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.»
decision/test-isolated-viola-home · reject — check 2: the VH start is a live start by hand, no test, so the row stays true; the exception's one home is security-plan decision/live-rig-isolated-child
name/other/viola-live-rig-vh · reject — check 1: an arm word of a test-only binary is no occupied resource (the earlier arms vr, mt, ts stand as no name row); the binary stands as name/binary/viola-live-rig
name/other/viola-live-rig-cx · reject — check 1: an arm word of a test-only binary is no occupied resource (the earlier arms stand as no name row); the binary stands as name/binary/viola-live-rig
name/other/viola-live-rig-rehearse-target-home · reject — check 1: a flag of a test-only binary's subcommand is no occupied resource; the binary stands as name/binary/viola-live-rig
name/other/viola-live-rig-rehearse-claude-dir · reject — check 1: a flag of a test-only binary's subcommand is no occupied resource; the binary stands as name/binary/viola-live-rig
contract/viola-live-rig-rehearse/vh-without-explicit-home · reject — check 2: the fact's home is security-plan decision/live-rig-isolated-child (its except names the explicit home, Claude dir and user config)
decision/live-rig-real-home-arm · reject — check 2: the fact's home is security-plan decision/live-rig-isolated-child
contract/viola-verify-output/browser-tools-line · reject — check 2: the fact's home is design-system contract/viola-verify-output/browser-tools-line

ops-security-plan.md
decision/live-rig-isolated-child · escalate → Q1 — check 1: a boundary widening (rul-r74hshif, rul-bznresq5: the VH arm runs under the real home and Claude config), the founder's own answer «Разрешить ветку VH» covers it · resolved: apply (Q1) — delegate the overseer, 2026-11-19: «the two security-plan rows (live-rig-isolated-child, live-rig-credentials-copy) are CONFIRMED as shown, PROVISIONAL until the founder's own word at the epoch boundary»
decision/tests-use-own-home · reject — check 2: the VH start is a live start by hand, no test, so the row stays true; the exception's one home is security-plan decision/live-rig-isolated-child
decision/verify-echo-probe-residuals · reject — check 7: not_master type 8 — the qualifier records one CLI version's reading, history the sidecar and the route's carry hold; the row's «stays unmeasured» stays true in general, so no correction is owed
decision/verify-never-answers-dialogs · apply — check 5: the verify line in the founder's words (plan entry design-system §Surface: cli, his answer 4) names who answers the browser-tools prompt
decision/live-rig-context-arm-keeps-no-transcript · apply — check 1: no rule; the CX arm under the rig's private config («Под конфигом стенда»), a new bounded count-only transcript reader, no widening

ops-design-system.md
contract/viola-verify-output/browser-tools-line · apply — check 5: the plan's entry design-system §Surface: cli, the line in the founder's words; the one home of the line among the six offers
decision/browser-tools-prompt-never-answered · reject — check 2: the fact's home is security-plan decision/verify-never-answers-dialogs

ops-layout-templates.md
contract/viola-verify-output/browser-tools-line · reject — check 2: the fact's home is design-system contract/viola-verify-output/browser-tools-line

ops-test-plan.md
decision/tests-never-read-host-sessions · reject — check 2: the VH start is a live start by hand, no test's own list child, so the row stays true; the exception's one home is security-plan decision/live-rig-isolated-child
contract/verify-output/browser-tools-line · reject — check 2: the fact's home is design-system contract/viola-verify-output/browser-tools-line

ops-obs-plan.md
decision/browser-tools-prompt-reading-uninstrumented · apply — check 5: the plan's entry obs-plan §6, verify's stdout gains the line with no log line
decision/test-homes-under-e2e-home · reject — check 2: the VH start is a live start by hand, no integration-test home, so the row stays true; the exception's one home is security-plan decision/live-rig-isolated-child
contract/verify-stdout/browser-tools-line · reject — check 2: the fact's home is design-system contract/viola-verify-output/browser-tools-line

ops-a11y-plan.md
contract/viola-verify/browser-tools-line · reject — check 2: the fact's home is design-system contract/viola-verify-output/browser-tools-line

ops-main-security-plan.md
decision/live-rig-credentials-copy · escalate → Q1 — check 5: the plan's entry security-plan §Secret Management (decision/live-rig-credentials-copy) no proposer raised; a boundary widening inside the founder's own answer «Разрешить ветку VH» · resolved: apply (Q1) — delegate the overseer, 2026-11-19: «the two security-plan rows (live-rig-isolated-child, live-rig-credentials-copy) are CONFIRMED as shown, PROVISIONAL until the founder's own word at the epoch boundary»

## Expected amendments (check 5)
- security-plan §Secret Management, decision/live-rig-isolated-child and decision/live-rig-credentials-copy: raised (Q1, the answer card).
- design-system §Surface: cli, the browser-tools line: carried, contract/viola-verify-output/browser-tools-line.
- layout-templates §Surface: cli, the wireframe gains the line: not carried as a row — check 2, the line's one home is design-system.
- test-plan §5, the line before the summary: not carried as a row — check 2, home design-system; the labels L1 / L1-2 / L2 and arms vh / cx: not carried — the labels are this chunk's ledger labels and the arms no occupied resource (architecture's name rejects above).
- obs-plan §6: carried, decision/browser-tools-prompt-reading-uninstrumented.
- architecture §Occupied Resources, the labels and arms: not carried (above); the /mcp disable reading: no master row (fit not_master, history) — routed to P5's carry; the /context reading: no master fact (its tag half unread) — routed to P5's carry.

## Spec claims disproved (check 6)
- The /context reading «from a readable transcript»: DISPOSED — routed to P5 route-resolve, the /context carry's next reading; no master row states it.
- L2's send not delivered: DISPOSED — routed with the /context carry at P5.
- The /mcp disable reading (After implement): DISPOSED — fit read the row's set not_master (one version's reading is history); routed to P5, the carry «why no new projects entry appeared», where the reading is recorded with its next reader.
