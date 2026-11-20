@row_set n=1 row=decision/screen-signatures-compiled cell=rule seen=2 fact=F1 detector=none
value: Modal and input-box signatures are compiled literals validated by the per-version stamp; a screen is ready when no modal literal stands option-shaped or with no input box beneath it, and a footer literal or the box's rows stand: a rule row of the rule char, 40 or more, above a prompt-char row.
basis: fct-xxy4my3t
because: The readiness predicate now refuses only a standing modal (a modal literal on an option-shaped row or with no input-box row below it) in place of any row holding a modal literal, so a label quoted above the box reads ready.
@row_set n=2 row=decision/vt100-readiness-gate cell=rule seen=2 detector=none
value: vt100 0.16.2 serves only as the pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no standing modal (a modal literal on an option-shaped row or with no input box beneath) and waits, up to its maximum wait, for the stamped input-box signature; passthrough never waits.
basis: fct-xxy4my3t
because: The gate's at-once refusal now reads a standing modal rather than any known modal signature on the screen, so the row's rule must name the narrowed refusal.
@row_set n=3 row=contract/send-results/input-not-ready cell=promise seen=1 detector=none
value: A screen not quiet within the maximum wait, a poisoned model, a quiet screen with no input-box row at the bound, or a standing modal (a modal literal on an option-shaped row or with no input box beneath) on a verified CLI makes send type nothing and report not-delivered, detail input-not-ready.
basis: fct-xxy4my3t
because: A modal label quoted in a transcript above the input box no longer refuses a send, so the promised input-not-ready cause is a standing modal, not any modal literal.
@row_set n=4 row=contract/human-hint-lines/input-not-ready cell=promise seen=1 detector=none
value: Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a standing modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
basis: fct-pn22p663
because: The hint text is unchanged but its modal cause is now a standing modal only, as the report's expected amendments carry for this row.
