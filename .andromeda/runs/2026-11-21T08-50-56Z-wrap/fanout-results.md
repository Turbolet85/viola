# Fan-out results — 2026-11-20-harness-prefix-and-identity-floor-ledger-rows (wrap 2026-11-21T08-50-56Z-wrap)

architecture: add 6 · set 3 · retire 1 · none 0 · ops-architecture.md
security-plan: add 5 · set 3 · retire 1 · none 0 · ops-security-plan.md
design-system: none · ops-design-system.md (reason: every verify row states «every relied row» and the count `MM` with no figure; no new UI or token surface)
layout-templates: add 0 · set 0 · retire 0 · none 3 · ops-layout-templates.md
test-plan: add 4 · set 4 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 2 · set 2 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md (reason: no interactive surface, output discipline, suite or schema changed)
main: ops-main-architecture.md (retire 1 · add 1 · set 1) · ops-main-security-plan.md (retire 1 · add 1 · set 2)

Check 5 (the plan's Expected amendments), entries no operation changes: security-plan `decision/r8-claude-env-strip` holds as written (the floor named, not counted; names only) — its members stand in architecture `decision/identity-floor-names`, the non-identity names in security-plan `decision/non-identity-hook-names`; security-plan `decision/hook-capture-arm` holds (the env-names read stands in `decision/capture-arms-isolated`); its Decisions Log entry owed by `decision/new-env-read-needs-log-entry` is the sidecar entry of `decision/capture-arms-isolated`'s set, its Why the founder's own word; `decision/fixture-scrub-and-refusal`, `decision/recorder-scrub`, `decision/verify-probe-captures-transient` hold (the env fixture passes the same scrub; `decision/env-fixture-names-only` adds it); test-plan `decision/hook-payloads-recorded-by-verify`, `decision/fake-agent-matches-verify-fixtures`, `decision/fake-agent-replay-for-stamps`, `decision/one-verify-stamp-per-build`, `decision/standing-live-rig` hold (no label, count or fixture class stated); architecture `contract/hook-capture-arm/payload-file` and `contract/ledger-stamps/content` hold — the env file and `env_probe` take items of their own. Matrix: ledger-note at P7.3.

Rows ratified ONLY for the sender or for Run B's `crossSessionInbound` literal (architecture `decision/verify-child-set`, security-plan `decision/verify-children-direct-spawn`, `decision/verify-trusted-runs-risk`, the residual rows `decision/verify-transcripts-accepted-residual`, `decision/verify-real-cli-transcripts-accepted`, the misaddressing residual, obs-plan `contract/verify-spawn-subject/*`, a11y-plan time-bound rows): NOT amended — the cut took their ground (inputs#I14 item 3); no operation offers them.

ops-architecture.md
decision/identity-floor-eleven-names · reject — check 1: superseded by ops-main-architecture.md's retire (its heir's why kept whole)
decision/identity-floor-names · reject — check 1: its why drops the standing qualifier «and the prefix rule already removes them when inherited» (inputs#I14 item 4); ops-main-architecture.md adds the row with the why whole
decision/identity-floor-ledger-row · escalate → Q2 — check 2: one fact offered twice, stated differently (security-plan's adds the stamp and the record and drops the unread or off-shape file's fail) · resolved: apply (Q2) — delegate the overseer, 2026-11-21, inputs#I15: «Architecture keeps it (Recommended)»
contract/hook-capture-arm/env-names-file · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
contract/ledger-stamps/env-probe · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1: names land in the stamp's measured block) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
contract/recorded-env-fixture/env-recorded-1-json · reject — check 2: the fact's home is test-plan contract/claude-fixtures/env-names (its PII rule security-plan decision/env-fixture-names-only)
decision/fake-agent-replays-env-names · reject — check 2: the fact's home is test-plan contract/fake-agent-hooks/session-start-env
decision/plugin-env-vars-unused · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 4, P1; «В не-идентичностные») · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/r8-identity-floor · reject — check 1: drops the qualifying word «measured» (inputs#I14 item 4); ops-main-architecture.md keeps it and states the bundle reading
decision/probe-capture-arms-inert · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»

ops-security-plan.md
decision/capture-arms-isolated · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1); its sidecar entry is the Decisions Log entry decision/new-env-read-needs-log-entry owes · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/live-rig-kept-stamp-owner-only · reject — check 1: adds HF-2, outside the founder's answer «Добавить HF и HG в строку (Recommended)»; ops-main-security-plan.md sets HF and HG alone
contract/artifact-canary-scan-classes/claude-stripped · reject — check 1: replaces one figure the code states with another the row can avoid; ops-main-security-plan.md states the needles with no count
decision/identity-floor-eleven-names · reject — check 1: superseded by ops-main-security-plan.md's retire (its heir's why kept whole)
decision/identity-floor-ledger-row · escalate → Q2 — check 2: one fact offered twice, stated differently (architecture's) · resolved: reject (Q2)
decision/non-identity-hook-names · reject — check 1: its why «Ratified.» drops the standing qualifier «the prefix rule already removes them when inherited outside the persistent set» (inputs#I14 item 4); ops-main-security-plan.md adds the row with the why whole
decision/env-fixture-names-only · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1: one fixture through the scrub) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/env-probe-names-and-counts-only · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1: names in the stamp, kept names removed in memory) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/launched-by-session-ids-on-floor · reject — check 3: its `state: provisional` contradicts the founder's ratification «На пол, вместе со спутником (Recommended)» (inputs#I10 item 2), and its rule is a measurement record (one recording, one version, a bundle reading) — the matrix note on v2-26 at P7.3 carries it

ops-test-plan.md
decision/r8-claude-env-strip · escalate → Q3 — check 1: F-R1, the floor's count leaves; inside the floor answer «На пол, вместе со спутником», confirmed by the overseer as delegate (PROVISIONAL) · resolved: apply (Q3) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm, PROVISIONAL (Recommended)»
decision/claude-env-strip · escalate → Q3 — check 1: F-R1, as above (PROVISIONAL) · resolved: apply (Q3) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm, PROVISIONAL (Recommended)»
decision/literal-test-oracles · escalate → Q3 — check 1: F-R1, as above (PROVISIONAL) · resolved: apply (Q3) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm, PROVISIONAL (Recommended)»
decision/identity-floor-tested-against-literal-copy · reject — check 7: not_master type 5 — the four names are an inventory the test's literal oracle holds; escalated (Q1), not asked; the standing row still holds
contract/claude-fixtures/env-names · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1: one fixture through the scrub) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
contract/fake-agent-hooks/session-start-env · apply — check 5: the plan's expected test-plan fake-agent amendment, an accurate this-chunk addition (fct-yw2wvymn)
decision/strip-control-oracle-keeps-guarded-names · apply — check 1: accurate this-chunk addition (the report's Decisions & corrections, `evidence/control-2.txt`, `-3.txt`)
decision/capture-arm-env-names-only · reject — check 2: the fact's home is architecture contract/hook-capture-arm/env-names-file and security-plan decision/capture-arms-isolated

ops-obs-plan.md
decision/identity-floor-always-stripped · reject — check 7: not_master type 5 — the four named hook names are an inventory the code's constant holds; escalated (Q1), not asked; the standing row is false (11 names, no list), so ops-main-obs-plan.md writes the shorter set
decision/env-names-only · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 3, E1) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/kept-env-names-never-stamped · reject — check 2: the fact's home is security-plan decision/env-probe-names-and-counts-only
decision/env-fixtures-names-only · reject — check 2: the fact's home is security-plan decision/env-fixture-names-only

ops-main-architecture.md
decision/identity-floor-eleven-names · escalate → Q1 — check 1: boundary widening the founder answered himself («На пол, вместе со спутником»: the floor of thirteen); the row's eleven-name list is false, retired with no heir (its heir's add waits for the founder, founder-forks.md item 1) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/identity-floor-names · reject — check 7: not_master type 5 — the member list is an inventory the code's IDENTITY_FLOOR holds; escalated (Q1), not asked — the founder is away and only his own word lands it against a not_master: founder-forks.md, item 1
decision/r8-identity-floor · reject — check 7: not_master type 8 — «measured or read from the CLI bundle» is provenance; escalated (Q1), not asked; the standing «measured» is false, so ops-main-architecture.2.md writes the shorter set

ops-main-security-plan.md
decision/identity-floor-eleven-names · escalate → Q1 — check 1: as above (the floor of thirteen; the non-identity half to its heir) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/non-identity-hook-names · escalate → Q1 — check 1: boundary widening the founder answered himself («В не-идентичностные»: the list of four) · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
decision/live-rig-kept-stamp-owner-only · escalate → Q1 — check 1: the founder's answer to F-P4-1 «Добавить HF и HG в строку (Recommended)» · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»
contract/artifact-canary-scan-classes/claude-stripped · reject — check 7: not_master type 5 — the named needles are an inventory the scan's code holds; the standing count is false, so ops-main-security-plan.2.md writes the shorter set

ops-main-architecture.2.md
decision/r8-identity-floor · escalate → Q1 — check 1: boundary widening the founder answered himself («На пол, вместе со спутником»); the shorter set after fit's not_master, «measured» dropped as false · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»

ops-main-obs-plan.md
decision/identity-floor-always-stripped · escalate → Q1 — check 1: boundary widening the founder answered himself (Fork 4, P1: the obs row reworded to the list as built); the shorter set after fit's not_master · resolved: apply (Q1) — delegate the overseer, 2026-11-21, inputs#I15: «Confirm (Recommended)»

ops-main-security-plan.2.md
contract/artifact-canary-scan-classes/claude-stripped · apply — check 1: routine — a count in a master that carries no rule, dropped; the shorter set after fit's not_master
