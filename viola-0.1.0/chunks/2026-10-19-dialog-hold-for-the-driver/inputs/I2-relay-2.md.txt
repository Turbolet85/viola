# P4 forks — 2026-10-19-dialog-hold-for-the-driver (the founder's own answers)

The P4 dialog was shown in this session at 2026-10-19T08:0xZ as one round of three questions, per the operator's
take-up direction (`inputs#I1`). It came back with an option chosen on each, and a note on each naming the founder's own
word, relayed verbatim by the operator.

## Q1 — Hold length (playbook pattern "Boundary widening", escalate: a new `config.json` key)
Shown: "Dialog hold length (founder's word): today the wrapper holds a dialog for the driver for a compiled 60 s, then
leaves it to the human. How long should it hold, and who sets it? Any key the human types still hands the dialog back
at once, whatever the hold. A new config.json key is a 'Boundary widening' (playbook): choosing an option with a key
ratifies it."
Options shown: «Key, default 60 (Recommended)» · «Key, default 3600» · «Compiled 3600, no key» · «Compiled 600, no key».
Chosen: **Key, default 60 (Recommended)** — config.json key `dialog_hold_s`, whole seconds 60–3600, default 60, a bad
value takes 60 with one parse-rejected line; hooks.json and the hook's bound compiled at the 3600 s ceiling.
Note, verbatim: "The founder's own word, live in the overseer's dialog 2026-10-19T08:04Z: «Ключ, по умолчанию 60» —
widening ratified by him."

## Q2 — At expiry (playbook pattern "Boundary widening" on the deny option)
Shown: "What the wrapper does when a hold expires (founder's word). Either way the chunk writes a 'dialog-expired'
event that wakes viola wait, so the driver learns the dialog is gone. A viola-built deny would be viola deciding a
dialog nobody answered: a 'Boundary widening' (playbook) and a departure from 'mechanism, not policy'."
Options shown: «Leave it to the human (Recommended)» · «Deny with a fixed message».
Chosen: **Leave it to the human (Recommended)** — `null` at expiry, the CLI's own menu; a late answer `unknown-dialog`.
Note, verbatim: "The founder's own word: «Отдать человеку»."

## Q3 — Live `claude` start cap
Shown: "Live claude start cap (founder's word): whether the CLI 2.1.287 actually waits on a dialog hook past 75 s has
never been measured, and CI's fake agent cannot show it. The hold only runs on a viola-verified CLI, and no standing
home has a current stamp, so a live round starts with one `viola verify` (its usual runs). How many live starts may this
chunk make? Linux dev host only; Windows owed to 'Windows-only live measurements'."
Options shown: «Verify + 1 start, 11 min (Recommended)» · «Verify + 1 start, full hour» · «No live start».
Chosen: **Verify + 1 start, 11 min (Recommended)** — one `viola verify` on a fresh rig home outside the repo, then one
wrapped claude 2.1.287 start whose one dialog is answered by `viola answer` after about 11 minutes; no key typed into any
dialog; no ledger row; the 3600 s ceiling stays PROVISIONAL.
Note, verbatim: "The founder's own word: «Verify + 1 старт, 11 мин» — one viola verify plus one wrapped claude 2.1.287
start; a red never re-fired."

Authority: the founder's own answers to the three P4 forks, given live in the overseer's dialog and relayed verbatim by
the operator. Q1 ratifies the `config.json` key widening; Q2 chooses the non-widening option; Q3 caps the live round at
one `viola verify` plus one wrapped start, a red never re-fired.
