@row_add n=1 row=contract/viola-verify-output/browser-tools-line area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola verify output
item: browser-tools line
promise: When the browser-tools prompt stood at start, stdout holds one fixed plain-ASCII line before the summary: `the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again`; the summary stays last and the prompt-absent stdout is unchanged.
basis: fct-sfqjrkq4
because: verify's stdout gains a fixed line before the summary on a met browser-tools prompt, and no row of the verify output contract states that line yet.
