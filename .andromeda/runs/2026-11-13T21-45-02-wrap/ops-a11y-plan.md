@row_add n=1 row=contract/cli-refusal-hint/answer-not-delivered area=cognitive-accessibility fact=F1 detector=none
boundary: CLI refusal hints
item: viola answer not-delivered
promise: A human-mode viola answer refusal not-delivered with detail control-character, empty-text, input-not-ready, no-prompt-submitted or turn-running ends with the hint viola send gives for that detail, byte for byte; unknown-dialog has its own hint; an opaque or missing detail gives no hint.
basis: fct-yeqzuggx
because: answer_hint now gives send's hint for each of five not-delivered details and its own for unknown-dialog, a promise on the refusal output that no a11y-plan row states.
@row_add n=2 row=contract/cli-refusal-hint/unverified-cli area=cognitive-accessibility fact=F2 detector=none
boundary: CLI refusal hints
item: unverified-cli
promise: An unverified-cli refusal (exit 12) of viola send or viola answer ends with the line hint: run viola verify for this CLI version.
basis: fct-yeqzuggx
because: send refused unverified-cli printed no hint before and now shares answer's fixed hint, a promise on the refusal output that no a11y-plan row states.
