# fanout-results — 2026-11-16-wait-wake-condition

architecture: add 0 · set 5 · retire 0 · none 0 · ops-architecture.md
security-plan: add 0 · set 4 · retire 0 · none 0 · ops-security-plan.md
design-system: add 1 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 0 · set 1 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 1 · set 6 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 0 · set 0 · retire 0 · none 1 · ops-obs-plan.md (one `@row_none` type 4)
a11y-plan: none · ops-a11y-plan.md — reason: the chunk is test-only; no a11y row moves.

Expected amendments (check 5): architecture contract/channel-wait/wake-kinds → ops-architecture.md n1 (wait-wake-kinds,
the background_tasks clause) and n2 (channel-wait/wake-kinds, the non-waking records) and n3 (no scheduled-wake count);
test-plan contract/viola-wait/non-waking-records → ops-test-plan.md n2; test-plan decision/fresh-home-per-test ·
rstest-fixture-chain · verify-sole-stamp-writer (one fact: a copy of one stamp per built binary, never a link, no owner
record) → ops-test-plan.md n4 (copy, never a link) · the add decision/one-verify-stamp-per-build (one stamp per build
under a lock) · n3 (no owner record, the try_lock sweep) — one home each (check 2); rstest-fixture-chain and
verify-sole-stamp-writer stand true (the chain is unchanged and verify stays the only stamp writer: a copy writes no
stamp); test-plan decision/verify-driven-tests-20s-kill · ci-stamped-home-kill-line → ops-test-plan.md n5 (the
rationale), ci-stamped-home-kill-line's rule (the 20 s kill) unchanged and true. Disproved claims (check 6): the two
plan claims are plan-only, absorbed by plan revs 6 and 10 — no master site (Phase 1 sweep).

ops-architecture.md
contract/channel-methods/wait-wake-kinds · apply — check 5: the plan's Expected amendment names this change (every turn-ended wakes whatever its background_tasks; telling done from paused is the driver's); rule Accurate this-chunk addition
contract/channel-wait/wake-kinds · apply — check 5: the plan's Expected amendment for this row; the report pins activity and a harness-origin prompt-submitted as non-waking after a task-free end
contract/event-data/turn-ended-last-assistant-message · apply — check 5: the Expected amendment's «viola carries no scheduled-wake count», stated where the turn-ended data is promised; an absence, no widening (session_crons stays the founder's candidate)
decision/test-home-owner-sweep · reject — check 2: the row stands true (a stamp dir is no test home); the fact's home is test-plan decision/test-home-owner-record-sweep
decision/test-homes-cleaned-by-owner · reject — check 2: the row stands true (a stamp dir is no test home); the fact's home is test-plan decision/test-home-owner-record-sweep

ops-security-plan.md
decision/stop-background-tasks-count-only · reject — check 2: the fact's home is architecture contract/event-data/turn-ended-last-assistant-message (no scheduled-wake count); the row stands true
decision/tests-use-own-home · reject — check 2: the fact's home is test-plan decision/fresh-home-per-test; the row stands true
decision/e2e-home-removal-own-dirs-only · reject — check 2: the row stands true (a stamp dir is a dir the test side itself created); the sweep's lock condition's home is test-plan decision/test-home-owner-record-sweep
decision/test-removal-own-dirs-only · escalate → Q1 — check 1: rule «Boundary widening» (rul-r74hshif, rul-p2zl2kgl) by subject: a security ban's list of how a test-side removal knows its own dir gains a fourth way (another build's stamp dir by its lock); the dirs removed stay test-created, but the list is the founder's · resolved: apply (Q1) — delegate overseer, 2026-11-16, inputs#I16: «Where a letter halts for his word, take the delegate's answer by relay, PROVISIONAL, the most conservative option; nothing is ratified» — the founder away; landed with `state: provisional` (ops-main-security-plan.2.md), the fork in founder-forks.md

ops-main-security-plan.2.md
decision/test-removal-own-dirs-only · apply — check 1: the state set that marks Q1's PROVISIONAL landing (delegate overseer, 2026-11-16, inputs#I16); a state set brings no text and takes no fit

ops-design-system.md
decision/wait-wakes-on-every-turn-end · reject — check 2: the fact's home is architecture contract/channel-methods/wait-wake-kinds

ops-layout-templates.md
contract/cli-verbs/wait · reject — check 2: the fact's home is architecture contract/channel-methods/wait-wake-kinds; the verb's layout promise stands true

ops-test-plan.md
contract/viola-wait/turn-ended · reject — check 2: the fact's home is architecture contract/channel-methods/wait-wake-kinds; the row stands true
contract/viola-wait/non-waking-records · apply — check 5: the plan's Expected amendment for this row
decision/test-home-owner-record-sweep · apply — check 5: the Expected amendment's «the stamp dir carrying no owner record»; the sweep of another build's stamp by try_lock (report Changes, Symbols / APIs)
decision/fresh-home-per-test · apply — check 5: the Expected amendment's «a copy … never a link»
decision/verify-driven-tests-20s-kill · apply — check 5: the Expected amendment's «rationale moves to the stamp taken once; the kill unchanged»
contract/fake-agent-cli/script · apply — check 1: rule Accurate this-chunk addition — the chunk's fake script drives Notification and UserPromptSubmit steps (report Schema / config)
decision/one-verify-stamp-per-build · apply — check 5: the Expected amendment's «one viola verify stamp per built binary», the lock (report Symbols / APIs)
