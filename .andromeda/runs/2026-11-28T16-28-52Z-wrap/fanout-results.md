# Fan-out results — 2026-11-28-console-actions wrap

architecture: add 5 · set 1 · retire 0 · none 0 · ops-architecture.md (re-spawned once, SET_IN_READ; raw twin .raw-ops-architecture.md)
security-plan: add 7 · set 3 · retire 0 · none 0 · ops-security-plan.md
design-system: add 13 · set 9 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 8 · set 3 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 3 · set 1 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 2 · set 4 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 7 · set 2 · retire 0 · none 2 · ops-a11y-plan.md

His answers, as recorded: F1–F7 (inputs#I2, relayed verbatim by the overseer, his two dialogs read 2026-11-28T14:32:05Z and 14:35:31Z); Q8 and Q9 (inputs#I5, relayed verbatim by the overseer, his dialog read 2026-11-28T15:24:31Z). The delegate's technical word: inputs#I6 (P1–P5 of the 2026-11-28T15-25-27Z-phase founder-forks, the recommended arms) and the P5 cards (inputs#I3, inputs#I7).

Check 5 — the plan's Expected amendments: every entry is met by an operation below, but test-plan `decision/harness-log-fields-owned-by-test-plan`: the row lets obs-plan add fields with no entry of its own, and the chunk only adds (`console-action`, `count`), so the row holds as written — no operation owed. The «obs Decisions Log entry» for the new event is the obs-plan sidecar entry `rows-land` writes with `contract/diag-line-fields/console-action`.
Check 6 — the one disproved claim (the refused case over a gone instance) stood in the plan alone (step 9), read into plan rev 6; the sweep's `D-gone` reads five true master rows: DISPOSED, no master site.

ops-architecture.md
decision/board-read-model-shape · reject — check 7: not_master type 4 — how one field of the board item is built, the standing row still true; `list --json` and the MCP list item are unchanged, held by their own contract rows
decision/console-runs-verbs-as-children · reject — check 2: the fact's home is security-plan decision/console-runs-verbs-as-children
decision/console-action-child-outlives-quit · reject — check 2: the fact's home is security-plan decision/console-action-child-outlives-quit
decision/console-answers-permission-and-plan · reject — check 2: the fact's home is design-system contract/fleet-console-keys/answer
decision/console-log-lines-codes-and-counts · reject — check 2: the fact's home is security-plan decision/console-diagnostics-closed-words
decision/unicode-width-column-widths · apply — check 1: accurate this-chunk addition (Dependencies: `unicode-width =0.2.2`) · word: delegate overseer, 2026-11-28, inputs#I7

ops-security-plan.md
decision/console-key-reader-bounded · apply — check 1: accurate this-chunk addition (the sequence decoder) · word: delegate overseer, 2026-11-28, inputs#I7
decision/child-path-trust · apply — check 1: accurate this-chunk addition (the action child is the own exe) · word: delegate overseer, 2026-11-28, inputs#I7
decision/refusals-never-block-human · reject — check 1: a change to the universal «the human always wins» invariant that no answer of his covers; the operator away, its most conservative option is no change, named in founder-forks.md
decision/console-runs-verbs-as-children · escalate → Q1 — check 1: boundary widening (a new subprocess crossing), his own answer to F1 covers it · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F1 «Дочерний, как человек (Recommended)», his dialog read 2026-11-28T14:32:05Z (inputs#I2); the confirm PROVISIONAL for the epoch batch
decision/console-typed-text-as-data · apply — check 1: accurate this-chunk addition (send's text on stdin his F4; switch's word rule and open's form the delegate's P2/P3) · word: delegate overseer, 2026-11-28, inputs#I6
decision/console-editing-line-bounded · apply — check 1: accurate this-chunk addition (the editing text's bound and the read-ending rule) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-action-child-outlives-quit · escalate → Q1 — check 1: boundary widening, his own answer to F1 (shown that a child may still run at a quit) covers it · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F1 «Дочерний, как человек (Recommended)», his dialog read 2026-11-28T14:32:05Z (inputs#I2); the confirm PROVISIONAL for the epoch batch
decision/console-answer-reads-kind-and-id · escalate → Q1 — check 1: his own answer to Q8 (no option or question text widened) covers it · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q8 «y/n, вопросы — в сессии (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
decision/board-dialog-id-never-serialised · reject — check 2: the fact's home is architecture decision/board-read-model-shape
decision/console-diagnostics-closed-words · apply — check 1: accurate this-chunk addition (`console-action`, `count`) · word: delegate overseer, 2026-11-28, inputs#I5

ops-design-system.md
decision/cli-ascii-only · escalate → Q1 — check 1: his own answer to Q9, the named exception to «plain ASCII» for the editing line · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q9 «Показывать как набрано (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
decision/ascii-only-human-output · escalate → Q1 — check 1: his own answer to Q9, the same exception · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q9 «Показывать как набрано (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
decision/list-is-the-board · escalate → Q1 — check 1: his own answer to F1/F2, a row that said the console acts on no strip · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answers to F1 «Дочерний, как человек» and F2·F7 «Всё одним чанком», his dialogs read 2026-11-28T14:32:05Z and 14:35:31Z (inputs#I2); the confirm PROVISIONAL for the epoch batch
decision/no-interactive-prompt-or-pager · escalate → Q1 — check 1: his own answers F3–F6 and Q8 (a selection, a confirm, an answer prompt, one editing line) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answers F3–F6 (inputs#I2, dialogs read 2026-11-28T14:32:05Z and 14:35:31Z) and Q8 «y/n, вопросы — в сессии (Recommended)» (inputs#I5, dialog read 2026-11-28T15:24:31Z); the confirm PROVISIONAL for the epoch batch
decision/one-progress-pattern · apply — check 1: accurate this-chunk addition (the static running line) · word: delegate overseer, 2026-11-28, inputs#I7
decision/step-counter-only-progress-pattern · apply — check 1: accurate this-chunk addition (the same) · word: delegate overseer, 2026-11-28, inputs#I7
contract/fleet-console-keys/unbound · apply — check 1: accurate this-chunk addition (every bound key; `a` with nothing pending unbound the delegate's P4) · word: delegate overseer, 2026-11-28, inputs#I6
contract/fleet-console-view/more-line · escalate → Q1 — check 1: his own answer to F6 · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F6 «Следует за выбором (Recommended)» (inputs#I2, his dialogs read 2026-11-28T14:32:05Z and 14:35:31Z); the confirm PROVISIONAL for the epoch batch
contract/fleet-console-keys/ctrl-c · apply — check 1: accurate this-chunk addition (no key trap, the plan's acceptance) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-actions-run-cli-verbs · apply — check 1: accurate this-chunk addition (his F3 letters, F2/F7 verbs; `a` and `o` the delegate's P1) · word: delegate overseer, 2026-11-28, inputs#I6
contract/fleet-console-view/selection · escalate → Q1 — check 1: his own answers to F3 (the selection over wrapped rows) and F6; its home here, layout-templates decision/console-selection read not_master by fit · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F3 «Стрелки/jk + буквы (Recommended)», his dialog of 2026-11-28T14:32:05Z/14:35:31Z (inputs#I2); the confirm PROVISIONAL for the epoch batch
contract/fleet-console-keys/confirm · escalate → Q1 — check 1: his own answer to F5 · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F5 «Только release и revive (Recommended)» (inputs#I2, his dialogs read 2026-11-28T14:32:05Z and 14:35:31Z); the confirm PROVISIONAL for the epoch batch
contract/fleet-console-keys/editing-line · apply — check 1: accurate this-chunk addition (one editing line his F4; the prompts' copy and open's form the delegate's P1/P2) · word: delegate overseer, 2026-11-28, inputs#I6
decision/console-typed-words-are-values · reject — check 2: the fact's home is security-plan decision/console-typed-text-as-data
contract/fleet-console-view/editing-line-width · escalate → Q1 — check 1: his own answer to Q9 (the row for a wide character's column width; the `...` lead the delegate's P5); its home here, layout-templates decision/console-editing-line read not_master by fit · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q9 «Показывать как набрано (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
contract/fleet-console-keys/answer · apply — check 1: accurate this-chunk addition (y/n his Q8; the prompts' wording the delegate's P4) · word: delegate overseer, 2026-11-28, inputs#I6
contract/fleet-console-keys/question · escalate → Q1 — check 1: his own answer to Q8 (a question left to the session, one line) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q8 «y/n, вопросы — в сессии (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
decision/console-result-rows-are-verb-output · apply — check 1: accurate this-chunk addition (output drawn escaped, his F1; refusals as the verb's own lines) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-one-action-at-a-time · escalate → Q1 — check 1: his own answer to F1 (one child at a time) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F1 «Дочерний, как человек (Recommended)», his dialog read 2026-11-28T14:32:05Z (inputs#I2); the confirm PROVISIONAL for the epoch batch
contract/fleet-console-view/key-line · reject — check 7: not_master type 5 — the key line letter by letter and its measured cut, an inventory the code's constant holds; whether the 80-column floor shows every letter is his open fork (founder-forks.md F-K)
absence/console-adopt · apply — check 1: accurate this-chunk addition (adopt owned by no entry, the P5 card's row) · word: delegate overseer, 2026-11-28, inputs#I7
contract/fleet-console-view/above-line · escalate → Q1 — check 1: his own answer to F6 · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F6 «Следует за выбором (Recommended)» (inputs#I2, his dialogs read 2026-11-28T14:32:05Z and 14:35:31Z); the confirm PROVISIONAL for the epoch batch

ops-layout-templates.md
decision/console-frame-floor · apply — check 1: accurate this-chunk addition (the key line on the last row; its 80-column cut is his open fork, as built while unanswered) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-own-surface · escalate → Q1 — check 1: his own answer to Q9 · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q9 «Показывать как набрано (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
decision/board-vs-strip · escalate → Q1 — check 1: his own answer to F1/F2, a row that said the console names no strip · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answers to F1 «Дочерний, как человек» and F2·F7 «Всё одним чанком», his dialogs read 2026-11-28T14:32:05Z and 14:35:31Z (inputs#I2); the confirm PROVISIONAL for the epoch batch
decision/console-key-line · reject — check 7: not_master type 4 — the key line's literal and its cut retell one constant; the code holds it (its printed promise lands as design-system contract/fleet-console-view/key-line)
decision/console-editing-line · reject — check 7: not_master type 4 — the build of one function; its printed promise lands as design-system contract/fleet-console-view/editing-line-width
decision/console-selection · reject — check 7: not_master type 4 — how the view is built; escalated (Q1), not asked (its promise lands as design-system contract/fleet-console-view/selection)
decision/console-result-rows · reject — check 7: not_master type 4 — how one view mechanism is built; the code holds it
decision/console-no-key-trap · reject — check 2: the fact's home is a11y-plan decision/console-no-key-trap
contract/console-prompts/editing · reject — check 2: the fact's home is design-system contract/fleet-console-keys/editing-line
contract/console-prompts/answer · reject — check 2: the fact's home is design-system contract/fleet-console-keys/answer
absence/console-adopt · reject — check 2: the fact's home is design-system absence/console-adopt

ops-obs-plan.md
contract/diag-line-fields/console-key · apply — check 1: accurate this-chunk addition (one line a read, the delegate's order; the fold and `question` technical) · word: delegate overseer, 2026-11-28, inputs#I5
decision/no-interactive-terminal-interface · escalate → Q1 — check 1: his own answers F3–F6 and Q8 · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answers F3–F6 (inputs#I2, dialogs read 2026-11-28T14:32:05Z and 14:35:31Z) and Q8 «y/n, вопросы — в сессии (Recommended)» (inputs#I5, dialog read 2026-11-28T15:24:31Z); the confirm PROVISIONAL for the epoch batch
contract/console-output/stdout · escalate → Q1 — check 1: his own answer to Q9, the named exception to «plain ASCII» · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to Q9 «Показывать как набрано (Recommended)», his dialog read 2026-11-28T15:24:31Z (inputs#I5); the confirm PROVISIONAL for the epoch batch
decision/console-agents-probe-volume · apply — check 1: accurate this-chunk addition (the plan's log-volume note) · word: delegate overseer, 2026-11-28, inputs#I7
contract/diag-line-fields/console-action · apply — check 1: accurate this-chunk addition (the new event) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-action-untraced · apply — check 1: accurate this-chunk addition (no span of its own) · word: delegate overseer, 2026-11-28, inputs#I7

ops-test-plan.md
decision/tui-verdicts-from-records · apply — check 1: accurate this-chunk addition (the verdicts read the verbs' own records) · word: delegate overseer, 2026-11-28, inputs#I7
contract/console-log-line/console-action · reject — check 2: the fact's home is obs-plan contract/diag-line-fields/console-action
contract/console-log-line/console-key-editing-line · reject — check 2: the fact's home is obs-plan contract/diag-line-fields/console-key
decision/console-actions-proven-live · apply — check 1: accurate this-chunk addition (v2-15's live half gated, `state: provisional`) · word: delegate overseer, 2026-11-28, inputs#I7

ops-a11y-plan.md
decision/cli-not-keyboard-interactive · reject — check 7: not_master type 5 — the standing exception (keys read in raw mode) stays true, the added tail an inventory of modes; escalated (Q1), not asked
decision/static-linear-output · escalate → Q1 — check 1: his own answers F3 and F6 (the redraw on a moved selection) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answers to F3 and F6 (inputs#I2, dialogs read 2026-11-28T14:32:05Z and 14:35:31Z); the confirm PROVISIONAL for the epoch batch
decision/console-keyboard-reach · reject — check 2: the fact's home is design-system decision/console-actions-run-cli-verbs
decision/console-no-key-trap · apply — check 1: accurate this-chunk addition (the plan's a11y acceptance) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-editing-line-exception · escalate → Q1 — check 1: his own answer to F4 (one editing line), the exception to «no interactive CLI input» · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: confirmed, all 20 — the row rests on his answer to F4 «Одна строка (Recommended)» (inputs#I2, his dialogs read 2026-11-28T14:32:05Z and 14:35:31Z); the confirm PROVISIONAL for the epoch batch
decision/console-answer-through-viola-answer · reject — check 2: the fact's home is design-system contract/fleet-console-keys/answer
decision/console-editing-line-width · reject — check 2: the fact's home is layout-templates decision/console-editing-line
decision/console-selection-by-gutter-mark · apply — check 1: accurate this-chunk addition (the gutter, never colour) · word: delegate overseer, 2026-11-28, inputs#I7
decision/console-refusals-are-the-verbs-own · reject — check 2: the fact's home is design-system decision/console-result-rows-are-verb-output


ops-main-security-plan.md
decision/child-path-trust · apply — check 1: the provisional mark of a row landed on a delegate's word (amendment-flow §Validate, the disposition line) · word: delegate overseer, 2026-11-28, inputs#I7

ops-main-design-system.md
decision/one-progress-pattern · apply — check 1: the provisional mark of a row landed on a delegate's word (amendment-flow §Validate, the disposition line) · word: delegate overseer, 2026-11-28, inputs#I7
decision/step-counter-only-progress-pattern · apply — check 1: the provisional mark of a row landed on a delegate's word (amendment-flow §Validate, the disposition line) · word: delegate overseer, 2026-11-28, inputs#I7

ops-main-layout-templates.md
decision/console-frame-floor · apply — check 1: the provisional mark of a row landed on a delegate's word (amendment-flow §Validate, the disposition line) · word: delegate overseer, 2026-11-28, inputs#I7

ops-main-test-plan.md
decision/tui-verdicts-from-records · apply — check 1: the provisional mark of a row landed on a delegate's word (amendment-flow §Validate, the disposition line) · word: delegate overseer, 2026-11-28, inputs#I7

