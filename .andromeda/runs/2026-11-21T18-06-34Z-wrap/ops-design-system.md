@row_add n=1 row=contract/viola-verify-output/cross-session-prefix-row area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola verify output
item: cross-session-prefix row
promise: The cross-session-prefix row prints its step line ending `pass` or `fail`, reading `fail` rather than refusing the run when no runtime dir is set, and no step line, summary, refusal or `--json` document of `viola verify` names the session's socket address, its path or its pid.
basis: fct-zjeenbpz
because: verify now reads XDG_RUNTIME_DIR once to compose a session's socket address for the 29th relied row, which reads pass or fail only and keeps the address out of every output, so the cli surface owes the promise beside the trust-record row's.
@row_none n=1 fact=F1 type=4
relied: the hidden `--names-only` option of `viola hook`, valid only with `--capture`, files only closed tool words and no payload byte; the hook verb already has no human design surface and appears in no help output, and the capture arm's file contract is architecture's.
