@row_set n=1 row=contract/viola-run-output/child-started cell=promise seen=2 detector=D-design-cli-wording
value: `run`, `revive` and `adopt` print nothing once the child starts, except `adopt`'s not-resumed end, which writes its fixed pair or `--json` document after the child released the terminal.
basis: fct-x4brbvla, fct-j2uatzhk
because: adopt's not-resumed end now writes its pair or document after pump_end returns, so the row's flat «adopt prints nothing» is false for that named case.
@row_set n=2 row=decision/no-json-after-child-held-terminal cell=except seen=2 fact=F2 detector=D-design-cli-wording
value: `viola adopt`'s not-resumed end writes its fixed pair or `--json` document after the child released the terminal; `run` and `revive` still write nothing past the child.
basis: fct-x4brbvla, fct-j2uatzhk, fct-uteui6y3
because: the rule's reason «viola writes nothing past it» now has one named exception, adopt's not-resumed end, scoped to adopt alone.
@row_set n=3 row=contract/cli-exit-codes/1-adopt-start cell=promise seen=1 fact=F3 detector=D-design-cli-wording
value: A start refusal of `viola adopt`, or its not-resumed end after the child ran, exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; a resumed child ends exit 0.
basis: fct-x4brbvla, fct-blcs6o5x
because: adopt's exit 1 now also covers the not-resumed end after the child ran, which the row named only as start refusals.
@row_add n=1 row=contract/viola-adopt-refusals/not-resumed area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola adopt refusals
item: not-resumed
promise: When the adopted child ends before its session resumed: `unable: <name>'s session did not resume` then `hint: check the session id - claude --resume <id> must open it by hand, under the same claude config`, or under `--json` the `not-resumed` document, detail null, stderr empty; exit 1.
basis: fct-x4brbvla, fct-blcs6o5x
because: the chunk adds adopt's third own refusal with a fixed pair and document, which no row of the adopt refusals states.
@row_add n=2 row=contract/log-detail-codes/not-resumed area=surface fact=F4 detector=none
boundary: log detail codes
item: not-resumed
promise: Written to the log by `viola adopt` as `process-exit` subject self, exit 1, after the child's own `process-exit`, when the child ended before its session resumed; no line carries the session id.
basis: fct-uwoaubez, fct-blcs6o5x
because: the closed process-exit detail set gains `not-resumed`, a log word at a boundary no detail-code row states.
