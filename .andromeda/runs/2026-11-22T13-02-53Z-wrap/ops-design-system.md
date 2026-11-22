@row_set n=1 row=contract/cli-exit-codes/13 cell=promise seen=1 detector=D-design-cli-wording
value: `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`, `empty-text` or `paste-cut`; under `--json` the refusal object.
basis: fct-vl24ioge
because: The closed `not-delivered` detail set gained `paste-cut`, refused with exit 13, so the row enumerating exit 13's details must name it.
@row_add n=1 row=contract/refusal-hints/not-delivered-paste-cut area=surface fact=F1 detector=D-design-cli-wording
boundary: refusal hints
item: not-delivered · paste-cut
promise: `<name> did not take the whole text in time; a human must look at the session before the next send`, printed in human mode only; the wording is provisional.
basis: fct-z6owtgsx
because: The chunk adds a new hint literal for the `paste-cut` detail, and every refusal hint stands as its own row in the refusal-hints contract.
@row_add n=2 row=decision/paste-cut-refuses-until-human area=surface-cli fact=F2 detector=none
name: Paste-cut refuses later sends until a human acts
rule: After a paste-cut every next send to that session is refused `not-delivered` `paste-cut` with no cursor and nothing typed, until a human key is read there or a `prompt-submitted`, `session-start` or `session-end` event lands; a release or a turn end leaves it standing; the mark is in memory only.
basis: fct-z6owtgsx
because: The chunk decides that a cut paste marks the session so the next sends are refused until a human intervenes, a rule every send caller now meets.
@row_add n=3 row=decision/cut-paste-closes-bracket-no-enter area=surface-cli fact=F3 detector=none
name: Cut paste closes its bracket without Enter
rule: A paste cut at its bound closes its bracketed-paste markers and types no Enter; the human's keys are never refused or dropped during a paste, only queued behind at most one in-flight chunk and the closing marker.
basis: fct-shrbvdzl
because: The chunk bounds the paste and decides how a cut ends it and how the human's keys wait behind it, which no row of the cli surface states.
