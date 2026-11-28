# Founder forks — the 2026-11-28-console-actions wrap (the operator away; no dialog raised for him)

Written 2026-11-28T17:05:05Z (`date -u`). Each fork is his; none lies inside his recorded answers (F1–F7, inputs#I2;
Q8 and Q9, inputs#I5). Until he answers, each stands as written under «while unanswered».

## F-K — the key line at the 80-column floor
At 80 columns the console's key line is cut to `q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m swi...`,
so `a answer` and `o open` do not show at the floor he set (`CONSOLE_KEYS`, `src/human/console.rs:20`, 98 columns in
full; the cut pinned by `console_run_on_q_draws_the_frame_then_quits_restored`). Priced from the code:
1. **Shorter labels that fit 80 columns (Recommended).** One candidate measured at 79 columns:
   `q quit  jk  l last  p pause  r rel  f focus  v rev  s send  m sw  a ans  o open`; a gentler one,
   `q quit jk l last p pause r release f focus v revive s send m switch a answer o open`, is 83 and still cut. Cost:
   one constant and the pinned literal; every letter shows at the floor, the words get terse.
2. **A second key line.** The key line takes two rows: the body falls from 22 to 21 fleet rows at 80x24 (one row in
   22 fewer, about 4.5%), `write_console_frame` and the view's row budget change, the frame-floor row and its tests
   with them. Every word stays whole.
3. **Leave it cut (as built).** `q quit` stays first and always shows; `a` and `o` work but are not shown at 80
   columns, only on a wider tile.
Rows it would amend: layout-templates `decision/console-frame-floor` (the frame and its last row) and the console's
key line (`CONSOLE_KEYS`, which no master row holds: fit read its literal not a master's, type 5). **While unanswered:
as built (option 3).**

## F-R — a refusal of a send the console runs reaches the human
security-plan `decision/refusals-never-block-human` reads «Security refusals go only to automation (send/answer),
never to the human». The console runs `viola send` / `viola answer` for the human as children and draws their refusal
lines in its result rows (a send from it is logged with origin `driver`: `console_send_a_cyrillic_line_as_typed`).
The security-plan proposer offered an exception naming the console. It changes a universal invariant («the human
always wins») that none of his answers names (F1 covers the console running the verbs as the human, not this row).
Options:
1. **Name the console as the row's exception (Recommended)** — the rule stands for every other path; the console's
   refusals are the verb's own lines, shown to the human who typed the action.
2. **Keep the row as it stands and have the console's actions go as the human's own** (a different origin on its
   sends) — a product change, a later route entry.
3. **Leave the row and the code as they are** — the row then reads wider than the console behaves.
**While unanswered: the most conservative option, no change to the row (the proposal rejected at this wrap).**

## F-N — two of his own-word rows that fit kept out of the masters
His answers stand on record whatever the masters hold; fit read these two operations not a master's:
- a11y-plan `decision/cli-not-keyboard-interactive` — the exception's tail listing the console's modes (a selection,
  y/n confirms, answer prompts, one editing line) read as an inventory (type 5); the standing exception (keys read in
  raw mode) stays true, and the modes land as their own rows (a11y `decision/console-editing-line-exception`, design
  `decision/no-interactive-prompt-or-pager`).
- layout-templates `decision/console-selection` — read as how the view is built (type 4); its promise lands instead as
  design-system `contract/fleet-console-view/selection`.
Only his own word lands an operation against a `not_master`. **While unanswered: left out.**
