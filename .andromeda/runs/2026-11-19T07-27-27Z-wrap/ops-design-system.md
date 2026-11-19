@row_add n=1 row=contract/viola-verify-output/browser-tools-line area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola verify output
item: browser-tools line
promise: After a successful `stamps.json` update and before the summary, a run that met the browser-tools prompt prints one plain ASCII stdout line, no SGR: `the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again`.
basis: fct-sfqjrkq4
because: verify's stdout gains one fixed line before the summary when a run met the browser-tools prompt, and no design-system row states that wording.
@row_add n=2 row=decision/browser-tools-prompt-never-answered area=surface-cli fact=F2 detector=none
name: viola never answers the browser-tools prompt
rule: viola never answers the CLI's browser-tools prompt; `viola verify` names it and leaves the human to answer it once in claude.
basis: fct-sfqjrkq4
because: the chunk's verify line states that viola never answers the browser-tools prompt and hands it to the human, a rule no design-system row holds yet.
