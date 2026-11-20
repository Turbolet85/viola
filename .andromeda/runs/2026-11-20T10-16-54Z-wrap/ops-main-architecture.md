@row_set n=1 row=decision/screen-signatures-compiled cell=rule seen=2 detector=none
value: Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row is a standing modal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char.
basis: fct-xxy4my3t, fct-pn22p663
because: The readiness predicate's modal refusal moved from any row holding a modal literal to a standing modal, rule (A) the founder chose, so the row's «no row holds a modal literal» reads «no row is a standing modal».
@row_set n=2 row=decision/vt100-readiness-gate cell=rule seen=2 detector=none
value: vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no standing modal and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model.
basis: fct-xxy4my3t, fct-pn22p663
because: The gate's at-once refusal now reads a standing modal in place of any known modal signature on the screen, so «requires no known modal signature» reads «requires no standing modal».
@row_set n=3 row=contract/send-results/input-not-ready cell=promise seen=1 detector=none
value: A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a standing modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
basis: fct-xxy4my3t, fct-pn22p663
because: A modal label quoted above the box no longer refuses a send, so the promised cause «a modal on a verified CLI» reads «a standing modal on a verified CLI».
