# fanout-results — 2026-11-22-paste-write-bounded (wrap 2026-11-22T13-02-53Z-wrap)

architecture: add 5 · set 6 · retire 0 · none 0 · ops-architecture.md
security-plan: add 3 · set 2 · retire 0 · none 3 · ops-security-plan.md
test-plan: add 4 · set 5 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 2 · set 6 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 2 · set 4 · retire 0 · none 0 · ops-a11y-plan.md
design-system: add 3 · set 1 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 2 · set 1 · retire 0 · none 0 · ops-layout-templates.md
main: ops-main-a11y-plan.md (set 2) · ops-main-obs-plan.md (set 1) · ops-main-test-plan.md (set 1)

Q1 — the answer card: rows lying wholly inside the operator's own answers of 2026-11-22 (inputs#I2, the phase run's founder-forks.md; inputs#I7 item 3): «Новое слово paste-cut (Recommended)» — the new `not-delivered` detail, exit 13, its hint (its words provisional); «5 секунд (Recommended)» — the compiled bound; «Отказывать до вмешательства (Recommended)» — the refusal after a cut until a human key or a prompt or session edge clears it. Rule «Boundary widening the founder answered himself to a P4 fork» (rul-bznresq5): ONE card, one confirm.

Expected amendments (plan) with no operation, each checked against its row (check 5): arch `contract/cli-exit-codes/13` — its promise «The `not-delivered` refusal.» names no detail and stays true · arch `decision/mcp-call-deadline` — its rule states no send total (55 s above the 50 s wait cap) and stays true at 53.5 s · test-plan option count — `contract/fake-agent-cli/argv-only-options` states no count · matrix#v2-24 notes — P7.3.

ops-architecture.md
decision/send-time-bound · escalate → Q1 — check 1: the compiled 5 s bound, inside «5 секунд (Recommended)» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
decision/atomic-bracketed-send · apply — check 1: the plan's Expected amendment «atomic-bracketed-send except a cut» names the change; narrows what is written, no widening
contract/refusal-reason/not-delivered · escalate → Q1 — check 1: the closed detail list gains paste-cut, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/refusal-order/send · escalate → Q1 — check 1: the two paste-cut rungs, inside «Новое слово paste-cut» and «Отказывать до вмешательства» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/send-refusal/order · escalate → Q1 — check 1: the two paste-cut rungs, inside «Новое слово paste-cut» and «Отказывать до вмешательства» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/send/paste-failure-after-issue · escalate → Q1 — check 1: a cut after issue refused paste-cut with its cursor, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
decision/paste-bound · escalate → Q1 — check 1: the cut at the compiled bound, inside «5 секунд (Recommended)»; the queued-paste cut (implement's deviation, the cut contract) rides the same answer · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
decision/paste-cut-mark · escalate → Q1 — check 1: the cut mark and its clears, inside «Отказывать до вмешательства (Recommended)»; this is the mark's one home (check 2) · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
decision/one-pty-writer-thread · reject — check 7: not_master type 4 — how the PTY writer is built (one worker thread, a queue of 8, catch_unwind) is a mechanism the code holds; an add, nothing follows (a key's bounded wait stands in a11y contract/tui-passthrough/human-keystrokes)
contract/not-delivered-details/paste-cut · escalate → Q1 — check 1: the new detail, exit 13, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/human-hint-lines/paste-cut · escalate → Q1 — check 1: the paste-cut hint, inside «Новое слово paste-cut» (its words provisional); the boundary holds one row an item, its input-not-ready sibling standing · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL

ops-security-plan.md
decision/bridge-send-one-bracketed-paste · apply — check 1: the plan's Expected amendment «bridge-send-one-bracketed-paste except a cut paste» names it; narrows what is written
decision/send-text-is-one-bracketed-paste · apply — check 1: the plan's Expected amendment «send-text-is-one-bracketed-paste except a cut paste» names it; narrows what is written
decision/paste-write-bounded · reject — check 2: the fact's home is architecture decision/paste-bound (the cut at the compiled bound, the queued cut); «no key, env var or flag sets it» stands by the universal no-config-switch rule
decision/paste-cut-mark · reject — check 2: the fact's home is architecture decision/paste-cut-mark
contract/channel-refusal/paste-cut · escalate → Q1 — check 1: the new detail on the channel's result.refusal, inside «Новое слово paste-cut»; the boundary holds one row an item, its empty-text sibling standing · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL

ops-test-plan.md
decision/send-refusal-order · reject — check 1: the set drops «again» from the re-read after the gate (inputs#I7 item 5: a set that drops a qualifying word is rejected); main's own set in ops-main-test-plan.md
contract/driver-send/typed-text · apply — check 1: the plan's Expected amendment «contract/driver-send/typed-text except a cut» names it
contract/viola-send/paste-framing · apply — check 1: the plan's Expected amendment «contract/viola-send/paste-framing except a cut» names it
contract/send-event-log/send-issued · escalate → Q1 — check 1: a cut after send-issued refused paste-cut with that cursor, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
decision/human-never-blocked · reject — check 7: not_master type 4 — the tail adds the writer's chunk size and closing marker, a mechanism the code holds; a set lengthening a row still true, nothing follows
decision/send-refused-after-a-paste-cut · reject — check 2: the fact's home is architecture decision/paste-cut-mark
contract/fake-agent-cli/stop-reading · apply — check 1: the plan's Expected amendment «a new contract/fake-agent-cli/stop-reading row» names it; argv only, no env seam
contract/viola-send/paste-cut · escalate → Q1 — check 1: the e2e outcome of the new word and the mark, inside «Новое слово paste-cut» and «Отказывать до вмешательства» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/viola-send/during-a-blocked-paste · apply — check 1: the build read inputs#I7 item 4 directs (a second send waits at most the bound, then turn-running; 4.0 s measured), a test promise; decision/send-never-queued stands as worded

ops-obs-plan.md
decision/run-threads-inside-catch-unwind · apply — check 1: the plan's Expected amendment «three run PTY threads» names it
decision/run-worker-panic-ends-run · apply — check 1: the plan's Expected amendment «three run PTY threads» names it
decision/panic-caught-at-every-role · apply — check 1: the same thread count moved (two → three), a stale count in a row the plan's entry did not list
contract/send-refused-detail/refusal-reason-words · escalate → Q1 — check 1: the closed list gains paste-cut, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/send-log-line/detail · escalate → Q1 — check 1: the closed list gains paste-cut, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/send-path-log-events/event-order · reject — check 1: the set drops «awaited» (inputs#I7 item 5); main's own set in ops-main-obs-plan.md
decision/send-cut-mark-uninstrumented · apply — check 1: the plan's Expected amendment «a new row: the cut mark opens no span and writes no line of its own» names it
contract/send-refused-detail/paste-cut · escalate → Q1 — check 1: the new detail's two placements on the log line, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL

ops-a11y-plan.md
decision/time-bounds-never-on-human · reject — check 1: the set drops «send» from «send readiness gate» and shortens «confirmation windows» (inputs#I7 item 5); main's own set in ops-main-a11y-plan.md
decision/time-bounds-fall-on-the-driver · reject — check 1: the set drops «own» from «viola's own child» (inputs#I7 item 5); main's own set in ops-main-a11y-plan.md
decision/human-keystrokes-never-blocked · reject — check 7: not_master type 4 — the value adds how the writer is built (a shared queue, a 4096-byte chunk); the standing «delayed past the current atomic paste» is false, so main's own shorter set stands in ops-main-a11y-plan.md
contract/tui-passthrough/human-keystrokes · apply — check 1: the same moved delay bound, the plan's Expected amendment as above
decision/paste-cut-mark-holds-sends · reject — check 2: the fact's home is architecture decision/paste-cut-mark
decision/cut-paste-never-submitted · reject — check 2: the fact's home is architecture decision/paste-bound and decision/atomic-bracketed-send's except

ops-design-system.md
contract/cli-exit-codes/13 · escalate → Q1 — check 1: exit 13's details gain paste-cut, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
contract/refusal-hints/not-delivered-paste-cut · escalate → Q1 — check 1: the hint, PROVISIONAL words, inside «Новое слово paste-cut» · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
decision/paste-cut-refuses-until-human · reject — check 2: the fact's home is architecture decision/paste-cut-mark
decision/cut-paste-closes-bracket-no-enter · reject — check 2: the fact's home is architecture decision/atomic-bracketed-send's except and a11y decision/human-keystrokes-never-blocked

ops-layout-templates.md
contract/cli-exit-codes/13 · reject — check 7: not_master type 4 — the value lengthens exit 13's promise with a paste cut's mechanics (bound, bracket, wheel) the refusal-reason rows and the code hold; the standing «`not-delivered`.» stays true; escalated (Q1), not asked
contract/viola-send-output/after-a-paste-cut · reject — check 2: the fact's home is architecture decision/paste-cut-mark; its printed form is contract/cli-exit-codes/13's
contract/viola-send-output/paste-cut-hint · escalate → Q1 — check 1: the hint on the send output's own struck line, PROVISIONAL words, inside «Новое слово paste-cut»; its input-not-ready-hint sibling standing · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL

ops-main-a11y-plan.md
decision/time-bounds-never-on-human · reject — check 7: not_master type 5 — fit reads «paste write» one more item of an inventory the code holds, the rule standing without it; escalated (Q1), not asked: the operator's answer «5 секунд» names the member (inputs#I7 item 3), and only his own word lands against not_master — founder-forks.md item 1 (291 of 300, every standing member and word kept)
decision/time-bounds-fall-on-the-driver · reject — check 7: not_master type 5 — as above; founder-forks.md item 1 (297 of 300, every standing member and word kept)
decision/human-keystrokes-never-blocked · apply — check 7: the correction fit's not_master owes (the standing «atomic paste» false), the shorter set naming the bound without the writer's build

ops-main-obs-plan.md
contract/send-path-log-events/event-order · apply — check 1: the cut path named, «awaited» and every standing exception kept (297 of 300)

ops-main-architecture.md
decision/human-always-wins-wheel · apply — check 1: the cascade's stale statement of the retired mechanism («the current atomic paste»), the same correction a11y decision/human-keystrokes-never-blocked took

ops-main-obs-plan.2.md
decision/run-worker-panic-ends-run · apply — check 1: the cascade's stale statement of the amended claim — the why's «either thread» beside a rule now naming three

ops-main-test-plan.md
decision/send-refusal-order · escalate → Q1 — check 1: the two paste-cut rungs, inside «Новое слово paste-cut» and «Отказывать до вмешательства»; «again» kept (290 of 300) · resolved: apply (Q1) — delegate overseer, 2026-11-22, inputs#I8: «Confirm all 19 (Recommended)», PROVISIONAL
