# Fan-out results — wrap 2026-11-28T08-40-41Z-wrap (chunk 2026-11-28-console-scaffold)

architecture: add 5 · set 6 · retire 0 · none 0 · ops-architecture.md
security-plan: add 6 · set 0 · retire 0 · none 0 · ops-security-plan.md
design-system: add 13 · set 4 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 7 · set 9 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 5 · set 14 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 7 · set 12 · retire 0 · none 0 · ops-obs-plan.md (the re-spawn's, propose-r1, after SET_IN_READ on a retire of absence/interactive-terminal-interface outside its rows file; main raises that retire in ops-main-obs-plan.md)
a11y-plan: add 5 · set 9 · retire 0 · none 2 · ops-a11y-plan.md
main: ops-main-obs-plan.md (add 1 · retire 1) · ops-main-test-plan.md (add 1)

ops-a11y-plan.md
decision/static-linear-output · escalate → Q1 — check 1: a boundary widening (rul-r74hshif, rul-p2zl2kgl), his own answer to P4 fork F2 (rul-bznresq5): the answer card · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-28, inputs#I2: «Именное исключение (Recommended)»; the card's confirm the overseer's, inputs#I8
decision/cli-not-keyboard-interactive · escalate → Q1 — check 1: a boundary widening (rul-r74hshif, rul-p2zl2kgl), his own answer to P4 fork F2 (rul-bznresq5): the answer card · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-28, inputs#I2: «Именное исключение (Recommended)»; the card's confirm the overseer's, inputs#I8
decision/cli-no-interactive-input · escalate → Q1 — check 1: a boundary widening (rul-r74hshif, rul-p2zl2kgl), his own answer to P4 fork F2 (rul-bznresq5): the answer card · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-28, inputs#I2: «Именное исключение (Recommended)»; the card's confirm the overseer's, inputs#I8
decision/linear-static-output · reject — check 1: a redraw exception on a row outside F2's four, a widening his answers do not cover (the phase's read: not reached, design-system decision/fleet-console-not-verb-output scoping verb output rules off the console's redraw); written to founder-forks.md
decision/linear-static-cli-output · reject — check 1: a redraw and cursor exception on a row outside F2's four (the phase's read: not reached); written to founder-forks.md
decision/no-spinners · reject — check 1: a redraw exception on a row outside F2's four (the phase's read: not reached); written to founder-forks.md
contract/log-identity/process · apply — check 1: rul-5hynfkq5, the process tag gains console · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/cli-fixed-message/form · apply — check 1: rul-5hynfkq5, the console's two fixed pairs
decision/strict-modes-refusal-before-any-write · apply — check 1: rul-5hynfkq5, the console's exit-1 strict refusal
contract/console-refusal/no-terminal · reject — check 2: the fact's home is design-system contract/viola-console-refusals/no-terminal
decision/console-keys-quit-and-interrupt-only · reject — check 2: the fact's home is design-system contract/fleet-console-keys/q, ctrl-c, unbound
decision/console-size-floor · reject — check 2: the fact's home is layout-templates decision/console-frame-floor
decision/console-plain-ascii-no-sgr · reject — check 2: the fact's home is design-system decision/fleet-console-plain-ascii
decision/console-restores-terminal-first · reject — check 2: the fact's home is obs-plan contract/panic-outcome-by-role/console

ops-architecture.md
decision/console-own-surface · reject — check 2: the fact's home is layout-templates decision/console-own-surface and decision/console-frame-floor
decision/console-start-checks · reject — check 2: the fact's home is security-plan decision/console-terminal-and-home-check
contract/diagnostics-role-files/console · reject — check 2: the fact's home is obs-plan contract/diagnostics-files/console
decision/console-keys-by-closed-class · reject — check 2: the fact's home is security-plan decision/console-key-reader-bounded
name/command/console · apply — check 1: rul-5hynfkq5, the occupied command name
decision/codes-only-process-logs · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/cli-exit-codes/1 · apply — check 1: rul-5hynfkq5, the console's exit-1 refusals
contract/cli-internal-error/output · reject — check 3: the plan holds it unamended (p5-card-4: read and holding); the console's restore-then-print order is obs-plan contract/panic-outcome-by-role/console
decision/new-capabilities-as-verbs · apply — check 1: rul-5hynfkq5, viola console takes no --json (plan step 5) · word: delegate the overseer, 2026-11-28, inputs#I5
decision/cli-human-text-json-exit-codes · apply — check 1: rul-5hynfkq5, viola console takes no --json (plan step 5) · word: delegate the overseer, 2026-11-28, inputs#I5
decision/cli-output-conventions · apply — check 1: rul-5hynfkq5, viola console takes no --json (plan step 5) · word: delegate the overseer, 2026-11-28, inputs#I5

ops-design-system.md
contract/help-output/verb-groups · apply — check 1: the help's board row, his own answer to F3 · word: relayed verbatim by the overseer, 2026-11-28, inputs#I3
contract/viola-help/verb-groups · apply — check 1: the help's board row, his own answer to F3 · word: relayed verbatim by the overseer, 2026-11-28, inputs#I3
decision/list-is-the-board · apply — check 1: rul-5hynfkq5, console names no strip
contract/cli-refusal-line/fixed-unable-form · apply — check 1: rul-5hynfkq5, the console's two exit-1 refusals
decision/fleet-console-own-surface · reject — check 2: the fact's home is layout-templates decision/console-own-surface
contract/fleet-console-frame/at-floor · reject — check 2: the fact's home is layout-templates decision/console-frame-floor
contract/fleet-console-frame/below-floor · reject — check 2: the fact's home is layout-templates decision/console-frame-floor
decision/fleet-console-redraws-on-resize · reject — check 2: the fact's home is layout-templates decision/console-redraw-on-resize
contract/fleet-console-keys/q · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
contract/fleet-console-keys/ctrl-c · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
contract/fleet-console-keys/unbound · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
decision/fleet-console-plain-ascii · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
name/command/console · reject — check 2: the fact's home is architecture name/command/console
contract/viola-console-refusals/no-terminal · apply — check 1: rul-5hynfkq5, the console's fixed refusal pair
contract/viola-console-refusals/home-refused · apply — check 1: rul-5hynfkq5, the console's fixed refusal pair
contract/cli-catch-site/console · reject — check 2: the fact's home is obs-plan contract/panic-outcome-by-role/console
contract/cli-exit-codes/1-console · reject — check 2: the fact's home is architecture contract/cli-exit-codes/1

ops-layout-templates.md
contract/viola-help/verb-groups · apply — check 1: the help's board row, his own answer to F3 · word: relayed verbatim by the overseer, 2026-11-28, inputs#I3
contract/cli-verbs/help · apply — check 1: the help's board row, his own answer to F3 · word: relayed verbatim by the overseer, 2026-11-28, inputs#I3
decision/board-vs-strip · apply — check 1: rul-5hynfkq5, console names no strip
decision/no-prompt-or-pager · reject — check 1: the standing row already scopes the console off the verb rules («the terminal fleet console is no verb's output»), a founder-kept row; rewording it is a widening outside F2's four — written to founder-forks.md
decision/no-persistent-footer · reject — check 1: an exception on a verb-output row outside F2's four, the console scoped off verb output by decision/no-prompt-or-pager; written to founder-forks.md
decision/human-tty-expression-level · reject — check 1: a redraw exception on a row outside F2's four; written to founder-forks.md
decision/only-header-bay-line · reject — check 1: an exception on a verb-output row outside F2's four; written to founder-forks.md
decision/plain-output-modes · reject — check 1: a cursor-control exception on a row outside F2's four; written to founder-forks.md
contract/cli-token-rendering/motion-tokens · reject — check 1: a redraw exception on a row outside F2's four (the phase's read: not reached); written to founder-forks.md
contract/cli-verbs/console · apply — check 1: rul-5hynfkq5, the new verb's item
contract/viola-console-refusals/no-terminal · reject — check 2: the fact's home is design-system contract/viola-console-refusals/no-terminal
contract/viola-console-refusals/home-refused · reject — check 2: the fact's home is design-system contract/viola-console-refusals/home-refused
decision/console-own-surface · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
decision/console-frame-floor · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
decision/console-redraw-on-resize · escalate → Q2 — «no fitting area»: area=new, section Surface: console · resolved: reject (Q2) — main's own add under the new area replaces it
decision/console-keys · reject — check 2: the fact's home is design-system contract/fleet-console-keys/q, ctrl-c, unbound

ops-security-plan.md
decision/console-key-reader-bounded · apply — check 5: the plan's Expected amendment, security-plan §Input Validation · word: delegate the overseer, 2026-11-28, inputs#I5
decision/console-terminal-and-home-check · apply — check 1: rul-5hynfkq5, the console's start checks
contract/strict-modes-refusal/console · apply — check 1: rul-5hynfkq5, the console's item beside the other verbs'
decision/console-restores-terminal-before-error · reject — check 2: the fact's home is obs-plan contract/panic-outcome-by-role/console
decision/console-log-content-free · reject — check 2: the fact's home is security-plan decision/console-key-reader-bounded (no key byte) with obs-plan contract/diag-line-fields/console-frame and console-key
decision/console-local-surface · reject — check 1: rul-ytq2orum, the Threat Model Summary is a verbatim upstream copy; no upstream line states it

ops-test-plan.md
decision/cli-tests-as-subprocess · apply — check 1: rul-5hynfkq5, the console takes no --json and is driven under a PTY
decision/cli-json-one-document · apply — check 1: rul-5hynfkq5, the console's two refusals are fixed stderr pairs
decision/hint-names-its-cause · apply — check 1: rul-5hynfkq5, the console's two causes have no --json document
decision/home-another-user-may-read-refused · apply — check 1: rul-5hynfkq5
decision/home-read-rule · apply — check 1: rul-5hynfkq5
decision/home-readable-by-others-refused · apply — check 1: rul-5hynfkq5
contract/strict-modes-refusal/21 · apply — check 1: rul-5hynfkq5
contract/strict-modes-refusal/no-line · apply — check 1: rul-5hynfkq5
decision/strict-modes-before-any-log · apply — check 1: rul-5hynfkq5
decision/tui-verdicts-from-records · apply — check 1: rul-5hynfkq5, the console's records and line mode
contract/log-line/process · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
decision/process-logs-via-tracing-subscriber · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
decision/structured-json-process-logs · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
decision/literal-test-oracles · apply — check 1: rul-5hynfkq5, M3's one source (cry-boujwuyr) · word: delegate the overseer, 2026-11-28, inputs#I2 (Q2)
contract/strict-modes-refusal/console · apply — check 1: rul-5hynfkq5, the console's item beside run's and verify's
decision/console-restore-read-on-own-terminal · apply — check 6: the disproved master-fd read, dispositioned as a test strategy
contract/log-event-fields/console-frame · reject — check 2: the fact's home is obs-plan contract/diag-line-fields/console-frame
contract/log-event-fields/console-key · reject — check 2: the fact's home is obs-plan contract/diag-line-fields/console-key
contract/caught-panic-exit-code/console · reject — check 2: the fact's home is obs-plan contract/panic-outcome-by-role/console

ops-obs-plan.md
decision/distinct-detail-code-per-exit-cause · apply — check 1: rul-5hynfkq5
decision/no-error-reporter · apply — check 1: rul-5hynfkq5, the role count at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/diag-line/process · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/log-line/process · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/process-log/process · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
decision/no-process-log-without-instance · apply — check 1: rul-5hynfkq5
decision/log-file-basenames-follow-tests · apply — check 1: rul-5hynfkq5
decision/structured-json-process-logs · reject — check 7: not_master type 5 — the added «a closed set of five» is an inventory; main writes the shorter set in its own file
contract/process-log-line/process · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/diagnostics-line/role-file-line-size · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
contract/diagnostics-files/paths · apply — check 1: rul-5hynfkq5
decision/zero-unlogged-panics · apply — check 1: rul-5hynfkq5, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
decision/console-sole-interactive-terminal · reject — check 5: main raises the absence's replacement in ops-main-obs-plan.md with F2's named exception as his answer words it
contract/diagnostics-files/console · apply — check 1: rul-5hynfkq5
decision/console-start-order · reject — check 2: the fact's home is security-plan decision/console-terminal-and-home-check
contract/diag-line-fields/console-frame · apply — check 5: the plan's Expected amendment, the event catalog · word: delegate the overseer, 2026-11-28, inputs#I5
contract/diag-line-fields/console-key · apply — check 5: the plan's Expected amendment, the event catalog · word: delegate the overseer, 2026-11-28, inputs#I5
contract/panic-outcome-by-role/console · apply — check 5: the plan's Expected amendment, obs-plan §7 · word: delegate the overseer, 2026-11-28, inputs#I4
contract/console-output/stdout · apply — check 1: rul-5hynfkq5, the console's stdout is its screen

ops-main-obs-plan.md
decision/structured-json-process-logs · apply — check 7: the shorter set fit's not_master owes, the role list at five · word: delegate the overseer, 2026-11-28, inputs#I2 (Q3)
decision/no-interactive-terminal-interface · escalate → Q1 — check 1: a boundary widening (rul-r74hshif), his own answer to P4 fork F2 (rul-bznresq5): the answer card · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-28, inputs#I2: «Именное исключение (Recommended)»; the card's confirm the overseer's, inputs#I8
absence/interactive-terminal-interface · escalate → Q1 — check 1: the absence leaves for the decision above, his own answer to F2: the answer card · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-28, inputs#I2: «Именное исключение (Recommended)»; the card's confirm the overseer's, inputs#I8

ops-main-test-plan.md
decision/boot-paths-declared-once · escalate → Q3 — check 7 expected not_master (the earlier wrap's fit, type 5): his own answer «Только boot-paths (Recommended)», relayed verbatim in inputs#I7, lands it as his exception · resolved: apply (Q3) — relayed verbatim by the overseer, 2026-11-28, inputs#I7: «Только boot-paths (Recommended)»

ops-main-design-system.md
contract/fleet-console-keys/q · escalate → Q2 — «no fitting area»: the new area surface-console, Surface: console, the plan's Expected amendment · resolved: apply (Q2) — delegate viola-overseer, 2026-11-28: make the area Surface: console, inputs#I8
contract/fleet-console-keys/ctrl-c · escalate → Q2 — «no fitting area»: the new area surface-console, Surface: console · resolved: apply (Q2) — delegate viola-overseer, 2026-11-28: make the area Surface: console, inputs#I8
contract/fleet-console-keys/unbound · escalate → Q2 — «no fitting area»: the new area surface-console, Surface: console · resolved: apply (Q2) — delegate viola-overseer, 2026-11-28: make the area Surface: console, inputs#I8
decision/fleet-console-plain-ascii · reject — check 7: not_master type 4 — the code holds the mechanism; escalated (Q2), not asked

ops-main-layout-templates.md
decision/console-own-surface · escalate → Q2 — «no fitting area»: the new area surface-console, Surface: console, the plan's Expected amendment · resolved: apply (Q2) — delegate viola-overseer, 2026-11-28: make the area Surface: console, inputs#I8
decision/console-frame-floor · escalate → Q2 — «no fitting area»: the new area surface-console, Surface: console; the floor his answer Q1 · resolved: apply (Q2) — delegate viola-overseer, 2026-11-28: make the area Surface: console, inputs#I8
decision/console-redraw-on-resize · reject — check 7: not_master type 4 — the code holds the mechanism; escalated (Q2), not asked
