@row_set n=1 row=contract/viola-verify-exit-codes/1-relied-row-failed cell=promise seen=1 detector=none
value: A relied ledger row failed; the failing row's stamp is still written. A probe run whose first screen read the trust or imports form is no failed row but the untrusted-folder refusal, which writes no stamp.
basis: fct-yf2imaoq
because: A run whose first screen read trust or imports now ends in a no-stamp refusal before any row is checked, so the exit-1 failing-row promise must say it no longer covers that case.
@row_set n=2 row=contract/viola-verify-exit-codes/1 cell=promise seen=1 detector=none
value: The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed. A probe run whose first screen read the trust or imports form exits 1 as untrusted-folder, no step line, no stamp.
basis: fct-yf2imaoq
because: verify gains an exit-1 refusal, untrusted-folder, that the occupied exit-1 promise does not name.
@row_add n=1 row=decision/verify-refuses-untrusted-folder area=established-decisions fact=F1 detector=none
name: Verify refuses an untrusted folder
rule: viola verify refuses with untrusted-folder, exit 1, when any of Runs B to E reads the trust or imports form on its first screen (Runs A and F never read): no step line, no summary, no stamp, probe folders removed; a start's automatic verify then records nothing and leaves the version un-attempted.
basis: fct-yf2imaoq
because: A confounded probe run is now a refusal that stamps nothing, a decision future verify and start work must respect.
@row_add n=2 row=contract/viola-verify-refusal-codes/untrusted-folder area=established-decisions fact=F2 detector=none
boundary: viola verify process-exit refusal codes
item: untrusted-folder
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a probe run's first screen read the trust or imports form; no stamp is written.
basis: fct-vmrlleio
because: verify's closed refusal code set gains untrusted-folder, and each code of the set is one contract row.
@row_add n=3 row=contract/verify-process-exit/untrusted-folder area=conventions fact=F3 detector=none
boundary: viola verify process-exit detail
item: untrusted-folder
promise: The code verify's process-exit carries, at ERROR under a parsed VIOLA_NAME, when a probe run's first screen read the trust or imports form.
basis: fct-yf2imaoq
because: verify's process-exit detail gains untrusted-folder beside its sibling codes in this section.
@row_add n=4 row=contract/process-exit-detail/untrusted-folder area=occupied-resources fact=F4 detector=D-arch-resources
boundary: process-exit detail enum
item: untrusted-folder
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola verify's untrusted-folder refusal.
basis: fct-wnswac6i
because: The diagnostics line schema's process-exit detail enum gains one value, an occupied word to register beside its siblings.
@row_add n=5 row=decision/window-step-keeps-view area=established-decisions fact=F5 detector=none
name: The window step keeps the view
rule: The window step reads the focused monitor's active workspace before its one move and, after a dispatched move, sends one switch back to it only when the active workspace changed; the placement word is set by the move alone, and a failed read or restore changes no word, field or line.
why: Ratified as a boundary widening.
basis: fct-yf2imaoq
because: A placing start now restores the active workspace its move took, a decision every caller of the window step inherits.
