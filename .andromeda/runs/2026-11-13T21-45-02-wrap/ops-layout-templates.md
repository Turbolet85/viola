@row_add n=1 row=contract/viola-answer-output/not-delivered-hints area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola answer output
item: not-delivered hints
promise: A human-mode `viola answer` refused not-delivered ends with its own hint for `unknown-dialog`, and for `control-character`, `empty-text`, `input-not-ready`, `no-prompt-submitted` and `turn-running` with the hint `viola send` holds for that detail; an opaque detail or no detail ends with no hint.
basis: fct-yeqzuggx
because: The answer hint table now carries the send hints for five more not-delivered details, a cli output wording the masters state only for unknown-dialog, so the answer output contract must name it.
@row_add n=2 row=contract/send-answer-hints/unverified-cli area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: send and answer refusal hints
item: unverified-cli
promise: A human-mode `viola send` or `viola answer` refused `unverified-cli` (exit 12) ends stderr with the one shared hint `hint: run viola verify for this CLI version`.
basis: fct-yeqzuggx
because: send now ends its unverified-cli refusal with a hint it did not print before, shared with answer, a new output wording no layout-templates row states.
