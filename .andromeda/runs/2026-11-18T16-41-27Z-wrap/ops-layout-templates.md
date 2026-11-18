@row_set n=1 row=contract/viola-verify-output/step-line cell=promise seen=1 fact=F1 detector=D-design-cli-wording
value: One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|absent|fail`, in ledger order, MM being the relied count; `absent` is the browser-tools-prompt row's word alone, when no typed run met the prompt; each appended once, with no progress bar and no redraw.
basis: fct-xmezyz7c, fct-tpkbiigi
because: The step line's verdict word gained `absent` for the browser-tools-prompt row, so the row's `pass|fail` wording no longer states what verify prints.
@row_set n=2 row=contract/viola-verify-output/summary cell=promise seen=1 detector=D-design-cli-wording
value: `stamped <ver>  <n> pass  <m> fail` is the last stdout line, `n` counting the rows that count, an `absent` browser-tools-prompt row among them; `fail` is never coloured.
basis: fct-tlsodzwd, fct-tpkbiigi
because: The summary's `n pass` now counts an `absent` browser-tools-prompt row while its step line keeps the word `absent`, so the row must say what `n` counts.
@row_set n=3 row=contract/viola-verify-output/exit-0 cell=promise seen=1 detector=D-design-cli-wording
value: Exit 0 when every relied row counts: it reads pass, or absent on the browser-tools-prompt row alone.
basis: fct-tlsodzwd, fct-wudigse5
because: The verified rule moved from every relied row passing to every relied row counting, with `absent` counting on the browser-tools-prompt row alone, so «every relied row passes» is no longer the exit-0 condition.
@row_add n=1 row=decision/nothing-typed-into-browser-tools-prompt area=surface-cli fact=F2 detector=none
name: Nothing typed into the browser-tools prompt
rule: viola verify types nothing into the claude CLI's browser-tools prompt and reads it as a modal, so the CLI version stays unverified until a person answers the prompt once.
basis: fct-zatpgkji, fct-qlgfl5fh
because: The chunk made the browser-tools prompt a compiled modal that verify settles on without typing, a chosen behaviour at the CLI surface that no surface-cli row states.
