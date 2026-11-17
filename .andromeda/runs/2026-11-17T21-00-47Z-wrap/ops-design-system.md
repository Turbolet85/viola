@row_add n=1 row=contract/viola-verify-output/trust-record-names-nothing area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola verify output
item: trust-record row
promise: The trust-record row prints its step line ending `pass` or `fail` like every relied row, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
basis: fct-oyqnxj2g, fct-ytf3ugzg
because: verify gained a row that writes and removes a trust record in the Claude config file, and the surface promises that none of its output names that file, key or value.
@row_add n=2 row=decision/session-folder-trusted-word-on-pass area=brand-identity fact=F2 detector=D-design-cli-wording
name: Session-folder-trusted word only on a passed row
rule: The display word `Session folder trusted by viola` is written only when verify's trust-record row passes for the running CLI version, never on a fail, an unverified CLI or a writer's code word.
why: Nothing announces success before it is confirmed.
basis: fct-2t56x2kc, fct-vw5u2c4x
because: the chunk ties the trust display word to the trust-record row's pass, a rule future display work must respect.
