@row_set n=1 row=contract/process-exit-detail/verify-refusal cell=promise seen=1 fact=F1 detector=D-obs-instrumentation
value: Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture, untrusted-folder or interrupted; a failing row carries no code.
basis: fct-wnswac6i, fct-vmrlleio, fct-yf2imaoq
because: verify's closed refusal codes on its process-exit gained untrusted-folder, added to the diag-line detail enum, so the row's closed list must name it.
@row_add n=1 row=contract/verify-untrusted-folder-stderr/unable-hint-pair area=span-trace-coverage fact=F2 detector=none
boundary: verify stderr on an untrusted folder
item: the untrusted-folder pair
promise: A verify whose Runs B to E meet the trust or imports form prints exactly "unable: claude did not start ready in this folder: it is not trusted" and "hint: run viola verify from a folder you trust in Claude Code" on stderr, with no step lines, no stamped summary and no stamp; wording provisional.
basis: fct-yf2imaoq, fct-r7kjslfe
because: verify gained a new refusal with its own fixed stderr pair, a promise at the stderr boundary beside the stop pair, its wording standing provisional.
@row_add n=2 row=decision/window-view-restore-uninstrumented area=span-trace-coverage fact=F3 detector=D-obs-instrumentation
name: Window view restore uninstrumented
rule: The window step's reads of the active workspace before and after its move, and its restore of the earlier active workspace when the move changed it, add no placement word, span field or log line of their own; a failed read or restore leaves the placement word as the move set it.
basis: fct-yf2imaoq, fct-p4ngd5ji, fct-gs7z5go5
because: the window step gained a before and after monitors read and a conditional restore whose instrumentation is only the seam's existing hyprctl-probe pairs, a choice future work on run.window must respect.
