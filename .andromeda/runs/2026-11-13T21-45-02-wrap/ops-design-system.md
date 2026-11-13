@row_set n=1 row=contract/refusal-hints/refusal-line cell=promise seen=1 fact=F1 detector=D-design-cli-wording
value: A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
basis: fct-yeqzuggx (F1 hints), fct-avspoz2u
because: answer's not-delivered, human-typing and unverified-cli refusals now carry the hint send holds for the same reason or detail, and send's unverified-cli now prints its hint, so the hint-keying contract must say the hint is shared by both verbs and that an opaque or absent detail gets none.
@row_set n=2 row=contract/exit-21-json-cause/unwrapped-name cell=promise seen=1 fact=F2 detector=D-design-cli-wording
value: With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent or a regular file reads `unwrapped-name`; a symbolic link in its place is refused by the strict-modes check and reads `strict-modes-failed`.
basis: fct-yeqzuggx (F5 strict link), fct-avspoz2u
because: the strict-modes check now refuses a symbolic link in place of an instance dir before its mode verdict, so a symlinked instance directory no longer reaches the unwrapped-name read and the row's symlink clause is false.
