@row_add n=1 row=contract/viola-verify/browser-tools-line area=1-a11y-scope-summary fact=F1 detector=none
boundary: viola verify output
item: browser-tools line
promise: On a run that met the browser-tools prompt, verify prints one fixed plain-ASCII line before the summary saying the prompt stood at start, viola never answers it, and to answer it once in claude then run viola verify again; the summary stays the last line, and the prompt-absent output is unchanged.
basis: fct-sfqjrkq4
because: verify's stdout gains a fixed line before the summary on a met browser-tools prompt, a new promise of the verify output surface the a11y plan's critical path covers, and no standing row states it.
