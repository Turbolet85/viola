@row_add n=1 row=decision/shared-start-prologue area=established-decisions fact=F1 detector=none
name: One start prologue shared by run, revive and adopt
rule: Run, revive and adopt start through one shared start prologue: the instance trust check, then the checked executable (a strict-modes refusal by the caller's writer under --json, else a home-tree refusal), then the wrapper log open, in that order; no start path carries its own copy of these checks.
basis: fct-xoaulb66
because: The chunk moved the trust check, the executable check and the log open of revive and adopt into one prologue that run's start module owns, so the rule that no start path keeps its own copy now binds the work to come.
@row_add n=2 row=decision/one-window-step-refusal-writer area=conventions fact=F2 detector=none
name: One refusal writer for a refused window step
rule: up's refused window-step bodies come from open's one refusal writer, so up and open refuse a window step with the same body; no verb keeps its own copy of that writer.
basis: fct-xoaulb66
because: The chunk removed up's byte-identical copy of the refusal writer and points up at open's, so a convention of one shared writer for the window step's refusals now stands.
