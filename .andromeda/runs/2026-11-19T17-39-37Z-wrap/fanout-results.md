# Fan-out results — 2026-11-19-a-live-stopfailure-payload (wrap 2026-11-19T17-39-37Z)

architecture: add 2 · set 6 · retire 0 · none 1 · ops-architecture.md
security-plan: add 2 · set 2 · retire 0 · none 2 · ops-security-plan.md
design-system: none · ops-design-system.md — the chunk is test-only; no CLI output wording, readback state or token changed
layout-templates: no proposer
test-plan: add 4 · set 6 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 1 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 0 · set 1 · retire 0 · none 0 · ops-a11y-plan.md
Main's own: ops-main-architecture.md (set 3) · ops-main-security-plan.md (set 2) · ops-main-test-plan.md (set 1) · ops-main-architecture.2.md (set 3) · ops-main-security-plan.2.md (set 1).

ops-architecture.md
decision/capability-ledger · reject — check 5: the plan's entry cites the live reading by version, date, start and word; the value omits the start — raised as main's own in ops-main-architecture.md
decision/cli-capability-ledger · reject — check 5: the plan's entry cites the live reading by version, date, start and word; the value omits the start — raised as main's own in ops-main-architecture.md
decision/capability-ledger-gate · reject — check 5: the value names no version, start or word of the reading — raised as main's own in ops-main-architecture.md
decision/claude-first-no-adapter-trait · escalate → Q1 — check 1: a boundary widening (rul-p2zl2kgl), the rig's StopFailure capture reading a payload's error word; the founder's own F1 answer covers it · resolved: reject (Q1) — the overseer's wording condition (inputs#I7): the name readings keep the word «name»; raised as main's own in ops-main-architecture.2.md
decision/claude-shapes-in-one-crate · escalate → Q1 — check 1: a boundary widening (rul-p2zl2kgl), the plan's expected amendment; the founder's own F1 answer covers it · resolved: reject (Q1) — the overseer's wording condition (inputs#I7); raised as main's own in ops-main-architecture.2.md
decision/claude-shapes-isolated · escalate → Q1 — check 1: a boundary widening (rul-p2zl2kgl); the founder's own F1 answer covers it · resolved: reject (Q1) — the overseer's wording condition (inputs#I7); raised as main's own in ops-main-architecture.2.md
decision/bwrap-test-only-rig-dependency · reject — check 7: not_master type 5 — a dev-tool reading that states no choice or reason; the code and the sidecar hold it
decision/live-rig-contained-arm-sandbox · reject — check 2: the fact's home is security-plan decision/live-rig-sf-arm-network-closed and decision/live-rig-sandbox-runtime-dir

ops-security-plan.md
decision/live-rig-prompt-capture-closed · reject — check 5: the value drops the standing clause that a non-object or over-long payload writes unread — raised as main's own in ops-main-security-plan.md
decision/unverified-cli-gate · reject — check 5: the value omits the reading's start — raised as main's own in ops-main-security-plan.md
decision/live-rig-sf-arm-network-closed · apply — check 5: the plan's expected amendment (security-plan, SF's network-closed bwrap sandbox and its refusal before the ledger line), held in a row of its own beside decision/live-rig-isolated-child; technical by inputs#I6 item 3
decision/live-rig-sandbox-runtime-dir · reject — check 7: not_master type 4 — a precondition of one probe function with no ratified reason; the code holds it

ops-test-plan.md
decision/hook-payloads-recorded-by-verify · reject — check 5: the value names no version or start of the reading — raised as main's own in ops-main-test-plan.md
decision/standing-live-rig · apply — check 5: the plan's expected amendment (the SF arm, CI unwrapped, the wrapped local entry)
decision/no-child-screen-assertions · escalate → Q2 — check 1: the rig's screen-reading exception gains the box row's cell attributes; the founder's own Q2 and F2 answers cover it · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-19, inputs#I3: «Сложить в SF (Recommended)»; inputs#I4: «После конца хода (Recommended)»; the card confirmed by the overseer, inputs#I7
decision/never-parse-rendered-child-screen · escalate → Q2 — check 1: the same reading; the founder's own Q2 and F2 answers cover it · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-19, inputs#I3: «Сложить в SF (Recommended)»; inputs#I4: «После конца хода (Recommended)»; the card confirmed by the overseer, inputs#I7
decision/no-screen-parsing · escalate → Q2 — check 1: the same reading; the founder's own Q2 and F2 answers cover it · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-19, inputs#I3: «Сложить в SF (Recommended)»; inputs#I4: «После конца хода (Recommended)»; the card confirmed by the overseer, inputs#I7
decision/discipline-at-every-layer · escalate → Q2 — check 1: the same reading; the founder's own Q2 and F2 answers cover it · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-19, inputs#I3: «Сложить в SF (Recommended)»; inputs#I4: «После конца хода (Recommended)»; the card confirmed by the overseer, inputs#I7
decision/live-rig-sf-arm-sandboxed · reject — check 2: the fact's home is security-plan decision/live-rig-sf-arm-network-closed
decision/live-rig-stop-failure-capture-closed · reject — check 2: the fact's home is security-plan decision/live-rig-prompt-capture-closed
contract/fake-agent-cli/turn-fail · apply — check 5: the plan's expected amendment (test-plan §7, the fake agent's --turn-fail beside --turn-stop)
contract/fake-agent-cli/settings-hooks · apply — check 3: a justified divergence (implement's Deviations 1, the capture hook could not fire under the fake otherwise), argv-only, fake-agent builds only; technical by inputs#I6 item 3

ops-obs-plan.md
decision/live-rig-stopfailure-capture-not-telemetry · reject — check 2: the fact's home is security-plan decision/live-rig-prompt-capture-closed

ops-a11y-plan.md
decision/no-screen-content-parsing · escalate → Q2 — check 1: the plan's expected amendment, the rig's screen-reading exception gaining the box row's cell attributes, never text, after the turn end; the founder's own Q2 and F2 answers cover it · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-19, inputs#I3: «Сложить в SF (Recommended)»; inputs#I4: «После конца хода (Recommended)»; the card confirmed by the overseer, inputs#I7

ops-main-architecture.md
decision/capability-ledger · escalate → Q3 — check 5: the plan's expected amendment, the exception kept on the founder's word on the live reading (outcome B); fit: not_master type 8 · resolved: reject (Q3) — the overseer, 2026-11-19, inputs#I7: the fit's form, the row's standing words true, the sidecar citing the reading
decision/cli-capability-ledger · escalate → Q3 — check 5: the same settlement; fit: not_master type 8 · resolved: reject (Q3) — the overseer, 2026-11-19, inputs#I7: the fit's form
decision/capability-ledger-gate · escalate → Q3 — check 5: the same settlement; fit: not_master type 8 · resolved: reject (Q3) — the overseer, 2026-11-19, inputs#I7: the fit's form

ops-main-security-plan.md
decision/unverified-cli-gate · escalate → Q3 — check 5: the same settlement; the entry that owned the exception is spent; fit: not_master type 8 · resolved: reject (Q3) — the overseer, 2026-11-19, inputs#I7: the fit's form; the spent owner dropped in ops-main-security-plan.2.md
decision/live-rig-prompt-capture-closed · escalate → Q1 — check 1: a boundary widening (rul-p2zl2kgl), the plan's expected amendment; the founder's own F1 answer covers it · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: «Назвать чтение (Recommended)»; the card confirmed by the overseer, inputs#I7

ops-main-test-plan.md
decision/hook-payloads-recorded-by-verify · escalate → Q3 — check 5: the same settlement; fit: not_master type 8 · resolved: reject (Q3) — the overseer, 2026-11-19, inputs#I7: the fit's form

ops-main-architecture.2.md
decision/claude-first-no-adapter-trait · escalate → Q1 — check 1: the boundary widening in the overseer's wording (inputs#I7) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: «Назвать чтение (Recommended)»; the card and its wording confirmed by the overseer, inputs#I7
decision/claude-shapes-in-one-crate · escalate → Q1 — check 1: the same · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: «Назвать чтение (Recommended)»; the card and its wording confirmed by the overseer, inputs#I7
decision/claude-shapes-isolated · escalate → Q1 — check 1: the same · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: «Назвать чтение (Recommended)»; the card and its wording confirmed by the overseer, inputs#I7

ops-main-security-plan.2.md
decision/unverified-cli-gate · apply — check 7: the shorter set fit's not_master owes on a row its own words make false (the owning entry is spent); Q3 resolved in the fit's form, the overseer, inputs#I7
