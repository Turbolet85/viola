# Fan-out results — wrap 2026-11-17T14-02-45Z of 2026-11-17-supply-chain-green-and-booted-tests-end

architecture: add 4 · set 0 · retire 0 · none 1 · ops-architecture.md
security-plan: add 2 · set 0 · retire 1 · none 1 · ops-security-plan.md
design-system: no proposer
layout-templates: no proposer
test-plan: add 2 · set 4 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 0 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md — reason: the chunk is test-only and supply-chain configuration; no a11y row it touches moves

ops-architecture.md
decision/advisory-ignore-grounds · reject — check 2: the fact's home is security-plan decision/advisory-ignore-grounds
decision/tied-boot-parent-death-signals · reject — check 2: the fact's home is test-plan decision/supervisor-ordinary-child
decision/every-in-test-boot-tied · reject — check 2: the fact's home is test-plan decision/no-untied-harness-boot
decision/shared-test-helper-cases-no-dev-deps · reject — check 2: the fact's home is test-plan decision/rstest-case-tables

ops-security-plan.md
decision/rmcp-advisory-ignores · reject — check 1: written by main in ops-main-security-plan.md, its heir keeping «until rmcp leaves its below-3.5 range»
decision/advisory-ignore-grounds · reject — check 1: its first ground read as «the Critical CVE response … for an unmaintained upstream»; re-worded by main in ops-main-security-plan.md on the founder's words
decision/advisory-ignore-list · reject — check 1: it dropped «standing until rmcp leaves its below-3.5 range»; re-worded by main in ops-main-security-plan.md

ops-main-security-plan.md
decision/rmcp-advisory-ignores · escalate → Q1 — check 1: boundary widening (rule «Boundary widening»), the retire whose heir lists the widened ground's id · resolved: apply (Q1) — delegate the overseer, 2026-11-17: «all six lie inside the founder's own dialog answer of 2026-11-17T12:05:56Z», inputs#I4
decision/advisory-ignore-grounds · escalate → Q1 — check 1: boundary widening (rule «Boundary widening»), the widened rule's standing row · resolved: apply (Q1) — delegate the overseer, 2026-11-17: «all six lie inside the founder's own dialog answer of 2026-11-17T12:05:56Z», inputs#I4
decision/advisory-ignore-list · escalate → Q1 — check 1: boundary widening (rule «Boundary widening»), the list gaining RUSTSEC-2026-0543 on the second ground · resolved: apply (Q1) — delegate the overseer, 2026-11-17: «all six lie inside the founder's own dialog answer of 2026-11-17T12:05:56Z», inputs#I4

ops-test-plan.md
decision/no-advisory-ignored · escalate → Q2 — check 1: boundary widening (rule «Boundary widening»), the exception taking the second ground · resolved: apply (Q2) — delegate the overseer, 2026-11-17: «all six lie inside the founder's own dialog answer of 2026-11-17T12:05:56Z», inputs#I4
decision/no-ignored-advisories · escalate → Q2 — check 1: boundary widening (rule «Boundary widening»), the exception taking the second ground · resolved: apply (Q2) — delegate the overseer, 2026-11-17: «all six lie inside the founder's own dialog answer of 2026-11-17T12:05:56Z», inputs#I4
decision/supervisor-ordinary-child · apply — check 5: the plan's expected amendment, verify SIGTERM and supervisor SIGKILL (report Counts moved, Harness / gate surface)
decision/rstest-case-tables · apply — check 3: the scope record's companion line (`tests/support/source_scan.rs · companion · serves step 6`), a shared helper's own cases a labelled table
decision/no-untied-harness-boot · apply — check 6: the census the chunk added, red on a planted positive (report Coverage of new surfaces)
decision/tied-verify-kill-witness · apply — check 6: disposes the two disproved plan claims (the wait and the stamp read)

ops-obs-plan.md
decision/direct-chrono-dependency · escalate → Q3 — check 1: boundary widening (rule «Boundary widening»), viola's own chrono use kept under the widened ground · resolved: apply (Q3) — delegate the overseer, 2026-11-17: «all six lie inside the founder's own dialog answer of 2026-11-17T12:05:56Z», inputs#I4
