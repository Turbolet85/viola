# Relay 2 — the overseer's technical answer on the box read before entry 10 (implement, 2026-11-15)

Question (asked as a dialog): the rig reads the `x` echo and the DEL erase from an ANSI-stripped stream with no
terminal model; a cell-diff redraw would read «stands» and, by the rule, end the step, spending the start and leaving
v2-02 unread on a false reading. What should the rig do?

Selected: "Add a screen model (Recommended)"

Notes, verbatim:

The overseer's technical answer (no founder fork): add the minimal VT screen model, controlled on synthetic streams of both renderer shapes and the «stands» arm, then re-fire entries 4–8. AND I correct my own rule of dialog 67, because it made a false reading cost the hold: a box that reads «stands» no longer ENDS the step. The rig records the reading as it read it (`box: erased` or `box: stands`), types NO further key either way, and goes on to `viola release` and the send. The send's own outcome is then the second witness: `send-confirmed` says the box was empty whatever the model read; `not-delivered` / a `wheel` line `human` says the `x` stood — then the hold is lost and the step reads red, the same loss ending it would have been. So a false «stands» costs nothing and a true one costs what it always did. Record both readings side by side. Rehearse the corrected arm under the fake (its no-erase gives the true-«stands» case: send not delivered, step red, nothing typed after). If the stop rule stands in a PLAN line (step 3 or 3b, a criterion), say which and stop for the phase door before firing; if it lives only in the rig and the outcome notes, record this as the deviation's correction and fire entry 10 on the word you hold.
