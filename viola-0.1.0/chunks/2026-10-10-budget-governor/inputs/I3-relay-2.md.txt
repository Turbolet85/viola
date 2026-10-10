The operator's answers to the four P4 forks of chunk 2026-10-10-budget-governor (the AskUserQuestion round, 2026-10-10, after P3), each question as asked, the option chosen, and the operator's note verbatim where one was given.

1. "With the instance check moved ahead of the log, where is a refused check recorded on `revive`'s start arm and on the statusline arm?"
   Answer: "No log line (Recommended)"
   Note, verbatim: "A refusal that lasts one invocation is no refusal. Put its cost on the review card for the founder to see: a refused check on the statusline arm leaves no trace anywhere, and say where an operator would learn of it."

2. "A window is over its threshold but its `resets_at` is `unknown` (a malformed value). Does the gate pause?"
   Answer: "Does not pause (Recommended)"

3. "What does `viola release <name> --budget` do when no pause stands?"
   Answer: "Nothing is recorded (Recommended)"
   Note, verbatim: "The human line wording goes on the card for the founder."

4. "Does an override outlive its wrapper (a `run` restart or a `revive` inside the crossed window)?"
   Answer: "No (Recommended)"

The options as offered:
1. No log line: the refused check opens nothing under the home; `revive` prints its state-files pair and exits 1, the arm exits 0 silently, the home's mode stays as found; the contracted `process-exit` / `hook-decision` line for `strict-modes-failed` is dropped on these two arms. Keep the line: check first, then open the log; the opening narrows the home, so the refusal lasts one invocation. Line, home not narrowed: writes under a home that failed the check, the playbook's "Boundary widening" pattern.
2. Does not pause: a window is judged only when both its figures are known. Pauses: the known percentage decides and the pause cannot lift by time.
3. Nothing is recorded: no override, no `budget-gate` line, no snapshot change; the result is `{wheel, budget_paused:false}` with no `override_until`, exit 0; planning form of the human line `<name>  budget gate open  wheel <holder>`. Pre-authorize: an override until the nearer known window's `resets_at`.
4. No: every start evaluates from `budget.json` alone. Yes: the start restores a still-future `budget_override_until`.
