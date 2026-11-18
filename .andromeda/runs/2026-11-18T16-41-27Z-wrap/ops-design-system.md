@row_set n=1 row=contract/viola-verify-output/step-line cell=promise seen=1 fact=F1 detector=D-design-cli-wording
value: One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
basis: fct-xmezyz7c, fct-tlsodzwd
because: verify's step line gained the verdict word `absent` on the 24th row, so the row's «ending `pass` or `fail`» no longer states the printed wording.
@row_set n=2 row=contract/viola-verify-output/summary cell=promise seen=1 detector=D-design-cli-wording
value: A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.
basis: fct-tlsodzwd
because: the summary's `n` now counts the rows that count, an `absent` browser-tools-prompt row included, which the row does not say.
@row_set n=3 row=contract/viola-verify-output/summary-line cell=promise seen=1 detector=D-design-cli-wording
value: The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`; a failing row prints no stderr word.
basis: fct-tlsodzwd
because: the summary's `n` now counts the rows that count, an `absent` browser-tools-prompt row included, which the row does not say.
@row_set n=4 row=contract/viola-verify-output/trust-record-names-nothing cell=promise seen=1 detector=D-design-cli-wording
value: The trust-record row prints its step line ending `pass` or `fail`, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
basis: fct-wudigse5, fct-xmezyz7c
because: not every relied row now ends `pass` or `fail` (the browser-tools-prompt row may end `absent`), so «like every relied row» is false.
@row_add n=1 row=decision/browser-tools-prompt-never-answered area=surface-cli fact=F2 detector=none
name: The CLI's browser-tools prompt is never answered
rule: The CLI's browser-tools prompt (`Yes, use my browser`) is a modal viola never answers: `viola verify` types nothing past it and `viola send` is refused while it stands, so the CLI version stays unverified until a person answers the prompt once.
basis: fct-qlgfl5fh, fct-orxmgvlj
because: the chunk made the browser-tools prompt a compiled modal that verify and send never type into, a decision no surface-cli row states.
